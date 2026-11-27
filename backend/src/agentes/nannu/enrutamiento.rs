use axum::http::StatusCode;

use crate::{
    agentes::mensajes::Agente,
    app::errores::ErrorApi,
    modules::intentos::intento::Intento,
    shared::autorizacion,
};

use super::perfil;

pub fn destino(intento: &Intento, hablante: &str) -> Result<(Agente, &'static str), ErrorApi> {
    autorizacion::entrevista_nueva(intento)?;
    if intento.caso != perfil::ESCENARIO {
        return Err(ErrorApi::nuevo(StatusCode::FORBIDDEN, "El intento no corresponde al escenario de Nannu."));
    }
    let voz = match hablante {
        perfil::PACIENTE => perfil::PACIENTE,
        perfil::ACOMPANANTE => perfil::ACOMPANANTE,
        _ => return Err(ErrorApi::nuevo(StatusCode::UNPROCESSABLE_ENTITY, "Selecciona a Nannu o a su acompañante.")),
    };
    Ok((perfil::AGENTE, voz))
}
