# Evaluador formativo · Fuentes recuperacion memoria

**Propósito.** Ligarlo a la copia inmutable y a los registros citables de la entrega.

**Justificación.** E12 exige recuperar la evidencia que sustenta cada juicio y la rúbrica empleada.

**Fuentes autorizadas.** La devolución usa la versión y huella de rúbrica que se fijaron en esa entrega, junto a turno, medición, orden, informe u otro registro admitido de su intento. [Pauta de origen de LR](<../../../../../datos/medsp1000/mededportal_10181/scenario1/evaluator/Case A-Asthma.md>).

**Comportamiento esperado.** Una observación conserva los ID de evidencia. Cambiar la huella o citar un registro de otra entrega produce rechazo.

**Implementación.** [modules/evaluacion/evidencia.rs](<../../../modules/evaluacion/evidencia.rs>).

**Prueba.** `reconstruye_desde_una_entrega_y_su_rubrica_congeladas` compara los IDs, huella y registros con la instantánea confirmada. [agente_evaluador.rs](<../../../../tests/agente_evaluador.rs>).
