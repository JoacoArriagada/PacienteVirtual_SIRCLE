mod fixtures;

use fixtures::Servidor;
use serde_json::json;
use sha2::{Digest, Sha256};
use sircle::{modules::evaluacion::rubricas::{self, Archivo, Constancia}, sqlite};

const ARCHIVO: &str = include_str!("fixtures/rubricas/lr-v1.json");

#[test]
fn carga_borrador_y_constancia_de_ensayo_sin_sustituir_version() {
    let servidor = Servidor::nuevo();
    let mut db = sqlite::abrir_existente(&servidor.db).unwrap();
    let mut archivo: Archivo = serde_json::from_str(ARCHIVO).unwrap();
    let borrador = rubricas::cargar_ensayo(&mut db, &archivo).unwrap();
    assert_eq!(borrador.estado, "borrador");
    assert_eq!(borrador.sha256, rubricas::huella(&archivo.contenido).unwrap());
    let antes: Option<String> = db.query_row("SELECT constancia FROM rubricas WHERE id=?1", [borrador.id], |fila| fila.get(0)).unwrap();
    assert!(antes.is_none());

    let constancia = Constancia {
        tipo: "ensayo".into(), docente_id: "docente-demo".into(), fecha: "2026-11-26T00:00:00Z".into(),
        motivo: "Constancia ficticia de calibración para la regresión T054".into(),
        codigo: borrador.codigo.clone(), version: borrador.version.clone(), sha256: borrador.sha256.clone(),
    };
    archivo.aprobacion = Some(constancia.clone());
    let aprobada = rubricas::cargar_ensayo(&mut db, &archivo).unwrap();
    assert_eq!(aprobada.estado, "aprobada");
    assert_eq!(borrador.id, aprobada.id);
    assert_eq!(rubricas::cargar_ensayo(&mut db, &archivo).unwrap(), aprobada);
    let (contenido, aprobacion): (Vec<u8>, String) = db.query_row("SELECT contenido,constancia FROM rubricas WHERE id=?1",
        [aprobada.id], |fila| Ok((fila.get(0)?, fila.get(1)?))).unwrap();
    assert_eq!(format!("{:x}", Sha256::digest(&contenido)), aprobada.sha256);
    assert_eq!(serde_json::from_str::<Constancia>(&aprobacion).unwrap(), constancia);
    assert_eq!(serde_json::to_vec(&archivo.contenido).unwrap(), contenido);

    let mut alterada = archivo.clone();
    alterada.aprobacion = None;
    alterada.contenido.criterios[0].peso=25;
    alterada.contenido.criterios[1].peso=15;
    let error = rubricas::cargar_ensayo(&mut db, &alterada).unwrap_err();
    assert!(error.to_string().contains("huella distinta"));
    let total: i64 = db.query_row("SELECT count(*) FROM rubricas", [], |fila| fila.get(0)).unwrap();
    assert_eq!(total, 1);
    let retenida: Vec<u8> = db.query_row("SELECT contenido FROM rubricas WHERE id=?1", [aprobada.id], |fila| fila.get(0)).unwrap();
    assert_eq!(retenida, contenido);
    println!("EVIDENCIA {}", json!({"borrador":borrador,"aprobada":aprobada,"constancia":constancia,
        "criterios":archivo.contenido.criterios,"regla":archivo.contenido.regla,
        "error_contenido_distinto":error.to_string(),"rubricas_finales":total}));
}

#[test]
fn rechaza_constancia_desvinculada_o_de_docente_ajeno() {
    let servidor = Servidor::nuevo();
    let mut db = sqlite::abrir_existente(&servidor.db).unwrap();
    db.execute("INSERT INTO identidades VALUES ('docente-otro','docente')", []).unwrap();
    let mut archivo: Archivo = serde_json::from_str(ARCHIVO).unwrap();
    let sha256 = rubricas::huella(&archivo.contenido).unwrap();
    let base = Constancia { tipo:"ensayo".into(), docente_id:"docente-demo".into(), fecha:"2026-11-26T00:00:00Z".into(),
        motivo:"Constancia ficticia".into(), codigo:archivo.contenido.codigo.clone(),
        version:archivo.contenido.version.clone(), sha256 };
    for constancia in [
        Constancia { fecha:"ayer".into(), ..base.clone() },
        Constancia { docente_id:"docente-otro".into(), ..base.clone() },
        Constancia { sha256:"9".repeat(64), ..base.clone() },
        Constancia { tipo:"formal".into(), ..base },
    ] {
        archivo.aprobacion = Some(constancia);
        assert!(rubricas::cargar_ensayo(&mut db, &archivo).is_err());
        assert_eq!(db.query_row("SELECT count(*) FROM rubricas", [], |fila| fila.get::<_, i64>(0)).unwrap(), 0);
    }
}
