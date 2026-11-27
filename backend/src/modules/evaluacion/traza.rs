use axum::{Json, extract::{Path, Query, State}, http::{HeaderMap, StatusCode}};
use rusqlite::{Connection, OptionalExtension, params};
use serde::Deserialize;
use serde_json::{Value, json};

use crate::{
    agentes::evaluador::perfil::{Evidencia, TipoEvidencia},
    app::{errores::ErrorApi, estado::Estado},
};

use super::{evidencia, revisiones, trabajos};

#[derive(Deserialize)]
pub struct Consulta {
    revision: Option<i64>,
}

pub async fn criterio(
    State(app): State<Estado>, Path((id, criterio)): Path<(String, String)>,
    Query(consulta): Query<Consulta>, cabeceras: HeaderMap,
) -> Result<Json<Value>, ErrorApi> {
    app.consultar(move |db| {
        let tx = db.transaction()?;
        trabajos::autorizar(&tx, &id, &cabeceras)?;
        let revision = match consulta.revision {
            Some(numero) => revisiones::buscar(&tx, &id, numero)?.ok_or_else(no_encontrada)?,
            None => revisiones::ultima(&tx, &id)?,
        };
        let observacion = revision.resultado.observaciones.iter()
            .find(|observacion| observacion.criterio_id == criterio).ok_or_else(no_encontrada)?;
        let (entrega, operacion, detalle): (String, String, Option<String>) = tx.query_row(
            "SELECT e.entrega_id,e.operacion_id,t.detalle FROM evaluaciones e
             LEFT JOIN trazas_evaluacion t ON t.evaluacion_id=e.id WHERE e.id=?1",
            [&id], |fila| Ok((fila.get(0)?, fila.get(1)?, fila.get(2)?)),
        )?;
        let contexto = evidencia::construir(&tx, &entrega).map_err(ErrorApi::interno)?.ok_or_else(no_encontrada)?;
        let citas = contexto.entrada.evidencias.iter()
            .filter(|dato| observacion.evidencias.contains(&dato.id))
            .map(|dato| Ok(json!({ "evidencia": dato, "procedencia": procedencia(&tx, dato)? })))
            .collect::<Result<Vec<Value>, ErrorApi>>()?;
        let detalle: Option<Value> = detalle.map(|detalle| serde_json::from_str(&detalle)).transpose().map_err(ErrorApi::interno)?;
        let credito = revision.puntaje.criterios.iter().find(|credito| credito.criterio_id == criterio);
        Ok(Json(json!({
            "evaluacion_id": id, "operacion_id": operacion, "revision": revision.revision, "credito": credito,
            "entrega": contexto.entrada.entrega, "rubrica": contexto.entrada.rubrica,
            "observacion": observacion, "citas": citas, "invocacion": detalle.and_then(|mut detalle| detalle.get_mut("invocacion").map(Value::take)),
            "nucleo": "backend/src/agentes/evaluador/08_observabilidad_regresion/README.md",
            "regresion": "backend/tests/agente_evaluador.rs::rastrear_criterio_a_evidencia_y_regresion",
        })))
    }).await
}

fn procedencia(db: &Connection, dato: &Evidencia) -> Result<Value, ErrorApi> {
    let sql = match dato.tipo {
        TipoEvidencia::Turno => {
            let detalle: Option<String> = db.query_row(
                "SELECT t.detalle FROM trazas_turno t JOIN turnos r ON r.id=t.turno_id
                 WHERE r.id=?1 AND r.intento_id=?2",
                params![dato.id, dato.intento_id], |r| r.get(0),
            ).optional()?;
            return detalle.map(|detalle| serde_json::from_str(&detalle).map_err(ErrorApi::interno))
                .transpose().map(|detalle| detalle.unwrap_or(Value::Null));
        }
        TipoEvidencia::Medicion => "SELECT revision_fuente_id FROM mediciones WHERE id=?1 AND intento_id=?2",
        TipoEvidencia::Hallazgo => "SELECT h.revision_fuente_id FROM hallazgos_exploracion h
            JOIN exploraciones e ON h.exploracion_id=e.id WHERE h.id=?1 AND e.intento_id=?2",
        TipoEvidencia::Orden => "SELECT i.revision_fuente_id FROM informes i JOIN ordenes o ON i.orden_id=o.id
            WHERE o.id=?1 AND o.intento_id=?2",
        TipoEvidencia::Informe => "SELECT i.revision_fuente_id FROM informes i JOIN ordenes o ON i.orden_id=o.id
            WHERE i.id=?1 AND o.intento_id=?2",
        TipoEvidencia::Apunte => return Ok(Value::Null),
    };
    Ok(db.query_row(
        &format!("SELECT r.id,f.ruta,f.rol,r.sha256 FROM revisiones_fuente r JOIN fuentes f ON f.id=r.fuente_id WHERE r.id=({sql})"),
        params![dato.id, dato.intento_id],
        |r| Ok(json!({
            "revision_fuente_id": r.get::<_, i64>(0)?, "ruta": r.get::<_, String>(1)?,
            "rol": r.get::<_, String>(2)?, "sha256": r.get::<_, String>(3)?,
        })),
    ).optional()?.unwrap_or(Value::Null))
}

fn no_encontrada() -> ErrorApi {
    ErrorApi::nuevo(StatusCode::NOT_FOUND, "No se encontró el criterio en esa revisión.")
}
