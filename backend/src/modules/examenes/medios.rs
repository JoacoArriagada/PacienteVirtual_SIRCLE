use std::path::Path;

use rusqlite::{Connection, OptionalExtension, params};
use serde::Serialize;

use crate::{agentes::dan_sparks::perfil, app::errores::ErrorApi, modules::intentos::intento::Intento};

#[derive(Debug, Serialize)]
pub struct Medio {
    pub examen: String,
    pub medio: String,
    pub disponibilidad: String,
    pub fuente: String,
    pub revision_fuente_id: i64,
}

pub(crate) fn preparar_dan(db: &Connection, catalogo: &Path) -> rusqlite::Result<()> {
    let documento = Path::new(perfil::FUENTE_ECG);
    let medio = documento.parent().unwrap_or(Path::new("")).join(perfil::REFERENCIA_ECG);
    let disponibilidad = if catalogo.join(medio).is_file() { "localizado" } else { "indisponible" };
    db.execute(
        "INSERT INTO medios_referidos
         SELECT c.version_caso_id, 'inicial', 'ecg', ?1, r.id, ?2
         FROM configuraciones_ensayo c JOIN versiones_fuentes vf ON vf.version_clinica_id=c.version_caso_id
         JOIN fuentes f ON f.id=vf.fuente_id AND f.rol='environment_controller'
         JOIN revisiones_fuente r ON r.id=vf.revision_fuente_id
         WHERE c.actividad_id=?3 AND f.ruta=?4 AND instr(CAST(r.contenido AS TEXT),?1)>0
         ON CONFLICT(version_caso_id, fase, examen, medio) DO UPDATE SET disponibilidad=excluded.disponibilidad",
        params![perfil::REFERENCIA_ECG, disponibilidad, perfil::ACTIVIDAD, perfil::FUENTE_ECG],
    )?;
    Ok(())
}

pub fn ecg_referido(db: &Connection, intento: &Intento) -> Result<Option<Medio>, ErrorApi> {
    Ok(db.query_row(
        "SELECT m.examen,m.medio,m.disponibilidad,f.ruta,r.id
         FROM intentos i JOIN versiones_clinicas v ON v.id=i.version_caso_id
         JOIN medios_referidos m ON m.version_caso_id=v.id AND m.fase=i.fase
         JOIN revisiones_fuente r ON r.id=m.revision_fuente_id
         JOIN fuentes f ON f.id=r.fuente_id AND f.rol='environment_controller'
         WHERE i.id=?1 AND i.estudiante_id=?2 AND v.version=?3 AND i.fase=?4 AND m.examen='ecg' AND i.pre<>'pendiente'",
        params![intento.intento_id,intento.estudiante_id,intento.version_caso,intento.fase],
        |fila| Ok(Medio { examen: fila.get(0)?, medio: fila.get(1)?, disponibilidad: fila.get(2)?,
            fuente: fila.get(3)?, revision_fuente_id: fila.get(4)? }),
    ).optional()?)
}
