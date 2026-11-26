# Dan Sparks · P03 · Identidad acceso

**Propósito.** Verificar identidad, asignación y el caso que fija cada intento.

**Justificación.** E16–E18 impiden reutilizar un intento P01 para construir un contexto de Dan.

**Fuentes autorizadas.** Sesión, actividad asignada de P03, versión de ensayo y fase inicial con pre resuelto. [Guion del actor de P03](<../../../../../datos/medsp1000/mededportal_3171/scenario1/sp_actor/Pulmonary Embolism Case Description.md>).

**Comportamiento esperado.** P01 admite su propio relato y no entrega contexto de Dan ni una referencia de ECG de P03.

**Implementación.** [shared/autorizacion.rs](<../../../shared/autorizacion.rs>).

**Prueba.** `excluye_contexto_de_p01` comprueba el rechazo del destino de Dan y la ausencia del medio de P03 en P01. [agente_dan_sparks.rs](<../../../../tests/agente_dan_sparks.rs>).
