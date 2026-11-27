use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;

#[derive(Debug)]
pub struct ErrorApi {
    pub estado: StatusCode,
    pub mensaje: &'static str,
}

impl ErrorApi {
    pub fn nuevo(estado: StatusCode, mensaje: &'static str) -> Self {
        Self { estado, mensaje }
    }

    pub fn interno(error: impl std::fmt::Display) -> Self {
        eprintln!("Fallo interno de la API: {error}");
        Self::nuevo(StatusCode::INTERNAL_SERVER_ERROR, "No se pudo completar la operación.")
    }
}

impl From<rusqlite::Error> for ErrorApi {
    fn from(error: rusqlite::Error) -> Self {
        Self::interno(error)
    }
}

impl IntoResponse for ErrorApi {
    fn into_response(self) -> Response {
        (self.estado, Json(json!({ "mensaje": self.mensaje }))).into_response()
    }
}
