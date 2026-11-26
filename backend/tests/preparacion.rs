use std::{fs, path::Path, process::Command};

use sircle::sqlite;

const FUENTE: &str = "medsp1000/mededportal_3171/scenario1/evaluator/Pulmonary Embolism EKG.md";
const LR_ACTOR: &str = "medsp1000/mededportal_10181/scenario1/sp_actor/Case A-Asthma.md";
const LR_CONTROLADOR: &str = "medsp1000/mededportal_10181/scenario1/environment_controller/Case A-Asthma.md";

#[test]
fn una_preparacion_invalida_conserva_el_catalogo_anterior() {
    let temporal = tempfile::tempdir().unwrap();
    let datos = temporal.path().join("datos");
    let db = temporal.path().join("prueba.sqlite");
    copiar(
        &Path::new(env!("CARGO_MANIFEST_DIR")).join("../datos/medsp1000"),
        &datos.join("medsp1000"),
    );

    let preparar = || {
        Command::new(env!("CARGO_BIN_EXE_sircle"))
            .arg("preparar")
            .arg("--db").arg(&db)
            .arg("--fuentes").arg(&datos)
            .output().unwrap()
    };

    let primera = preparar();
    assert!(primera.status.success(), "{}", String::from_utf8_lossy(&primera.stderr));
    let antes = inventario(&db);
    assert_eq!(antes.len(), 14);
    registrar("original", &antes);

    let segunda = preparar();
    assert!(segunda.status.success(), "{}", String::from_utf8_lossy(&segunda.stderr));
    assert_eq!(primera.stdout, segunda.stdout);
    let repeticion = inventario(&db);
    assert_eq!(antes, repeticion);
    registrar("repeticion", &repeticion);

    let archivo = datos.join(FUENTE);
    let contenido = fs::read(&archivo).unwrap();
    fs::remove_file(&archivo).unwrap();
    for (estado, motivo) in [("ausente", "No se pudo leer"), ("invalida", "no contiene texto UTF-8 válido")] {
        let fallo = preparar();
        assert!(!fallo.status.success());
        let error = String::from_utf8(fallo.stderr).unwrap();
        assert!(error.replace('\\', "/").contains(FUENTE) && error.contains(motivo), "{error}");
        println!("ERROR\t{estado}\t{}", error.lines().next().unwrap());

        let despues = inventario(&db);
        assert_eq!(antes, despues);
        registrar(estado, &despues);
        if estado == "ausente" {
            fs::write(&archivo, [0xff]).unwrap();
        }
    }
    fs::write(&archivo, contenido).unwrap();

    for (fuente, pasaje, motivo) in [
        (LR_ACTOR, "Vitals: BP 148/85 HR 102", "No se encontró el triaje inicial"),
        (LR_CONTROLADOR, "initial EKG (sinus tachycardia)", "No se encontró el pasaje del ECG inicial"),
        (LR_ACTOR, "Family Hx: unknown, patient was adopted.", "Falta el pasaje del hecho"),
    ] {
        let ruta = datos.join(fuente);
        let original = fs::read_to_string(&ruta).unwrap();
        assert!(original.contains(pasaje), "{fuente}");
        fs::write(&ruta, original.replace(pasaje, "Pasaje omitido para la prueba")).unwrap();
        let fallo = preparar();
        assert!(!fallo.status.success(), "Se activó una versión incompleta de {fuente}");
        let error = String::from_utf8(fallo.stderr).unwrap();
        assert!(error.replace('\\', "/").contains(fuente) && error.contains(motivo), "{error}");
        assert_eq!(inventario(&db), antes);
        fs::write(ruta, original).unwrap();
    }
}

fn copiar(origen: &Path, destino: &Path) {
    fs::create_dir_all(destino).unwrap();
    for entrada in fs::read_dir(origen).unwrap() {
        let entrada = entrada.unwrap();
        let destino = destino.join(entrada.file_name());
        if entrada.file_type().unwrap().is_dir() {
            copiar(&entrada.path(), &destino);
        } else {
            fs::copy(entrada.path(), destino).unwrap();
        }
    }
}

fn inventario(ruta: &Path) -> Vec<String> {
    let conexion = sqlite::abrir_existente(ruta).unwrap();
    for (tabla, esperado) in [
        ("escenarios", 3), ("fuentes", 14), ("revisiones_fuente", 14),
        ("versiones_clinicas WHERE estado='pendiente'", 3),
        ("versiones_fuentes JOIN versiones_clinicas v ON v.id=version_clinica_id WHERE v.estado='pendiente'", 14),
    ] {
        let total: i64 = conexion
            .query_row(&format!("SELECT count(*) FROM {tabla}"), [], |fila| fila.get(0))
            .unwrap();
        assert_eq!(total, esperado, "{tabla}");
    }
    let mut consulta = conexion.prepare(
        "SELECT f.escenario_id || char(9) || f.id || char(9) || r.id || char(9) ||
                v.id || char(9) || v.version || char(9) || f.rol || char(9) ||
                f.ruta || char(9) || r.sha256 || char(9) || v.estado
         FROM versiones_fuentes vf
         JOIN fuentes f ON f.id = vf.fuente_id
         JOIN revisiones_fuente r ON r.id = vf.revision_fuente_id
         JOIN versiones_clinicas v ON v.id = vf.version_clinica_id
         WHERE v.estado='pendiente' ORDER BY f.escenario_id, f.ruta",
    ).unwrap();
    consulta.query_map([], |fila| fila.get(0)).unwrap().collect::<Result<_, _>>().unwrap()
}

fn registrar(etapa: &str, inventario: &[String]) {
    for fila in inventario {
        println!("INVENTARIO\t{etapa}\t{fila}");
    }
}
