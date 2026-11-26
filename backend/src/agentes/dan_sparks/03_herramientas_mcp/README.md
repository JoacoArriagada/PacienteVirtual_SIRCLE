# Dan Sparks · P03 · Herramientas mcp

**Propósito.** Consultar el relato y el registro de disponibilidad del medio por operaciones internas.

**Justificación.** E10 y E16 separan el relato del acceso al informe del controlador.

**Fuentes autorizadas.** Relato sp_actor en P03. El examen se documenta bajo environment_controller y su referencia se conserva en la versión de ensayo del caso. [Guion del actor de P03](<../../../../../datos/medsp1000/mededportal_3171/scenario1/sp_actor/Pulmonary Embolism Case Description.md>).

**Comportamiento esperado.** Preguntar por un examen no habilita su consulta. La consulta HTTP sin orden autorizada queda reservada y omite el medio.

**Implementación.** [shared/operaciones.rs](<../../../shared/operaciones.rs>).

**Prueba.** `protege_informe_del_controlador` comprueba la reserva de la consulta del ECG sin orden en P03. [agente_dan_sparks.rs](<../../../../tests/agente_dan_sparks.rs>).
