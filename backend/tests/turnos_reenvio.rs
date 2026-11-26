mod fixtures;

use std::{
    path::Path,
    sync::{Arc, atomic::{AtomicUsize, Ordering}},
    time::Duration,
};

use axum::{Json, Router, extract::State, http::{StatusCode, header}, routing::{get, post}};
use fixtures::{Servidor, intento_lr};
use serde_json::{Value, json};
use sircle::{
    agentes::supervisor::Supervisor,
    app::{estado::Estado, router::router},
    modules::entrevista::ollama::{DIGEST_MODELO, MODELO},
    sqlite,
};
use tokio::{net::TcpListener, sync::Notify, time::{sleep, timeout}};

#[derive(Default)]
struct Motor {
    consultas: AtomicUsize,
    recibida: Notify,
    liberar: Notify,
}

async fn chat(State(motor): State<Arc<Motor>>) -> (StatusCode, Json<Value>) {
    let llamada = motor.consultas.fetch_add(1, Ordering::SeqCst);
    if llamada == 0 {
        motor.recibida.notify_one();
        motor.liberar.notified().await;
    }
    if llamada == 1 {
        return (StatusCode::SERVICE_UNAVAILABLE, Json(json!({"error":"fallo de prueba"})));
    }
    (StatusCode::OK, Json(json!({
        "model": MODELO, "done": true, "message": {"content":r#"{"hecho_id":"lr-inicio"}"#},
    })))
}

#[tokio::test]
async fn un_turno_en_curso_se_confirma_una_vez_y_la_clave_fallida_admite_reintento() {
    let mut servidor = Servidor::nuevo();
    let (cookie, _) = servidor.entrar(intento_lr::ESTUDIANTE_LR).await;
    let alta = intento_lr::alta(&servidor, &cookie).await;
    let intento = alta.cuerpo["intento_id"].as_str().unwrap();
    intento_lr::resolver_pre(&servidor, intento);
    let ruta = format!("/api/intentos/{intento}/turnos");
    let motor = Arc::new(Motor::default());
    let ollama = Router::new()
        .route("/api/tags", get(|| async { Json(json!({"models":[{"name": MODELO, "digest": DIGEST_MODELO}]})) }))
        .route("/api/chat", post(chat))
        .with_state(motor.clone());
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let url = format!("http://{}", listener.local_addr().unwrap());
    let http = tokio::spawn(async move { axum::serve(listener, ollama).await.unwrap() });
    let supervisor = Supervisor::iniciar(Path::new(env!("CARGO_BIN_EXE_sircle")), &url).await.unwrap();
    servidor.app = router(Estado::con_base(servidor.db.clone()).unwrap().con_trabajadores(supervisor.clone()));
    let servidor = Arc::new(servidor);

    let texto = json!({"texto":"¿Desde cuándo te cuesta respirar?"});
    let diferente = json!({"texto":"¿Qué te duele?"});
    let inicial = {
        let (servidor, ruta, cookie, texto) = (servidor.clone(), ruta.clone(), cookie.clone(), texto.clone());
        tokio::spawn(async move { servidor.peticion("POST", &ruta, Some(&cookie), Some("mismo-turno"), Some(texto)).await })
    };
    timeout(Duration::from_secs(10), motor.recibida.notified()).await.unwrap();
    let enviar = |clave, texto| servidor.peticion("POST", &ruta, Some(&cookie), Some(clave), Some(texto));
    let pendiente = timeout(Duration::from_secs(5), enviar("mismo-turno", texto.clone())).await.unwrap();
    assert_eq!(pendiente.estado, StatusCode::ACCEPTED);
    assert_eq!(pendiente.cabeceras[header::RETRY_AFTER], "1");
    assert_eq!(pendiente.cuerpo["estado"], "en_curso");
    assert!(pendiente.cuerpo["operacion_id"].as_str().is_some());
    assert_eq!(enviar("mismo-turno", diferente.clone()).await.estado, StatusCode::CONFLICT);
    let medicion = servidor.peticion("POST", &format!("/api/intentos/{intento}/mediciones"), Some(&cookie),
        Some("mismo-turno"), Some(json!({"instrumento":"presion","accion":"toma_completa"}))).await;
    assert_eq!(medicion.estado, StatusCode::CONFLICT);
    let sin_sesion = servidor.peticion("POST", &ruta, None, Some("mismo-turno"), Some(texto.clone())).await;
    assert_eq!(sin_sesion.estado, StatusCode::UNAUTHORIZED);

    inicial.abort();
    assert!(inicial.await.is_err());
    assert_eq!(enviar("mismo-turno", texto.clone()).await.estado, StatusCode::ACCEPTED);
    motor.liberar.notify_one();
    let confirmado = timeout(Duration::from_secs(10), async {
        loop {
            let respuesta = enviar("mismo-turno", texto.clone()).await;
            if respuesta.estado != StatusCode::ACCEPTED {
                break respuesta;
            }
            sleep(Duration::from_millis(20)).await;
        }
    }).await.unwrap();
    assert_eq!(confirmado.estado, StatusCode::OK);
    assert_eq!(confirmado.cuerpo["operacion_id"], pendiente.cuerpo["operacion_id"]);
    assert_eq!(motor.consultas.load(Ordering::SeqCst), 1);

    assert_eq!(enviar("recuperar-turno", texto.clone()).await.estado, StatusCode::SERVICE_UNAVAILABLE);
    assert_eq!(enviar("recuperar-turno", diferente).await.estado, StatusCode::CONFLICT);
    let recuperado = enviar("recuperar-turno", texto.clone()).await;
    assert_eq!(recuperado.estado, StatusCode::CREATED);
    assert_eq!(enviar("recuperar-turno", texto).await.cuerpo, recuperado.cuerpo);
    assert_eq!(motor.consultas.load(Ordering::SeqCst), 3);

    let db = sqlite::abrir_existente(&servidor.db).unwrap();
    let cuenta = |sql| db.query_row(sql, [], |fila| fila.get::<_, usize>(0)).unwrap();
    assert_eq!(cuenta("SELECT count(*) FROM turnos"), 2);
    assert_eq!(cuenta("SELECT count(*) FROM trazas_turno"), 2);
    supervisor.apagar().await;
    http.abort();
}
