# Nannu · P02 · Controles entrada contexto salida

**Propósito.** Comprobar el candidato y su voz antes de confirmar una respuesta.

**Justificación.** E16 y E18 exigen detectar un dato que se atribuye al hablante equivocado.

**Fuentes autorizadas.** Candidato de la versión y fase de P02 seleccionado desde el rol sp_actor de la voz autorizada. [Guion del actor de P02](<../../../../../datos/medsp1000/mep_2374-8265.10924-s001/scenario3/sp_actor/C. Pneumonia with Septic Shock.md>).

**Comportamiento esperado.** Una respuesta armada con un candidato de la otra voz se descarta aunque el candidato haya sido añadido al conjunto por error.

**Implementación.** [agentes/nannu/entrevista.rs](<../entrevista.rs>).

**Prueba.** `rechaza_atribucion_incorrecta` comprueba el rechazo de un candidato del familiar en el contexto de la paciente. [agente_nannu.rs](<../../../../tests/agente_nannu.rs>).
