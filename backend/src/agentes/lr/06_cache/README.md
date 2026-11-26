# LR · P01 · Cache

**Propósito.** Resolver la sugerencia exacta mediante la ficha autorizada.

**Justificación.** E06 y E22 requieren un resultado literal disponible cuando falla el motor local.

**Fuentes autorizadas.** Sugerencia literal y hecho vigente de LR con rol sp_actor, fase inicial y versión propia del intento. [Guion del actor de LR](<../../../../../datos/medsp1000/mededportal_10181/scenario1/sp_actor/Case A-Asthma.md>).

**Comportamiento esperado.** La consulta directa de la ficha da el hecho y su procedencia. El reenvío recupera el acuse de la misma operación autorizada.

**Implementación.** [shared/ficha_aprobada.rs](<../../../shared/ficha_aprobada.rs>).

**Prueba.** `sugerencia_exacta_con_motor_caido` comprueba el camino literal y su reenvío sin un motor disponible. [agente_lr.rs](<../../../../tests/agente_lr.rs>).
