use std::collections::{BTreeMap, HashSet};

use anyhow::{Result, ensure};
use serde::{Deserialize, Serialize};

use crate::agentes::mensajes::Agente;

pub const AGENTE: Agente = Agente::Evaluador;
pub const NIVEL: &str = "pregrado_avanzado";

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Entrada {
    pub entrega: Entrega,
    pub rubrica: RubricaFijada,
    pub evidencias: Vec<Evidencia>,
    pub hipotesis: Vec<Hipotesis>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Entrega {
    pub id: String,
    pub intento_id: String,
    pub revision: i64,
    pub caso: String,
    pub version_caso: String,
    pub confirmada: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RubricaFijada {
    pub id: String,
    pub version: String,
    pub sha256: String,
    pub nivel: String,
    pub criterios: Vec<Criterio>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Criterio {
    pub id: String,
    pub expectativa: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Evidencia {
    pub id: String,
    pub intento_id: String,
    pub tipo: TipoEvidencia,
    pub resumen: String,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TipoEvidencia {
    Turno,
    Medicion,
    Hallazgo,
    Orden,
    Informe,
    Apunte,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Hipotesis {
    pub rol: String,
    pub titulo: String,
    pub explicacion: String,
    pub evidencias: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Borrador {
    pub entrega_id: String,
    pub rubrica_sha256: String,
    pub observaciones: Vec<Observacion>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prioridad: Option<Prioridad>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Prioridad {
    pub criterio_id: String,
    pub evidencia_id: String,
    pub motivo: String,
    pub accion: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(from = "FormatoPropuesta")]
pub struct Propuesta {
    pub observaciones: Vec<PropuestaCriterio>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PropuestaCriterio {
    pub criterio_id: String,
    pub actuacion: String,
    pub evidencias: Vec<String>,
    pub juicio: Juicio,
    pub explicacion: String,
    pub mejora: String,
}

#[derive(Deserialize)]
#[serde(untagged, deny_unknown_fields)]
enum FormatoPropuesta {
    Modelo { resultados: BTreeMap<String, CamposCriterio> },
    Trabajador { observaciones: Vec<PropuestaCriterio> },
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct CamposCriterio {
    a: String,
    e: Vec<String>,
    j: Juicio,
    x: String,
    m: String,
}

impl From<FormatoPropuesta> for Propuesta {
    fn from(formato: FormatoPropuesta) -> Self {
        let observaciones = match formato {
            FormatoPropuesta::Modelo { resultados } => resultados.into_iter().map(|(criterio_id, campos)| PropuestaCriterio {
                criterio_id, actuacion: campos.a, evidencias: campos.e, juicio: campos.j,
                explicacion: campos.x, mejora: campos.m,
            }).collect(),
            FormatoPropuesta::Trabajador { observaciones } => observaciones,
        };
        Self { observaciones }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Observacion {
    pub criterio_id: String,
    pub actuacion: String,
    pub evidencias: Vec<String>,
    pub expectativa: String,
    pub juicio: Juicio,
    pub explicacion: String,
    pub mejora: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub incidencia: Option<Incidencia>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Incidencia {
    pub motivo: String,
    pub acuses: Vec<AcuseSinEvento>,
    pub descuento_automatico: u32,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AcuseSinEvento {
    pub tipo: TipoEvidencia,
    pub referencia: String,
    pub clave: String,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Juicio {
    Acierto,
    Error,
    Omision,
    NoEvaluable,
    RevisionDocente,
}

pub fn validar_entrada(entrada: &Entrada) -> Result<()> {
    let entrega = &entrada.entrega;
    let rubrica = &entrada.rubrica;
    ensure!(entrega.confirmada && entrega.revision >= 0, "La entrega debe estar confirmada");
    ensure!([&entrega.id, &entrega.intento_id, &entrega.caso, &entrega.version_caso, &rubrica.id, &rubrica.version]
        .into_iter().all(|valor| !valor.trim().is_empty()), "Faltan referencias de la entrega o de su rúbrica");
    ensure!(rubrica.nivel == NIVEL && rubrica.sha256.len() == 64
        && rubrica.sha256.bytes().all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase()),
        "La rúbrica fijada requiere nivel y huella");
    let mut criterios = HashSet::new();
    ensure!(!rubrica.criterios.is_empty() && rubrica.criterios.iter().all(|criterio|
        !criterio.id.trim().is_empty() && !criterio.expectativa.trim().is_empty() && criterios.insert(&criterio.id)),
        "La rúbrica requiere criterios distintos con expectativas");
    let mut ids = HashSet::new();
    ensure!(entrada.evidencias.iter().all(|e| e.intento_id == entrega.intento_id
        && !e.id.trim().is_empty() && ids.insert(&e.id)), "La entrada contiene evidencia ajena o repetida");
    ensure!(entrada.hipotesis.iter().all(|h| h.evidencias.iter().all(|id| ids.contains(id))),
        "Una hipótesis cita evidencia que no pertenece a la entrega");
    Ok(())
}

pub fn validar_borrador(entrada: &Entrada, borrador: &Borrador) -> Result<()> {
    validar_entrada(entrada)?;
    ensure!(borrador.entrega_id == entrada.entrega.id && borrador.rubrica_sha256 == entrada.rubrica.sha256,
        "El borrador corresponde a otra entrega o rúbrica");
    let ids = entrada.evidencias.iter().map(|e| &e.id).collect::<HashSet<_>>();
    let mut criterios = HashSet::new();
    ensure!(borrador.observaciones.len() == entrada.rubrica.criterios.len(),
        "Se requiere una observación por criterio");
    for observacion in &borrador.observaciones {
        ensure!(entrada.rubrica.criterios.iter().any(|c| c.id == observacion.criterio_id && c.expectativa == observacion.expectativa)
            && criterios.insert(&observacion.criterio_id), "El borrador cita un criterio o expectativa que no pertenece a su rúbrica");
        ensure!([&observacion.actuacion, &observacion.explicacion, &observacion.mejora]
            .into_iter().all(|texto| !texto.trim().is_empty()),
            "El criterio {} requiere actuación, explicación y mejora", observacion.criterio_id);
        ensure!(observacion.evidencias.iter().all(|id| ids.contains(id)), "La observación cita evidencia ajena");
        if let Some(incidencia) = &observacion.incidencia {
            ensure!(observacion.juicio == Juicio::RevisionDocente && incidencia.descuento_automatico == 0
                && !incidencia.acuses.is_empty() && !incidencia.motivo.trim().is_empty(),
                "Las incidencias de registro requieren revisión docente y cero descuentos automáticos");
        }
        if matches!(observacion.juicio, Juicio::Acierto | Juicio::Error) {
            ensure!(!observacion.evidencias.is_empty(), "El juicio requiere un registro que lo sustente");
        }
    }
    if let Some(prioridad) = &borrador.prioridad {
        ensure!(!prioridad.motivo.trim().is_empty() && !prioridad.accion.trim().is_empty()
            && borrador.observaciones.iter().any(|o| o.criterio_id == prioridad.criterio_id
                && o.juicio == Juicio::Error && o.evidencias.contains(&prioridad.evidencia_id))
            && entrada.evidencias.iter().any(|e| e.id == prioridad.evidencia_id && e.tipo == TipoEvidencia::Informe),
            "La prioridad requiere un error documentado por el informe citado");
    }
    Ok(())
}
