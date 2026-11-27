use axum::http::StatusCode;
use rusqlite::Transaction;
use serde_json::json;

use crate::{
    app::errores::ErrorApi,
    modules::{
        entrevista::{hechos, turnos::{self, Salida}},
        intentos::intento::Intento,
    },
    shared::ficha_aprobada::Sugerencia,
};

use super::perfil;

pub fn confirmar_literal(
    db: &Transaction<'_>, contexto: &Intento, texto: &str, operacion: &str, sugerencia: Sugerencia,
) -> Result<turnos::Turno, ErrorApi> {
    let actual = turnos::contexto_vigente(db, contexto)?;
    let elegido = hechos::candidatos_lr(db, &actual)?.into_iter()
        .find(|hecho| hecho.id == sugerencia.hecho_id)
        .ok_or_else(|| ErrorApi::nuevo(StatusCode::CONFLICT, "La sugerencia ya no está disponible."))?;
    let detalle = json!({ "hecho": elegido, "origen": "ficha", "sugerencia_id": sugerencia.id });
    turnos::guardar(db, &actual, Salida::hecho(perfil::AGENTE, &elegido), texto, operacion, &detalle)
}
