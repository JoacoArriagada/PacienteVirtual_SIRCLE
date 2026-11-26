# Evaluador formativo · Enrutamiento

**Propósito.** Dirigir los mensajes de evaluación al trabajador evaluador.

**Justificación.** E22 verifica un único resultado tras recuperar el motor y reintentar el trabajo pendiente.

**Fuentes autorizadas.** La entrada de evaluación procede de la entrega confirmada y de una toma vigente de su trabajo. El canal se selecciona por la identidad evaluador. [Pauta de origen de LR](<../../../../../datos/medsp1000/mededportal_10181/scenario1/evaluator/Case A-Asthma.md>).

**Comportamiento esperado.** El contrato correlaciona la misma operación, caso, fase y versión con el borrador devuelto. La confirmación depende de la toma del trabajo.

**Implementación.** [modules/evaluacion/trabajos.rs](<../../../modules/evaluacion/trabajos.rs>).

**Prueba.** `descarta_resultado_tardio_de_toma_vencida_y_confirma_una_evaluacion` recupera el mismo trabajo, corta el resultado vencido y conserva una sola evaluación. [recuperacion_e22.rs](<../../../../tests/recuperacion_e22.rs>).
