use std::collections::HashSet;

use crate::shared::contexto::ContextoEntrevista;

use super::{
    hechos::Hecho,
    ollama::{MAX_PARTES, Parte, Seleccion, Situacion},
};

const SALUDOS: &[&str] = &["Hola.", "Buenos días.", "Buenas tardes.", "Buenas noches."];

pub(super) struct Dialogo<'a> {
    pub texto: String,
    pub declaracion: Option<String>,
    pub hechos: Vec<&'a Hecho>,
    pub turnos_referidos: Vec<&'a str>,
    pub partes_aplicadas: Vec<Situacion>,
    pub tipo: &'static str,
    pub motivo: Option<&'static str>,
    pub redirigio: bool,
}

pub(super) fn renderizar<'a>(
    contexto: &'a ContextoEntrevista, enviados: &[Hecho], pregunta: &str, plan: &Seleccion,
) -> Result<Dialogo<'a>, &'static str> {
    use Situacion::*;

    if plan.partes.is_empty() || plan.partes.len() > MAX_PARTES {
        return Err("cantidad_partes_invalida");
    }
    let partes = reconocer_apertura(contexto, pregunta, &plan.partes);
    let tipos: HashSet<_> = partes.iter().map(|parte| parte.tipo).collect();
    if tipos.contains(&Hecho) && pide_evaluacion(pregunta) && !tipos.contains(&FueraDeTema) {
        return Err("solicitud_de_evaluacion_fuera_de_rol");
    }
    let ultima = contexto.memoria.last();
    let mut texto = Vec::new();
    let mut declaraciones = Vec::new();
    let mut seleccionados = Vec::<&super::hechos::Hecho>::new();
    let mut turnos_referidos = Vec::new();
    let mut partes_sociales = HashSet::new();
    let mut partes_aplicadas = Vec::new();
    for parte in &partes {
        let variante = parte.variante.unwrap_or_default();
        if parte.tipo == Hecho {
            let id = parte.hecho_id.as_deref().ok_or("hecho_sin_identificador")?;
            let hecho = contexto.candidatos.iter().find(|hecho| hecho.id == id && enviados.contains(hecho))
                .ok_or("hecho_fuera_del_contexto")?;
            if seleccionados.contains(&hecho) {
                return Err("hecho_repetido");
            }
            if let Some(referencia) = parte.referente {
                let turno = referencia.checked_sub(1).and_then(|indice| contexto.memoria.get(indice))
                    .ok_or("referente_fuera_del_contexto")?;
                if !contexto.candidatos.iter().any(|dato| turno.hechos.contains(&dato.id) && dato.tema == hecho.tema) {
                    return Err("referente_sin_hecho_del_tema");
                }
                let turno_id = turno.turno_id.as_str();
                if !turnos_referidos.contains(&turno_id) {
                    turnos_referidos.push(turno_id);
                }
            } else if alusion_sin_tema(pregunta) {
                return Err("hecho_sin_referente");
            }
            let frase = hecho.redaccion(variante).ok_or("variante_no_autorizada")?;
            declaraciones.push(frase);
            seleccionados.push(hecho);
            texto.push(frase);
            partes_aplicadas.push(Hecho);
            continue;
        }
        if !partes_sociales.insert(parte.tipo) {
            continue;
        }
        let alternativas: &[&str] = match parte.tipo {
            Saludo => SALUDOS,
            Cortesia => &["Gracias.", "De nada.", "Hasta luego."],
            Aclaracion => &["¿Se refiere a algo en particular?", "¿Me puede precisar la pregunta?", "¿A qué se refiere?"],
            DatoAusente => &["No sabría decirle.", "Eso no se lo podría decir.", "Sobre eso no sabría responderle."],
            FueraDeTema => &["Sobre eso no sé qué decirle.", "De eso no sabría hablarle.", "¿Podemos seguir con la consulta?"],
            Hecho => unreachable!(),
        };
        let variante = if variante < alternativas.len() { variante } else { 0 };
        let mut frase = alternativas[variante];
        if matches!(parte.tipo, Aclaracion | DatoAusente | FueraDeTema)
            && ultima.is_some_and(|turno| turno.respuesta.contains(frase))
        {
            frase = alternativas[(variante + 1) % alternativas.len()];
        }
        texto.push(frase);
        partes_aplicadas.push(parte.tipo);
    }

    let redirigio = plan.retomar && declaraciones.is_empty() && !tipos.contains(&Aclaracion)
        && (tipos.contains(&DatoAusente) || tipos.contains(&FueraDeTema))
        && !ultima.is_some_and(|turno| turno.redirigio)
        && !texto.iter().any(|frase| frase.ends_with('?'));
    if redirigio {
        texto.push(retomar(contexto));
    }

    let (tipo, motivo) = if !declaraciones.is_empty() { ("hecho", None) }
        else if tipos.contains(&Aclaracion) { ("aclaracion", Some("referente_ambiguo")) }
        else if tipos.contains(&DatoAusente) { ("dato_ausente", Some("dato_no_registrado")) }
        else if tipos.contains(&FueraDeTema) { ("fuera_de_tema", Some("pregunta_fuera_de_tema")) }
        else { ("social", None) };
    Ok(Dialogo {
        texto: texto.join(" "),
        declaracion: (!declaraciones.is_empty()).then(|| declaraciones.join(" ")),
        hechos: seleccionados,
        turnos_referidos,
        partes_aplicadas,
        tipo,
        motivo,
        redirigio,
    })
}

fn reconocer_apertura(contexto: &ContextoEntrevista, pregunta: &str, partes: &[Parte]) -> Vec<Parte> {
    use Situacion::*;
    let parte_simple = |tipo, variante| Parte { tipo, variante, hecho_id: None, referente: None };
    let pregunta = pregunta.trim_start_matches(|c: char| !c.is_alphanumeric()).to_lowercase();
    let mut consulta = pregunta.as_str();
    let mut respuesta = Vec::new();
    for (saludo, variante) in [("buenos días", 1), ("buen día", 1), ("buenas tardes", 2), ("buenas noches", 3), ("hola", 0), ("buenas", 0)] {
        let Some(resto) = pregunta.strip_prefix(saludo) else { continue };
        if resto.starts_with(char::is_alphanumeric) {
            continue;
        }
        respuesta.push(parte_simple(Saludo, Some(variante)));
        consulta = resto;
        break;
    }
    if !respuesta.is_empty() && !consulta.contains(char::is_alphanumeric) {
        return respuesta;
    }
    if alusion_sin_tema(consulta) && contexto.memoria.iter().all(|turno| turno.hechos.is_empty()) {
        respuesta.push(parte_simple(Aclaracion, None));
        return respuesta;
    }

    // Estas señales corrigen el acto de conversación sin convertir la pregunta en un hecho.
    let tema_ajeno = ["color favorito", "película favorita", "equipo favorito", "quién ganó el partido"]
        .iter().any(|tema| consulta.contains(tema));
    let consulta_clinica = ["dolor", "duele", "fiebre", "inhalador", "síntoma", "temperatura", "respirar",
        "medicamento", "sangre", "asma", "consulta", "edad", "se llama", "te llamas"]
        .iter().any(|termino| consulta.contains(termino));
    if tema_ajeno && !consulta_clinica {
        respuesta.push(parte_simple(FueraDeTema, None));
        return respuesta;
    }
    let saludo_explicito = !respuesta.is_empty();
    respuesta.extend(partes.iter().filter(|parte| !saludo_explicito || parte.tipo != Saludo).cloned().map(|mut parte| {
        if parte.tipo == FueraDeTema && consulta_clinica && !tema_ajeno && !pide_evaluacion(consulta) {
            parte.tipo = DatoAusente;
        }
        parte
    }));
    respuesta
}

fn retomar(contexto: &ContextoEntrevista) -> &'static str {
    let ultimo_hecho = contexto.memoria.iter().rev().find_map(|turno| turno.hechos.last());
    let tema = ultimo_hecho.and_then(|id| contexto.candidatos.iter().find(|hecho| &hecho.id == id));
    match tema.map(|hecho| hecho.tema.as_str()) {
        Some("inicio") => "¿Quiere que sigamos con cómo empezó?",
        Some("inhalador") => "¿Quiere que sigamos con lo del inhalador?",
        Some("dolor") => "¿Quiere que sigamos con lo del dolor?",
        Some("tos") => "¿Quiere que sigamos con lo de la tos?",
        Some(_) => "¿Quiere que sigamos con lo que hablábamos?",
        None => "¿Qué le gustaría saber?",
    }
}

fn alusion_sin_tema(pregunta: &str) -> bool {
    let pregunta = pregunta.trim_matches(|c: char| c.is_whitespace() || matches!(c, '¿' | '?' | '.' | '¡' | '!'));
    ["y eso", "eso", "y entonces"].iter().any(|alusion| pregunta.eq_ignore_ascii_case(alusion))
}

fn pide_evaluacion(pregunta: &str) -> bool {
    let pregunta = pregunta.to_lowercase();
    let palabras: Vec<_> = pregunta.split(|c: char| !c.is_alphabetic()).filter(|parte| !parte.is_empty()).collect();
    palabras.iter().any(|palabra| matches!(*palabra, "rúbrica" | "rubrica" | "califícame" | "calificame"))
        || palabras.windows(2).any(|par| matches!(par, ["mi", "calificación" | "calificacion" | "puntaje" | "nota"] | ["nota", "me"]))
}
