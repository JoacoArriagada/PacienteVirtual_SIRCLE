use anyhow::{Result, ensure};
use axum::{Json, extract::{Path, State}, http::{HeaderMap, StatusCode}};
use rusqlite::{Connection, OptionalExtension, Row, TransactionBehavior, params};
use serde::{Deserialize, Serialize};

use crate::{
    agentes::evaluador::perfil::{Juicio, Observacion, Prioridad},
    app::{errores::ErrorApi, estado::Estado},
    shared::{autorizacion, operaciones::Operacion},
};

use super::{revisiones::{self, Puntaje}, trabajos};

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Peticion {
    revision: i64,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Publicacion {
    evaluacion_id: String,
    numero: i64,
    revision: i64,
    docente_id: String,
    publicada_en: String,
}

#[derive(Serialize)]
#[serde(tag = "estado", rename_all = "snake_case")]
pub enum Devolucion {
    NoDisponible,
    PostPendiente,
    Pendiente,
    Publicada {
        revision: i64,
        publicada_en: String,
        observaciones: Vec<Observacion>,
        prioridad: Option<Prioridad>,
        puntaje: Puntaje,
    },
}

pub async fn publicar(
    State(app): State<Estado>, Path(id): Path<String>, cabeceras: HeaderMap, Json(peticion): Json<Peticion>,
) -> Result<(StatusCode, Json<Publicacion>), ErrorApi> {
    app.consultar(move |db| {
        let tx = db.transaction_with_behavior(TransactionBehavior::Immediate)?;
        let docente = trabajos::autorizar(&tx, &id, &cabeceras)?;
        let operacion = Operacion::nueva(&docente.id, &id, &cabeceras, "POST publicacion", &peticion)?;
        if let Some(acuse) = operacion.recuperar(&tx)? {
            return Ok((StatusCode::OK, Json(acuse)));
        }
        let revision = revisiones::buscar(&tx, &id, peticion.revision)?
            .ok_or_else(|| ErrorApi::nuevo(StatusCode::NOT_FOUND, "No existe esa revisión de la evaluación."))?;
        if revision.resultado.observaciones.iter().any(|o| o.juicio == Juicio::RevisionDocente || o.incidencia.is_some()) {
            return Err(ErrorApi::nuevo(StatusCode::CONFLICT, "Resuelve los criterios pendientes de revisión antes de publicar."));
        }
        if let Some(anterior) = ultima(&tx, &id)? {
            if anterior.revision == peticion.revision {
                operacion.confirmar(&tx, &anterior)?;
                tx.commit()?;
                return Ok((StatusCode::OK, Json(anterior)));
            }
        }
        let acuse = tx.query_row(
            "INSERT INTO publicaciones_evaluacion (evaluacion_id,numero,revision,docente_id,publicada_en)
             VALUES (?1,1+coalesce((SELECT max(numero) FROM publicaciones_evaluacion WHERE evaluacion_id=?1),0),
                     ?2,?3,strftime('%Y-%m-%dT%H:%M:%fZ','now'))
             RETURNING evaluacion_id,numero,revision,docente_id,publicada_en",
            params![id, peticion.revision, docente.id], desde_fila,
        )?;
        operacion.confirmar(&tx, &acuse)?;
        tx.commit()?;
        Ok((StatusCode::CREATED, Json(acuse)))
    }).await
}

pub async fn leer(
    State(app): State<Estado>, Path(id): Path<String>, cabeceras: HeaderMap,
) -> Result<Json<Devolucion>, ErrorApi> {
    app.consultar(move |db| {
        let tx = db.transaction()?;
        let (_, intento) = autorizacion::intento_propio(&tx, &cabeceras, &id)?;
        let evaluacion: Option<Option<String>> = tx.query_row(
            "SELECT v.id FROM entregas e LEFT JOIN evaluaciones v ON v.entrega_id=e.id WHERE e.intento_id=?1",
            [&id], |r| r.get(0),
        ).optional()?;
        let Some(evaluacion) = evaluacion.filter(|_| intento.estado == "entregado") else {
            return Ok(Json(Devolucion::NoDisponible));
        };
        if intento.post == "pendiente" {
            return Ok(Json(Devolucion::PostPendiente));
        }
        let Some(evaluacion) = evaluacion else { return Ok(Json(Devolucion::Pendiente)) };
        let Some(publicada) = ultima(&tx, &evaluacion)? else { return Ok(Json(Devolucion::Pendiente)) };
        let revision = revisiones::buscar(&tx, &evaluacion, publicada.revision)?
            .ok_or_else(|| ErrorApi::interno("Falta la revisión publicada"))?;
        Ok(Json(Devolucion::Publicada {
            revision: publicada.revision, publicada_en: publicada.publicada_en,
            observaciones: revision.resultado.observaciones, prioridad: revision.resultado.prioridad, puntaje: revision.puntaje,
        }))
    }).await
}

pub(crate) fn ultima(db: &Connection, evaluacion: &str) -> rusqlite::Result<Option<Publicacion>> {
    db.query_row(
        "SELECT evaluacion_id,numero,revision,docente_id,publicada_en FROM publicaciones_evaluacion
         WHERE evaluacion_id=?1 ORDER BY numero DESC LIMIT 1",
        [evaluacion], desde_fila,
    ).optional()
}

fn desde_fila(r: &Row<'_>) -> rusqlite::Result<Publicacion> {
    Ok(Publicacion { evaluacion_id:r.get(0)?, numero:r.get(1)?, revision:r.get(2)?, docente_id:r.get(3)?, publicada_en:r.get(4)? })
}

pub fn resolver_post_ensayo(db: &mut Connection, intento: &str, estado: &str) -> Result<()> {
    ensure!(matches!(estado, "respondido" | "omitido"), "Indica respondido u omitido");
    let tx = db.transaction_with_behavior(TransactionBehavior::Immediate)?;
    let autorizada: bool = tx.query_row(
        "SELECT EXISTS(SELECT 1 FROM entregas e JOIN intentos i ON i.id=e.intento_id
         JOIN actividades a ON a.id=i.actividad_id
         WHERE i.id=?1 AND i.estado='entregado' AND a.tipo='ensayo' AND a.habilitada=1)",
        [intento], |r| r.get(0),
    )?;
    ensure!(autorizada, "Solo se puede resolver el post de una entrega de ensayo");
    let anterior = crate::modules::encuestas::respuestas::estado_post(&tx, intento)?;
    ensure!(anterior == "pendiente" || anterior == estado, "El post de ensayo ya está resuelto");
    tx.execute(
        "INSERT INTO post_ensayo (intento_id,estado,momento)
         VALUES (?1,?2,strftime('%Y-%m-%dT%H:%M:%fZ','now')) ON CONFLICT(intento_id) DO NOTHING",
        params![intento, estado],
    )?;
    tx.commit()?;
    Ok(())
}
