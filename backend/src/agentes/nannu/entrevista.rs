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
        && hecho.id == seleccion && hecho.escenario == contexto.caso
        && hecho.version == contexto.version && hecho.fase == contexto.fase
        && hecho.hablante == contexto.hablante && matches!(hecho.hablante.as_str(), perfil::PACIENTE | perfil::ACOMPANANTE)
    ).ok_or_else(|| ErrorApi::nuevo(StatusCode::SERVICE_UNAVAILABLE, "La selección no corresponde al hablante autorizado de P02."))
}

pub(crate) fn pertinente(hecho: &Hecho, pregunta: &str) -> bool {
    let palabras = pregunta.split(|caracter: char| !caracter.is_alphabetic()).map(str::to_lowercase);
    let permitidas: &[&str] = match hecho.id.as_str() {
        "p02-dolor" => &["dolor", "duele", "molest"],
        "p02-tos-productiva" => &["tos", "flema", "expector"],
        _ => &[],
    };
    palabras.into_iter().any(|palabra| permitidas.iter().any(|raiz| palabra.starts_with(raiz)))
}
