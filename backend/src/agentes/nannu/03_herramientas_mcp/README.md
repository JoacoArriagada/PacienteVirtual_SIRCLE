# Nannu · P02 · Herramientas mcp

**Propósito.** Recuperar el dato del acompañante mediante la consulta interna de hechos.

**Justificación.** E16 exige que dirigir una pregunta al familiar conserve la procedencia del relato.

**Fuentes autorizadas.** La operación se limita a hechos sp_actor de P02 con la voz Acompañante autorizada en la versión y fase del intento. [Guion del actor de P02](<../../../../../datos/medsp1000/mep_2374-8265.10924-s001/scenario3/sp_actor/C. Pneumonia with Septic Shock.md>).

**Comportamiento esperado.** La consulta al familiar puede seleccionar la tos productiva. Pedir en ese contexto el hecho de dolor de la paciente se rechaza.

**Implementación.** [shared/operaciones.rs](<../../../shared/operaciones.rs>).

**Prueba.** `consulta_al_acompanante` comprueba el origen del candidato después de seleccionar al acompañante. [agente_nannu.rs](<../../../../tests/agente_nannu.rs>).
