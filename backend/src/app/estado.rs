use std::path::PathBuf;

use anyhow::{Context, Result, ensure};
use axum::http::StatusCode;
use rusqlite::Connection;

use crate::{agentes::supervisor::Supervisor, sqlite};

use super::errores::ErrorApi;

#[derive(Clone, Default)]
pub struct Estado {
    pub(crate) db: Option<PathBuf>,
    pub(crate) trabajadores: Option<Supervisor>,
}

impl Estado {
    pub fn con_base(db: PathBuf) -> Result<Self> {
        let conexion = sqlite::abrir_existente(&db)
            .with_context(|| format!("No se pudo abrir {}. Ejecuta sircle preparar.", db.display()))?;
        let preparados: i64 = conexion.query_row(
            "SELECT count(DISTINCT escenario_id) FROM versiones_clinicas",
            [],
            |fila| fila.get(0),
        ).context("No se encontró el catálogo preparado. Ejecuta sircle preparar.")?;
        ensure!(preparados == 3, "Catálogo incompleto. Ejecuta sircle preparar.");
        Ok(Self { db: Some(db), trabajadores: None })
    }

    pub fn con_trabajadores(mut self, trabajadores: Supervisor) -> Self {
        self.trabajadores = Some(trabajadores);
        self
    }

    pub async fn consultar<T, F>(&self, operacion: F) -> Result<T, ErrorApi>
    where
        T: Send + 'static,
        F: FnOnce(&mut Connection) -> Result<T, ErrorApi> + Send + 'static,
    {
        let ruta = self.db.clone().ok_or_else(|| {
            ErrorApi::nuevo(StatusCode::SERVICE_UNAVAILABLE, "Prepara la base de demostración.")
        })?;
        tokio::task::spawn_blocking(move || operacion(&mut sqlite::abrir_existente(ruta)?))
            .await
            .map_err(ErrorApi::interno)?
    }
}
