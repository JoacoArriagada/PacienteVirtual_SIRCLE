use std::{fs, path::Path};

use anyhow::{Context, Result, ensure};
use rusqlite::{Transaction, TransactionBehavior, params};
use sha2::{Digest, Sha256};

use crate::{modules::{acceso::actividades, entrevista::hechos, intentos::intento, valoracion::{hallazgos, mediciones}}, sqlite};
use crate::modules::examenes::{medios, ordenes};
use crate::agentes::{dan_sparks::perfil as dan, nannu::perfil as nannu};

const ROLES: &[&str] = &["environment_controller", "evaluator", "examinee", "sp_actor"];

struct Escenario {
    id: &'static str,
    paciente: &'static str,
    carpeta: &'static str,
    documento: &'static str,
    informes: &'static [(&'static str, &'static str)],
}

const ESCENARIOS: &[Escenario] = &[
    Escenario {
        id: "P01",
        paciente: "LR",
        carpeta: "mededportal_10181/scenario1",
        documento: "Case A-Asthma.md",
        informes: &[],
    },
    Escenario {
        id: "P02",
        paciente: "Nannu",
        carpeta: "mep_2374-8265.10924-s001/scenario3",
        documento: "C. Pneumonia with Septic Shock.md",
        informes: &[],
    },
    Escenario {
        id: "P03",
        paciente: "Dan Sparks",
        carpeta: "mededportal_3171/scenario1",
        documento: "Pulmonary Embolism Case Description.md",
        informes: &[
            ("environment_controller", "Pulmonary Embolism EKG.md"),
            ("evaluator", "Pulmonary Embolism EKG.md"),
        ],
    },
];

struct Fuente {
    rol: &'static str,
    ruta: String,
    contenido: Vec<u8>,
    sha256: String,
}

pub struct FilaInventario {
    pub escenario: &'static str,
    pub fuente_id: i64,
    pub revision_id: i64,
    pub version_id: i64,
    pub version: String,
    pub rol: &'static str,
    pub ruta: String,
    pub sha256: String,
}

pub fn preparar(db: &Path, datos: &Path) -> Result<Vec<FilaInventario>> {
    let catalogo = datos.join("medsp1000");
    let entradas = ESCENARIOS
        .iter()
        .map(|escenario| leer_fuentes(escenario, &catalogo).map(|fuentes| (escenario, fuentes)))
        .collect::<Result<Vec<_>>>()?;

    if let Some(carpeta) = db.parent().filter(|ruta| !ruta.as_os_str().is_empty()) {
        fs::create_dir_all(carpeta).with_context(|| format!("No se pudo crear {}", carpeta.display()))?;
    }
    let mut conexion = sqlite::abrir(db).with_context(|| format!("No se pudo abrir {}", db.display()))?;
    sqlite::migrar(&mut conexion).context("No se aplicaron las migraciones")?;

    let mut inventario = Vec::new();
    let transaccion = conexion.transaction_with_behavior(TransactionBehavior::Immediate)?;
    for (escenario, fuentes) in entradas {
        guardar_escenario(&transaccion, escenario, fuentes, &mut inventario)
            .with_context(|| format!("No se pudo guardar {}", escenario.id))?;
    }
    actividades::preparar_ensayo(&transaccion).context("No se pudo preparar la actividad de ensayo")?;
    let original_lr = inventario.iter().find(|fila| fila.escenario == "P01").context("Falta el caso P01")?;
    intento::preparar_ensayo(&transaccion, "lr-ensayo", original_lr.version_id).context("No se configuró el ensayo de LR")?;
    mediciones::preparar_presion_lr(&transaccion).context("No se preparó la toma de presión de ensayo")?;
    mediciones::preparar_signos_lr(&transaccion).context("No se prepararon los signos vitales de ensayo")?;
    mediciones::preparar_peso_lr(&transaccion).context("No se preparó el peso de ensayo")?;
    hallazgos::preparar_exploracion_lr(&transaccion).context("No se prepararon las exploraciones de ensayo")?;
    ordenes::preparar_ecg_lr(&transaccion).context("No se preparó el ECG inicial de ensayo")?;
    hechos::preparar_lr(&transaccion).context("No se prepararon los hechos de ensayo de LR")?;
    for (caso, actividad, nombre) in [
        (nannu::ESCENARIO, nannu::ACTIVIDAD, "Práctica técnica de Nannu"),
        (dan::ESCENARIO, dan::ACTIVIDAD, "Práctica técnica de Dan Sparks"),
    ] {
        let original = inventario.iter().find(|fila| fila.escenario == caso).with_context(|| format!("Falta el caso {caso}"))?;
        transaccion.execute(
            "INSERT INTO actividades VALUES (?1,?2,?3,'ensayo',0) ON CONFLICT(id) DO NOTHING",
            params![actividad, caso, nombre],
        )?;
        intento::preparar_ensayo(&transaccion, actividad, original.version_id)
            .with_context(|| format!("No se configuró el ensayo de {caso}"))?;
    }
    hechos::preparar_nannu(&transaccion).context("No se prepararon los hechos de ensayo de Nannu")?;
    hechos::preparar_dan(&transaccion).context("No se prepararon los hechos de ensayo de Dan")?;
    medios::preparar_dan(&transaccion, &catalogo).context("No se catalogó la disponibilidad del ECG de Dan")?;
    transaccion.commit()?;
    Ok(inventario)
}

fn leer_fuentes(escenario: &Escenario, catalogo: &Path) -> Result<Vec<Fuente>> {
    let mut fuentes = ROLES
        .iter()
        .map(|rol| (*rol, escenario.documento))
        .chain(escenario.informes.iter().copied())
        .map(|(rol, documento)| {
            let ruta = format!("{}/{rol}/{documento}", escenario.carpeta);
            let archivo = catalogo.join(&ruta);
            let contenido = fs::read(&archivo)
                .with_context(|| format!("No se pudo leer {}", archivo.display()))?;
            let texto = std::str::from_utf8(&contenido)
                .with_context(|| format!("{} no contiene texto UTF-8 válido", archivo.display()))?;
            ensure!(!texto.trim().is_empty(), "{} está vacío", archivo.display());
            let sha256 = format!("{:x}", Sha256::digest(&contenido));
            Ok(Fuente { rol, ruta, contenido, sha256 })
        })
        .collect::<Result<Vec<_>>>()?;
    fuentes.sort_unstable_by(|izquierda, derecha| izquierda.ruta.cmp(&derecha.ruta));
    Ok(fuentes)
}

fn guardar_escenario(
    db: &Transaction<'_>,
    escenario: &Escenario,
    fuentes: Vec<Fuente>,
    inventario: &mut Vec<FilaInventario>,
) -> Result<()> {
    db.execute(
        "INSERT INTO escenarios (id, paciente) VALUES (?1, ?2) ON CONFLICT(id) DO NOTHING",
        params![escenario.id, escenario.paciente],
    )?;

    let mut huella = Sha256::new();
    for fuente in &fuentes {
        huella.update(fuente.ruta.as_bytes());
        huella.update(b"\0");
        huella.update(fuente.sha256.as_bytes());
    }
    let version = format!("originales-{:x}", huella.finalize());
    db.execute(
        "INSERT INTO versiones_clinicas (escenario_id, version, estado) VALUES (?1, ?2, 'pendiente')
         ON CONFLICT(escenario_id, version) DO NOTHING",
        params![escenario.id, version],
    )?;
    let version_id: i64 = db.query_row(
        "SELECT id FROM versiones_clinicas WHERE escenario_id=?1 AND version=?2",
        params![escenario.id, version],
        |fila| fila.get(0),
    )?;

    for fuente in fuentes {
        let (fuente_id, revision_id) = guardar_fuente(db, escenario.id, &fuente)
            .with_context(|| format!("No se pudo guardar {}", fuente.ruta))?;
        db.execute(
            "INSERT INTO versiones_fuentes VALUES (?1, ?2, ?3, ?4)
             ON CONFLICT(version_clinica_id, fuente_id) DO NOTHING",
            params![escenario.id, version_id, fuente_id, revision_id],
        )?;
        inventario.push(FilaInventario {
            escenario: escenario.id,
            fuente_id,
            revision_id,
            version_id,
            version: version.clone(),
            rol: fuente.rol,
            ruta: fuente.ruta,
            sha256: fuente.sha256,
        });
    }
    Ok(())
}

fn guardar_fuente(db: &Transaction<'_>, escenario: &str, fuente: &Fuente) -> Result<(i64, i64)> {
    let clave = params![escenario, fuente.rol, fuente.ruta];
    db.execute(
        "INSERT INTO fuentes (escenario_id, rol, ruta) VALUES (?1, ?2, ?3)
         ON CONFLICT(escenario_id, rol, ruta) DO NOTHING",
        clave,
    )?;
    let fuente_id: i64 = db.query_row(
        "SELECT id FROM fuentes WHERE escenario_id=?1 AND rol=?2 AND ruta=?3",
        clave,
        |fila| fila.get(0),
    )?;
    db.execute(
        "INSERT INTO revisiones_fuente (fuente_id, sha256, contenido) VALUES (?1, ?2, ?3)
         ON CONFLICT(fuente_id, sha256) DO NOTHING",
        params![fuente_id, fuente.sha256, fuente.contenido],
    )?;
    let revision_id: i64 = db.query_row(
        "SELECT id FROM revisiones_fuente WHERE fuente_id=?1 AND sha256=?2",
        params![fuente_id, fuente.sha256],
        |fila| fila.get(0),
    )?;
    Ok((fuente_id, revision_id))
}
