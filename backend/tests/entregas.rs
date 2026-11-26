mod fixtures;

use std::{path::Path, sync::Arc, time::Duration};

use axum::{Json, Router, extract::State, http::StatusCode, routing::{get, post}};
use fixtures::{Servidor, intento_lr, versiones_rubrica::{self, ESTUDIANTE_B, V1, V2}};
use rusqlite::{Connection, params};
use serde_json::{Value, json};
use sircle::{
    agentes::supervisor::Supervisor, app::{estado::Estado, router::router},
    modules::{entrevista::ollama::{DIGEST_MODELO, MODELO}, evaluacion::rubricas},
    sqlite,
};
use tokio::{net::TcpListener, sync::Notify, time::timeout};

#[derive(Default)]
struct Barrera {
    recibida: Notify,
    terminar: Notify,
}

async fn inventario() -> Json<Value> {
    Json(json!({"models":[{"name":MODELO,"digest":DIGEST_MODELO}]}))
}

async fn completar(State(barrera): State<Arc<Barrera>>) -> Json<Value> {
    barrera.recibida.notify_one();
    barrera.terminar.notified().await;
    Json(json!({"model":MODELO,"done":true,"message":{"content":r#"{"hecho_id":"lr-inicio"}"#}}))
}

async fn base_calibrada() -> Servidor {
    if let Ok(ruta) = std::env::var("SIRCLE_T055_DB") {
        return Servidor::copiar(Path::new(&ruta));
    }
    let servidor = Servidor::nuevo();
    let mut db = sqlite::abrir_existente(&servidor.db).unwrap();
    versiones_rubrica::asignar_segundo_estudiante(&db);
    let v1 = versiones_rubrica::cargar(&mut db, V1);
    rubricas::habilitar_ensayo(&db, v1.id).unwrap();
    versiones_rubrica::abrir(&servidor, intento_lr::ESTUDIANTE_LR, v1).await;
    let v2 = versiones_rubrica::cargar(&mut db, V2);
    rubricas::habilitar_ensayo(&db, v2.id).unwrap();
    versiones_rubrica::abrir(&servidor, ESTUDIANTE_B, v2).await;
    servidor
}

const CLAVE: &str = "t056-entrega";

fn recuentos(db: &Connection, intento: &str) -> Value {
    db.query_row(
        "SELECT i.estado,i.revision,
          (SELECT count(*) FROM entregas e WHERE e.intento_id=i.id),
          (SELECT count(*) FROM trabajos_evaluacion t JOIN entregas e ON e.id=t.entrega_id WHERE e.intento_id=i.id),
          (SELECT count(*) FROM operaciones o WHERE o.ambito=i.id AND o.clave=?2),
          (SELECT count(*) FROM turnos r WHERE r.intento_id=i.id)
         FROM intentos i WHERE i.id=?1",
        params![intento, CLAVE], |r| Ok(json!({"estado":r.get::<_,String>(0)?,"revision":r.get::<_,i64>(1)?,
            "entregas":r.get::<_,i64>(2)?,"trabajos":r.get::<_,i64>(3)?,"acuses":r.get::<_,i64>(4)?,"turnos":r.get::<_,i64>(5)?})),
    ).unwrap()
}

#[tokio::test]
async fn congela_a_con_v1_revierte_un_fallo_y_descarta_respuesta_tardia() {
    let mut servidor = base_calibrada().await;
    let db = sqlite::abrir_existente(&servidor.db).unwrap();
    let barrera = Arc::new(Barrera::default());
    let motor = Router::new().route("/api/tags", get(inventario)).route("/api/chat", post(completar)).with_state(barrera.clone());
    let escucha = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let url = format!("http://{}", escucha.local_addr().unwrap());
    let escucha = tokio::spawn(async move { axum::serve(escucha, motor).await.unwrap() });
    let trabajadores = Supervisor::iniciar(Path::new(env!("CARGO_BIN_EXE_sircle")), &url).await.unwrap();
    servidor.app = router(Estado::con_base(servidor.db.clone()).unwrap().con_trabajadores(trabajadores.clone()));
    let (titular, _) = servidor.entrar(intento_lr::ESTUDIANTE_LR).await;
    let (ajena, _) = servidor.entrar(ESTUDIANTE_B).await;
    let alta = intento_lr::alta(&servidor, &titular).await;
    let a = alta.cuerpo["intento_id"].as_str().unwrap().to_owned();
    let ruta = format!("/api/intentos/{a}");
    let entrega = format!("{ruta}/entrega");
    let enviar = |revision, clave, cookie| servidor.peticion("POST", &entrega, Some(cookie), Some(clave),
        Some(json!({"revision_esperada":revision})));
    assert_eq!(enviar(0, CLAVE, &titular).await.estado, StatusCode::CONFLICT);
    intento_lr::resolver_pre(&servidor, &a);
    let toma = servidor.peticion("POST", &format!("{ruta}/mediciones"), Some(&titular), Some("t056-presion"),
        Some(json!({"instrumento":"presion","accion":"toma_completa"}))).await;
    assert_eq!(toma.estado, StatusCode::CREATED);
    let orden = servidor.peticion("POST", &format!("{ruta}/ordenes"), Some(&titular), Some("t056-ecg"),
        Some(json!({"examen":"ecg-inicial","accion":"confirmar"}))).await;
    assert_eq!(orden.estado, StatusCode::CREATED);
    let borrador = servidor.peticion("PUT", &format!("{ruta}/razonamiento"), Some(&titular), Some("t056-hipotesis"),
        Some(json!({"revision_esperada":orden.cuerpo["revision"],"hipotesis":[{
            "rol":"principal","titulo":"Hipótesis parcial de ensayo","explicacion":"",
            "evidencias":[{"tipo":"medicion","origen_id":toma.cuerpo["medicion_id"]}]}]}))).await;
    assert_eq!(borrador.estado, StatusCode::OK);
    let revision = borrador.cuerpo["revision"].as_i64().unwrap();
    assert_eq!(enviar(revision-1, CLAVE, &titular).await.estado, StatusCode::CONFLICT);
    assert_eq!(enviar(revision, CLAVE, &ajena).await.estado, StatusCode::FORBIDDEN);

    db.execute_batch(
        "CREATE TRIGGER falla_t056 BEFORE INSERT ON operaciones WHEN NEW.clave='t056-entrega'
         BEGIN SELECT RAISE(ABORT,'Fallo de prueba antes del commit'); END;",
    ).unwrap();
    let fallo = enviar(revision, CLAVE, &titular).await;
    assert_eq!(fallo.estado, StatusCode::INTERNAL_SERVER_ERROR);
    let revertido = recuentos(&db, &a);
    assert_eq!(revertido, json!({"estado":"atencion","revision":revision,"entregas":0,"trabajos":0,"acuses":0,"turnos":0}));
    db.execute("DROP TRIGGER falla_t056", []).unwrap();

    let ruta_turnos = format!("{ruta}/turnos");
    let tardia = servidor.peticion("POST", &ruta_turnos, Some(&titular), Some("t056-tardia"),
        Some(json!({"texto":"¿Cuándo empezó la dificultad para respirar?"})));
    let confirmar = async {
        timeout(Duration::from_secs(15), barrera.recibida.notified()).await.unwrap();
        let (primero, otro) = tokio::join!(enviar(revision, CLAVE, &titular), enviar(revision, CLAVE, &titular));
        let (creado, repetido) = if primero.estado == StatusCode::CREATED { (primero, otro) } else { (otro, primero) };
        assert_eq!(creado.estado, StatusCode::CREATED);
        assert_eq!(repetido.estado, StatusCode::OK);
        assert_eq!(creado.cuerpo, repetido.cuerpo);
        let original: String = db.query_row("SELECT instantanea FROM entregas WHERE intento_id=?1", [&a], |r| r.get(0)).unwrap();
        barrera.terminar.notify_one();
        (creado, original)
    };
    let (tardia, (creado, original)) = tokio::join!(tardia, confirmar);
    assert_eq!(tardia.estado, StatusCode::CONFLICT);
    assert_eq!(creado.cuerpo["rubrica_id"], alta.cuerpo["rubrica_id"]);
    assert_eq!(creado.cuerpo["version_rubrica"], "1");
    assert_eq!(creado.cuerpo["rubrica_sha256"], alta.cuerpo["rubrica_sha256"]);
    assert_eq!(creado.cuerpo["revision"], revision);
    assert_eq!(creado.cuerpo["post"], "pendiente");
    assert_eq!(creado.cuerpo["estado_evaluacion"], "pendiente");
    let nueva_clave = enviar(revision, "t056-clave-nueva", &titular).await;
    assert_eq!(nueva_clave.estado, StatusCode::OK);
    assert_eq!(nueva_clave.cuerpo, creado.cuerpo);
    let conflicto = enviar(revision+1, CLAVE, &titular).await;
    assert_eq!(conflicto.estado, StatusCode::CONFLICT);
    let informe_tardio = servidor.consultar("GET", &format!("{ruta}/ordenes/{}", orden.cuerpo["orden_id"].as_str().unwrap()),
        Some(&titular), None).await;
    assert_eq!(informe_tardio.estado, StatusCode::CONFLICT);
    let cerrado = servidor.consultar("GET", &ruta, Some(&titular), None).await;
    assert_eq!(cerrado.cuerpo["entrega"], creado.cuerpo);
    assert!(cerrado.cuerpo["entrega"].get("instantanea").is_none());
    let finalizado = recuentos(&db, &a);
    assert_eq!(finalizado, json!({"estado":"entregado","revision":revision+1,"entregas":1,"trabajos":1,"acuses":1,"turnos":0}));
    let (en_cola, instantanea, version_activa): (String, String, String) = db.query_row(
        "SELECT t.estado,e.instantanea,r.version FROM trabajos_evaluacion t JOIN entregas e ON e.id=t.entrega_id
         JOIN intentos i ON i.id=e.intento_id JOIN rubricas_activas_ensayo ra ON ra.actividad_id=i.actividad_id
         JOIN rubricas r ON r.id=ra.rubrica_id WHERE i.id=?1", [&a], |r| Ok((r.get(0)?,r.get(1)?,r.get(2)?)),
    ).unwrap();
    assert_eq!(version_activa, "2");
    assert_eq!(en_cola, "pendiente");
    assert_eq!(instantanea, original);
    let congelada: Value = serde_json::from_str(&instantanea).unwrap();
    assert_eq!(congelada["borrador"], borrador.cuerpo);
    assert_eq!(congelada["evidencias"].as_array().unwrap().len(), 1);
    assert_eq!(congelada["evidencias"][0]["origen_id"], toma.cuerpo["medicion_id"]);
    assert_eq!(congelada["ordenes"], json!([orden.cuerpo]));
    assert_eq!(congelada["turnos"], json!([]));
    trabajadores.apagar().await;
    escucha.abort();

    let retenida = std::env::var("SIRCLE_T056_DB").ok();
    if let Some(ruta) = &retenida {
        db.execute("VACUUM INTO ?1", [ruta]).unwrap();
    }
    println!("EVIDENCIA {}", json!({"acuse":creado.cuerpo,"fallo_http":fallo.estado.as_u16(),"revertido":revertido,
        "final":finalizado,"trabajo":en_cola,"rubrica_activa":version_activa,"instantanea":congelada,
        "respuesta_tardia_http":tardia.estado.as_u16(),"informe_tardio_http":informe_tardio.estado.as_u16(),
        "reenvio_clave_nueva_http":nueva_clave.estado.as_u16(),"copia":retenida}));
}
