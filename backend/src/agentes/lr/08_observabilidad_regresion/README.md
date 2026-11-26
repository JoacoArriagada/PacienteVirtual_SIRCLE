# LR · P01 · Observabilidad regresion

**Propósito.** Conservar el origen por turno para revisión docente.

**Justificación.** E04 y E18 exigen llegar a la operación, fase, versión y pasaje a partir del turno.

**Fuentes autorizadas.** Traza SQLite restringida al docente responsable. El pasaje procede del actor P01 en la versión ligada al turno. [Guion del actor de LR](<../../../../../datos/medsp1000/mededportal_10181/scenario1/sp_actor/Case A-Asthma.md>).

**Comportamiento esperado.** La revisión identifica la operación y el origen. En texto libre, el registro conserva modelo, digest y selección de la llamada de Ollama. El titular recibe el turno de estudiante.

**Implementación.** [shared/trazas.rs](<../../../shared/trazas.rs>).

**Prueba.** `localiza_origen_de_turno_lr` comprueba la recuperación de la fuente y línea con la identidad docente asignada. [agente_lr.rs](<../../../../tests/agente_lr.rs>).
