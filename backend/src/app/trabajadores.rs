use anyhow::{Result, ensure};
use tokio::io::{BufReader, stdin, stdout};

use crate::{
    agentes::{
        evaluador::inferencia,
        mensajes::{self, Agente},
        supervisor::{self, Acuse, Peticion, Supervisor},
    },
    modules::entrevista::ollama::Ollama,
    shared::{contexto, trazas},
};

pub async fn iniciar(url: &str) -> Result<Supervisor> {
    Supervisor::iniciar(&std::env::current_exe()?, url).await
}

pub async fn ejecutar(agente: Agente, url: &str) -> Result<()> {
    let motor = Ollama::nueva(url)?;
    let pid = std::process::id();
    let mut entrada = BufReader::new(stdin());
    let mut salida = stdout();
    supervisor::escribir(&mut salida, &Acuse::Inicio(supervisor::ficha_proceso(agente, pid, url))).await?;
    loop {
        let peticion = match supervisor::leer(&mut entrada).await {
            Ok(peticion) => peticion,
            Err(error) if error.downcast_ref::<std::io::Error>()
                .is_some_and(|io| io.kind() == std::io::ErrorKind::UnexpectedEof) => return Ok(()),
            Err(error) => return Err(error),
        };
        let acuse = match procesar(agente, pid, &motor, peticion).await {
            Ok(acuse) => acuse,
            Err(error) => Acuse::Error { mensaje: format!("{error:#}") },
        };
        supervisor::escribir(&mut salida, &acuse).await?;
    }
}

async fn procesar(agente: Agente, pid: u32, motor: &Ollama, peticion: Peticion) -> Result<Acuse> {
    match peticion {
        Peticion::Probar { operacion_id } => Ok(Acuse::Prueba { agente, pid, operacion_id }),
        Peticion::Entrevistar(solicitud) => {
            ensure!(agente == solicitud.cabecera.agente
                && contexto::voz_autorizada(agente, &solicitud.cabecera.caso, &solicitud.cabecera.hablante),
                "El canal no corresponde a la entrevista");
            trazas::evento("SIRCLE_TRABAJADOR_OPERACION", &serde_json::json!({
                "pid": pid, "agente": agente.id(), "operacion_id": solicitud.cabecera.operacion_id,
                "intento_id": solicitud.cabecera.intento_id, "inicio_unix_ms": trazas::ahora_ms(),
            }));
            Ok(Acuse::Entrevista(mensajes::entrevistar(motor, solicitud).await?))
        }
        Peticion::Evaluar(solicitud) => {
            ensure!(agente == Agente::Evaluador, "El canal no corresponde al evaluador");
            Ok(Acuse::Evaluacion(inferencia::ejecutar(motor, solicitud).await?))
        }
    }
}

pub async fn senal_de_cierre() {
    #[cfg(unix)]
    {
        if let Ok(mut terminar) = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate()) {
            tokio::select! {
                _ = terminar.recv() => {},
                _ = tokio::signal::ctrl_c() => {},
            }
            return;
        }
    }
    let _ = tokio::signal::ctrl_c().await;
}
