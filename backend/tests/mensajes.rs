mod fixtures;

use axum::http::StatusCode;
use fixtures::{Servidor, intento_lr};
use rusqlite::{TransactionBehavior, params};
use serde_json::{Value, json};
use sircle::{
    agentes::mensajes::{Agente, Cabecera, Pregunta, Respuesta, Solicitud, TipoResultado},
    modules::{entrevista::{hechos, ollama::{DIGEST_MODELO, Invocacion, MODELO, Seleccion}, turnos}, intentos::intento},
    sqlite,
};

const TEXTO: &str = "¿Desde cuándo te cuesta respirar?";

fn resultado(cabecera: Cabecera) -> Respuesta<Invocacion> {
    Respuesta {
        tipo: TipoResultado::Seleccion,
        resultado: Invocacion {
            operacion_id: cabecera.operacion_id.clone(), inicio_unix_ms: 1000, fin_unix_ms: 1001,
            modelo: MODELO.into(), digest: DIGEST_MODELO.into(), solicitud: json!({ "pregunta": TEXTO }),
            seleccion: Seleccion { hecho_id: "lr-inicio".into() }, respuesta_cruda: r#"{"hecho_id":"lr-inicio"}"#.into(),
            duracion_motor_ns: None, carga_modelo_ns: None, tokens_respuesta: None,
        },
        cabecera,
    }
}

#[tokio::test]
async fn correlaciona_el_mensaje_y_descarta_cruces_sin_cambiar_el_historial() {
    let servidor = Servidor::nuevo();
    let (cookie, _) = servidor.entrar(intento_lr::ESTUDIANTE_LR).await;
    let alta = intento_lr::alta(&servidor, &cookie).await;
    let id = alta.cuerpo["intento_id"].as_str().unwrap();
    let ruta = format!("/api/intentos/{id}");
    intento_lr::resolver_pre(&servidor, id);
    let mut db = sqlite::abrir_existente(&servidor.db).unwrap();
    let intento = intento::recuperar(&db, id, intento_lr::ESTUDIANTE_LR).unwrap();
    let hechos = hechos::candidatos_lr(&db, &intento).unwrap();
    let cabecera = Cabecera {
        operacion_id: "t039-valida".into(), intento_id: id.into(), agente: Agente::Lr,
        caso: intento.caso.clone(), version: intento.version_caso.clone(), fase: intento.fase.clone(), hablante: "LR".into(),
    };
    let solicitud = Solicitud { cabecera: cabecera.clone(),
        contenido: Pregunta { texto: TEXTO.into(), candidatos: hechos.iter().map(Into::into).collect(), memoria: vec![] },
    };
    let mensaje = serde_json::to_value(&solicitud).unwrap();
    assert_eq!(serde_json::from_value::<Solicitud<Pregunta>>(mensaje.clone()).unwrap().cabecera, cabecera);
    assert!(mensaje["contenido"]["candidatos"][0].get("pasaje").is_none());
    assert!(mensaje["contenido"]["candidatos"][0].get("respuesta").is_none());

    let valida = serde_json::to_value(resultado(cabecera.clone())).unwrap();
    let tx = db.transaction_with_behavior(TransactionBehavior::Immediate).unwrap();
    let confirmada = turnos::confirmar_mensaje(
        &tx, &intento, &hechos, TEXTO, &cabecera, &serde_json::from_value(valida.clone()).unwrap(),
    ).unwrap().unwrap();
    tx.commit().unwrap();
    assert_eq!(confirmada.operacion_id, cabecera.operacion_id);
    assert_eq!(confirmada.intento_id, id);
    assert_eq!(confirmada.hablante, "LR");
    let antes = servidor.consultar("GET", &ruta, Some(&cookie), None).await;
    assert_eq!(antes.cuerpo["turnos"], json!([confirmada]));
    let intento = intento::recuperar(&db, id, intento_lr::ESTUDIANTE_LR).unwrap();
    let (docente, _) = servidor.entrar("docente-demo").await;
    let traza = servidor.consultar("GET", &format!("{ruta}/turnos/{}/traza", confirmada.turno_id), Some(&docente), None).await;
    assert_eq!(traza.estado, StatusCode::OK);
    assert_eq!(traza.cuerpo["mensaje"], mensaje["cabecera"]);

    let casos = [
        ("/cabecera/operacion_id", json!("otra-operacion"), "operacion_no_coincide"),
        ("/cabecera/intento_id", json!("otro-intento"), "intento_no_coincide"),
        ("/cabecera/agente", json!("nannu"), "agente_no_coincide"),
        ("/cabecera/caso", json!("P02"), "caso_no_coincide"),
        ("/cabecera/version", json!("otra-revision"), "version_no_coincide"),
        ("/cabecera/fase", json!("posterior"), "fase_no_coincide"),
        ("/cabecera/hablante", json!("acompañante"), "hablante_no_coincide"),
        ("/tipo", json!("evaluacion"), "tipo_no_coincide"),
        ("/resultado/operacion_id", json!("invocacion-ajena"), "invocacion_no_coincide"),
    ];
    let mut descartes = Vec::new();
    for (ruta_json, valor, motivo) in casos {
        let mut esperada = cabecera.clone();
        esperada.operacion_id = format!("t039-{motivo}");
        let mut mal = serde_json::to_value(resultado(esperada.clone())).unwrap();
        *mal.pointer_mut(ruta_json).unwrap() = valor.clone();
        let mal: Respuesta<Invocacion> = serde_json::from_value(mal).unwrap();
        let tx = db.transaction_with_behavior(TransactionBehavior::Immediate).unwrap();
        assert!(turnos::confirmar_mensaje(&tx, &intento, &hechos, TEXTO, &esperada, &mal).unwrap().is_none());
        tx.commit().unwrap();
        let guardado: String = db.query_row(
            "SELECT motivo FROM rechazos_mensajes WHERE intento_id=?1 AND operacion_id=?2",
            params![id, esperada.operacion_id], |fila| fila.get(0),
        ).unwrap();
        assert_eq!(guardado, motivo);
        let despues = servidor.consultar("GET", &ruta, Some(&cookie), None).await;
        assert_eq!(despues.cuerpo["turnos"], antes.cuerpo["turnos"]);
        assert_eq!(despues.cuerpo["revision"], antes.cuerpo["revision"]);
        descartes.push(json!({ "campo": ruta_json, "recibido": valor,
            "operacion_id": esperada.operacion_id, "motivo_guardado": guardado, "turnos": despues.cuerpo["turnos"].as_array().unwrap().len() }));
    }
    let total: i64 = db.query_row("SELECT count(*) FROM rechazos_mensajes", [], |fila| fila.get(0)).unwrap();
    assert_eq!(total as usize, descartes.len());
    println!("EVIDENCIA {}", json!({ "solicitud": mensaje, "respuesta": valida, "turno": confirmada,
        "cabecera_en_traza_docente": traza.cuerpo["mensaje"], "descartes": Value::Array(descartes),
        "turnos_antes": antes.cuerpo["turnos"].as_array().unwrap().len(), "total_descartes": total }));
}
