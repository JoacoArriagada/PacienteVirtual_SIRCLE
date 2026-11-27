use axum::http::StatusCode;
use rusqlite::{Connection, params};
use serde::{Deserialize, Serialize};

use crate::{
    agentes::{dan_sparks::perfil as dan, lr::perfil, mensajes::Agente, nannu::perfil as nannu},
    app::errores::ErrorApi,
    modules::{entrevista::hechos::{self, Hecho}, intentos::intento::Intento},
};

use super::autorizacion;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Intercambio {
    pub turno_id: String,
    pub hablante: String,
    pub pregunta: String,
    pub respuesta: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub hechos: Vec<String>,
    #[serde(default)]
    pub redirigio: bool,
}

#[derive(Debug, Eq, PartialEq, Serialize)]
pub struct ContextoEntrevista {
    pub estudiante_id: String,
    pub intento_id: String,
    pub agente: Agente,
    pub caso: String,
    pub version: String,
    pub fase: String,
    pub hablante: String,
    pub memoria: Vec<Intercambio>,
    pub candidatos: Vec<Hecho>,
}

pub fn voz_autorizada(agente: Agente, caso: &str, hablante: &str) -> bool {
    matches!((agente, caso, hablante),
        (perfil::AGENTE, perfil::ESCENARIO, perfil::HABLANTE)
        | (nannu::AGENTE, nannu::ESCENARIO, nannu::PACIENTE | nannu::ACOMPANANTE)
        | (dan::AGENTE, dan::ESCENARIO, dan::HABLANTE))
}

pub fn entrevista(db: &Connection, intento: &Intento, agente: Agente, hablante: &str) -> Result<ContextoEntrevista, ErrorApi> {
    autorizacion::entrevista_nueva(intento)?;
    if !voz_autorizada(agente, &intento.caso, hablante) {
        return Err(ErrorApi::nuevo(StatusCode::FORBIDDEN, "El agente o el hablante no está autorizado para el caso."));
    }
    let candidatos = hechos::candidatos(db, intento, hablante)?;
    let mut consulta = db.prepare(
        "SELECT t.id, t.hablante, t.pregunta, t.respuesta,
                CASE WHEN json_type(tr.detalle, '$.hechos')='array' THEN
                    (SELECT json_group_array(json_extract(h.value,'$.id')) FROM json_each(tr.detalle, '$.hechos') h)
                WHEN json_type(tr.detalle, '$.hecho.id')='text' THEN json_array(json_extract(tr.detalle, '$.hecho.id'))
                ELSE '[]' END,
                coalesce(json_extract(tr.detalle,'$.redirigio'),0)
         FROM turnos t LEFT JOIN trazas_turno tr ON tr.turno_id=t.id
         JOIN intentos i ON i.id=t.intento_id
         JOIN versiones_clinicas v ON v.id=i.version_caso_id
         WHERE i.id=?1 AND i.estudiante_id=?2 AND t.agente=?3
           AND i.escenario_id=?4 AND v.version=?5
           AND i.fase=?6 AND t.fase=?6 AND t.hablante=?7 AND i.estado='atencion' AND i.pre <> 'pendiente'
         ORDER BY t.orden DESC LIMIT 3",
    )?;
    let filas = consulta.query_map(params![intento.intento_id, intento.estudiante_id, agente.id(),
        intento.caso, intento.version_caso, intento.fase, hablante], |fila| Ok(Intercambio {
        turno_id: fila.get(0)?, hablante: fila.get(1)?, pregunta: fila.get(2)?, respuesta: fila.get(3)?,
        hechos: serde_json::from_str(fila.get_ref(4)?.as_str()?).map_err(|error|
            rusqlite::Error::FromSqlConversionFailure(4, rusqlite::types::Type::Text, Box::new(error)))?,
        redirigio: fila.get(5)?,
    }))?;
    let mut memoria: Vec<_> = filas.collect::<rusqlite::Result<_>>()?;
    memoria.reverse();
    Ok(ContextoEntrevista {
        estudiante_id: intento.estudiante_id.clone(), intento_id: intento.intento_id.clone(),
        agente, caso: intento.caso.clone(), version: intento.version_caso.clone(), fase: intento.fase.clone(),
        hablante: hablante.into(), memoria, candidatos,
    })
}
