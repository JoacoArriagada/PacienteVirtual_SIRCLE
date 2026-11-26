use std::{collections::HashMap, path::{Path, PathBuf}, process::Stdio, sync::Arc, time::Duration};

use anyhow::{Context, Result, anyhow, ensure};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use tokio::{
    io::{AsyncBufRead, AsyncBufReadExt, AsyncReadExt, AsyncWrite, AsyncWriteExt, BufReader},
    process::{Child, ChildStdin, ChildStdout, Command},
    sync::Mutex,
    time::timeout,
};

use crate::modules::entrevista::ollama::{DIGEST_MODELO, Invocacion, MODELO, TIEMPO_EVALUADOR};

use super::evaluador::{inferencia::Evaluar, perfil::Propuesta};
use super::mensajes::{Agente, Pregunta, Respuesta, Solicitud};

const LIMITE_MENSAJE: u64 = 1024 * 1024;
const AGENTES: [Agente; 4] = [Agente::Lr, Agente::Nannu, Agente::DanSparks, Agente::Evaluador];

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case", tag = "tipo", content = "datos")]
pub enum Peticion {
    Probar { operacion_id: String },
    Entrevistar(Solicitud<Pregunta>),
    Evaluar(Solicitud<Evaluar>),
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case", tag = "tipo", content = "datos")]
pub enum Acuse {
    Inicio(ProcesoActivo),
    Prueba { agente: Agente, pid: u32, operacion_id: String },
    Entrevista(Respuesta<Invocacion>),
    Evaluacion(Respuesta<Invocacion<Propuesta>>),
    Error { mensaje: String },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ProcesoActivo {
    pub agente: Agente,
    pub pid: u32,
    pub canal: String,
    pub ollama: String,
    pub modelo: String,
    pub digest: String,
}

#[derive(Clone)]
pub struct Supervisor {
    procesos: Arc<HashMap<Agente, Mutex<Option<Proceso>>>>,
    ejecutable: Arc<PathBuf>,
    ollama: Arc<String>,
}

struct Proceso {
    hijo: Child,
    entrada: ChildStdin,
    salida: BufReader<ChildStdout>,
    ficha: ProcesoActivo,
}

impl Supervisor {
    pub async fn iniciar(ejecutable: &Path, ollama: &str) -> Result<Self> {
        let mut procesos = HashMap::new();
        for agente in AGENTES {
            let proceso = Proceso::iniciar(ejecutable, agente, ollama).await?;
            procesos.insert(agente, Mutex::new(Some(proceso)));
        }
        Ok(Self { procesos: Arc::new(procesos), ejecutable: Arc::new(ejecutable.into()), ollama: Arc::new(ollama.into()) })
    }

    pub async fn inventario(&self) -> Vec<ProcesoActivo> {
        let mut fichas = Vec::new();
        for agente in AGENTES {
            if let Some(proceso) = self.procesos[&agente].lock().await.as_ref() {
                fichas.push(proceso.ficha.clone());
            }
        }
        fichas
    }

    pub async fn enviar(&self, agente: Agente, peticion: Peticion) -> Result<Acuse> {
        let supervisor = self.clone();
        // Un cliente que se desconecta no interrumpe la lectura de su respuesta del canal.
        tokio::spawn(async move { supervisor.enviar_en_canal(agente, &peticion).await }).await?
    }

    async fn enviar_en_canal(&self, agente: Agente, peticion: &Peticion) -> Result<Acuse> {
        let mut puesto = self.procesos[&agente].lock().await;
        let repuesto = match puesto.as_mut() {
            None => true,
            Some(proceso) => proceso.hijo.try_wait()?.is_some(),
        };
        if repuesto {
            *puesto = Some(Proceso::iniciar(&self.ejecutable, agente, &self.ollama).await?);
        }
        let proceso = puesto.as_mut().context("Falta el trabajador seleccionado")?;
        let espera = if agente == Agente::Evaluador { TIEMPO_EVALUADOR + Duration::from_secs(12) } else { Duration::from_secs(192) };
        let respuesta = timeout(espera, proceso.intercambiar(peticion)).await;
        let error = match respuesta {
            Ok(Ok(acuse)) => return Ok(acuse),
            Ok(Err(error)) => error,
            Err(error) => anyhow!("Se agotó el tiempo en el canal del trabajador: {error}"),
        };
        if let Some(proceso) = puesto.take() {
            proceso.cerrar().await;
        }
        Err(error)
    }

    pub async fn apagar(&self) {
        for agente in AGENTES {
            if let Some(proceso) = self.procesos[&agente].lock().await.take() {
                proceso.cerrar().await;
            }
        }
    }
}

impl Proceso {
    async fn iniciar(ejecutable: &Path, agente: Agente, ollama: &str) -> Result<Self> {
        let mut hijo = Command::new(ejecutable)
            .arg("trabajador").arg("--agente").arg(agente.id()).arg("--ollama-url").arg(ollama)
            .env_clear()
            .stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::inherit())
            .kill_on_drop(true).spawn().with_context(|| format!("No se inició {}", agente.id()))?;
        let pid = hijo.id().context("El trabajador no tiene PID")?;
        let entrada = hijo.stdin.take().context("El trabajador no tiene entrada")?;
        let mut salida = BufReader::new(hijo.stdout.take().context("El trabajador no tiene salida")?);
        let inicio: Acuse = timeout(Duration::from_secs(5), leer(&mut salida)).await
            .context("El trabajador no confirmó el canal")??;
        let Acuse::Inicio(ficha) = inicio else { return Err(anyhow!("Acuse de inicio inválido")) };
        ensure!(ficha == ficha_proceso(agente, pid, ollama), "El trabajador confirmó una identidad o configuración distintas");
        eprintln!("SIRCLE_TRABAJADOR {}", serde_json::to_string(&ficha)?);
        Ok(Self { hijo, entrada, salida, ficha })
    }

    async fn intercambiar(&mut self, peticion: &Peticion) -> Result<Acuse> {
        escribir(&mut self.entrada, peticion).await?;
        leer(&mut self.salida).await
    }

    async fn cerrar(mut self) {
        let _ = self.hijo.kill().await;
    }
}

pub fn ficha_proceso(agente: Agente, pid: u32, ollama: &str) -> ProcesoActivo {
    ProcesoActivo {
        agente, pid, canal: format!("stdin/stdout:{}", agente.id()),
        ollama: ollama.into(), modelo: MODELO.into(), digest: DIGEST_MODELO.into(),
    }
}

pub async fn escribir<W: AsyncWrite + Unpin>(salida: &mut W, mensaje: &impl Serialize) -> Result<()> {
    let mut bytes = serde_json::to_vec(mensaje)?;
    ensure!(bytes.len() < LIMITE_MENSAJE as usize, "El mensaje excede el límite del canal");
    bytes.push(b'\n');
    salida.write_all(&bytes).await?;
    salida.flush().await?;
    Ok(())
}

pub async fn leer<R: AsyncBufRead + Unpin, T: DeserializeOwned>(entrada: &mut R) -> Result<T> {
    let mut bytes = Vec::new();
    let leidos = entrada.take(LIMITE_MENSAJE).read_until(b'\n', &mut bytes).await?;
    if leidos == 0 {
        return Err(std::io::Error::new(std::io::ErrorKind::UnexpectedEof, "Canal cerrado").into());
    }
    ensure!(bytes.last() == Some(&b'\n'), "Mensaje sin terminador o demasiado largo");
    Ok(serde_json::from_slice(&bytes)?)
}
