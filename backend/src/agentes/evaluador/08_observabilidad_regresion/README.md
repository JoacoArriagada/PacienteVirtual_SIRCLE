# Evaluador formativo · Observabilidad regresion

**Propósito.** Dejar referencias que permitan revisar un criterio y su evidencia.

**Justificación.** E12 y E18 requieren poder reconstruir el juicio y verificar la procedencia.

**Fuentes autorizadas.** Revisión docente de la rúbrica y las evidencias congeladas de la entrega. La traza se vincula por el criterio y el trabajo autorizado. [Pauta de origen de LR](<../../../../../datos/medsp1000/mededportal_10181/scenario1/evaluator/Case A-Asthma.md>).

**Comportamiento esperado.** El resultado conserva criterio, referencias y huella. El docente puede seguir esos ID hasta el registro de origen y su revisión.

**Implementación.** [shared/trazas.rs](<../../../shared/trazas.rs>).

**Prueba.** `rastrear_criterio_a_evidencia_y_regresion` sigue un juicio desde el docente hasta la entrega, la operación del trabajador y la respuesta del motor. [agente_evaluador.rs](<../../../../tests/agente_evaluador.rs>).
