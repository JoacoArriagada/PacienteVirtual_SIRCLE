mod fixtures;

use axum::http::StatusCode;
use fixtures::{Servidor, intento_lr};
use serde_json::json;
use sircle::app::{estado::Estado, router::router};

#[tokio::test]
async fn reabre_y_reenvia_el_mismo_intento_tecnico_con_pre_resuelto() {
    let mut servidor = Servidor::nuevo();
    let (cookie, _) = servidor.entrar(intento_lr::ESTUDIANTE_LR).await;
    let alta = intento_lr::alta(&servidor, &cookie).await;
    assert_eq!(alta.estado, StatusCode::CREATED);
    assert_eq!(alta.cuerpo["pre"], "pendiente");
    let id = alta.cuerpo["intento_id"].as_str().unwrap();
    let ruta = format!("/api/intentos/{id}");
    let antes_del_pre = servidor.consultar("GET", &ruta, Some(&cookie), None).await;
    for reservado in ["introduccion", "ficha", "indicaciones", "ayuda", "sugerencias", "respiraciones_simuladas", "turnos", "mediciones"] {
        assert!(antes_del_pre.cuerpo.get(reservado).is_none());
    }
    intento_lr::resolver_pre(&servidor, id);
    intento_lr::resolver_pre(&servidor, id);

    let vista = servidor.consultar("GET", &ruta, Some(&cookie), None).await;
    assert_eq!(vista.estado, StatusCode::OK);
    assert_eq!(vista.cuerpo["tipo"], "ensayo");
    assert_eq!(vista.cuerpo["caso"], "P01");
    assert_eq!(vista.cuerpo["estudiante_id"], intento_lr::ESTUDIANTE_LR);
    assert_eq!(vista.cuerpo["actividad_id"], "lr-ensayo");
    assert_eq!(vista.cuerpo["pre"], "omitido");
    assert_eq!(vista.cuerpo["fase"], "inicial");
    assert_eq!(vista.cuerpo["estado"], "atencion");
    assert_eq!(vista.cuerpo["introduccion"]["paciente"], "LR");
    assert_eq!(vista.cuerpo["ficha"]["datos"][0]["valor"], "148/85 mmHg");
    assert_eq!(vista.cuerpo["ayuda"]["origen"], "docente");
    assert!(!vista.cuerpo["indicaciones"].as_array().unwrap().is_empty());
    assert_eq!(vista.cuerpo["revision"], 1);
    assert!(vista.cuerpo["version_caso"].as_str().unwrap().starts_with("ensayo-originales-"));
    assert_eq!(vista.cuerpo["version_rubrica"], "rubrica-ensayo-v1");
    assert_eq!(vista.cuerpo["version_instrumento"], "instrumento-ensayo-v1");

    let salida = servidor.consultar("DELETE", "/api/sesiones", Some(&cookie), None).await;
    assert_eq!(salida.estado, StatusCode::NO_CONTENT);
    servidor.app = router(Estado::con_base(servidor.db.clone()).unwrap());
    let expirada = servidor.consultar("GET", &ruta, Some(&cookie), None).await;
    assert_eq!(expirada.estado, StatusCode::UNAUTHORIZED);
    let (cookie_nueva, _) = servidor.entrar(intento_lr::ESTUDIANTE_LR).await;
    let lista = servidor.consultar("GET", "/api/actividades", Some(&cookie_nueva), None).await;
    assert_eq!(lista.cuerpo[0]["intento_id"], id);
    assert_eq!(lista.cuerpo[0]["pre"], "omitido");
    let recuperado = servidor.consultar("GET", &ruta, Some(&cookie_nueva), None).await;
    let reenvio = intento_lr::alta(&servidor, &cookie_nueva).await;
    assert_eq!(recuperado.estado, StatusCode::OK);
    assert_eq!(recuperado.cuerpo, vista.cuerpo);
    assert_eq!(reenvio.estado, StatusCode::OK);
    let mut vista_base = vista.cuerpo.clone();
    assert_eq!(vista_base.as_object_mut().unwrap().remove("mediciones"), Some(json!([])));
    assert_eq!(vista_base.as_object_mut().unwrap().remove("turnos"), Some(json!([])));
    assert_eq!(vista_base.as_object_mut().unwrap().remove("exploraciones"), Some(json!([])));
    assert_eq!(vista_base.as_object_mut().unwrap().remove("ordenes"), Some(json!([])));
    assert_eq!(vista_base.as_object_mut().unwrap().remove("apuntes"), Some(json!([])));
    let disponibles = vista_base.as_object_mut().unwrap().remove("evidencias").unwrap();
    assert_eq!(disponibles.as_array().unwrap().len(), 1);
    assert_eq!(disponibles[0]["categoria"], "orientacion");
    for seccion in ["introduccion", "ficha", "indicaciones", "ayuda", "sugerencias", "respiraciones_simuladas"] {
        vista_base.as_object_mut().unwrap().remove(seccion).unwrap();
    }
    assert_eq!(reenvio.cuerpo, vista_base);

    let (otra, _) = servidor.entrar("estudiante-sin-asignacion").await;
    let ajena = servidor.consultar("GET", &ruta, Some(&otra), None).await;
    assert_eq!(ajena.estado, StatusCode::FORBIDDEN);
    let otro_alta = intento_lr::alta(&servidor, &otra).await;
    assert_eq!(otro_alta.estado, StatusCode::FORBIDDEN);

    println!("EVIDENCIA {}", json!({
        "alta": {"http": alta.estado.as_u16(), "clave": intento_lr::ALTA_CLAVE, "cuerpo": alta.cuerpo},
        "antes_de_salir": vista.cuerpo,
        "tras_reabrir": recuperado.cuerpo,
        "tras_reenviar": {"http": reenvio.estado.as_u16(), "cuerpo": reenvio.cuerpo},
        "actividad_reabierta": lista.cuerpo[0],
        "sesion_cerrada_http": expirada.estado.as_u16(),
        "vista_ajena_http": ajena.estado.as_u16(),
    }));
}
