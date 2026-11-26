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
