use std::time::Duration;

use axum::{Json, extract::{Path, State}, http::{HeaderMap, StatusCode, header}, response::{IntoResponse, Response}};
use rusqlite::{Connection, OptionalExtension, Row, Transaction, TransactionBehavior, params};
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

use super::{dialogo, hechos, ollama::{Invocacion, VERSION_DIALOGO}};

const INTERVALO_RESERVA: Duration = Duration::from_secs(10);

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
    EnCurso(String),
    Nueva(EntrevistaPreparada),
}

struct EntrevistaPreparada {
    intento: intento::Intento,
    contexto: ContextoEntrevista,
    operacion_id: String,
    toma_id: String,
}

pub async fn crear(
    State(app): State<Estado>,
    Path(id): Path<String>,
    cabeceras: HeaderMap,
    Json(pregunta): Json<Pregunta>,
) -> Result<Response, ErrorApi> {
    let recibido_ms = trazas::ahora_ms();
    // Conserva la reserva y el acuse aunque el cliente cierre la conexión.
    tokio::spawn(procesar(app, id, cabeceras, pregunta, recibido_ms)).await.map_err(ErrorApi::interno)?
}

async fn procesar(
    app: Estado, id: String, cabeceras: HeaderMap, pregunta: Pregunta, recibido_ms: u64,
) -> Result<Response, ErrorApi> {
    let pregunta_inicial = pregunta.clone();
    let cabeceras_iniciales = cabeceras.clone();
    let inicio = app.consultar(move |db| {
        let tx = db.transaction_with_behavior(TransactionBehavior::Immediate)?;
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
        let operacion_id = operacion.id();
        let Some(toma_id) = operacion.reservar(&tx)? else {
            return Ok(Inicio::EnCurso(operacion_id));
        };
        let (agente, hablante) = destino(&contexto, pregunta_inicial.destinatario.as_deref())?;
        let entorno = contexto::entrevista(&tx, &contexto, agente, hablante)?;
        if entorno.candidatos.is_empty() {
            return Err(ErrorApi::nuevo(StatusCode::CONFLICT, "La ficha de la fase actual todavía no permite entrevistar."));
        }
        tx.commit()?;
        Ok(Inicio::Nueva(EntrevistaPreparada { intento: contexto, contexto: entorno, operacion_id, toma_id }))
    }).await?;
    let preparada = match inicio {
        Inicio::Reenvio(turno) => return Ok((StatusCode::OK, Json(turno)).into_response()),
        Inicio::EnCurso(operacion_id) => return Ok((
            StatusCode::ACCEPTED, [(header::RETRY_AFTER, "1")],
            Json(json!({"estado":"en_curso","operacion_id":operacion_id})),
        ).into_response()),
        Inicio::Nueva(preparada) => preparada,
    };
    let toma = preparada.toma_id.clone();
    let resultado = confirmar_turno(&app, preparada, cabeceras, pregunta, recibido_ms).await;
    if resultado.is_err() {
        if let Err(error) = app.consultar(move |db| Ok(Operacion::liberar_reserva(db, &toma)?)).await {
            trazas::registro(&format!("No se pudo liberar la reserva del turno · {}", error.mensaje));
        }
    }
    Ok(resultado?.into_response())
}

async fn confirmar_turno(
    app: &Estado, preparada: EntrevistaPreparada, cabeceras: HeaderMap, pregunta: Pregunta, recibido_ms: u64,
) -> Result<(StatusCode, Json<Turno>), ErrorApi> {
    let EntrevistaPreparada { intento, contexto, operacion_id, toma_id } = preparada;
    let texto = pregunta.texto.trim().to_owned();
    let candidatos = contexto.candidatos;
    let literal = ficha_aprobada::literal(pregunta.sugerencia_id.as_deref(), &pregunta.texto, &candidatos);
    let cabecera = Cabecera {
        operacion_id: operacion_id.clone(), intento_id: contexto.intento_id, agente: contexto.agente,
        caso: contexto.caso, version: contexto.version, fase: contexto.fase, hablante: contexto.hablante,
    };
    trazas::evento("SIRCLE_TURNO_RECIBIDO", &json!({
        "operacion_id": operacion_id, "intento_id": intento.intento_id,
        "agente": cabecera.agente.id(), "recibido_unix_ms": recibido_ms,
    }));
    let respuesta = if literal.is_none() {
        let solicitud = Solicitud {
            cabecera: cabecera.clone(),
            contenido: mensajes::Pregunta { texto: texto.clone(), candidatos: candidatos.iter().map(Into::into).collect(),
                memoria: contexto.memoria },
        };
        let acuse = enviar_con_reserva(app, &toma_id, solicitud).await?;
        let respuesta = match acuse {
            Acuse::Entrevista(respuesta) => respuesta,
            Acuse::Error { mensaje } => {
                trazas::registro(&format!("Ollama · {operacion_id} · {mensaje}"));
                return Err(inferencia_indisponible());
            }
            _ => return Err(inferencia_indisponible()),
        };
        let invocacion = &respuesta.resultado;
        trazas::evento("SIRCLE_INFERENCIA", &json!({
            "operacion_id": invocacion.operacion_id, "modelo": invocacion.modelo, "digest": invocacion.digest,
            "inicio_unix_ms": invocacion.inicio_unix_ms, "fin_unix_ms": invocacion.fin_unix_ms,
            "seleccion": invocacion.seleccion,
        }));
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
        operacion.confirmar_reserva(&tx, &toma_id, &turno)?;
        tx.commit()?;
        Ok((StatusCode::CREATED, Json(turno)))
    }).await?;
    let (estado, Json(turno)) = &resultado;
    trazas::evento("SIRCLE_TURNO_CONFIRMADO", &json!({
        "operacion_id": turno.operacion_id, "intento_id": turno.intento_id, "turno_id": turno.turno_id,
        "http": estado.as_u16(), "confirmado_unix_ms": trazas::ahora_ms(),
    }));
    Ok(resultado)
}

async fn enviar_con_reserva(
    app: &Estado, toma: &str, solicitud: Solicitud<mensajes::Pregunta>,
) -> Result<Acuse, ErrorApi> {
    let trabajadores = app.trabajadores.as_ref().ok_or_else(inferencia_indisponible)?;
    let operacion_id = solicitud.cabecera.operacion_id.clone();
    let envio = trabajadores.enviar(solicitud.cabecera.agente, Peticion::Entrevistar(solicitud));
    tokio::pin!(envio);
    let mut renovacion = tokio::time::interval_at(tokio::time::Instant::now() + INTERVALO_RESERVA, INTERVALO_RESERVA);
    loop {
        tokio::select! {
            acuse = &mut envio => return acuse.map_err(|error| {
                trazas::registro(&format!("Ollama · {operacion_id} · {error:#}"));
                inferencia_indisponible()
            }),
            _ = renovacion.tick() => {
                let toma = toma.to_owned();
                if !app.consultar(move |db| Ok(Operacion::renovar_reserva(db, &toma)?)).await? {
                    return Err(inferencia_indisponible());
                }
            }
        }
    }
}

pub fn confirmar_mensaje(
    db: &Transaction<'_>, contexto: &intento::Intento, candidatos: &[hechos::Hecho], texto: &str,
    esperada: &Cabecera, recibida: &Respuesta<Invocacion>,
) -> Result<Option<Turno>, ErrorApi> {
    let motivo = mensajes::discrepancia(esperada, &recibida.cabecera)
        .or_else(|| (recibida.tipo != TipoResultado::Dialogo).then_some("tipo_no_coincide"))
        .or_else(|| (recibida.resultado.operacion_id != esperada.operacion_id).then_some("invocacion_no_coincide"));
    if let Some(motivo) = motivo {
        mensajes::registrar_rechazo(db, esperada, motivo)?;
        return Ok(None);
    }
    let actual = contexto_vigente(db, contexto)?;
    let contexto = contexto::entrevista(db, &actual, esperada.agente, &esperada.hablante)?;
    let invocacion = &recibida.resultado;
    let dialogo = match dialogo::renderizar(&contexto, candidatos, texto, &invocacion.seleccion) {
        Ok(dialogo) => dialogo,
        Err(motivo) => {
            rechazo_inferencia(db, esperada, motivo, invocacion, candidatos)?;
            return Ok(None);
        }
    };
    let mut detalle = json!({
        "origen": "ollama", "version_instrucciones": VERSION_DIALOGO, "mensaje": esperada,
        "invocacion": invocacion, "hecho": dialogo.hechos.first(), "hechos": dialogo.hechos,
        "declaracion": dialogo.declaracion, "redirigio": dialogo.redirigio, "turnos_referidos": dialogo.turnos_referidos,
        "partes_aplicadas": dialogo.partes_aplicadas,
    });
    if let Some(motivo) = dialogo.motivo {
        detalle["motivo"] = json!(motivo);
    }
    guardar(db, &actual, Salida {
        agente: esperada.agente, hablante: &esperada.hablante, texto: &dialogo.texto, tipo: dialogo.tipo,
    }, texto, &esperada.operacion_id, &detalle).map(Some)
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

fn rechazo_inferencia(
    db: &Connection, cabecera: &Cabecera, motivo: &str, invocacion: &Invocacion, candidatos: &[hechos::Hecho],
) -> rusqlite::Result<()> {
    let fuentes: Vec<_> = candidatos.iter().map(|hecho| json!({
        "id": hecho.id, "fuente": hecho.fuente, "linea": hecho.linea, "pasaje": hecho.pasaje,
        "revision_fuente_id": hecho.revision_fuente_id,
    })).collect();
    mensajes::registrar_rechazo_con_evidencia(db, cabecera, motivo, json!({
        "seleccion": invocacion.seleccion, "modelo": invocacion.modelo, "digest": invocacion.digest,
        "inicio_unix_ms": invocacion.inicio_unix_ms, "fin_unix_ms": invocacion.fin_unix_ms, "fuentes": fuentes,
    }))
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

pub(crate) fn por_operacion(db: &Connection, id: &str, operacion: &str) -> Result<Option<Turno>, ErrorApi> {
    Ok(db.query_row(
        "SELECT id,intento_id,orden,agente,hablante,pregunta,respuesta,tipo_respuesta,operacion_id,revision
         FROM turnos WHERE intento_id=?1 AND operacion_id=?2",
        params![id, operacion], desde_fila,
    ).optional()?)
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
