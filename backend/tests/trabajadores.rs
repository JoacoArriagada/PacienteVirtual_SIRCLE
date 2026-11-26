use std::{collections::HashSet, path::Path};

use serde_json::json;
use sircle::{
    agentes::{
        mensajes::{Agente, Cabecera, Pregunta, Solicitud},
        supervisor::{Acuse, Peticion, Supervisor},
    },
    modules::entrevista::ollama::{DIGEST_MODELO, MODELO},
};

#[tokio::test]
async fn inicia_cuatro_canales_y_cada_uno_responde_con_su_identidad() {
    let destino = "http://127.0.0.1:11434";
    let supervisor = Supervisor::iniciar(Path::new(env!("CARGO_BIN_EXE_sircle")), destino).await.unwrap();
    let inventario = supervisor.inventario().await;
    assert_eq!(inventario.len(), 4);
    assert_eq!(inventario.iter().map(|ficha| ficha.pid).collect::<HashSet<_>>().len(), 4);
    assert_eq!(inventario.iter().map(|ficha| &ficha.canal).collect::<HashSet<_>>().len(), 4);
    assert_eq!(inventario.iter().map(|ficha| ficha.agente).collect::<HashSet<_>>().len(), 4);
    let mut comprobaciones = Vec::new();
    for ficha in &inventario {
        assert_eq!(ficha.modelo, MODELO);
        assert_eq!(ficha.digest, DIGEST_MODELO);
        assert_eq!(ficha.ollama, destino);
        assert_ne!(ficha.pid, std::process::id());
        let operacion = format!("t040-prueba-{}", ficha.agente.id());
        let acuse = supervisor.enviar(ficha.agente, Peticion::Probar { operacion_id: operacion.clone() }).await.unwrap();
        let Acuse::Prueba { agente, pid, operacion_id } = acuse else { panic!("Acuse inesperado: {acuse:?}") };
        assert_eq!((agente, pid, operacion_id.as_str()), (ficha.agente, ficha.pid, operacion.as_str()));
        comprobaciones.push(json!({ "agente": agente, "pid": pid, "operacion_id": operacion_id }));
    }
    let cruce = Solicitud {
        cabecera: Cabecera { operacion_id: "t040-cruce".into(), intento_id: "prueba-t040".into(), agente: Agente::Lr,
            caso: "P01".into(), version: "ensayo".into(), fase: "inicial".into(), hablante: "LR".into() },
        contenido: Pregunta { texto: "Mensaje de prueba".into(), candidatos: Vec::new(), memoria: vec![] },
    };
    let acuse = supervisor.enviar(Agente::Nannu, Peticion::Entrevistar(cruce)).await.unwrap();
    assert!(matches!(acuse, Acuse::Error { ref mensaje } if mensaje == "El canal no corresponde a la entrevista"));
    assert_eq!(supervisor.inventario().await, inventario);
    supervisor.apagar().await;
    assert!(supervisor.inventario().await.is_empty());
    println!("EVIDENCIA {}", json!({
        "inventario": inventario, "pruebas_de_enrutamiento": comprobaciones,
        "canal_equivocado": acuse, "trabajadores_tras_cierre": 0,
    }));
}
