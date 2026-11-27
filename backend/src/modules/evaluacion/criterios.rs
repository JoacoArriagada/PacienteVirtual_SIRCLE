use std::collections::HashMap;

use anyhow::{Context as _, Result, ensure};
use rusqlite::{Connection, params};

use crate::agentes::evaluador::perfil::{self, Borrador, Incidencia, Juicio, Observacion, Prioridad, Propuesta, TipoEvidencia};

use super::{evidencia::Contexto, rubricas::{Ambito, Criterio}};

pub fn aplicables(db: &Connection, contexto: &Contexto) -> Result<Vec<String>> {
    let entrega = &contexto.entrada.entrega;
    let (entrevista, valoracion, decisiones): (bool, bool, bool) = db.query_row(
        "SELECT EXISTS(SELECT 1 FROM hechos WHERE version_clinica_id=v.id AND fase='inicial'),
                EXISTS(SELECT 1 FROM lecturas_versionadas WHERE version_caso_id=v.id AND fase='inicial')
                    OR EXISTS(SELECT 1 FROM zonas_exploracion WHERE version_caso_id=v.id AND fase='inicial'),
                EXISTS(SELECT 1 FROM examenes_versionados WHERE version_caso_id=v.id AND fase='inicial')
         FROM versiones_clinicas v WHERE v.escenario_id=?1 AND v.version=?2",
        params![entrega.caso, entrega.version_caso], |fila| Ok((fila.get(0)?, fila.get(1)?, fila.get(2)?)),
    )?;
    Ok(contexto.rubrica.criterios.iter().filter(|criterio| match criterio.ambito {
        Ambito::Entrevista => entrevista,
        Ambito::Valoracion => valoracion,
        Ambito::Decisiones => decisiones,
        Ambito::UsoDeEvidencias | Ambito::Hipotesis => entrevista || valoracion || decisiones,
    }).filter(|criterio| !contexto.acuses_sin_evento.iter().any(|acuse| afecta(criterio, acuse.tipo)))
        .map(|criterio| criterio.id.clone()).collect())
}

fn afecta(criterio: &Criterio, tipo: TipoEvidencia) -> bool {
    criterio.evidencias.contains(&tipo) || match criterio.ambito {
        Ambito::Entrevista => tipo == TipoEvidencia::Turno,
        Ambito::Valoracion => matches!(tipo, TipoEvidencia::Medicion | TipoEvidencia::Hallazgo),
        Ambito::Decisiones => matches!(tipo, TipoEvidencia::Orden | TipoEvidencia::Informe),
        Ambito::UsoDeEvidencias | Ambito::Hipotesis => false,
    }
}

pub fn aplicar(contexto: &Contexto, aplicables: &[String], propuesta: Propuesta) -> Result<Borrador> {
    let entrada = &contexto.entrada;
    ensure!(propuesta.observaciones.len() == aplicables.len(), "Faltan observaciones de criterios aplicables");
    let fuentes: HashMap<_, _> = entrada.evidencias.iter().map(|dato| (dato.id.as_str(), dato)).collect();
    let mut propuestas = HashMap::with_capacity(aplicables.len());
    for item in propuesta.observaciones {
        ensure!(propuestas.insert(item.criterio_id.clone(), item).is_none(), "El resultado repite un criterio");
    }
    let mut observaciones = Vec::with_capacity(contexto.rubrica.criterios.len());
    let mut prioridad = None;
    for criterio in &contexto.rubrica.criterios {
        let acuses: Vec<_> = contexto.acuses_sin_evento.iter().filter(|acuse| afecta(criterio, acuse.tipo)).cloned().collect();
        let item = if !acuses.is_empty() {
            Observacion {
                criterio_id: criterio.id.clone(), expectativa: criterio.expectativa.clone(),
                actuacion: "El servicio confirmó una acción cuyo evento no está en la entrega.".into(),
                evidencias: Vec::new(), juicio: Juicio::RevisionDocente,
                explicacion: "Falta el evento asociado al acuse. La actuación queda pendiente de revisión docente.".into(),
                mejora: "Coteja el acuse con el registro del intento y resuelve la incidencia antes de calificar.".into(),
                incidencia: Some(Incidencia { motivo: "acuse_sin_evento".into(), acuses, descuento_automatico: 0 }),
            }
        } else if aplicables.contains(&criterio.id) {
            let propuesta = propuestas.remove(&criterio.id).context("El resultado no cubre todos los criterios aplicables")?;
            ensure!(propuesta.juicio != Juicio::NoEvaluable, "El criterio tiene una oportunidad formativa habilitada");
            if propuesta.juicio == Juicio::Omision && criterio.ambito == Ambito::Decisiones {
                let requiere_ecg = criterio.evidencias.contains(&TipoEvidencia::Orden)
                    && criterio.evidencias.contains(&TipoEvidencia::Informe);
                ensure!(!requiere_ecg || [TipoEvidencia::Orden, TipoEvidencia::Informe].into_iter()
                    .any(|tipo| !entrada.evidencias.iter().any(|e| e.tipo == tipo)),
                    "La entrega conserva la orden y el informe requeridos");
            }
            for id in &propuesta.evidencias {
                ensure!(fuentes.contains_key(id.as_str()), "El resultado cita una evidencia fuera de la entrega");
            }
            if matches!(propuesta.juicio, Juicio::Acierto | Juicio::Error) {
                if let Some(posicion) = criterio.hipotesis_posicion {
                    let hipotesis = entrada.hipotesis.get(posicion - 1).context("Falta la hipótesis evaluada")?;
                    ensure!(!hipotesis.explicacion.trim().is_empty()
                        && propuesta.evidencias.iter().any(|id| hipotesis.evidencias.contains(id)),
                        "La hipótesis {} carece de fundamento citable", posicion);
                }
                ensure!(propuesta.evidencias.iter().any(|id| criterio.evidencias.contains(&fuentes[id.as_str()].tipo)),
                    "La evidencia no sustenta el juicio del criterio {}", criterio.id);
            }
            let mut mejora = propuesta.mejora;
            let mut explicacion = propuesta.explicacion;
            if let Some(posicion) = criterio.hipotesis_posicion {
                let registros = propuesta.evidencias.iter().map(|id| fuentes[id.as_str()]);
                match propuesta.juicio {
                    Juicio::Omision => {
                        let accion = if propuesta.evidencias.is_empty() {
                            "Anota qué hallazgo apoyaría o debilitaría esta hipótesis."
                        } else {
                            "Revisa el registro citado y anota si documenta el dato que falta. Indica qué hallazgo apoyaría o debilitaría esta hipótesis."
                        };
                        mejora = format!("{} {accion}", mejora.trim());
                    }
                    Juicio::Error => {
                        let hipotesis = &entrada.hipotesis[posicion - 1];
                        if let Some(informe) = registros.clone().find(|r| r.tipo == TipoEvidencia::Informe) {
                            explicacion = format!("{} Afirmación de la hipótesis {posicion}: «{}». Informe citado: «{}».",
                                explicacion.trim(), hipotesis.titulo, informe.resumen);
                            if prioridad.is_none() {
                                prioridad = Some(Prioridad {
                                    criterio_id: criterio.id.clone(), evidencia_id: informe.id.clone(),
                                    motivo: format!("El informe registra «{}», un resultado que contradice la hipótesis {posicion}. Corrige esa contradicción antes de completar los demás fundamentos.", informe.resumen),
                                    accion: format!("Relee el informe citado y corrige la hipótesis {posicion} según el resultado registrado."),
                                });
                            }
                        } else if registros.clone().any(|r| r.tipo == TipoEvidencia::Turno) {
                            mejora = format!("{} Distingue en la ficha lo referido en el turno de lo medido con un instrumento.", mejora.trim());
                        }
                    }
                    _ => {}
                }
            }
            Observacion {
                criterio_id: criterio.id.clone(), expectativa: criterio.expectativa.clone(),
                actuacion: propuesta.actuacion, evidencias: propuesta.evidencias, juicio: propuesta.juicio,
                explicacion, mejora,
                incidencia: None,
            }
        } else {
            Observacion {
                criterio_id: criterio.id.clone(), expectativa: criterio.expectativa.clone(),
                actuacion: "La actividad no ofreció esta actuación.".into(), evidencias: Vec::new(),
                juicio: Juicio::NoEvaluable, explicacion: "La versión clínica de la entrega carece de esa oportunidad.".into(),
                mejora: "Practica este criterio en una actividad que habilite la actuación.".into(),
                incidencia: None,
            }
        };
        observaciones.push(item);
    }
    let borrador = Borrador { entrega_id: entrada.entrega.id.clone(), rubrica_sha256: entrada.rubrica.sha256.clone(),
        observaciones, prioridad };
    perfil::validar_borrador(entrada, &borrador)?;
    Ok(borrador)
}
