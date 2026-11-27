use std::collections::BTreeMap;

use axum::{Json, extract::{Path, State}, http::{HeaderMap, StatusCode}};
use rusqlite::{Connection, OptionalExtension, TransactionBehavior, params};
use serde::{Deserialize, Serialize};

use crate::{
    app::{errores::ErrorApi, estado::Estado},
    modules::intentos::intento::Intento,
    shared::{autorizacion, operaciones::Operacion},
};

use super::{formas, instrumento::{self, Item}};

pub(crate) type Respuestas = BTreeMap<String, u8>;

#[derive(Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Momento { Pre, Post }

impl Momento {
    pub(crate) fn texto(self) -> &'static str {
        match self {
            Self::Pre => "pre",
            Self::Post => "post",
        }
    }

    fn disponible(self, intento: &Intento) -> Result<(), ErrorApi> {
        if matches!(self, Self::Post) && intento.estado != "entregado" {
            return Err(ErrorApi::nuevo(StatusCode::CONFLICT, "El post se ofrece después de entregar."));
        }
        Ok(())
    }
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
enum Decision { Respondido, Omitido }

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Peticion {
    estado: Decision,
    #[serde(default)]
    respuestas: Respuestas,
}

#[derive(Deserialize, Serialize)]
pub struct Acuse {
    intento_id: String,
    version: String,
    momento: Momento,
    forma: String,
    estado: String,
    instante: Option<String>,
}

#[derive(Serialize)]
pub struct Formulario {
    #[serde(flatten)]
    acuse: Acuse,
    items: Vec<Item>,
}

impl Acuse {
    fn leer(db: &Connection, intento: &Intento, momento: Momento) -> Result<Self, ErrorApi> {
        let id = &intento.intento_id;
        let ordinal: i64 = db.query_row(
            "SELECT ordinal FROM secuencias_educativas WHERE intento_id=?1",
            [id], |fila| fila.get(0),
        )?;
        let forma = match momento {
            Momento::Pre => formas::pre(ordinal),
            Momento::Post => formas::post(ordinal),
        };
        let instante = db.query_row(
            "SELECT instante FROM respuestas_educativas WHERE intento_id=?1 AND etapa=?2",
            params![id, momento.texto()], |fila| fila.get(0),
        ).optional()?;
        Ok(Self {
            intento_id: id.clone(), version: intento.version_instrumento.clone(), momento, forma: forma.into(),
            estado: match momento { Momento::Pre => intento.pre.clone(), Momento::Post => estado_post(db, id)? },
            instante,
        })
    }
}

pub(crate) fn estado_post(db: &Connection, intento: &str) -> rusqlite::Result<String> {
    db.query_row(
        "SELECT coalesce(
            (SELECT estado FROM respuestas_educativas WHERE intento_id=?1 AND etapa='post'),
            (SELECT estado FROM post_ensayo WHERE intento_id=?1), 'pendiente')",
        [intento], |fila| fila.get(0),
    )
}

pub async fn leer(
    State(app): State<Estado>, Path((id, momento)): Path<(String, Momento)>, cabeceras: HeaderMap,
) -> Result<Json<Formulario>, ErrorApi> {
    app.consultar(move |db| {
        let tx = db.transaction()?;
        let (_, intento) = autorizacion::intento_propio(&tx, &cabeceras, &id)?;
        momento.disponible(&intento)?;
        let acuse = Acuse::leer(&tx, &intento, momento)?;
        let items = if acuse.estado == "pendiente" { items(&tx, &acuse)? } else { Vec::new() };
        Ok(Json(Formulario { acuse, items }))
    }).await
}

pub async fn confirmar(
    State(app): State<Estado>, Path((id, momento)): Path<(String, Momento)>, cabeceras: HeaderMap,
    Json(peticion): Json<Peticion>,
) -> Result<Json<Acuse>, ErrorApi> {
    app.consultar(move |db| {
        let tx = db.transaction_with_behavior(TransactionBehavior::Immediate)?;
        let (titular, intento) = autorizacion::intento_propio(&tx, &cabeceras, &id)?;
        let accion = format!("PUT encuestas/{}", momento.texto());
        let operacion = Operacion::nueva(&titular.id, &id, &cabeceras, &accion, &peticion)?;
        if let Some(acuse) = operacion.recuperar(&tx)? {
            return Ok(Json(acuse));
        }
        momento.disponible(&intento)?;
        let mut acuse = Acuse::leer(&tx, &intento, momento)?;
        if acuse.estado != "pendiente" {
            return Err(ErrorApi::nuevo(StatusCode::CONFLICT, "Esta encuesta ya está cerrada."));
        }
        let items = items(&tx, &acuse)?;
        let estado = match peticion.estado {
            Decision::Respondido if !peticion.respuestas.is_empty() => "respondido",
            Decision::Omitido if peticion.respuestas.is_empty() => "omitido",
            _ => return Err(ErrorApi::nuevo(StatusCode::UNPROCESSABLE_ENTITY, "Responde al menos un ítem o elige omitir.")),
        };
        if peticion.respuestas.iter().any(|(id, valor)| {
            !items.iter().any(|item| item.id == *id && (1..=item.opciones.len()).contains(&usize::from(*valor)))
        }) {
            return Err(ErrorApi::nuevo(StatusCode::UNPROCESSABLE_ENTITY, "Alguna respuesta no pertenece a la forma asignada."));
        }
        let respuestas = serde_json::to_string(&peticion.respuestas).map_err(ErrorApi::interno)?;
        let instante: String = tx.query_row(
            "INSERT INTO respuestas_educativas VALUES (?1,?2,?3,strftime('%Y-%m-%dT%H:%M:%fZ','now'),?4)
             RETURNING instante",
            params![id, momento.texto(), estado, respuestas], |fila| fila.get(0),
        )?;
        if matches!(momento, Momento::Pre) {
            tx.execute("UPDATE intentos SET pre=?2, estado='atencion', revision=revision+1 WHERE id=?1", params![id, estado])?;
        }
        acuse.estado = estado.into();
        acuse.instante = Some(instante);
        operacion.confirmar(&tx, &acuse)?;
        tx.commit()?;
        Ok(Json(acuse))
    }).await
}

fn items(db: &Connection, acuse: &Acuse) -> Result<Vec<Item>, ErrorApi> {
    let instrumento = instrumento::leer(db, &acuse.version).map_err(ErrorApi::interno)?
        .ok_or_else(|| ErrorApi::nuevo(StatusCode::CONFLICT, "El instrumento de este intento no está disponible."))?;
    Ok(instrumento.items(&acuse.forma, matches!(acuse.momento, Momento::Post)))
}
