mod fixtures;

use axum::http::StatusCode;
use fixtures::{Servidor, intento_lr};
use rusqlite::TransactionBehavior;
use serde_json::json;
use sircle::{
    agentes::lr::{enrutamiento, entrevista, perfil},
    modules::{
        entrevista::{hechos, ollama::{Invocacion, Seleccion}},
        intentos::intento,
    },
    sqlite,
};

const PREGUNTA: &str = "¿Desde cuándo te cuesta respirar?";

#[tokio::test]
async fn confirma_solo_hechos_vigentes_pertinentes_y_del_actor_lr() {
    let servidor = Servidor::nuevo();
    let (cookie, _) = servidor.entrar(intento_lr::ESTUDIANTE_LR).await;
    let alta = intento_lr::alta(&servidor, &cookie).await;
    let id = alta.cuerpo["intento_id"].as_str().unwrap();
    intento_lr::resolver_pre(&servidor, id);
    let mut db = sqlite::abrir_existente(&servidor.db).unwrap();
    let inicial = intento::recuperar(&db, id, intento_lr::ESTUDIANTE_LR).unwrap();
    let permitidos = hechos::candidatos_lr(&db, &inicial).unwrap();
    let tx = db.transaction_with_behavior(TransactionBehavior::Immediate).unwrap();
    let confirmado = entrevista::confirmar(&tx, &inicial, &permitidos, PREGUNTA, &invocacion("valido", "lr-inicio")).unwrap();
    assert_eq!(confirmado.hablante, "LR");
    assert!(confirmado.respuesta.contains("hace dos días"));
    assert_eq!(confirmado.orden, 1);
    tx.commit().unwrap();

    let actual = intento::recuperar(&db, id, intento_lr::ESTUDIANTE_LR).unwrap();
    let permitido = permitidos.iter().find(|hecho| hecho.id == "lr-inicio").unwrap();
    let mut ajeno = permitido.clone();
    ajeno.escenario = "P02".into();
    let mut otra_fase = permitido.clone();
    otra_fase.fase = "posterior".into();
    let mut otra_voz = permitido.clone();
    otra_voz.hablante = "Acompañante".into();

    let mut rechazos = Vec::new();
    for (caso, conjunto, seleccionado) in [
        ("otro_caso", vec![ajeno], "lr-inicio"),
        ("otra_fase", vec![otra_fase], "lr-inicio"),
        ("otro_hablante", vec![otra_voz], "lr-inicio"),
        ("id_ausente", permitidos.clone(), "fuera-de-la-ficha"),
        ("no_pertinente", permitidos.clone(), "lr-antecedentes-familiares"),
        ("omision_del_hecho_disponible", permitidos.clone(), "sin_hecho"),
    ] {
        let tx = db.transaction_with_behavior(TransactionBehavior::Immediate).unwrap();
        let resultado = entrevista::confirmar(&tx, &actual, &conjunto, PREGUNTA, &invocacion(caso, seleccionado));
        let error = match resultado {
            Err(error) => error,
            Ok(_) => panic!("Se confirmó el caso {caso}"),
        };
        assert_eq!(error.estado, StatusCode::SERVICE_UNAVAILABLE);
        rechazos.push(json!({"caso": caso, "http": error.estado.as_u16(), "mensaje": error.mensaje}));
    }
    db.execute(
        "UPDATE intentos SET estado='entregado', revision=revision+1 WHERE id=?1",
        [id],
    ).unwrap();
    let tx = db.transaction_with_behavior(TransactionBehavior::Immediate).unwrap();
    let resultado = entrevista::confirmar(&tx, &actual, &permitidos, PREGUNTA, &invocacion("cierre-durante-llamada", "lr-inicio"));
    let error = match resultado {
        Err(error) => error,
        Ok(_) => panic!("Se confirmó un turno después de entregar"),
    };
    assert_eq!(error.estado, StatusCode::CONFLICT);
    rechazos.push(json!({"caso": "cierre-durante-llamada", "http": error.estado.as_u16(), "mensaje": error.mensaje}));
    drop(tx);

    let ruta = format!("/api/intentos/{id}");
    let vista = servidor.consultar("GET", &ruta, Some(&cookie), None).await;
    assert_eq!(vista.estado, StatusCode::OK);
    assert_eq!(vista.cuerpo["turnos"], json!([confirmado]));
    let traza = format!("{ruta}/turnos/{}/traza", confirmado.turno_id);
    let privada = servidor.consultar("GET", &traza, Some(&cookie), None).await;
    assert_eq!(privada.estado, StatusCode::FORBIDDEN);
    assert!(privada.cuerpo.get("hecho").is_none());
    let (docente, _) = servidor.entrar("docente-demo").await;
    let autorizada = servidor.consultar("GET", &traza, Some(&docente), None).await;
    assert_eq!(autorizada.estado, StatusCode::OK);
    assert_eq!(autorizada.cuerpo["operacion_id"], "valido");
    assert_eq!(autorizada.cuerpo["hecho"]["id"], "lr-inicio");
    println!("EVIDENCIA {}", json!({
        "intento_id": id,
        "confirmado": confirmado,
        "rechazos": rechazos,
        "historial_final": vista.cuerpo["turnos"],
    }));
}

fn invocacion(operacion: &str, hecho: &str) -> Invocacion {
    Invocacion {
        operacion_id: operacion.into(),
        inicio_unix_ms: 1000,
        fin_unix_ms: 1001,
        modelo: "fixture-T014".into(),
        digest: "fixture-sin-inferencia".into(),
        solicitud: json!({"texto": PREGUNTA}),
        seleccion: Seleccion { hecho_id: hecho.into() },
        respuesta_cruda: json!({"hecho_id": hecho}).to_string(),
        duracion_motor_ns: None,
        carga_modelo_ns: None,
        tokens_respuesta: None,
    }
}

async fn abrir_lr(servidor: &Servidor, identidad: &str) -> (String, String) {
    let (cookie, _) = servidor.entrar(identidad).await;
    let alta = intento_lr::alta(servidor, &cookie).await;
    assert_eq!(alta.estado, StatusCode::CREATED);
    let id = alta.cuerpo["intento_id"].as_str().unwrap().to_owned();
    intento_lr::resolver_pre(servidor, &id);
    (cookie, id)
}

async fn literal(servidor: &Servidor, cookie: &str, id: &str) -> fixtures::Respuesta {
    servidor.peticion("POST", &format!("/api/intentos/{id}/turnos"), Some(cookie), Some("literal-lr"),
        Some(json!({"sugerencia_id":"inicio","texto":"¿Desde cuándo comenzó?"}))).await
}

#[tokio::test]
async fn mantiene_voz_de_lr() {
    let servidor = Servidor::nuevo();
    let (_, id) = abrir_lr(&servidor, intento_lr::ESTUDIANTE_LR).await;
    let mut db = sqlite::abrir_existente(&servidor.db).unwrap();
    let mut intento = intento::recuperar(&db, &id, intento_lr::ESTUDIANTE_LR).unwrap();
    assert_eq!(enrutamiento::destino(&intento).unwrap(), (perfil::AGENTE, perfil::HABLANTE));
    let hechos = hechos::candidatos_lr(&db, &intento).unwrap();
    assert!(hechos.iter().all(|h| h.hablante == perfil::HABLANTE && h.fuente == perfil::FUENTE_ACTOR));
    let tx = db.transaction().unwrap();
    let salida = entrevista::confirmar(&tx, &intento, &hechos, "¿Qué nota me pondrías?",
        &invocacion("peticion-nota", "lr-inicio"));
    assert!(matches!(salida, Err(error) if error.estado == StatusCode::SERVICE_UNAVAILABLE));
    drop(tx);
    intento.caso = "P02".into();
    assert_eq!(enrutamiento::destino(&intento).unwrap_err().estado, StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn cita_desconocimiento_familiar() {
    let servidor = Servidor::nuevo();
    let (_, id) = abrir_lr(&servidor, intento_lr::ESTUDIANTE_LR).await;
    let mut db = sqlite::abrir_existente(&servidor.db).unwrap();
    let intento = intento::recuperar(&db, &id, intento_lr::ESTUDIANTE_LR).unwrap();
    let hechos = hechos::candidatos_lr(&db, &intento).unwrap();
    let tx = db.transaction().unwrap();
    let turno = entrevista::confirmar(&tx, &intento, &hechos, "¿Conoces los antecedentes de tu familia?",
        &invocacion("familia-fixture", "lr-antecedentes-familiares")).unwrap();
    assert_eq!(turno.hablante, "LR");
    assert!(turno.respuesta.starts_with("No conozco") && turno.respuesta.contains("fui adoptada"));
    tx.commit().unwrap();
    let (revisor, _) = servidor.entrar("docente-demo").await;
    let traza = servidor.consultar("GET", &format!("/api/intentos/{id}/turnos/{}/traza", turno.turno_id), Some(&revisor), None).await;
    assert_eq!(traza.estado, StatusCode::OK);
    assert_eq!(traza.cuerpo["hecho"]["pasaje"], "Family Hx: unknown, patient was adopted.");
    assert_eq!(traza.cuerpo["hecho"]["version"], intento.version_caso);
}

#[tokio::test]
async fn preguntar_por_presion_no_mide() {
    let servidor = Servidor::nuevo();
    let (cookie, id) = abrir_lr(&servidor, intento_lr::ESTUDIANTE_LR).await;
    let ruta = format!("/api/intentos/{id}");
    let pregunta = servidor.peticion("POST", &format!("{ruta}/turnos"), Some(&cookie), Some("pregunta-presion"),
        Some(json!({"texto":"¿Cuál es tu presión?"}))).await;
    assert_eq!(pregunta.estado, StatusCode::SERVICE_UNAVAILABLE);
    let medidas = servidor.consultar("GET", &format!("{ruta}/mediciones"), Some(&cookie), None).await;
    assert_eq!(medidas.cuerpo, json!([]));
}

#[tokio::test]
async fn aisla_dos_sugerencias_simultaneas() {
    let servidor = Servidor::nuevo();
    let db = sqlite::abrir_existente(&servidor.db).unwrap();
    db.execute("INSERT INTO asignaciones VALUES ('lr-ensayo','estudiante-sin-asignacion','docente-demo')", []).unwrap();
    let (a, id_a) = abrir_lr(&servidor, intento_lr::ESTUDIANTE_LR).await;
    let (b, id_b) = abrir_lr(&servidor, "estudiante-sin-asignacion").await;
    let (ta, tb) = tokio::join!(literal(&servidor, &a, &id_a), literal(&servidor, &b, &id_b));
    for (id, turno, cookie) in [(&id_a, &ta, &a), (&id_b, &tb, &b)] {
        assert_eq!(turno.estado, StatusCode::CREATED);
        assert_eq!(turno.cuerpo["intento_id"], *id);
        let vista = servidor.consultar("GET", &format!("/api/intentos/{id}"), Some(cookie), None).await;
        assert_eq!(vista.cuerpo["turnos"], json!([turno.cuerpo]));
    }
    assert_ne!(ta.cuerpo["turno_id"], tb.cuerpo["turno_id"]);
}

#[tokio::test]
async fn distingue_sugerencia_y_texto_editado() {
    let servidor = Servidor::nuevo();
    let (cookie, id) = abrir_lr(&servidor, intento_lr::ESTUDIANTE_LR).await;
    let servicio = format!("/api/intentos/{id}/turnos");
    let editada = servidor.peticion("POST", &servicio, Some(&cookie), Some("editada"),
        Some(json!({"sugerencia_id":"inicio","texto":"¿Desde cuándo comenzó la falta de aire?"}))).await;
    assert_eq!(editada.estado, StatusCode::SERVICE_UNAVAILABLE);
    assert_eq!(literal(&servidor, &cookie, &id).await.estado, StatusCode::CREATED);
}

#[tokio::test]
async fn sugerencia_exacta_con_motor_caido() {
    let servidor = Servidor::nuevo();
    let (cookie, id) = abrir_lr(&servidor, intento_lr::ESTUDIANTE_LR).await;
    let primera = literal(&servidor, &cookie, &id).await;
    let reenvio = literal(&servidor, &cookie, &id).await;
    assert_eq!(primera.estado, StatusCode::CREATED);
    assert_eq!(reenvio.estado, StatusCode::OK);
    assert_eq!(primera.cuerpo, reenvio.cuerpo);
    assert_eq!(primera.cuerpo["hablante"], "LR");
}

#[tokio::test]
async fn separa_turno_medicion_y_orden() {
    let servidor = Servidor::nuevo();
    let (cookie, id) = abrir_lr(&servidor, intento_lr::ESTUDIANTE_LR).await;
    let ruta = format!("/api/intentos/{id}");
    assert_eq!(literal(&servidor, &cookie, &id).await.estado, StatusCode::CREATED);
    let inicial = servidor.consultar("GET", &ruta, Some(&cookie), None).await;
    assert_eq!(inicial.cuerpo["mediciones"], json!([]));
    assert_eq!(inicial.cuerpo["ordenes"], json!([]));
    let medida = servidor.peticion("POST", &format!("{ruta}/mediciones"), Some(&cookie), Some("toma"),
        Some(json!({"instrumento":"presion","accion":"toma_completa"}))).await;
    assert_eq!(medida.estado, StatusCode::CREATED);
    let orden = servidor.peticion("POST", &format!("{ruta}/ordenes"), Some(&cookie), Some("orden"),
        Some(json!({"examen":"ecg-inicial","accion":"confirmar"}))).await;
    assert_eq!(orden.estado, StatusCode::CREATED);
    let finalizada = servidor.consultar("GET", &ruta, Some(&cookie), None).await;
    assert_eq!(finalizada.cuerpo["turnos"], inicial.cuerpo["turnos"]);
    assert_eq!(finalizada.cuerpo["mediciones"], json!([medida.cuerpo]));
    assert_eq!(finalizada.cuerpo["ordenes"], json!([orden.cuerpo]));
}

#[tokio::test]
async fn localiza_origen_de_turno_lr() {
    let servidor = Servidor::nuevo();
    let (cookie, id) = abrir_lr(&servidor, intento_lr::ESTUDIANTE_LR).await;
    let turno = literal(&servidor, &cookie, &id).await;
    let t = &turno.cuerpo;
    let traza = format!("/api/intentos/{id}/turnos/{}/traza", t["turno_id"].as_str().unwrap());
    assert_eq!(servidor.consultar("GET", &traza, Some(&cookie), None).await.estado, StatusCode::FORBIDDEN);
    let (docente, _) = servidor.entrar("docente-demo").await;
    let leida = servidor.consultar("GET", &traza, Some(&docente), None).await;
    assert_eq!(leida.estado, StatusCode::OK);
    assert_eq!(leida.cuerpo["operacion_id"], t["operacion_id"]);
    assert_eq!(leida.cuerpo["hecho"]["fuente"], perfil::FUENTE_ACTOR);
    assert_eq!(leida.cuerpo["hecho"]["fase"], "inicial");
    assert_eq!(leida.cuerpo["hecho"]["linea"], 7);
}

#[tokio::test]
async fn pide_referente_sin_crear_hallazgo() {
    let servidor = Servidor::nuevo();
    let (cookie, id) = abrir_lr(&servidor, intento_lr::ESTUDIANTE_LR).await;
    let ruta = format!("/api/intentos/{id}");
    let antes = servidor.consultar("GET", &ruta, Some(&cookie), None).await;
    assert_eq!(antes.cuerpo["turnos"], json!([]));
    let mut db = sqlite::abrir_existente(&servidor.db).unwrap();
    let intento = intento::recuperar(&db, &id, intento_lr::ESTUDIANTE_LR).unwrap();
    let hechos = hechos::candidatos_lr(&db, &intento).unwrap();
    let tx = db.transaction_with_behavior(TransactionBehavior::Immediate).unwrap();
    let turno = entrevista::confirmar(&tx, &intento, &hechos, "¿Y eso?", &invocacion("t047-aclaracion", "sin_hecho")).unwrap();
    assert_eq!(turno.hablante, "LR");
    assert_eq!(turno.respuesta, "¿A qué te refieres con eso?");
    assert_eq!(turno.tipo_respuesta, "aclaracion");
    tx.commit().unwrap();
    let despues = servidor.consultar("GET", &ruta, Some(&cookie), None).await;
    assert_eq!(despues.cuerpo["turnos"], json!([turno]));
    assert_eq!(despues.cuerpo["evidencias"], antes.cuerpo["evidencias"]);
    assert_eq!(despues.cuerpo["ficha"], antes.cuerpo["ficha"]);
    let (docente, _) = servidor.entrar("docente-demo").await;
    let detalle = servidor.consultar("GET", &format!("{ruta}/turnos/{}/traza", turno.turno_id), Some(&docente), None).await;
    assert_eq!(detalle.cuerpo["motivo"], "referente_ambiguo");
    assert!(detalle.cuerpo["hecho"].is_null());
    let mut db = sqlite::abrir_existente(&servidor.db).unwrap();
    let intento = intento::recuperar(&db, &id, intento_lr::ESTUDIANTE_LR).unwrap();
    let tx = db.transaction().unwrap();
    assert!(entrevista::confirmar(&tx, &intento, &hechos, "¿Y eso?", &invocacion("t047-repeticion", "sin_hecho")).is_err());
}

#[tokio::test]
async fn distingue_los_limites_del_relato() {
    for (pregunta, seleccionado, voz, tipo, nuevas, aviso) in [
        ("¿Y eso?", "sin_hecho", "LR", "aclaracion", 0, None),
        ("¿Conoces los antecedentes de tu familia?", "lr-antecedentes-familiares", "LR", "hecho", 1, None),
        ("¿Cuál es su grupo sanguíneo?", "sin_hecho", "Sistema", "dato_ausente", 0,
            Some("El grupo sanguíneo no está disponible en este escenario.")),
        ("¿Cuál es tu color favorito?", "sin_hecho", "Sistema", "dato_ausente", 0,
            Some("Ese dato no está disponible en este escenario.")),
    ] {
        let servidor = Servidor::nuevo();
        let (cookie, id) = abrir_lr(&servidor, intento_lr::ESTUDIANTE_LR).await;
        let ruta = format!("/api/intentos/{id}");
        let antes = servidor.consultar("GET", &ruta, Some(&cookie), None).await;
        let mut db = sqlite::abrir_existente(&servidor.db).unwrap();
        let intento = intento::recuperar(&db, &id, intento_lr::ESTUDIANTE_LR).unwrap();
        let hechos = hechos::candidatos_lr(&db, &intento).unwrap();
        let tx = db.transaction_with_behavior(TransactionBehavior::Immediate).unwrap();
        let turno = entrevista::confirmar(&tx, &intento, &hechos, pregunta, &invocacion("limite-relato", seleccionado)).unwrap();
        assert_eq!((turno.hablante.as_str(), turno.tipo_respuesta.as_str()), (voz, tipo));
        tx.commit().unwrap();
        let despues = servidor.consultar("GET", &ruta, Some(&cookie), None).await;
        assert_eq!(despues.cuerpo["turnos"], json!([turno]));
        assert_eq!(despues.cuerpo["ficha"], antes.cuerpo["ficha"]);
        assert_eq!(despues.cuerpo["evidencias"].as_array().unwrap().len() - antes.cuerpo["evidencias"].as_array().unwrap().len(), nuevas);
        if let Some(aviso) = aviso {
            assert_eq!(turno.respuesta, aviso);
            let (docente, _) = servidor.entrar("docente-demo").await;
            let detalle = servidor.consultar("GET", &format!("{ruta}/turnos/{}/traza", turno.turno_id), Some(&docente), None).await;
            assert_eq!(detalle.cuerpo["motivo"], "dato_no_registrado");
            assert!(detalle.cuerpo["hecho"].is_null());
        }
    }
}

#[tokio::test]
#[ignore = "requiere Axum con los cuatro trabajadores y Ollama local"]
async fn aisla_dos_preguntas_simultaneas() {
    use std::{fs, path::PathBuf, process::Command, time::Duration};
    use reqwest::{Client, header};
    use serde_json::Value;

    async fn sesion(cliente: &Client, origen: &str, identidad: &str) -> String {
        let respuesta = cliente.post(format!("{origen}/api/sesiones")).json(&json!({"identidad": identidad}))
            .send().await.unwrap();
        assert_eq!(respuesta.status(), StatusCode::CREATED);
        respuesta.headers()[header::SET_COOKIE].to_str().unwrap().split(';').next().unwrap().to_owned()
    }
    async fn vista(cliente: &Client, ruta: &str, cookie: &str) -> Value {
        cliente.get(ruta).header(header::COOKIE, cookie).send().await.unwrap()
            .error_for_status().unwrap().json().await.unwrap()
    }
    async fn enviar(cliente: &Client, ruta: &str, cookie: &str, texto: &str) -> Value {
        let respuesta = cliente.post(format!("{ruta}/turnos")).header(header::COOKIE, cookie)
            .header("Idempotency-Key", "t052-pregunta").json(&json!({"texto": texto})).send().await.unwrap();
        let estado = respuesta.status();
        let cuerpo: Value = respuesta.json().await.unwrap();
        assert_eq!(estado, StatusCode::CREATED, "{cuerpo}");
        cuerpo
    }

    let origen = std::env::var("SIRCLE_URL").unwrap_or_else(|_| "http://127.0.0.1:18081".into());
    let base = PathBuf::from(std::env::var_os("SIRCLE_T052_DB").expect("Indica la base temporal del servidor."));
    let log = PathBuf::from(std::env::var_os("SIRCLE_T052_LOG").expect("Indica el registro de Axum."));
    let cliente = Client::builder().no_proxy().timeout(Duration::from_secs(390)).build().unwrap();
    let identidades = ["estudiante-lr", "estudiante-sin-asignacion"];
    let mut sesiones = Vec::new();
    let mut rutas = Vec::new();
    let mut ids = Vec::new();
    for identidad in identidades {
        let cookie = sesion(&cliente, &origen, identidad).await;
        let respuesta = cliente.post(format!("{origen}/api/intentos")).header(header::COOKIE, &cookie)
            .header("Idempotency-Key", "t052-alta").json(&json!({"actividad_id":"lr-ensayo"})).send().await.unwrap();
        assert_eq!(respuesta.status(), StatusCode::CREATED);
        let alta: Value = respuesta.json().await.unwrap();
        let id = alta["intento_id"].as_str().unwrap();
        let comando = Command::new(env!("CARGO_BIN_EXE_sircle")).arg("resolver-pre-ensayo")
            .arg("--db").arg(&base).arg("--intento").arg(id).output().unwrap();
        assert!(comando.status.success(), "{}", String::from_utf8_lossy(&comando.stderr));
        let ruta = format!("{origen}/api/intentos/{id}");
        assert_eq!(vista(&cliente, &ruta, &cookie).await["turnos"], json!([]));
        sesiones.push(cookie);
        rutas.push(ruta);
        ids.push(id.to_owned());
    }
    let preguntas = ["¿Desde cuándo comenzó la falta de aire?", "¿Conoce sus antecedentes familiares?"];
    let (a, b) = tokio::join!(
        enviar(&cliente, &rutas[0], &sesiones[0], preguntas[0]),
        enviar(&cliente, &rutas[1], &sesiones[1], preguntas[1]),
    );
    let turnos = [a, b];
    assert_ne!(turnos[0]["turno_id"], turnos[1]["turno_id"]);
    assert_ne!(turnos[0]["operacion_id"], turnos[1]["operacion_id"]);
    let docente = sesion(&cliente, &origen, "docente-demo").await;
    let registros = fs::read_to_string(log).unwrap();
    let mut comparaciones = Vec::new();
    for i in 0..2 {
        let turno = &turnos[i];
        let operacion = turno["operacion_id"].as_str().unwrap();
        assert_eq!(turno["intento_id"], ids[i]);
        assert_eq!(turno["pregunta"], preguntas[i]);
        assert_eq!(turno["hablante"], "LR");
        let fragmento = ["hace dos días", "fui adoptada"][i];
        assert!(turno["respuesta"].as_str().unwrap().contains(fragmento));
        let sesion_nueva = sesion(&cliente, &origen, identidades[i]).await;
        let reabierto = vista(&cliente, &rutas[i], &sesion_nueva).await;
        assert_eq!(reabierto["turnos"], json!([turno]));
        let ajeno = cliente.get(&rutas[1 - i]).header(header::COOKIE, &sesion_nueva).send().await.unwrap();
        assert_eq!(ajeno.status(), StatusCode::FORBIDDEN);
        let detalle = vista(&cliente, &format!("{}/turnos/{}/traza", rutas[i], turno["turno_id"].as_str().unwrap()), &docente).await;
        assert_eq!(detalle["operacion_id"], turno["operacion_id"]);
        assert_eq!(detalle["invocacion"]["seleccion"]["hecho_id"], ["lr-inicio", "lr-antecedentes-familiares"][i]);
        let leer_marca = |prefijo| -> Value {
            registros.lines().filter_map(|linea| linea.strip_prefix(prefijo))
                .map(|json| serde_json::from_str::<Value>(json).unwrap())
                .find(|marca| marca["operacion_id"] == operacion).unwrap()
        };
        let recepcion = leer_marca("SIRCLE_TURNO_RECIBIDO ");
        let confirmacion = leer_marca("SIRCLE_TURNO_CONFIRMADO ");
        assert_eq!(recepcion["intento_id"], ids[i]);
        assert_eq!(confirmacion["turno_id"], turno["turno_id"]);
        comparaciones.push(json!({"turno":turno,"inicio_api":recepcion,"fin_api":confirmacion,
            "invocacion": detalle["invocacion"], "turnos_antes":0, "turnos_despues":reabierto["turnos"].as_array().unwrap().len(),
            "lectura_ajena_http":ajeno.status().as_u16()}));
    }
    let ultima_recepcion = comparaciones.iter().map(|x| x["inicio_api"]["recibido_unix_ms"].as_u64().unwrap()).max().unwrap();
    let primera_confirmacion = comparaciones.iter().map(|x| x["fin_api"]["confirmado_unix_ms"].as_u64().unwrap()).min().unwrap();
    assert!(ultima_recepcion < primera_confirmacion);
    println!("EVIDENCIA {}", json!({"operaciones": comparaciones, "ultima_recepcion_unix_ms": ultima_recepcion,
        "primera_confirmacion_unix_ms": primera_confirmacion, "solapamiento_ms": primera_confirmacion - ultima_recepcion}));
}
