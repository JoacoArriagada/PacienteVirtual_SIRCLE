#[path = "fixtures/http_real.rs"]
mod http_real;

use http_real::{PREGUNTA, preguntar, recuperar, sesion};
use reqwest::StatusCode;
use serde_json::{Value, json};
use std::time::{Instant, SystemTime, UNIX_EPOCH};

const CLAVE: &str = "t015-parafrasis-inicio";
const SUGERENCIAS_LR: &str = include_str!("../../datos/adaptaciones/lr/entrevista-lr.json");

#[tokio::test]
#[ignore = "requiere Axum y Ollama reales y el intento de ensayo de T008"]
async fn una_parafrasis_inedita_confirma_y_recupera_un_turno_real_de_lr() {
    let sugerencias: Vec<Value> = serde_json::from_str(SUGERENCIAS_LR).unwrap();
    assert!(!sugerencias.iter().any(|sugerencia| sugerencia["prompt"] == PREGUNTA));

    let (cliente, cookie, ruta) = sesion().await;
    let antes = recuperar(&cliente, &ruta, &cookie).await;
    assert_eq!(antes["pre"], "omitido");
    let peticion = json!({"texto": PREGUNTA});
    let (codigo, acuse) = preguntar(&cliente, &ruta, &cookie, CLAVE).await;
    assert!(matches!(codigo, StatusCode::CREATED | StatusCode::OK), "HTTP {codigo}: {acuse}");
    assert_eq!(acuse["intento_id"], antes["intento_id"]);
    assert_eq!(acuse["pregunta"], PREGUNTA);
    assert_eq!(acuse["hablante"], "LR");
    assert_eq!(acuse["tipo_respuesta"], "hecho");
    let texto = acuse["respuesta"].as_str().unwrap();
    assert!(texto.starts_with("Empecé") && texto.contains("hace dos días"));
    assert!(acuse.get("traza").is_none() && acuse.get("digest").is_none());

    let despues = recuperar(&cliente, &ruta, &cookie).await;
    let historial = despues["turnos"].as_array().unwrap();
    let coincidencias = historial.iter().filter(|turno| turno["turno_id"] == acuse["turno_id"]).collect::<Vec<_>>();
    assert_eq!(coincidencias, [&acuse]);
    let recuento_previo = antes["turnos"].as_array().unwrap().len();
    let incremento = usize::from(codigo == StatusCode::CREATED);
    assert_eq!(historial.len(), recuento_previo + incremento);

    println!("EVIDENCIA {}", json!({
        "peticion": peticion,
        "ruta": ruta + "/turnos",
        "clave": CLAVE,
        "http": codigo.as_u16(),
        "acuse": acuse,
        "turno_recuperado": coincidencias[0],
        "numero_de_sugerencias_cotejadas": sugerencias.len(),
        "recuento_previo": recuento_previo,
        "recuento_posterior": historial.len(),
    }));
}

#[tokio::test]
#[ignore = "requiere Axum y Ollama reales y descarga previa del modelo"]
async fn mide_la_pregunta_en_frio_y_caliente() {
    let (cliente, cookie, ruta) = sesion().await;
    let motor = std::env::var("OLLAMA_URL").unwrap_or_else(|_| "http://127.0.0.1:11434".into());
    let modelo = sircle::modules::entrevista::ollama::MODELO;
    cliente.post(format!("{motor}/api/generate"))
        .json(&json!({"model": modelo, "keep_alive": 0, "stream": false}))
        .send().await.unwrap().error_for_status().unwrap()
        .json::<Value>().await.unwrap();
    let tags: Value = cliente.get(format!("{motor}/api/tags")).send().await.unwrap()
        .error_for_status().unwrap().json().await.unwrap();
    let etiqueta = tags["models"].as_array().unwrap().iter()
        .find(|item| item["name"] == modelo).unwrap();
    assert_eq!(etiqueta["digest"], sircle::modules::entrevista::ollama::DIGEST_MODELO);
    let corrida = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
    for (estado, residente) in [("frio", false), ("caliente", true)] {
        let respuesta: Value = cliente.get(format!("{motor}/api/ps")).send().await.unwrap()
            .error_for_status().unwrap().json().await.unwrap();
        let procesos = respuesta["models"].as_array().unwrap();
        let proceso = procesos.iter().find(|item| item["name"] == modelo);
        assert_eq!(proceso.is_some(), residente, "{respuesta}");
        if let Some(proceso) = proceso {
            assert_eq!(proceso["size_vram"], 0);
            assert_eq!(proceso["context_length"], 2048);
        }
        let clave = format!("t019-{corrida}-{estado}");
        let inicio = Instant::now();
        let (codigo, acuse) = preguntar(&cliente, &ruta, &cookie, &clave).await;
        let duracion_ms = inicio.elapsed().as_secs_f64() * 1000.0;
        assert_eq!(codigo, StatusCode::CREATED, "{acuse}");
        assert_eq!(acuse["hablante"], "LR");
        assert!(acuse["respuesta"].as_str().unwrap().contains("hace dos días"));
        println!("MUESTRA {}", json!({
            "estado": estado, "residente_antes": residente, "clave": clave,
            "latencia_http_ms": duracion_ms, "http": codigo.as_u16(), "acuse": acuse,
            "proceso_antes": proceso,
        }));
    }
}

