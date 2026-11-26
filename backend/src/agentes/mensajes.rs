use anyhow::Result;
use rusqlite::{Connection, params};
use serde::{Deserialize, Serialize};

use crate::modules::entrevista::{hechos::Hecho, ollama::{Invocacion, Ollama}};
use crate::shared::contexto::Intercambio;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize, clap::ValueEnum)]
#[serde(rename_all = "snake_case")]
#[value(rename_all = "snake_case")]
pub enum Agente {
    Lr,
    Nannu,
    DanSparks,
    Evaluador,
}

impl Agente {
    pub fn id(self) -> &'static str {
        match self {
            Self::Lr => "lr",
            Self::Nannu => "nannu",
            Self::DanSparks => "dan_sparks",
            Self::Evaluador => "evaluador",
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Cabecera {
    pub operacion_id: String,
    pub intento_id: String,
    pub agente: Agente,
    pub caso: String,
    pub version: String,
    pub fase: String,
    pub hablante: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Solicitud<T> {
    pub cabecera: Cabecera,
    pub contenido: T,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Respuesta<T> {
    pub cabecera: Cabecera,
    pub tipo: TipoResultado,
    pub resultado: T,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TipoResultado {
    Seleccion,
    Evaluacion,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Pregunta {
    pub texto: String,
    pub candidatos: Vec<Candidato>,
    pub memoria: Vec<Intercambio>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Candidato {
    pub id: String,
    pub condicion: String,
}

impl From<&Hecho> for Candidato {
    fn from(hecho: &Hecho) -> Self {
        Self { id: hecho.id.clone(), condicion: hecho.condicion.clone() }
    }
}

pub fn discrepancia(esperada: &Cabecera, recibida: &Cabecera) -> Option<&'static str> {
    [
        (esperada.operacion_id == recibida.operacion_id, "operacion_no_coincide"),
        (esperada.intento_id == recibida.intento_id, "intento_no_coincide"),
        (esperada.agente == recibida.agente, "agente_no_coincide"),
        (esperada.caso == recibida.caso, "caso_no_coincide"),
        (esperada.version == recibida.version, "version_no_coincide"),
        (esperada.fase == recibida.fase, "fase_no_coincide"),
        (esperada.hablante == recibida.hablante, "hablante_no_coincide"),
    ].into_iter().find_map(|(coincide, motivo)| (!coincide).then_some(motivo))
}

pub fn registrar_rechazo(db: &Connection, esperada: &Cabecera, motivo: &str) -> rusqlite::Result<()> {
    registrar_rechazo_con_evidencia(db, esperada, motivo, serde_json::Value::Null)
}

pub(crate) fn registrar_rechazo_con_evidencia(
    db: &Connection, esperada: &Cabecera, motivo: &str, evidencia: serde_json::Value,
) -> rusqlite::Result<()> {
    let detalle = serde_json::json!({ "mensaje": esperada, "evidencia": evidencia });
    db.execute(
        "INSERT INTO rechazos_mensajes (intento_id, operacion_id, agente, motivo, momento, detalle)
         VALUES (?1, ?2, ?3, ?4, strftime('%Y-%m-%dT%H:%M:%fZ','now'), ?5)",
        params![esperada.intento_id, esperada.operacion_id, esperada.agente.id(), motivo, detalle.to_string()],
    )?;
    eprintln!("SIRCLE_MENSAJE_RECHAZADO {}", serde_json::json!({
        "operacion_id": esperada.operacion_id, "agente": esperada.agente.id(), "motivo": motivo,
    }));
    Ok(())
}

pub async fn entrevistar(motor: &Ollama, solicitud: Solicitud<Pregunta>) -> Result<Respuesta<Invocacion>> {
    let resultado = motor.seleccionar(&solicitud.cabecera,
        &solicitud.contenido.texto, &solicitud.contenido.candidatos, &solicitud.contenido.memoria).await?;
    Ok(Respuesta { cabecera: solicitud.cabecera, tipo: TipoResultado::Seleccion, resultado })
}
