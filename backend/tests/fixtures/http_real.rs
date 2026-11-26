use std::time::Duration;

use reqwest::{Client, StatusCode, header};
use serde_json::{Value, json};

pub const PREGUNTA: &str = "¿Hace cuánto empezaste a sentir que te faltaba el aire?";

pub async fn sesion() -> (Client, String, String) {
    let cliente = Client::builder().no_proxy().timeout(Duration::from_secs(205)).build().unwrap();
    let origen = std::env::var("SIRCLE_URL").unwrap_or_else(|_| "http://127.0.0.1:8081".into());
    let respuesta = cliente.post(format!("{origen}/api/sesiones"))
        .json(&json!({"identidad": "estudiante-lr"})).send().await.unwrap();
    assert_eq!(respuesta.status(), StatusCode::CREATED);
    let cookie = respuesta.headers()[header::SET_COOKIE].to_str().unwrap()
        .split(';').next().unwrap().to_owned();
    let actividades = recuperar(&cliente, &format!("{origen}/api/actividades"), &cookie).await;
    let actividad = actividades.as_array().unwrap().iter()
        .find(|actividad| actividad["actividad_id"] == "lr-ensayo").unwrap();
    let id = actividad["intento_id"].as_str().expect("Conservar el intento de T008");
    let ruta = format!("{origen}/api/intentos/{id}");
    (cliente, cookie, ruta)
}

pub async fn recuperar(cliente: &Client, ruta: &str, cookie: &str) -> Value {
    cliente.get(ruta).header(header::COOKIE, cookie).send().await.unwrap()
        .error_for_status().unwrap().json().await.unwrap()
}

pub async fn preguntar(cliente: &Client, ruta: &str, cookie: &str, clave: &str) -> (StatusCode, Value) {
    let respuesta = cliente.post(format!("{ruta}/turnos"))
        .header(header::COOKIE, cookie)
        .header("Idempotency-Key", clave)
        .json(&json!({"texto": PREGUNTA}))
        .send().await.unwrap();
    (respuesta.status(), respuesta.json().await.unwrap())
}
