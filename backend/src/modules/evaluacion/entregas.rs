use axum::{Json, extract::{Path, State}, http::{HeaderMap, StatusCode}};
use rusqlite::{Connection, OptionalExtension, Row, TransactionBehavior, params};
use serde::{Deserialize, Serialize};

use crate::{
    agentes::evaluador::perfil::AcuseSinEvento,
    app::{errores::ErrorApi, estado::Estado},
    modules::{entrevista::turnos, examenes::ordenes, razonamiento::{evidencias, hipotesis}, valoracion::hallazgos},
    shared::{autorizacion, operaciones::Operacion},
};
use super::integridad;

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Peticion {
    revision_esperada: i64,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Instantanea {
    pub borrador: Option<hipotesis::Borrador>,
    pub evidencias: Vec<evidencias::Evidencia>,
    pub turnos: Vec<turnos::Turno>,
    pub exploraciones: Vec<hallazgos::Exploracion>,
    pub ordenes: Vec<ordenes::Orden>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub acuses_sin_evento: Option<Vec<AcuseSinEvento>>,
}

#[derive(Deserialize, Serialize)]
pub struct Acuse {
    pub entrega_id: String,
    pub intento_id: String,
    pub revision: i64,
    pub rubrica_id: i64,
    pub version_rubrica: String,
    pub rubrica_sha256: String,
    pub confirmada_en: String,
    estado_evaluacion: String,
    post: String,
}

pub async fn confirmar(
    State(app): State<Estado>, Path(id): Path<String>, cabeceras: HeaderMap,
    Json(peticion): Json<Peticion>,
) -> Result<(StatusCode, Json<Acuse>), ErrorApi> {
    app.consultar(move |db| {
        let tx = db.transaction_with_behavior(TransactionBehavior::Immediate)?;
        let (identidad, intento) = autorizacion::intento_propio(&tx, &cabeceras, &id)?;
        let operacion = Operacion::nueva(&identidad.id, &id, &cabeceras, "POST entrega", &peticion)?;
        if let Some(acuse) = operacion.recuperar(&tx)? {
            return Ok((StatusCode::OK, Json(acuse)));
        }
        if let Some(acuse) = recuperar_acuse(&tx, &id)? {
            if acuse.revision == peticion.revision_esperada {
                return Ok((StatusCode::OK, Json(acuse)));
            }
            return Err(ErrorApi::nuevo(StatusCode::CONFLICT, "El intento ya está entregado."));
        }
        if intento.estado != "atencion" || intento.pre == "pendiente" {
            return Err(ErrorApi::nuevo(StatusCode::CONFLICT, "El estado del intento impide entregar."));
        }
        if intento.revision != peticion.revision_esperada {
            return Err(ErrorApi::nuevo(StatusCode::CONFLICT, "El intento cambió. Revisa el trabajo guardado antes de entregar."));
        }
        let (Some(rubrica_id), Some(sha256)) = (intento.rubrica_id, &intento.rubrica_sha256) else {
            return Err(ErrorApi::nuevo(StatusCode::CONFLICT, "El intento técnico requiere una rúbrica fijada desde su apertura."));
        };
        let mut instantanea = Instantanea {
            borrador: hipotesis::ultimo(&tx, &id)?,
            evidencias: evidencias::listar(&tx, &id, None)?,
            turnos: turnos::historico(&tx, &id)?,
            exploraciones: hallazgos::historico(&tx, &id)?,
            ordenes: ordenes::historico(&tx, &id)?,
            acuses_sin_evento: None,
        };
        instantanea.acuses_sin_evento = Some(integridad::detectar(&tx, &id, intento.revision, &instantanea).map_err(ErrorApi::interno)?);
        let instantanea = serde_json::to_string(&instantanea).map_err(ErrorApi::interno)?;
        let acuse = tx.query_row(
            "INSERT INTO entregas (id,intento_id,revision,caso,version_caso,rubrica_id,version_rubrica,rubrica_sha256,confirmada_en,instantanea)
             VALUES (lower(hex(randomblob(16))),?1,?2,?3,?4,?5,?6,?7,strftime('%Y-%m-%dT%H:%M:%fZ','now'),?8)
             RETURNING id,intento_id,revision,rubrica_id,version_rubrica,rubrica_sha256,confirmada_en",
            params![id, intento.revision, intento.caso, intento.version_caso, rubrica_id, intento.version_rubrica, sha256, instantanea],
            desde_fila,
        )?;
        tx.execute("INSERT INTO trabajos_evaluacion (entrega_id,estado,creado_en) VALUES (?1,'pendiente',?2)",
            params![acuse.entrega_id, acuse.confirmada_en])?;
        tx.execute("UPDATE intentos SET estado='entregado', revision=revision+1 WHERE id=?1", [&id])?;
        operacion.confirmar(&tx, &acuse)?;
        tx.commit()?;
        Ok((StatusCode::CREATED, Json(acuse)))
    }).await
}

pub(crate) fn recuperar_acuse(db: &Connection, intento: &str) -> rusqlite::Result<Option<Acuse>> {
    db.query_row(
        "SELECT id,intento_id,revision,rubrica_id,version_rubrica,rubrica_sha256,confirmada_en
         FROM entregas WHERE intento_id=?1",
        [intento], desde_fila,
    ).optional()
}

fn desde_fila(fila: &Row<'_>) -> rusqlite::Result<Acuse> {
    Ok(Acuse {
        entrega_id: fila.get(0)?, intento_id: fila.get(1)?, revision: fila.get(2)?,
        rubrica_id: fila.get(3)?, version_rubrica: fila.get(4)?, rubrica_sha256: fila.get(5)?,
        confirmada_en: fila.get(6)?, estado_evaluacion: "pendiente".into(), post: "pendiente".into(),
    })
}
