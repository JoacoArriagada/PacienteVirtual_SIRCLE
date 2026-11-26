mod fixtures;

use axum::http::StatusCode;
use fixtures::{Servidor, intento_lr};
use serde_json::json;
use sircle::sqlite;

#[tokio::test]
async fn guarda_solo_una_toma_completa_en_fase_autorizada() {
    let servidor = Servidor::nuevo();
    let (cookie, _) = servidor.entrar(intento_lr::ESTUDIANTE_LR).await;
    let alta = intento_lr::alta(&servidor, &cookie).await;
    let id = alta.cuerpo["intento_id"].as_str().unwrap();
    let ruta = format!("/api/intentos/{id}");
    let mediciones = format!("{ruta}/mediciones");
    let completa = json!({"instrumento": "presion", "accion": "toma_completa"});
    let incompleta = json!({"instrumento": "presion", "accion": "toma_incompleta"});
    let pendiente = servidor.peticion("POST", &mediciones, Some(&cookie), Some("pre-pendiente"), Some(completa.clone())).await;
    assert_eq!(pendiente.estado, StatusCode::CONFLICT);
    let historico = servidor.consultar("GET", &mediciones, Some(&cookie), None).await;
    assert_eq!(historico.cuerpo, json!([]));

    intento_lr::resolver_pre(&servidor, id);
    let toma = servidor.peticion("POST", &mediciones, Some(&cookie), Some("presion-1"), Some(completa.clone())).await;
    assert_eq!(toma.estado, StatusCode::CREATED);
    assert_eq!(toma.cuerpo["intento_id"], id);
    assert_eq!(toma.cuerpo["instrumento"], "presion");
    assert_eq!(toma.cuerpo["valor"], json!({"sistolica": 148, "diastolica": 85}));
    assert_eq!(toma.cuerpo["unidad"], "mmHg");
    assert_eq!(toma.cuerpo["origen"], "Brazalete simulado, triaje inicial de LR");
    assert_eq!(toma.cuerpo["fase"], "inicial");
    assert_eq!(toma.cuerpo["en_ficha"], false);
    assert_eq!(toma.cuerpo["revision"], 2);
    assert!(toma.cuerpo["momento"].as_str().unwrap().ends_with('Z'));
    assert_eq!(toma.cuerpo["medicion_id"].as_str().unwrap().len(), 32);

    let rechazada = servidor.peticion("POST", &mediciones, Some(&cookie), Some("incompleta"), Some(incompleta.clone())).await;
    assert_eq!(rechazada.estado, StatusCode::UNPROCESSABLE_ENTITY);
    sqlite::abrir_existente(&servidor.db).unwrap()
        .execute("UPDATE intentos SET fase='posterior' WHERE id=?1", [id]).unwrap();
    let fuera = servidor.peticion("POST", &mediciones, Some(&cookie), Some("otra-fase"), Some(completa.clone())).await;
    assert_eq!(fuera.estado, StatusCode::CONFLICT);

    let historico = servidor.consultar("GET", &mediciones, Some(&cookie), None).await;
    assert_eq!(historico.estado, StatusCode::OK);
    assert_eq!(historico.cuerpo, json!([toma.cuerpo]));
    let vista = servidor.consultar("GET", &ruta, Some(&cookie), None).await;
    assert_eq!(vista.estado, StatusCode::OK);
    assert_eq!(vista.cuerpo["mediciones"], historico.cuerpo);
    println!("EVIDENCIA {}", json!({
        "intento_id": id,
        "toma_completa": {"clave": "presion-1", "peticion": completa, "http": toma.estado.as_u16(), "acuse": toma.cuerpo},
        "pre_pendiente_http": pendiente.estado.as_u16(),
        "toma_incompleta": {"peticion": incompleta, "http": rechazada.estado.as_u16()},
        "fase_posterior_http": fuera.estado.as_u16(),
        "historico": historico.cuerpo,
        "vista_del_intento": vista.cuerpo,
    }));
}

#[tokio::test]
async fn reenvios_simultaneos_conservan_un_acuse_y_una_toma_nueva_agrega_otra() {
    let servidor = Servidor::nuevo();
    let (cookie, _) = servidor.entrar(intento_lr::ESTUDIANTE_LR).await;
    let alta = intento_lr::alta(&servidor, &cookie).await;
    let id = alta.cuerpo["intento_id"].as_str().unwrap();
    intento_lr::resolver_pre(&servidor, id);
    let ruta = format!("/api/intentos/{id}/mediciones");
    let completa = json!({"instrumento": "presion", "accion": "toma_completa"});
    let enviar = |clave| servidor.peticion("POST", &ruta, Some(&cookie), Some(clave), Some(completa.clone()));
    let historico = || servidor.consultar("GET", &ruta, Some(&cookie), None);
    assert_eq!(historico().await.cuerpo, json!([]));

    let (a, b) = tokio::join!(enviar("misma-toma"), enviar("misma-toma"));
    let (primero, repetido) = if a.estado == StatusCode::CREATED { (a, b) } else { (b, a) };
    assert_eq!(primero.estado, StatusCode::CREATED);
    assert_eq!(repetido.estado, StatusCode::OK);
    assert_eq!(primero.cuerpo, repetido.cuerpo);
    let una = historico().await;
    assert_eq!(una.cuerpo, json!([primero.cuerpo]));

    let nueva = enviar("nueva-toma").await;
    assert_eq!(nueva.estado, StatusCode::CREATED);
    assert_ne!(nueva.cuerpo["medicion_id"], primero.cuerpo["medicion_id"]);
    let ocupada = servidor.peticion("POST", &ruta, Some(&cookie), Some("misma-toma"),
        Some(json!({"instrumento": "presion", "accion": "toma_incompleta"}))).await;
    assert_eq!(ocupada.estado, StatusCode::CONFLICT);

    sqlite::abrir_existente(&servidor.db).unwrap()
        .execute("UPDATE intentos SET fase='posterior' WHERE id=?1", [id]).unwrap();
    let tardia = enviar("misma-toma").await;
    assert_eq!(tardia.estado, StatusCode::OK);
    assert_eq!(tardia.cuerpo, primero.cuerpo);
    let dos = historico().await;
    assert_eq!(dos.cuerpo, json!([primero.cuerpo, nueva.cuerpo]));
    println!("EVIDENCIA {}", json!({
        "intento_id": id,
        "lecturas_iniciales": 0,
        "original": {"clave": "misma-toma", "http": primero.estado.as_u16(), "acuse": primero.cuerpo},
        "reenvio": {"clave": "misma-toma", "http": repetido.estado.as_u16(), "acuse": repetido.cuerpo},
        "lecturas_tras_reenviar": una.cuerpo.as_array().unwrap().len(),
        "nueva": {"clave": "nueva-toma", "http": nueva.estado.as_u16(), "acuse": nueva.cuerpo},
        "misma_clave_otro_contenido": {"http": ocupada.estado.as_u16(), "cuerpo": ocupada.cuerpo},
        "reenvio_tras_cambio_fase": {"http": tardia.estado.as_u16(), "acuse": tardia.cuerpo},
        "lecturas_finales": dos.cuerpo.as_array().unwrap().len(),
    }));
}
