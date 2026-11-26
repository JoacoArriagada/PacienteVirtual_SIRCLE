# Evaluador formativo · Identidad acceso

**Propósito.** Delimitar el contexto evaluativo a la entrega asignada.

**Justificación.** E10, E13 y E16 protegen las claves educativas, los borradores y la historia de otros intentos.

**Fuentes autorizadas.** Una entrega confirmada con acceso por asignación. La ficha de entrada admite evidencias de su propio intento y la rúbrica fijada. [Pauta de origen de LR](<../../../../../datos/medsp1000/mededportal_10181/scenario1/evaluator/Case A-Asthma.md>).

**Comportamiento esperado.** La entrada rechaza evidencia ajena y campos de encuestas o claves. El estudiante accede a la devolución mediante la publicación autorizada.

**Implementación.** [shared/autorizacion.rs](<../../../shared/autorizacion.rs>).

**Prueba.** `excluye_entregas_y_encuestas_ajenas` rechaza evidencia de otro intento y campos educativos en el contrato del trabajador. [agente_evaluador.rs](<../../../../tests/agente_evaluador.rs>).
