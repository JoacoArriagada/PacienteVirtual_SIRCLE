# Evaluador formativo · Controles entrada contexto salida

**Propósito.** Verificar la estructura de cada observación y sus citas.

**Justificación.** E12 requiere actuación, evidencia, expectativa, explicación y mejora. Las ausencias técnicas pasan a revisión docente.

**Fuentes autorizadas.** Rúbrica fijada, base de evidencias de la entrega y juicios tipados. Cada criterio tiene una expectativa asociada. [Pauta de origen de LR](<../../../../../datos/medsp1000/mededportal_10181/scenario1/evaluator/Case A-Asthma.md>).

**Comportamiento esperado.** El borrador conserva una fila por criterio. Un juicio de acierto o error requiere una cita citable de la entrega.

**Implementación.** [modules/evaluacion/criterios.rs](<../../../modules/evaluacion/criterios.rs>).

**Prueba.** `distingue_error_omision_y_acuse_sin_evento` coteja el acto, la omisión y la incidencia de registro con la devolución. [agente_evaluador.rs](<../../../../tests/agente_evaluador.rs>).
