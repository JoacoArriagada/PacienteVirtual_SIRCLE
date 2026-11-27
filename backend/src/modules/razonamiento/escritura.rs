use axum::{Json, extract::{Path, State}, http::{HeaderMap, StatusCode}};
use rusqlite::{TransactionBehavior, params};
use serde::{Deserialize, Serialize};

use crate::{
    app::{errores::ErrorApi, estado::Estado},
    shared::{autorizacion, operaciones::Operacion},
};

use super::{evidencias, hipotesis};

#[derive(Deserialize, Serialize)]
#[serde(untagged)]
pub enum Escritura {
    Apunte(evidencias::Peticion),
    Hipotesis(hipotesis::Peticion),
}

#[derive(Deserialize, Serialize)]
#[serde(untagged)]
pub enum Acuse {
    Apunte(evidencias::Apunte),
    Hipotesis(hipotesis::Borrador),
}

pub async fn guardar(
    State(app): State<Estado>, Path(id): Path<String>, cabeceras: HeaderMap, Json(peticion): Json<Escritura>,
) -> Result<Json<Acuse>, ErrorApi> {
    app.consultar(move |db| {
        let tx = db.transaction_with_behavior(TransactionBehavior::Immediate)?;
        let (identidad, intento) = autorizacion::intento_propio(&tx, &cabeceras, &id)?;
        let (accion, esperada) = match &peticion {
            Escritura::Apunte(contenido) => ("PUT razonamiento: apunte", contenido.revision_esperada),
            Escritura::Hipotesis(contenido) => ("PUT razonamiento: hipotesis", contenido.revision_esperada),
        };
        let operacion = Operacion::nueva(&identidad.id, &id, &cabeceras, accion, &peticion)?;
        if let Some(guardada) = operacion.recuperar(&tx)? {
            return Ok(Json(guardada));
        }
        if intento.estado != "atencion" || intento.pre == "pendiente" {
            return Err(ErrorApi::nuevo(StatusCode::CONFLICT, "El estado del intento impide guardar el razonamiento."));
        }
        if intento.revision != esperada {
            return Err(ErrorApi::nuevo(StatusCode::CONFLICT, "El intento cambió. Revisa la última versión guardada."));
        }
        let revision = intento.revision + 1;
        let acuse = match peticion {
            Escritura::Apunte(contenido) => Acuse::Apunte(evidencias::guardar(&tx, &id, &identidad.id, revision, &contenido.apunte)?),
            Escritura::Hipotesis(contenido) => Acuse::Hipotesis(hipotesis::guardar(&tx, &id, revision, contenido.hipotesis)?),
        };
        tx.execute("UPDATE intentos SET revision=?1 WHERE id=?2", params![revision, id])?;
        operacion.confirmar(&tx, &acuse)?;
        tx.commit()?;
        Ok(Json(acuse))
    }).await
}
