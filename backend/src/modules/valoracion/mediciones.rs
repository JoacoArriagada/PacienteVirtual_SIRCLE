use axum::{
    Json,
    extract::{Path, Query, State},
    http::{HeaderMap, StatusCode},
};
use rusqlite::{Connection, OptionalExtension, Row, Transaction, TransactionBehavior, params, types::Type};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    app::{errores::ErrorApi, estado::Estado},
    modules::intentos::intento::Intento,
    shared::{autorizacion, operaciones::Operacion},
};

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Toma {
    instrumento: String,
    accion: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    ciclos: Option<u16>,
}

#[derive(Deserialize)]
pub struct Filtro {
    instrumento: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct Medicion {
    pub(crate) medicion_id: String,
    pub(crate) intento_id: String,
    instrumento: String,
    accion: String,
    fase: String,
    valor: Value,
    unidad: String,
    momento: String,
    pub(crate) en_ficha: bool,
    revision: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    origen: Option<String>,
}

pub(crate) fn preparar_presion_lr(db: &Transaction<'_>) -> rusqlite::Result<()> {
    db.execute(
        "INSERT INTO lecturas_versionadas (version_caso_id, fase, instrumento, valor, unidad, revision_fuente_id)
         SELECT c.version_caso_id, 'inicial', 'presion', '{\"sistolica\":148,\"diastolica\":85}', 'mmHg', r.id
         FROM configuraciones_ensayo c
         JOIN versiones_fuentes vf ON vf.version_clinica_id=c.version_caso_id
         JOIN fuentes f ON f.id=vf.fuente_id AND f.rol='sp_actor'
         JOIN revisiones_fuente r ON r.id=vf.revision_fuente_id
         WHERE c.actividad_id='lr-ensayo' AND instr(CAST(r.contenido AS TEXT), 'Vitals: BP 148/85 HR 102') > 0
         ON CONFLICT(version_caso_id, fase, instrumento) DO NOTHING",
        [],
    )?;
    Ok(())
}

pub(crate) fn preparar_signos_lr(db: &Transaction<'_>) -> rusqlite::Result<()> {
    for (instrumento, valor, unidad, pasaje) in [
        ("pulso", r#"{"frecuencia":102}"#, "lpm", "HR 102"),
        ("respiracion", r#"{"frecuencia":26,"segundos_reales":24,"segundos_simulados":60}"#, "resp/min", "RR 26"),
        ("temperatura", r#"{"lectura":36.6}"#, "°C", "Temp 97.9"),
        ("saturacion", r#"{"lectura":84}"#, "%", "SaO2 84% on RA"),
    ] {
        db.execute(
            "INSERT INTO lecturas_versionadas (version_caso_id, fase, instrumento, valor, unidad, revision_fuente_id)
             SELECT l.version_caso_id, l.fase, ?1, ?2, ?3, l.revision_fuente_id
             FROM configuraciones_ensayo c
             JOIN lecturas_versionadas l ON l.version_caso_id=c.version_caso_id
             JOIN revisiones_fuente r ON r.id=l.revision_fuente_id
             WHERE c.actividad_id='lr-ensayo' AND l.instrumento='presion'
               AND instr(CAST(r.contenido AS TEXT), ?4) > 0
             ON CONFLICT(version_caso_id, fase, instrumento) DO NOTHING",
            params![instrumento, valor, unidad, pasaje],
        )?;
    }
    Ok(())
}

pub(crate) fn preparar_peso_lr(db: &Transaction<'_>) -> anyhow::Result<()> {
    let peso: Value = serde_json::from_str(include_str!("../../../tests/fixtures/lr/peso_semana5.json"))?;
    let valor = serde_json::to_string(&peso["valor"])?;
    let procedencia = serde_json::to_string(&peso["procedencia"])?;
    db.execute(
        "INSERT INTO lecturas_versionadas
             (version_caso_id, fase, instrumento, valor, unidad, revision_fuente_id, procedencia_ensayo)
         SELECT l.version_caso_id, l.fase, 'peso', ?1, ?2, l.revision_fuente_id, ?3
         FROM configuraciones_ensayo c
         JOIN lecturas_versionadas l ON l.version_caso_id=c.version_caso_id AND l.instrumento='presion'
         JOIN versiones_clinicas v ON v.id=l.version_caso_id AND v.estado='ensayo'
         WHERE c.actividad_id='lr-ensayo'
         ON CONFLICT(version_caso_id, fase, instrumento) DO NOTHING",
        params![valor, peso["unidad"].as_str(), procedencia],
    )?;
    Ok(())
}

pub async fn crear(
    State(app): State<Estado>,
    Path(id): Path<String>,
    cabeceras: HeaderMap,
    Json(toma): Json<Toma>,
) -> Result<(StatusCode, Json<Medicion>), ErrorApi> {
    app.consultar(move |db| {
        let tx = db.transaction_with_behavior(TransactionBehavior::Immediate)?;
        let (identidad, intento) = autorizacion::intento_propio(&tx, &cabeceras, &id)?;
        let operacion = Operacion::nueva(&identidad.id, &id, &cabeceras, "POST mediciones", &toma)?;
        if let Some(medicion) = operacion.recuperar(&tx)? {
            return Ok((StatusCode::OK, Json(medicion)));
        }
        let medicion = guardar(&tx, &intento, &toma)?;
        operacion.confirmar(&tx, &medicion)?;
        tx.commit()?;
        Ok((StatusCode::CREATED, Json(medicion)))
    }).await
}

fn guardar(db: &Transaction<'_>, intento: &Intento, toma: &Toma) -> Result<Medicion, ErrorApi> {
    let toma_valida = matches!(
        (toma.instrumento.as_str(), toma.ciclos),
        ("presion" | "pulso" | "temperatura" | "saturacion" | "peso", None) | ("respiracion", Some(0..=120))
    );
    if toma.accion != "toma_completa" || !toma_valida {
        return Err(ErrorApi::nuevo(StatusCode::UNPROCESSABLE_ENTITY, "Completa la toma con el instrumento seleccionado antes de confirmar."));
    }
    if intento.estado != "atencion" || intento.fase != "inicial" {
        return Err(ErrorApi::nuevo(StatusCode::CONFLICT, "La fase del intento no permite esta medición."));
    }
    let alta = db.query_row(
        "INSERT INTO mediciones (id, intento_id, instrumento, accion, fase, valor, unidad,
                                 revision_fuente_id, momento, revision, procedencia_ensayo)
         SELECT lower(hex(randomblob(16))), i.id, l.instrumento, ?2, i.fase,
                CASE WHEN l.instrumento='respiracion' THEN json_set(l.valor, '$.ciclos', ?4) ELSE l.valor END, l.unidad,
                l.revision_fuente_id, strftime('%Y-%m-%dT%H:%M:%fZ', 'now'), i.revision+1, l.procedencia_ensayo
         FROM intentos i JOIN lecturas_versionadas l ON l.version_caso_id=i.version_caso_id AND l.fase=i.fase
         WHERE i.id=?1 AND l.instrumento=?3
           AND (l.instrumento <> 'respiracion' OR ?4 <= json_extract(l.valor, '$.frecuencia'))
         RETURNING id, intento_id, instrumento, accion, fase, valor, unidad, momento, en_ficha, revision",
        params![intento.intento_id, toma.accion, toma.instrumento, toma.ciclos],
        leer_medicion,
    ).optional()?.ok_or_else(|| ErrorApi::nuevo(StatusCode::CONFLICT, "La versión de este intento no habilita la lectura."))?;
    db.execute("UPDATE intentos SET revision=?1 WHERE id=?2", params![alta.revision, intento.intento_id])?;
    Ok(alta)
}

pub async fn listar(
    State(app): State<Estado>,
    Path(id): Path<String>,
    Query(filtro): Query<Filtro>,
    cabeceras: HeaderMap,
) -> Result<Json<Vec<Medicion>>, ErrorApi> {
    app.consultar(move |db| {
        autorizacion::intento_propio(db, &cabeceras, &id)?;
        historico(db, &id, filtro.instrumento.as_deref()).map(Json)
    }).await
}

pub(crate) fn historico(db: &Connection, id: &str, instrumento: Option<&str>) -> Result<Vec<Medicion>, ErrorApi> {
    let mut consulta = db.prepare(
        "SELECT id, intento_id, instrumento, accion, fase, valor, unidad, momento, en_ficha, revision
         FROM mediciones WHERE intento_id=?1 AND (?2 IS NULL OR instrumento=?2) ORDER BY rowid",
    )?;
    let filas = consulta.query_map(params![id, instrumento], leer_medicion)?;
    Ok(filas.collect::<rusqlite::Result<_>>()?)
}

pub(crate) fn por_id(db: &Connection, intento_id: &str, id: &str) -> Result<Medicion, ErrorApi> {
    db.query_row(
        "SELECT id, intento_id, instrumento, accion, fase, valor, unidad, momento, en_ficha, revision
         FROM mediciones WHERE intento_id=?1 AND id=?2",
        params![intento_id, id],
        leer_medicion,
    ).optional()?.ok_or_else(|| ErrorApi::nuevo(
        StatusCode::FORBIDDEN, "La medición no pertenece a este intento.",
    ))
}

fn leer_medicion(fila: &Row<'_>) -> rusqlite::Result<Medicion> {
    let instrumento: String = fila.get(2)?;
    let origen = match instrumento.as_str() {
        "pulso" | "saturacion" => Some("Pulsioxímetro simulado".to_owned()),
        "temperatura" => Some("Termómetro frontal simulado".to_owned()),
        "peso" => Some("Báscula de ensayo, Semana 5".to_owned()),
        "respiracion" => Some("Observación y conteo del estudiante".to_owned()),
        _ => None,
    };
    let valor: String = fila.get(5)?;
    let valor = serde_json::from_str(&valor)
        .map_err(|error| rusqlite::Error::FromSqlConversionFailure(5, Type::Text, Box::new(error)))?;
    Ok(Medicion {
        medicion_id: fila.get(0)?, intento_id: fila.get(1)?, instrumento,
        accion: fila.get(3)?, fase: fila.get(4)?, valor, unidad: fila.get(6)?, momento: fila.get(7)?,
        en_ficha: fila.get(8)?, revision: fila.get(9)?, origen,
    })
}
