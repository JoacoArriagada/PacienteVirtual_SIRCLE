#[path = "fixtures/http_real.rs"]
mod http_real;

use http_real::{PREGUNTA, preguntar, recuperar, sesion};
use reqwest::StatusCode;
use serde_json::json;

const CLAVE_FALLIDA: &str = "t017-recuperacion-qwen";

#[tokio::test]
#[ignore = "requiere Axum real, el turno T015 y Ollama detenido"]
async fn una_pregunta_nueva_sin_motor_devuelve_503_y_preserva_el_historial() {
    let (cliente, cookie, ruta) = sesion().await;
    let antes = recuperar(&cliente, &ruta, &cookie).await;
    let confirmados = antes["turnos"].as_array().unwrap();
    assert!(confirmados.iter().any(|turno| turno["pregunta"] == PREGUNTA));
    let (codigo, error) = preguntar(&cliente, &ruta, &cookie, CLAVE_FALLIDA).await;
    assert_eq!(codigo, StatusCode::SERVICE_UNAVAILABLE);
    assert!(error["mensaje"].as_str().unwrap().contains("Reintenta"));
    assert!(error.get("turno_id").is_none());
    let despues = recuperar(&cliente, &ruta, &cookie).await;
    assert_eq!(despues["turnos"], antes["turnos"]);
    assert_eq!(despues["revision"], antes["revision"]);

    println!("EVIDENCIA {}", json!({
        "intento_id": antes["intento_id"],
        "pregunta": PREGUNTA,
        "clave": CLAVE_FALLIDA,
        "http": codigo.as_u16(),
        "respuesta": error,
        "turnos_antes": confirmados.iter().map(|turno| &turno["turno_id"]).collect::<Vec<_>>(),
        "turnos_despues": despues["turnos"].as_array().unwrap().iter().map(|turno| &turno["turno_id"]).collect::<Vec<_>>(),
        "turnos_nuevos": 0,
        "revision": despues["revision"],
    }));
}

#[tokio::test]
#[ignore = "requiere Axum real y Ollama recuperado después de T017"]
async fn la_clave_fallida_se_recupera_una_vez_y_reenvia_el_mismo_acuse() {
    let (cliente, cookie, ruta) = sesion().await;
    let antes = recuperar(&cliente, &ruta, &cookie).await;
    let (codigo, acuse) = preguntar(&cliente, &ruta, &cookie, CLAVE_FALLIDA).await;
    assert!(matches!(codigo, StatusCode::CREATED | StatusCode::OK), "HTTP {codigo}: {acuse}");
    assert_eq!(acuse["intento_id"], antes["intento_id"]);
    assert_eq!(acuse["hablante"], "LR");
    assert_eq!(acuse["pregunta"], PREGUNTA);
    assert!(acuse["respuesta"].as_str().unwrap().contains("hace dos días"));

    let (codigo_reenvio, reenvio) = preguntar(&cliente, &ruta, &cookie, CLAVE_FALLIDA).await;
    assert_eq!(codigo_reenvio, StatusCode::OK);
    assert_eq!(reenvio, acuse);
    let despues = recuperar(&cliente, &ruta, &cookie).await;
    let historial = despues["turnos"].as_array().unwrap();
    let coincidencias = historial.iter()
        .filter(|turno| turno["operacion_id"] == acuse["operacion_id"])
        .collect::<Vec<_>>();
    assert_eq!(coincidencias, [&acuse]);
    let antes = antes["turnos"].as_array().unwrap();
    assert_eq!(historial.len(), antes.len() + usize::from(codigo == StatusCode::CREATED));

    println!("EVIDENCIA {}", json!({
        "clave": CLAVE_FALLIDA,
        "primera_confirmacion": {"http": codigo.as_u16(), "acuse": acuse},
        "reenvio": {"http": codigo_reenvio.as_u16(), "acuse": reenvio},
        "turnos_antes": antes.iter().map(|turno| &turno["turno_id"]).collect::<Vec<_>>(),
        "turnos_despues": historial.iter().map(|turno| &turno["turno_id"]).collect::<Vec<_>>(),
        "pares_confirmados_para_la_operacion": coincidencias.len(),
    }));
}
