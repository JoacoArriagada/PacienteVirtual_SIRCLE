use axum::http::{HeaderMap, StatusCode};
use rusqlite::{Connection, OptionalExtension, Transaction, params};
use serde::{Serialize, de::DeserializeOwned};
use sha2::{Digest, Sha256};

use crate::app::errores::ErrorApi;

const SEGUNDOS_RESERVA: u64 = 30;

pub struct Operacion<'a> {
    identidad: &'a str,
    ambito: &'a str,
    clave: &'a str,
    huella: String,
}

impl<'a> Operacion<'a> {
    pub fn nueva(
        identidad: &'a str, ambito: &'a str, cabeceras: &'a HeaderMap,
        accion: &str, contenido: &impl Serialize,
    ) -> Result<Self, ErrorApi> {
        let clave = clave(cabeceras)?;
        let cuerpo = serde_json::to_vec(contenido).map_err(ErrorApi::interno)?;
        let mut huella = Sha256::new();
        huella.update(accion.as_bytes());
        huella.update(b"\n");
        huella.update(cuerpo);
        Ok(Self { identidad, ambito, clave, huella: format!("{:x}", huella.finalize()) })
    }

    pub fn recuperar<T: DeserializeOwned>(&self, db: &Transaction<'_>) -> Result<Option<T>, ErrorApi> {
        let anterior = db.query_row(
            "SELECT huella, respuesta FROM operaciones WHERE identidad_id=?1 AND ambito=?2 AND clave=?3
             UNION ALL SELECT huella, NULL FROM reservas_operaciones WHERE identidad_id=?1 AND ambito=?2 AND clave=?3",
            params![self.identidad, self.ambito, self.clave],
            |fila| Ok((fila.get::<_, String>(0)?, fila.get::<_, Option<String>>(1)?)),
        ).optional()?;
        if let Some((huella, respuesta)) = anterior {
            if huella != self.huella {
                return Err(ErrorApi::nuevo(StatusCode::CONFLICT, "La clave ya se usó con otro contenido."));
            }
            return respuesta
                .map(|respuesta| serde_json::from_str(&respuesta).map_err(ErrorApi::interno))
                .transpose();
        }
        Ok(None)
    }

    pub(crate) fn reservar(&self, db: &Transaction<'_>) -> rusqlite::Result<Option<String>> {
        db.query_row(
            "INSERT INTO reservas_operaciones VALUES (?1,?2,?3,?4,lower(hex(randomblob(16))),unixepoch()+?5)
             ON CONFLICT(identidad_id,ambito,clave) DO UPDATE SET toma_id=excluded.toma_id,vence=excluded.vence
             WHERE reservas_operaciones.vence<=unixepoch() AND reservas_operaciones.huella=excluded.huella
             RETURNING toma_id",
            params![self.identidad, self.ambito, self.clave, self.huella, SEGUNDOS_RESERVA],
            |fila| fila.get(0),
        ).optional()
    }

    pub(crate) fn renovar_reserva(db: &Connection, toma: &str) -> rusqlite::Result<bool> {
        let filas = db.execute(
            "UPDATE reservas_operaciones SET vence=unixepoch()+?2 WHERE toma_id=?1 AND vence>unixepoch()",
            params![toma, SEGUNDOS_RESERVA],
        )?;
        Ok(filas == 1)
    }

    pub(crate) fn liberar_reserva(db: &Connection, toma: &str) -> rusqlite::Result<()> {
        db.execute("UPDATE reservas_operaciones SET vence=0 WHERE toma_id=?1", [toma])?;
        Ok(())
    }

    pub(crate) fn confirmar_reserva(
        &self, db: &Transaction<'_>, toma: &str, respuesta: &impl Serialize,
    ) -> Result<(), ErrorApi> {
        let filas = db.execute("DELETE FROM reservas_operaciones WHERE toma_id=?1 AND vence>unixepoch()", [toma])?;
        if filas == 0 {
            return Err(ErrorApi::nuevo(StatusCode::SERVICE_UNAVAILABLE, "La operación venció. Reintenta con la misma clave."));
        }
        self.confirmar(db, respuesta)
    }

    pub fn id(&self) -> String {
        let mut hash = Sha256::new();
        for parte in [self.identidad, self.ambito, self.clave, &self.huella] {
            hash.update((parte.len() as u64).to_be_bytes());
            hash.update(parte.as_bytes());
        }
        format!("{:x}", hash.finalize())[..32].to_owned()
    }

    pub fn confirmar(&self, db: &Transaction<'_>, respuesta: &impl Serialize) -> Result<(), ErrorApi> {
        let respuesta = serde_json::to_string(respuesta).map_err(ErrorApi::interno)?;
        db.execute(
            "INSERT INTO operaciones (identidad_id, ambito, clave, huella, respuesta) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![self.identidad, self.ambito, self.clave, self.huella, respuesta],
        )?;
        Ok(())
    }
}

pub fn clave(cabeceras: &HeaderMap) -> Result<&str, ErrorApi> {
    cabeceras.get("idempotency-key")
        .and_then(|clave| clave.to_str().ok())
        .filter(|clave| !clave.trim().is_empty() && clave.len() <= 128)
        .ok_or_else(|| ErrorApi::nuevo(StatusCode::UNPROCESSABLE_ENTITY, "Indica una clave de operación válida."))
}
