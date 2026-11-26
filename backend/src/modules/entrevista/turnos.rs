use axum::{Json, extract::{Path, State}, http::{HeaderMap, StatusCode}};
use rusqlite::{Connection, Row, Transaction, TransactionBehavior, params};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::{
    agentes::{
        dan_sparks::{self as dan, perfil as p03},
        lr::{enrutamiento, entrevista, perfil as lr},
        mensajes::{self, Agente, Cabecera, Respuesta, Solicitud, TipoResultado},
        nannu::{self, perfil as p02},
        supervisor::{Acuse, Peticion},
    },
    app::{errores::ErrorApi, estado::Estado},
    modules::intentos::intento,
    shared::{autorizacion, contexto::{self, ContextoEntrevista}, ficha_aprobada::{self, Sugerencia}, operaciones::Operacion, trazas},
};

use super::hechos;
use super::ollama::Invocacion;

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Pregunta {
    texto: String,
    sugerencia_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    destinatario: Option<String>,
}

impl Pregunta {
    fn operacion<'a>(&self, identidad: &'a str, ambito: &'a str, cabeceras: &'a HeaderMap) -> Result<Operacion<'a>, ErrorApi> {
        if self.sugerencia_id.is_some() || self.destinatario.is_some() {
            Operacion::nueva(identidad, ambito, cabeceras, "POST turnos", self)
        } else {
            Operacion::nueva(identidad, ambito, cabeceras, "POST turnos", &self.texto.trim())
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct Turno {
    pub turno_id: String,
    pub intento_id: String,
    pub orden: i64,
    pub agente: String,
    pub hablante: String,
    pub pregunta: String,
    pub respuesta: String,
    pub tipo_respuesta: String,
    pub operacion_id: String,
    pub revision: i64,
}

enum Inicio {
    Reenvio(Turno),
    Nueva { intento: intento::Intento, contexto: ContextoEntrevista, operacion_id: String },
}

pub async fn crear(
    State(app): State<Estado>,
    Path(id): Path<String>,
    cabeceras: HeaderMap,
    Json(pregunta): Json<Pregunta>,
) -> Result<(StatusCode, Json<Turno>), ErrorApi> {
    let recibido_ms = trazas::ahora_ms();
    let texto = pregunta.texto.trim().to_owned();
    let pregunta_inicial = pregunta.clone();
    let cabeceras_iniciales = cabeceras.clone();
    let inicio = app.consultar(move |db| {
        let tx = db.transaction()?;
        let (identidad, contexto) = autorizacion::intento_propio(&tx, &cabeceras_iniciales, &id)?;
        autorizacion::pre_resuelto(&contexto)?;
        let texto_inicial = pregunta_inicial.texto.trim();
        if texto_inicial.is_empty() || texto_inicial.chars().count() > 1000 {
            return Err(ErrorApi::nuevo(StatusCode::UNPROCESSABLE_ENTITY, "Escribe una pregunta de hasta 1000 caracteres."));
        }
        let operacion = pregunta_inicial.operacion(&identidad.id, &id, &cabeceras_iniciales)?;
        if let Some(turno) = operacion.recuperar(&tx)? {
            return Ok(Inicio::Reenvio(turno));
        }
        let (agente, hablante) = destino(&contexto, pregunta_inicial.destinatario.as_deref())?;
        let entorno = contexto::entrevista(&tx, &contexto, agente, hablante)?;
        if entorno.candidatos.is_empty() {
            return Err(ErrorApi::nuevo(StatusCode::CONFLICT, "La ficha de la fase actual todavía no permite entrevistar."));
        }
        let operacion_id = operacion.id();
        Ok(Inicio::Nueva { intento: contexto, contexto: entorno, operacion_id })
    }).await?;
    let (intento, contexto, operacion_id) = match inicio {
        Inicio::Reenvio(turno) => return Ok((StatusCode::OK, Json(turno))),
        Inicio::Nueva { intento, contexto, operacion_id } => (intento, contexto, operacion_id),
    };
    let candidatos = contexto.candidatos;
    let literal = ficha_aprobada::literal(pregunta.sugerencia_id.as_deref(), &pregunta.texto, &candidatos);
    let cabecera = Cabecera {
        operacion_id: operacion_id.clone(), intento_id: contexto.intento_id, agente: contexto.agente,
        caso: contexto.caso, version: contexto.version, fase: contexto.fase, hablante: contexto.hablante,
    };
    eprintln!("SIRCLE_TURNO_RECIBIDO {}", json!({
        "operacion_id": operacion_id, "intento_id": intento.intento_id,
        "agente": cabecera.agente.id(), "recibido_unix_ms": recibido_ms,
    }));
    let respuesta = if literal.is_none() {
        let trabajadores = app.trabajadores.as_ref().ok_or_else(inferencia_indisponible)?;
        let solicitud = Solicitud {
            cabecera: cabecera.clone(),
            contenido: mensajes::Pregunta { texto: texto.clone(), candidatos: candidatos.iter().map(Into::into).collect(),
                memoria: contexto.memoria },
        };
        let acuse = trabajadores.enviar(cabecera.agente, Peticion::Entrevistar(solicitud)).await
            .map_err(|error| {
                eprintln!("Ollama · {operacion_id} · {error:#}");
                inferencia_indisponible()
            })?;
        let respuesta = match acuse {
            Acuse::Entrevista(respuesta) => respuesta,
            Acuse::Error { mensaje } => {
                eprintln!("Ollama · {operacion_id} · {mensaje}");
                return Err(inferencia_indisponible());
            }
            _ => return Err(inferencia_indisponible()),
        };
        let registro = serde_json::to_string(&respuesta.resultado).map_err(ErrorApi::interno)?;
        eprintln!("SIRCLE_INFERENCIA {registro}");
        Some(respuesta)
    } else {
        None
    };

    let resultado = app.consultar(move |db| {
        let tx = db.transaction_with_behavior(TransactionBehavior::Immediate)?;
        let (identidad, actual) = autorizacion::intento_propio(&tx, &cabeceras, &intento.intento_id)?;
        autorizacion::pre_resuelto(&actual)?;
        let operacion = pregunta.operacion(&identidad.id, &intento.intento_id, &cabeceras)?;
        if let Some(turno) = operacion.recuperar(&tx)? {
            return Ok((StatusCode::OK, Json(turno)));
        }
        let turno = match literal {
            Some(sugerencia) if cabecera.agente == Agente::Lr =>
                Some(entrevista::confirmar_literal(&tx, &intento, &texto, &operacion_id, sugerencia)?),
            Some(sugerencia) => confirmar_literal_actor(&tx, &intento, &cabecera, &texto, sugerencia).map(Some)?,
            None => {
                let respuesta = respuesta.ok_or_else(inferencia_indisponible)?;
                confirmar_mensaje(&tx, &intento, &candidatos, &texto, &cabecera, &respuesta)?
            }
        };
        let Some(turno) = turno else {
            tx.commit()?;
            return Err(inferencia_indisponible());
        };
        operacion.confirmar(&tx, &turno)?;
        tx.commit()?;
        Ok((StatusCode::CREATED, Json(turno)))
    }).await?;
    let (estado, Json(turno)) = &resultado;
    eprintln!("SIRCLE_TURNO_CONFIRMADO {}", json!({
        "operacion_id": turno.operacion_id, "intento_id": turno.intento_id, "turno_id": turno.turno_id,
        "http": estado.as_u16(), "confirmado_unix_ms": trazas::ahora_ms(),
    }));
    Ok(resultado)
}

pub fn confirmar_mensaje(
    db: &Transaction<'_>, contexto: &intento::Intento, candidatos: &[hechos::Hecho], texto: &str,
    esperada: &Cabecera, recibida: &Respuesta<Invocacion>,
) -> Result<Option<Turno>, ErrorApi> {
    let motivo = mensajes::discrepancia(esperada, &recibida.cabecera)
        .or_else(|| (recibida.tipo != TipoResultado::Seleccion).then_some("tipo_no_coincide"))
        .or_else(|| (recibida.resultado.operacion_id != esperada.operacion_id).then_some("invocacion_no_coincide"));
    if let Some(motivo) = motivo {
        mensajes::registrar_rechazo(db, esperada, motivo)?;
        return Ok(None);
    }
    match esperada.agente {
        Agente::Lr => entrevista::confirmar_correlacionada(db, contexto, candidatos, texto, &recibida.resultado, esperada).map(Some),
        Agente::Nannu | Agente::DanSparks => confirmar_actor(db, contexto, esperada, candidatos, texto, &recibida.resultado),
        Agente::Evaluador => Err(inferencia_indisponible()),
    }
}

fn destino(intento: &intento::Intento, solicitado: Option<&str>) -> Result<(Agente, &'static str), ErrorApi> {
    match intento.caso.as_str() {
        p02::ESCENARIO => nannu::enrutamiento::destino(intento, solicitado.unwrap_or(p02::PACIENTE)),
        p03::ESCENARIO if solicitado.is_none_or(|voz| voz == p03::HABLANTE) => dan::enrutamiento::destino(intento),
        lr::ESCENARIO if solicitado.is_none_or(|voz| voz == lr::HABLANTE) => enrutamiento::destino(intento),
        _ => Err(ErrorApi::nuevo(StatusCode::UNPROCESSABLE_ENTITY, "El destinatario no corresponde a la entrevista de este caso.")),
    }
}

fn hecho_de_actor<'a>(contexto: &'a ContextoEntrevista, seleccion: &str) -> Result<&'a hechos::Hecho, ErrorApi> {
    match contexto.agente {
        Agente::Nannu => nannu::entrevista::elegir(contexto, seleccion),
        Agente::DanSparks => dan::entrevista::elegir(contexto, seleccion),
        _ => Err(inferencia_indisponible()),
    }
}

fn confirmar_actor(
    db: &Transaction<'_>, previo: &intento::Intento, cabecera: &Cabecera, candidatos: &[hechos::Hecho],
    pregunta: &str, invocacion: &Invocacion,
) -> Result<Option<Turno>, ErrorApi> {
    let actual = contexto_vigente(db, previo)?;
    let contexto = contexto::entrevista(db, &actual, cabecera.agente, &cabecera.hablante)?;
    let Ok(hecho) = hecho_de_actor(&contexto, &invocacion.seleccion.hecho_id) else {
        let motivo = if cabecera.agente == Agente::Nannu { "seleccion_fuera_de_hablante" } else { "seleccion_fuera_contexto_p03" };
        mensajes::registrar_rechazo(db, cabecera, motivo)?;
        return Ok(None);
    };
    let pertinente = match cabecera.agente {
        Agente::Nannu => nannu::entrevista::pertinente(hecho, pregunta),
        Agente::DanSparks => dan::entrevista::pertinente(hecho, pregunta),
        _ => false,
    };
    if !candidatos.contains(hecho) || !pertinente {
        mensajes::registrar_rechazo(db, cabecera, "seleccion_no_pertinente")?;
        return Ok(None);
    }
    let detalle = json!({"hecho": hecho, "origen": "ollama", "invocacion": invocacion, "mensaje": cabecera});
    guardar(db, &actual, Salida::hecho(cabecera.agente, hecho), pregunta, &cabecera.operacion_id, &detalle).map(Some)
}

fn confirmar_literal_actor(
    db: &Transaction<'_>, previo: &intento::Intento, cabecera: &Cabecera, pregunta: &str, sugerencia: Sugerencia,
) -> Result<Turno, ErrorApi> {
    let actual = contexto_vigente(db, previo)?;
    let contexto = contexto::entrevista(db, &actual, cabecera.agente, &cabecera.hablante)?;
    let hecho = hecho_de_actor(&contexto, sugerencia.hecho_id)?;
    let detalle = json!({"hecho": hecho, "origen": "ficha", "sugerencia_id": sugerencia.id, "mensaje": cabecera});
    guardar(db, &actual, Salida::hecho(cabecera.agente, hecho), pregunta, &cabecera.operacion_id, &detalle)
}

pub(crate) fn contexto_vigente(db: &Connection, anterior: &intento::Intento) -> Result<intento::Intento, ErrorApi> {
    let actual = intento::recuperar(db, &anterior.intento_id, &anterior.estudiante_id)?;
    if actual.revision != anterior.revision || actual.estado != "atencion" || actual.pre == "pendiente"
        || actual.fase != anterior.fase || actual.version_caso != anterior.version_caso
    {
        return Err(ErrorApi::nuevo(StatusCode::CONFLICT, "El intento cambió durante la entrevista. Recarga su estado."));
    }
    Ok(actual)
}

pub(crate) struct Salida<'a> {
    pub agente: Agente,
    pub hablante: &'a str,
    pub texto: &'a str,
    pub tipo: &'a str,
}

impl<'a> Salida<'a> {
    pub fn hecho(agente: Agente, hecho: &'a hechos::Hecho) -> Self {
        Self { agente, hablante: &hecho.hablante, texto: &hecho.respuesta, tipo: "hecho" }
    }
}

pub(crate) fn guardar(
    db: &Transaction<'_>, actual: &intento::Intento, salida: Salida<'_>, texto: &str,
    operacion: &str, detalle: &Value,
) -> Result<Turno, ErrorApi> {
    let turno = db.query_row(
        "INSERT INTO turnos (id, intento_id, orden, agente, hablante, pregunta, respuesta, tipo_respuesta, operacion_id, revision, fase)
         VALUES (lower(hex(randomblob(16))), ?1, (SELECT coalesce(max(orden),0)+1 FROM turnos WHERE intento_id=?1),
                 ?2,?3,?4,?5,?6,?7,?8,?9)
         RETURNING id,intento_id,orden,agente,hablante,pregunta,respuesta,tipo_respuesta,operacion_id,revision",
        params![actual.intento_id, salida.agente.id(), salida.hablante, texto, salida.texto, salida.tipo, operacion,
            actual.revision + 1, actual.fase], desde_fila,
    )?;
    trazas::guardar_turno(db, &turno.turno_id, &turno.operacion_id, detalle)?;
    db.execute("UPDATE intentos SET revision=?1 WHERE id=?2", params![turno.revision, actual.intento_id])?;
    Ok(turno)
}

pub(crate) fn historico(db: &Connection, id: &str) -> Result<Vec<Turno>, ErrorApi> {
    let mut consulta = db.prepare(
        "SELECT id, intento_id, orden, agente, hablante, pregunta, respuesta,
                tipo_respuesta, operacion_id, revision
         FROM turnos WHERE intento_id=?1 ORDER BY orden",
    )?;
    let filas = consulta.query_map([id], desde_fila)?;
    Ok(filas.collect::<rusqlite::Result<_>>()?)
}

pub(crate) fn desde_fila(fila: &Row<'_>) -> rusqlite::Result<Turno> {
    Ok(Turno {
        turno_id: fila.get(0)?, intento_id: fila.get(1)?, orden: fila.get(2)?, agente: fila.get(3)?,
        hablante: fila.get(4)?, pregunta: fila.get(5)?, respuesta: fila.get(6)?, tipo_respuesta: fila.get(7)?,
        operacion_id: fila.get(8)?, revision: fila.get(9)?,
    })
}

fn inferencia_indisponible() -> ErrorApi {
    ErrorApi::nuevo(StatusCode::SERVICE_UNAVAILABLE, "Entrevista temporalmente indisponible. Reintenta cuando se recupere el modelo local.")
}
