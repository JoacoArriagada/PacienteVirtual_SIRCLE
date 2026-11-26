# LR · P01 · Controles entrada contexto salida

**Propósito.** Validar el significado de la selección del trabajador antes de registrar.

**Justificación.** E03–E06 y E18 requieren distinguir ambigüedad, desconocimiento documentado y dato ausente.

**Fuentes autorizadas.** Entrada de la sesión estudiantil y ficha de hechos sp_actor en la fase inicial de la versión fijada. [Guion del actor de LR](<../../../../../datos/medsp1000/mededportal_10181/scenario1/sp_actor/Case A-Asthma.md>).

**Comportamiento esperado.** Una sugerencia editada sigue la ruta libre. Una selección ajena al relato o sin pertinencia se descarta por el servidor.

**Implementación.** [agentes/lr/entrevista.rs](<../entrevista.rs>).

**Prueba.** `distingue_los_limites_del_relato` comprueba aclaración, pasaje documentado y dato ausente sin cambiar la ficha. [agente_lr.rs](<../../../../tests/agente_lr.rs>).

La prueba `distingue_los_limites_del_relato` cubre los tres resultados en intentos independientes. T047 a T049 consignan las llamadas al motor en los recorridos de ensayo.
