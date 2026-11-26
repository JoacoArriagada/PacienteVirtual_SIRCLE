use anyhow::{Context, Result};
use rusqlite::{Connection, Transaction, params};
use serde::{Deserialize, Serialize};

use crate::{agentes::{dan_sparks::perfil as dan, lr::perfil, nannu::perfil as nannu}, app::errores::ErrorApi, modules::intentos::intento::Intento};

const ENSAYO_LR: &str = include_str!("../../../tests/fixtures/lr/hechos_iniciales.json");
const ENSAYO_NANNU: &str = include_str!("../../../tests/fixtures/nannu/hechos_iniciales.json");
const ENSAYO_DAN: &str = include_str!("../../../tests/fixtures/dan_sparks/hechos_iniciales.json");

#[derive(Deserialize)]
struct HechoEnsayo {
    id: String,
    hablante: Option<String>,
    condicion: String,
    respuesta: String,
    pasaje: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Hecho {
    pub id: String,
    pub escenario: String,
    pub version: String,
    pub fase: String,
    pub hablante: String,
    pub condicion: String,
    pub respuesta: String,
    pub fuente: String,
    pub revision_fuente_id: i64,
    pub linea: i64,
    pub pasaje: String,
}

pub(crate) fn preparar_lr(db: &Transaction<'_>) -> Result<()> {
    preparar_actor(db, "lr-ensayo", perfil::FUENTE_ACTOR, perfil::HABLANTE, ENSAYO_LR)
}

pub(crate) fn preparar_nannu(db: &Transaction<'_>) -> Result<()> {
    preparar_actor(db, nannu::ACTIVIDAD, nannu::FUENTE_ACTOR, nannu::PACIENTE, ENSAYO_NANNU)
}

pub(crate) fn preparar_dan(db: &Transaction<'_>) -> Result<()> {
    preparar_actor(db, dan::ACTIVIDAD, dan::FUENTE_ACTOR, dan::HABLANTE, ENSAYO_DAN)
}

fn preparar_actor(db: &Transaction<'_>, actividad: &str, ruta: &str, voz: &str, ficha: &str) -> Result<()> {
    let (escenario, version, fuente, revision, original): (String, i64, i64, i64, Vec<u8>) = db.query_row(
        "SELECT c.escenario_id, c.version_caso_id, f.id, r.id, r.contenido FROM configuraciones_ensayo c
         JOIN versiones_fuentes vf ON vf.version_clinica_id=c.version_caso_id
         JOIN fuentes f ON f.id=vf.fuente_id
         JOIN revisiones_fuente r ON r.id=vf.revision_fuente_id
         WHERE c.actividad_id=?1 AND f.rol='sp_actor' AND f.ruta=?2",
        params![actividad, ruta],
        |fila| Ok((fila.get(0)?, fila.get(1)?, fila.get(2)?, fila.get(3)?, fila.get(4)?)),
    ).with_context(|| format!("No se encontró el guion del actor de {actividad}"))?;
    let original = std::str::from_utf8(&original)?;
    for hecho in serde_json::from_str::<Vec<HechoEnsayo>>(ficha)? {
        let linea = original.lines().position(|linea| linea.contains(&hecho.pasaje))
            .with_context(|| format!("Falta el pasaje del hecho {} en medsp1000/{ruta}", hecho.id))?;
        db.execute(
            "INSERT INTO hechos VALUES (?1, ?2, ?3, 'inicial', ?4, ?5, ?6, ?7, ?8, ?9, ?10)
             ON CONFLICT(version_clinica_id, fase, hablante, id) DO NOTHING",
            params![hecho.id, escenario, version, hecho.hablante.as_deref().unwrap_or(voz), hecho.condicion,
                hecho.respuesta, fuente, revision, linea + 1, hecho.pasaje],
        )?;
    }
    Ok(())
}

pub fn candidatos_lr(db: &Connection, intento: &Intento) -> Result<Vec<Hecho>, ErrorApi> {
    if intento.caso != perfil::ESCENARIO {
        return Ok(Vec::new());
    }
    candidatos(db, intento, perfil::HABLANTE)
}

pub fn candidatos(db: &Connection, intento: &Intento, hablante: &str) -> Result<Vec<Hecho>, ErrorApi> {
    let mut consulta = db.prepare(
        "SELECT h.id, h.escenario_id, v.version, h.fase, h.hablante, h.condicion, h.respuesta,
                f.ruta, h.revision_fuente_id, h.linea, h.pasaje
         FROM intentos i JOIN versiones_clinicas v ON v.id=i.version_caso_id
         JOIN hechos h ON h.version_clinica_id=i.version_caso_id AND h.escenario_id=i.escenario_id AND h.fase=i.fase
         JOIN fuentes f ON f.id=h.fuente_id AND f.rol='sp_actor'
         WHERE i.id=?1 AND i.estudiante_id=?2 AND i.escenario_id=?3 AND v.version=?4
           AND i.fase=?5 AND h.hablante=?6 AND i.estado='atencion' AND i.pre <> 'pendiente'
         ORDER BY h.id",
    )?;
    let filas = consulta.query_map(params![intento.intento_id, intento.estudiante_id, intento.caso,
        intento.version_caso, intento.fase, hablante], |fila| {
        Ok(Hecho {
            id: fila.get(0)?, escenario: fila.get(1)?, version: fila.get(2)?, fase: fila.get(3)?,
            hablante: fila.get(4)?, condicion: fila.get(5)?, respuesta: fila.get(6)?, fuente: fila.get(7)?,
            revision_fuente_id: fila.get(8)?, linea: fila.get(9)?, pasaje: fila.get(10)?,
        })
    })?;
    Ok(filas.collect::<rusqlite::Result<_>>()?)
}
