use std::collections::BTreeMap;

use axum::{Json, extract::{Path, State}, http::{HeaderMap, StatusCode}};
use serde::Serialize;

use crate::{
    app::{errores::ErrorApi, estado::Estado},
    modules::acceso::sesiones,
};

use super::{formas, instrumento::{self, Instrumento}, respuestas::Respuestas};

type Distribucion = BTreeMap<i16, usize>;

#[derive(Default, Serialize)]
pub struct Informe {
    actividad_id: String,
    invitados: usize,
    sin_intento: usize,
    pre_respondidos: usize,
    post_respondidos: usize,
    pares_completos: usize,
    repeticiones: usize,
    intentos: Vec<IntentoEducativo>,
    grupos: Vec<Grupo>,
}

#[derive(Serialize)]
struct IntentoEducativo {
    estudiante_id: String,
    intento_id: String,
    version: String,
    repeticion: bool,
    secuencia: String,
    pre: String,
    post: String,
    pre_en: Option<String>,
    post_en: Option<String>,
}

#[derive(Serialize)]
struct Grupo {
    version: String,
    repeticion: bool,
    percepcion: [Percepcion; 3],
    utilidad: [Utilidad; 2],
    razonamiento: Vec<Razonamiento>,
    cambio_razonamiento: Distribucion,
    pares_completos: usize,
}

#[derive(Default, Serialize)]
struct Percepcion {
    enunciado: String,
    pre: Distribucion,
    post: Distribucion,
    cambio: Distribucion,
    faltantes_pre: usize,
    faltantes_post: usize,
}

#[derive(Default, Serialize)]
struct Utilidad {
    enunciado: String,
    post: Distribucion,
    faltantes_post: usize,
}

#[derive(Serialize)]
struct Razonamiento {
    secuencia: String,
    momento: &'static str,
    forma: &'static str,
    porcentaje: Distribucion,
    faltantes_por_item: [usize; 2],
}

impl Grupo {
    fn nuevo(instrumento: &Instrumento, repeticion: bool) -> Self {
        Self {
            version: instrumento.version.clone(), repeticion,
            percepcion: std::array::from_fn(|i| Percepcion {
                enunciado: instrumento.percepcion[i].clone(), ..Percepcion::default()
            }),
            utilidad: std::array::from_fn(|i| Utilidad {
                enunciado: instrumento.utilidad[i].clone(), ..Utilidad::default()
            }),
            razonamiento: Vec::new(), cambio_razonamiento: BTreeMap::new(), pares_completos: 0,
        }
    }

    fn registrar(&mut self, instrumento: &Instrumento, ordinal: i64, pre: &Respuestas, post: &Respuestas) -> bool {
        let mut percepcion_completa = true;
        for (i, medida) in self.percepcion.iter_mut().enumerate() {
            let item = format!("P{}", i+1);
            sumar(&mut medida.pre, &mut medida.faltantes_pre, pre.get(&item).copied());
            sumar(&mut medida.post, &mut medida.faltantes_post, post.get(&item).copied());
            if let Some((antes, despues)) = pre.get(&item).zip(post.get(&item)) {
                *medida.cambio.entry(i16::from(*despues)-i16::from(*antes)).or_default() += 1;
            } else {
                percepcion_completa = false;
            }
        }
        for (i, medida) in self.utilidad.iter_mut().enumerate() {
            sumar(&mut medida.post, &mut medida.faltantes_post, post.get(&format!("U{}", i+1)).copied());
        }
        let secuencia = secuencia(ordinal);
        let antes = self.registrar_razonamiento(instrumento, &secuencia, "pre", formas::pre(ordinal), pre);
        let despues = self.registrar_razonamiento(instrumento, &secuencia, "post", formas::post(ordinal), post);
        if let Some((antes, despues)) = antes.zip(despues) {
            *self.cambio_razonamiento.entry(despues-antes).or_default() += 1;
            self.pares_completos += usize::from(percepcion_completa);
            return percepcion_completa;
        }
        false
    }

    fn registrar_razonamiento(
        &mut self, instrumento: &Instrumento, secuencia: &str, momento: &'static str, forma: &'static str,
        respuestas: &Respuestas,
    ) -> Option<i16> {
        let indice = self.razonamiento.iter().position(|grupo| grupo.secuencia == secuencia && grupo.momento == momento)
            .unwrap_or_else(|| {
                self.razonamiento.push(Razonamiento {
                    secuencia: secuencia.into(), momento, forma, porcentaje: BTreeMap::new(), faltantes_por_item: [0; 2],
                });
                self.razonamiento.len()-1
            });
        let fila = &mut self.razonamiento[indice];
        let mut aciertos = 0;
        let mut completa = true;
        for (i, ejercicio) in instrumento.ejercicios(forma).iter().enumerate() {
            match respuestas.get(&format!("R{}", i+1)) {
                Some(valor) => aciertos += i16::from(*valor == ejercicio.clave),
                None => {
                    fila.faltantes_por_item[i] += 1;
                    completa = false;
                }
            }
        }
        let puntaje = aciertos*50;
        if completa {
            *fila.porcentaje.entry(puntaje).or_default() += 1;
        }
        completa.then_some(puntaje)
    }
}

fn secuencia(ordinal: i64) -> String {
    format!("{}→{}", formas::pre(ordinal), formas::post(ordinal))
}

fn sumar(distribucion: &mut Distribucion, faltantes: &mut usize, respuesta: Option<u8>) {
    if let Some(valor) = respuesta {
        *distribucion.entry(i16::from(valor)).or_default() += 1;
    } else {
        *faltantes += 1;
    }
}

pub async fn leer(
    State(app): State<Estado>, Path(id): Path<String>, cabeceras: HeaderMap,
) -> Result<Json<Informe>, ErrorApi> {
    app.consultar(move |db| {
        let tx = db.transaction()?;
        let docente = sesiones::autenticar(&tx, &cabeceras)?;
        if docente.rol != "docente" {
            return Err(reservado());
        }
        let mut consulta = tx.prepare(
            "SELECT a.estudiante_id, i.id, i.version_instrumento, s.ordinal, i.pre,
                    coalesce(post.estado, legado.estado, 'pendiente'), pre.instante,
                    coalesce(post.instante, legado.momento), pre.respuestas, post.respuestas,
                    EXISTS(SELECT 1 FROM intentos anterior WHERE anterior.estudiante_id=i.estudiante_id
                           AND anterior.escenario_id=i.escenario_id AND anterior.rowid<i.rowid)
             FROM asignaciones a
             LEFT JOIN intentos i ON i.actividad_id=a.actividad_id AND i.estudiante_id=a.estudiante_id
             LEFT JOIN secuencias_educativas s ON s.intento_id=i.id
             LEFT JOIN respuestas_educativas pre ON pre.intento_id=i.id AND pre.etapa='pre'
             LEFT JOIN respuestas_educativas post ON post.intento_id=i.id AND post.etapa='post'
             LEFT JOIN post_ensayo legado ON legado.intento_id=i.id
             WHERE a.actividad_id=?1 AND a.docente_id=?2 ORDER BY s.ordinal, a.estudiante_id",
        )?;
        let mut filas = consulta.query([&id, &docente.id])?;
        let mut informe = Informe { actividad_id: id, ..Informe::default() };
        let mut instrumentos = BTreeMap::new();
        let mut grupos = BTreeMap::new();
        while let Some(fila) = filas.next()? {
            informe.invitados += 1;
            let Some(intento_id) = fila.get::<_, Option<String>>(1)? else {
                informe.sin_intento += 1;
                continue;
            };
            let version: String = fila.get(2)?;
            let ordinal: i64 = fila.get(3)?;
            let repeticion: bool = fila.get(10)?;
            informe.repeticiones += usize::from(repeticion);
            let pre = leer_respuestas(fila.get(8)?)?;
            let post = leer_respuestas(fila.get(9)?)?;
            informe.pre_respondidos += usize::from(!pre.is_empty());
            informe.post_respondidos += usize::from(!post.is_empty());
            if !instrumentos.contains_key(&version) {
                let instrumento = instrumento::leer(&tx, &version).map_err(ErrorApi::interno)?;
                instrumentos.insert(version.clone(), instrumento);
            }
            if let Some(instrumento) = &instrumentos[&version] {
                let grupo = grupos.entry((version.clone(), repeticion))
                    .or_insert_with(|| Grupo::nuevo(instrumento, repeticion));
                informe.pares_completos += usize::from(grupo.registrar(instrumento, ordinal, &pre, &post));
            }
            informe.intentos.push(IntentoEducativo {
                estudiante_id: fila.get(0)?, intento_id, version, repeticion, secuencia: secuencia(ordinal),
                pre: fila.get(4)?, post: fila.get(5)?, pre_en: fila.get(6)?, post_en: fila.get(7)?,
            });
        }
        if informe.invitados == 0 {
            return Err(reservado());
        }
        informe.grupos = grupos.into_values().collect();
        Ok(Json(informe))
    }).await
}

fn leer_respuestas(contenido: Option<String>) -> Result<Respuestas, ErrorApi> {
    contenido.map(|contenido| serde_json::from_str(&contenido).map_err(ErrorApi::interno))
        .transpose().map(Option::unwrap_or_default)
}

fn reservado() -> ErrorApi {
    ErrorApi::nuevo(StatusCode::FORBIDDEN, "El informe educativo está reservado al docente responsable.")
}
