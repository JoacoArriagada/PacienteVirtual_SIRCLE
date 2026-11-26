# Nannu · P02 · Observabilidad regresion

**Propósito.** Ubicar el rol, el hablante y el pasaje que justifican la atribución.

**Justificación.** E18 exige que una atribución incorrecta se pueda localizar y reproducir.

**Fuentes autorizadas.** Revisión de la fuente sp_actor de P02, versionada por SHA-256. La traza queda bajo el permiso de la asignación docente. [Guion del actor de P02](<../../../../../datos/medsp1000/mep_2374-8265.10924-s001/scenario3/sp_actor/C. Pneumonia with Septic Shock.md>).

**Comportamiento esperado.** La revisión muestra el ID de la fuente, línea y fragmento de la voz seleccionada. Un cruce de hablante se diagnostica por la correlación del mensaje.

**Implementación.** [shared/trazas.rs](<../../../shared/trazas.rs>).

**Prueba.** `localiza_fallo_de_atribucion` sigue una selección rechazada hasta la voz y el pasaje del acompañante. [agente_nannu.rs](<../../../../tests/agente_nannu.rs>).

La prueba `localiza_fallo_de_atribucion` verifica el descarte y su motivo persistido. T050 consigna además las dos trazas de Qwen.
