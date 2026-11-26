# Nannu · P02 · Fuentes recuperacion memoria

**Propósito.** Construir la respuesta desde el pasaje y el historial de la voz solicitada.

**Justificación.** E16 distingue el dolor referido por la paciente de la tos productiva observada por su familiar.

**Fuentes autorizadas.** El guion sp_actor de P02, línea 5 de la revisión importada, contiene «R sided chest pain when breathing.» y «Family will say she has had a productive cough.». Ambos hechos conservan la versión de ensayo y fase inicial. [Guion del actor de P02](<../../../../../datos/medsp1000/mep_2374-8265.10924-s001/scenario3/sp_actor/C. Pneumonia with Septic Shock.md>).

**Comportamiento esperado.** Nannu relata su dolor de pecho al respirar. El acompañante refiere que la paciente ha tosido con flema. Cada selección guarda su hablante y su fragmento de origen.

**Implementación.** [shared/contexto.rs](<../../../shared/contexto.rs>).

**Prueba.** `atribuye_el_dolor_y_la_tos` comprueba las dos atribuciones, la versión común y los pasajes distintos. [agente_nannu.rs](<../../../../tests/agente_nannu.rs>).
