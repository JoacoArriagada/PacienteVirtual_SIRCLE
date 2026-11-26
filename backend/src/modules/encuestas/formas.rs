use rusqlite::{Connection, params};

pub(crate) fn asignar(db: &Connection, id: &str, actividad: &str, version: &str) -> rusqlite::Result<()> {
    db.execute(
        "INSERT INTO secuencias_educativas (intento_id, actividad_id, version, ordinal)
         SELECT ?1, ?2, ?3, 1+coalesce(max(ordinal),0)
         FROM secuencias_educativas WHERE actividad_id=?2 AND version=?3",
        params![id, actividad, version],
    )?;
    Ok(())
}

pub(crate) fn pre(ordinal: i64) -> &'static str {
    if ordinal % 2 == 0 { "B" } else { "A" }
}

pub(crate) fn post(ordinal: i64) -> &'static str {
    if ordinal % 2 == 0 { "A" } else { "B" }
}
