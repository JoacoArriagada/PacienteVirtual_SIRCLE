mod fixtures;

use std::{path::Path, sync::{Arc, Mutex}, time::Duration};

use axum::{Json, Router, extract::State, http::{HeaderMap, StatusCode}, routing::{get, post}};
use fixtures::{Servidor, intento_lr, versiones_rubrica::{self, V1}};
use serde_json::{Value, json};
use sircle::{
    agentes::supervisor::Supervisor, app::estado::Estado,
    modules::{entrevista::ollama::{DIGEST_MODELO, MODELO}, evaluacion::{rubricas, trabajos}}, sqlite,
};
use tokio::{net::TcpListener, sync::Notify, time::{sleep, timeout}};

#[derive(Default)]
struct Control {
    primera_llamada: Notify,
    liberar: Notify,
    operaciones: Mutex<Vec<String>>,
}

async fn inventario() -> Json<Value> {
    Json(json!({"models":[{"name":MODELO,"digest":DIGEST_MODELO}]}))
}

async fn evaluar(State(control): State<Arc<Control>>, headers: HeaderMap, Json(datos): Json<Value>) -> Json<Value> {
    let numero = {
        let mut ops = control.operaciones.lock().unwrap();
        ops.push(headers["x-sircle-operacion"].to_str().unwrap().to_owned());
        ops.len()
    };
    if numero == 1 {
        control.primera_llamada.notify_one();
        control.liberar.notified().await;
    }
    let entrada: Value = serde_json::from_str(datos["messages"][1]["content"].as_str().unwrap()).unwrap();
    let resultados: serde_json::Map<String, Value> = entrada["criterios"].as_array().unwrap().iter().map(|c| {
        let id = c["id"].as_str().unwrap();
        (id.to_owned(), json!({"a":format!("Ensayo de toma {numero}"),"j":"omision","e":[],
            "x":"El registro de esta prueba está vacío.","m":"Realiza y registra la acción del criterio."}))
    }).collect();
    Json(json!({"model":MODELO,"done":true,"message":{"content":json!({"resultados":resultados}).to_string()}}))
}

#[tokio::test]
async fn descarta_resultado_tardio_de_toma_vencida_y_confirma_una_evaluacion() {
    let servidor = Servidor::nuevo();
    let mut db = sqlite::abrir_existente(&servidor.db).unwrap();
    let v1 = versiones_rubrica::cargar(&mut db, V1);
    rubricas::habilitar_ensayo(&db, v1.id).unwrap();
    let intento = versiones_rubrica::abrir(&servidor, intento_lr::ESTUDIANTE_LR, v1).await;
    intento_lr::resolver_pre(&servidor, &intento.id);
    let acuse = servidor.peticion("POST", &format!("/api/intentos/{}/entrega", intento.id), Some(&intento.cookie),
        Some("e22-entrega"), Some(json!({"revision_esperada":1}))).await;
    assert_eq!(acuse.estado, StatusCode::CREATED);
    let entrega = acuse.cuerpo["entrega_id"].as_str().unwrap();
    let control = Arc::new(Control::default());
    let motor = Router::new().route("/api/tags", get(inventario)).route("/api/chat", post(evaluar)).with_state(control.clone());
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let url = format!("http://{}", listener.local_addr().unwrap());
    let http = tokio::spawn(async move { axum::serve(listener, motor).await.unwrap() });
    let supervisor = Supervisor::iniciar(Path::new(env!("CARGO_BIN_EXE_sircle")), &url).await.unwrap();
    let app = Estado::con_base(servidor.db.clone()).unwrap().con_trabajadores(supervisor.clone());
    let primera_app = app.clone();
    let primera = tokio::spawn(async move { trabajos::procesar_siguiente(&primera_app).await.unwrap() });
    timeout(Duration::from_secs(15), control.primera_llamada.notified()).await.unwrap();
    let vigente = || db.query_row(
        "SELECT toma_id,vence FROM trabajos_evaluacion WHERE entrega_id=?1",
        [entrega], |r| Ok((r.get::<_, String>(0)?,r.get::<_, i64>(1)?)),
    ).unwrap();
    let (toma_anterior, vencimiento) = vigente();
    db.execute("UPDATE trabajos_evaluacion SET vence=unixepoch()-1 WHERE entrega_id=?1 AND toma_id=?2",
        rusqlite::params![entrega,toma_anterior]).unwrap();
    let segunda_app = app.clone();
    let segunda = tokio::spawn(async move { trabajos::procesar_siguiente(&segunda_app).await.unwrap() });
    let (toma_nueva, segundo_vencimiento) = timeout(Duration::from_secs(5), async {
        loop {
            let nueva = vigente();
            if nueva.0 != toma_anterior { break nueva; }
            sleep(Duration::from_millis(20)).await;
        }
    }).await.unwrap();
    assert_eq!(db.query_row("SELECT count(*) FROM evaluaciones", [], |r| r.get::<_, i64>(0)).unwrap(), 0);
    control.liberar.notify_one();
    assert!(!timeout(Duration::from_secs(10), primera).await.unwrap().unwrap());
    assert!(timeout(Duration::from_secs(10), segunda).await.unwrap().unwrap());
    let (evaluacion, operacion, borrador, trabajo): (String, String, String, String) = db.query_row(
        "SELECT v.id,v.operacion_id,v.borrador,t.estado FROM evaluaciones v JOIN trabajos_evaluacion t ON t.entrega_id=v.entrega_id WHERE v.entrega_id=?1",
        [entrega], |r| Ok((r.get(0)?,r.get(1)?,r.get(2)?,r.get(3)?)),
    ).unwrap();
    assert_eq!(operacion, toma_nueva);
    assert!(borrador.contains("Ensayo de toma 2") && !borrador.contains("Ensayo de toma 1"));
    assert_eq!(trabajo, "terminado");
    assert_eq!(db.query_row("SELECT count(*) FROM evaluaciones", [], |r| r.get::<_, i64>(0)).unwrap(), 1);
    assert_eq!(db.query_row("SELECT count(*) FROM trazas_evaluacion", [], |r| r.get::<_, i64>(0)).unwrap(), 1);
    assert!(!trabajos::procesar_siguiente(&app).await.unwrap());
    assert_eq!(*control.operaciones.lock().unwrap(), [toma_anterior.clone(), toma_nueva.clone()]);
    supervisor.apagar().await;
    http.abort();
    println!("EVIDENCIA {}", json!({"entrega_id":entrega,"toma_vencida":toma_anterior,"vencimiento_original":vencimiento,
        "toma_recuperada":toma_nueva,"vencimiento_nuevo":segundo_vencimiento,"evaluacion_id":evaluacion,
        "evaluaciones":1,"trazas":1,"resultado_tardio":"descartado","trabajo":trabajo}));
}
