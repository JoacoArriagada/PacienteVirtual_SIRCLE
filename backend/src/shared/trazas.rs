use std::time::{SystemTime, UNIX_EPOCH};

use rusqlite::{Transaction, params};
use serde_json::Value;

use crate::app::errores::ErrorApi;

pub fn ahora_ms() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_millis() as u64
}

pub fn guardar_turno(
    db: &Transaction<'_>, turno_id: &str, operacion_id: &str, detalle: &Value,
) -> Result<(), ErrorApi> {
    let detalle = serde_json::to_string(detalle).map_err(ErrorApi::interno)?;
    db.execute(
        "INSERT INTO trazas_turno (turno_id, operacion_id, detalle) VALUES (?1, ?2, ?3)",
        params![turno_id, operacion_id, detalle],
    )?;
    Ok(())
}

pub fn guardar_evaluacion(
    db: &Transaction<'_>, evaluacion_id: &str, operacion_id: &str, detalle: &Value,
) -> Result<(), ErrorApi> {
    db.execute(
        "INSERT INTO trazas_evaluacion (evaluacion_id, operacion_id, detalle) VALUES (?1, ?2, ?3)",
        params![evaluacion_id, operacion_id, detalle.to_string()],
    )?;
    Ok(())
}
