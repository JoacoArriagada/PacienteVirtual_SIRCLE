# Nannu · P02 · Enrutamiento

**Propósito.** Dirigir el turno de P02 a la voz solicitada que autoriza el escenario.

**Justificación.** E16 y E17 exigen conservar el destino al reabrir y al recuperar una operación interrumpida.

**Fuentes autorizadas.** Fase inicial, actividad de ensayo de P02 habilitada y asignada, versión fijada en el intento y destinatario validado. [Guion del actor de P02](<../../../../../datos/medsp1000/mep_2374-8265.10924-s001/scenario3/sp_actor/C. Pneumonia with Septic Shock.md>).

**Comportamiento esperado.** Una segunda conexión reconstruye el destino del acompañante desde el mismo intento. El supervisor reconoce ese destino por operación y canal.

**Implementación.** [agentes/nannu/enrutamiento.rs](<../enrutamiento.rs>).

**Prueba.** `retoma_turno_del_familiar` interrumpe el trabajador, verifica continuidad de LR y reintenta la misma operación de Nannu. [agente_nannu.rs](<../../../../tests/agente_nannu.rs>).

La prueba `retoma_turno_del_familiar` verifica con Qwen el reintento tras interrumpir solo ese trabajador. T053 conserva los PID y acuses.
