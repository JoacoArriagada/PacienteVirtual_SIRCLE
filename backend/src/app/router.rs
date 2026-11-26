use axum::{Json, Router, extract::State, routing::{get, post, put}};
use serde_json::{Value, json};

use crate::modules::{acceso::{actividades, sesiones}, entrevista::{traza, turnos}, intentos::{intento, vista}, valoracion::{ficha, hallazgos, mediciones}};
use crate::modules::examenes::ordenes;
use crate::modules::encuestas::{informe, respuestas};
use crate::modules::evaluacion::{entregas, publicacion, revisiones, trabajos, traza as traza_evaluacion};
use crate::modules::razonamiento::escritura;

use super::estado::Estado;

pub fn router(app: Estado) -> Router {
    Router::new()
        .route("/api/estado", get(estado))
        .route("/api/sesiones", post(sesiones::crear).get(sesiones::leer).delete(sesiones::cerrar))
        .route("/api/actividades", get(actividades::listar))
        .route("/api/actividades/{id}", get(actividades::leer))
        .route("/api/docente/actividades", get(revisiones::actividades))
        .route("/api/actividades/{id}/entregas", get(revisiones::entregas))
        .route("/api/actividades/{id}/resultados-educativos", get(informe::leer))
        .route("/api/intentos", post(intento::crear))
        .route("/api/intentos/{id}", get(vista::leer))
        .route("/api/intentos/{id}/encuestas/{momento}", get(respuestas::leer).put(respuestas::confirmar))
        .route("/api/intentos/{id}/mediciones", get(mediciones::listar).post(mediciones::crear))
        .route("/api/intentos/{id}/mediciones/{medicion}/ficha", put(ficha::registrar))
        .route("/api/intentos/{id}/exploraciones", get(hallazgos::listar).post(hallazgos::crear))
        .route("/api/intentos/{id}/ordenes", post(ordenes::confirmar))
        .route("/api/intentos/{id}/ordenes/{orden}", get(ordenes::consultar))
        .route("/api/intentos/{id}/razonamiento", put(escritura::guardar))
        .route("/api/intentos/{id}/entrega", post(entregas::confirmar))
        .route("/api/intentos/{id}/devolucion", get(publicacion::leer))
        .route("/api/evaluaciones/{id}", get(trabajos::leer).patch(revisiones::rectificar))
        .route("/api/evaluaciones/{id}/revisiones/{revision}", get(revisiones::leer))
        .route("/api/evaluaciones/{id}/publicacion", post(publicacion::publicar))
        .route("/api/evaluaciones/{id}/traza", get(trabajos::leer_traza))
        .route("/api/evaluaciones/{id}/criterios/{criterio}/traza", get(traza_evaluacion::criterio))
        .route("/api/intentos/{id}/turnos", post(turnos::crear))
        .route("/api/intentos/{id}/turnos/{turno}/traza", get(traza::leer))
        .route("/api/intentos/{id}/trazas-rechazadas", get(traza::rechazos))
        .with_state(app)
}

async fn estado(State(app): State<Estado>) -> Json<Value> {
    let (base_disponible, disponible) = tokio::join!(app.base_disponible(), async {
        match &app.ollama {
            Some(ollama) => ollama.disponible().await,
            None => false,
        }
    });
    let mut modelo = json!({ "disponible": disponible });
    if app.ollama.is_some() {
        modelo["nombre"] = json!(crate::modules::entrevista::ollama::MODELO);
        modelo["digest_esperado"] = json!(crate::modules::entrevista::ollama::DIGEST_MODELO);
        modelo["contexto_entrevista"] = json!(crate::modules::entrevista::ollama::CONTEXTO);
        if !disponible {
            modelo["aviso"] = json!("La inferencia aún no está disponible. Si la preparación falló, revisa los registros de inferencia y reinicia el servicio.");
        }
    }
    Json(json!({
        "servicio": "sircle",
        "base": { "disponible": base_disponible },
        "modelo": modelo,
    }))
}
