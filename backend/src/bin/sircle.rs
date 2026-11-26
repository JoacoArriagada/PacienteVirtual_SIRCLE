use std::{net::SocketAddr, path::PathBuf};

use clap::{Parser, Subcommand};
use sircle::{agentes::mensajes::Agente, app::{estado::Estado, router::router, trabajadores},
    modules::{entrevista::ollama::Ollama, escenarios::{formal, importacion}, evaluacion::{publicacion, rubricas, trabajos}, intentos::intento}, sqlite};
use tokio::net::TcpListener;

#[derive(Parser)]
#[command(about = "Servicio local de Paciente Virtual SIRCLE")]
struct Cli {
    #[command(subcommand)]
    comando: Comando,
}

#[derive(Subcommand)]
enum Comando {
    /// Servir la API local.
    Servir {
        #[arg(long, default_value = "127.0.0.1:8081")]
        direccion: SocketAddr,
        #[arg(long)]
        db: Option<PathBuf>,
    },
    /// Migrar SQLite e importar los originales del catálogo.
    Preparar {
        #[arg(long)]
        db: PathBuf,
        #[arg(long)]
        fuentes: PathBuf,
        #[arg(long)]
        rubrica_ensayo: bool,
        #[arg(long)]
        actividad_formal: Option<PathBuf>,
    },
    /// Exportar para revisión el material clínico, educativo y la rúbrica de LR.
    MaterialFormal {
        #[arg(long)]
        db: PathBuf,
        #[arg(long, default_value = "lr-v1")]
        version_clinica: String,
        #[arg(long, default_value = "lr-educativo-v1")]
        version_instrumento: String,
        #[arg(long, default_value = "1")]
        version_rubrica: String,
    },
    /// Calcular las huellas del material formal después de editar el borrador.
    HuellasFormales {
        #[arg(long)]
        archivo: PathBuf,
    },
    /// Generar una copia consistente de SQLite, incluso con la API activa.
    Respaldar {
        #[arg(long)]
        db: PathBuf,
        #[arg(long)]
        destino: PathBuf,
    },
    /// Omitir el pre en un intento técnico local.
    ResolverPreEnsayo {
        #[arg(long)]
        db: PathBuf,
        #[arg(long)]
        intento: String,
    },
    /// Marcar el post de una entrega en una prueba local de publicación.
    ResolverPostEnsayo {
        #[arg(long)]
        db: PathBuf,
        #[arg(long)]
        intento: String,
        #[arg(long)]
        estado: String,
    },
    /// Cargar una rúbrica privada de ensayo con su constancia opcional.
    CargarRubricaEnsayo {
        #[arg(long)]
        db: PathBuf,
        #[arg(long)]
        archivo: PathBuf,
    },
    /// Seleccionar la rúbrica aprobada para nuevos intentos de ensayo.
    HabilitarRubricaEnsayo {
        #[arg(long)]
        db: PathBuf,
        #[arg(long)]
        rubrica_id: i64,
    },
    #[command(hide = true)]
    Trabajador {
        #[arg(long)]
        agente: Agente,
        #[arg(long)]
        ollama_url: String,
    },
}

#[tokio::main(worker_threads = 2)]
async fn main() -> anyhow::Result<()> {
    match Cli::parse().comando {
        Comando::Servir { direccion, db } => {
            let estado = db.map(Estado::con_base).transpose()?.unwrap_or_default();
            let url = std::env::var("OLLAMA_URL").unwrap_or_else(|_| "http://127.0.0.1:11434".into());
            let listener = TcpListener::bind(direccion).await?;
            let supervisor = trabajadores::iniciar(&url).await?;
            let estado = estado.con_trabajadores(supervisor.clone()).con_ollama(Ollama::nueva(&url)?);
            let cola = estado.clone();
            let cola = tokio::spawn(trabajos::cola(cola));
            println!("SIRCLE API en http://{}", listener.local_addr()?);
            let resultado = axum::serve(listener, router(estado))
                .with_graceful_shutdown(trabajadores::senal_de_cierre()).await;
            cola.abort();
            supervisor.apagar().await;
            resultado?;
        }
        Comando::Preparar { db, fuentes, rubrica_ensayo, actividad_formal } => {
            let archivo_local = fuentes.join("adaptaciones/lr/actividad-formal.local.json");
            let actividad_formal = actividad_formal.or_else(|| archivo_local.is_file().then_some(archivo_local));
            let inventario = importacion::preparar_con_material(&db, &fuentes, rubrica_ensayo, actividad_formal.as_deref())?;
            println!("escenario\tfuente_id\trevision_id\tversion_id\tversion\trol\truta\tsha256");
            for fila in inventario {
                println!("{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
                    fila.escenario, fila.fuente_id, fila.revision_id, fila.version_id,
                    fila.version, fila.rol, fila.ruta, fila.sha256);
            }
        }
        Comando::MaterialFormal { db, version_clinica, version_instrumento, version_rubrica } => {
            let db = sqlite::abrir_existente(db)?;
            let material = formal::borrador(&db, version_clinica, version_instrumento, version_rubrica)?;
            println!("{}", serde_json::to_string_pretty(&material)?);
        }
        Comando::HuellasFormales { archivo } => {
            println!("{}", serde_json::to_string_pretty(&formal::huellas(&archivo)?)?);
        }
        Comando::Respaldar { db, destino } => sqlite::respaldar(&db, &destino)?,
        Comando::ResolverPreEnsayo { db, intento } => {
            intento::resolver_pre_ensayo(&mut sqlite::abrir_existente(db)?, &intento)?;
            println!("Pre de ensayo resuelto para {intento}");
        }
        Comando::ResolverPostEnsayo { db, intento, estado } => {
            publicacion::resolver_post_ensayo(&mut sqlite::abrir_existente(db)?, &intento, &estado)?;
            println!("Post de ensayo {estado} para {intento}");
        }
        Comando::CargarRubricaEnsayo { db, archivo } => {
            let registro = rubricas::cargar_archivo(&mut sqlite::abrir_existente(db)?, &archivo)?;
            println!("{}", serde_json::to_string(&registro)?);
        }
        Comando::HabilitarRubricaEnsayo { db, rubrica_id } => {
            rubricas::habilitar_ensayo(&sqlite::abrir_existente(db)?, rubrica_id)?;
            println!("Rúbrica {rubrica_id} habilitada para nuevos intentos de ensayo");
        }
        Comando::Trabajador { agente, ollama_url } => trabajadores::ejecutar(agente, &ollama_url).await?,
    }
    Ok(())
}
