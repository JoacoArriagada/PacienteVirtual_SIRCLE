# Dan Sparks · P03 · Fuentes recuperacion memoria

**Propósito.** Recuperar la cronología y los intercambios confirmados de este intento.

**Justificación.** E16 exige distinguir el inicio de Dan de la cronología de LR.

**Fuentes autorizadas.** El pasaje de la línea 29 del actor P03 identifica el inicio hace cuatro horas mientras hacía un trabajo liviano en el jardín. La consulta usa la versión y fase del intento. [Guion del actor de P03](<../../../../../datos/medsp1000/mededportal_3171/scenario1/sp_actor/Pulmonary Embolism Case Description.md>).

**Comportamiento esperado.** El hecho seleccionado conserva las cuatro horas y el lugar. Al cambiar de caso se reconstruye el historial desde SQLite.

**Implementación.** [shared/contexto.rs](<../../../shared/contexto.rs>).

**Prueba.** `recupera_cuatro_horas_de_p03` comprueba la respuesta en primera persona, el jardín y la procedencia del actor. [agente_dan_sparks.rs](<../../../../tests/agente_dan_sparks.rs>).
