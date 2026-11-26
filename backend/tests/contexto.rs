mod fixtures;

use axum::http::{HeaderMap, HeaderValue, StatusCode};
use fixtures::{Servidor, intento_lr};
use rusqlite::{Connection, params};
use serde_json::json;
use sircle::{
    agentes::mensajes::Agente,
    app::{estado::Estado, router::router},
    modules::intentos::intento,
    shared::{contexto::{self, ContextoEntrevista}, operaciones::Operacion},
    sqlite,
};

fn turno(db: &Connection, id: &str, intento: &str, agente: &str, fase: &str, hablante: &str) {
    db.execute(
        "INSERT INTO turnos (id,intento_id,orden,agente,hablante,pregunta,respuesta,tipo_respuesta,operacion_id,revision,fase)
         VALUES (?1, ?2, (SELECT coalesce(max(orden),0)+1 FROM turnos WHERE intento_id=?2),
                 ?3,?4,'Pregunta de ensayo','Respuesta de ensayo','hecho',?1,1,?5)",
        params![id, intento, agente, hablante, fase],
    ).unwrap();
}

fn version_p02(db: &Connection, nombre: &str) -> i64 {
    let id: i64 = db.query_row(
        "INSERT INTO versiones_clinicas (escenario_id, version, estado) VALUES ('P02',?1,'ensayo') RETURNING id",
        [nombre], |fila| fila.get(0),
    ).unwrap();
    db.execute(
        "INSERT INTO versiones_fuentes
         SELECT vf.escenario_id, ?1, vf.fuente_id, vf.revision_fuente_id
         FROM versiones_fuentes vf JOIN versiones_clinicas v ON v.id=vf.version_clinica_id
         WHERE vf.escenario_id='P02' AND v.version LIKE 'originales-%'",
        [id],
    ).unwrap();
    id
}

fn hecho(db: &Connection, version: i64, id: &str, fase: &str, hablante: &str, pasaje: &str) {
    let (fuente, revision, texto): (i64, i64, Vec<u8>) = db.query_row(
        "SELECT f.id, r.id, r.contenido FROM versiones_fuentes vf
         JOIN fuentes f ON f.id=vf.fuente_id JOIN revisiones_fuente r ON r.id=vf.revision_fuente_id
         WHERE vf.version_clinica_id=?1 AND f.rol='sp_actor'",
        [version], |fila| Ok((fila.get(0)?, fila.get(1)?, fila.get(2)?)),
    ).unwrap();
    let linea = std::str::from_utf8(&texto).unwrap().lines().position(|linea| linea.contains(pasaje)).unwrap() + 1;
    db.execute(
        "INSERT INTO hechos VALUES (?1,'P02',?2,?3,?4,'Condición de ensayo','Respuesta de ensayo',?5,?6,?7,?8)",
        params![id, version, fase, hablante, fuente, revision, linea, pasaje],
    ).unwrap();
}

fn reconstruir(db: &Connection, id: &str, estudiante: &str, agente: Agente, hablante: &str) -> ContextoEntrevista {
    let intento = intento::recuperar(db, id, estudiante).unwrap();
    contexto::entrevista(db, &intento, agente, hablante).unwrap()
}

#[tokio::test]
async fn reconstruye_memoria_por_estudiante_intento_agente_y_voz() {
    let mut servidor = Servidor::nuevo();
    let db = sqlite::abrir_existente(&servidor.db).unwrap();
    db.execute("INSERT INTO asignaciones VALUES ('lr-ensayo','estudiante-sin-asignacion','docente-demo')", []).unwrap();
    let (sesion_a, _) = servidor.entrar(intento_lr::ESTUDIANTE_LR).await;
    let (sesion_b, _) = servidor.entrar("estudiante-sin-asignacion").await;
    let mut ids = Vec::new();
    let mut turnos_lr = Vec::new();
    for cookie in [&sesion_a, &sesion_b] {
        let alta = intento_lr::alta(&servidor, cookie).await;
        assert_eq!(alta.estado, StatusCode::CREATED);
        let id = alta.cuerpo["intento_id"].as_str().unwrap();
        intento_lr::resolver_pre(&servidor, id);
        let turno = servidor.peticion("POST", &format!("/api/intentos/{id}/turnos"), Some(cookie), Some("misma-clave"),
            Some(json!({"sugerencia_id":"inicio","texto":"¿Desde cuándo comenzó?"}))).await;
        assert_eq!(turno.estado, StatusCode::CREATED);
        ids.push(id.to_owned());
        turnos_lr.push(turno.cuerpo);
    }
    assert_ne!(turnos_lr[0]["operacion_id"], turnos_lr[1]["operacion_id"]);

    let autorizada = version_p02(&db, "ensayo-contexto-p02-v1");
    let otra = version_p02(&db, "ensayo-contexto-p02-v2");
    db.execute_batch(
        "INSERT INTO actividades VALUES ('p02-t042','P02','Ensayo de contexto de Nannu','ensayo',1);
         INSERT INTO asignaciones VALUES ('p02-t042','estudiante-lr','docente-demo');",
    ).unwrap();
    db.execute(
        "INSERT INTO configuraciones_ensayo VALUES ('p02-t042','P02',?1,'rubrica-ensayo-v1','instrumento-ensayo-v1')",
        [autorizada],
    ).unwrap();
    let alta = servidor.peticion("POST", "/api/intentos", Some(&sesion_a), Some("t042-p02"),
        Some(json!({"actividad_id":"p02-t042"}))).await;
    assert_eq!(alta.estado, StatusCode::CREATED);
    let nannu = alta.cuerpo["intento_id"].as_str().unwrap();
    intento_lr::resolver_pre(&servidor, nannu);
    let dolor = "R sided chest pain when breathing.";
    let tos = "Family will say she has had a productive cough.";
    hecho(&db, autorizada, "p02-dolor-t042", "inicial", "Nannu", dolor);
    hecho(&db, autorizada, "p02-tos-t042", "inicial", "Acompañante", tos);
    hecho(&db, autorizada, "p02-fase-ajena", "posterior", "Nannu", dolor);
    hecho(&db, otra, "p02-version-ajena", "inicial", "Nannu", dolor);
    turno(&db, "p02-habla-nannu", nannu, "nannu", "inicial", "Nannu");
    turno(&db, "p02-habla-acompanante", nannu, "nannu", "inicial", "Acompañante");
    turno(&db, "p02-fase-posterior", nannu, "nannu", "posterior", "Nannu");
    turno(&db, "p02-agente-ajeno", nannu, "lr", "inicial", "Nannu");

    let matriz = [
        (ids[0].as_str(), "estudiante-lr", Agente::Lr, "LR", turnos_lr[0]["turno_id"].as_str().unwrap(),
            vec!["lr-antecedentes-familiares", "lr-inicio"]),
        (ids[1].as_str(), "estudiante-sin-asignacion", Agente::Lr, "LR", turnos_lr[1]["turno_id"].as_str().unwrap(),
            vec!["lr-antecedentes-familiares", "lr-inicio"]),
        (nannu, "estudiante-lr", Agente::Nannu, "Nannu", "p02-habla-nannu", vec!["p02-dolor-t042"]),
        (nannu, "estudiante-lr", Agente::Nannu, "Acompañante", "p02-habla-acompanante", vec!["p02-tos-t042"]),
    ];
    let mut contextos = Vec::new();
    for (intento, estudiante, agente, voz, turno, hechos) in &matriz {
        let contexto = reconstruir(&db, intento, estudiante, *agente, voz);
        assert_eq!(contexto.memoria.iter().map(|t| t.turno_id.as_str()).collect::<Vec<_>>(), [*turno]);
        assert_eq!(contexto.candidatos.iter().map(|h| h.id.as_str()).collect::<Vec<_>>(), *hechos);
        assert!(contexto.candidatos.iter().all(|h|
            h.escenario == contexto.caso && h.version == contexto.version && h.fase == contexto.fase && h.hablante == *voz));
        contextos.push(contexto);
    }
    let propia = intento::recuperar(&db, nannu, "estudiante-lr").unwrap();
    assert_eq!(contexto::entrevista(&db, &propia, Agente::Lr, "LR").unwrap_err().estado, StatusCode::FORBIDDEN);
    assert_eq!(contexto::entrevista(&db, &propia, Agente::Nannu, "LR").unwrap_err().estado, StatusCode::FORBIDDEN);
    assert!(intento::recuperar(&db, nannu, "estudiante-sin-asignacion").is_err());
    let mut cabeceras = HeaderMap::new();
    cabeceras.insert("idempotency-key", HeaderValue::from_static("misma"));
    let operacion = |actor, alcance| Operacion::nueva(actor, alcance, &cabeceras, "POST turno", &"pregunta").unwrap().id();
    assert_eq!(operacion("A", "1"), operacion("A", "1"));
    assert_ne!(operacion("A", "1"), operacion("B", "1"));
    assert_ne!(operacion("A", "1"), operacion("A", "2"));
    drop(db);

    servidor.app = router(Estado::con_base(servidor.db.clone()).unwrap());
    let db = sqlite::abrir_existente(&servidor.db).unwrap();
    for (fila, esperado) in matriz.iter().zip(&contextos) {
        let (id, estudiante, agente, voz, _, _) = fila;
        assert_eq!(reconstruir(&db, id, estudiante, *agente, voz), *esperado);
        let cookie = if *estudiante == "estudiante-lr" { &sesion_a } else { &sesion_b };
        assert_eq!(servidor.consultar("GET", &format!("/api/intentos/{id}"), Some(cookie), None).await.estado, StatusCode::OK);
    }
    println!("EVIDENCIA {}", json!({ "contextos_antes_y_despues": contextos,
        "rechazados": ["p02-fase-ajena","p02-version-ajena","p02-fase-posterior","p02-agente-ajeno"],
        "reinicio_iguales": true, "origen_p02": "material identificable de ensayo, perfiles clínicos en T044" }));
}
