mod fixtures;

use axum::{
    body::{Body, to_bytes},
    http::{Request, StatusCode, header},
};
use serde_json::{Value, json};
use sircle::app::{estado::Estado, router::router};
use tower::ServiceExt;

#[tokio::test]
async fn el_router_responde_json_sin_frontend_ni_inferencia() {
    let peticion = Request::get("/api/estado").body(Body::empty()).unwrap();
    let respuesta = router(Estado::default()).oneshot(peticion).await.unwrap();

    assert_eq!(respuesta.status(), StatusCode::OK);
    assert_eq!(respuesta.headers()[header::CONTENT_TYPE], "application/json");
    let cuerpo = to_bytes(respuesta.into_body(), 1024).await.unwrap();
    let json: Value = serde_json::from_slice(&cuerpo).unwrap();
    assert_eq!(
        json,
        json!({
            "servicio": "sircle",
            "base": { "disponible": false },
            "modelo": { "disponible": false },
        }),
    );
}

#[tokio::test]
async fn estado_detecta_la_perdida_y_recuperacion_de_sqlite_sin_inferencia() {
    let servidor = fixtures::Servidor::nuevo();
    let respaldo = servidor.db.with_extension("respaldo.sqlite");
    for (esperado, cambio) in [
        (true, None),
        (false, Some((&servidor.db, &respaldo))),
        (true, Some((&respaldo, &servidor.db))),
    ] {
        if let Some((origen, destino)) = cambio {
            std::fs::rename(origen, destino).unwrap();
        }
        let respuesta = servidor.consultar("GET", "/api/estado", None, None).await;
        assert_eq!(respuesta.estado, StatusCode::OK);
        assert_eq!(respuesta.cuerpo["base"]["disponible"], esperado);
        assert_eq!(respuesta.cuerpo["modelo"]["disponible"], false);
        assert_eq!(servidor.db.is_file(), esperado);
    }
}
