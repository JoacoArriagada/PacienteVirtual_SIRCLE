mod fixtures;

use std::{fs, path::Path};

use axum::http::StatusCode;
use fixtures::Servidor;
use serde_json::json;
use sircle::{modules::escenarios::{formal, importacion}, sqlite};

#[tokio::test]
async fn habilita_lr_solo_con_tres_constancias_y_material_coincidente() {
    let inicial = Servidor::nuevo();
    let (cookie, _) = inicial.entrar("estudiante-lr").await;
    let alta = json!({"actividad_id":"lr-formal"});
    let sin_aprobacion = inicial.peticion("POST", "/api/intentos", Some(&cookie), Some("alta"), Some(alta.clone())).await;
    assert_eq!(sin_aprobacion.estado, StatusCode::FORBIDDEN);

    let base = sqlite::abrir_existente(&inicial.db).unwrap();
    let material = formal::borrador(&base, "lr-formal-prueba".into(), "instrumento-formal-prueba".into(), "1".into()).unwrap();
    drop(base);
    let carpeta = tempfile::tempdir().unwrap();
    let archivo = carpeta.path().join("actividad.json");
    let rubrica = carpeta.path().join("rubricas/1.json");
    fs::create_dir(rubrica.parent().unwrap()).unwrap();
    let mut actividad = material["actividad"].clone();
    let mut criterios = material["rubrica"].clone();
    fs::write(&archivo, actividad.to_string()).unwrap();
    fs::write(&rubrica, criterios.to_string()).unwrap();
    let fuentes = Path::new(env!("CARGO_MANIFEST_DIR")).join("../datos");
    assert!(importacion::preparar_con_material(&inicial.db, &fuentes, false, Some(&archivo)).is_err());

    let huellas = formal::huellas(&archivo).unwrap();
    let constancia = |tipo: &str| {
        let campos = &huellas[tipo];
        json!({"tipo":"formal","docente_id":"docente-demo","fecha":"2026-11-25T00:00:00Z",
            "motivo":"Fixture de validación de las tres constancias",
            "codigo":campos["codigo"],"version":campos["version"],"sha256":campos["sha256"]})
    };
    actividad["clinica"]["aprobacion"] = constancia("clinica");
    actividad["instrumento"]["aprobacion"] = constancia("instrumento");
    criterios["aprobacion"] = constancia("rubrica");
    fs::write(&rubrica, criterios.to_string()).unwrap();
    actividad["instrumento"]["aprobacion"]["sha256"] = json!("incorrecta");
    fs::write(&archivo, actividad.to_string()).unwrap();
    assert!(importacion::preparar_con_material(&inicial.db, &fuentes, false, Some(&archivo)).is_err());
    actividad["instrumento"]["aprobacion"] = constancia("instrumento");
    fs::write(&archivo, actividad.to_string()).unwrap();
    for _ in 0..2 {
        importacion::preparar_con_material(&inicial.db, &fuentes, false, Some(&archivo)).unwrap();
    }

    let intento = inicial.peticion("POST", "/api/intentos", Some(&cookie), Some("alta"), Some(alta)).await;
    assert_eq!(intento.estado, StatusCode::CREATED, "{}", intento.cuerpo);
    assert_eq!(intento.cuerpo["tipo"], "formal");
    assert_eq!(intento.cuerpo["version_caso"], "lr-formal-prueba");
    assert_eq!(intento.cuerpo["rubrica_sha256"], huellas["rubrica"]["sha256"]);
    assert_eq!(intento.cuerpo["version_instrumento"], "instrumento-formal-prueba");
    let ruta = format!("/api/intentos/{}", intento.cuerpo["intento_id"].as_str().unwrap());
    let pre = inicial.peticion("PUT", &format!("{ruta}/encuestas/pre"), Some(&cookie), Some("omitir"),
        Some(json!({"estado":"omitido"}))).await;
    assert_eq!(pre.estado, StatusCode::OK);
    let vista = inicial.consultar("GET", &ruta, Some(&cookie), None).await;
    assert_eq!(vista.cuerpo["ficha"]["datos"][0]["valor"], "148/85 mmHg");
    assert_eq!(vista.cuerpo["ficha"]["fuente"], "Triaje inicial de LR");
    let pulso = inicial.peticion("POST", &format!("{ruta}/mediciones"), Some(&cookie), Some("pulso"),
        Some(json!({"instrumento":"pulso","accion":"toma_completa"}))).await;
    assert_eq!(pulso.estado, StatusCode::CREATED);
    assert_eq!(pulso.cuerpo["valor"]["frecuencia"], 102);
}
