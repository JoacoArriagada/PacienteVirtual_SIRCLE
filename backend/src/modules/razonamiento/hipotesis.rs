use std::collections::HashSet;

use axum::http::StatusCode;
use rusqlite::{Connection, params};
use serde::{Deserialize, Serialize};

use crate::app::errores::ErrorApi;

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Peticion {
    pub revision_esperada: i64,
    pub hipotesis: Vec<Hipotesis>,
}

#[derive(Deserialize, Serialize)]
pub struct Borrador {
    pub(crate) revision: i64,
    pub(crate) hipotesis: Vec<Hipotesis>,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Hipotesis {
    pub(crate) rol: String,
    pub(crate) titulo: String,
    pub(crate) explicacion: String,
    pub(crate) evidencias: Vec<Referencia>,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Referencia {
    pub(crate) tipo: String,
    pub(crate) origen_id: String,
}

pub(crate) fn guardar(
    db: &Connection, id: &str, revision: i64, mut hipotesis: Vec<Hipotesis>,
) -> Result<Borrador, ErrorApi> {
    if hipotesis.len() > 12 || hipotesis.iter().filter(|h| h.rol == "principal").count() > 1
        || hipotesis.iter().filter(|h| h.rol == "alternativa").count() > 11
    {
        return Err(ErrorApi::nuevo(StatusCode::UNPROCESSABLE_ENTITY, "Indica una principal y hasta once alternativas."));
    }
    let mut verificar = db.prepare_cached(
        "SELECT EXISTS(SELECT 1 FROM evidencias_registradas WHERE intento_id=?1 AND tipo=?2 AND origen_id=?3)",
    )?;
    for item in &mut hipotesis {
        item.titulo = item.titulo.trim().into();
        item.explicacion = item.explicacion.trim().into();
        if !matches!(item.rol.as_str(), "principal" | "alternativa")
            || item.titulo.chars().count() > 180
            || item.explicacion.chars().count() > 1400
            || item.evidencias.len() > 100
        {
            return Err(ErrorApi::nuevo(StatusCode::UNPROCESSABLE_ENTITY, "Revisa el rol, el texto y las referencias de la hipótesis."));
        }
        let mut usadas = HashSet::new();
        for referencia in &item.evidencias {
            let propia: bool = verificar.query_row(params![id, referencia.tipo, referencia.origen_id], |fila| fila.get(0))?;
            if !propia || !usadas.insert((&referencia.tipo, &referencia.origen_id)) {
                return Err(ErrorApi::nuevo(StatusCode::UNPROCESSABLE_ENTITY, "Selecciona evidencias disponibles y distintas de este intento."));
            }
        }
    }
    db.execute("INSERT INTO borradores_hipotesis VALUES (?1, ?2)", params![id, revision])?;
    for (posicion, item) in hipotesis.iter().enumerate() {
        db.execute("INSERT INTO hipotesis VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![id, revision, posicion, item.rol, item.titulo, item.explicacion])?;
        for (orden, referencia) in item.evidencias.iter().enumerate() {
            db.execute("INSERT INTO vinculos_hipotesis VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                params![id, revision, posicion, orden, referencia.tipo, referencia.origen_id])?;
        }
    }
    Ok(Borrador { revision, hipotesis })
}

pub(crate) fn ultimo(db: &Connection, id: &str) -> Result<Option<Borrador>, ErrorApi> {
    let revision = db.query_row(
        "SELECT max(revision) FROM borradores_hipotesis WHERE intento_id=?1",
        [id], |fila| fila.get::<_, Option<i64>>(0),
    )?;
    let Some(revision) = revision else { return Ok(None) };
    let mut lectura = db.prepare(
        "SELECT posicion, rol, titulo, explicacion FROM hipotesis WHERE intento_id=?1 AND revision=?2 ORDER BY posicion",
    )?;
    let mut vinculos = db.prepare(
        "SELECT tipo, origen_id FROM vinculos_hipotesis
         WHERE intento_id=?1 AND revision=?2 AND posicion=?3 ORDER BY orden",
    )?;
    let filas = lectura.query_map(params![id, revision], |fila| {
        let referencias = vinculos.query_map(params![id, revision, fila.get::<_, i64>(0)?], |vinculo| {
            Ok(Referencia { tipo: vinculo.get(0)?, origen_id: vinculo.get(1)? })
        })?;
        Ok(Hipotesis {
            rol: fila.get(1)?, titulo: fila.get(2)?, explicacion: fila.get(3)?,
            evidencias: referencias.collect::<rusqlite::Result<_>>()?,
        })
    })?;
    Ok(Some(Borrador { revision, hipotesis: filas.collect::<rusqlite::Result<_>>()? }))
}
