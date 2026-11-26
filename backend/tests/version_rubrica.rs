mod fixtures;

use axum::http::StatusCode;
use fixtures::{Servidor, intento_lr, versiones_rubrica::{self, ESTUDIANTE_B, V1, V2}};
use serde_json::json;
use sircle::{
    app::{estado::Estado, router::router},
    modules::evaluacion::rubricas::{self, Archivo, Rubrica},
    sqlite,
};

#[tokio::test]
async fn reabre_a_con_v1_y_b_con_v2_aunque_cambie_la_rubrica_configurada() {
    let mut servidor = Servidor::nuevo();
    let mut db = sqlite::abrir_existente(&servidor.db).unwrap();
    versiones_rubrica::asignar_segundo_estudiante(&db);
    let v1 = versiones_rubrica::cargar(&mut db, V1);
    let activar_v1 = versiones_rubrica::habilitar_por_cli(&servidor, v1.id);
    assert!(activar_v1.status.success(), "{activar_v1:?}");
    let a = versiones_rubrica::abrir(&servidor, intento_lr::ESTUDIANTE_LR, v1).await;

    let archivo_v2: Archivo = serde_json::from_str(V2).unwrap();
    let borrador = rubricas::cargar_ensayo(&mut db, &archivo_v2).unwrap();
    let denegado = versiones_rubrica::habilitar_por_cli(&servidor, borrador.id);
    assert!(!denegado.status.success(), "{denegado:?}");
    assert_eq!(db.query_row("SELECT rubrica_id FROM rubricas_activas_ensayo WHERE actividad_id='lr-ensayo'",
        [], |fila| fila.get::<_, i64>(0)).unwrap(), a.rubrica.id);
    let v2 = versiones_rubrica::cargar(&mut db, V2);
    assert_eq!(v2.id, borrador.id);
    assert_ne!(a.rubrica.sha256, v2.sha256);
    let activar_v2 = versiones_rubrica::habilitar_por_cli(&servidor, v2.id);
    assert!(activar_v2.status.success(), "{activar_v2:?}");
    let b = versiones_rubrica::abrir(&servidor, ESTUDIANTE_B, v2).await;

    servidor.app = router(Estado::con_base(servidor.db.clone()).unwrap());
    let mut resultados = Vec::new();
    for (intento, estudiante, peso) in [(&a, intento_lr::ESTUDIANTE_LR, 20), (&b, ESTUDIANTE_B, 30)] {
        let (cookie, _) = servidor.entrar(estudiante).await;
        let vista = servidor.consultar("GET", &format!("/api/intentos/{}", intento.id), Some(&cookie), None).await;
        assert_eq!(vista.estado, StatusCode::OK);
        let reenvio = intento_lr::alta(&servidor, &cookie).await;
        assert_eq!(reenvio.estado, StatusCode::OK);
        assert_eq!(vista.cuerpo, reenvio.cuerpo);
        assert_eq!(vista.cuerpo["intento_id"], intento.id);
        assert_eq!(vista.cuerpo["estudiante_id"], estudiante);
        assert_eq!(vista.cuerpo["rubrica_id"], intento.rubrica.id);
        assert_eq!(vista.cuerpo["version_rubrica"], intento.rubrica.version);
        assert_eq!(vista.cuerpo["rubrica_sha256"], intento.rubrica.sha256);
        assert!(vista.cuerpo.get("criterios").is_none());
        let contenido: Vec<u8> = db.query_row(
            "SELECT r.contenido FROM intentos i JOIN rubricas r ON r.id=i.rubrica_id
             AND r.version=i.version_rubrica AND r.sha256=i.rubrica_sha256 WHERE i.id=?1",
            [&intento.id], |fila| fila.get(0),
        ).unwrap();
        let privada: Rubrica = serde_json::from_slice(&contenido).unwrap();
        assert_eq!(rubricas::huella(&privada).unwrap(), intento.rubrica.sha256);
        assert_eq!(privada.criterios.iter().find(|c| c.id == "entrevista").unwrap().peso, peso);
        let ajena = servidor.consultar("GET", &format!("/api/intentos/{}", intento.id),
            Some(if intento.id == a.id { &b.cookie } else { &a.cookie }), None).await;
        assert_eq!(ajena.estado, StatusCode::FORBIDDEN);
        resultados.push(json!({"intento":intento.id,"estudiante":estudiante,"rubrica":intento.rubrica,
            "peso_entrevista":peso,"reabierto_http":vista.estado.as_u16(),"reenvio_http":reenvio.estado.as_u16(),
            "lectura_ajena_http":ajena.estado.as_u16()}));
    }
    let copia = std::env::var("SIRCLE_T055_DB").ok();
    if let Some(ruta) = &copia {
        db.execute("VACUUM INTO ?1", [ruta]).unwrap();
    }
    println!("EVIDENCIA {}", json!({"intentos":resultados,"copia":copia,
        "activar_borrador_codigo":denegado.status.code(),
        "activar_borrador_error":String::from_utf8_lossy(&denegado.stderr).trim()}));
}
