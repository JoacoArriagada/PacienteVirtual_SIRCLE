mod fixtures;

use axum::http::StatusCode;
use fixtures::{Servidor, intento_lr};
use rusqlite::params;
use serde_json::json;
use sircle::{
    agentes::{mensajes::{Cabecera, Respuesta, TipoResultado}, nannu::{enrutamiento, entrevista, perfil}},
    modules::{entrevista::{ollama::{Invocacion, Seleccion}, turnos}, intentos::intento::{self, Intento}},
    shared::{contexto::{self, ContextoEntrevista}, ficha_aprobada},
    sqlite,
};

const DOLOR: &str = "p02-dolor";
const TOS: &str = "p02-tos-productiva";

async fn iniciar(servidor: &Servidor) -> (String, Intento) {
    let db = sqlite::abrir_existente(&servidor.db).unwrap();
    db.execute("UPDATE actividades SET habilitada=1 WHERE id=?1", [perfil::ACTIVIDAD]).unwrap();
    db.execute("INSERT INTO asignaciones VALUES (?1,?2,'docente-demo')",
        params![perfil::ACTIVIDAD, intento_lr::ESTUDIANTE_LR]).unwrap();
    let (cookie, _) = servidor.entrar(intento_lr::ESTUDIANTE_LR).await;
    let alta = servidor.peticion("POST", "/api/intentos", Some(&cookie), Some("alta-nannu"),
        Some(json!({"actividad_id": perfil::ACTIVIDAD}))).await;
    assert_eq!(alta.estado, StatusCode::CREATED);
    let id = alta.cuerpo["intento_id"].as_str().unwrap();
    intento_lr::resolver_pre(servidor, id);
    (cookie, intento::recuperar(&db, id, intento_lr::ESTUDIANTE_LR).unwrap())
}

fn recuperar(servidor: &Servidor, intento: &Intento, voz: &str) -> ContextoEntrevista {
    let db = sqlite::abrir_existente(&servidor.db).unwrap();
    let (agente, hablante) = enrutamiento::destino(intento, voz).unwrap();
    contexto::entrevista(&db, intento, agente, hablante).unwrap()
}

#[tokio::test]
async fn mantiene_paciente_y_acompanante() {
    let servidor = Servidor::nuevo();
    let (_, intento) = iniciar(&servidor).await;
    for voz in [perfil::PACIENTE, perfil::ACOMPANANTE] {
        assert_eq!(enrutamiento::destino(&intento, voz).unwrap(), (perfil::AGENTE, voz));
    }
    assert_eq!(enrutamiento::destino(&intento, "LR").unwrap_err().estado, StatusCode::UNPROCESSABLE_ENTITY);
}

#[tokio::test]
async fn atribuye_el_dolor_y_la_tos() {
    let servidor = Servidor::nuevo();
    let (_, intento) = iniciar(&servidor).await;
    let paciente = recuperar(&servidor, &intento, perfil::PACIENTE);
    let familiar = recuperar(&servidor, &intento, perfil::ACOMPANANTE);
    let dolor = entrevista::elegir(&paciente, DOLOR).unwrap();
    let tos = entrevista::elegir(&familiar, TOS).unwrap();
    assert_eq!(dolor.hablante, perfil::PACIENTE);
    assert_eq!(tos.hablante, perfil::ACOMPANANTE);
    assert_eq!(dolor.escenario, tos.escenario);
    assert_eq!(dolor.version, tos.version);
    assert_eq!(dolor.linea, 5);
    assert_eq!(tos.linea, 5);
    assert!(dolor.respuesta.starts_with("Me duele") && tos.respuesta.contains("flema"));
    assert_eq!(dolor.pasaje, "R sided chest pain when breathing.");
    assert_eq!(tos.pasaje, "Family will say she has had a productive cough.");
    println!("EVIDENCIA {}", json!({"intento": intento.intento_id, "dolor": dolor, "tos": tos}));
}

#[tokio::test]
async fn consulta_al_acompanante() {
    let servidor = Servidor::nuevo();
    let (_, intento) = iniciar(&servidor).await;
    let familiar = recuperar(&servidor, &intento, perfil::ACOMPANANTE);
    let dato = entrevista::elegir(&familiar, TOS).unwrap();
    assert_eq!((dato.id.as_str(), dato.hablante.as_str()), (TOS, perfil::ACOMPANANTE));
    assert!(entrevista::elegir(&familiar, DOLOR).is_err());
}

#[tokio::test]
async fn cambiar_hablante_conserva_propietario() {
    let servidor = Servidor::nuevo();
    let (_, intento) = iniciar(&servidor).await;
    let paciente = recuperar(&servidor, &intento, perfil::PACIENTE);
    let familiar = recuperar(&servidor, &intento, perfil::ACOMPANANTE);
    assert_eq!(paciente.estudiante_id, familiar.estudiante_id);
    assert_eq!(paciente.intento_id, familiar.intento_id);
    assert_eq!(paciente.fase, familiar.fase);
    let (otra, _) = servidor.entrar("estudiante-sin-asignacion").await;
    let ajeno = servidor.consultar("GET", &format!("/api/intentos/{}", intento.intento_id), Some(&otra), None).await;
    assert_eq!(ajeno.estado, StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn rechaza_atribucion_incorrecta() {
    let servidor = Servidor::nuevo();
    let (_, intento) = iniciar(&servidor).await;
    let mut paciente = recuperar(&servidor, &intento, perfil::PACIENTE);
    let dato = recuperar(&servidor, &intento, perfil::ACOMPANANTE).candidatos.pop().unwrap();
    paciente.candidatos.push(dato);
    assert_eq!(entrevista::elegir(&paciente, TOS).unwrap_err().estado, StatusCode::SERVICE_UNAVAILABLE);
}

#[tokio::test]
async fn separa_sugerencias_por_hablante() {
    let servidor = Servidor::nuevo();
    let (_, intento) = iniciar(&servidor).await;
    let paciente = recuperar(&servidor, &intento, perfil::PACIENTE);
    let familiar = recuperar(&servidor, &intento, perfil::ACOMPANANTE);
    let sugerencia = ficha_aprobada::sugerencias(&familiar.candidatos);
    assert_eq!(sugerencia.len(), 1);
    let opcion = sugerencia[0];
    assert_eq!(opcion.hecho_id, TOS);
    assert!(ficha_aprobada::literal(Some(opcion.id), opcion.texto, &paciente.candidatos).is_none());
    assert!(ficha_aprobada::literal(Some(opcion.id), opcion.texto, &familiar.candidatos).is_some());
}

#[tokio::test]
async fn recupera_ruta_del_acompanante() {
    let servidor = Servidor::nuevo();
    let (_, intento) = iniciar(&servidor).await;
    let ruta = recuperar(&servidor, &intento, perfil::ACOMPANANTE);
    let db = sqlite::abrir_existente(&servidor.db).unwrap();
    let reabierto = intento::recuperar(&db, &intento.intento_id, &intento.estudiante_id).unwrap();
    assert_eq!(recuperar(&servidor, &reabierto, perfil::ACOMPANANTE), ruta);
}

#[tokio::test]
async fn localiza_pasaje_del_acompanante() {
    let servidor = Servidor::nuevo();
    let (_, intento) = iniciar(&servidor).await;
    let familiar = recuperar(&servidor, &intento, perfil::ACOMPANANTE);
    let hecho = entrevista::elegir(&familiar, TOS).unwrap();
    let db = sqlite::abrir_existente(&servidor.db).unwrap();
    let bytes: Vec<u8> = db.query_row("SELECT contenido FROM revisiones_fuente WHERE id=?1",
        [hecho.revision_fuente_id], |fila| fila.get(0)).unwrap();
    let indice: usize = (hecho.linea - 1).try_into().unwrap();
    let linea = std::str::from_utf8(&bytes).unwrap().lines().nth(indice).unwrap();
    assert!(linea.contains(&hecho.pasaje));
    assert_eq!(hecho.fuente, perfil::FUENTE_ACTOR);
}

#[tokio::test]
async fn confirma_turnos_de_ambas_voces_por_http() {
    let servidor = Servidor::nuevo();
    let (cookie, intento) = iniciar(&servidor).await;
    let ruta = format!("/api/intentos/{}", intento.intento_id);
    let turnos = format!("{ruta}/turnos");
    let (docente, _) = servidor.entrar("docente-demo").await;
    let mut guardados = Vec::new();
    for (voz, id, clave, texto, candidato) in [
        (perfil::PACIENTE, "p02-dolor", "t050-dolor", "¿Te duele el pecho al respirar?", DOLOR),
        (perfil::ACOMPANANTE, "p02-tos", "t050-tos", "¿Ha tenido tos con flema?", TOS),
    ] {
        let cuerpo = json!({"texto": texto, "sugerencia_id": id, "destinatario": voz});
        let confirmado = servidor.peticion("POST", &turnos, Some(&cookie), Some(clave), Some(cuerpo.clone())).await;
        assert_eq!(confirmado.estado, StatusCode::CREATED);
        assert_eq!(confirmado.cuerpo["agente"], "nannu");
        assert_eq!(confirmado.cuerpo["hablante"], voz);
        assert_eq!(confirmado.cuerpo["intento_id"], intento.intento_id);
        let traza = servidor.consultar("GET", &format!("{turnos}/{}/traza", confirmado.cuerpo["turno_id"].as_str().unwrap()),
            Some(&docente), None).await;
        assert_eq!(traza.cuerpo["hecho"]["hablante"], voz);
        assert_eq!(traza.cuerpo["hecho"]["id"], candidato);
        assert_eq!(traza.cuerpo["mensaje"]["hablante"], voz);
        let mut cruzada = cuerpo;
        cruzada["destinatario"] = json!("LR");
        let conflicto = servidor.peticion("POST", &turnos, Some(&cookie), Some(clave), Some(cruzada)).await;
        assert_eq!(conflicto.estado, StatusCode::CONFLICT);
        guardados.push(confirmado.cuerpo);
    }
    let vista = servidor.consultar("GET", &ruta, Some(&cookie), None).await;
    assert_eq!(vista.cuerpo["turnos"], json!(guardados));
}

#[tokio::test]
async fn localiza_fallo_de_atribucion() {
    let servidor = Servidor::nuevo();
    let (cookie, intento) = iniciar(&servidor).await;
    let cabecera = Cabecera { operacion_id: "t050-cruce".into(), intento_id: intento.intento_id.clone(),
        agente: perfil::AGENTE, caso: intento.caso.clone(), version: intento.version_caso.clone(),
        fase: intento.fase.clone(), hablante: perfil::PACIENTE.into() };
    let respuesta = Respuesta { cabecera: cabecera.clone(), tipo: TipoResultado::Seleccion,
        resultado: Invocacion { operacion_id: cabecera.operacion_id.clone(), inicio_unix_ms: 10, fin_unix_ms: 11,
            modelo: "fixture-t050".into(), digest: "fixture".into(), solicitud: json!({}),
            seleccion: Seleccion { hecho_id: TOS.into() }, respuesta_cruda: json!({"hecho_id":TOS}).to_string(),
            duracion_motor_ns: None, carga_modelo_ns: None, tokens_respuesta: None } };
    let paciente = recuperar(&servidor, &intento, perfil::PACIENTE);
    let acompanante = recuperar(&servidor, &intento, perfil::ACOMPANANTE);
    let origen = acompanante.candidatos.iter().find(|hecho| hecho.id == TOS).unwrap();
    assert!(!paciente.candidatos.iter().any(|hecho| hecho.id == TOS));
    let mut db = sqlite::abrir_existente(&servidor.db).unwrap();
    let tx = db.transaction().unwrap();
    assert!(turnos::confirmar_mensaje(&tx, &intento, &paciente.candidatos,
        "¿Te duele el pecho?", &cabecera, &respuesta).unwrap().is_none());
    tx.commit().unwrap();
    let motivo: String = db.query_row("SELECT motivo FROM rechazos_mensajes WHERE operacion_id=?1",
        [&cabecera.operacion_id], |fila| fila.get(0)).unwrap();
    assert_eq!(motivo, "seleccion_fuera_de_hablante");
    let vista = servidor.consultar("GET", &format!("/api/intentos/{}", intento.intento_id), Some(&cookie), None).await;
    assert_eq!(vista.cuerpo["turnos"], json!([]));
    assert_eq!(vista.cuerpo["revision"], intento.revision);
    println!("EVIDENCIA {}", json!({"intentado": cabecera, "candidato_cruzado": TOS, "motivo": motivo,
        "turnos": 0, "origen_candidato": origen, "adaptador":"fixture-t050"}));
}

#[cfg(target_os = "linux")]
#[tokio::test]
#[ignore = "requiere Axum de ensayo con cuatro trabajadores y Ollama local"]
async fn retoma_turno_del_familiar() {
    use std::{fs, path::PathBuf, process::Command, time::Duration};
    use reqwest::{Client, header};
    use serde_json::Value;

    async fn http(cliente: &Client, ruta: &str, cookie: Option<&str>, envio: Option<(&str, Value)>) -> (StatusCode, Value, Option<String>) {
        let mut peticion = if let Some((clave, cuerpo)) = envio {
            cliente.post(ruta).header("Idempotency-Key", clave).json(&cuerpo)
        } else { cliente.get(ruta) };
        if let Some(cookie) = cookie {
            peticion = peticion.header(header::COOKIE, cookie);
        }
        let respuesta = peticion.send().await.unwrap();
        let estado = respuesta.status();
        let sesion = respuesta.headers().get(header::SET_COOKIE).map(|valor| valor.to_str().unwrap().split(';').next().unwrap().to_owned());
        (estado, respuesta.json().await.unwrap(), sesion)
    }
    fn registros(log: &PathBuf, prefijo: &str) -> Vec<Value> {
        fs::read_to_string(log).unwrap().lines().filter_map(|linea| linea.strip_prefix(prefijo))
            .filter_map(|linea| serde_json::from_str(linea).ok()).collect()
    }

    let origen = std::env::var("SIRCLE_URL").unwrap_or_else(|_| "http://127.0.0.1:18081".into());
    let base = PathBuf::from(std::env::var_os("SIRCLE_T053_DB").expect("Indica la base temporal del servidor."));
    let log = PathBuf::from(std::env::var_os("SIRCLE_T053_LOG").expect("Indica el registro de Axum."));
    let cliente = Client::builder().no_proxy().timeout(Duration::from_secs(270)).build().unwrap();
    let (estado, _, cookie) = http(&cliente, &format!("{origen}/api/sesiones"), None,
        Some(("t053-sesion", json!({"identidad":"estudiante-lr"})))).await;
    assert_eq!(estado, StatusCode::CREATED);
    let cookie = cookie.unwrap();
    let mut intentos = Vec::new();
    for actividad in ["nannu-ensayo", "lr-ensayo"] {
        let (estado, alta, _) = http(&cliente, &format!("{origen}/api/intentos"), Some(&cookie),
            Some((actividad, json!({"actividad_id":actividad})))).await;
        assert_eq!(estado, StatusCode::CREATED);
        let id = alta["intento_id"].as_str().unwrap();
        let resultado = Command::new(env!("CARGO_BIN_EXE_sircle")).arg("resolver-pre-ensayo")
            .arg("--db").arg(&base).arg("--intento").arg(id).output().unwrap();
        assert!(resultado.status.success());
        intentos.push((id.to_owned(), format!("{origen}/api/intentos/{id}")));
    }
    let (id_nannu, ruta_nannu) = &intentos[0];
    let (_, ruta_lr) = &intentos[1];
    let previa = registros(&log, "SIRCLE_TRABAJADOR ");
    assert_eq!(previa.len(), 4);
    let nannu = previa.iter().find(|p| p["agente"] == "nannu").unwrap();
    let pid_nannu = nannu["pid"].as_u64().unwrap();
    let cmdline = fs::read_to_string(format!("/proc/{pid_nannu}/cmdline")).unwrap();
    assert!(cmdline.contains("trabajador\0--agente\0nannu\0"), "{cmdline:?}");
    let pregunta = json!({"texto":"¿La ha visto toser y expulsar flema?","destinatario":"Acompañante"});
    let clave = "t053-familiar";
    let ruta_envio = format!("{ruta_nannu}/turnos");
    let primera_respuesta = {
        let (cliente, cookie, ruta, pregunta) = (cliente.clone(), cookie.clone(), ruta_envio.clone(), pregunta.clone());
        tokio::spawn(async move { http(&cliente, &ruta, Some(&cookie), Some((clave, pregunta))).await })
    };
    let primera = tokio::time::timeout(Duration::from_secs(20), async {
        loop {
            if let Some(marca) = registros(&log, "SIRCLE_TRABAJADOR_OPERACION ").into_iter()
                .find(|m| m["pid"] == pid_nannu && m["intento_id"] == *id_nannu)
            {
                break marca;
            }
            tokio::time::sleep(Duration::from_millis(25)).await;
        }
    }).await.unwrap();
    assert!(!primera_respuesta.is_finished());
    assert_eq!(registros(&log, "SIRCLE_TURNO_CONFIRMADO ").len(), 0);
    let fin = Command::new("kill").args(["-KILL", &pid_nannu.to_string()]).status().unwrap();
    assert!(fin.success());
    let (interrupcion, fallo, _) = primera_respuesta.await.unwrap();
    assert_eq!(interrupcion, StatusCode::SERVICE_UNAVAILABLE);
    let (_, vacio, _) = http(&cliente, ruta_nannu, Some(&cookie), None).await;
    assert_eq!(vacio["turnos"], json!([]));

    let (estado_lr, turno_lr, _) = http(&cliente, &format!("{ruta_lr}/turnos"), Some(&cookie), Some(("t053-lr",
        json!({"texto":"¿Desde cuándo comenzó la falta de aire?"})))).await;
    assert_eq!(estado_lr, StatusCode::CREATED);
    assert!(turno_lr["respuesta"].as_str().unwrap().contains("hace dos días"));
    assert_eq!(registros(&log, "SIRCLE_TRABAJADOR ").len(), 4);
    let pid_lr = previa.iter().find(|p| p["agente"] == "lr").unwrap()["pid"].as_u64().unwrap();
    let evento_lr = registros(&log, "SIRCLE_TRABAJADOR_OPERACION ").into_iter()
        .find(|m| m["operacion_id"] == turno_lr["operacion_id"]).unwrap();
    assert_eq!(evento_lr["pid"], pid_lr);
    assert!(!PathBuf::from(format!("/proc/{pid_nannu}")).exists());

    let (recuperada, turno, _) = http(&cliente, &ruta_envio, Some(&cookie), Some((clave, pregunta.clone()))).await;
    assert_eq!(recuperada, StatusCode::CREATED, "{turno}");
    assert_eq!(turno["hablante"], "Acompañante");
    assert_eq!(turno["intento_id"], *id_nannu);
    assert_eq!(turno["operacion_id"], primera["operacion_id"]);
    let (reenvio, igual, _) = http(&cliente, &ruta_envio, Some(&cookie), Some((clave, pregunta))).await;
    assert_eq!(reenvio, StatusCode::OK);
    assert_eq!(igual, turno);
    let (_, vista, _) = http(&cliente, ruta_nannu, Some(&cookie), None).await;
    assert_eq!(vista["turnos"], json!([turno]));
    let arranques = registros(&log, "SIRCLE_TRABAJADOR ");
    assert_eq!(arranques.len(), 5);
    let repuesto = arranques.last().unwrap();
    assert_eq!(repuesto["agente"], "nannu");
    assert_ne!(repuesto["pid"], pid_nannu);
    let pid_nuevo = repuesto["pid"].as_u64().unwrap();
    let ultimo = registros(&log, "SIRCLE_TRABAJADOR_OPERACION ").into_iter()
        .find(|m| m["operacion_id"] == turno["operacion_id"] && m["pid"] == pid_nuevo).unwrap();
    for ficha in previa.iter().filter(|p| p["agente"] != "nannu") {
        assert!(PathBuf::from(format!("/proc/{}", ficha["pid"])).exists());
    }
    println!("EVIDENCIA {}", json!({
        "inventario_previo": previa, "arranque_recuperado": repuesto, "envio_interrumpido": primera,
        "interrupcion_http": interrupcion.as_u16(), "interrupcion_cuerpo": fallo,
        "otros_durante_interrupcion": evento_lr, "turno_lr": turno_lr,
        "envio_recuperado": ultimo, "acuse_recuperado": turno, "reenvio_http": reenvio.as_u16(),
        "turnos_tras_interrupcion": 0, "turnos_tras_reintento": vista["turnos"].as_array().unwrap().len(),
    }));
}
