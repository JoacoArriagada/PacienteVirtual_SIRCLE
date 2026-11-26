mod fixtures;

use std::path::Path;

use axum::http::StatusCode;
use fixtures::{Servidor, intento_lr};
use serde_json::json;
use sircle::{modules::escenarios::importacion, sqlite};

#[tokio::test]
async fn respalda_y_reabre_el_mismo_intento_en_otra_base() {
    let servidor = Servidor::nuevo();
    let (cookie, _) = servidor.entrar(intento_lr::ESTUDIANTE_LR).await;
    let alta = intento_lr::alta(&servidor, &cookie).await;
    let id = alta.cuerpo["intento_id"].as_str().unwrap();
    intento_lr::resolver_pre(&servidor, id);
    let ruta = format!("/api/intentos/{id}");
    let turno = servidor.peticion("POST", &format!("{ruta}/turnos"), Some(&cookie), Some("turno"),
        Some(json!({"texto":"¿Desde cuándo comenzó?","sugerencia_id":"inicio"}))).await;
    assert_eq!(turno.estado, StatusCode::CREATED);
    let medicion = servidor.peticion("POST", &format!("{ruta}/mediciones"), Some(&cookie), Some("presion"),
        Some(json!({"instrumento":"presion","accion":"toma_completa"}))).await;
    assert_eq!(medicion.estado, StatusCode::CREATED);
    let original = servidor.consultar("GET", &ruta, Some(&cookie), None).await.cuerpo;
    let fuentes = Path::new(env!("CARGO_MANIFEST_DIR")).join("../datos");
    let inventario = |db: &Path| importacion::preparar(db, &fuentes).unwrap().into_iter()
        .map(|fila| (fila.fuente_id, fila.revision_id, fila.version_id, fila.sha256)).collect::<Vec<_>>();
    let antes = inventario(&servidor.db);
    assert_eq!(antes.len(), 14);
    assert_eq!(inventario(&servidor.db), antes);
    let restaurada = Servidor::copiar(&servidor.db);
    assert!(sqlite::respaldar(&servidor.db, &restaurada.db).is_err());
    assert_eq!(inventario(&restaurada.db), antes);
    let vista = restaurada.consultar("GET", &ruta, Some(&cookie), None).await;
    assert_eq!(vista.estado, StatusCode::OK);
    assert_eq!(vista.cuerpo, original);
    assert_eq!(vista.cuerpo["turnos"][0]["turno_id"], turno.cuerpo["turno_id"]);
    assert_eq!(vista.cuerpo["mediciones"][0]["medicion_id"], medicion.cuerpo["medicion_id"]);
    let db = sqlite::abrir_existente(&restaurada.db).unwrap();
    let integridad: String = db.query_row("PRAGMA integrity_check", [], |fila| fila.get(0)).unwrap();
    assert_eq!(integridad, "ok");
}
