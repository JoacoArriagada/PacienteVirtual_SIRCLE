use axum::{Json, extract::{Path, State}, http::{HeaderMap, StatusCode}};
use rusqlite::{Connection, OptionalExtension, Row, TransactionBehavior, params};
use serde::{Deserialize, Serialize};

use crate::{
    app::{errores::ErrorApi, estado::Estado},
    shared::{autorizacion, operaciones::Operacion},
};

const ECG_INICIAL: &str = "ecg-inicial";
const ORDEN: &str = "SELECT id, intento_id, examen, fase, momento, revision FROM ordenes";

#[derive(Deserialize, Serialize)]
pub struct Peticion {
    examen: String,
    accion: String,
}

#[derive(Deserialize, Serialize)]
pub struct Orden {
    pub(crate) orden_id: String,
    pub(crate) intento_id: String,
    pub(crate) examen: String,
    fase: String,
    momento: String,
    revision: i64,
}

#[derive(Serialize)]
pub struct Informe {
    informe_id: String,
    orden_id: String,
    intento_id: String,
    resultado: String,
    origen: String,
}

pub(crate) fn preparar_ecg_lr(db: &Connection) -> rusqlite::Result<()> {
    db.execute(
        "INSERT INTO examenes_versionados
         SELECT c.version_caso_id, 'inicial', ?1, 'taquicardia sinusal', r.id
         FROM configuraciones_ensayo c
         JOIN versiones_fuentes vf ON vf.version_clinica_id=c.version_caso_id
         JOIN fuentes f ON f.id=vf.fuente_id AND f.rol='environment_controller'
         JOIN revisiones_fuente r ON r.id=vf.revision_fuente_id
         WHERE c.actividad_id='lr-ensayo' AND instr(CAST(r.contenido AS TEXT), 'initial EKG (sinus tachycardia)') > 0
         ON CONFLICT(version_caso_id, fase, examen) DO NOTHING",
        [ECG_INICIAL],
    )?;
    Ok(())
}

pub async fn confirmar(
    State(app): State<Estado>, Path(id): Path<String>, cabeceras: HeaderMap, Json(peticion): Json<Peticion>,
) -> Result<(StatusCode, Json<Orden>), ErrorApi> {
    app.consultar(move |db| {
        let tx = db.transaction_with_behavior(TransactionBehavior::Immediate)?;
        let (identidad, intento) = autorizacion::intento_propio(&tx, &cabeceras, &id)?;
        let operacion = Operacion::nueva(&identidad.id, &id, &cabeceras, "POST ordenes", &peticion)?;
        if let Some(guardada) = operacion.recuperar(&tx)? {
            return Ok((StatusCode::OK, Json(guardada)));
        }
        if peticion.examen != ECG_INICIAL || peticion.accion != "confirmar" {
            return Err(ErrorApi::nuevo(StatusCode::UNPROCESSABLE_ENTITY, "Confirma una orden de ECG inicial."));
        }
        let existente = tx.query_row(
            &format!("{ORDEN} WHERE intento_id=?1 AND examen=?2"),
            params![id, ECG_INICIAL], desde_fila,
        ).optional()?;
        if let Some(orden) = existente {
            operacion.confirmar(&tx, &orden)?;
            tx.commit()?;
            return Ok((StatusCode::OK, Json(orden)));
        }
        if intento.estado != "atencion" || intento.pre == "pendiente" || intento.fase != "inicial" {
            return Err(ErrorApi::nuevo(StatusCode::CONFLICT, "La fase actual no permite confirmar el ECG inicial."));
        }
        let (resultado, fuente): (String, i64) = tx.query_row(
            "SELECT e.resultado, e.revision_fuente_id FROM examenes_versionados e
             JOIN intentos i ON i.version_caso_id=e.version_caso_id AND i.fase=e.fase
             WHERE i.id=?1 AND e.examen=?2",
            params![id, ECG_INICIAL], |fila| Ok((fila.get(0)?, fila.get(1)?)),
        ).optional()?.ok_or_else(informe_reservado)?;
        let orden = tx.query_row(
            "INSERT INTO ordenes (id, intento_id, examen, fase, momento, revision)
             VALUES (lower(hex(randomblob(16))), ?1, ?2, ?3, strftime('%Y-%m-%dT%H:%M:%fZ','now'), ?4)
             RETURNING id, intento_id, examen, fase, momento, revision",
            params![id, ECG_INICIAL, intento.fase, intento.revision + 1], desde_fila,
        )?;
        tx.execute(
            "INSERT INTO informes (id, orden_id, resultado, revision_fuente_id)
             VALUES (lower(hex(randomblob(16))), ?1, ?2, ?3)",
            params![orden.orden_id, resultado, fuente],
        )?;
        tx.execute("UPDATE intentos SET revision=?1 WHERE id=?2", params![orden.revision, id])?;
        operacion.confirmar(&tx, &orden)?;
        tx.commit()?;
        Ok((StatusCode::CREATED, Json(orden)))
    }).await
}

pub async fn consultar(
    State(app): State<Estado>, Path((id, orden)): Path<(String, String)>, cabeceras: HeaderMap,
) -> Result<Json<Informe>, ErrorApi> {
    app.consultar(move |db| {
        let tx = db.transaction_with_behavior(TransactionBehavior::Immediate)?;
        let (_, intento) = autorizacion::intento_propio(&tx, &cabeceras, &id)?;
        if intento.pre == "pendiente" {
            return Err(informe_reservado());
        }
        let (informe, consultado) = tx.query_row(
            "SELECT f.id, o.id, o.intento_id, f.resultado, f.consultado_en IS NOT NULL FROM informes f
             JOIN ordenes o ON o.id=f.orden_id
             WHERE o.intento_id=?1 AND (o.id=?2 OR o.examen=?2)",
            params![id, orden], |fila| Ok((Informe {
                informe_id: fila.get(0)?, orden_id: fila.get(1)?, intento_id: fila.get(2)?,
                resultado: fila.get(3)?, origen: "Electrocardiograma inicial".into(),
            }, fila.get::<_, bool>(4)?)),
        ).optional()?.ok_or_else(informe_reservado)?;
        if !consultado {
            if intento.estado != "atencion" {
                return Err(ErrorApi::nuevo(StatusCode::CONFLICT, "El intento ya está entregado."));
            }
            tx.execute(
                "UPDATE informes SET consultado_en=strftime('%Y-%m-%dT%H:%M:%fZ','now') WHERE id=?1",
                [&informe.informe_id],
            )?;
            tx.execute("UPDATE intentos SET revision=revision+1 WHERE id=?1", [&id])?;
        }
        tx.commit()?;
        Ok(Json(informe))
    }).await
}

pub(crate) fn historico(db: &Connection, id: &str) -> Result<Vec<Orden>, ErrorApi> {
    let mut consulta = db.prepare(&format!("{ORDEN} WHERE intento_id=?1 ORDER BY rowid"))?;
    let filas = consulta.query_map([id], desde_fila)?;
    Ok(filas.collect::<rusqlite::Result<_>>()?)
}

fn desde_fila(fila: &Row<'_>) -> rusqlite::Result<Orden> {
    Ok(Orden {
        orden_id: fila.get(0)?, intento_id: fila.get(1)?, examen: fila.get(2)?,
        fase: fila.get(3)?, momento: fila.get(4)?, revision: fila.get(5)?,
    })
}

fn informe_reservado() -> ErrorApi {
    ErrorApi::nuevo(StatusCode::NOT_FOUND, "El informe está reservado hasta confirmar la orden de este intento.")
}
