use std::{fs, path::Path};

use anyhow::{Context, Result, ensure};
use rusqlite::{Connection, Transaction, params};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};

use crate::modules::{
    encuestas::instrumento::{self, Instrumento},
    evaluacion::rubricas::{self, Constancia},
};

const ACTIVIDAD: &str = "lr-formal";
const CODIGO_CLINICO: &str = "lr-clinica";
const CODIGO_EDUCATIVO: &str = "lr-instrumento";

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Documento {
    clinica: Material<Clinica>,
    instrumento: Material<Instrumento>,
    version_rubrica: String,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Material<T> {
    contenido: T,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    aprobacion: Option<Constancia>,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Clinica {
    version: String,
    base: String,
    ficha: FichaLr,
    catalogo: Value,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct FichaLr {
    pub paciente: String,
    pub edad: u32,
    pub motivo: String,
    pub contexto: String,
    pub fuente: String,
    pub indicaciones: Vec<String>,
}

impl FichaLr {
    pub fn inicial(contexto: &str, fuente: &str) -> Self {
        Self {
            paciente: "LR".into(), edad: 34, motivo: "Falta de aire".into(),
            contexto: contexto.into(), fuente: fuente.into(),
            indicaciones: vec![
                "Entrevista a LR y registra las mediciones que realices.".into(),
                "Vincula los hallazgos que sustenten tu hipótesis.".into(),
            ],
        }
    }
}

pub(crate) fn preparar_asignacion(db: &Connection) -> Result<()> {
    db.execute_batch(
        "INSERT INTO actividades VALUES ('lr-formal','P01','Práctica clínica de LR','formal',0)
            ON CONFLICT(id) DO NOTHING;
         INSERT INTO asignaciones VALUES ('lr-formal','estudiante-lr','docente-demo')
            ON CONFLICT(actividad_id,estudiante_id) DO NOTHING;",
    )?;
    Ok(())
}

pub fn borrador(db: &Connection, version: String, version_instrumento: String, version_rubrica: String) -> Result<Value> {
    let (id, base): (i64, String) = db.query_row(
        "SELECT v.id,v.version FROM configuraciones_ensayo c JOIN versiones_clinicas v ON v.id=c.version_caso_id
         WHERE c.actividad_id='lr-ensayo'",
        [], |fila| Ok((fila.get(0)?, fila.get(1)?)),
    ).context("Primero prepara las fuentes de LR")?;
    let clinica = Clinica { version, base, ficha: FichaLr::inicial("Servicio de urgencias.", "Triaje inicial de LR"), catalogo: catalogo(db, id)? };
    let mut instrumento = Instrumento::ensayo()?;
    instrumento.version = version_instrumento;
    let mut rubrica = rubricas::borrador_demo()?;
    rubrica.contenido.codigo = "lr-pregrado-formal".into();
    rubrica.contenido.actividad = ACTIVIDAD.into();
    rubrica.contenido.version = version_rubrica.clone();
    let actividad = Documento {
        clinica: Material { contenido: clinica, aprobacion: None },
        instrumento: Material { contenido: instrumento, aprobacion: None },
        version_rubrica,
    };
    Ok(json!({ "actividad": actividad, "rubrica": rubrica, "huellas": huellas_de(&actividad, &rubrica)? }))
}

pub fn huellas(archivo: &Path) -> Result<Value> {
    let (documento, rubrica) = leer_archivos(archivo)?;
    huellas_de(&documento, &rubrica)
}

fn leer_archivos(archivo: &Path) -> Result<(Documento, rubricas::Archivo)> {
    let documento: Documento = leer_json(archivo)?;
    let version = &documento.version_rubrica;
    ensure!(!version.is_empty() && version.bytes().all(|b| b.is_ascii_alphanumeric() || b"._-".contains(&b)),
        "La versión de rúbrica debe ser un nombre de archivo");
    let carpeta = archivo.parent().context("Falta el directorio de la actividad")?;
    let rubrica: rubricas::Archivo = leer_json(&carpeta.join("rubricas").join(format!("{version}.json")))?;
    ensure!(rubrica.contenido.version == *version && rubrica.contenido.actividad == ACTIVIDAD && rubrica.contenido.caso == "P01",
        "La rúbrica debe corresponder a LR, a la actividad formal y a la versión indicada");
    Ok((documento, rubrica))
}

fn leer_json<T: DeserializeOwned>(ruta: &Path) -> Result<T> {
    let contenido = fs::read(ruta).with_context(|| format!("No se pudo leer {}", ruta.display()))?;
    serde_json::from_slice(&contenido).with_context(|| format!("JSON inválido en {}", ruta.display()))
}

fn huellas_de(archivo: &Documento, rubrica: &rubricas::Archivo) -> Result<Value> {
    let clinica = &archivo.clinica.contenido;
    let instrumento = &archivo.instrumento.contenido;
    let rubrica = &rubrica.contenido;
    Ok(json!({
        "clinica": { "codigo": CODIGO_CLINICO, "version": clinica.version, "sha256": huella_clinica(clinica)? },
        "instrumento": { "codigo": CODIGO_EDUCATIVO, "version": instrumento.version, "sha256": instrumento.huella()? },
        "rubrica": { "codigo": rubrica.codigo, "version": rubrica.version, "sha256": rubricas::huella(rubrica)? },
    }))
}

fn huella_clinica(contenido: &Clinica) -> Result<String> {
    Ok(format!("{:x}", Sha256::digest(serde_json::to_vec(contenido)?)))
}

pub(crate) fn cargar(db: &Transaction<'_>, archivo: &Path) -> Result<()> {
    let (documento, rubrica) = leer_archivos(archivo)?;
    let clinica = &documento.clinica.contenido;
    let instrumento = &documento.instrumento.contenido;
    let aprobacion_clinica = documento.clinica.aprobacion.as_ref().context("Falta la aprobación clínica")?;
    let aprobacion_educativa = documento.instrumento.aprobacion.as_ref().context("Falta la aprobación educativa")?;
    let aprobacion_rubrica = rubrica.aprobacion.as_ref().context("Falta la aprobación de la rúbrica")?;
    ensure!(aprobacion_educativa.docente_id == aprobacion_clinica.docente_id
        && aprobacion_rubrica.docente_id == aprobacion_clinica.docente_id,
        "Las tres constancias deben identificar al mismo docente responsable");
    let huella = huella_clinica(clinica)?;
    aprobacion_clinica.validar(db, ACTIVIDAD, "formal", CODIGO_CLINICO, &clinica.version, &huella)?;
    aprobacion_educativa.validar(db, ACTIVIDAD, "formal", CODIGO_EDUCATIVO, &instrumento.version, &instrumento.huella()?)?;
    ensure!(!clinica.version.trim().is_empty() && clinica.ficha.paciente == "LR" && clinica.ficha.edad == 34
        && [&clinica.ficha.motivo, &clinica.ficha.contexto, &clinica.ficha.fuente].iter().all(|valor| !valor.trim().is_empty())
        && !clinica.ficha.indicaciones.is_empty() && clinica.ficha.indicaciones.iter().all(|valor| !valor.trim().is_empty()),
        "La ficha de LR requiere identificación, motivo, contexto, fuente e indicaciones");
    let base: i64 = db.query_row(
        "SELECT id FROM versiones_clinicas WHERE escenario_id='P01' AND version=?1 AND estado='ensayo'",
        [&clinica.base], |fila| fila.get(0),
    ).context("No está cargada la versión base de LR")?;
    ensure!(catalogo(db, base)? == clinica.catalogo, "El material clínico difiere de las fuentes y pasajes preparados");

    let registro = rubricas::cargar(db, &rubrica, "formal")?;
    instrumento::guardar(db, instrumento, Some(&serde_json::to_string(aprobacion_educativa)?))?;
    let version_id = guardar_clinica(db, base, clinica, aprobacion_clinica, &huella)?;
    db.execute(
        "INSERT INTO configuraciones_formales VALUES (?1,?2,?3,?4)
         ON CONFLICT(actividad_id) DO UPDATE SET version_caso_id=excluded.version_caso_id,
           version_instrumento=excluded.version_instrumento, rubrica_id=excluded.rubrica_id",
        params![ACTIVIDAD, version_id, instrumento.version, registro.id],
    )?;
    db.execute("UPDATE actividades SET habilitada=1 WHERE id=?1", [ACTIVIDAD])?;
    Ok(())
}

fn guardar_clinica(db: &Connection, base: i64, clinica: &Clinica, aprobacion: &Constancia, huella: &str) -> Result<i64> {
    db.execute(
        "INSERT INTO versiones_clinicas (escenario_id,version,estado,autorizada_por,autorizada_en,motivo_autorizacion)
         VALUES ('P01',?1,'aprobada',?2,?3,?4) ON CONFLICT(escenario_id,version) DO NOTHING",
        params![clinica.version, aprobacion.docente_id, aprobacion.fecha, aprobacion.motivo],
    )?;
    let id: i64 = db.query_row(
        "SELECT id FROM versiones_clinicas WHERE escenario_id='P01' AND version=?1 AND estado='aprobada'",
        [&clinica.version], |fila| fila.get(0),
    ).context("La versión clínica ya existe con un estado diferente")?;
    let constancia = serde_json::to_string(aprobacion)?;
    let nueva = db.execute(
        "INSERT INTO aprobaciones_clinicas VALUES (?1,?2,?3,?4) ON CONFLICT(version_id) DO NOTHING",
        params![id, serde_json::to_string(clinica)?, huella, constancia],
    )?;
    let anterior: (String, String) = db.query_row(
        "SELECT sha256,constancia FROM aprobaciones_clinicas WHERE version_id=?1",
        [id], |fila| Ok((fila.get(0)?, fila.get(1)?)),
    )?;
    ensure!(anterior == (huella.to_owned(), constancia), "La versión clínica ya tiene otro contenido o constancia");
    if nueva == 1 {
        for sql in [
            "INSERT INTO versiones_fuentes SELECT escenario_id,?1,fuente_id,revision_fuente_id FROM versiones_fuentes WHERE version_clinica_id=?2",
            "INSERT INTO hechos (id,escenario_id,version_clinica_id,fase,hablante,condicion,respuesta,
                 fuente_id,revision_fuente_id,linea,pasaje,tema,variantes)
             SELECT id,escenario_id,?1,fase,hablante,condicion,respuesta,fuente_id,revision_fuente_id,linea,pasaje,tema,variantes
             FROM hechos WHERE version_clinica_id=?2",
            "INSERT INTO lecturas_versionadas SELECT ?1,fase,instrumento,valor,unidad,revision_fuente_id,NULL FROM lecturas_versionadas WHERE version_caso_id=?2 AND procedencia_ensayo IS NULL",
            "INSERT INTO zonas_exploracion SELECT ?1,fase,instrumento,zona,etiqueta,
                CASE WHEN procedencia_ensayo IS NULL THEN resultado END,
                CASE WHEN procedencia_ensayo IS NULL THEN revision_fuente_id END,NULL
                FROM zonas_exploracion WHERE version_caso_id=?2",
            "INSERT INTO examenes_versionados SELECT ?1,fase,examen,resultado,revision_fuente_id FROM examenes_versionados WHERE version_caso_id=?2",
        ] {
            db.execute(sql, params![id, base])?;
        }
    }
    Ok(id)
}

fn catalogo(db: &Connection, id: i64) -> Result<Value> {
    let fuentes = filas_json(db, id,
        "SELECT json_object('ruta',f.ruta,'rol',f.rol,'sha256',r.sha256) FROM versiones_fuentes v
         JOIN fuentes f ON f.id=v.fuente_id JOIN revisiones_fuente r ON r.id=v.revision_fuente_id
         WHERE v.version_clinica_id=?1 ORDER BY f.ruta")?;
    let hechos = filas_json(db, id,
        "SELECT json_patch(json_object('id',h.id,'fase',h.fase,'hablante',h.hablante,'condicion',h.condicion,
             'respuesta',h.respuesta,'fuente',f.ruta,'linea',h.linea,'pasaje',h.pasaje),
             CASE WHEN h.tema='' AND h.variantes='[]' THEN '{}'
                  ELSE json_object('tema',h.tema,'variantes',json(h.variantes)) END)
         FROM hechos h JOIN fuentes f ON f.id=h.fuente_id WHERE h.version_clinica_id=?1 ORDER BY h.fase,h.hablante,h.id")?;
    let lecturas = filas_json(db, id,
        "SELECT json_object('fase',l.fase,'instrumento',l.instrumento,'valor',json(l.valor),'unidad',l.unidad,'fuente',f.ruta)
         FROM lecturas_versionadas l JOIN revisiones_fuente r ON r.id=l.revision_fuente_id JOIN fuentes f ON f.id=r.fuente_id
         WHERE l.version_caso_id=?1 AND l.procedencia_ensayo IS NULL ORDER BY l.fase,l.instrumento")?;
    let zonas = filas_json(db, id,
        "SELECT json_object('fase',z.fase,'instrumento',z.instrumento,'zona',z.zona,'etiqueta',z.etiqueta,
             'resultado',CASE WHEN z.procedencia_ensayo IS NULL THEN z.resultado END,'fuente',f.ruta)
         FROM zonas_exploracion z LEFT JOIN revisiones_fuente r ON r.id=z.revision_fuente_id AND z.procedencia_ensayo IS NULL
         LEFT JOIN fuentes f ON f.id=r.fuente_id WHERE z.version_caso_id=?1 ORDER BY z.fase,z.instrumento,z.zona")?;
    let examenes = filas_json(db, id,
        "SELECT json_object('fase',e.fase,'examen',e.examen,'resultado',e.resultado,'fuente',f.ruta)
         FROM examenes_versionados e JOIN revisiones_fuente r ON r.id=e.revision_fuente_id JOIN fuentes f ON f.id=r.fuente_id
         WHERE e.version_caso_id=?1 ORDER BY e.fase,e.examen")?;
    ensure!(fuentes.len() == 4 && !hechos.is_empty() && ["presion","pulso"].iter()
        .all(|instrumento| lecturas.iter().any(|fila| fila["instrumento"] == *instrumento))
        && examenes.iter().any(|fila| fila["examen"] == "ecg-inicial"), "El material inicial de LR está incompleto");
    Ok(json!({ "fuentes": fuentes, "hechos": hechos, "lecturas": lecturas, "zonas": zonas, "examenes": examenes }))
}

fn filas_json(db: &Connection, id: i64, sql: &str) -> Result<Vec<Value>> {
    let mut consulta = db.prepare(sql)?;
    let filas = consulta.query_map([id], |fila| fila.get::<_, String>(0))?;
    filas.map(|fila| Ok(serde_json::from_str(&fila?)?)).collect()
}
