use std::collections::HashSet;

use anyhow::Result;
use rusqlite::{Connection, params};
use serde_json::Value;

use crate::agentes::evaluador::perfil::{AcuseSinEvento, TipoEvidencia};

use super::entregas::Instantanea;

pub fn detectar(db: &Connection, intento_id: &str, revision: i64, instantanea: &Instantanea) -> Result<Vec<AcuseSinEvento>> {
    use TipoEvidencia::{Apunte, Hallazgo, Medicion, Orden, Turno};
    let eventos: HashSet<_> = instantanea.turnos.iter().map(|e| (Turno, e.turno_id.as_str()))
        .chain(instantanea.ordenes.iter().map(|e| (Orden, e.orden_id.as_str())))
        .chain(instantanea.exploraciones.iter().map(|e| (Hallazgo, e.exploracion_id.as_str())))
        .chain(instantanea.evidencias.iter().filter_map(|e| match e.tipo.as_str() {
            "medicion" => Some((Medicion, e.origen_id.as_str())),
            "apunte" => Some((Apunte, e.origen_id.as_str())),
            _ => None,
        })).collect();
    let mut consulta = db.prepare(
        "SELECT o.clave,o.respuesta FROM operaciones o JOIN intentos i ON i.id=o.ambito AND i.estudiante_id=o.identidad_id
         WHERE i.id=?1 AND json_extract(o.respuesta,'$.intento_id')=i.id
           AND json_type(o.respuesta,'$.revision')='integer' AND json_extract(o.respuesta,'$.revision')<=?2
         ORDER BY o.clave",
    )?;
    let acuses = consulta.query_map(params![intento_id, revision], |fila|
        Ok((fila.get::<_, String>(0)?, fila.get::<_, String>(1)?)))?;
    let tipos = [(Turno, "turno_id"), (Medicion, "medicion_id"), (Hallazgo, "exploracion_id"),
        (Orden, "orden_id"), (Apunte, "apunte_id")];
    let mut incidencias = Vec::new();
    let mut referencias = HashSet::new();
    for fila in acuses {
        let (clave, respuesta) = fila?;
        let respuesta: Value = serde_json::from_str(&respuesta)?;
        for (tipo, campo) in tipos {
            let Some(id) = respuesta[campo].as_str() else { continue };
            if !eventos.contains(&(tipo, id)) && referencias.insert((tipo, id.to_owned())) {
                incidencias.push(AcuseSinEvento { tipo, referencia: id.to_owned(), clave: clave.clone() });
            }
        }
    }
    Ok(incidencias)
}
