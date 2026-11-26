#![allow(dead_code)]

use std::path::{Path, PathBuf};

use axum::{
    Router,
    body::{Body, to_bytes},
    http::{HeaderMap, Request, StatusCode, header},
};
use serde_json::{Value, json};
use sircle::{app::{estado::Estado, router::router}, modules::escenarios::importacion};
use tower::ServiceExt;

pub struct Servidor {
    _directorio: tempfile::TempDir,
    pub db: PathBuf,
    pub app: Router,
}

impl Servidor {
    pub fn nuevo() -> Self {
        let directorio = tempfile::tempdir().unwrap();
        let db = directorio.path().join("prueba.sqlite");
        let datos = Path::new(env!("CARGO_MANIFEST_DIR")).join("../datos");
        importacion::preparar(&db, &datos).unwrap();
        let app = router(Estado::con_base(db.clone()).unwrap());
        Self { _directorio: directorio, db, app }
    }

    pub fn copiar(base: &Path) -> Self {
        let directorio = tempfile::tempdir().unwrap();
        let db = directorio.path().join("prueba.sqlite");
        let origen = sircle::sqlite::abrir_existente(base).unwrap();
        origen.execute("VACUUM INTO ?1", [db.to_str().unwrap()]).unwrap();
        sircle::sqlite::migrar(&mut sircle::sqlite::abrir_existente(&db).unwrap()).unwrap();
        let app = router(Estado::con_base(db.clone()).unwrap());
        Self { _directorio: directorio, db, app }
    }

    pub async fn consultar(
        &self, metodo: &str, ruta: &str, cookie: Option<&str>, cuerpo: Option<Value>,
    ) -> Respuesta {
        self.peticion(metodo, ruta, cookie, None, cuerpo).await
    }

    pub async fn peticion(
        &self, metodo: &str, ruta: &str, cookie: Option<&str>, clave: Option<&str>, cuerpo: Option<Value>,
    ) -> Respuesta {
        let mut peticion = Request::builder().method(metodo).uri(ruta);
        if let Some(cookie) = cookie {
            peticion = peticion.header(header::COOKIE, cookie);
        }
        if let Some(clave) = clave {
            peticion = peticion.header("idempotency-key", clave);
        }
        let cuerpo = match cuerpo {
            Some(cuerpo) => {
                peticion = peticion.header(header::CONTENT_TYPE, "application/json");
                Body::from(cuerpo.to_string())
            }
            None => Body::empty(),
        };
        let res = self.app.clone().oneshot(peticion.body(cuerpo).unwrap()).await.unwrap();
        let (partes, cuerpo) = res.into_parts();
        let bytes = to_bytes(cuerpo, 1024 * 1024).await.unwrap();
        let cuerpo = if bytes.is_empty() { Value::Null } else { serde_json::from_slice(&bytes).unwrap() };
        Respuesta { estado: partes.status, cabeceras: partes.headers, cuerpo }
    }

    pub async fn entrar(&self, identidad: &str) -> (String, Respuesta) {
        let sesion = self.consultar("POST", "/api/sesiones", None, Some(json!({"identidad": identidad}))).await;
        assert_eq!(sesion.estado, StatusCode::CREATED);
        let cookie = sesion.cabeceras[header::SET_COOKIE].to_str().unwrap();
        assert!(cookie.contains("HttpOnly") && cookie.contains("SameSite=Strict"));
        (cookie.split(';').next().unwrap().to_owned(), sesion)
    }
}

pub struct Respuesta {
    pub estado: StatusCode,
    pub cabeceras: HeaderMap,
    pub cuerpo: Value,
}
pub mod intento_lr;
pub mod versiones_rubrica;

