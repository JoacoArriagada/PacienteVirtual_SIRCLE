use axum::{Json, extract::{Path, State}, http::{HeaderMap, StatusCode}};
use rusqlite::{TransactionBehavior, params};

use crate::{
    app::{errores::ErrorApi, estado::Estado},
    shared::{autorizacion, operaciones::Operacion},
};

use super::mediciones::{self, Medicion};

pub async fn registrar(
    State(app): State<Estado>,
    Path((intento_id, medicion_id)): Path<(String, String)>,
    cabeceras: HeaderMap,
) -> Result<Json<Medicion>, ErrorApi> {
    app.consultar(move |db| {
        let tx = db.transaction_with_behavior(TransactionBehavior::Immediate)?;
        let (identidad, intento) = autorizacion::intento_propio(&tx, &cabeceras, &intento_id)?;
        let medicion = mediciones::por_id(&tx, &intento_id, &medicion_id)?;
        let operacion = Operacion::nueva(&identidad.id, &intento_id, &cabeceras, "PUT medicion/ficha", &medicion_id)?;
        if let Some(confirmada) = operacion.recuperar(&tx)? {
            return Ok(Json(confirmada));
        }

        if !medicion.en_ficha {
            if intento.estado != "atencion" {
                return Err(ErrorApi::nuevo(StatusCode::CONFLICT, "El estado del intento impide modificar la ficha."));
            }
            let revision = intento.revision + 1;
            tx.execute(
                "UPDATE mediciones SET en_ficha=1, revision=?1 WHERE intento_id=?2 AND id=?3",
                params![revision, intento_id, medicion_id],
            )?;
            tx.execute("UPDATE intentos SET revision=?1 WHERE id=?2", params![revision, intento_id])?;
        }
        let confirmada = mediciones::por_id(&tx, &intento_id, &medicion_id)?;
        operacion.confirmar(&tx, &confirmada)?;
        tx.commit()?;
        Ok(Json(confirmada))
    }).await
}
