use std::path::PathBuf;

use anyhow::{Context, Result, ensure};
use axum::http::StatusCode;
use rusqlite::Connection;

use crate::{agentes::supervisor::Supervisor, modules::entrevista::ollama::Ollama, sqlite};

use super::errores::ErrorApi;

#[derive(Clone, Default)]
pub struct Estado {
    pub(crate) db: Option<PathBuf>,
    pub(crate) trabajadores: Option<Supervisor>,
    pub(crate) ollama: Option<Ollama>,
}

impl Estado {
    pub fn con_base(db: PathBuf) -> Result<Self> {
        let conexion = sqlite::abrir_existente(&db)
            .with_context(|| format!("No se pudo abrir {}. Ejecuta sircle preparar.", db.display()))?;
        let preparada = catalogo_preparado(&conexion)
            .context("No se encontró el catálogo preparado. Ejecuta sircle preparar.")?;
        ensure!(preparada, "Catálogo incompleto. Ejecuta sircle preparar.");
        Ok(Self { db: Some(db), ..Self::default() })
    }

    pub(crate) async fn base_disponible(&self) -> bool {
        self.consultar(|db| Ok(catalogo_preparado(db)?)).await.unwrap_or(false)
    }

    pub fn con_trabajadores(mut self, trabajadores: Supervisor) -> Self {
        self.trabajadores = Some(trabajadores);
        self
    }

    pub fn con_ollama(mut self, ollama: Ollama) -> Self {
        self.ollama = Some(ollama);
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

fn catalogo_preparado(db: &Connection) -> rusqlite::Result<bool> {
    db.query_row(
        "SELECT (SELECT count(*) FROM escenarios)=3 AND (SELECT count(*) FROM fuentes)=14
            AND EXISTS(SELECT 1 FROM configuraciones_ensayo WHERE actividad_id='lr-ensayo')
            AND EXISTS(SELECT 1 FROM instrumentos_educativos WHERE version='instrumento-ensayo-v1')",
        [],
        |fila| fila.get(0),
    )
}
