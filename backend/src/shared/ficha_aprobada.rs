use serde::Serialize;

use crate::modules::entrevista::hechos::Hecho;

#[derive(Serialize)]
pub struct Orientacion {
    pub orientacion_id: &'static str,
    pub origen: &'static str,
    pub texto: &'static str,
}

pub const ORIENTACION_INICIAL_LR: Orientacion = Orientacion {
    orientacion_id: "orientacion-inicial-lr",
    origen: "docente",
    texto: "Orienta la entrevista al inicio de los síntomas y su evolución. Vincula tus observaciones con los registros de tu intento.",
};

#[derive(Clone, Copy, Serialize)]
pub struct Sugerencia {
    pub id: &'static str,
    pub texto: &'static str,
    #[serde(skip)]
    pub hecho_id: &'static str,
}

const INICIO: Sugerencia = Sugerencia {
    id: "inicio",
    texto: "¿Desde cuándo comenzó?",
    hecho_id: "lr-inicio",
};

const DOLOR_NANNU: Sugerencia = Sugerencia {
    id: "p02-dolor",
    texto: "¿Te duele el pecho al respirar?",
    hecho_id: "p02-dolor",
};

const TOS_NANNU: Sugerencia = Sugerencia {
    id: "p02-tos",
    texto: "¿Ha tenido tos con flema?",
    hecho_id: "p02-tos-productiva",
};

const INICIO_DAN: Sugerencia = Sugerencia {
    id: "inicio",
    texto: INICIO.texto,
    hecho_id: "p03-inicio",
};

pub fn sugerencias(hechos: &[Hecho]) -> Vec<Sugerencia> {
    [INICIO, DOLOR_NANNU, TOS_NANNU, INICIO_DAN].into_iter()
        .filter(|opcion| hechos.iter().any(|hecho| hecho.id == opcion.hecho_id)).collect()
}

pub fn literal(id: Option<&str>, texto: &str, hechos: &[Hecho]) -> Option<Sugerencia> {
    sugerencias(hechos).into_iter()
        .find(|sugerencia| Some(sugerencia.id) == id && sugerencia.texto == texto)
}
