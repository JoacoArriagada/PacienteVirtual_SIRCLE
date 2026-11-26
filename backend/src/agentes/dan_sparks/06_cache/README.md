# Dan Sparks · P03 · Cache

**Propósito.** Resolver una pregunta literal desde la ficha activa.

**Justificación.** E06 y E16 exigen recuperar el candidato del caso vigente incluso si la frase coincide con una usada en P01.

**Fuentes autorizadas.** Sugerencias referidas al hecho del actor P03 y a la versión de ensayo fijada en fase inicial. [Guion del actor de P03](<../../../../../datos/medsp1000/mededportal_3171/scenario1/sp_actor/Pulmonary Embolism Case Description.md>).

**Comportamiento esperado.** La consulta directa resuelve la misma pregunta con el hecho de inicio del paciente actual. Reabrir vuelve a construir el contexto.

**Implementación.** [shared/ficha_aprobada.rs](<../../../shared/ficha_aprobada.rs>).

**Prueba.** `sugerencia_literal_respeta_p03` comprueba la diferencia de hecho cuando la sugerencia exacta se usa con LR y con Dan. [agente_dan_sparks.rs](<../../../../tests/agente_dan_sparks.rs>).
