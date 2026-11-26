use std::process::Command;

use serde_json::json;

use super::{Respuesta, Servidor};

pub const ESTUDIANTE_LR: &str = "estudiante-lr";
pub const ALTA_CLAVE: &str = "alta-lr-ensayo";

pub async fn alta(servidor: &Servidor, cookie: &str) -> Respuesta {
    servidor.peticion(
        "POST", "/api/intentos", Some(cookie), Some(ALTA_CLAVE),
        Some(json!({"actividad_id": "lr-ensayo"})),
    ).await
}

pub fn resolver_pre(servidor: &Servidor, id: &str) {
    let salida = Command::new(env!("CARGO_BIN_EXE_sircle"))
        .arg("resolver-pre-ensayo")
        .arg("--db").arg(&servidor.db)
        .arg("--intento").arg(id)
        .output().unwrap();
    assert!(salida.status.success(), "{}", String::from_utf8_lossy(&salida.stderr));
}
