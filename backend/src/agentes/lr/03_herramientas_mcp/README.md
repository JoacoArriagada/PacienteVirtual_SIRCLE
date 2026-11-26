# LR · P01 · Herramientas mcp

**Propósito.** Resolver el relato mediante la consulta interna de hechos y proteger el gesto de valoración.

**Justificación.** E07 y E12 requieren distinguir una pregunta de una toma confirmada.

**Fuentes autorizadas.** La entrevista usa sp_actor en la fase inicial de la versión fijada. Los actos se autorizan por propietario, actividad y estado. [Guion del actor de LR](<../../../../../datos/medsp1000/mededportal_10181/scenario1/sp_actor/Case A-Asthma.md>).

**Comportamiento esperado.** Preguntar por presión inicia una operación de entrevista. El instrumento solo registra una medición con su confirmación autorizada.

**Implementación.** [shared/operaciones.rs](<../../../shared/operaciones.rs>).

**Prueba.** `preguntar_por_presion_no_mide` comprueba el recuento de mediciones después de una petición textual de presión. [agente_lr.rs](<../../../../tests/agente_lr.rs>).
