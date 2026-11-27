use anyhow::Context;
use axum::{Json, extract::{Path, State}, http::{HeaderMap, StatusCode}};
use rusqlite::{Transaction, TransactionBehavior, params};
use serde::Serialize;
use serde_json::{Value, json};

use crate::{
    agentes::{dan_sparks::perfil as dan, lr::perfil as lr, mensajes::Agente, nannu::perfil as nannu},
    app::{errores::ErrorApi, estado::Estado},
    modules::{entrevista::turnos::{self, Salida, Turno}, intentos::intento::Intento},
    shared::autorizacion,
};

use super::mediciones::{self, Medicion};

const VERSION: &str = "reaccion-temperatura-v1";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
enum Banda {
    Baja,
    Habitual,
    Intermedia,
    Alta,
}

pub(crate) fn preparar(db: &Transaction<'_>) -> anyhow::Result<()> {
    for (actividad, ruta, pasaje, valor, unidad) in [
        (nannu::ACTIVIDAD, nannu::FUENTE_ENTORNO, "Temp 39.2", r#"{"lectura":39.2}"#, "°C"),
        (dan::ACTIVIDAD, dan::FUENTE_ENTORNO, "Temperature – normal.", r#"{"banda":"habitual"}"#, ""),
    ] {
        let (version, fuente): (i64, i64) = db.query_row(
            "SELECT c.version_caso_id, r.id
             FROM configuraciones_ensayo c
             JOIN versiones_fuentes vf ON vf.version_clinica_id=c.version_caso_id
             JOIN fuentes f ON f.id=vf.fuente_id AND f.rol='environment_controller'
             JOIN revisiones_fuente r ON r.id=vf.revision_fuente_id
             WHERE c.actividad_id=?1 AND f.ruta=?2 AND instr(CAST(r.contenido AS TEXT),?3)>0",
            params![actividad, ruta, pasaje], |fila| Ok((fila.get(0)?, fila.get(1)?)),
        ).with_context(|| format!("Falta la temperatura inicial de {actividad}"))?;
        db.execute(
            "INSERT INTO lecturas_versionadas (version_caso_id,fase,instrumento,valor,unidad,revision_fuente_id)
             VALUES (?1,'inicial','temperatura',?2,?3,?4)
             ON CONFLICT(version_caso_id,fase,instrumento) DO NOTHING",
            params![version, valor, unidad, fuente],
        )?;
    }
    Ok(())
}

pub(super) fn preguntar_lectura(
    db: &Transaction<'_>, intento: &Intento, medicion: &Medicion,
) -> Result<Turno, ErrorApi> {
    let (agente, hablante) = paciente(intento)?;
    // La pregunta y la toma se confirman juntas y comparten la revisión de la toma.
    turnos::guardar(db, intento, Salida {
        agente, hablante, texto: "¿Cuánto marcó el termómetro?", tipo: "reaccion_medicion",
    }, "", &format!("temperatura-toma-{}", medicion.medicion_id), &json!({
        "origen": "valoracion", "medicion_id": medicion.medicion_id, "version_instrucciones": VERSION,
    }))
}

pub(crate) async fn compartir(
    State(app): State<Estado>, Path((id, medicion_id)): Path<(String, String)>, cabeceras: HeaderMap,
) -> Result<(StatusCode, Json<Turno>), ErrorApi> {
    app.consultar(move |db| {
        let tx = db.transaction_with_behavior(TransactionBehavior::Immediate)?;
        let (_, intento) = autorizacion::intento_propio(&tx, &cabeceras, &id)?;
        let operacion = format!("temperatura-compartida-{medicion_id}");
        if let Some(turno) = turnos::por_operacion(&tx, &id, &operacion)? {
            return Ok((StatusCode::OK, Json(turno)));
        }
        autorizacion::entrevista_nueva(&intento)?;
        let medicion = mediciones::por_id(&tx, &id, &medicion_id)?;
        if medicion.instrumento != "temperatura" || medicion.fase != intento.fase {
            return Err(ErrorApi::nuevo(StatusCode::UNPROCESSABLE_ENTITY, "Comparte una toma de temperatura de esta fase."));
        }
        let (agente, hablante) = paciente(&intento)?;
        let (lectura, banda) = interpretar(&medicion.valor, &medicion.unidad)
            .ok_or_else(|| ErrorApi::nuevo(StatusCode::CONFLICT, "La lectura de temperatura no está disponible."))?;
        let texto = match banda {
            Banda::Baja => "¿Tan baja? ¿Me la puede volver a tomar?",
            Banda::Alta => "¿Tan alta? Eso me preocupa.",
            Banda::Habitual => "Ah, ya. Gracias por decírmelo.",
            Banda::Intermedia => "Ah, ya. ¿Ese valor está bien?",
        };
        let turno = turnos::guardar(&tx, &intento, Salida {
            agente, hablante, texto, tipo: "reaccion_medicion",
        }, &format!("Le comunico la temperatura: {lectura}."), &operacion, &json!({
            "origen": "valoracion", "medicion_id": medicion_id, "banda_temperatura": banda,
            "version_instrucciones": VERSION,
        }))?;
        tx.commit()?;
        Ok((StatusCode::CREATED, Json(turno)))
    }).await
}

fn paciente(intento: &Intento) -> Result<(Agente, &'static str), ErrorApi> {
    match intento.caso.as_str() {
        lr::ESCENARIO => Ok((lr::AGENTE, lr::HABLANTE)),
        nannu::ESCENARIO => Ok((nannu::AGENTE, nannu::PACIENTE)),
        dan::ESCENARIO => Ok((dan::AGENTE, dan::HABLANTE)),
        _ => Err(ErrorApi::nuevo(StatusCode::CONFLICT, "El intento no permite compartir la toma.")),
    }
}

fn interpretar(valor: &Value, unidad: &str) -> Option<(String, Banda)> {
    if valor["banda"].as_str() == Some("habitual") && unidad.is_empty() {
        return Some(("en el rango habitual".into(), Banda::Habitual));
    }
    let celsius = valor["lectura"].as_f64().filter(|numero| numero.is_finite() && unidad == "°C")?;
    // Bandas de diálogo del termómetro simulado. Entre 37,2 y 38 se usa una reacción neutra.
    let banda = match celsius {
        n if n < 36.1 => Banda::Baja,
        n if n <= 37.2 => Banda::Habitual,
        n if n < 38.0 => Banda::Intermedia,
        _ => Banda::Alta,
    };
    Some((format!("{} °C", celsius.to_string().replace('.', ",")), banda))
}
