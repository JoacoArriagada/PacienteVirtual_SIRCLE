use axum::http::StatusCode;

use crate::{
    agentes::mensajes::Agente,
    app::errores::ErrorApi,
    modules::intentos::intento::Intento,
    shared::autorizacion,
};

use super::perfil;

pub fn destino(intento: &Intento) -> Result<(Agente, &'static str), ErrorApi> {
    autorizacion::entrevista_nueva(intento)?;
    if intento.caso != perfil::ESCENARIO {
        return Err(ErrorApi::nuevo(StatusCode::FORBIDDEN, "El intento no corresponde al escenario de Dan Sparks."));
    }
    Ok((perfil::AGENTE, perfil::HABLANTE))
}
