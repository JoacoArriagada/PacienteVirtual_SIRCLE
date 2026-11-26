# Dan Sparks · P03 · Controles entrada contexto salida

**Propósito.** Validar fuente, fase y disponibilidad antes de aceptar una selección.

**Justificación.** E16 y E18 exigen rechazar hechos de otro caso y sostener la disponibilidad real del medio.

**Fuentes autorizadas.** Guion sp_actor de P03 y registro de disponibilidad del medio declarado por environment_controller en la versión fijada. [Guion del actor de P03](<../../../../../datos/medsp1000/mededportal_3171/scenario1/sp_actor/Pulmonary Embolism Case Description.md>).

**Comportamiento esperado.** Una selección ajena se descarta. Como el JPG referenciado falta, el catálogo lo conserva como indisponible.

**Implementación.** [agentes/dan_sparks/entrevista.rs](<../entrevista.rs>).

**Prueba.** `rechaza_hecho_ajeno_o_imagen_ausente` comprueba el rechazo de una selección de LR y la disponibilidad observada del JPG. [agente_dan_sparks.rs](<../../../../tests/agente_dan_sparks.rs>).
