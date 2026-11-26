# Dan Sparks · P03 · Observabilidad regresion

**Propósito.** Permitir seguir la cronología de Dan hasta el pasaje fijado.

**Justificación.** E16 y E18 requieren poder reproducir un cruce de caso o una diferencia de cronología.

**Fuentes autorizadas.** Guion actor de P03, fase del intento y versión de la fuente. La traza clínica queda bajo autorización docente. [Guion del actor de P03](<../../../../../datos/medsp1000/mededportal_3171/scenario1/sp_actor/Pulmonary Embolism Case Description.md>).

**Comportamiento esperado.** El hecho permite cotejar su línea y pasaje con la revisión importada. T051 verificará los turnos confirmados al alternar Dan y LR.

**Implementación.** [shared/trazas.rs](<../../../shared/trazas.rs>).

**Prueba.** `rastrear_cronologia_de_dan` comprueba la línea original del relato desde el ID de revisión de fuente. [agente_dan_sparks.rs](<../../../../tests/agente_dan_sparks.rs>).
