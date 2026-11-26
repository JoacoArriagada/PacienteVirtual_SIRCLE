# LR · P01 · Rol objetivo modelo

**Propósito.** Fijar LR como voz de la paciente P01 y mantener su relato en primera persona.

**Justificación.** FR-004 y E16 exigen una voz identificable incluso cuando se intenta trasladar la conversación a la nota.

**Fuentes autorizadas.** P01 usa el rol sp_actor de la versión fijada al abrir el intento, con pre resuelto y fase inicial. [Guion del actor de LR](<../../../../../datos/medsp1000/mededportal_10181/scenario1/sp_actor/Case A-Asthma.md>).

**Comportamiento esperado.** Axum selecciona el trabajador de LR. Una pregunta sobre la nota no confirma un hecho del actor ni cambia el agente. La pregunta libre usa el Qwen fijado en el inventario de T040.

**Implementación.** [agentes/lr/perfil.rs](<../perfil.rs>).

**Prueba.** `mantiene_voz_de_lr` comprueba el hablante asignado, la selección fuera de significado y el rechazo de otro escenario. [agente_lr.rs](<../../../../tests/agente_lr.rs>).
