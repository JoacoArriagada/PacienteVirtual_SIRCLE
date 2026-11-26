mod fixtures;

use std::{path::Path, sync::{Arc, Mutex}};

use axum::{Json, Router, extract::State, http::{HeaderMap, StatusCode}, routing::{get, post}};
use fixtures::{Servidor, intento_lr};
use serde_json::{Value, json};
use sircle::{
    agentes::supervisor::Supervisor,
    app::{estado::Estado, router::router},
    modules::entrevista::ollama::{DIGEST_MODELO, MODELO},
    sqlite,
};
use tokio::net::TcpListener;

const PREGUNTA: &str = "¿Desde cuándo te cuesta respirar?";
const CLAVE: &str = "t041-turno-propio";
type Invocaciones = Arc<Mutex<Vec<String>>>;

async fn inventario() -> Json<Value> {
    Json(json!({ "models": [{ "name": MODELO, "digest": DIGEST_MODELO }] }))
}

async fn completar(State(invocaciones): State<Invocaciones>, cabeceras: HeaderMap) -> Json<Value> {
    let operacion = cabeceras["X-Sircle-Operacion"].to_str().unwrap().to_owned();
    invocaciones.lock().unwrap().push(operacion);
    Json(json!({ "model": MODELO, "done": true, "message": { "content": r#"{"hecho_id":"lr-inicio"}"# } }))
}

#[tokio::test]
async fn impide_lectura_reenvio_e_inferencia_ajenos_antes_de_enrutar() {
    let mut servidor = Servidor::nuevo();
    let invocaciones = Invocaciones::default();
    let escucha = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let direccion = format!("http://{}", escucha.local_addr().unwrap());
    let motor = Router::new().route("/api/tags", get(inventario)).route("/api/chat", post(completar))
        .with_state(invocaciones.clone());
    let tarea = tokio::spawn(async move { axum::serve(escucha, motor).await.unwrap() });
    let trabajadores = Supervisor::iniciar(Path::new(env!("CARGO_BIN_EXE_sircle")), &direccion).await.unwrap();
    servidor.app = router(Estado::con_base(servidor.db.clone()).unwrap().con_trabajadores(trabajadores.clone()));

    let (titular, _) = servidor.entrar(intento_lr::ESTUDIANTE_LR).await;
    let (ajena, _) = servidor.entrar("estudiante-sin-asignacion").await;
    let (docente, _) = servidor.entrar("docente-demo").await;
    let db = sqlite::abrir_existente(&servidor.db).unwrap();
    db.execute("INSERT INTO identidades VALUES ('otro-docente', 'docente')", []).unwrap();
    let (docente_ajeno, _) = servidor.entrar("otro-docente").await;

    let alta = intento_lr::alta(&servidor, &titular).await;
    let id = alta.cuerpo["intento_id"].as_str().unwrap();
    let vista = format!("/api/intentos/{id}");
    let turnos = format!("{vista}/turnos");
    let peticion = json!({"texto":PREGUNTA});
    let enviar = |cookie, clave| servidor.peticion("POST", &turnos, cookie, Some(clave), Some(peticion.clone()));
    let recuentos = || {
        let total: i64 = db.query_row("SELECT count(*) FROM turnos WHERE intento_id=?1", [id], |fila| fila.get(0)).unwrap();
        json!({ "llamadas_chat": invocaciones.lock().unwrap().len(), "turnos": total })
    };
    let pre = enviar(Some(&titular), CLAVE).await;
    assert_eq!(pre.estado, StatusCode::CONFLICT);
    let previo = recuentos();
    assert_eq!(previo, json!({ "llamadas_chat": 0, "turnos": 0 }));
    intento_lr::resolver_pre(&servidor, id);
    let propio = enviar(Some(&titular), CLAVE).await;
    assert_eq!(propio.estado, StatusCode::CREATED);
    let antes = recuentos();
    assert_eq!(antes, json!({ "llamadas_chat": 1, "turnos": 1 }));
    assert_eq!(invocaciones.lock().unwrap().as_slice(), [propio.cuerpo["operacion_id"].as_str().unwrap()]);

    let traza = format!("{turnos}/{}/traza", propio.cuerpo["turno_id"].as_str().unwrap());
    let mut matriz = Vec::new();
    for (actor, sesion, lectura, reenvio, nueva, lectura_traza) in [
        ("titular", Some(titular.as_str()), 200, 200, None, 403),
        ("estudiante ajeno", Some(ajena.as_str()), 403, 403, Some(403), 403),
        ("docente asignado", Some(docente.as_str()), 403, 403, Some(403), 200),
        ("docente ajeno", Some(docente_ajeno.as_str()), 403, 403, Some(403), 403),
        ("sin sesion", None, 401, 401, Some(401), 401),
    ] {
        let obtenida = servidor.consultar("GET", &vista, sesion, None).await;
        let repetida = enviar(sesion, CLAVE).await;
        let detalle = servidor.consultar("GET", &traza, sesion, None).await;
        assert_eq!(obtenida.estado.as_u16(), lectura);
        assert_eq!(repetida.estado.as_u16(), reenvio);
        assert_eq!(detalle.estado.as_u16(), lectura_traza);
        if lectura == 200 {
            assert_eq!(obtenida.cuerpo["turnos"], json!([propio.cuerpo]));
            assert_eq!(repetida.cuerpo, propio.cuerpo);
        } else {
            for cuerpo in [&obtenida.cuerpo, &repetida.cuerpo] {
                assert!(cuerpo.get("turno_id").is_none() && cuerpo.get("intento_id").is_none());
            }
        }
        if let Some(estado) = nueva {
            assert_eq!(enviar(sesion, "t041-clave-nueva").await.estado.as_u16(), estado);
        }
        assert_eq!(recuentos(), antes);
        matriz.push(json!({ "actor": actor, "leer": lectura, "reenviar": reenvio,
            "operacion_nueva": nueva, "leer_traza": lectura_traza, "recuentos": recuentos() }));
    }

    db.execute("UPDATE actividades SET habilitada=0 WHERE id='lr-ensayo'", []).unwrap();
    let inhabilitada = enviar(Some(&titular), CLAVE).await;
    assert_eq!(inhabilitada.estado, StatusCode::FORBIDDEN);
    db.execute("UPDATE actividades SET habilitada=1 WHERE id='lr-ensayo'", []).unwrap();
    db.execute("UPDATE identidades SET rol='docente' WHERE id='estudiante-lr'", []).unwrap();
    let rol_cambiado = enviar(Some(&titular), CLAVE).await;
    assert_eq!(rol_cambiado.estado, StatusCode::FORBIDDEN);
    db.execute("UPDATE identidades SET rol='estudiante' WHERE id='estudiante-lr'", []).unwrap();
    db.execute("UPDATE intentos SET estado='entregado' WHERE id=?1", [id]).unwrap();
    let tardia = enviar(Some(&titular), "t041-tardia").await;
    assert_eq!(tardia.estado, StatusCode::CONFLICT);
    assert_eq!(enviar(Some(&titular), CLAVE).await.cuerpo, propio.cuerpo);
    assert_eq!(recuentos(), antes);
    trabajadores.apagar().await;
    tarea.abort();

    println!("EVIDENCIA {}", json!({
        "intento": id, "turno_propio": propio.cuerpo, "pre_http": pre.estado.as_u16(),
        "antes_de_confirmar": previo, "matriz": matriz, "antes_de_pruebas_de_acceso": antes,
        "despues_de_pruebas_de_acceso": recuentos(), "actividad_inhabilitada_http": inhabilitada.estado.as_u16(),
        "rol_cambiado_http": rol_cambiado.estado.as_u16(), "operacion_tardia_http": tardia.estado.as_u16(),
        "motor": "doble HTTP con contador de peticiones efectivas a /api/chat",
    }));
}
