mod fixtures;

use std::{collections::HashMap, path::Path, sync::{Arc, Mutex}, time::Duration};

use axum::{Json, Router, extract::State, http::{HeaderMap, HeaderValue, StatusCode, header}, routing::{get, post}};
use fixtures::{Servidor, intento_lr, versiones_rubrica::{self, ESTUDIANTE_B, V1, V2}};
use rusqlite::params;
use serde_json::{Value, json};
use sircle::{
    agentes::{
        evaluador::perfil::{self, Borrador, Criterio, Entrada, Entrega, Evidencia, Hipotesis, Juicio, Observacion, RubricaFijada, TipoEvidencia},
        mensajes::{self, Agente, Cabecera, Respuesta, Solicitud, TipoResultado},
        supervisor::Supervisor,
    },
    app::{estado::Estado, router::router},
    modules::{entrevista::ollama::{DIGEST_MODELO, MODELO}, evaluacion::{evidencia, publicacion, rubricas, trabajos}},
    shared::autorizacion,
    sqlite,
};
use tokio::{net::TcpListener, task::JoinHandle, time::timeout};

async fn preparar_entrega() -> Servidor {
    if let Ok(base) = std::env::var("SIRCLE_T056_DB") {
        return Servidor::copiar(std::path::Path::new(&base));
    }
    let servidor = Servidor::nuevo();
    let mut db = sqlite::abrir_existente(&servidor.db).unwrap();
    versiones_rubrica::asignar_segundo_estudiante(&db);
    let v1 = versiones_rubrica::cargar(&mut db, V1);
    rubricas::habilitar_ensayo(&db, v1.id).unwrap();
    let a = versiones_rubrica::abrir(&servidor, intento_lr::ESTUDIANTE_LR, v1).await;
    let v2 = versiones_rubrica::cargar(&mut db, V2);
    rubricas::habilitar_ensayo(&db, v2.id).unwrap();
    versiones_rubrica::abrir(&servidor, ESTUDIANTE_B, v2).await;
    intento_lr::resolver_pre(&servidor, &a.id);
    let ruta = format!("/api/intentos/{}", a.id);
    let toma = servidor.peticion("POST", &format!("{ruta}/mediciones"), Some(&a.cookie), Some("t057-a-presion"),
        Some(json!({"instrumento":"presion","accion":"toma_completa"}))).await;
    assert_eq!(toma.estado, StatusCode::CREATED);
    let orden = servidor.peticion("POST", &format!("{ruta}/ordenes"), Some(&a.cookie), Some("t057-a-orden"),
        Some(json!({"examen":"ecg-inicial","accion":"confirmar"}))).await;
    assert_eq!(orden.estado, StatusCode::CREATED);
    let borrador = servidor.peticion("PUT", &format!("{ruta}/razonamiento"), Some(&a.cookie), Some("t057-a-borrador"),
        Some(json!({"revision_esperada":orden.cuerpo["revision"],"hipotesis":[{
            "rol":"principal","titulo":"Hipótesis parcial de ensayo","explicacion":"",
            "evidencias":[{"tipo":"medicion","origen_id":toma.cuerpo["medicion_id"]}]}]}))).await;
    assert_eq!(borrador.estado, StatusCode::OK);
    let entrega = servidor.peticion("POST", &format!("{ruta}/entrega"), Some(&a.cookie), Some("t057-a-entrega"),
        Some(json!({"revision_esperada":borrador.cuerpo["revision"]}))).await;
    assert_eq!(entrega.estado, StatusCode::CREATED);
    servidor
}

#[tokio::test]
async fn reconstruye_desde_una_entrega_y_su_rubrica_congeladas() {
    let servidor = preparar_entrega().await;
    let db = sqlite::abrir_existente(&servidor.db).unwrap();
    let (cookie_b, _) = servidor.entrar(ESTUDIANTE_B).await;
    let alta_b = intento_lr::alta(&servidor, &cookie_b).await;
    assert_eq!(alta_b.estado, StatusCode::OK);
    let b = alta_b.cuerpo["intento_id"].as_str().unwrap();
    intento_lr::resolver_pre(&servidor, b);
    let apunte_b = servidor.peticion("PUT", &format!("/api/intentos/{b}/razonamiento"), Some(&cookie_b),
        Some("t057-apunte-b"), Some(json!({"revision_esperada":1,"apunte":"TESTIGO_REGISTRO_AJENO_T057"}))).await;
    assert_eq!(apunte_b.estado, StatusCode::OK);
    assert_eq!(alta_b.cuerpo["version_rubrica"], "2");
    assert!(evidencia::construir(&db, b).unwrap().is_none());
    let trabajos_b: i64 = db.query_row("SELECT count(*) FROM trabajos_evaluacion t JOIN entregas e ON e.id=t.entrega_id WHERE e.intento_id=?1",
        [b], |r| r.get(0)).unwrap();
    assert_eq!(trabajos_b, 0);

    let (entrega, a, revision, caso, version_caso, rubrica, version, sha, congelada):
        (String, String, i64, String, String, i64, String, String, String) = db.query_row(
        "SELECT e.id,e.intento_id,e.revision,e.caso,e.version_caso,e.rubrica_id,e.version_rubrica,e.rubrica_sha256,e.instantanea
         FROM entregas e JOIN intentos i ON i.id=e.intento_id WHERE i.estudiante_id='estudiante-lr'",
        [], |r| Ok((r.get(0)?,r.get(1)?,r.get(2)?,r.get(3)?,r.get(4)?,r.get(5)?,r.get(6)?,r.get(7)?,r.get(8)?)),
    ).unwrap();
    let snapshot: Value = serde_json::from_str(&congelada).unwrap();
    db.execute_batch("CREATE TEMP TABLE encuestas_t057 (intento_id TEXT, respuestas TEXT);
                      CREATE TEMP TABLE claves_educativas_t057 (clave TEXT);
                      INSERT INTO claves_educativas_t057 VALUES ('TESTIGO_CLAVE_PRIVADA_T057');").unwrap();
    db.execute("INSERT INTO encuestas_t057 VALUES (?1,'TESTIGO_ENCUESTA_PROPIA_T057'), (?2,'TESTIGO_ENCUESTA_AJENA_T057')",
        params![a, b]).unwrap();
    let contexto = evidencia::construir(&db, &entrega).unwrap().unwrap();
    let entrada = serde_json::to_value(&contexto.entrada).unwrap();
    assert_eq!(entrada["entrega"], json!({"id":entrega,"intento_id":a,"revision":revision,
        "caso":caso,"version_caso":version_caso,"confirmada":true}));
    assert_eq!(entrada["rubrica"]["id"], rubrica.to_string());
    assert_eq!(entrada["rubrica"]["version"], version);
    assert_eq!(entrada["rubrica"]["version"], "1");
    assert_eq!(entrada["rubrica"]["sha256"], sha);
    assert_eq!(contexto.rubrica.criterios[0].peso, 20);
    let ids_esperados: Vec<_> = snapshot["evidencias"].as_array().unwrap().iter().map(|e| &e["origen_id"])
        .chain(snapshot["ordenes"].as_array().unwrap().iter().map(|o| &o["orden_id"])).collect();
    let ids_entrada: Vec<_> = entrada["evidencias"].as_array().unwrap().iter().map(|e| &e["id"]).collect();
    assert_eq!(ids_entrada, ids_esperados);
    assert!(contexto.entrada.evidencias.iter().all(|item| item.intento_id == a));
    assert_eq!(contexto.entrada.hipotesis[0].evidencias[0], snapshot["borrador"]["hipotesis"][0]["evidencias"][0]["origen_id"]);
    let serializada = serde_json::to_string(&entrada).unwrap();
    for testigo in ["TESTIGO_REGISTRO_AJENO_T057","TESTIGO_ENCUESTA_PROPIA_T057",
        "TESTIGO_ENCUESTA_AJENA_T057","TESTIGO_CLAVE_PRIVADA_T057",b] {
        assert!(!serializada.contains(testigo), "{testigo}");
    }
    db.execute("UPDATE mediciones SET valor=?1 WHERE intento_id=?2", params![r#"{"sistolica":100,"diastolica":60}"#, a]).unwrap();
    let reconstruida = evidencia::construir(&db, &entrega).unwrap().unwrap();
    assert_eq!(serde_json::to_value(reconstruida.entrada).unwrap(), entrada);
    db.execute("UPDATE rubricas SET contenido=CAST('{}' AS BLOB) WHERE id=?1", [rubrica]).unwrap();
    let alterada = evidencia::construir(&db, &entrega).err().unwrap().to_string();
    assert!(alterada.contains("huella"));
    println!("EVIDENCIA {}", json!({"entrada_congelada":entrada,"instantanea":snapshot,"intento_b":b,
        "trabajos_intento_abierto":trabajos_b,"invariancia_tras_mutar_registro_vivo":true,
        "rubrica_alterada":alterada,"testigos_ausentes":4}));
}

type Llamadas = Arc<Mutex<Vec<(String, Value)>>>;

async fn inventario_doble() -> Json<Value> {
    Json(json!({"models":[{"name":MODELO,"digest":DIGEST_MODELO}]}))
}

async fn evaluar_doble(State(llamadas): State<Llamadas>, cabeceras: HeaderMap, Json(pedido): Json<Value>) -> Json<Value> {
    let operacion = cabeceras["x-sircle-operacion"].to_str().unwrap().to_owned();
    let datos: Value = serde_json::from_str(pedido["messages"][1]["content"].as_str().unwrap()).unwrap();
    let refs = datos["evidencias"].as_array().unwrap();
    let id_de = |tipo: &str| refs.iter().find(|e| e["tipo"] == tipo).map(|e| e["id"].clone());
    let propuestas: serde_json::Map<String, Value> = datos["criterios"].as_array().unwrap().iter().map(|c| {
        let id = c["id"].as_str().unwrap();
        let medidas = id_de("medicion");
        let (actuacion, juicio, citas, explicacion, mejora) = match (id, medidas) {
            ("entrevista", _) => ("No se registraron preguntas.", "omision", vec![],
                "Faltó precisar el inicio de la falta de aire.", "Pregunta cuándo empezó y si se agravó."),
            ("valoracion", Some(m)) => ("Midió la presión.", "acierto", vec![m],
                "La presión tiene valor y unidad instrumentales.", "Añade frecuencia respiratoria en la próxima valoración."),
            ("decisiones", _) if id_de("apunte").is_some() && id_de("informe").is_some() =>
                ("Registró la decisión de interpretar el ECG como fibrilación auricular.", "error",
                    vec![id_de("apunte").unwrap(), id_de("informe").unwrap()],
                    "El informe documentó taquicardia sinusal.", "Contrasta la decisión con el informe y corrige la interpretación."),
            ("decisiones", _) if id_de("orden").is_none() =>
                ("No confirmó el ECG en esta oportunidad.", "omision", vec![],
                    "El registro íntegro carece de orden confirmada.", "Confirma la orden y consulta su informe cuando se habilite."),
            ("decisiones", _) => ("Confirmó el ECG y dejó el informe sin consultar.", "omision", id_de("orden").into_iter().collect(),
                "La orden está registrada, pero faltó consultar su resultado.", "Abre el informe y contrástalo con la hipótesis."),
            ("evidencias", Some(m)) => ("Vinculó el registro de presión.", "acierto", vec![m],
                "La cita de la medición es recuperable.", "Distingue además los datos de una declaración del paciente."),
            ("hipotesis", Some(m)) => ("Guardó una hipótesis parcial.", "omision", vec![m],
                "Falta explicar el vínculo y contrastar una alternativa.", "Compara la presión registrada con una hipótesis alternativa."),
            _ => ("No registró evidencia para este criterio.", "omision", vec![],
                "El intento cerró sin esa actuación.", "Incorpora la actuación y un vínculo recuperable."),
        };
        (id.to_owned(), json!({"a":actuacion,"e":citas,"j":juicio,"x":explicacion,"m":mejora}))
    }).collect();
    llamadas.lock().unwrap().push((operacion, pedido));
    Json(json!({"model":MODELO,"done":true,"message":{"content":json!({"resultados":propuestas}).to_string()}}))
}

async fn servicio_de_ensayo(servidor: &mut Servidor) -> (Estado, Supervisor, Llamadas, JoinHandle<()>) {
    let llamadas = Llamadas::default();
    let adaptador = Router::new().route("/api/tags", get(inventario_doble)).route("/api/chat", post(evaluar_doble))
        .with_state(llamadas.clone());
    let escucha = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let url = format!("http://{}", escucha.local_addr().unwrap());
    let tarea = tokio::spawn(async move { axum::serve(escucha, adaptador).await.unwrap() });
    let supervisor = Supervisor::iniciar(Path::new(env!("CARGO_BIN_EXE_sircle")), &url).await.unwrap();
    let app = Estado::con_base(servidor.db.clone()).unwrap().con_trabajadores(supervisor.clone());
    servidor.app = router(app.clone());
    (app, supervisor, llamadas, tarea)
}

#[tokio::test]
async fn rastrear_criterio_a_evidencia_y_regresion() {
    let mut servidor = preparar_entrega().await;
    let db = sqlite::abrir_existente(&servidor.db).unwrap();
    let (app, supervisor, llamadas, tarea) = servicio_de_ensayo(&mut servidor).await;
    assert!(trabajos::procesar_siguiente(&app).await.unwrap());
    assert!(!trabajos::procesar_siguiente(&app).await.unwrap());
    let (id, entrega, operacion, estado): (String, String, String, String) = db.query_row(
        "SELECT v.id,v.entrega_id,v.operacion_id,t.estado FROM evaluaciones v JOIN trabajos_evaluacion t ON t.entrega_id=v.entrega_id",
        [], |r| Ok((r.get(0)?,r.get(1)?,r.get(2)?,r.get(3)?)),
    ).unwrap();
    assert_eq!(estado, "terminado");
    let ruta = format!("/api/evaluaciones/{id}");
    let ruta_traza = format!("{ruta}/traza");
    let ruta_criterio = format!("{ruta}/criterios/valoracion/traza?revision=1");
    let (docente, _) = servidor.entrar("docente-demo").await;
    let (estudiante, _) = servidor.entrar(intento_lr::ESTUDIANTE_LR).await;
    db.execute("INSERT INTO identidades VALUES ('docente-ajeno-t058','docente')", []).unwrap();
    let (ajeno, _) = servidor.entrar("docente-ajeno-t058").await;
    let vista = servidor.consultar("GET", &ruta, Some(&docente), None).await;
    let traza = servidor.consultar("GET", &ruta_traza, Some(&docente), None).await;
    assert_eq!(vista.estado, StatusCode::OK);
    assert_eq!(traza.estado, StatusCode::OK);
    let criterio = servidor.consultar("GET", &ruta_criterio, Some(&docente), None).await;
    assert_eq!(criterio.estado, StatusCode::OK);
    assert_eq!(criterio.cuerpo["revision"], 1);
    assert_eq!(criterio.cuerpo["observacion"]["criterio_id"], "valoracion");
    assert!(criterio.cuerpo["citas"].as_array().unwrap().iter().any(|cita| {
        cita["evidencia"]["tipo"] == "medicion" && cita["procedencia"]["sha256"].as_str().is_some()
    }));
    let mut denegaciones = Vec::new();
    for actor in [("titular", &estudiante), ("docente ajeno", &ajeno)] {
        for recurso in [&ruta, &ruta_traza, &ruta_criterio] {
            let r = servidor.consultar("GET", recurso, Some(actor.1), None).await;
            assert_eq!(r.estado, StatusCode::FORBIDDEN);
            denegaciones.push(json!({"actor":actor.0,"ruta":recurso,"http":r.estado.as_u16()}));
        }
    }
    assert_eq!(vista.cuerpo["evaluacion_id"], id);
    assert_eq!(vista.cuerpo["entrega_id"], entrega);
    assert_eq!(vista.cuerpo["operacion_id"], operacion);
    assert_eq!(vista.cuerpo["version_rubrica"], "1");
    assert_eq!(vista.cuerpo["rubrica_sha256"], traza.cuerpo["rubrica"]["sha256"]);
    assert_eq!(traza.cuerpo["entrega"]["id"], entrega);
    assert_eq!(traza.cuerpo["mensaje"]["operacion_id"], operacion);
    assert_eq!(traza.cuerpo["invocacion"]["operacion_id"], operacion);
    assert_eq!(traza.cuerpo["invocacion"]["modelo"], MODELO);
    assert_eq!(traza.cuerpo["invocacion"]["digest"], DIGEST_MODELO);
    let criterios = traza.cuerpo["invocacion"]["solicitud"]["format"]["properties"]["resultados"]["properties"]
        .as_object().unwrap();
    for criterio in criterios.values() {
        for campo in ["a", "x", "m"] {
            assert_eq!(criterio["properties"][campo]["minLength"], 1);
        }
    }
    assert_eq!(traza.cuerpo["evaluacion_id"], id);
    assert_eq!(traza.cuerpo["borrador_guardado"], vista.cuerpo["borrador"]);
    {
        let capturadas = llamadas.lock().unwrap();
        assert_eq!(capturadas.len(), 1);
        assert_eq!(capturadas[0].0, operacion);
        assert_eq!(capturadas[0].1, traza.cuerpo["invocacion"]["solicitud"]);
    }
    let observaciones = vista.cuerpo["borrador"]["observaciones"].as_array().unwrap();
    assert_eq!(observaciones.len(), 5);
    let mut aciertos = 0;
    let evidencia = traza.cuerpo["entrada"]["evidencias"].as_array().unwrap();
    let generadas = traza.cuerpo["invocacion"]["seleccion"]["observaciones"].as_array().unwrap();
    for guardada in observaciones {
        let generada = generadas.iter().find(|o| o["criterio_id"] == guardada["criterio_id"]).unwrap();
        for campo in ["criterio_id","actuacion","evidencias","juicio","explicacion","mejora"] {
            assert_eq!(guardada[campo], generada[campo], "{campo}");
        }
        assert!(!guardada["expectativa"].as_str().unwrap().is_empty());
        if guardada["juicio"] == "acierto" {
            aciertos += 1;
            assert!(guardada["evidencias"].as_array().unwrap().iter().all(|id| evidencia.iter()
                .any(|e| &e["id"] == id && e["intento_id"] == vista.cuerpo["intento_id"])));
        }
    }
    assert_eq!(aciertos, 2);
    supervisor.apagar().await;
    tarea.abort();
    println!("EVIDENCIA {}", json!({"motor":"doble de prueba T058, protocolo Ollama","evaluacion_docente":vista.cuerpo,
        "traza_docente":traza.cuerpo,"denegaciones":denegaciones,"creditos_citados":aciertos,
        "llamadas":llamadas.lock().unwrap().len(),"trabajo":estado}));
}

#[tokio::test]
async fn escenario_sin_examen_no_evaluable() {
    let mut servidor = Servidor::nuevo();
    let mut db = sqlite::abrir_existente(&servidor.db).unwrap();
    db.execute_batch("UPDATE actividades SET habilitada=1 WHERE id='nannu-ensayo';
        INSERT INTO asignaciones VALUES ('nannu-ensayo','estudiante-lr','docente-demo');").unwrap();
    let fuente = V1.replace("lr-pregrado", "nannu-t058").replace("lr-ensayo", "nannu-ensayo").replace("P01", "P02");
    let aprobada = versiones_rubrica::cargar(&mut db, &fuente);
    rubricas::habilitar_ensayo(&db, aprobada.id).unwrap();
    let (cookie, _) = servidor.entrar(intento_lr::ESTUDIANTE_LR).await;
    let alta = servidor.peticion("POST", "/api/intentos", Some(&cookie), Some("t058-nannu"),
        Some(json!({"actividad_id":"nannu-ensayo"}))).await;
    assert_eq!(alta.estado, StatusCode::CREATED);
    let id = alta.cuerpo["intento_id"].as_str().unwrap();
    intento_lr::resolver_pre(&servidor, id);
    let entrega = servidor.peticion("POST", &format!("/api/intentos/{id}/entrega"), Some(&cookie), Some("t058-p02-entrega"),
        Some(json!({"revision_esperada":1}))).await;
    assert_eq!(entrega.estado, StatusCode::CREATED);
    let (app, supervisor, llamadas, tarea) = servicio_de_ensayo(&mut servidor).await;
    assert!(trabajos::procesar_siguiente(&app).await.unwrap());
    let (id, borrador): (String, String) = db.query_row("SELECT id,borrador FROM evaluaciones", [], |r| Ok((r.get(0)?,r.get(1)?))).unwrap();
    let guardado: Borrador = serde_json::from_str(&borrador).unwrap();
    let decision = guardado.observaciones.iter().find(|c| c.criterio_id == "decisiones").unwrap();
    assert_eq!(decision.juicio, Juicio::NoEvaluable);
    assert!(decision.evidencias.is_empty());
    let peticion = llamadas.lock().unwrap()[0].1.clone();
    assert!(peticion["format"]["properties"]["resultados"]["properties"].get("decisiones").is_none());
    supervisor.apagar().await;
    tarea.abort();
    println!("EVIDENCIA {}", json!({"motor":"doble de prueba T058, protocolo Ollama","caso":"P02",
        "evaluacion_id":id,"criterio_sin_oportunidad":decision}));
}

#[tokio::test]
async fn distingue_error_omision_y_acuse_sin_evento() {
    let mut servidor = Servidor::nuevo();
    let mut db = sqlite::abrir_existente(&servidor.db).unwrap();
    versiones_rubrica::asignar_segundo_estudiante(&db);
    db.execute_batch("INSERT INTO identidades VALUES ('estudiante-t061-c','estudiante');
        INSERT INTO asignaciones VALUES ('lr-ensayo','estudiante-t061-c','docente-demo');").unwrap();
    let mut fixture: Value = serde_json::from_str(V1).unwrap();
    fixture["contenido"]["codigo"] = json!("lr-integridad-ensayo");
    fixture["contenido"]["criterios"][2]["evidencias"] = json!(["orden","informe","apunte"]);
    let rubrica = versiones_rubrica::cargar(&mut db, &fixture.to_string());
    rubricas::habilitar_ensayo(&db, rubrica.id).unwrap();
    let mut pruebas = Vec::new();
    let orden = json!({"examen":"ecg-inicial","accion":"confirmar"});
    let mut acuse_sin_orden = Value::Null;
    let mut apunte_erroneo = Value::Null;
    let mut informe_error = Value::Null;
    for (identidad, juicio) in [(intento_lr::ESTUDIANTE_LR, "error"), (ESTUDIANTE_B, "omision"),
        ("estudiante-t061-c", "revision_docente")] {
        let intento = versiones_rubrica::abrir(&servidor, identidad, rubrica.clone()).await;
        intento_lr::resolver_pre(&servidor, &intento.id);
        let ruta = format!("/api/intentos/{}", intento.id);
        if juicio != "omision" {
            let alta = servidor.peticion("POST", &format!("{ruta}/ordenes"), Some(&intento.cookie), Some("t061-orden"), Some(orden.clone())).await;
            assert_eq!(alta.estado, StatusCode::CREATED);
            let orden_id = alta.cuerpo["orden_id"].as_str().unwrap();
            if juicio == "error" {
                let consulta = servidor.consultar("GET", &format!("{ruta}/ordenes/{orden_id}"), Some(&intento.cookie), None).await;
                assert_eq!(consulta.estado, StatusCode::OK);
                informe_error = consulta.cuerpo;
                let actual = servidor.consultar("GET", &ruta, Some(&intento.cookie), None).await;
                let guardado = servidor.peticion("PUT", &format!("{ruta}/razonamiento"), Some(&intento.cookie), Some("t061-decision"),
                    Some(json!({"revision_esperada":actual.cuerpo["revision"],
                        "apunte":"Decidí interpretar el ECG como fibrilación auricular."}))).await;
                assert_eq!(guardado.estado, StatusCode::OK);
                apunte_erroneo = guardado.cuerpo;
            } else {
                let tx = db.transaction().unwrap();
                assert_eq!(tx.execute("DELETE FROM informes WHERE orden_id=?1", [orden_id]).unwrap(), 1);
                assert_eq!(tx.execute("DELETE FROM ordenes WHERE id=?1", [orden_id]).unwrap(), 1);
                tx.commit().unwrap();
                let reenvio = servidor.peticion("POST", &format!("{ruta}/ordenes"), Some(&intento.cookie), Some("t061-orden"), Some(orden.clone())).await;
                assert_eq!(reenvio.estado, StatusCode::OK);
                assert_eq!(reenvio.cuerpo, alta.cuerpo);
                acuse_sin_orden = reenvio.cuerpo;
                let sin_evento = servidor.consultar("GET", &format!("{ruta}/ordenes/{orden_id}"), Some(&intento.cookie), None).await;
                assert_eq!(sin_evento.estado, StatusCode::NOT_FOUND);
            }
        }
        let actual = servidor.consultar("GET", &ruta, Some(&intento.cookie), None).await;
        let confirmada = servidor.peticion("POST", &format!("{ruta}/entrega"), Some(&intento.cookie), Some("t061-entrega"),
            Some(json!({"revision_esperada":actual.cuerpo["revision"]}))).await;
        assert_eq!(confirmada.estado, StatusCode::CREATED);
        pruebas.push((juicio, confirmada.cuerpo));
    }
    let (app, supervisor, llamadas, tarea) = servicio_de_ensayo(&mut servidor).await;
    for _ in &pruebas {
        assert!(trabajos::procesar_siguiente(&app).await.unwrap());
    }
    assert!(!trabajos::procesar_siguiente(&app).await.unwrap());
    assert_eq!(llamadas.lock().unwrap().len(), 3);
    let (docente, _) = servidor.entrar("docente-demo").await;
    let mut resultados = Vec::new();
    for (juicio, confirmada) in pruebas {
        let entrega = confirmada["entrega_id"].as_str().unwrap();
        let id: String = db.query_row("SELECT id FROM evaluaciones WHERE entrega_id=?1", [entrega], |r| r.get(0)).unwrap();
        let ruta = format!("/api/evaluaciones/{id}");
        let vista = servidor.consultar("GET", &ruta, Some(&docente), None).await;
        let traza = servidor.consultar("GET", &format!("{ruta}/traza"), Some(&docente), None).await;
        assert_eq!(vista.estado, StatusCode::OK);
        assert_eq!(traza.estado, StatusCode::OK);
        let decisiones = vista.cuerpo["borrador"]["observaciones"].as_array().unwrap().iter()
            .find(|o| o["criterio_id"] == "decisiones").unwrap();
        assert_eq!(decisiones["juicio"], juicio, "{decisiones}");
        match juicio {
            "error" => {
                assert_eq!(decisiones["evidencias"], json!([apunte_erroneo["apunte_id"],informe_error["informe_id"]]));
                assert_eq!(traza.cuerpo["acuses_sin_evento"], json!([]));
            }
            "omision" => {
                assert_eq!(decisiones["evidencias"], json!([]));
                assert_eq!(traza.cuerpo["acuses_sin_evento"], json!([]));
                assert!(traza.cuerpo["aplicables"].as_array().unwrap().contains(&json!("decisiones")));
            }
            _ => {
                assert_eq!(decisiones["evidencias"], json!([]));
                assert_eq!(decisiones["incidencia"]["motivo"], "acuse_sin_evento");
                assert_eq!(decisiones["incidencia"]["descuento_automatico"], 0);
                assert_eq!(decisiones["incidencia"]["acuses"], json!([{"tipo":"orden","clave":"t061-orden","referencia":acuse_sin_orden["orden_id"]}]));
                assert_eq!(decisiones["incidencia"]["acuses"], traza.cuerpo["acuses_sin_evento"]);
                assert!(!traza.cuerpo["aplicables"].as_array().unwrap().contains(&json!("decisiones")));
                assert!(!traza.cuerpo["invocacion"]["solicitud"].to_string().contains(acuse_sin_orden["orden_id"].as_str().unwrap()));
            }
        }
        resultados.push(json!({"evaluacion_id":id,"entrega":confirmada,"decision":decisiones,
            "aplicables":traza.cuerpo["aplicables"],"acuses_sin_evento":traza.cuerpo["acuses_sin_evento"]}));
    }
    supervisor.apagar().await;
    tarea.abort();
    let copia = std::env::var("SIRCLE_T061_DB").ok();
    if let Some(ruta) = &copia {
        db.execute("VACUUM INTO ?1",[ruta]).unwrap();
    }
    println!("EVIDENCIA {}", json!({"regresion_incidente":resultados,"acuse_sin_orden":acuse_sin_orden,
        "acto_erroneo":apunte_erroneo,"informe":informe_error,"copia":copia}));
}

#[tokio::test]
async fn rectifica_con_motivo_y_reproduce_desglose_de_v1_despues_de_v2() {
    let mut servidor = Servidor::nuevo();
    let mut db = sqlite::abrir_existente(&servidor.db).unwrap();
    versiones_rubrica::asignar_segundo_estudiante(&db);
    let v1 = versiones_rubrica::cargar(&mut db, V1);
    rubricas::habilitar_ensayo(&db, v1.id).unwrap();
    let a = versiones_rubrica::abrir(&servidor, intento_lr::ESTUDIANTE_LR, v1.clone()).await;
    intento_lr::resolver_pre(&servidor, &a.id);
    let ruta = format!("/api/intentos/{}", a.id);
    let medida = servidor.peticion("POST", &format!("{ruta}/mediciones"), Some(&a.cookie), Some("t062-medicion"),
        Some(json!({"instrumento":"presion","accion":"toma_completa"}))).await;
    assert_eq!(medida.estado, StatusCode::CREATED);
    let entrega = servidor.peticion("POST", &format!("{ruta}/entrega"), Some(&a.cookie), Some("t062-entrega"),
        Some(json!({"revision_esperada":medida.cuerpo["revision"]}))).await;
    assert_eq!(entrega.estado, StatusCode::CREATED);
    let (app, supervisor, _, tarea) = servicio_de_ensayo(&mut servidor).await;
    assert!(trabajos::procesar_siguiente(&app).await.unwrap());
    let id: String = db.query_row("SELECT id FROM evaluaciones", [], |r| r.get(0)).unwrap();
    let revision_count = |db: &rusqlite::Connection| db.query_row(
        "SELECT count(*) FROM revisiones_evaluacion WHERE evaluacion_id=?1", [&id], |r| r.get::<_, i64>(0),
    ).unwrap();
    assert_eq!(revision_count(&db), 1);
    let ruta = format!("/api/evaluaciones/{id}");
    let ruta_base = format!("{ruta}/revisiones/1");
    let (docente, _) = servidor.entrar("docente-demo").await;
    let inicial = servidor.consultar("GET", &ruta, Some(&docente), None).await;
    assert_eq!(inicial.estado, StatusCode::OK);
    let primera = &inicial.cuerpo["revision_actual"];
    assert_eq!(primera["puntaje"]["total_centesimas"], 4000);
    assert_eq!(inicial.cuerpo["rubrica_sha256"], v1.sha256);

    let v2 = versiones_rubrica::cargar(&mut db, V2);
    rubricas::habilitar_ensayo(&db, v2.id).unwrap();
    let b = versiones_rubrica::abrir(&servidor, ESTUDIANTE_B, v2.clone()).await;
    let reproducida = servidor.consultar("GET", &ruta_base, Some(&docente), None).await;
    assert_eq!(reproducida.estado, StatusCode::OK);
    assert_eq!(reproducida.cuerpo, *primera);
    let rubrica_v1: Vec<u8> = db.query_row("SELECT contenido FROM rubricas WHERE id=?1", [v1.id], |r| r.get(0)).unwrap();
    let rubrica_v1: rubricas::Rubrica = serde_json::from_slice(&rubrica_v1).unwrap();
    let guardado: Borrador = serde_json::from_value(primera["resultado"].clone()).unwrap();
    let verificado = sircle::modules::evaluacion::revisiones::calcular(&rubrica_v1, &guardado, &HashMap::new()).unwrap();
    assert_eq!(serde_json::to_value(verificado).unwrap(), primera["puntaje"]);

    db.execute("INSERT INTO identidades VALUES ('docente-ajeno-t062','docente')", []).unwrap();
    let (ajeno, _) = servidor.entrar("docente-ajeno-t062").await;
    let listado = servidor.consultar("GET", "/api/actividades/lr-ensayo/entregas", Some(&docente), None).await;
    assert_eq!(listado.estado, StatusCode::OK);
    assert_eq!(listado.cuerpo[0]["evaluacion_id"], id);
    assert_eq!(servidor.consultar("GET", "/api/actividades/lr-ensayo/entregas", Some(&ajeno), None).await.estado, StatusCode::FORBIDDEN);
    let body = json!({"revision_esperada":1,"motivo":"La toma consta, pero la valoración inicial fue parcial.",
        "cambios":[{"criterio_id":"valoracion","actuacion":"Midió la presión arterial.",
            "evidencias":[medida.cuerpo["medicion_id"]],"juicio":"acierto","credito":75,
            "explicacion":"La presión tiene un registro válido. Faltó completar la valoración inicial.",
            "mejora":"Registra también la frecuencia respiratoria."}]});
    let rectificar = |actor, clave, datos| servidor.peticion("PATCH", &ruta, Some(actor), Some(clave), Some(datos));
    assert_eq!(rectificar(&ajeno, "t062-ajeno", body.clone()).await.estado, StatusCode::FORBIDDEN);
    assert_eq!(rectificar(&a.cookie, "t062-estudiante", body.clone()).await.estado, StatusCode::FORBIDDEN);
    let mut sin_motivo = body.clone();
    sin_motivo["motivo"] = json!("   ");
    assert_eq!(rectificar(&docente, "t062-sin-motivo", sin_motivo).await.estado, StatusCode::UNPROCESSABLE_ENTITY);
    let mut ajena = body.clone();
    ajena["cambios"][0]["evidencias"] = json!([b.id]);
    assert_eq!(rectificar(&docente, "t062-cita-ajena", ajena).await.estado, StatusCode::UNPROCESSABLE_ENTITY);
    let corregida = rectificar(&docente, "t062-rectificar", body.clone()).await;
    assert_eq!(corregida.estado, StatusCode::CREATED);
    assert_eq!(corregida.cuerpo["autor_id"], "docente-demo");
    assert_eq!(corregida.cuerpo["anterior"], 1);
    assert_eq!(corregida.cuerpo["revision"], 2);
    assert_eq!(corregida.cuerpo["motivo"], body["motivo"]);
    assert_eq!(corregida.cuerpo["diferencias"][0]["criterio_id"], "valoracion");
    assert_eq!(corregida.cuerpo["diferencias"][0]["credito_anterior"], 100);
    assert_eq!(corregida.cuerpo["diferencias"][0]["credito_nuevo"], 75);
    assert_eq!(corregida.cuerpo["puntaje"]["total_centesimas"], 3500);
    let reenvio = rectificar(&docente, "t062-rectificar", body.clone()).await;
    assert_eq!(reenvio.estado, StatusCode::OK);
    assert_eq!(reenvio.cuerpo, corregida.cuerpo);
    let obsoleta = rectificar(&docente, "t062-obsoleta", body).await;
    assert_eq!(obsoleta.estado, StatusCode::CONFLICT);
    assert_eq!(revision_count(&db), 2);
    let historica = servidor.consultar("GET", &ruta_base, Some(&docente), None).await;
    assert_eq!(historica.cuerpo, *primera);
    let vigente = servidor.consultar("GET", &ruta, Some(&docente), None).await;
    assert_eq!(vigente.cuerpo["revision_actual"], corregida.cuerpo);
    assert_eq!(vigente.cuerpo["borrador"], inicial.cuerpo["borrador"]);
    supervisor.apagar().await;
    tarea.abort();
    let copia = std::env::var("SIRCLE_T062_DB").ok();
    if let Some(ruta) = &copia {
        db.execute("VACUUM INTO ?1",[ruta]).unwrap();
    }
    println!("EVIDENCIA {}", json!({"serie_t062":{"a":a.id,"b":b.id,"rubrica_v1":v1,"rubrica_v2":v2,
        "evaluacion_id":id,"entrega":entrega.cuerpo,"inicial_antes_de_v2":primera,
        "reproducida_despues_de_v2":reproducida.cuerpo,"rectificacion":corregida.cuerpo,
        "obsoleta_http":obsoleta.estado.as_u16(),"reenvio_http":reenvio.estado.as_u16(),"copia":copia}}));
}

#[tokio::test]
async fn calcula_los_intentos_retenidos_de_t055_y_t056_con_su_version() {
    let mut servidor = preparar_entrega().await;
    let db = sqlite::abrir_existente(&servidor.db).unwrap();
    let (cookie, _) = servidor.entrar(ESTUDIANTE_B).await;
    let alta = intento_lr::alta(&servidor, &cookie).await;
    assert_eq!(alta.estado, StatusCode::OK);
    let b = alta.cuerpo["intento_id"].as_str().unwrap();
    intento_lr::resolver_pre(&servidor, b);
    let ruta = format!("/api/intentos/{b}");
    let medida = servidor.peticion("POST", &format!("{ruta}/mediciones"), Some(&cookie), Some("t062-b-medicion"),
        Some(json!({"instrumento":"presion","accion":"toma_completa"}))).await;
    assert_eq!(medida.estado, StatusCode::CREATED);
    let entrega = servidor.peticion("POST", &format!("{ruta}/entrega"), Some(&cookie), Some("t062-b-entrega"),
        Some(json!({"revision_esperada":medida.cuerpo["revision"]}))).await;
    assert_eq!(entrega.estado, StatusCode::CREATED);
    let (app, supervisor, _, tarea) = servicio_de_ensayo(&mut servidor).await;
    for _ in 0..2 {
        assert!(trabajos::procesar_siguiente(&app).await.unwrap());
    }
    let mut consulta = db.prepare("SELECT i.id,e.version_rubrica,r.puntaje FROM entregas e JOIN intentos i ON e.intento_id=i.id
       JOIN evaluaciones v ON v.entrega_id=e.id JOIN revisiones_evaluacion r ON r.evaluacion_id=v.id WHERE r.revision=1 ORDER BY e.version_rubrica").unwrap();
    let filas = consulta.query_map([], |fila| Ok((fila.get::<_, String>(0)?,fila.get::<_, String>(1)?,fila.get::<_, String>(2)?)))
        .unwrap().collect::<rusqlite::Result<Vec<_>>>().unwrap();
    assert_eq!(filas.len(), 2);
    let puntajes: Vec<_> = filas.iter().map(|(id, version, puntaje)| {
        let datos: Value = serde_json::from_str(puntaje).unwrap();
        json!({"id":id,"version":version,"puntaje":datos})
    }).collect();
    assert_eq!(puntajes[0]["puntaje"]["total_centesimas"], 4000);
    assert_eq!(puntajes[1]["puntaje"]["total_centesimas"], 3000);
    supervisor.apagar().await;
    tarea.abort();
    println!("EVIDENCIA {}", json!({"puntajes_intentos_retenidos":puntajes}));
}

#[tokio::test]
async fn publica_revision_elegida_y_reserva_devolucion_hasta_cerrar_post() {
    let mut servidor = match std::env::var("SIRCLE_T063_REVISION_DB") {
        Ok(db) => Servidor::copiar(Path::new(&db)),
        Err(_) => preparar_entrega().await,
    };
    let mut db = sqlite::abrir_existente(&servidor.db).unwrap();
    let (app, supervisor, _, tarea) = servicio_de_ensayo(&mut servidor).await;
    trabajos::procesar_siguiente(&app).await.unwrap();
    let (a, id): (String, String) = db.query_row(
        "SELECT e.intento_id,v.id FROM entregas e JOIN evaluaciones v ON v.entrega_id=e.id",
        [], |r| Ok((r.get(0)?,r.get(1)?)),
    ).unwrap();
    let doc_ruta = format!("/api/evaluaciones/{id}");
    let pub_ruta = format!("{doc_ruta}/publicacion");
    let ruta_a = format!("/api/intentos/{a}/devolucion");
    let (a_cookie, _) = servidor.entrar(intento_lr::ESTUDIANTE_LR).await;
    let (b_cookie, _) = servidor.entrar(ESTUDIANTE_B).await;
    let (docente, _) = servidor.entrar("docente-demo").await;
    db.execute("INSERT INTO identidades VALUES ('docente-ajeno-t063','docente')", []).unwrap();
    let (ajeno, _) = servidor.entrar("docente-ajeno-t063").await;
    let acceso = |actor, destino| servidor.consultar("GET", destino, Some(actor), None);
    let publicar = |actor, clave, revision| servidor.peticion("POST", &pub_ruta, Some(actor), Some(clave), Some(json!({"revision":revision})));
    let inicial = acceso(&a_cookie, &ruta_a).await;
    assert_eq!(inicial.cuerpo, json!({"estado":"post_pendiente"}));
    assert_eq!(publicar(&ajeno, "t063-ajeno", 1).await.estado, StatusCode::FORBIDDEN);
    assert_eq!(publicar(&a_cookie, "t063-alumno", 1).await.estado, StatusCode::FORBIDDEN);
    let antes = acceso(&docente, &doc_ruta).await;
    let ruta_base = format!("{doc_ruta}/revisiones/1");
    let base = acceso(&docente, &ruta_base).await;
    assert_eq!(base.estado, StatusCode::OK);
    let primera = publicar(&docente, "t063-primera", 1).await;
    assert_eq!(primera.estado, StatusCode::CREATED);
    let reenvio = publicar(&docente, "t063-primera", 1).await;
    assert_eq!(reenvio.estado, StatusCode::OK);
    assert_eq!(reenvio.cuerpo, primera.cuerpo);
    let retenida = acceso(&a_cookie, &ruta_a).await;
    assert_eq!(retenida.cuerpo, inicial.cuerpo);
    assert_eq!(acceso(&b_cookie, &ruta_a).await.estado, StatusCode::FORBIDDEN);
    assert_eq!(acceso(&ajeno, &doc_ruta).await.estado, StatusCode::FORBIDDEN);
    let post = format!("/api/intentos/{a}/encuestas/post");
    let respondido = servidor.peticion("PUT", &post, Some(&a_cookie), Some("t063-post"), Some(json!({
        "estado":"respondido","respuestas":{"P1":5,"P2":1},
    }))).await;
    assert_eq!(respondido.estado, StatusCode::OK);
    let visible_a = acceso(&a_cookie, &ruta_a).await;
    assert_eq!(visible_a.cuerpo["estado"], "publicada");
    assert_eq!(visible_a.cuerpo["revision"], 1);
    assert_eq!(visible_a.cuerpo["observaciones"], base.cuerpo["resultado"]["observaciones"]);
    assert_eq!(visible_a.cuerpo["puntaje"], base.cuerpo["puntaje"]);
    assert!(visible_a.cuerpo.get("autor_id").is_none());
    assert!(visible_a.cuerpo.get("borrador").is_none());
    assert!(publicacion::resolver_post_ensayo(&mut db, &a, "omitido").is_err());
    let retroactivo = servidor.peticion("PUT", &post, Some(&a_cookie), Some("t063-retroactivo"), Some(json!({"estado":"omitido"}))).await;
    assert_eq!(retroactivo.estado, StatusCode::CONFLICT);
    let revision_nueva = if antes.cuerpo["revision_actual"]["revision"] == 1 {
        let original = &base.cuerpo["resultado"]["observaciones"][1];
        let editada = servidor.peticion("PATCH", &doc_ruta, Some(&docente), Some("t063-editar"), Some(json!({
            "revision_esperada":1,"motivo":"Diferenciar la revisión aún no publicada",
            "cambios":[{"criterio_id":"valoracion","actuacion":original["actuacion"],"evidencias":original["evidencias"],
                "juicio":"acierto","credito":75,"explicacion":original["explicacion"],"mejora":"TESTIGO_REVISION_NO_PUBLICADA_T063"}]
        }))).await;
        assert_eq!(editada.estado, StatusCode::CREATED);
        editada.cuerpo
    } else {
        antes.cuerpo["revision_actual"].clone()
    };
    assert_eq!(revision_nueva["revision"], 2);
    assert_ne!(revision_nueva["resultado"]["observaciones"], visible_a.cuerpo["observaciones"]);
    assert_eq!(acceso(&a_cookie, &ruta_a).await.cuerpo, visible_a.cuerpo);
    let segunda = publicar(&docente, "t063-segunda", 2).await;
    assert_eq!(segunda.estado, StatusCode::CREATED);
    assert_eq!(segunda.cuerpo["numero"], 2);
    assert_eq!(publicar(&docente, "t063-primera", 1).await.cuerpo, primera.cuerpo);
    let actualizada = acceso(&a_cookie, &ruta_a).await;
    assert_eq!(actualizada.cuerpo["observaciones"], revision_nueva["resultado"]["observaciones"]);
    assert_eq!(actualizada.cuerpo["revision"], 2);
    let copia = Servidor::copiar(&servidor.db);
    let restaurada = copia.consultar("GET", &ruta_a, Some(&a_cookie), None).await;
    assert_eq!(restaurada.cuerpo, actualizada.cuerpo);
    let ruta_intento = format!("/api/intentos/{a}");
    let esperado = acceso(&a_cookie, &ruta_intento).await;
    assert_eq!(copia.consultar("GET", &ruta_intento, Some(&a_cookie), None).await.cuerpo, esperado.cuerpo);

    let alta_b = intento_lr::alta(&servidor, &b_cookie).await;
    let b = alta_b.cuerpo["intento_id"].as_str().unwrap();
    let ruta_b = format!("/api/intentos/{b}");
    let b_devolucion = format!("{ruta_b}/devolucion");
    assert_eq!(acceso(&b_cookie, &b_devolucion).await.cuerpo, json!({"estado":"no_disponible"}));
    assert!(publicacion::resolver_post_ensayo(&mut db, b, "omitido").is_err());
    intento_lr::resolver_pre(&servidor, b);
    let entregada_b = servidor.peticion("POST", &format!("{ruta_b}/entrega"), Some(&b_cookie), Some("t063-b-entrega"),
        Some(json!({"revision_esperada":1}))).await;
    assert_eq!(entregada_b.estado, StatusCode::CREATED);
    publicacion::resolver_post_ensayo(&mut db, b, "omitido").unwrap();
    assert!(trabajos::procesar_siguiente(&app).await.unwrap());
    let evaluacion_b: String = db.query_row("SELECT v.id FROM evaluaciones v JOIN entregas e ON e.id=v.entrega_id WHERE e.intento_id=?1",
        [b], |r| r.get(0)).unwrap();
    let sin_publicacion = acceso(&b_cookie, &b_devolucion).await;
    assert_eq!(sin_publicacion.cuerpo, json!({"estado":"pendiente"}));
    let publica_b = servidor.peticion("POST", &format!("/api/evaluaciones/{evaluacion_b}/publicacion"), Some(&docente),
        Some("t063-b-publica"), Some(json!({"revision":1}))).await;
    assert_eq!(publica_b.estado, StatusCode::CREATED);
    let visible_b = acceso(&b_cookie, &b_devolucion).await;
    assert_eq!(visible_b.cuerpo["estado"], "publicada");
    assert_eq!(visible_b.cuerpo["revision"], 1);
    assert_eq!(acceso(&a_cookie, &b_devolucion).await.estado, StatusCode::FORBIDDEN);
    supervisor.apagar().await;
    tarea.abort();
    println!("EVIDENCIA {}", json!({"matriz_t063":{"a":a,"b":b,"antes_publicar":inicial.cuerpo,"publicacion_con_post_pendiente":primera.cuerpo,
        "vista_pendiente":retenida.cuerpo,"post_respondido":visible_a.cuerpo,"publicacion_rectificacion":segunda.cuerpo,
        "visible_rectificada":actualizada.cuerpo,"post_omitido_sin_publicacion":sin_publicacion.cuerpo,"post_omitido_publicada":visible_b.cuerpo,
        "idempotencia_http":reenvio.estado.as_u16(),"acceso_cruzado_http":403}}));
}

#[tokio::test]
async fn bloquea_publicacion_de_la_incidencia_t061() {
    let servidor = match std::env::var("SIRCLE_T063_INCIDENCIA_DB") {
        Ok(db) => Servidor::copiar(Path::new(&db)),
        Err(_) => preparar_entrega().await,
    };
    let db = sqlite::abrir_existente(&servidor.db).unwrap();
    let retenida = std::env::var("SIRCLE_T063_INCIDENCIA_DB").is_ok();
    if !retenida {
        let entrega: String = db.query_row("SELECT id FROM entregas", [], |r| r.get(0)).unwrap();
        let contexto = evidencia::construir(&db, &entrega).unwrap().unwrap();
        let criterio = &contexto.rubrica.criterios[2];
        let incidencia = perfil::Incidencia { motivo:"acuse_sin_evento".into(), descuento_automatico:0,
            acuses:vec![perfil::AcuseSinEvento { tipo:TipoEvidencia::Orden, referencia:"orden-ausente".into(), clave:"acuse-ensayo".into() }] };
        let observaciones: Vec<_> = contexto.rubrica.criterios.iter().map(|c| Observacion {
            criterio_id:c.id.clone(),expectativa:c.expectativa.clone(),actuacion:"Actuación de ensayo".into(),evidencias:vec![],
            juicio:if c.id == criterio.id { Juicio::RevisionDocente } else { Juicio::Omision },
            explicacion:"Explicación de ensayo".into(),mejora:"Mejora de ensayo".into(),
            incidencia:(c.id == criterio.id).then(|| incidencia.clone()),
        }).collect();
        let resultado = Borrador { entrega_id:entrega.clone(),rubrica_sha256:contexto.entrada.rubrica.sha256.clone(),observaciones,prioridad:None };
        let texto = serde_json::to_string(&resultado).unwrap();
        let puntaje = serde_json::to_string(&sircle::modules::evaluacion::revisiones::calcular(&contexto.rubrica,&resultado,&HashMap::new()).unwrap()).unwrap();
        db.execute("INSERT INTO evaluaciones VALUES ('eval-incidente-ensayo',?1,'operacion-incidente-ensayo',?2,'2026-11-26T00:00:00Z')",
            params![entrega,texto]).unwrap();
        db.execute("INSERT INTO revisiones_evaluacion VALUES ('eval-incidente-ensayo',1,NULL,NULL,'Ensayo','2026-11-26T00:00:00Z',?1,?2,'[]')",
            params![texto,puntaje]).unwrap();
    }
    let id: String = db.query_row(
        "SELECT r.evaluacion_id FROM revisiones_evaluacion r, json_each(r.resultado,'$.observaciones') o
         WHERE json_extract(o.value,'$.incidencia.motivo')='acuse_sin_evento' LIMIT 1",
        [], |r| r.get(0),
    ).unwrap();
    let (docente, _) = servidor.entrar("docente-demo").await;
    let envio = servidor.peticion("POST", &format!("/api/evaluaciones/{id}/publicacion"), Some(&docente), Some("t063-incidencia"),
        Some(json!({"revision":1}))).await;
    assert_eq!(envio.estado, StatusCode::CONFLICT);
    assert_eq!(db.query_row("SELECT count(*) FROM publicaciones_evaluacion WHERE evaluacion_id=?1",[&id], |r| r.get::<_, i64>(0)).unwrap(),0);
    println!("EVIDENCIA {}", json!({"bloqueo_t063":{"evaluacion_id":id,"usa_base_t061":retenida,"http":envio.estado.as_u16(),
        "cuerpo":envio.cuerpo,"publicaciones":0}}));
}

const RUBRICA_E12: &str = include_str!("fixtures/rubricas/lr-e12-v1.json");

async fn calibrar_e12(fixture: &str) -> Value {
    let datos: Value = serde_json::from_str(fixture).unwrap();
    let mut servidor = Servidor::nuevo();
    let mut db = sqlite::abrir_existente(&servidor.db).unwrap();
    let rubrica = versiones_rubrica::cargar(&mut db, RUBRICA_E12);
    if let Some(esperada) = datos["rubrica_sha256"].as_str() {
        assert_eq!(rubrica.sha256, esperada);
    }
    rubricas::habilitar_ensayo(&db, rubrica.id).unwrap();
    let intento = versiones_rubrica::abrir(&servidor, intento_lr::ESTUDIANTE_LR, rubrica).await;
    intento_lr::resolver_pre(&servidor, &intento.id);
    let ruta = format!("/api/intentos/{}", intento.id);
    let inicio = servidor.peticion("POST", &format!("{ruta}/turnos"), Some(&intento.cookie), Some("e12-relato"),
        Some(json!({"sugerencia_id":"inicio","texto":"¿Desde cuándo comenzó?"}))).await;
    assert_eq!(inicio.estado, StatusCode::CREATED);
    let medicion = servidor.peticion("POST", &format!("{ruta}/mediciones"), Some(&intento.cookie), Some("e12-presion"),
        Some(json!({"instrumento":"presion","accion":"toma_completa"}))).await;
    assert_eq!(medicion.estado, StatusCode::CREATED);
    let orden = servidor.peticion("POST", &format!("{ruta}/ordenes"), Some(&intento.cookie), Some("e12-orden"),
        Some(json!({"examen":"ecg-inicial","accion":"confirmar"}))).await;
    assert_eq!(orden.estado, StatusCode::CREATED);
    let informe = servidor.consultar("GET", &format!("{ruta}/ordenes/{}", orden.cuerpo["orden_id"].as_str().unwrap()),
        Some(&intento.cookie), None).await;
    assert_eq!(informe.estado, StatusCode::OK);
    let referencias = HashMap::from([
        ("inicio", ("turno", &inicio.cuerpo["turno_id"])),
        ("presion", ("medicion", &medicion.cuerpo["medicion_id"])),
        ("ecg", ("informe", &informe.cuerpo["informe_id"])),
    ]);
    let actual = servidor.consultar("GET", &ruta, Some(&intento.cookie), None).await;
    let hipotesis: Vec<_> = datos["hipotesis"].as_array().unwrap().iter().map(|h| {
        let evidencias: Vec<_> = h["registros"].as_array().unwrap().iter().map(|nombre| {
            let (tipo, origen_id) = referencias[nombre.as_str().unwrap()];
            json!({"tipo":tipo,"origen_id":origen_id})
        }).collect();
        json!({"rol":h["rol"],"titulo":h["titulo"],"explicacion":h["explicacion"],"evidencias":evidencias})
    }).collect();
    let borrador = servidor.peticion("PUT", &format!("{ruta}/razonamiento"), Some(&intento.cookie), Some("e12-hipotesis"),
        Some(json!({"revision_esperada":actual.cuerpo["revision"],"hipotesis":hipotesis}))).await;
    assert_eq!(borrador.estado, StatusCode::OK);
    let acuse = servidor.peticion("POST", &format!("{ruta}/entrega"), Some(&intento.cookie), Some("e12-entrega"),
        Some(json!({"revision_esperada":borrador.cuerpo["revision"]}))).await;
    assert_eq!(acuse.estado, StatusCode::CREATED);
    assert_eq!(acuse.cuerpo["rubrica_sha256"], intento.rubrica.sha256);

    let url = std::env::var("SIRCLE_OLLAMA_URL").unwrap_or_else(|_| "http://127.0.0.1:11434".into());
    let supervisor = Supervisor::iniciar(Path::new(env!("CARGO_BIN_EXE_sircle")), &url).await.unwrap();
    let app = Estado::con_base(servidor.db.clone()).unwrap().con_trabajadores(supervisor.clone());
    servidor.app = router(app.clone());
    let consulta = trabajos::procesar_siguiente(&app);
    assert!(timeout(Duration::from_secs(540), consulta).await.unwrap().unwrap());
    let (evaluacion, operacion): (String, String) = db.query_row(
        "SELECT id,operacion_id FROM evaluaciones WHERE entrega_id=?1",
        [acuse.cuerpo["entrega_id"].as_str().unwrap()], |r| Ok((r.get(0)?,r.get(1)?)),
    ).unwrap();
    let (docente, _) = servidor.entrar("docente-demo").await;
    let ruta_evaluacion = format!("/api/evaluaciones/{evaluacion}");
    let resultado = servidor.consultar("GET", &ruta_evaluacion, Some(&docente), None).await;
    let traza = servidor.consultar("GET", &format!("{ruta_evaluacion}/traza"), Some(&docente), None).await;
    assert_eq!(resultado.estado, StatusCode::OK);
    assert_eq!(traza.estado, StatusCode::OK);
    let cliente = reqwest::Client::builder().no_proxy().timeout(Duration::from_secs(4)).build().unwrap();
    let modelos: Value = cliente.get(format!("{url}/api/ps")).send().await.unwrap().error_for_status().unwrap().json().await.unwrap();
    let motor = modelos["models"].as_array().unwrap().iter().find(|m| m["name"] == MODELO).unwrap();
    assert_eq!(motor["size_vram"], 0);
    assert_eq!(traza.cuerpo["invocacion"]["modelo"], MODELO);
    assert_eq!(traza.cuerpo["invocacion"]["digest"], motor["digest"]);
    assert_eq!(traza.cuerpo["invocacion"]["operacion_id"], operacion);
    assert_eq!(traza.cuerpo["entrega"]["id"], acuse.cuerpo["entrega_id"]);
    assert_eq!(traza.cuerpo["rubrica"]["sha256"], intento.rubrica.sha256);
    assert_eq!(traza.cuerpo["evaluacion_id"], resultado.cuerpo["evaluacion_id"]);
    assert_eq!(traza.cuerpo["borrador_guardado"], resultado.cuerpo["borrador"]);

    let criterios = resultado.cuerpo["borrador"]["observaciones"].as_array().unwrap();
    let salida = traza.cuerpo["invocacion"]["seleccion"]["observaciones"].as_array().unwrap();
    let cotejo: Vec<_> = datos["clave"].as_array().unwrap().iter().map(|esperada| {
        let guardada = criterios.iter().find(|c| c["criterio_id"] == esperada["criterio"]).unwrap();
        let devuelta = salida.iter().find(|c| c["criterio_id"] == esperada["criterio"]).unwrap();
        assert_eq!(guardada["actuacion"], devuelta["actuacion"]);
        assert_eq!(guardada["juicio"], devuelta["juicio"]);
        assert_eq!(guardada["evidencias"], devuelta["evidencias"]);
        let id_esperado = esperada["registro"].as_str().map(|refe| referencias[refe].1);
        let cita_coincide = id_esperado.map(|id| guardada["evidencias"].as_array().unwrap().contains(id));
        json!({"clave":esperada,"registro_esperado":id_esperado,"juicio_coincide":guardada["juicio"]==esperada["juicio"],
            "cita_coincide":cita_coincide,"observacion":guardada})
    }).collect();
    supervisor.apagar().await;
    json!({"resultado":resultado.cuerpo,"acuse":acuse.cuerpo,"cotejo":cotejo,
        "traza":traza.cuerpo,"entradas_clinicas":{"turno":inicio.cuerpo,"medicion":medicion.cuerpo,"informe":informe.cuerpo},
        "ollama_ps":motor})
}

#[tokio::test]
#[ignore = "requiere Ollama local y qwen3.5:4b en CPU"]
async fn calibra_tres_de_cuatro_con_qwen() {
    let evidencia = calibrar_e12(include_str!("fixtures/evaluacion/tres_de_cuatro.json")).await;
    println!("EVIDENCIA {}", evidencia);
    for cotejo in evidencia["cotejo"].as_array().unwrap() {
        assert_eq!(cotejo["juicio_coincide"], true, "{cotejo}");
        assert_eq!(cotejo["cita_coincide"], true, "{cotejo}");
    }
    let mejora = evidencia["cotejo"][3]["observacion"]["mejora"].as_str().unwrap();
    assert!(mejora.contains("Revisa el registro citado y anota"), "{mejora}");
}

#[tokio::test]
#[ignore = "requiere Ollama local y qwen3.5:4b en CPU"]
async fn calibra_uno_de_cuatro_y_prioriza_ecg_con_qwen() {
    let evidencia = calibrar_e12(include_str!("fixtures/evaluacion/uno_de_cuatro.json")).await;
    println!("EVIDENCIA {}", evidencia);
    for cotejo in evidencia["cotejo"].as_array().unwrap() {
        assert_eq!(cotejo["juicio_coincide"], true, "{cotejo}");
        assert_eq!(cotejo["cita_coincide"], true, "{cotejo}");
    }
    let observaciones = &evidencia["cotejo"];
    let segunda = observaciones[1]["observacion"]["explicacion"].as_str().unwrap().to_lowercase();
    assert!(segunda.contains("fibrilación auricular") && segunda.contains("taquicardia sinusal"), "{segunda}");
    let tercera = observaciones[2]["observacion"]["mejora"].as_str().unwrap();
    assert!(tercera.contains("qué hallazgo apoyaría o debilitaría"), "{tercera}");
    let cuarta = observaciones[3]["observacion"]["mejora"].as_str().unwrap();
    assert!(cuarta.contains("lo referido en el turno de lo medido"), "{cuarta}");
    let prioridad = &evidencia["resultado"]["borrador"]["prioridad"];
    assert_eq!(prioridad["criterio_id"], "hipotesis-2");
    assert_eq!(prioridad["evidencia_id"], evidencia["entradas_clinicas"]["informe"]["informe_id"]);
    assert!(prioridad["motivo"].as_str().unwrap().contains("taquicardia sinusal"));
    assert!(prioridad["accion"].as_str().unwrap().contains("Relee el informe"));
}

fn ensayo() -> (Entrada, Borrador) {
    let expectativa = "Explicitar qué hallazgo respalda la hipótesis.".to_owned();
    let huella = "a".repeat(64);
    let entrada = Entrada {
        entrega: Entrega { id: "entrega-ensayo".into(), intento_id: "intento-ensayo".into(), revision: 4,
            caso: "P01".into(), version_caso: "caso-ensayo-v1".into(), confirmada: true },
        rubrica: RubricaFijada { id: "rubrica-ensayo".into(), version: "1".into(), sha256: huella.clone(),
            nivel: perfil::NIVEL.into(), criterios: vec![Criterio { id: "fundamento".into(), expectativa: expectativa.clone() }] },
        evidencias: vec![Evidencia { id: "turno-ensayo".into(), intento_id: "intento-ensayo".into(),
            tipo: TipoEvidencia::Turno, resumen: "Declaración de ensayo sobre la cronología.".into() }],
        hipotesis: vec![Hipotesis { rol: "principal".into(), titulo: "Hipótesis de ensayo".into(),
            explicacion: "Relaciono el relato registrado con el inicio.".into(), evidencias: vec!["turno-ensayo".into()] }],
    };
    let borrador = Borrador {
        entrega_id: entrada.entrega.id.clone(), rubrica_sha256: huella,
        prioridad: None,
        observaciones: vec![Observacion { criterio_id: "fundamento".into(),
            actuacion: "Relacionó su hipótesis con el relato.".into(), evidencias: vec!["turno-ensayo".into()],
            expectativa, juicio: Juicio::Acierto, explicacion: "El registro tiene su origen indicado.".into(),
            mejora: "Explicitar qué cambiaría su hipótesis.".into(), incidencia: None }],
    };
    (entrada, borrador)
}

#[test]
fn evalua_solo_entrega_confirmada() {
    let (mut entrada, borrador) = ensayo();
    perfil::validar_entrada(&entrada).unwrap();
    perfil::validar_borrador(&entrada, &borrador).unwrap();
    entrada.entrega.confirmada = false;
    assert!(perfil::validar_entrada(&entrada).is_err());
}

#[test]
fn cita_la_entrega_y_su_version() {
    let (entrada, mut borrador) = ensayo();
    perfil::validar_borrador(&entrada, &borrador).unwrap();
    borrador.rubrica_sha256 = "b".repeat(64);
    assert!(perfil::validar_borrador(&entrada, &borrador).is_err());
    borrador.rubrica_sha256 = entrada.rubrica.sha256.clone();
    borrador.observaciones[0].evidencias = vec!["turno-de-otra-entrega".into()];
    assert!(perfil::validar_borrador(&entrada, &borrador).is_err());
}

#[tokio::test]
async fn propuesta_no_publica() {
    let (_, borrador) = ensayo();
    let mut datos = serde_json::to_value(borrador).unwrap();
    datos["publicada"] = json!(true);
    assert!(serde_json::from_value::<Borrador>(datos).is_err());
    let servidor = Servidor::nuevo();
    let (estudiante, _) = servidor.entrar(intento_lr::ESTUDIANTE_LR).await;
    let alta = intento_lr::alta(&servidor, &estudiante).await;
    let id = alta.cuerpo["intento_id"].as_str().unwrap();
    let (docente, _) = servidor.entrar("docente-demo").await;
    let cabeceras = |cookie: &str| {
        let mut resultado = HeaderMap::new();
        resultado.insert(header::COOKIE, HeaderValue::from_str(cookie).unwrap());
        resultado
    };
    let db = sqlite::abrir_existente(&servidor.db).unwrap();
    match autorizacion::docente_del_intento(&db, &cabeceras(&estudiante), id) {
        Ok(_) => panic!("Se autorizó a estudiante como docente"),
        Err(error) => assert_eq!(error.estado, StatusCode::FORBIDDEN),
    }
    assert_eq!(autorizacion::docente_del_intento(&db, &cabeceras(&docente), id).unwrap().id, "docente-demo");
}

#[test]
fn excluye_entregas_y_encuestas_ajenas() {
    let (mut entrada, _) = ensayo();
    let mut datos = serde_json::to_value(&entrada).unwrap();
    datos["encuestas"] = json!([{"clave":"ensayo"}]);
    assert!(serde_json::from_value::<Entrada>(datos).is_err());
    entrada.evidencias[0].intento_id = "intento-ajeno".into();
    assert!(perfil::validar_entrada(&entrada).is_err());
}

#[test]
fn valida_cinco_partes_de_la_observacion() {
    let (entrada, mut borrador) = ensayo();
    let primero = borrador.observaciones[0].clone();
    for campo in ["actuacion", "expectativa", "explicacion", "mejora", "evidencias"] {
        let mut alterado = serde_json::to_value(&primero).unwrap();
        alterado[campo] = if campo == "evidencias" { json!([]) } else { json!("") };
        borrador.observaciones[0] = serde_json::from_value(alterado).unwrap();
        assert!(perfil::validar_borrador(&entrada, &borrador).is_err(), "{campo}");
    }
    borrador.observaciones[0] = primero;
    assert!(perfil::validar_borrador(&entrada, &borrador).is_ok());
}

#[test]
fn solo_correlaciona_borrador_con_su_entrega() {
    let (entrada, mut borrador) = ensayo();
    let otra = borrador.entrega_id.clone();
    borrador.entrega_id = "otra-entrega".into();
    assert!(perfil::validar_borrador(&entrada, &borrador).is_err());
    borrador.entrega_id = otra;
    assert!(perfil::validar_borrador(&entrada, &borrador).is_ok());
}

#[test]
fn correlaciona_el_evaluador() {
    let (entrada, borrador) = ensayo();
    let cabecera = Cabecera { operacion_id: "evaluacion-ensayo".into(), intento_id: entrada.entrega.intento_id.clone(),
        agente: perfil::AGENTE, caso: entrada.entrega.caso.clone(), version: entrada.entrega.version_caso.clone(),
        fase: "entregado".into(), hablante: "Evaluador".into() };
    let solicitud = Solicitud { cabecera: cabecera.clone(), contenido: entrada };
    let respuesta = Respuesta { cabecera: cabecera.clone(), tipo: TipoResultado::Evaluacion, resultado: borrador };
    let mensaje: Value = serde_json::to_value(&solicitud).unwrap();
    let acuse = serde_json::to_value(&respuesta).unwrap();
    let decodificada: Respuesta<Borrador> = serde_json::from_value(acuse.clone()).unwrap();
    perfil::validar_borrador(&solicitud.contenido, &decodificada.resultado).unwrap();
    assert_eq!(mensajes::discrepancia(&cabecera, &decodificada.cabecera), None);
    let otra = Cabecera { agente: Agente::Lr, ..cabecera.clone() };
    assert_eq!(mensajes::discrepancia(&otra, &decodificada.cabecera), Some("agente_no_coincide"));
    println!("EVIDENCIA {}", json!({ "origen": "fixture de contrato", "entrada": mensaje, "borrador": acuse }));
}

#[test]
fn conserva_referencias_para_revision() {
    let (entrada, borrador) = ensayo();
    perfil::validar_borrador(&entrada, &borrador).unwrap();
    let criterio = &borrador.observaciones[0];
    assert_eq!(criterio.criterio_id, entrada.rubrica.criterios[0].id);
    assert_eq!(criterio.evidencias, [entrada.evidencias[0].id.clone()]);
    assert_eq!(borrador.rubrica_sha256, entrada.rubrica.sha256);
}
