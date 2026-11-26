use axum::{
    Json,
    extract::State,
    http::{HeaderMap, StatusCode},
};
use rusqlite::{Connection, OptionalExtension, Transaction, TransactionBehavior, params};
use serde::{Deserialize, Serialize};

use crate::{
    app::{errores::ErrorApi, estado::Estado},
    modules::acceso::{actividades::actividad_no_asignada, sesiones},
    shared::operaciones,
};

#[derive(Deserialize)]
pub struct AltaIntento {
    actividad_id: String,
}

#[derive(Serialize)]
pub struct Intento {
    pub intento_id: String,
    pub actividad_id: String,
    pub estudiante_id: String,
    pub tipo: String,
    pub caso: String,
    pub version_caso: String,
    pub version_rubrica: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rubrica_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rubrica_sha256: Option<String>,
    pub version_instrumento: String,
    pub pre: String,
    pub fase: String,
    pub estado: String,
    pub revision: i64,
}

pub(crate) fn preparar_ensayo(db: &Transaction<'_>, actividad: &str, original_id: i64) -> rusqlite::Result<()> {
    db.execute(
        "INSERT INTO versiones_clinicas (escenario_id, version, estado)
         SELECT escenario_id, 'ensayo-' || version, 'ensayo' FROM versiones_clinicas WHERE id=?1
         ON CONFLICT(escenario_id, version) DO NOTHING",
        [original_id],
    )?;
    let ensayo: i64 = db.query_row(
        "SELECT ensayo.id FROM versiones_clinicas original
         JOIN versiones_clinicas ensayo ON ensayo.escenario_id=original.escenario_id
          AND ensayo.version='ensayo-' || original.version AND ensayo.estado='ensayo'
         WHERE original.id=?1",
        [original_id],
        |fila| fila.get(0),
    )?;
    db.execute(
        "INSERT INTO versiones_fuentes (escenario_id, version_clinica_id, fuente_id, revision_fuente_id)
         SELECT escenario_id, ?1, fuente_id, revision_fuente_id FROM versiones_fuentes
         WHERE version_clinica_id=?2 ON CONFLICT(version_clinica_id, fuente_id) DO NOTHING",
        params![ensayo, original_id],
    )?;
    db.execute(
        "INSERT INTO configuraciones_ensayo
         SELECT ?1, escenario_id, id, 'rubrica-ensayo-v1', 'instrumento-ensayo-v1' FROM versiones_clinicas WHERE id=?2
         ON CONFLICT(actividad_id) DO UPDATE SET version_caso_id=excluded.version_caso_id",
        params![actividad, ensayo],
    )?;
    Ok(())
}

pub async fn crear(
    State(app): State<Estado>,
    cabeceras: HeaderMap,
    Json(alta): Json<AltaIntento>,
) -> Result<(StatusCode, Json<Intento>), ErrorApi> {
    app.consultar(move |db| {
        let tx = db.transaction_with_behavior(TransactionBehavior::Immediate)?;
        let identidad = sesiones::autenticar(&tx, &cabeceras)?;
        let asignada: bool = tx.query_row(
            "SELECT EXISTS(SELECT 1 FROM asignaciones s JOIN actividades a ON a.id=s.actividad_id
             WHERE s.estudiante_id=?1 AND a.id=?2 AND a.habilitada=1)",
            params![identidad.id, alta.actividad_id],
            |fila| fila.get(0),
        )?;
        if identidad.rol != "estudiante" || !asignada {
            return Err(actividad_no_asignada());
        }
        operaciones::clave(&cabeceras)?;

        let nuevo = tx.execute(
            "INSERT INTO intentos (id, actividad_id, estudiante_id, escenario_id, version_caso_id,
                                   version_rubrica, version_instrumento, pre, estado, fase, revision,
                                   rubrica_id, rubrica_sha256)
             SELECT lower(hex(randomblob(16))), a.id, ?1, a.escenario_id, c.version_caso_id,
                    coalesce(r.version,c.version_rubrica), c.version_instrumento, 'pendiente',
                    'pre_pendiente', 'inicial', 0, r.id, r.sha256
             FROM actividades a JOIN configuraciones_ensayo c ON a.id=c.actividad_id
             JOIN versiones_clinicas v ON v.id=c.version_caso_id
             LEFT JOIN rubricas_activas_ensayo ra ON ra.actividad_id=a.id
             LEFT JOIN rubricas r ON r.id=ra.rubrica_id AND r.actividad_id=a.id
               AND r.caso=a.escenario_id AND r.estado='aprobada'
             WHERE a.id=?2 AND a.tipo='ensayo' AND v.estado='ensayo'
               AND (ra.rubrica_id IS NULL OR r.id IS NOT NULL)
             ON CONFLICT(actividad_id, estudiante_id) DO NOTHING",
            params![identidad.id, alta.actividad_id],
        )?;
        let id: String = tx.query_row(
            "SELECT id FROM intentos WHERE estudiante_id=?1 AND actividad_id=?2",
            params![identidad.id, alta.actividad_id],
            |fila| fila.get(0),
        ).optional()?.ok_or_else(|| ErrorApi::nuevo(
            StatusCode::CONFLICT, "La actividad todavía no tiene versiones habilitadas.",
        ))?;
        let intento = recuperar(&tx, &id, &identidad.id)?;
        tx.commit()?;
        let estado = if nuevo == 1 { StatusCode::CREATED } else { StatusCode::OK };
        Ok((estado, Json(intento)))
    }).await
}

pub fn recuperar(db: &Connection, id: &str, estudiante: &str) -> Result<Intento, ErrorApi> {
    db.query_row(
        "SELECT i.id, i.actividad_id, i.estudiante_id, a.tipo, i.escenario_id,
                v.version, i.version_rubrica, i.version_instrumento, i.pre, i.fase, i.estado,
                i.revision, i.rubrica_id, i.rubrica_sha256
         FROM intentos i JOIN actividades a ON i.actividad_id=a.id
         JOIN asignaciones s ON s.actividad_id=i.actividad_id AND s.estudiante_id=i.estudiante_id
         JOIN versiones_clinicas v ON v.id=i.version_caso_id
         WHERE i.id=?1 AND i.estudiante_id=?2 AND a.habilitada=1",
        params![id, estudiante],
        |fila| Ok(Intento {
            intento_id: fila.get(0)?, actividad_id: fila.get(1)?, estudiante_id: fila.get(2)?,
            tipo: fila.get(3)?, caso: fila.get(4)?, version_caso: fila.get(5)?,
            version_rubrica: fila.get(6)?, version_instrumento: fila.get(7)?, pre: fila.get(8)?,
            fase: fila.get(9)?, estado: fila.get(10)?, revision: fila.get(11)?,
            rubrica_id: fila.get(12)?, rubrica_sha256: fila.get(13)?,
        }),
    ).optional()?.ok_or_else(actividad_no_asignada)
}

pub fn resolver_pre_ensayo(db: &mut Connection, id: &str) -> anyhow::Result<()> {
    let tx = db.transaction_with_behavior(TransactionBehavior::Immediate)?;
    let autorizado: bool = tx.query_row(
        "SELECT EXISTS(SELECT 1 FROM intentos i JOIN actividades a ON a.id=i.actividad_id
         WHERE i.id=?1 AND a.tipo='ensayo' AND a.habilitada=1 AND i.estado IN ('pre_pendiente', 'atencion'))",
        [id],
        |fila| fila.get(0),
    )?;
    anyhow::ensure!(autorizado, "Solo se puede resolver el pre de un intento de ensayo abierto.");
    tx.execute(
        "UPDATE intentos SET pre='omitido', estado='atencion', revision=revision+1
         WHERE id=?1 AND estado='pre_pendiente'",
        [id],
    )?;
    tx.commit()?;
    Ok(())
}

