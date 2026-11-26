use axum::http::{HeaderMap, StatusCode};
use rusqlite::{OptionalExtension, Transaction, params};
use serde::{Serialize, de::DeserializeOwned};
use sha2::{Digest, Sha256};

use crate::app::errores::ErrorApi;

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
            "SELECT huella, respuesta FROM operaciones WHERE identidad_id=?1 AND ambito=?2 AND clave=?3",
            params![self.identidad, self.ambito, self.clave],
            |fila| Ok((fila.get::<_, String>(0)?, fila.get::<_, String>(1)?)),
        ).optional()?;
        anterior.map(|(huella, respuesta)| {
            if huella != self.huella {
                return Err(ErrorApi::nuevo(StatusCode::CONFLICT, "La clave ya se usó con otro contenido."));
            }
            serde_json::from_str(&respuesta).map_err(ErrorApi::interno)
        }).transpose()
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
