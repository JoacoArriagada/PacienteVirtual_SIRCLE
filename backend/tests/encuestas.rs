mod fixtures;

use axum::http::StatusCode;
use fixtures::{Servidor, versiones_rubrica};
use serde_json::json;
use sircle::{modules::evaluacion::rubricas, sqlite};

#[tokio::test]
async fn conserva_decisiones_formas_y_pares_sin_revelar_claves() {
    let servidor = Servidor::nuevo();
    let mut db = sqlite::abrir_existente(&servidor.db).unwrap();
    let rubrica = versiones_rubrica::cargar(&mut db, versiones_rubrica::V1);
    rubricas::habilitar_ensayo(&db, rubrica.id).unwrap();
    db.execute_batch(
        "INSERT INTO identidades VALUES ('estudiante-2','estudiante'), ('estudiante-3','estudiante');
         INSERT INTO asignaciones VALUES
             ('lr-ensayo','estudiante-2','docente-demo'), ('lr-ensayo','estudiante-3','docente-demo');",
    ).unwrap();
    drop(db);

    let mut casos = Vec::new();
    for identidad in ["estudiante-lr", "estudiante-2", "estudiante-3"] {
        let (cookie, _) = servidor.entrar(identidad).await;
        let alta = json!({"actividad_id":"lr-ensayo"});
        let intento = servidor.peticion("POST", "/api/intentos", Some(&cookie), Some("alta"), Some(alta.clone())).await;
        assert_eq!(intento.estado, StatusCode::CREATED);
        let repetido = servidor.peticion("POST", "/api/intentos", Some(&cookie), Some("alta"), Some(alta)).await;
        assert_eq!(repetido.cuerpo, intento.cuerpo);
        let ruta = format!("/api/intentos/{}", intento.cuerpo["intento_id"].as_str().unwrap());
        casos.push((cookie, ruta));
    }
    let (cookie, ruta) = &casos[0];
    let vista = servidor.consultar("GET", ruta, Some(cookie), None).await;
    assert!(vista.cuerpo.get("introduccion").is_none());
    let ajena = servidor.consultar("GET", &format!("{ruta}/encuestas/pre"), Some(&casos[1].0), None).await;
    assert_eq!(ajena.estado, StatusCode::FORBIDDEN);
    let prematuro = servidor.consultar("GET", &format!("{ruta}/encuestas/post"), Some(cookie), None).await;
    assert_eq!(prematuro.estado, StatusCode::CONFLICT);

    let mut percepcion = Vec::new();
    for (n, (cookie, ruta)) in casos.iter().enumerate() {
        let pre = format!("{ruta}/encuestas/pre");
        let formulario = servidor.consultar("GET", &pre, Some(cookie), None).await;
        assert_eq!(formulario.estado, StatusCode::OK);
        let forma = if n == 1 { "B" } else { "A" };
        assert_eq!(formulario.cuerpo["forma"], forma);
        let items = formulario.cuerpo["items"].as_array().unwrap();
        assert_eq!(items.len(), 5);
        assert!(items.iter().all(|item| item.get("clave").is_none() && item.get("justificacion").is_none()));
        percepcion.push(items[..3].to_vec());
        let decision = if n == 1 { json!({"estado":"omitido"}) }
            else { json!({"estado":"respondido","respuestas":{"P1":2,"P2":3,"P3":4,"R1":2,"R2":1}}) };
        let acuse = servidor.peticion("PUT", &pre, Some(cookie), Some("pre"), Some(decision)).await;
        assert_eq!(acuse.estado, StatusCode::OK, "{}", acuse.cuerpo);
        let vista = servidor.consultar("GET", ruta, Some(cookie), None).await;
        assert_eq!(vista.cuerpo["estado"], "atencion");
        assert!(vista.cuerpo.get("introduccion").is_some());
        let cambio = servidor.peticion("PUT", &pre, Some(cookie), Some("pre-tarde"), Some(json!({"estado":"omitido"}))).await;
        assert_eq!(cambio.estado, StatusCode::CONFLICT);
        let entrega = servidor.peticion("POST", &format!("{ruta}/entrega"), Some(cookie), Some("entrega"),
            Some(json!({"revision_esperada":vista.cuerpo["revision"]}))).await;
        assert_eq!(entrega.estado, StatusCode::CREATED, "{}", entrega.cuerpo);
    }

    for (n, (cookie, ruta)) in casos.iter().enumerate() {
        let post = format!("{ruta}/encuestas/post");
        let formulario = servidor.consultar("GET", &post, Some(cookie), None).await;
        assert_eq!(formulario.estado, StatusCode::OK);
        assert_eq!(formulario.cuerpo["forma"], if n == 1 { "A" } else { "B" });
        let items = formulario.cuerpo["items"].as_array().unwrap();
        assert_eq!(items.len(), 7);
        assert_eq!(items[..3], percepcion[n]);
        let decision = match n {
            0 => json!({"estado":"respondido","respuestas":{"P1":4,"P2":3,"P3":5,"R1":1,"R2":2,"U1":5}}),
            1 => json!({"estado":"respondido","respuestas":{"P1":3,"R1":2}}),
            _ => json!({"estado":"omitido"}),
        };
        let acuse = servidor.peticion("PUT", &post, Some(cookie), Some("post"), Some(decision.clone())).await;
        assert_eq!(acuse.estado, StatusCode::OK);
        let repetido = servidor.peticion("PUT", &post, Some(cookie), Some("post"), Some(decision)).await;
        assert_eq!(repetido.cuerpo, acuse.cuerpo);
    }
    let informe = "/api/actividades/lr-ensayo/resultados-educativos";
    assert_eq!(servidor.consultar("GET", informe, Some(cookie), None).await.estado, StatusCode::FORBIDDEN);
    let (docente, _) = servidor.entrar("docente-demo").await;
    let resultados = servidor.consultar("GET", informe, Some(&docente), None).await;
    assert_eq!(resultados.estado, StatusCode::OK);
    for (campo, esperado) in [("invitados", 3), ("pre_respondidos", 2), ("post_respondidos", 2), ("pares_completos", 1)] {
        assert_eq!(resultados.cuerpo[campo], esperado, "{}", resultados.cuerpo);
    }
    assert_eq!(resultados.cuerpo["grupos"][0]["cambio_razonamiento"]["50"], 1);
    assert_eq!(resultados.cuerpo["grupos"][0]["percepcion"][0]["cambio"]["2"], 1);
    println!("E14 {}", resultados.cuerpo);
}

#[tokio::test]
async fn alterna_altas_concurrentes_y_compara_las_dos_secuencias_completas() {
    let servidor = Servidor::nuevo();
    let mut db = sqlite::abrir_existente(&servidor.db).unwrap();
    let rubrica = versiones_rubrica::cargar(&mut db, versiones_rubrica::V1);
    rubricas::habilitar_ensayo(&db, rubrica.id).unwrap();
    versiones_rubrica::asignar_segundo_estudiante(&db);
    drop(db);
    let (a, _) = servidor.entrar("estudiante-lr").await;
    let (b, _) = servidor.entrar(versiones_rubrica::ESTUDIANTE_B).await;
    let abrir = |cookie| servidor.peticion("POST", "/api/intentos", Some(cookie), Some("alta"),
        Some(json!({"actividad_id":"lr-ensayo"})));
    let (alta_a, alta_b, reenvio_a) = tokio::join!(abrir(&a), abrir(&b), abrir(&a));
    assert_eq!(alta_a.cuerpo, reenvio_a.cuerpo);
    let mut formas = Vec::new();
    for (cookie, alta) in [(&a, alta_a), (&b, alta_b)] {
        let ruta = format!("/api/intentos/{}", alta.cuerpo["intento_id"].as_str().unwrap());
        let pre = format!("{ruta}/encuestas/pre");
        let formulario = servidor.consultar("GET", &pre, Some(cookie), None).await;
        let forma = formulario.cuerpo["forma"].as_str().unwrap();
        formas.push(forma.to_owned());
        let responder = |ruta, clave, forma| {
            let (r1, r2) = if forma == "A" { (2, 3) } else { (1, 2) };
            servidor.peticion("PUT", ruta, Some(cookie), Some(clave), Some(json!({
                "estado":"respondido","respuestas":{"P1":3,"P2":3,"P3":3,"R1":r1,"R2":r2},
            })))
        };
        let acuse = responder(&pre, "pre", forma).await;
        assert_eq!(acuse.estado, StatusCode::OK);
        let guardada = servidor.consultar("GET", &pre, Some(cookie), None).await;
        assert_eq!(guardada.cuerpo["forma"], forma);
        assert_eq!(guardada.cuerpo["instante"], acuse.cuerpo["instante"]);
        let entrega = servidor.peticion("POST", &format!("{ruta}/entrega"), Some(cookie), Some("entrega"),
            Some(json!({"revision_esperada":1}))).await;
        assert_eq!(entrega.estado, StatusCode::CREATED);
        let post = format!("{ruta}/encuestas/post");
        let opuesta = if forma == "A" { "B" } else { "A" };
        assert_eq!(responder(&post, "post", opuesta).await.cuerpo["forma"], opuesta);
    }
    formas.sort();
    assert_eq!(formas, ["A", "B"]);
    let (docente, _) = servidor.entrar("docente-demo").await;
    let informe = servidor.consultar("GET", "/api/actividades/lr-ensayo/resultados-educativos", Some(&docente), None).await;
    assert_eq!(informe.cuerpo["pares_completos"], 2);
    let resultados = informe.cuerpo["grupos"][0]["razonamiento"].as_array().unwrap();
    assert_eq!(resultados.len(), 4);
    assert!(resultados.iter().all(|fila| fila["porcentaje"]["100"] == 1));
    assert_eq!(informe.cuerpo["grupos"][0]["cambio_razonamiento"]["0"], 2);
    println!("E15 {}", informe.cuerpo);
}
