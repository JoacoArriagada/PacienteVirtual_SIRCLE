use axum::{
    Json,
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
};
use rusqlite::{Connection, Transaction};
use serde::Serialize;

use crate::app::{errores::ErrorApi, estado::Estado};

use super::sesiones;

#[derive(Serialize)]
pub struct Actividad {
    actividad_id: String,
    nombre: String,
    tipo: String,
    pre: String,
    intento_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    estado: Option<String>,
}

pub(crate) fn preparar_ensayo(db: &Transaction<'_>) -> rusqlite::Result<()> {
    db.execute_batch(
        "INSERT INTO identidades VALUES
           ('estudiante-lr', 'estudiante'),
           ('estudiante-sin-asignacion', 'estudiante'),
           ('docente-demo', 'docente')
         ON CONFLICT(id) DO NOTHING;
         INSERT INTO actividades (id, escenario_id, nombre, tipo, habilitada)
           VALUES ('lr-ensayo', 'P01', 'Práctica técnica de LR', 'ensayo', 1)
         ON CONFLICT(id) DO NOTHING;
         INSERT INTO asignaciones VALUES ('lr-ensayo', 'estudiante-lr', 'docente-demo')
         ON CONFLICT(actividad_id, estudiante_id) DO NOTHING;",
    )
}

pub async fn listar(
    State(app): State<Estado>,
    cabeceras: HeaderMap,
) -> Result<Json<Vec<Actividad>>, ErrorApi> {
    app.consultar(move |db| permitidas(db, &cabeceras)).await.map(Json)
}

pub async fn leer(
    State(app): State<Estado>,
    Path(id): Path<String>,
    cabeceras: HeaderMap,
) -> Result<Json<Actividad>, ErrorApi> {
    app.consultar(move |db| {
        permitidas(db, &cabeceras)?.into_iter()
            .find(|actividad| actividad.actividad_id == id)
            .ok_or_else(actividad_no_asignada)
    }).await.map(Json)
}

fn permitidas(db: &Connection, cabeceras: &HeaderMap) -> Result<Vec<Actividad>, ErrorApi> {
    let identidad = sesiones::autenticar(db, cabeceras)?;
    let mut consulta = db.prepare(
        "SELECT a.id, a.nombre, a.tipo, coalesce(i.pre, 'pendiente'), i.id, i.estado FROM actividades a
         JOIN asignaciones s ON s.actividad_id=a.id
         LEFT JOIN intentos i ON i.actividad_id=a.id AND i.estudiante_id=s.estudiante_id
         WHERE s.estudiante_id=?1 AND a.habilitada=1 ORDER BY a.id",
    )?;
    let filas = consulta.query_map([identidad.id], |fila| {
        Ok(Actividad {
            actividad_id: fila.get(0)?,
            nombre: fila.get(1)?,
            tipo: fila.get(2)?,
            pre: fila.get(3)?,
            intento_id: fila.get(4)?,
            estado: fila.get(5)?,
        })
    })?;
    Ok(filas.collect::<rusqlite::Result<_>>()?)
}

pub(crate) fn actividad_no_asignada() -> ErrorApi {
    ErrorApi::nuevo(StatusCode::FORBIDDEN, "Esta actividad no está asignada a tu identidad.")
}
