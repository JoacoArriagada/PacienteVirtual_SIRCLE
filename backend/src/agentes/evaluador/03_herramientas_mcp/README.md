# Evaluador formativo · Herramientas mcp

**Propósito.** Usar la lectura interna de la entrega y entregar una propuesta revisable.

**Justificación.** E12 y E13 reservan ponderaciones y publicación al docente asignado.

**Fuentes autorizadas.** Copia del intento confirmado y rúbrica aprobada. Cualquier revisión o publicación requiere identidad de docente y asignación vigente. [Pauta de origen de LR](<../../../../../datos/medsp1000/mededportal_10181/scenario1/evaluator/Case A-Asthma.md>).

**Comportamiento esperado.** La respuesta del trabajador contiene criterios y juicios. La autorización para publicar se comprueba con el docente responsable.

**Implementación.** [modules/evaluacion/publicacion.rs](<../../../modules/evaluacion/publicacion.rs>).

**Prueba.** `propuesta_no_publica` comprueba que el trabajador carece de una acción de publicación y que la revisión exige rol docente. [agente_evaluador.rs](<../../../../tests/agente_evaluador.rs>).
