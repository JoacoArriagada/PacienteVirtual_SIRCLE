use axum::http::{HeaderMap, StatusCode};
use rusqlite::{Connection, params};

use crate::{
    app::errores::ErrorApi,
    modules::{
        acceso::{actividades::actividad_no_asignada, sesiones::{self, Identidad}},
        intentos::intento::{self, Intento},
    },
};

pub fn intento_propio(db: &Connection, cabeceras: &HeaderMap, id: &str) -> Result<(Identidad, Intento), ErrorApi> {
    let identidad = sesiones::autenticar(db, cabeceras)?;
    if identidad.rol != "estudiante" {
        return Err(actividad_no_asignada());
    }
    let intento = intento::recuperar(db, id, &identidad.id)?;
    Ok((identidad, intento))
}

pub fn docente_del_intento(db: &Connection, cabeceras: &HeaderMap, id: &str) -> Result<Identidad, ErrorApi> {
    let identidad = sesiones::autenticar(db, cabeceras)?;
    if identidad.rol != "docente" {
        return Err(revision_reservada());
    }
    let asignado: bool = db.query_row(
        "SELECT EXISTS(SELECT 1 FROM intentos i JOIN asignaciones a
         ON a.actividad_id=i.actividad_id AND a.estudiante_id=i.estudiante_id
         WHERE i.id=?1 AND a.docente_id=?2)",
        params![id, identidad.id], |fila| fila.get(0),
    )?;
    if !asignado {
        return Err(revision_reservada());
    }
    Ok(identidad)
}

pub fn pre_resuelto(intento: &Intento) -> Result<(), ErrorApi> {
    if intento.pre == "pendiente" {
        return Err(ErrorApi::nuevo(StatusCode::CONFLICT, "Resuelve el pre antes de entrevistar."));
    }
    Ok(())
}

pub fn entrevista_nueva(intento: &Intento) -> Result<(), ErrorApi> {
    pre_resuelto(intento)?;
    if intento.estado != "atencion" || intento.fase != "inicial" {
        return Err(ErrorApi::nuevo(StatusCode::CONFLICT, "La fase actual no permite iniciar la entrevista."));
    }
    Ok(())
}

fn revision_reservada() -> ErrorApi {
    ErrorApi::nuevo(StatusCode::FORBIDDEN, "La revisión está reservada al docente responsable.")
}
