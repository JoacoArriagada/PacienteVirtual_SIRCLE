use axum::http::StatusCode;

use crate::{
    app::errores::ErrorApi,
    modules::entrevista::hechos::Hecho,
    shared::contexto::ContextoEntrevista,
};

use super::perfil;

pub fn elegir<'a>(contexto: &'a ContextoEntrevista, seleccion: &str) -> Result<&'a Hecho, ErrorApi> {
    contexto.candidatos.iter().find(|hecho|
        contexto.agente == perfil::AGENTE && contexto.caso == perfil::ESCENARIO
        && contexto.hablante == perfil::HABLANTE && hecho.id == seleccion
        && hecho.escenario == contexto.caso && hecho.version == contexto.version && hecho.fase == contexto.fase
        && hecho.hablante == contexto.hablante && hecho.fuente == perfil::FUENTE_ACTOR
    ).ok_or_else(|| ErrorApi::nuevo(StatusCode::SERVICE_UNAVAILABLE, "La selección no corresponde al relato autorizado de Dan."))
}
