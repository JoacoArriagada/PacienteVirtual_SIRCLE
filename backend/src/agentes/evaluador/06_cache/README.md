# Evaluador formativo · Cache

**Propósito.** Correlacionar cada borrador con la entrega que activó su trabajo.

**Justificación.** E11 y E22 conservan la entrega y las revisiones al repetir el trabajo.

**Fuentes autorizadas.** La consulta interna usa ID de trabajo, entrega y huella de rúbrica, bajo los permisos de esa operación. [Pauta de origen de LR](<../../../../../datos/medsp1000/mededportal_10181/scenario1/evaluator/Case A-Asthma.md>).

**Comportamiento esperado.** La relectura por entrega reutiliza el resultado correspondiente. Un borrador con otro identificador se descarta.

**Implementación.** [modules/evaluacion/trabajos.rs](<../../../modules/evaluacion/trabajos.rs>).

**Prueba.** `rectifica_con_motivo_y_reproduce_desglose_de_v1_despues_de_v2` recupera la revisión de V1 y mantiene sus pesos al activar V2. `rastrear_criterio_a_evidencia_y_regresion` verifica que la cola no vuelva a evaluar una entrega ya terminada. [agente_evaluador.rs](<../../../../tests/agente_evaluador.rs>).
