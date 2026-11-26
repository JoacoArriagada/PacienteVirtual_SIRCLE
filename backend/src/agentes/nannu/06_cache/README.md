# Nannu · P02 · Cache

**Propósito.** Resolver la sugerencia literal con la ficha de la voz activa.

**Justificación.** E16 necesita conservar la atribución incluso cuando una selección usa un texto similar.

**Fuentes autorizadas.** Sugerencias asociadas a los ID de la ficha sp_actor vigente de P02 y filtradas por hablante y fase. [Guion del actor de P02](<../../../../../datos/medsp1000/mep_2374-8265.10924-s001/scenario3/sp_actor/C. Pneumonia with Septic Shock.md>).

**Comportamiento esperado.** La consulta directa encuentra la sugerencia únicamente en el conjunto de su propia voz. Otra voz conserva su lista de candidatos.

**Implementación.** [shared/ficha_aprobada.rs](<../../../shared/ficha_aprobada.rs>).

**Prueba.** `separa_sugerencias_por_hablante` comprueba la coincidencia literal de tos solo al dirigirse al acompañante. [agente_nannu.rs](<../../../../tests/agente_nannu.rs>).
