use axum::http::StatusCode;
use rusqlite::{Transaction, params};
use serde_json::json;

use crate::{
    agentes::mensajes::Cabecera,
    app::errores::ErrorApi,
    modules::{
        entrevista::{hechos::{self, Hecho}, ollama::Invocacion, turnos::{self, Salida}},
        intentos::intento::Intento,
    },
    shared::ficha_aprobada::Sugerencia,
};

use super::perfil;

pub fn confirmar(
    db: &Transaction<'_>,
    contexto: &Intento,
    candidatos: &[Hecho],
    texto: &str,
    invocacion: &Invocacion,
) -> Result<turnos::Turno, ErrorApi> {
    confirmar_respuesta(db, contexto, candidatos, texto, invocacion, None)
}

pub(crate) fn confirmar_correlacionada(
    db: &Transaction<'_>, contexto: &Intento, candidatos: &[Hecho], texto: &str,
    invocacion: &Invocacion, cabecera: &Cabecera,
) -> Result<turnos::Turno, ErrorApi> {
    confirmar_respuesta(db, contexto, candidatos, texto, invocacion, Some(cabecera))
}

fn confirmar_respuesta(
    db: &Transaction<'_>, contexto: &Intento, candidatos: &[Hecho], texto: &str,
    invocacion: &Invocacion, cabecera: Option<&Cabecera>,
) -> Result<turnos::Turno, ErrorApi> {
    let operacion = &invocacion.operacion_id;
    let seleccion = &invocacion.seleccion.hecho_id;
    let actual = validar_contexto(db, contexto, operacion, seleccion)?;
    let mut detalle = json!({ "invocacion": invocacion, "origen": "ollama" });
    if let Some(cabecera) = cabecera {
        detalle["mensaje"] = serde_json::to_value(cabecera).map_err(ErrorApi::interno)?;
    }
    if seleccion == "sin_hecho" {
        if sin_referente(db, &actual, texto)? {
            detalle["motivo"] = json!("referente_ambiguo");
            return turnos::guardar(db, &actual, Salida {
                agente: perfil::AGENTE, hablante: perfil::HABLANTE, texto: "¿A qué te refieres con eso?", tipo: "aclaracion",
            }, texto, operacion, &detalle);
        }
        let consulta = normalizar(texto);
        if ["grupo sanguineo", "tipo de sangre", "tipo sanguineo"].iter().any(|dato| consulta.contains(dato)) {
            detalle["motivo"] = json!("dato_no_registrado");
            return turnos::guardar(db, &actual, Salida {
                agente: perfil::AGENTE,
                hablante: "Sistema",
                texto: "El grupo sanguíneo no está disponible en este escenario.",
                tipo: "dato_ausente",
            }, texto, operacion, &detalle);
        }
    }
    let elegido = candidatos.iter().find(|hecho| hecho.id == invocacion.seleccion.hecho_id)
        .ok_or_else(|| rechazar(operacion, seleccion, StatusCode::SERVICE_UNAVAILABLE, "seleccion_fuera_del_contexto"))?;
    let vigentes = hechos::candidatos_lr(db, &actual)?;
    if !vigentes.contains(elegido) {
        return Err(rechazar(operacion, seleccion, StatusCode::SERVICE_UNAVAILABLE, "caso_fase_o_hablante_invalido"));
    }
    if !pertinente(&elegido.id, texto) {
        return Err(rechazar(operacion, seleccion, StatusCode::SERVICE_UNAVAILABLE, "seleccion_no_pertinente"));
    }

    detalle["hecho"] = serde_json::to_value(elegido).map_err(ErrorApi::interno)?;
    turnos::guardar(db, &actual, Salida::hecho(perfil::AGENTE, elegido), texto, operacion, &detalle)
}

pub fn confirmar_literal(
    db: &Transaction<'_>, contexto: &Intento, texto: &str, operacion: &str, sugerencia: Sugerencia,
) -> Result<turnos::Turno, ErrorApi> {
    let actual = validar_contexto(db, contexto, operacion, sugerencia.hecho_id)?;
    let elegido = hechos::candidatos_lr(db, &actual)?.into_iter()
        .find(|hecho| hecho.id == sugerencia.hecho_id)
        .ok_or_else(|| rechazar(operacion, sugerencia.hecho_id, StatusCode::CONFLICT, "sugerencia_obsoleta"))?;
    let detalle = json!({ "hecho": elegido, "origen": "ficha", "sugerencia_id": sugerencia.id });
    turnos::guardar(db, &actual, Salida::hecho(perfil::AGENTE, &elegido), texto, operacion, &detalle)
}

fn validar_contexto(
    db: &Transaction<'_>, contexto: &Intento, operacion: &str, seleccion: &str,
) -> Result<Intento, ErrorApi> {
    turnos::contexto_vigente(db, contexto).map_err(|error| {
        if error.estado == StatusCode::CONFLICT {
            rechazar(operacion, seleccion, StatusCode::CONFLICT, "contexto_obsoleto")
        } else {
            error
        }
    })
}

fn sin_referente(db: &Transaction<'_>, intento: &Intento, texto: &str) -> Result<bool, ErrorApi> {
    let pregunta = texto.trim_matches(|letra: char| letra.is_whitespace() || matches!(letra, '¿' | '?' | '.'));
    if !pregunta.eq_ignore_ascii_case("y eso") {
        return Ok(false);
    }
    let hay_turno = db.query_row(
        "SELECT EXISTS(SELECT 1 FROM turnos WHERE intento_id=?1 AND agente=?2 AND fase=?3 AND hablante=?4)",
        params![intento.intento_id, perfil::AGENTE.id(), intento.fase, perfil::HABLANTE], |fila| fila.get::<_, bool>(0),
    )?;
    Ok(!hay_turno)
}

fn pertinente(id: &str, texto: &str) -> bool {
    let texto = normalizar(texto);
    let contiene = |partes: &[&str]| partes.iter().any(|parte| texto.contains(parte));
    match id {
        "lr-inicio" => {
            contiene(&["cuand", "desde", "empez", "comenz", "inicio", "tiempo", "dia", "notar"])
                && contiene(&["aire", "respir", "ahog", "sofoc", "disnea", "sintoma"])
        }
        "lr-antecedentes-familiares" => {
            contiene(&["famil", "madre", "padre", "mama", "papa", "herman", "biologic", "hered"])
                && contiene(&["anteced", "enferm", "problema", "asma", "respir", "historia", "salud", "padece", "sabes", "conoces"])
        }
        _ => false,
    }
}

fn normalizar(texto: &str) -> String {
    texto.chars().flat_map(char::to_lowercase).map(|letra| match letra {
        'á' => 'a',
        'é' => 'e',
        'í' => 'i',
        'ó' => 'o',
        'ú' | 'ü' => 'u',
        otra => otra,
    }).collect()
}

fn rechazar(operacion: &str, seleccion: &str, estado: StatusCode, motivo: &str) -> ErrorApi {
    eprintln!("SIRCLE_TURNO_RECHAZADO {}", json!({
        "operacion_id": operacion,
        "seleccion": seleccion,
        "motivo": motivo,
    }));
    let mensaje = if estado == StatusCode::CONFLICT {
        "El intento cambió durante la entrevista. Recarga su estado."
    } else {
        "No se pudo confirmar la respuesta de LR. Reintenta la pregunta."
    };
    ErrorApi::nuevo(estado, mensaje)
}
