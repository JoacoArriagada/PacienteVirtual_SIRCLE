# Dan Sparks · P03 · Rol objetivo modelo

**Propósito.** Mantener la identidad narrativa del paciente Dan Sparks.

**Justificación.** E16 requiere conservar el personaje al alternar P01 y P03.

**Fuentes autorizadas.** El actor sp_actor de la versión fijada de P03 da el relato en fase inicial y tras resolver el pre. [Guion del actor de P03](<../../../../../datos/medsp1000/mededportal_3171/scenario1/sp_actor/Pulmonary Embolism Case Description.md>).

**Comportamiento esperado.** Al venir de LR, el trabajador de P03 recibe solo el contexto de Dan y el relato se atribuye a ese paciente.

**Implementación.** [agentes/dan_sparks/perfil.rs](<../perfil.rs>).

**Prueba.** `mantiene_identidad_despues_de_lr` comprueba la identidad y la memoria de Dan después de guardar un turno de LR. [agente_dan_sparks.rs](<../../../../tests/agente_dan_sparks.rs>).
