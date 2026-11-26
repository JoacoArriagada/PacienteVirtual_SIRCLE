use std::{collections::HashSet, fs, path::Path};

use anyhow::{Context, Result, ensure};
use rusqlite::{Connection, OptionalExtension, TransactionBehavior, params};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::agentes::evaluador::perfil::{NIVEL, TipoEvidencia};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Archivo {
    pub contenido: Rubrica,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aprobacion: Option<Constancia>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Rubrica {
    pub codigo: String,
    pub version: String,
    pub actividad: String,
    pub caso: String,
    pub nivel: String,
    pub criterios: Vec<Criterio>,
    pub regla: Regla,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Criterio {
    pub id: String,
    pub ambito: Ambito,
    pub peso: u32,
    pub evidencias: Vec<TipoEvidencia>,
    pub expectativa: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hipotesis_posicion: Option<usize>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Ambito {
    Entrevista,
    Valoracion,
    Decisiones,
    UsoDeEvidencias,
    Hipotesis,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Regla {
    SumaPonderadaNormalizada,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Constancia {
    pub tipo: String,
    pub docente_id: String,
    pub fecha: String,
    pub motivo: String,
    pub codigo: String,
    pub version: String,
    pub sha256: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Registro {
    pub id: i64,
    pub codigo: String,
    pub version: String,
    pub sha256: String,
    pub estado: String,
}

pub fn huella(rubrica: &Rubrica) -> Result<String> {
    Ok(format!("{:x}", Sha256::digest(serde_json::to_vec(rubrica)?)))
}

pub fn cargar_archivo(db: &mut Connection, ruta: &Path) -> Result<Registro> {
    let bytes = fs::read(ruta).with_context(|| format!("No se pudo leer {}", ruta.display()))?;
    let archivo = serde_json::from_slice::<Archivo>(&bytes).context("Formato de rúbrica inválido")?;
    cargar_ensayo(db, &archivo)
}

pub fn cargar_ensayo(db: &mut Connection, archivo: &Archivo) -> Result<Registro> {
    let rubrica = &archivo.contenido;
    validar(rubrica)?;
    let contenido = serde_json::to_vec(rubrica)?;
    let sha256 = format!("{:x}", Sha256::digest(&contenido));
    let tx = db.transaction_with_behavior(TransactionBehavior::Immediate)?;
    let actividad_valida: bool = tx.query_row(
        "SELECT EXISTS(SELECT 1 FROM actividades WHERE id=?1 AND escenario_id=?2 AND tipo='ensayo')",
        params![rubrica.actividad, rubrica.caso], |fila| fila.get(0),
    )?;
    ensure!(actividad_valida, "La rúbrica requiere una actividad de ensayo del mismo caso");
    if let Some(constancia) = &archivo.aprobacion {
        validar_constancia(&tx, rubrica, &sha256, constancia)?;
    }

    let previa = tx.query_row(
        "SELECT id, sha256, constancia FROM rubricas WHERE codigo=?1 AND version=?2",
        params![rubrica.codigo, rubrica.version],
        |fila| Ok((fila.get::<_, i64>(0)?, fila.get::<_, String>(1)?, fila.get::<_, Option<String>>(2)?)),
    ).optional()?;
    let (id, anterior) = if let Some((id, huella_previa, aprobacion)) = previa {
        ensure!(sha256 == huella_previa, "La versión ya existe con una huella distinta");
        (id, aprobacion)
    } else {
        let id = tx.query_row(
            "INSERT INTO rubricas (codigo,version,actividad_id,caso,nivel,sha256,contenido,estado)
             VALUES (?1,?2,?3,?4,?5,?6,?7,'borrador') RETURNING id",
            params![rubrica.codigo, rubrica.version, rubrica.actividad, rubrica.caso, rubrica.nivel, sha256, contenido],
            |fila| fila.get(0),
        )?;
        (id, None)
    };
    if let Some(constancia) = &archivo.aprobacion {
        if let Some(anterior) = &anterior {
            ensure!(&serde_json::from_str::<Constancia>(anterior)? == constancia, "La versión ya tiene otra constancia");
        } else {
            tx.execute("UPDATE rubricas SET estado='aprobada', constancia=?1 WHERE id=?2",
                params![serde_json::to_string(constancia)?, id])?;
        }
    }
    let estado = if anterior.is_some() || archivo.aprobacion.is_some() { "aprobada" } else { "borrador" };
    tx.commit()?;
    Ok(Registro { id, codigo: rubrica.codigo.clone(), version: rubrica.version.clone(), sha256, estado: estado.into() })
}

pub fn habilitar_ensayo(db: &Connection, id: i64) -> Result<()> {
    let filas = db.execute(
        "INSERT INTO rubricas_activas_ensayo (actividad_id,rubrica_id)
         SELECT r.actividad_id,r.id FROM rubricas r
         JOIN configuraciones_ensayo c ON c.actividad_id=r.actividad_id AND c.escenario_id=r.caso
         JOIN actividades a ON a.id=c.actividad_id AND a.tipo='ensayo'
         WHERE r.id=?1 AND r.estado='aprobada' AND json_extract(r.constancia,'$.tipo')='ensayo'
         ON CONFLICT(actividad_id) DO UPDATE SET rubrica_id=excluded.rubrica_id",
        [id],
    )?;
    ensure!(filas == 1, "Solo se puede habilitar una rúbrica aprobada para una actividad de ensayo preparada");
    Ok(())
}

fn validar(rubrica: &Rubrica) -> Result<()> {
    ensure!(rubrica.nivel == NIVEL && [&rubrica.codigo, &rubrica.version, &rubrica.actividad, &rubrica.caso]
        .into_iter().all(|texto| !texto.trim().is_empty()), "Indica el caso, la actividad y el nivel de pregrado avanzado");
    let mut ids = HashSet::new();
    let mut ambitos = HashSet::new();
    let mut posiciones = HashSet::new();
    let mut peso = 0_u64;
    for criterio in &rubrica.criterios {
        ensure!(!criterio.id.trim().is_empty() && ids.insert(&criterio.id) && !criterio.expectativa.trim().is_empty()
            && criterio.peso > 0 && !criterio.evidencias.is_empty(), "Cada criterio requiere ID, expectativa, peso y evidencia");
        if let Some(posicion) = criterio.hipotesis_posicion {
            ensure!(criterio.ambito == Ambito::Hipotesis && (1..=12).contains(&posicion) && posiciones.insert(posicion),
                "La posición del criterio debe identificar una hipótesis distinta");
        }
        ambitos.insert(criterio.ambito);
        peso += u64::from(criterio.peso);
    }
    ensure!(ambitos.len() == 5 && peso == 100, "Los cinco ámbitos deben sumar un peso de 100");
    Ok(())
}

fn validar_constancia(db: &Connection, rubrica: &Rubrica, sha256: &str, constancia: &Constancia) -> Result<()> {
    ensure!(constancia.tipo == "ensayo" && constancia.codigo == rubrica.codigo && constancia.version == rubrica.version
        && constancia.sha256 == sha256 && !constancia.motivo.trim().is_empty(), "La constancia de ensayo no corresponde a la versión y huella");
    let autorizada: bool = db.query_row(
        "SELECT strftime('%Y-%m-%dT%H:%M:%SZ',?1) IS ?1 AND EXISTS(
             SELECT 1 FROM asignaciones a JOIN identidades i ON i.id=a.docente_id
             WHERE a.actividad_id=?2 AND a.docente_id=?3 AND i.rol='docente')",
        params![constancia.fecha, rubrica.actividad, constancia.docente_id], |fila| fila.get(0),
    )?;
    ensure!(autorizada, "La constancia requiere fecha UTC y docente responsable");
    Ok(())
}
