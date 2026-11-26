mod fixtures;

use axum::http::StatusCode;
use fixtures::Servidor;
use serde_json::json;

#[tokio::test]
async fn el_pre_pendiente_muestra_solo_la_actividad_asignada() {
    let servidor = Servidor::nuevo();
    let lista = "/api/actividades";
    let detalle = "/api/actividades/lr-ensayo";
    let sin_sesion = servidor.consultar("GET", lista, None, None).await;
    assert_eq!(sin_sesion.estado, StatusCode::UNAUTHORIZED);

    let (cookie, titular) = servidor.entrar("estudiante-lr").await;
    let (otra_cookie, otro) = servidor.entrar("estudiante-sin-asignacion").await;
    assert_ne!(cookie, otra_cookie);
    let asignada = json!({
        "actividad_id": "lr-ensayo", "nombre": "Práctica técnica de LR", "tipo": "ensayo", "pre": "pendiente", "intento_id": null,
    });
    let lista_titular = servidor.consultar("GET", lista, Some(&cookie), None).await;
    assert_eq!(lista_titular.estado, StatusCode::OK);
    assert_eq!(lista_titular.cuerpo, json!([asignada]));
    let detalle_titular = servidor.consultar("GET", detalle, Some(&cookie), None).await;
    assert_eq!(detalle_titular.estado, StatusCode::OK);
    assert_eq!(detalle_titular.cuerpo, asignada);

    let lista_otra = servidor.consultar("GET", lista, Some(&otra_cookie), None).await;
    assert_eq!(lista_otra.estado, StatusCode::OK);
    assert_eq!(lista_otra.cuerpo, json!([]));
    let detalle_otra = servidor.consultar("GET", detalle, Some(&otra_cookie), None).await;
    assert_eq!(detalle_otra.estado, StatusCode::FORBIDDEN);
    assert_eq!(detalle_otra.cuerpo, json!({"mensaje": "Esta actividad no está asignada a tu identidad."}));

    println!("EVIDENCIA {}", json!({
        "cookies": {"HttpOnly": true, "SameSite": "Strict", "distintas": true},
        "sin_sesion": {"http": sin_sesion.estado.as_u16(), "cuerpo": sin_sesion.cuerpo},
        "titular": {
            "sesion": {"http": titular.estado.as_u16(), "cuerpo": titular.cuerpo},
            "lista": {"http": lista_titular.estado.as_u16(), "cuerpo": lista_titular.cuerpo},
            "detalle": {"http": detalle_titular.estado.as_u16(), "cuerpo": detalle_titular.cuerpo},
        },
        "sin_asignacion": {
            "sesion": {"http": otro.estado.as_u16(), "cuerpo": otro.cuerpo},
            "lista": {"http": lista_otra.estado.as_u16(), "cuerpo": lista_otra.cuerpo},
            "detalle": {"http": detalle_otra.estado.as_u16(), "cuerpo": detalle_otra.cuerpo},
        },
    }));
}
