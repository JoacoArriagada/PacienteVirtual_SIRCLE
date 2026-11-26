use axum::{Json, extract::{Path, State}, http::HeaderMap};
use rusqlite::{Connection, OptionalExtension};
use serde::Serialize;
use serde_json::{Value, json};

use crate::{
    app::{errores::ErrorApi, estado::Estado},
    modules::{entrevista::{hechos, turnos}, evaluacion::entregas, examenes::ordenes,
        razonamiento::{evidencias, hipotesis}, valoracion::{hallazgos, mediciones}},
    shared::autorizacion,
    shared::ficha_aprobada::{self, Orientacion, Sugerencia},
};

use super::intento::Intento;

#[derive(Serialize)]
pub struct VistaIntento {
    #[serde(flatten)]
    intento: Intento,
    #[serde(flatten)]
    contenido: Option<SeccionesClinicas>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mediciones: Option<Vec<mediciones::Medicion>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    turnos: Option<Vec<turnos::Turno>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exploraciones: Option<Vec<hallazgos::Exploracion>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ordenes: Option<Vec<ordenes::Orden>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    apuntes: Option<Vec<evidencias::Apunte>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    evidencias: Option<Vec<evidencias::Evidencia>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    borrador: Option<hipotesis::Borrador>,
    #[serde(skip_serializing_if = "Option::is_none")]
    entrega: Option<entregas::Acuse>,
}

#[derive(Serialize)]
struct SeccionesClinicas {
    introduccion: Value,
    ficha: Value,
    indicaciones: Vec<&'static str>,
    ayuda: Orientacion,
    sugerencias: Vec<Sugerencia>,
    respiraciones_simuladas: Option<i64>,
}

pub async fn leer(
    State(app): State<Estado>,
    Path(id): Path<String>,
    cabeceras: HeaderMap,
) -> Result<Json<VistaIntento>, ErrorApi> {
    app.consultar(move |db| {
        let tx = db.transaction()?;
        let (_, intento) = autorizacion::intento_propio(&tx, &cabeceras, &id)?;
        let abierto = intento.pre != "pendiente";
        let mediciones = abierto.then(|| mediciones::historico(&tx, &id, None)).transpose()?;
        let contenido = (abierto && intento.caso == "P01" && intento.fase == "inicial")
            .then(|| vista_inicial_lr(&tx, &intento, mediciones.as_deref().unwrap_or_default())).transpose()?.flatten();
        let turnos = abierto.then(|| turnos::historico(&tx, &id)).transpose()?;
        let exploraciones = abierto.then(|| hallazgos::historico(&tx, &id)).transpose()?;
        let ordenes = abierto.then(|| ordenes::historico(&tx, &id)).transpose()?;
        let apuntes = abierto.then(|| evidencias::apuntes(&tx, &id)).transpose()?;
        let evidencias = abierto.then(|| evidencias::listar(
            &tx, &id, contenido.as_ref().map(|secciones| &secciones.ayuda),
        )).transpose()?;
        let borrador = abierto.then(|| hipotesis::ultimo(&tx, &id)).transpose()?.flatten();
        let entrega = entregas::recuperar_acuse(&tx, &id)?;
        Ok(Json(VistaIntento { intento, contenido, mediciones, turnos, exploraciones, ordenes, apuntes, evidencias, borrador, entrega }))
    }).await
}

fn vista_inicial_lr(
    db: &Connection, intento: &Intento, mediciones: &[mediciones::Medicion],
) -> Result<Option<SeccionesClinicas>, ErrorApi> {
    let triaje: Option<(String, String)> = db.query_row(
        "SELECT l.valor, l.unidad FROM intentos i
         JOIN versiones_clinicas v ON v.id=i.version_caso_id AND v.estado='ensayo'
         JOIN lecturas_versionadas l ON l.version_caso_id=v.id AND l.fase=i.fase
         JOIN revisiones_fuente r ON r.id=l.revision_fuente_id
         WHERE i.id=?1 AND l.instrumento='presion'
           AND instr(CAST(r.contenido AS TEXT), 'LR is a 34-year-old female') > 0
           AND instr(CAST(r.contenido AS TEXT), 'Chief Complaint: Shortness of breath.') > 0",
        [&intento.intento_id],
        |fila| Ok((fila.get(0)?, fila.get(1)?)),
    ).optional()?;
    let Some((presion, unidad)) = triaje else { return Ok(None) };
    let presion: Value = serde_json::from_str(&presion).map_err(ErrorApi::interno)?;
    let lectura = format!("{}/{} {unidad}", presion["sistolica"], presion["diastolica"]);
    let respiraciones_simuladas = db.query_row(
        "SELECT json_extract(l.valor, '$.frecuencia')
         FROM intentos i JOIN lecturas_versionadas l ON l.version_caso_id=i.version_caso_id AND l.fase=i.fase
         WHERE i.id=?1 AND l.instrumento='respiracion'",
        [&intento.intento_id],
        |fila| fila.get(0),
    ).optional()?;
    Ok(Some(SeccionesClinicas {
        introduccion: json!({
            "paciente": "LR", "edad": 34, "motivo": "Falta de aire",
            "contexto": "Servicio de urgencias. Práctica técnica con material de ensayo.",
        }),
        ficha: json!({
            "fuente": "Triaje de ensayo de LR", "fase": "inicial",
            "datos": [{ "etiqueta": "Presión inicial", "valor": lectura }],
            "hallazgos": mediciones.iter().filter(|medicion| medicion.en_ficha).collect::<Vec<_>>(),
        }),
        indicaciones: vec![
            "Entrevista a LR y registra las mediciones que realices.",
            "Vincula los hallazgos que sustenten tu hipótesis.",
        ],
        ayuda: ficha_aprobada::ORIENTACION_INICIAL_LR,
        sugerencias: ficha_aprobada::sugerencias(&hechos::candidatos_lr(db, intento)?),
        respiraciones_simuladas,
    }))
}
