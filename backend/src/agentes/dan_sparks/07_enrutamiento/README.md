# Dan Sparks · P03 · Enrutamiento

**Propósito.** Separar la conversación del paciente y la consulta del examen del controlador.

**Justificación.** E16 vincula cada operación al rol que habilita su fuente.

**Fuentes autorizadas.** Actor sp_actor para el relato. El Markdown Pulmonary Embolism EKG del controlador declara el JPG en la misma versión P03. [Declaración del ECG](<../../../../../datos/medsp1000/mededportal_3171/scenario1/environment_controller/Pulmonary Embolism EKG.md>) · [Guion del actor de P03](<../../../../../datos/medsp1000/mededportal_3171/scenario1/sp_actor/Pulmonary Embolism Case Description.md>).

**Comportamiento esperado.** El relato va a Dan. El registro del medio conserva su propia revisión y permanece indisponible mientras falte el JPG.

**Implementación.** [agentes/dan_sparks/enrutamiento.rs](<../enrutamiento.rs>).

**Prueba.** `separa_relato_e_informe` comprueba las revisiones distintas de ambos documentos y el estado del medio. [agente_dan_sparks.rs](<../../../../tests/agente_dan_sparks.rs>).
