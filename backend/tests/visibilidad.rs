mod fixtures;

use axum::http::StatusCode;
use fixtures::{Servidor, intento_lr};
use serde_json::{Value, json};
use sircle::sqlite;

fn sin_material_reservado(vista: &Value, testigos: &Value) {
    let json = vista.to_string();
    for (tipo, texto) in testigos.as_object().unwrap() {
        assert!(!json.contains(texto.as_str().unwrap()), "material reservado en la sesión: {tipo}");
    }
}

#[tokio::test]
async fn proyecta_solo_registros_de_la_fase_y_el_propietario() {
    let servidor = Servidor::nuevo();
    let testigos: Value = serde_json::from_str(include_str!("../../frontend/tests/fixtures/reserva-publica.json")).unwrap();
    let (cookie, _) = servidor.entrar(intento_lr::ESTUDIANTE_LR).await;
    let alta = intento_lr::alta(&servidor, &cookie).await;
    let id = alta.cuerpo["intento_id"].as_str().unwrap();
    let ruta = format!("/api/intentos/{id}");
    let antes = servidor.consultar("GET", &ruta, Some(&cookie), None).await;
    assert_eq!(antes.estado, StatusCode::OK);
    for campo in ["introduccion", "ficha", "turnos", "mediciones", "apuntes", "evidencias", "ayuda", "borrador", "ordenes"] {
        assert!(antes.cuerpo.get(campo).is_none(), "sección anterior al pre: {campo}");
    }
    sin_material_reservado(&antes.cuerpo, &testigos);
    intento_lr::resolver_pre(&servidor, id);
    let turno = servidor.peticion("POST", &format!("{ruta}/turnos"), Some(&cookie), Some("t038-turno"),
        Some(json!({"sugerencia_id":"inicio","texto":"¿Desde cuándo comenzó?"}))).await;
    assert_eq!(turno.estado, StatusCode::CREATED);
    assert!(turno.cuerpo["respuesta"].as_str().unwrap().contains("hace dos días"));
    sin_material_reservado(&turno.cuerpo, &testigos);
    let archivo_traza = format!("{ruta}/turnos/{}/traza", turno.cuerpo["turno_id"].as_str().unwrap());
    let estudiante = servidor.consultar("GET", &archivo_traza, Some(&cookie), None).await;
    assert_eq!(estudiante.estado, StatusCode::FORBIDDEN);
    sin_material_reservado(&estudiante.cuerpo, &testigos);
    let (docente, _) = servidor.entrar("docente-demo").await;
    let observada = servidor.consultar("GET", &archivo_traza, Some(&docente), None).await;
    assert_eq!(observada.estado, StatusCode::OK);
    assert!(observada.cuerpo["hecho"]["pasaje"].as_str().unwrap().contains(testigos["traza"].as_str().unwrap()));

    let atencion = servidor.consultar("GET", &ruta, Some(&cookie), None).await;
    assert_eq!(atencion.cuerpo["introduccion"]["paciente"], "LR");
    assert_eq!(atencion.cuerpo["turnos"], json!([turno.cuerpo]));
    sin_material_reservado(&atencion.cuerpo, &testigos);
    let reservado = servidor.consultar("GET", &format!("{ruta}/ordenes/ecg-inicial"), Some(&cookie), None).await;
    assert_eq!(reservado.estado, StatusCode::NOT_FOUND);
    assert!(!reservado.cuerpo.to_string().contains("taquicardia sinusal"));

    let db = sqlite::abrir_existente(&servidor.db).unwrap();
    db.execute("UPDATE intentos SET estado='entregado' WHERE id=?1", [id]).unwrap();
    let (ajena, _) = servidor.entrar("estudiante-sin-asignacion").await;
    let ajeno = servidor.consultar("GET", &ruta, Some(&ajena), None).await;
    assert_eq!(ajeno.estado, StatusCode::FORBIDDEN);
    sin_material_reservado(&ajeno.cuerpo, &testigos);
    let entregada = servidor.consultar("GET", &ruta, Some(&cookie), None).await;
    assert_eq!(entregada.cuerpo["estado"], "entregado");
    sin_material_reservado(&entregada.cuerpo, &testigos);
    println!("T038 · {}: pre 200, atención 200, traza estudiante 403, traza docente 200, ECG reservado 404, ajena 403, antes de publicar 200", id);
}
