mod fixtures;

use axum::http::StatusCode;
use fixtures::{Servidor, intento_lr};
use rusqlite::params;
use serde_json::json;
use sircle::{
    agentes::dan_sparks::{enrutamiento, entrevista, perfil},
    modules::{examenes::medios, intentos::intento::{self, Intento}},
    shared::{contexto::{self, ContextoEntrevista}, ficha_aprobada},
    sqlite,
};

const INICIO: &str = "p03-inicio";

async fn iniciar(servidor: &Servidor) -> (String, Intento) {
    let db = sqlite::abrir_existente(&servidor.db).unwrap();
    db.execute("UPDATE actividades SET habilitada=1 WHERE id=?1", [perfil::ACTIVIDAD]).unwrap();
    db.execute("INSERT INTO asignaciones VALUES (?1,?2,'docente-demo')",
        params![perfil::ACTIVIDAD, intento_lr::ESTUDIANTE_LR]).unwrap();
    let (cookie, _) = servidor.entrar(intento_lr::ESTUDIANTE_LR).await;
    let alta = servidor.peticion("POST", "/api/intentos", Some(&cookie), Some("alta-dan"),
        Some(json!({"actividad_id": perfil::ACTIVIDAD}))).await;
    assert_eq!(alta.estado, StatusCode::CREATED);
    let id = alta.cuerpo["intento_id"].as_str().unwrap();
    let pre = intento::recuperar(&db, id, intento_lr::ESTUDIANTE_LR).unwrap();
    assert!(medios::ecg_referido(&db, &pre).unwrap().is_none());
    intento_lr::resolver_pre(servidor, id);
    (cookie, intento::recuperar(&db, id, intento_lr::ESTUDIANTE_LR).unwrap())
}

fn recuperar(servidor: &Servidor, intento: &Intento) -> ContextoEntrevista {
    let db = sqlite::abrir_existente(&servidor.db).unwrap();
    let (agente, hablante) = enrutamiento::destino(intento).unwrap();
    contexto::entrevista(&db, intento, agente, hablante).unwrap()
}

async fn abrir_lr(servidor: &Servidor, cookie: &str) -> Intento {
    let alta = intento_lr::alta(servidor, cookie).await;
    let id = alta.cuerpo["intento_id"].as_str().unwrap();
    intento_lr::resolver_pre(servidor, id);
    let db = sqlite::abrir_existente(&servidor.db).unwrap();
    intento::recuperar(&db, id, intento_lr::ESTUDIANTE_LR).unwrap()
}

#[tokio::test]
async fn mantiene_identidad_despues_de_lr() {
    let servidor = Servidor::nuevo();
    let (cookie, dan) = iniciar(&servidor).await;
    let lr = abrir_lr(&servidor, &cookie).await;
    let previo = servidor.peticion("POST", &format!("/api/intentos/{}/turnos", lr.intento_id),
        Some(&cookie), Some("literal-lr"), Some(json!({"texto":"¿Desde cuándo comenzó?","sugerencia_id":"inicio"}))).await;
    assert_eq!(previo.estado, StatusCode::CREATED);
    let contexto = recuperar(&servidor, &dan);
    assert!(contexto.memoria.is_empty());
    assert_eq!(entrevista::elegir(&contexto, INICIO).unwrap().hablante, perfil::HABLANTE);
    assert_ne!(contexto.intento_id, lr.intento_id);
}

#[tokio::test]
async fn recupera_cuatro_horas_de_p03() {
    let servidor = Servidor::nuevo();
    let (_, dan) = iniciar(&servidor).await;
    let contexto = recuperar(&servidor, &dan);
    let hecho = entrevista::elegir(&contexto, INICIO).unwrap();
    assert!(hecho.respuesta.starts_with("Me empezó"));
    assert!(hecho.respuesta.contains("hace cuatro horas") && hecho.respuesta.contains("jardín"));
    assert_eq!(hecho.fuente, perfil::FUENTE_ACTOR);
    assert_eq!(hecho.version, dan.version_caso);
}

#[tokio::test]
async fn protege_informe_del_controlador() {
    let servidor = Servidor::nuevo();
    let (cookie, dan) = iniciar(&servidor).await;
    let ruta = format!("/api/intentos/{}/ordenes/ecg-inicial", dan.intento_id);
    let consulta = servidor.consultar("GET", &ruta, Some(&cookie), None).await;
    assert_eq!(consulta.estado, StatusCode::NOT_FOUND);
    assert!(!consulta.cuerpo.to_string().contains(perfil::REFERENCIA_ECG));
}

#[tokio::test]
async fn excluye_contexto_de_p01() {
    let servidor = Servidor::nuevo();
    let (cookie, _) = iniciar(&servidor).await;
    let lr = abrir_lr(&servidor, &cookie).await;
    assert_eq!(enrutamiento::destino(&lr).unwrap_err().estado, StatusCode::FORBIDDEN);
    let db = sqlite::abrir_existente(&servidor.db).unwrap();
    assert!(medios::ecg_referido(&db, &lr).unwrap().is_none());
}

#[tokio::test]
async fn rechaza_hecho_ajeno_o_imagen_ausente() {
    let servidor = Servidor::nuevo();
    let (cookie, dan) = iniciar(&servidor).await;
    let lr = abrir_lr(&servidor, &cookie).await;
    let db = sqlite::abrir_existente(&servidor.db).unwrap();
    let mut propio = recuperar(&servidor, &dan);
    propio.candidatos.extend(sircle::modules::entrevista::hechos::candidatos_lr(&db, &lr).unwrap());
    assert_eq!(entrevista::elegir(&propio, "lr-inicio").unwrap_err().estado, StatusCode::SERVICE_UNAVAILABLE);
    assert!(entrevista::elegir(&propio, perfil::REFERENCIA_ECG).is_err());
    assert_eq!(medios::ecg_referido(&db, &dan).unwrap().unwrap().disponibilidad, "indisponible");
}

#[tokio::test]
async fn sugerencia_literal_respeta_p03() {
    let servidor = Servidor::nuevo();
    let (cookie, dan) = iniciar(&servidor).await;
    let lr = abrir_lr(&servidor, &cookie).await;
    let db = sqlite::abrir_existente(&servidor.db).unwrap();
    let hechos_lr = sircle::modules::entrevista::hechos::candidatos_lr(&db, &lr).unwrap();
    let hechos_dan = recuperar(&servidor, &dan).candidatos;
    let opcion = ficha_aprobada::literal(Some("inicio"), "¿Desde cuándo comenzó?", &hechos_lr).unwrap();
    assert_eq!(opcion.hecho_id, "lr-inicio");
    let opcion = ficha_aprobada::literal(Some("inicio"), "¿Desde cuándo comenzó?", &hechos_dan).unwrap();
    assert_eq!(opcion.hecho_id, INICIO);
}

#[tokio::test]
async fn separa_relato_e_informe() {
    let servidor = Servidor::nuevo();
    let (_, dan) = iniciar(&servidor).await;
    let db = sqlite::abrir_existente(&servidor.db).unwrap();
    let contexto = recuperar(&servidor, &dan);
    let hecho = entrevista::elegir(&contexto, INICIO).unwrap();
    let medio = medios::ecg_referido(&db, &dan).unwrap().unwrap();
    assert_eq!(medio.fuente, perfil::FUENTE_ECG);
    assert_eq!(medio.medio, perfil::REFERENCIA_ECG);
    assert_ne!(medio.revision_fuente_id, hecho.revision_fuente_id);
    assert_eq!(medio.disponibilidad, "indisponible");
    println!("EVIDENCIA {}", json!({"intento": dan.intento_id, "cronologia": hecho, "medio_ecg": medio}));
}

#[tokio::test]
async fn rastrear_cronologia_de_dan() {
    let servidor = Servidor::nuevo();
    let (_, dan) = iniciar(&servidor).await;
    let contexto = recuperar(&servidor, &dan);
    let hecho = entrevista::elegir(&contexto, INICIO).unwrap();
    let db = sqlite::abrir_existente(&servidor.db).unwrap();
    let bytes: Vec<u8> = db.query_row("SELECT contenido FROM revisiones_fuente WHERE id=?1",
        [hecho.revision_fuente_id], |fila| fila.get(0)).unwrap();
    let indice: usize = (hecho.linea - 1).try_into().unwrap();
    assert!(std::str::from_utf8(&bytes).unwrap().lines().nth(indice).unwrap().contains(&hecho.pasaje));
    assert_eq!(hecho.linea, 29);
}

#[tokio::test]
async fn alterna_cronologias_y_reabre_cada_intento_propio() {
    let servidor = Servidor::nuevo();
    let (cookie, dan) = iniciar(&servidor).await;
    let lr = abrir_lr(&servidor, &cookie).await;
    let (docente, _) = servidor.entrar("docente-demo").await;
    let cuerpo = json!({"texto":"¿Desde cuándo comenzó?","sugerencia_id":"inicio"});
    let mut secuencia = Vec::new();
    for (intento, voz, dato, plazo) in [
        (&lr, "LR", "lr-inicio", "hace dos días"),
        (&dan, perfil::HABLANTE, INICIO, "hace cuatro horas"),
    ] {
        let turno = servidor.peticion("POST", &format!("/api/intentos/{}/turnos", intento.intento_id),
            Some(&cookie), Some("t051-misma-clave"), Some(cuerpo.clone())).await;
        assert_eq!(turno.estado, StatusCode::CREATED);
        assert_eq!(turno.cuerpo["hablante"], voz);
        assert!(turno.cuerpo["respuesta"].as_str().unwrap().contains(plazo));
        let traza = servidor.consultar("GET", &format!("/api/intentos/{}/turnos/{}/traza", intento.intento_id,
            turno.cuerpo["turno_id"].as_str().unwrap()), Some(&docente), None).await;
        assert_eq!(traza.cuerpo["hecho"]["id"], dato);
        assert_eq!(traza.cuerpo["hecho"]["version"], intento.version_caso);
        secuencia.push(turno.cuerpo);
    }
    let (reingreso, _) = servidor.entrar(intento_lr::ESTUDIANTE_LR).await;
    for (i, intento) in [&lr, &dan].into_iter().enumerate() {
        let vista = servidor.consultar("GET", &format!("/api/intentos/{}", intento.intento_id), Some(&reingreso), None).await;
        assert_eq!(vista.cuerpo["turnos"], json!([secuencia[i]]));
    }
    assert_ne!(secuencia[0]["turno_id"], secuencia[1]["turno_id"]);
}
