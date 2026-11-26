use std::{path::Path, time::Duration};

use rusqlite::{Connection, Error, OpenFlags, Result, TransactionBehavior};

const MIGRACIONES: &[&str] = &[
    include_str!("../migrations/0001_fuentes.sql"),
    include_str!("../migrations/0002_asignaciones.sql"),
    include_str!("../migrations/0003_intentos.sql"),
    include_str!("../migrations/0004_mediciones.sql"),
    include_str!("../migrations/0004a_operaciones.sql"),
    include_str!("../migrations/0005_hechos.sql"),
    include_str!("../migrations/0006_turnos.sql"),
    include_str!("../migrations/0007_trazas.sql"),
    include_str!("../migrations/0007a_procedencia_ensayo.sql"),
    include_str!("../migrations/0007b_exploraciones.sql"),
    include_str!("../migrations/0008_ordenes.sql"),
    include_str!("../migrations/0009_razonamiento.sql"),
    include_str!("../migrations/0010_hipotesis.sql"),
    include_str!("../migrations/0010a_mensajes.sql"),
    include_str!("../migrations/0010b_memoria.sql"),
    include_str!("../migrations/0010c_medios_examen.sql"),
    include_str!("../migrations/0010_rubricas.sql"),
    include_str!("../migrations/0010d_rubrica_intento.sql"),
    include_str!("../migrations/0011_entregas.sql"),
    include_str!("../migrations/0012_evaluacion.sql"),
    include_str!("../migrations/0013_revisiones.sql"),
    include_str!("../migrations/0013a_publicacion.sql"),
];
const VERSION_REVISIONES: usize = 21;

pub fn abrir(ruta: impl AsRef<Path>) -> Result<Connection> {
    configurar(Connection::open(ruta)?)
}

pub fn abrir_existente(ruta: impl AsRef<Path>) -> Result<Connection> {
    configurar(Connection::open_with_flags(ruta, OpenFlags::SQLITE_OPEN_READ_WRITE)?)
}

fn configurar(conexion: Connection) -> Result<Connection> {
    conexion.busy_timeout(Duration::from_secs(5))?;
    conexion.pragma_update(None, "foreign_keys", true)?;
    Ok(conexion)
}

pub fn migrar(conexion: &mut Connection) -> Result<()> {
    let transaccion = conexion.transaction_with_behavior(TransactionBehavior::Immediate)?;
    let actual: usize = transaccion.pragma_query_value(None, "user_version", |fila| fila.get(0))?;
    let pendientes = MIGRACIONES.get(actual..).ok_or_else(|| {
        Error::InvalidParameterName(format!("versión de SQLite no reconocida: {actual}"))
    })?;

    for migracion in pendientes {
        transaccion.execute_batch(migracion)?;
    }
    if actual < VERSION_REVISIONES {
        crate::modules::evaluacion::revisiones::inicializar_existentes(&transaccion)
            .map_err(|error| Error::ToSqlConversionFailure(error.into()))?;
    }
    transaccion.pragma_update(None, "user_version", MIGRACIONES.len())?;
    transaccion.commit()
}
