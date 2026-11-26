mod fixtures;

use fixtures::{Servidor, intento_lr};
use sircle::{modules::{entrevista::hechos, intentos::intento}, sqlite};

#[tokio::test]
async fn los_dos_candidatos_pertenecen_al_actor_y_a_la_fase_del_intento() {
    let servidor = Servidor::nuevo();
    let (cookie, _) = servidor.entrar(intento_lr::ESTUDIANTE_LR).await;
    let alta = intento_lr::alta(&servidor, &cookie).await;
    let id = alta.cuerpo["intento_id"].as_str().unwrap();
    let db = sqlite::abrir_existente(&servidor.db).unwrap();
    let previo = intento::recuperar(&db, id, intento_lr::ESTUDIANTE_LR).unwrap();
    assert!(hechos::candidatos_lr(&db, &previo).unwrap().is_empty());
    intento_lr::resolver_pre(&servidor, id);
    let intento = intento::recuperar(&db, id, intento_lr::ESTUDIANTE_LR).unwrap();
    let candidatos = hechos::candidatos_lr(&db, &intento).unwrap();
    assert_eq!(candidatos.len(), 2);
    for hecho in &candidatos {
        assert_eq!(hecho.escenario, "P01");
        assert_eq!(hecho.version, intento.version_caso);
        assert_eq!(hecho.fase, "inicial");
        assert_eq!(hecho.hablante, "LR");
        assert!(!hecho.condicion.is_empty());
        assert!(hecho.fuente.ends_with("sp_actor/Case A-Asthma.md"));
        let original = std::fs::read_to_string(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../datos/medsp1000").join(&hecho.fuente),
        ).unwrap();
        assert!(original.lines().nth(hecho.linea as usize - 1).unwrap().ends_with(&hecho.pasaje));
        match hecho.id.as_str() {
            "lr-inicio" => {
                assert!(hecho.respuesta.contains("hace dos días"));
                assert_eq!(hecho.linea, 7);
            }
            "lr-antecedentes-familiares" => {
                assert!(hecho.respuesta.contains("fui adoptada"));
                assert_eq!(hecho.linea, 17);
            }
            id => panic!("Candidato inesperado: {id}"),
        }
        println!("EVIDENCIA {}", serde_json::to_string(hecho).unwrap());
    }
}
