use axum::{Json, extract::{Path, State}, http::{HeaderMap, StatusCode}};
use rusqlite::{Connection, OptionalExtension, Row, Transaction, TransactionBehavior, params};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    app::{errores::ErrorApi, estado::Estado},
    shared::{autorizacion, operaciones::Operacion},
};

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Peticion {
    instrumento: String,
    zona: String,
    accion: String,
}

#[derive(Deserialize, Serialize)]
pub struct Exploracion {
    pub(crate) exploracion_id: String,
    intento_id: String,
    instrumento: String,
    zona: String,
    etiqueta: String,
    fase: String,
    momento: String,
    revision: i64,
    estado: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    hallazgo: Option<Hallazgo>,
}

#[derive(Deserialize, Serialize)]
struct Hallazgo {
    hallazgo_id: String,
    texto: String,
    origen: String,
}

pub(crate) fn preparar_exploracion_lr(db: &Transaction<'_>) -> rusqlite::Result<()> {
    for (instrumento, zona, etiqueta) in [
        ("inspeccion", "frontal", "Tórax anterior"),
        ("inspeccion", "posterior", "Tórax posterior"),
        ("palpacion", "central", "Tórax anterior"),
        ("palpacion", "extremidad", "Pantorrilla derecha"),
        ("auscultacion", "postLeft", "Base izquierda"),
        ("auscultacion", "postRight", "Base derecha"),
        ("auscultacion", "antRight", "Campo derecho"),
        ("auscultacion", "antLeft", "Campo izquierdo"),
        ("auscultacion", "aortic", "Foco aórtico"),
        ("auscultacion", "mitral", "Foco mitral"),
    ] {
        db.execute(
            "INSERT INTO zonas_exploracion (version_caso_id, fase, instrumento, zona, etiqueta)
             SELECT version_caso_id, 'inicial', ?1, ?2, ?3 FROM configuraciones_ensayo
             WHERE actividad_id='lr-ensayo'
             ON CONFLICT(version_caso_id, fase, instrumento, zona) DO NOTHING",
            params![instrumento, zona, etiqueta],
        )?;
    }
    db.execute(
        "UPDATE zonas_exploracion AS z
         SET (resultado, revision_fuente_id) = (
             SELECT 'Se observa esfuerzo respiratorio con uso de musculatura accesoria. LR habla en frases cortas.', r.id
             FROM versiones_fuentes vf JOIN fuentes f ON f.id=vf.fuente_id AND f.rol='examinee'
             JOIN revisiones_fuente r ON r.id=vf.revision_fuente_id
             WHERE vf.version_clinica_id=z.version_caso_id
               AND instr(CAST(r.contenido AS TEXT), 'Accessory muscle use, nasal flaring. Speaking in 3-word sentences.') > 0
         )
         WHERE z.version_caso_id=(SELECT version_caso_id FROM configuraciones_ensayo WHERE actividad_id='lr-ensayo')
           AND z.fase='inicial' AND z.instrumento='inspeccion' AND z.zona='frontal' AND z.resultado IS NULL",
        [],
    )?;
    let prueba_palpacion = json!({
        "ruta": "docs/referencias/prototipo-html/Paciente_Virtual.html",
        "sha256": "ff9dea8958759550c5241ab9f0560c46f2726bf04ba1d46f927fb9febfeee032",
        "localizacion": "CASES.lr.exam.central",
        "naturaleza": "dato de ensayo",
    }).to_string();
    db.execute(
        "UPDATE zonas_exploracion AS z
         SET resultado='Expansión palpable en ambos lados. No refiere dolor localizado con el contacto superficial.',
             revision_fuente_id=(SELECT l.revision_fuente_id FROM lecturas_versionadas l
                                  WHERE l.version_caso_id=z.version_caso_id AND l.fase=z.fase AND l.instrumento='presion'),
             procedencia_ensayo=?1
         WHERE z.version_caso_id=(SELECT version_caso_id FROM configuraciones_ensayo WHERE actividad_id='lr-ensayo')
           AND z.fase='inicial' AND z.instrumento='palpacion' AND z.zona='central' AND z.resultado IS NULL
           AND EXISTS (SELECT 1 FROM lecturas_versionadas l WHERE l.version_caso_id=z.version_caso_id
                      AND l.fase=z.fase AND l.instrumento='presion')",
        [prueba_palpacion],
    )?;
    db.execute(
        "UPDATE zonas_exploracion AS z
         SET (resultado, revision_fuente_id) = (
             SELECT 'Se oyen silbidos al final de la espiración y la entrada de aire está disminuida.', r.id
             FROM versiones_fuentes vf
             JOIN fuentes f ON f.id=vf.fuente_id AND f.rol='environment_controller'
             JOIN revisiones_fuente r ON r.id=vf.revision_fuente_id
             WHERE vf.version_clinica_id=z.version_caso_id
              AND instr(CAST(r.contenido AS TEXT), 'Diffuse end-expiratory wheezing, decreased air entry bilaterally.') > 0
         )
         WHERE z.version_caso_id=(SELECT version_caso_id FROM configuraciones_ensayo WHERE actividad_id='lr-ensayo')
           AND z.fase='inicial' AND z.instrumento='auscultacion' AND z.zona='postLeft' AND z.resultado IS NULL",
        [],
    )?;
    Ok(())
}

pub async fn crear(
    State(app): State<Estado>, Path(id): Path<String>, cabeceras: HeaderMap, Json(peticion): Json<Peticion>,
) -> Result<(StatusCode, Json<Exploracion>), ErrorApi> {
    app.consultar(move |db| {
        let tx = db.transaction_with_behavior(TransactionBehavior::Immediate)?;
        let (identidad, intento) = autorizacion::intento_propio(&tx, &cabeceras, &id)?;
        let operacion = Operacion::nueva(&identidad.id, &id, &cabeceras, "POST exploraciones", &peticion)?;
        if let Some(guardada) = operacion.recuperar(&tx)? {
            return Ok((StatusCode::OK, Json(guardada)));
        }
        if peticion.accion != "exploracion_completa" {
            return Err(ErrorApi::nuevo(StatusCode::UNPROCESSABLE_ENTITY, "Completa la exploración antes de confirmar."));
        }
        if intento.estado != "atencion" || intento.fase != "inicial" {
            return Err(ErrorApi::nuevo(StatusCode::CONFLICT, "La fase actual no permite explorar al paciente."));
        }
        let etiqueta: String = tx.query_row(
            "SELECT z.etiqueta FROM zonas_exploracion z JOIN intentos i ON i.version_caso_id=z.version_caso_id
             AND i.fase=z.fase WHERE i.id=?1 AND z.instrumento=?2 AND z.zona=?3",
            params![id, peticion.instrumento, peticion.zona],
            |fila| fila.get(0),
        ).optional()?.ok_or_else(|| ErrorApi::nuevo(StatusCode::UNPROCESSABLE_ENTITY, "Selecciona una zona del instrumento."))?;
        let exploracion_id: String = tx.query_row(
            "INSERT INTO exploraciones VALUES
              (lower(hex(randomblob(16))), ?1, ?2, ?3, ?4, ?5, strftime('%Y-%m-%dT%H:%M:%fZ','now'), ?6)
             RETURNING id",
            params![id, peticion.instrumento, peticion.zona, etiqueta, intento.fase, intento.revision + 1],
            |fila| fila.get(0),
        )?;
        tx.execute(
            "INSERT INTO hallazgos_exploracion (id, exploracion_id, texto, revision_fuente_id, procedencia_ensayo)
             SELECT lower(hex(randomblob(16))), e.id, z.resultado, z.revision_fuente_id, z.procedencia_ensayo
             FROM exploraciones e JOIN intentos i ON i.id=e.intento_id
             JOIN zonas_exploracion z ON z.version_caso_id=i.version_caso_id
              AND z.fase=e.fase AND z.instrumento=e.instrumento AND z.zona=e.zona
             WHERE e.id=?1 AND z.resultado IS NOT NULL",
            [&exploracion_id],
        )?;
        let confirmada = tx.query_row(&format!("{CONSULTA} WHERE e.id=?1"), [&exploracion_id], desde_fila)?;
        tx.execute("UPDATE intentos SET revision=?1 WHERE id=?2", params![confirmada.revision, id])?;
        operacion.confirmar(&tx, &confirmada)?;
        tx.commit()?;
        Ok((StatusCode::CREATED, Json(confirmada)))
    }).await
}

pub async fn listar(
    State(app): State<Estado>, Path(id): Path<String>, cabeceras: HeaderMap,
) -> Result<Json<Vec<Exploracion>>, ErrorApi> {
    app.consultar(move |db| {
        autorizacion::intento_propio(db, &cabeceras, &id)?;
        historico(db, &id).map(Json)
    }).await
}

pub(crate) fn historico(db: &Connection, intento_id: &str) -> Result<Vec<Exploracion>, ErrorApi> {
    let mut consulta = db.prepare(&format!("{CONSULTA} WHERE e.intento_id=?1 ORDER BY e.rowid"))?;
    let filas = consulta.query_map([intento_id], desde_fila)?;
    Ok(filas.collect::<rusqlite::Result<_>>()?)
}

const CONSULTA: &str =
    "SELECT e.id, e.intento_id, e.instrumento, e.zona, e.etiqueta, e.fase, e.momento, e.revision,
            h.id, h.texto, h.procedencia_ensayo
     FROM exploraciones e LEFT JOIN hallazgos_exploracion h ON h.exploracion_id=e.id";

fn desde_fila(fila: &Row<'_>) -> rusqlite::Result<Exploracion> {
    let hallazgo_id: Option<String> = fila.get(8)?;
    let hallazgo = if let Some(hallazgo_id) = hallazgo_id {
        let procedencia: Option<String> = fila.get(10)?;
        Some(Hallazgo {
            hallazgo_id,
            texto: fila.get(9)?,
            origen: if procedencia.is_some() { "Material de ensayo del prototipo".into() } else { "Versión de ensayo de LR".into() },
        })
    } else {
        None
    };
    Ok(Exploracion {
        exploracion_id: fila.get(0)?, intento_id: fila.get(1)?, instrumento: fila.get(2)?,
        zona: fila.get(3)?, etiqueta: fila.get(4)?, fase: fila.get(5)?, momento: fila.get(6)?,
        revision: fila.get(7)?, estado: if hallazgo.is_some() { "habilitado".into() } else { "pendiente".into() },
        hallazgo,
    })
}
