use anyhow::{Result, ensure};
use rusqlite::{Connection, OptionalExtension, params};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Instrumento {
    pub version: String,
    pub escala: [String; 5],
    pub percepcion: [String; 3],
    a: [Ejercicio; 2],
    b: [Ejercicio; 2],
    pub utilidad: [String; 2],
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Ejercicio {
    pregunta: String,
    opciones: [String; 3],
    pub clave: u8,
    justificacion: String,
}

#[derive(Serialize)]
pub struct Item {
    pub id: String,
    tipo: &'static str,
    enunciado: String,
    pub opciones: Vec<String>,
}

impl Instrumento {
    pub fn ensayo() -> Result<Self> {
        Ok(serde_json::from_str(include_str!("instrumento-ensayo-v1.json"))?)
    }

    pub fn ejercicios(&self, forma: &str) -> &[Ejercicio; 2] {
        if forma == "A" { &self.a } else { &self.b }
    }

    pub fn items(&self, forma: &str, post: bool) -> Vec<Item> {
        let mut items: Vec<_> = self.percepcion.iter().enumerate().map(|(i, enunciado)| Item {
            id: format!("P{}", i+1), tipo: "percepcion", enunciado: enunciado.clone(),
            opciones: self.escala.to_vec(),
        }).collect();
        items.extend(self.ejercicios(forma).iter().enumerate().map(|(i, ejercicio)| Item {
            id: format!("R{}", i+1), tipo: "razonamiento", enunciado: ejercicio.pregunta.clone(),
            opciones: ejercicio.opciones.to_vec(),
        }));
        if post {
            items.extend(self.utilidad.iter().enumerate().map(|(i, enunciado)| Item {
                id: format!("U{}", i+1), tipo: "utilidad", enunciado: enunciado.clone(),
                opciones: self.escala.to_vec(),
            }));
        }
        items
    }

    pub fn huella(&self) -> Result<String> {
        Ok(format!("{:x}", Sha256::digest(serde_json::to_vec(self)?)))
    }
}

pub(crate) fn guardar(db: &Connection, instrumento: &Instrumento, constancia: Option<&str>) -> Result<()> {
    let textos = instrumento.escala.iter().chain(&instrumento.percepcion).chain(&instrumento.utilidad);
    ensure!(!instrumento.version.trim().is_empty() && textos.into_iter().all(|texto| !texto.trim().is_empty()),
        "La versión, la escala y los enunciados del instrumento son obligatorios.");
    for ejercicio in instrumento.a.iter().chain(&instrumento.b) {
        ensure!((1..=3).contains(&ejercicio.clave) && !ejercicio.pregunta.trim().is_empty()
            && !ejercicio.justificacion.trim().is_empty()
            && ejercicio.opciones.iter().all(|opcion| !opcion.trim().is_empty()),
            "Cada ejercicio requiere una pregunta, tres alternativas, clave y justificación.");
    }
    let sha256 = instrumento.huella()?;
    db.execute(
        "INSERT INTO instrumentos_educativos VALUES (?1,?2,?3,?4) ON CONFLICT(version) DO NOTHING",
        params![instrumento.version, serde_json::to_string(instrumento)?, sha256, constancia],
    )?;
    let anterior: (String, Option<String>) = db.query_row(
        "SELECT sha256,constancia FROM instrumentos_educativos WHERE version=?1",
        [&instrumento.version], |fila| Ok((fila.get(0)?, fila.get(1)?)),
    )?;
    ensure!(anterior == (sha256, constancia.map(str::to_owned)), "La versión educativa ya tiene otro contenido o constancia.");
    Ok(())
}

pub(crate) fn leer(db: &Connection, version: &str) -> Result<Option<Instrumento>> {
    let contenido: Option<String> = db.query_row(
        "SELECT contenido FROM instrumentos_educativos WHERE version=?1",
        [version], |fila| fila.get(0),
    ).optional()?;
    Ok(contenido.map(|contenido| serde_json::from_str(&contenido)).transpose()?)
}
