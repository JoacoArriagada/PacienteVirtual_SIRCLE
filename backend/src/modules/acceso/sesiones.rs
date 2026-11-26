use axum::{
    Json,
    extract::State,
    http::{HeaderMap, HeaderValue, StatusCode, header},
};
use rusqlite::{Connection, OptionalExtension, TransactionBehavior, params};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::app::{errores::ErrorApi, estado::Estado};

const COOKIE: &str = "sircle_sesion";
const DURACION_SEGUNDOS: i64 = 8 * 60 * 60;

#[derive(Deserialize)]
pub struct AltaSesion {
    identidad: String,
}

#[derive(Serialize)]
pub struct Identidad {
    pub id: String,
    pub rol: String,
}

pub async fn crear(
    State(app): State<Estado>,
    Json(alta): Json<AltaSesion>,
) -> Result<(StatusCode, HeaderMap, Json<Identidad>), ErrorApi> {
    let mut semilla = [0_u8; 32];
    getrandom::fill(&mut semilla).map_err(ErrorApi::interno)?;
    let token = format!("{:x}", Sha256::digest(semilla));
    let huella = huella(&token);
    let identidad = app.consultar(move |db| {
        let tx = db.transaction_with_behavior(TransactionBehavior::Immediate)?;
        let identidad = tx.query_row(
            "SELECT id, rol FROM identidades WHERE id=?1",
            [&alta.identidad],
            leer_identidad,
        ).optional()?.ok_or_else(sesion_requerida)?;
        tx.execute("DELETE FROM sesiones WHERE vence <= unixepoch()", [])?;
        tx.execute(
            "INSERT INTO sesiones (huella, identidad_id, vence) VALUES (?1, ?2, unixepoch() + ?3)",
            params![huella, identidad.id, DURACION_SEGUNDOS],
        )?;
        tx.commit()?;
        Ok(identidad)
    }).await?;

    let cookie = format!("{COOKIE}={token}; Path=/api; HttpOnly; SameSite=Strict; Max-Age={DURACION_SEGUNDOS}");
    let mut cabeceras = HeaderMap::new();
    cabeceras.insert(header::SET_COOKIE, HeaderValue::from_str(&cookie).map_err(ErrorApi::interno)?);
    cabeceras.insert(header::CACHE_CONTROL, HeaderValue::from_static("no-store"));
    Ok((StatusCode::CREATED, cabeceras, Json(identidad)))
}

pub fn autenticar(db: &Connection, cabeceras: &HeaderMap) -> Result<Identidad, ErrorApi> {
    let token = token(cabeceras).ok_or_else(sesion_requerida)?;
    db.query_row(
        "SELECT i.id, i.rol FROM sesiones s JOIN identidades i ON i.id=s.identidad_id
         WHERE s.huella=?1 AND s.vence > unixepoch()",
        [huella(token)],
        leer_identidad,
    ).optional()?.ok_or_else(sesion_requerida)
}

pub async fn cerrar(
    State(app): State<Estado>,
    cabeceras: HeaderMap,
) -> Result<(StatusCode, HeaderMap), ErrorApi> {
    if let Some(token) = token(&cabeceras) {
        let huella = huella(token);
        app.consultar(move |db| {
            db.execute("DELETE FROM sesiones WHERE huella=?1", [huella])?;
            Ok(())
        }).await?;
    }
    let mut respuesta = HeaderMap::new();
    respuesta.insert(header::SET_COOKIE, HeaderValue::from_static("sircle_sesion=; Path=/api; HttpOnly; SameSite=Strict; Max-Age=0"));
    Ok((StatusCode::NO_CONTENT, respuesta))
}

pub async fn leer(State(app): State<Estado>, cabeceras: HeaderMap) -> Result<Json<Identidad>, ErrorApi> {
    app.consultar(move |db| autenticar(db, &cabeceras)).await.map(Json)
}

fn token(cabeceras: &HeaderMap) -> Option<&str> {
    cabeceras.get_all(header::COOKIE).iter()
        .filter_map(|valor| valor.to_str().ok())
        .flat_map(|valor| valor.split(';'))
        .filter_map(|parte| parte.trim().split_once('='))
        .find_map(|(nombre, valor)| (nombre == COOKIE).then_some(valor))
        .filter(|valor| valor.len() == 64 && valor.bytes().all(|byte| byte.is_ascii_hexdigit()))
}

fn huella(token: &str) -> String {
    format!("{:x}", Sha256::digest(token.as_bytes()))
}

fn leer_identidad(fila: &rusqlite::Row<'_>) -> rusqlite::Result<Identidad> {
    Ok(Identidad { id: fila.get(0)?, rol: fila.get(1)? })
}

fn sesion_requerida() -> ErrorApi {
    ErrorApi::nuevo(StatusCode::UNAUTHORIZED, "Selecciona una identidad ficticia disponible.")
}
