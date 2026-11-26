use std::collections::{HashMap, HashSet};

use anyhow::{Context as _, Result, ensure};
use rusqlite::{Connection, OptionalExtension};
use sha2::{Digest, Sha256};

use crate::{
    agentes::evaluador::perfil::{self, AcuseSinEvento, Entrada, TipoEvidencia},
    modules::razonamiento::evidencias::Evidencia,
};

use super::{entregas::Instantanea, integridad, rubricas::Rubrica};

pub struct Contexto {
    pub entrada: Entrada,
    pub rubrica: Rubrica,
    pub acuses_sin_evento: Vec<AcuseSinEvento>,
}

pub fn construir(db: &Connection, entrega_id: &str) -> Result<Option<Contexto>> {
    let fila = db.query_row(
        "SELECT e.id,e.intento_id,e.revision,e.caso,e.version_caso,e.rubrica_id,
                e.version_rubrica,e.rubrica_sha256,e.instantanea,r.contenido
         FROM trabajos_evaluacion t JOIN entregas e ON e.id=t.entrega_id
         JOIN rubricas r ON r.id=e.rubrica_id AND r.version=e.version_rubrica
           AND r.sha256=e.rubrica_sha256 AND r.caso=e.caso AND r.estado='aprobada'
         WHERE e.id=?1",
        [entrega_id],
        |fila| Ok((
            perfil::Entrega { id: fila.get(0)?, intento_id: fila.get(1)?, revision: fila.get(2)?,
                caso: fila.get(3)?, version_caso: fila.get(4)?, confirmada: true },
            fila.get::<_, i64>(5)?, fila.get::<_, String>(6)?, fila.get::<_, String>(7)?,
            fila.get::<_, String>(8)?, fila.get::<_, Vec<u8>>(9)?,
        )),
    ).optional()?;
    let Some((entrega, rubrica_id, version, sha256, instantanea, bytes_rubrica)) = fila else {
        return Ok(None);
    };
    ensure!(format!("{:x}", Sha256::digest(&bytes_rubrica)) == sha256, "La huella de la rúbrica congelada no coincide");
    let rubrica: Rubrica = serde_json::from_slice(&bytes_rubrica).context("Rúbrica congelada inválida")?;
    ensure!(rubrica.version == version && rubrica.caso == entrega.caso, "La rúbrica no corresponde a la entrega");
    let mut instantanea: Instantanea = serde_json::from_str(&instantanea).context("Instantánea confirmada inválida")?;
    let acuses_sin_evento = match instantanea.acuses_sin_evento.take() {
        Some(acuses) => acuses,
        None => integridad::detectar(db, &entrega.intento_id, entrega.revision, &instantanea)?,
    };
    let rubrica_fijada = perfil::RubricaFijada {
        id: rubrica_id.to_string(), version, sha256, nivel: rubrica.nivel.clone(),
        criterios: rubrica.criterios.iter().map(|criterio| perfil::Criterio {
            id: criterio.id.clone(), expectativa: criterio.expectativa.clone(),
        }).collect(),
    };

    let turnos: HashMap<_, _> = instantanea.turnos.iter()
        .map(|turno| (turno.turno_id.as_str(), turno)).collect();
    let referencias: HashSet<_> = instantanea.evidencias.iter()
        .map(|dato| (dato.tipo.clone(), dato.origen_id.clone())).collect();
    let mut evidencias = Vec::with_capacity(instantanea.evidencias.len() + instantanea.ordenes.len());
    for evidencia in instantanea.evidencias {
        let (tipo, resumen) = match evidencia.tipo.as_str() {
            "turno" => {
                let turno = turnos.get(evidencia.origen_id.as_str())
                    .filter(|turno| turno.intento_id == entrega.intento_id && turno.tipo_respuesta == "hecho")
                    .context("La declaración no tiene un turno propio recuperable en la instantánea")?;
                (TipoEvidencia::Turno, format!("Pregunta: {}. {} respondió: {}", turno.pregunta, turno.hablante, turno.respuesta))
            }
            "medicion" => (
                TipoEvidencia::Medicion, format!("{}: {} {}", evidencia.instrumento.as_deref().unwrap_or_default(),
                    evidencia.valor.as_ref().context("La medición carece de valor")?, evidencia.unidad.as_deref().unwrap_or_default()),
            ),
            "hallazgo" => (TipoEvidencia::Hallazgo, texto(&evidencia)?),
            "informe" => (TipoEvidencia::Informe, texto(&evidencia)?),
            "apunte" => (TipoEvidencia::Apunte, texto(&evidencia)?),
            otro => anyhow::bail!("Tipo de evidencia fuera de la entrega: {otro}"),
        };
        evidencias.push(perfil::Evidencia { id: evidencia.origen_id, intento_id: evidencia.intento_id, tipo, resumen });
    }
    evidencias.extend(instantanea.ordenes.into_iter().map(|orden| perfil::Evidencia {
        id: orden.orden_id, intento_id: orden.intento_id, tipo: TipoEvidencia::Orden,
        resumen: format!("Se confirmó la orden {}", orden.examen),
    }));
    let mut hipotesis = Vec::new();
    if let Some(borrador) = instantanea.borrador {
        ensure!(borrador.revision <= entrega.revision, "La revisión de hipótesis es posterior a la entrega");
        for hipotesis_guardada in borrador.hipotesis {
            let citas = hipotesis_guardada.evidencias.into_iter().map(|cita| {
                let referencia = (cita.tipo, cita.origen_id);
                ensure!(referencias.contains(&referencia), "La referencia de la hipótesis no corresponde a la instantánea");
                Ok(referencia.1)
            }).collect::<Result<_>>()?;
            hipotesis.push(perfil::Hipotesis {
                rol: hipotesis_guardada.rol, titulo: hipotesis_guardada.titulo, explicacion: hipotesis_guardada.explicacion,
                evidencias: citas,
            });
        }
    }
    let entrada = Entrada { entrega, rubrica: rubrica_fijada, evidencias, hipotesis };
    perfil::validar_entrada(&entrada)?;
    Ok(Some(Contexto { entrada, rubrica, acuses_sin_evento }))
}

fn texto(evidencia: &Evidencia) -> Result<String> {
    evidencia.texto.clone().context("El registro carece de texto")
}
