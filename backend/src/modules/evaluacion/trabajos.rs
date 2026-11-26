use std::time::Duration;

use anyhow::{Result, anyhow, ensure};
use axum::{Json, extract::{Path, State}, http::{HeaderMap, StatusCode}};
use rusqlite::{Connection, OptionalExtension, TransactionBehavior, params};
use serde::Serialize;
use serde_json::{Value, json};

use crate::{
    agentes::{
        evaluador::{inferencia::Evaluar, perfil::{self, Borrador, Evidencia, Propuesta}},
        mensajes::{self, Cabecera, Solicitud, TipoResultado},
        supervisor::{Acuse, Peticion, Supervisor},
    },
    app::{errores::ErrorApi, estado::Estado},
    modules::acceso::sesiones::Identidad,
    modules::entrevista::ollama::{DIGEST_MODELO, MODELO, TIEMPO_EVALUADOR},
    shared::{autorizacion, trazas},
};

use super::{criterios, evidencia::{self, Contexto}, publicacion, revisiones};

struct Toma {
    entrega_id: String,
    operacion_id: String,
}

pub async fn cola(app: Estado) {
    loop {
        if let Err(error) = procesar_siguiente(&app).await {
            trazas::registro(&format!("Error en la cola de evaluación: {}", error.mensaje));
        }
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}

pub async fn procesar_siguiente(app: &Estado) -> Result<bool, ErrorApi> {
    let Some(supervisor) = &app.trabajadores else { return Ok(false) };
    if app.db.is_none() {
        return Ok(false);
    }
    let Some(toma) = app.consultar(tomar).await? else { return Ok(false) };
    trazas::evento("SIRCLE_EVALUACION_TOMA", &json!({
        "entrega_id":toma.entrega_id, "toma_id":toma.operacion_id, "inicio_unix_ms":trazas::ahora_ms(),
    }));
    let entrega = toma.entrega_id.clone();
    let preparado = app.consultar(move |db| {
        let tx = db.transaction()?;
        let contexto = evidencia::construir(&tx, &entrega).map_err(ErrorApi::interno)?
            .ok_or_else(|| ErrorApi::interno("No se recuperó la entrega confirmada"))?;
        let aplicables = criterios::aplicables(&tx, &contexto).map_err(ErrorApi::interno)?;
        Ok((contexto, aplicables))
    }).await;
    let (contexto, aplicables) = match preparado {
        Ok(preparado) => preparado,
        Err(error) => {
            reprogramar(app, toma, "contexto").await?;
            return Err(error);
        }
    };
    match inferir(supervisor, &toma, contexto, aplicables).await {
        Ok((borrador, detalle)) => app.consultar(move |db| guardar(db, toma, borrador, detalle)).await,
        Err(error) => {
            trazas::registro(&format!("SIRCLE_EVALUACION_FALLO {} · {error:#}", toma.operacion_id));
            reprogramar(app, toma, "inferencia_o_validacion").await?;
            Ok(false)
        }
    }
}

fn tomar(db: &mut Connection) -> Result<Option<Toma>, ErrorApi> {
    let tx = db.transaction_with_behavior(TransactionBehavior::Immediate)?;
    let toma = tx.query_row(
        "UPDATE trabajos_evaluacion SET estado='en_curso',toma_id=lower(hex(randomblob(16))),vence=unixepoch()+?1
         WHERE entrega_id=(
           SELECT entrega_id FROM trabajos_evaluacion
           WHERE (estado='pendiente' AND reintentar_desde<=unixepoch()) OR (estado='en_curso' AND vence<=unixepoch())
           ORDER BY creado_en,entrega_id LIMIT 1
         ) RETURNING entrega_id,toma_id",
        [TIEMPO_EVALUADOR.as_secs() + 30], |fila| Ok(Toma { entrega_id: fila.get(0)?, operacion_id: fila.get(1)? }),
    ).optional()?;
    tx.commit()?;
    Ok(toma)
}

async fn reprogramar(app: &Estado, toma: Toma, motivo: &'static str) -> Result<(), ErrorApi> {
    app.consultar(move |db| {
        db.execute(
            "UPDATE trabajos_evaluacion SET estado='pendiente',toma_id=NULL,vence=NULL,reintentar_desde=unixepoch()+15,ultimo_error=?1
             WHERE entrega_id=?2 AND toma_id=?3 AND estado='en_curso'",
            params![motivo, toma.entrega_id, toma.operacion_id],
        )?;
        Ok(())
    }).await
}

async fn inferir(supervisor: &Supervisor, toma: &Toma, contexto: Contexto, aplicables: Vec<String>) -> Result<(Borrador, Value)> {
    let entrada = &contexto.entrada;
    let cabecera = Cabecera {
        operacion_id: toma.operacion_id.clone(), intento_id: entrada.entrega.intento_id.clone(), agente: perfil::AGENTE,
        caso: entrada.entrega.caso.clone(), version: entrada.entrega.version_caso.clone(),
        fase: "entregado".into(), hablante: "Evaluador".into(),
    };
    let (propuesta, invocacion) = if aplicables.is_empty() {
        (Propuesta { observaciones: Vec::new() }, Value::Null)
    } else {
        let peticion = Peticion::Evaluar(Solicitud {
            cabecera: cabecera.clone(), contenido: Evaluar {
                entrada: entrada.clone(), aplicables: aplicables.clone(),
                fuentes_permitidas: contexto.rubrica.criterios.iter()
                    .map(|c| (c.id.clone(), c.evidencias.clone())).collect(),
                posiciones: contexto.rubrica.criterios.iter()
                    .filter_map(|c| c.hipotesis_posicion.map(|n| (c.id.clone(), n))).collect(),
            },
        });
        let enviado = supervisor.enviar(perfil::AGENTE, peticion).await?;
        let respuesta = match enviado {
            Acuse::Evaluacion(respuesta) => respuesta,
            Acuse::Error { mensaje } => return Err(anyhow!(mensaje)),
            _ => return Err(anyhow!("Acuse de evaluación inesperado")),
        };
        let r = &respuesta.resultado;
        ensure!(mensajes::discrepancia(&cabecera, &respuesta.cabecera).is_none()
            && respuesta.tipo == TipoResultado::Evaluacion && r.operacion_id == toma.operacion_id
            && r.modelo == MODELO && r.digest == DIGEST_MODELO, "La respuesta no corresponde a la operación de evaluación");
        trazas::evento("SIRCLE_INFERENCIA_EVALUACION", &json!({
            "entrega_id":toma.entrega_id, "operacion_id":r.operacion_id,
            "inicio_unix_ms":r.inicio_unix_ms, "fin_unix_ms":r.fin_unix_ms,
        }));
        let invocacion = serde_json::to_value(r)?;
        (respuesta.resultado.seleccion, invocacion)
    };
    let borrador = criterios::aplicar(&contexto, &aplicables, propuesta)?;
    let detalle = json!({
        "entrega":entrada.entrega,"rubrica":entrada.rubrica,"mensaje":cabecera,
        "aplicables":aplicables,"entrada":entrada,"invocacion":invocacion,
        "acuses_sin_evento":contexto.acuses_sin_evento,
    });
    Ok((borrador, detalle))
}

fn guardar(db: &mut Connection, toma: Toma, borrador: Borrador, mut detalle: Value) -> Result<bool, ErrorApi> {
    let tx = db.transaction_with_behavior(TransactionBehavior::Immediate)?;
    let filas = tx.execute(
        "UPDATE trabajos_evaluacion SET estado='terminado',toma_id=NULL,vence=NULL,reintentar_desde=0,ultimo_error=NULL
         WHERE entrega_id=?1 AND toma_id=?2 AND estado='en_curso' AND vence>unixepoch()",
        params![toma.entrega_id, toma.operacion_id],
    )?;
    if filas == 0 {
        trazas::evento("SIRCLE_EVALUACION_DESCARTADA", &json!({
            "entrega_id":toma.entrega_id,"toma_id":toma.operacion_id,"motivo":"toma_vencida_o_reemplazada",
        }));
        return Ok(false);
    }
    let datos = serde_json::to_value(&borrador).map_err(ErrorApi::interno)?;
    let (id, momento): (String, String) = tx.query_row(
        "INSERT INTO evaluaciones (id,entrega_id,operacion_id,borrador,momento)
         VALUES (lower(hex(randomblob(16))),?1,?2,?3,strftime('%Y-%m-%dT%H:%M:%fZ','now')) RETURNING id,momento",
        params![toma.entrega_id, toma.operacion_id, datos.to_string()], |fila| Ok((fila.get(0)?, fila.get(1)?)),
    )?;
    revisiones::registrar_base(&tx, &id, borrador, momento).map_err(ErrorApi::interno)?;
    detalle["evaluacion_id"] = json!(id);
    detalle["borrador_guardado"] = datos;
    trazas::guardar_evaluacion(&tx, &id, &toma.operacion_id, &detalle)?;
    tx.commit()?;
    trazas::evento("SIRCLE_EVALUACION_CONFIRMADA", &json!({"entrega_id":toma.entrega_id,"toma_id":toma.operacion_id,"evaluacion_id":id}));
    Ok(true)
}

#[derive(Serialize)]
pub struct Evaluacion {
    evaluacion_id: String,
    intento_id: String,
    entrega_id: String,
    operacion_id: String,
    rubrica_id: i64,
    version_rubrica: String,
    rubrica_sha256: String,
    borrador: Borrador,
    revision_actual: revisiones::Revision,
    evidencias: Vec<Evidencia>,
    publicacion: Option<publicacion::Publicacion>,
}

pub async fn leer(
    State(app): State<Estado>, Path(id): Path<String>, cabeceras: HeaderMap,
) -> Result<Json<Evaluacion>, ErrorApi> {
    app.consultar(move |db| {
        let tx = db.transaction()?;
        autorizar(&tx, &id, &cabeceras)?;
        let (intento_id, entrega_id, operacion_id, rubrica_id, version_rubrica, rubrica_sha256, borrador):
            (String, String, String, i64, String, String, String) = tx.query_row(
            "SELECT e.intento_id,v.entrega_id,v.operacion_id,e.rubrica_id,e.version_rubrica,e.rubrica_sha256,v.borrador
             FROM evaluaciones v JOIN entregas e ON e.id=v.entrega_id WHERE v.id=?1",
            [&id], |fila| Ok((fila.get(0)?,fila.get(1)?,fila.get(2)?,fila.get(3)?,fila.get(4)?,fila.get(5)?,fila.get(6)?)),
        )?;
        let revision_actual = revisiones::ultima(&tx, &id)?;
        let publicacion = publicacion::ultima(&tx, &id)?;
        let contexto = evidencia::construir(&tx, &entrega_id).map_err(ErrorApi::interno)?
            .ok_or_else(|| ErrorApi::interno("Falta la entrega de la evaluación"))?;
        Ok(Json(Evaluacion { evaluacion_id: id, intento_id, entrega_id, operacion_id, rubrica_id, version_rubrica,
            rubrica_sha256, borrador: serde_json::from_str(&borrador).map_err(ErrorApi::interno)?,
            revision_actual, evidencias: contexto.entrada.evidencias, publicacion }))
    }).await
}

pub async fn leer_traza(
    State(app): State<Estado>, Path(id): Path<String>, cabeceras: HeaderMap,
) -> Result<Json<Value>, ErrorApi> {
    app.consultar(move |db| {
        let tx = db.transaction()?;
        autorizar(&tx, &id, &cabeceras)?;
        let detalle: String = tx.query_row("SELECT detalle FROM trazas_evaluacion WHERE evaluacion_id=?1", [&id], |fila| fila.get(0))?;
        Ok(Json(serde_json::from_str(&detalle).map_err(ErrorApi::interno)?))
    }).await
}

pub(crate) fn autorizar(db: &Connection, id: &str, cabeceras: &HeaderMap) -> Result<Identidad, ErrorApi> {
    let id: String = db.query_row(
        "SELECT e.intento_id FROM evaluaciones v JOIN entregas e ON e.id=v.entrega_id WHERE v.id=?1",
        [id], |fila| fila.get(0),
    ).optional()?.ok_or_else(|| ErrorApi::nuevo(StatusCode::FORBIDDEN, "La evaluación está reservada al docente responsable."))?;
    autorizacion::docente_del_intento(db, cabeceras, &id)
}
