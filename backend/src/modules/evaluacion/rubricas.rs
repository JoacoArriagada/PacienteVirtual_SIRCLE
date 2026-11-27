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
    let tx = db.transaction_with_behavior(TransactionBehavior::Immediate)?;
    let registro = cargar(&tx, archivo, "ensayo")?;
    tx.commit()?;
    Ok(registro)
}

pub(crate) fn cargar(db: &Connection, archivo: &Archivo, tipo: &str) -> Result<Registro> {
    let rubrica = &archivo.contenido;
    validar(rubrica)?;
    let contenido = serde_json::to_vec(rubrica)?;
    let sha256 = format!("{:x}", Sha256::digest(&contenido));
    let actividad_valida: bool = db.query_row(
        "SELECT EXISTS(SELECT 1 FROM actividades WHERE id=?1 AND escenario_id=?2 AND tipo=?3)",
        params![rubrica.actividad, rubrica.caso, tipo], |fila| fila.get(0),
    )?;
    ensure!(actividad_valida, "La rúbrica requiere una actividad del mismo caso y modalidad");
    if let Some(constancia) = &archivo.aprobacion {
        constancia.validar(db, &rubrica.actividad, tipo, &rubrica.codigo, &rubrica.version, &sha256)?;
    }

    let previa = db.query_row(
        "SELECT id, sha256, constancia FROM rubricas WHERE codigo=?1 AND version=?2",
        params![rubrica.codigo, rubrica.version],
        |fila| Ok((fila.get::<_, i64>(0)?, fila.get::<_, String>(1)?, fila.get::<_, Option<String>>(2)?)),
    ).optional()?;
    let (id, anterior) = if let Some((id, huella_previa, aprobacion)) = previa {
        ensure!(sha256 == huella_previa, "La versión ya existe con una huella distinta");
        (id, aprobacion)
    } else {
        let id = db.query_row(
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
            db.execute("UPDATE rubricas SET estado='aprobada', constancia=?1 WHERE id=?2",
                params![serde_json::to_string(constancia)?, id])?;
        }
    }
    let estado = if anterior.is_some() || archivo.aprobacion.is_some() { "aprobada" } else { "borrador" };
    Ok(Registro { id, codigo: rubrica.codigo.clone(), version: rubrica.version.clone(), sha256, estado: estado.into() })
}

pub(crate) fn borrador_demo() -> Result<Archivo> {
    Ok(serde_json::from_str(include_str!("../../../recursos/ensayo/rubricas/lr-v1.json"))?)
}

pub(crate) fn preparar_demo(db: &Connection) -> Result<()> {
    let activa: bool = db.query_row(
        "SELECT EXISTS(SELECT 1 FROM rubricas_activas_ensayo WHERE actividad_id='lr-ensayo')",
        [], |fila| fila.get(0),
    )?;
    if activa {
        return Ok(());
    }
    let mut archivo = borrador_demo()?;
    archivo.contenido.codigo = "lr-demostracion-local".into();
    archivo.aprobacion = Some(Constancia {
        tipo: "ensayo".into(),
        docente_id: "docente-demo".into(),
        fecha: "2026-11-25T00:00:00Z".into(),
        motivo: "Constancia técnica ficticia de la demostración local".into(),
        codigo: archivo.contenido.codigo.clone(),
        version: archivo.contenido.version.clone(),
        sha256: huella(&archivo.contenido)?,
    });
    let registro = cargar(db, &archivo, "ensayo")?;
    habilitar_ensayo(db, registro.id)
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

impl Constancia {
    pub(crate) fn validar(&self, db: &Connection, actividad: &str, tipo: &str, codigo: &str, version: &str, sha256: &str) -> Result<()> {
        ensure!(self.tipo == tipo && self.codigo == codigo && self.version == version
            && self.sha256 == sha256 && !self.motivo.trim().is_empty(), "La constancia no corresponde a la modalidad, versión y huella");
        let autorizada: bool = db.query_row(
            "SELECT strftime('%Y-%m-%dT%H:%M:%SZ',?1) IS ?1 AND EXISTS(
             SELECT 1 FROM asignaciones a JOIN identidades i ON i.id=a.docente_id
             WHERE a.actividad_id=?2 AND a.docente_id=?3 AND i.rol='docente')",
            params![self.fecha, actividad, self.docente_id], |fila| fila.get(0),
        )?;
        ensure!(autorizada, "La constancia requiere fecha UTC y docente responsable");
        Ok(())
    }
}
