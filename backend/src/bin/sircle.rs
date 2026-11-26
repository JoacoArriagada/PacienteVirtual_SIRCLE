use std::{net::SocketAddr, path::PathBuf};

use clap::{Parser, Subcommand};
use sircle::{agentes::mensajes::Agente, app::{estado::Estado, router::router, trabajadores},
    modules::{escenarios::importacion, evaluacion::{publicacion, rubricas, trabajos}, intentos::intento}, sqlite};
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
            let estado = estado.con_trabajadores(supervisor.clone());
            let cola = estado.clone();
            let cola = tokio::spawn(trabajos::cola(cola));
            println!("SIRCLE API en http://{}", listener.local_addr()?);
            let resultado = axum::serve(listener, router(estado))
                .with_graceful_shutdown(trabajadores::senal_de_cierre()).await;
            cola.abort();
            supervisor.apagar().await;
            resultado?;
        }
        Comando::Preparar { db, fuentes } => {
            let inventario = importacion::preparar(&db, &fuentes)?;
            println!("escenario\tfuente_id\trevision_id\tversion_id\tversion\trol\truta\tsha256");
            for fila in inventario {
                println!("{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
                    fila.escenario, fila.fuente_id, fila.revision_id, fila.version_id,
                    fila.version, fila.rol, fila.ruta, fila.sha256);
            }
        }
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
