use std::process::{Command, Output};

use axum::http::StatusCode;
use rusqlite::Connection;
use sircle::modules::evaluacion::rubricas::{self, Archivo, Constancia, Registro};

use super::{Servidor, intento_lr};

pub const V1: &str = include_str!("rubricas/lr-v1.json");
pub const V2: &str = include_str!("rubricas/lr-v2.json");
pub const ESTUDIANTE_B: &str = "estudiante-t055-b";

pub struct IntentoCalibracion {
    pub cookie: String,
    pub id: String,
    pub rubrica: Registro,
}

pub fn asignar_segundo_estudiante(db: &Connection) {
    db.execute_batch(
        "INSERT INTO identidades VALUES ('estudiante-t055-b','estudiante');
         INSERT INTO asignaciones VALUES ('lr-ensayo','estudiante-t055-b','docente-demo');",
    ).unwrap();
}

pub fn cargar(db: &mut Connection, fuente: &str) -> Registro {
    let mut archivo: Archivo = serde_json::from_str(fuente).unwrap();
    archivo.aprobacion = Some(Constancia {
        tipo: "ensayo".into(), docente_id: "docente-demo".into(), fecha: "2026-11-26T00:00:00Z".into(),
        motivo: "Constancia ficticia para contraste de rúbricas de ensayo".into(),
        codigo: archivo.contenido.codigo.clone(), version: archivo.contenido.version.clone(),
        sha256: rubricas::huella(&archivo.contenido).unwrap(),
    });
    rubricas::cargar_ensayo(db, &archivo).unwrap()
}

pub fn habilitar_por_cli(servidor: &Servidor, id: i64) -> Output {
    Command::new(env!("CARGO_BIN_EXE_sircle"))
        .arg("habilitar-rubrica-ensayo").arg("--db").arg(&servidor.db)
        .arg("--rubrica-id").arg(id.to_string())
        .output().unwrap()
}

pub async fn abrir(servidor: &Servidor, estudiante: &str, rubrica: Registro) -> IntentoCalibracion {
    let (cookie, _) = servidor.entrar(estudiante).await;
    let respuesta = intento_lr::alta(servidor, &cookie).await;
    assert_eq!(respuesta.estado, StatusCode::CREATED);
    assert_eq!(respuesta.cuerpo["rubrica_id"], rubrica.id);
    assert_eq!(respuesta.cuerpo["version_rubrica"], rubrica.version);
    assert_eq!(respuesta.cuerpo["rubrica_sha256"], rubrica.sha256);
    let id = respuesta.cuerpo["intento_id"].as_str().unwrap().to_owned();
    IntentoCalibracion { cookie, id, rubrica }
}
