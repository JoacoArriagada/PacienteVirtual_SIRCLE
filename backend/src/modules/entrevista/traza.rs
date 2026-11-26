use axum::{
    Json,
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
};
use rusqlite::{OptionalExtension, params};
use serde_json::{Value, json};

use crate::{
    app::{errores::ErrorApi, estado::Estado},
    shared::autorizacion,
};

pub async fn leer(
    State(app): State<Estado>,
    Path((intento_id, turno_id)): Path<(String, String)>,
    cabeceras: HeaderMap,
) -> Result<Json<Value>, ErrorApi> {
    app.consultar(move |db| {
        let tx = db.transaction()?;
        autorizacion::docente_del_intento(&tx, &cabeceras, &intento_id)?;
        let (operacion_id, detalle): (String, String) = tx.query_row(
            "SELECT tr.operacion_id, tr.detalle
             FROM trazas_turno tr JOIN turnos t ON t.id=tr.turno_id
             WHERE t.intento_id=?1 AND t.id=?2",
            params![intento_id, turno_id],
            |fila| Ok((fila.get(0)?, fila.get(1)?)),
        ).optional()?.ok_or_else(reservada)?;
        let detalle: Value = serde_json::from_str(&detalle).map_err(ErrorApi::interno)?;
        let mut respuesta = json!({
            "intento_id": intento_id,
            "turno_id": turno_id,
            "operacion_id": operacion_id,
            "invocacion": detalle["invocacion"],
            "hecho": detalle["hecho"],
            "origen": detalle["origen"].as_str().unwrap_or("ollama"),
            "sugerencia_id": detalle["sugerencia_id"],
        });
        if let Some(mensaje) = detalle.get("mensaje") {
            respuesta["mensaje"] = mensaje.clone();
        }
        if let Some(motivo) = detalle.get("motivo") {
            respuesta["motivo"] = motivo.clone();
        }
        Ok(Json(respuesta))
    }).await
}

fn reservada() -> ErrorApi {
    ErrorApi::nuevo(StatusCode::FORBIDDEN, "La traza está reservada al docente responsable.")
}
