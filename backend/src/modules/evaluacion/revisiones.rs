use std::collections::{HashMap, HashSet};

use anyhow::{Context as _, Result, ensure};
use axum::{Json, extract::{Path, State}, http::{HeaderMap, StatusCode}};
use rusqlite::{Connection, OptionalExtension, Row, TransactionBehavior, params};
use serde::{Deserialize, Serialize};

use crate::{
    agentes::evaluador::perfil::{self, Borrador, Juicio, Observacion},
    app::{errores::ErrorApi, estado::Estado},
    modules::acceso::sesiones,
    shared::operaciones::Operacion,
};

use super::{evidencia::{self, Contexto}, rubricas::Rubrica, trabajos};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Revision {
    pub revision: i64,
    pub anterior: Option<i64>,
    pub autor_id: Option<String>,
    pub motivo: String,
    pub momento: String,
    pub resultado: Borrador,
    pub puntaje: Puntaje,
    pub diferencias: Vec<Diferencia>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Puntaje {
    pub peso_aplicable: u32,
    pub total_centesimas: Option<u32>,
    pub criterios: Vec<Credito>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Credito {
    pub criterio_id: String,
    pub juicio: Juicio,
    pub peso: u32,
    pub credito: Option<u8>,
    pub puntos_ponderados: Option<u32>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Diferencia {
    pub criterio_id: String,
    pub antes: Observacion,
    pub despues: Observacion,
    pub credito_anterior: Option<u8>,
    pub credito_nuevo: Option<u8>,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Rectificacion {
    revision_esperada: i64,
    motivo: String,
    cambios: Vec<Cambio>,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Cambio {
    criterio_id: String,
    actuacion: String,
    evidencias: Vec<String>,
    juicio: Juicio,
    explicacion: String,
    mejora: String,
    credito: Option<u8>,
}

#[derive(Serialize)]
pub struct ActividadDocente {
    actividad_id: String,
    nombre: String,
}

#[derive(Serialize)]
pub struct EntregaDocente {
    entrega_id: String,
    estudiante_id: String,
    intento_id: String,
    confirmada_en: String,
    estado: String,
    evaluacion_id: Option<String>,
}

pub async fn actividades(State(app): State<Estado>, cabeceras: HeaderMap) -> Result<Json<Vec<ActividadDocente>>, ErrorApi> {
    app.consultar(move |db| {
        let identidad = sesiones::autenticar(db, &cabeceras)?;
        if identidad.rol != "docente" {
            return Err(reservada());
        }
        let mut consulta = db.prepare(
            "SELECT DISTINCT a.id,a.nombre FROM actividades a JOIN asignaciones s ON s.actividad_id=a.id
             WHERE s.docente_id=?1 AND a.habilitada=1 ORDER BY a.id",
        )?;
        let filas = consulta.query_map([identidad.id], |r| Ok(ActividadDocente { actividad_id:r.get(0)?, nombre:r.get(1)? }))?;
        Ok(Json(filas.collect::<rusqlite::Result<_>>()?))
    }).await
}

pub async fn entregas(
    State(app): State<Estado>, Path(id): Path<String>, cabeceras: HeaderMap,
) -> Result<Json<Vec<EntregaDocente>>, ErrorApi> {
    app.consultar(move |db| {
        let tx = db.transaction()?;
        let identidad = sesiones::autenticar(&tx, &cabeceras)?;
        let asignada: bool = tx.query_row(
            "SELECT EXISTS(SELECT 1 FROM asignaciones WHERE docente_id=?1 AND actividad_id=?2)",
            params![identidad.id, id], |r| r.get(0),
        )?;
        if identidad.rol != "docente" || !asignada {
            return Err(reservada());
        }
        let mut consulta = tx.prepare(
            "SELECT e.id,i.estudiante_id,i.id,e.confirmada_en,t.estado,v.id
             FROM entregas e JOIN intentos i ON i.id=e.intento_id
             JOIN asignaciones s ON s.estudiante_id=i.estudiante_id AND s.actividad_id=i.actividad_id
             JOIN trabajos_evaluacion t ON t.entrega_id=e.id LEFT JOIN evaluaciones v ON v.entrega_id=e.id
             WHERE i.actividad_id=?1 AND s.docente_id=?2 ORDER BY e.confirmada_en,e.id",
        )?;
        let filas = consulta.query_map(params![id, identidad.id], |r| Ok(EntregaDocente {
            entrega_id:r.get(0)?, estudiante_id:r.get(1)?, intento_id:r.get(2)?, confirmada_en:r.get(3)?,
            estado:r.get(4)?, evaluacion_id:r.get(5)?,
        }))?;
        Ok(Json(filas.collect::<rusqlite::Result<_>>()?))
    }).await
}

pub async fn rectificar(
    State(app): State<Estado>, Path(id): Path<String>, cabeceras: HeaderMap,
    Json(rectificacion): Json<Rectificacion>,
) -> Result<(StatusCode, Json<Revision>), ErrorApi> {
    app.consultar(move |db| {
        let tx = db.transaction_with_behavior(TransactionBehavior::Immediate)?;
        let docente = trabajos::autorizar(&tx, &id, &cabeceras)?;
        let operacion = Operacion::nueva(&docente.id, &id, &cabeceras, "PATCH evaluacion", &rectificacion)?;
        if let Some(anterior) = operacion.recuperar(&tx)? {
            return Ok((StatusCode::OK, Json(anterior)));
        }
        let actual = ultima(&tx, &id)?;
        if actual.revision != rectificacion.revision_esperada {
            return Err(ErrorApi::nuevo(StatusCode::CONFLICT, "La revisión cambió. Vuelve a cargar la evaluación."));
        }
        let context = contexto(&tx, &id).map_err(ErrorApi::interno)?;
        let mut nueva = editar(&context, &actual, rectificacion)?;
        nueva.autor_id = Some(docente.id.clone());
        nueva.momento = guardar(&tx, &id, &nueva)?;
        operacion.confirmar(&tx, &nueva)?;
        tx.commit()?;
        Ok((StatusCode::CREATED, Json(nueva)))
    }).await
}

pub async fn leer(
    State(app): State<Estado>, Path((id, revision)): Path<(String, i64)>, cabeceras: HeaderMap,
) -> Result<Json<Revision>, ErrorApi> {
    app.consultar(move |db| {
        let tx = db.transaction()?;
        trabajos::autorizar(&tx, &id, &cabeceras)?;
        let revision = buscar(&tx, &id, revision)?.ok_or_else(omitida)?;
        Ok(Json(revision))
    }).await
}

fn editar(contexto: &Contexto, actual: &Revision, peticion: Rectificacion) -> Result<Revision, ErrorApi> {
    let motivo = peticion.motivo.trim();
    if motivo.is_empty() || motivo.chars().count() > 1000 || peticion.cambios.is_empty()
        || peticion.cambios.len() > actual.resultado.observaciones.len()
    {
        return Err(ErrorApi::nuevo(StatusCode::UNPROCESSABLE_ENTITY, "Indica el motivo y los criterios que deseas rectificar."));
    }
    let mut resultado = actual.resultado.clone();
    let mut creditos: HashMap<_, _> = actual.puntaje.criterios.iter().map(|c| (c.criterio_id.clone(), c.credito)).collect();
    let mut diferencias = Vec::new();
    let mut usados = HashSet::new();
    for cambio in peticion.cambios {
        if !usados.insert(cambio.criterio_id.clone()) {
            return Err(ErrorApi::nuevo(StatusCode::UNPROCESSABLE_ENTITY, "Cada criterio se rectifica una vez por revisión."));
        }
        let observacion = resultado.observaciones.iter_mut().find(|item| item.criterio_id == cambio.criterio_id)
            .ok_or_else(cambio_invalido)?;
        let criterio = contexto.rubrica.criterios.iter().find(|c| c.id == observacion.criterio_id)
            .ok_or_else(cambio_invalido)?;
        let antes = observacion.clone();
        observacion.actuacion = cambio.actuacion.trim().into();
        observacion.juicio = cambio.juicio;
        observacion.evidencias = cambio.evidencias;
        observacion.explicacion = cambio.explicacion.trim().into();
        observacion.mejora = cambio.mejora.trim().into();
        if observacion.juicio != Juicio::RevisionDocente {
            observacion.incidencia = None;
        }
        let concede_credito = cambio.credito.is_some_and(|valor| valor > 0);
        let requiere_cita = concede_credito || matches!(observacion.juicio, Juicio::Acierto | Juicio::Error);
        let cita_valida = observacion.evidencias.iter().any(|id| contexto.entrada.evidencias.iter()
            .any(|e| &e.id == id && criterio.evidencias.contains(&e.tipo)));
        if !credito_valido(cambio.juicio, cambio.credito) || (requiere_cita && !cita_valida)
            || [&observacion.actuacion, &observacion.explicacion, &observacion.mejora].into_iter().any(|s| s.chars().count() > 1400)
        {
            return Err(cambio_invalido());
        }
        let credito_anterior = creditos.insert(cambio.criterio_id.clone(), cambio.credito).flatten();
        if *observacion != antes || cambio.credito != credito_anterior {
            diferencias.push(Diferencia {
                criterio_id: cambio.criterio_id, antes, despues: observacion.clone(),
                credito_anterior, credito_nuevo: cambio.credito,
            });
        }
    }
    if diferencias.is_empty() {
        return Err(ErrorApi::nuevo(StatusCode::UNPROCESSABLE_ENTITY, "La revisión requiere al menos un cambio."));
    }
    if resultado.prioridad.as_ref().is_some_and(|p| diferencias.iter().any(|c| c.criterio_id == p.criterio_id)) {
        resultado.prioridad = None;
    }
    perfil::validar_borrador(&contexto.entrada, &resultado).map_err(|_| cambio_invalido())?;
    let puntaje = calcular(&contexto.rubrica, &resultado, &creditos).map_err(ErrorApi::interno)?;
    Ok(Revision {
        revision: actual.revision + 1, anterior: Some(actual.revision), autor_id: None,
        motivo: motivo.into(), momento: String::new(), resultado, puntaje, diferencias,
    })
}

pub(crate) fn ultima(db: &Connection, evaluacion: &str) -> Result<Revision, ErrorApi> {
    let ultima: Option<i64> = db.query_row(
        "SELECT max(revision) FROM revisiones_evaluacion WHERE evaluacion_id=?1",
        [evaluacion], |fila| fila.get(0),
    )?;
    buscar(db, evaluacion, ultima.ok_or_else(omitida)?)?.ok_or_else(omitida)
}

pub(crate) fn buscar(db: &Connection, evaluacion: &str, revision: i64) -> Result<Option<Revision>, ErrorApi> {
    Ok(db.query_row(
        "SELECT revision,anterior,autor_id,motivo,momento,resultado,puntaje,diferencias
         FROM revisiones_evaluacion WHERE evaluacion_id=?1 AND revision=?2",
        params![evaluacion, revision], desde_fila,
    ).optional()?)
}

pub(crate) fn inicializar_existentes(db: &Connection) -> Result<()> {
    let mut consulta = db.prepare("SELECT id,borrador,momento FROM evaluaciones ORDER BY id")?;
    let anteriores = consulta.query_map([], |fila| Ok((
        fila.get::<_, String>(0)?, fila.get::<_, String>(1)?, fila.get::<_, String>(2)?,
    )))?.collect::<rusqlite::Result<Vec<_>>>()?;
    for (id, resultado, momento) in anteriores {
        registrar_base(db, &id, serde_json::from_str(&resultado)?, momento)?;
    }
    Ok(())
}

pub(crate) fn registrar_base(db: &Connection, id: &str, resultado: Borrador, momento: String) -> Result<()> {
    let contexto = contexto(db, id)?;
    let puntaje = calcular(&contexto.rubrica, &resultado, &HashMap::new())?;
    let revision = Revision {
        revision: 1, anterior: None, autor_id: None, motivo: "Borrador del evaluador".into(), momento,
        resultado, puntaje, diferencias: Vec::new(),
    };
    guardar(db, id, &revision)?;
    Ok(())
}

fn contexto(db: &Connection, evaluacion: &str) -> Result<Contexto> {
    let entrega: String = db.query_row("SELECT entrega_id FROM evaluaciones WHERE id=?1", [evaluacion], |r| r.get(0))?;
    evidencia::construir(db, &entrega)?.context("Falta la entrega con rúbrica congelada")
}

pub fn calcular(rubrica: &Rubrica, resultado: &Borrador, ajustes: &HashMap<String, Option<u8>>) -> Result<Puntaje> {
    let mut criterios = Vec::with_capacity(rubrica.criterios.len());
    let mut peso_aplicable = 0;
    let mut puntos = 0;
    let mut pendiente = false;
    for criterio in &rubrica.criterios {
        let observacion = resultado.observaciones.iter().find(|o| o.criterio_id == criterio.id)
            .context("Falta el juicio de un criterio en la revisión")?;
        let juicio = observacion.juicio;
        let credito = ajustes.get(&criterio.id).copied().unwrap_or_else(|| match juicio {
            Juicio::Acierto => Some(100),
            Juicio::Error | Juicio::Omision => Some(0),
            Juicio::NoEvaluable | Juicio::RevisionDocente => None,
        });
        ensure!(credito_valido(juicio, credito), "El juicio y el crédito son incompatibles");
        let aporte = credito.map(|valor| criterio.peso * u32::from(valor));
        if let Some(aporte) = aporte {
            peso_aplicable += criterio.peso;
            puntos += aporte;
        }
        pendiente |= juicio == Juicio::RevisionDocente;
        criterios.push(Credito { criterio_id: criterio.id.clone(), juicio, peso: criterio.peso,
            credito, puntos_ponderados: aporte });
    }
    let total_centesimas = if pendiente || peso_aplicable == 0 { None } else {
        Some((puntos * 100 + peso_aplicable / 2) / peso_aplicable)
    };
    Ok(Puntaje { peso_aplicable, total_centesimas, criterios })
}

fn credito_valido(juicio: Juicio, credito: Option<u8>) -> bool {
    match juicio {
        Juicio::Acierto => credito.is_some_and(|valor| (1..=100).contains(&valor)),
        Juicio::Error | Juicio::Omision => credito.is_some_and(|valor| valor <= 100),
        Juicio::NoEvaluable | Juicio::RevisionDocente => credito.is_none(),
    }
}

fn guardar(db: &Connection, id: &str, revision: &Revision) -> rusqlite::Result<String> {
    db.query_row(
        "INSERT INTO revisiones_evaluacion (evaluacion_id,revision,anterior,autor_id,motivo,momento,resultado,puntaje,diferencias)
         VALUES (?1,?2,?3,?4,?5,coalesce(nullif(?6,''),strftime('%Y-%m-%dT%H:%M:%fZ','now')),?7,?8,?9)
         RETURNING momento",
        params![id, revision.revision, revision.anterior, revision.autor_id, revision.motivo, revision.momento,
            json(&revision.resultado)?, json(&revision.puntaje)?, json(&revision.diferencias)?],
        |fila| fila.get(0),
    )
}

fn json(valor: &impl Serialize) -> rusqlite::Result<String> {
    serde_json::to_string(valor).map_err(|error| rusqlite::Error::ToSqlConversionFailure(Box::new(error)))
}

fn desde_fila(fila: &Row<'_>) -> rusqlite::Result<Revision> {
    fn leer<T: serde::de::DeserializeOwned>(fila: &Row<'_>, columna: usize) -> rusqlite::Result<T> {
        serde_json::from_str(&fila.get::<_, String>(columna)?).map_err(|error|
            rusqlite::Error::FromSqlConversionFailure(columna, rusqlite::types::Type::Text, Box::new(error)))
    }
    Ok(Revision {
        revision: fila.get(0)?, anterior: fila.get(1)?, autor_id: fila.get(2)?, motivo: fila.get(3)?, momento: fila.get(4)?,
        resultado: leer(fila, 5)?, puntaje: leer(fila, 6)?, diferencias: leer(fila, 7)?,
    })
}

fn cambio_invalido() -> ErrorApi {
    ErrorApi::nuevo(StatusCode::UNPROCESSABLE_ENTITY, "Revisa el juicio, el crédito, la retroalimentación y las citas propias del criterio.")
}

fn omitida() -> ErrorApi {
    ErrorApi::nuevo(StatusCode::NOT_FOUND, "No existe esa revisión de la evaluación.")
}

fn reservada() -> ErrorApi {
    ErrorApi::nuevo(StatusCode::FORBIDDEN, "Las entregas están reservadas al docente responsable.")
}
