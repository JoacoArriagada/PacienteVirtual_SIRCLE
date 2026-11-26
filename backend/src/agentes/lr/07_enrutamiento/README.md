# LR · P01 · Enrutamiento

**Propósito.** Separar entrevista, valoración y orden de examen por la operación confirmada.

**Justificación.** E09 y E22 mantienen disponible la valoración cuando la entrevista libre depende del motor.

**Fuentes autorizadas.** Relato del actor P01, lectura instrumental de la versión inicial y examen del controlador habilitado por una orden propia. [Guion del actor de LR](<../../../../../datos/medsp1000/mededportal_10181/scenario1/sp_actor/Case A-Asthma.md>).

**Comportamiento esperado.** La sugerencia exacta se resuelve en la ficha. La edición o pregunta libre va al trabajador de LR. La toma y la orden usan sus rutas independientes.

**Implementación.** [agentes/lr/enrutamiento.rs](<../enrutamiento.rs>).

**Prueba.** `separa_turno_medicion_y_orden` comprueba la separación de una conversación, una presión y una orden de ECG en un mismo intento. [agente_lr.rs](<../../../../tests/agente_lr.rs>).
