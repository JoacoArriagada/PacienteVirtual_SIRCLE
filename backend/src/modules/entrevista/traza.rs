use axum::{
    Json,
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
};
use rusqlite::{OptionalExtension, params};
use serde_json::{Value, json};

use crate::{
    app::{errores::ErrorApi, estado::Estado},
    shared::autorizacion,
};

pub async fn rechazos(
    State(app): State<Estado>, Path(intento): Path<String>, cabeceras: HeaderMap,
) -> Result<Json<Value>, ErrorApi> {
    app.consultar(move |db| {
        let tx = db.transaction()?;
        autorizacion::docente_del_intento(&tx, &cabeceras, &intento)?;
        let mut consulta = tx.prepare(
            "SELECT operacion_id,agente,motivo,momento,detalle FROM rechazos_mensajes
             WHERE intento_id=?1 ORDER BY id DESC LIMIT 50",
        )?;
        let filas = consulta.query_map([&intento], |fila| Ok((
            fila.get::<_, String>(0)?, fila.get::<_, String>(1)?, fila.get::<_, String>(2)?,
            fila.get::<_, String>(3)?, fila.get::<_, Option<String>>(4)?,
        )))?;
        let mut rechazos = Vec::new();
        for fila in filas {
            let (operacion, agente, motivo, momento, detalle) = fila?;
            let detalle: Option<Value> = detalle.map(|detalle| serde_json::from_str(&detalle)).transpose().map_err(ErrorApi::interno)?;
            rechazos.push(json!({ "operacion_id": operacion, "agente": agente, "motivo": motivo,
                "momento": momento, "detalle": detalle }));
        }
        Ok(Json(json!({ "intento_id": intento, "rechazos": rechazos })))
    }).await
}

pub async fn leer(
    State(app): State<Estado>,
    Path((intento_id, turno_id)): Path<(String, String)>,
    cabeceras: HeaderMap,
) -> Result<Json<Value>, ErrorApi> {
    app.consultar(move |db| {
        let tx = db.transaction()?;
        autorizacion::docente_del_intento(&tx, &cabeceras, &intento_id)?;
        let (operacion_id, detalle): (String, String) = tx.query_row(
            "SELECT tr.operacion_id, tr.detalle
             FROM trazas_turno tr JOIN turnos t ON t.id=tr.turno_id
             WHERE t.intento_id=?1 AND t.id=?2",
            params![intento_id, turno_id],
            |fila| Ok((fila.get(0)?, fila.get(1)?)),
        ).optional()?.ok_or_else(reservada)?;
        let detalle: Value = serde_json::from_str(&detalle).map_err(ErrorApi::interno)?;
        let mut respuesta = json!({
            "intento_id": intento_id,
            "turno_id": turno_id,
            "operacion_id": operacion_id,
            "invocacion": detalle["invocacion"],
            "hecho": detalle["hecho"],
            "origen": detalle["origen"].as_str().unwrap_or("ollama"),
            "sugerencia_id": detalle["sugerencia_id"],
        });
        if let Some(mensaje) = detalle.get("mensaje") {
            respuesta["mensaje"] = mensaje.clone();
        }
        for campo in ["motivo", "hechos", "version_instrucciones", "medicion_id", "banda_temperatura",
            "redirigio", "turnos_referidos", "partes_aplicadas"] {
            if let Some(valor) = detalle.get(campo) {
                respuesta[campo] = valor.clone();
            }
        }
        Ok(Json(respuesta))
    }).await
}

fn reservada() -> ErrorApi {
    ErrorApi::nuevo(StatusCode::FORBIDDEN, "La traza está reservada al docente responsable.")
}
