use axum::http::StatusCode;
use rusqlite::{Connection, Row, params};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{app::errores::ErrorApi, shared::ficha_aprobada::Orientacion};

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Peticion {
    pub revision_esperada: i64,
    pub apunte: String,
}

#[derive(Deserialize, Serialize)]
pub struct Apunte {
    apunte_id: String,
    intento_id: String,
    autor_id: String,
    intencion: String,
    momento: String,
    revision: i64,
}

#[derive(Deserialize, Serialize)]
pub struct Evidencia {
    pub(crate) intento_id: String,
    pub(crate) tipo: String,
    pub(crate) origen_id: String,
    categoria: String,
    autor: String,
    origen: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) texto: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) instrumento: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) valor: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) unidad: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    momento: Option<String>,
}

pub(crate) fn guardar(db: &Connection, id: &str, autor: &str, revision: i64, texto: &str) -> Result<Apunte, ErrorApi> {
    let texto = texto.trim();
    if texto.is_empty() || texto.chars().count() > 1500 {
        return Err(ErrorApi::nuevo(StatusCode::UNPROCESSABLE_ENTITY, "Escribe un apunte de hasta 1500 caracteres."));
    }
    Ok(db.query_row(
        "INSERT INTO apuntes (id, intento_id, autor_id, intencion, momento, revision)
         VALUES (lower(hex(randomblob(16))), ?1, ?2, ?3, strftime('%Y-%m-%dT%H:%M:%fZ','now'), ?4)
         RETURNING id, intento_id, autor_id, intencion, momento, revision",
        params![id, autor, texto, revision], desde_fila,
    )?)
}

pub(crate) fn apuntes(db: &Connection, id: &str) -> Result<Vec<Apunte>, ErrorApi> {
    let mut consulta = db.prepare("SELECT id, intento_id, autor_id, intencion, momento, revision
                                   FROM apuntes WHERE intento_id=?1 ORDER BY revision")?;
    let filas = consulta.query_map([id], desde_fila)?;
    Ok(filas.collect::<rusqlite::Result<_>>()?)
}

fn desde_fila(fila: &Row<'_>) -> rusqlite::Result<Apunte> {
    Ok(Apunte {
        apunte_id: fila.get(0)?, intento_id: fila.get(1)?, autor_id: fila.get(2)?,
        intencion: fila.get(3)?, momento: fila.get(4)?, revision: fila.get(5)?,
    })
}

pub(crate) fn listar(db: &Connection, id: &str, orientacion: Option<&Orientacion>) -> Result<Vec<Evidencia>, ErrorApi> {
    let mut consulta = db.prepare(
        "SELECT tipo, origen_id, categoria, autor, origen, texto, instrumento, valor, unidad, momento
         FROM evidencias_registradas WHERE intento_id=?1 ORDER BY revision, tipo, origen_id",
    )?;
    let filas = consulta.query_map([id], |fila| {
        let valor: Option<String> = fila.get(7)?;
        let valor = valor.map(|texto| serde_json::from_str(&texto)).transpose()
            .map_err(|error| rusqlite::Error::FromSqlConversionFailure(7, rusqlite::types::Type::Text, Box::new(error)))?;
        Ok(Evidencia {
            intento_id: id.into(), tipo: fila.get(0)?, origen_id: fila.get(1)?, categoria: fila.get(2)?,
            autor: fila.get(3)?, origen: fila.get(4)?, texto: fila.get(5)?, instrumento: fila.get(6)?,
            valor, unidad: fila.get(8)?, momento: fila.get(9)?,
        })
    })?;
    let mut evidencias: Vec<_> = filas.collect::<rusqlite::Result<_>>()?;
    if let Some(orientacion) = orientacion {
        evidencias.insert(0, Evidencia {
            intento_id: id.into(), tipo: "orientacion".into(), origen_id: orientacion.orientacion_id.into(),
            categoria: "orientacion".into(), autor: orientacion.origen.into(), origen: "Ayuda de la actividad".into(),
            texto: Some(orientacion.texto.into()), instrumento: None, valor: None, unidad: None, momento: None,
        });
    }
    Ok(evidencias)
}
