# Contrato interno y mapa de los cuatro agentes

El backend inicia cuatro procesos trabajadores. Cada proceso tiene entrada y canal local propios. El mensaje JSON identifica operación, intento, agente, caso y versión, fase, hablante y hechos o evidencias ya autorizados. La respuesta incluye la misma cabecera, tipo de resultado y selección o propuesta. Axum valida esa relación antes de persistir. Los trabajadores comparten los pesos de `qwen3.5:4b` por Ollama y carecen de acceso directo a SQLite. [Especificación de los trabajadores](../spec.md#hito-3--habilitar-los-cuatro-trabajadores-y-comprobar-sus-ocho-capas)

La solicitud de entrevista usa `{cabecera, contenido}`. La cabecera contiene `operacion_id`, `intento_id`, `agente`, `caso`, `version`, `fase` y `hablante`. El contenido incluye texto y candidatos con ID y condición. La respuesta usa `{cabecera, tipo, resultado}` y devuelve la selección y los datos de la invocación. Axum valida la cabecera completa, el tipo y el ID de la invocación antes de confirmar el turno. Una discrepancia deja su motivo en `rechazos_mensajes` y el historial confirmado conserva su revisión. La traza restringida del turno confirmado incluye la cabecera.

Axum reconstruye la memoria de entrevista desde SQLite después de verificar el acceso. Envía hasta los tres intercambios confirmados más recientes, ordenados cronológicamente y filtrados por estudiante, intento, agente, versión, fase y hablante. Los trabajadores reciben esa memoria en cada solicitud y el contexto del modelo respeta los mismos límites después de un reinicio. La selección final se vuelve a validar en Axum.

## Cómo localizar una decisión

Cada fila identifica el `README.md` dentro del directorio del agente y del núcleo indicado. Las rutas de implementación de las tablas parten de `backend/src/`. Bajo cada agente se indica su archivo de pruebas. La fila de recuperación del evaluador indica el archivo específico de E22. Cada caso se puede ejecutar con Cargo. La matriz de cierre enlaza los 32 README, las implementaciones y los casos verificados. [Matriz T065](../evidencias/h3/T065/matriz.tsv)

Cada README documenta propósito, justificación y requisito, fuentes autorizadas con versión y fase, comportamiento esperado, implementación y prueba. El núcleo de fuentes de Nannu remite al contexto compartido y a la prueba que localiza el pasaje de tos productiva. [Fuentes de Nannu](../../../backend/src/agentes/nannu/02_fuentes_recuperacion_memoria/README.md)

Los README enlazan los módulos compartidos y describen la regla y prueba propias de cada agente. El núcleo de MCP usa las operaciones internas de la API. La ficha versionada resuelve sugerencias literales y la entrega conserva el resultado de evaluación. Las sugerencias exactas conservan caso, versión, fase y hablante. El historial se reconstruye para el intento vigente. El despachador aplica las reglas de enrutamiento antes de invocar al trabajador. Valoración y exámenes atienden las acciones confirmadas incluso si la entrevista libre está indisponible.

## LR · P01

Directorio `backend/src/agentes/lr/`. Archivo de pruebas `backend/tests/agente_lr.rs`.

El ensayo por HTTP y el chat integrado llaman a la misma operación de entrevista de P01. Qwen recibe el texto libre y candidatos del actor en la versión y fase del intento. Axum valida el hecho y su atribución antes de guardar la respuesta. El actor aporta el relato de LR y el desconocimiento de sus antecedentes familiares por adopción. Los hallazgos instrumentales y el ECG proceden del controlador. La adaptación enlaza el pasaje y la versión correspondiente. [Actor de LR](<../../../datos/medsp1000/mededportal_10181/scenario1/sp_actor/Case A-Asthma.md>) · [Fuentes y adaptación](../../../datos/adaptaciones/lr/procedencia.md)

| Núcleo | Fuente, regla y motivo de revisión | Archivo de implementación | Caso de prueba y escenario |
| --- | --- | --- | --- |
| `01_rol_objetivo_modelo/` | Ficha del actor P01. LR habla en primera persona y mantiene su rol si se pregunta por la nota. | `agentes/lr/perfil.rs` | `mantiene_voz_de_lr` · E16 |
| `02_fuentes_recuperacion_memoria/` | Hechos P01 aprobados y turnos del intento. La respuesta sobre la familia enlaza el pasaje de adopción. | `shared/contexto.rs` | `cita_desconocimiento_familiar` · E04 |
| `03_herramientas_mcp/` | Operaciones internas del relato. Preguntar por la presión no ejecuta el instrumento ni crea medición. | `shared/operaciones.rs` | `preguntar_por_presion_no_mide` · E07, E12 |
| `04_identidad_acceso/` | Asignación, propietario y fase. Las dos preguntas simultáneas se resuelven solo con sus historiales respectivos. | `shared/autorizacion.rs` | `aisla_dos_preguntas_simultaneas` · E17 |
| `05_controles_entrada_contexto_salida/` | Texto recibido y ficha de hechos permitidos. Validar el identificador y el significado. Una edición sigue la ruta libre. Separar referente ambiguo, desconocimiento y dato ausente. | `agentes/lr/entrevista.rs` | `distingue_los_limites_del_relato` · E03–E06, E18 |
| `06_cache/` | Sugerencia literal y ficha de la versión fijada. Identificar el hecho sin inferencia, sin conservar respuestas de otros intentos. | `shared/ficha_aprobada.rs` | `sugerencia_exacta_con_motor_caido` · E06, E22 |
| `07_enrutamiento/` | Acción y fase. Enrutar conversación, toma de presión y orden de ECG según el gesto confirmado. | `agentes/lr/enrutamiento.rs` | `separa_turno_medicion_y_orden` · E09, E22 |
| `08_observabilidad_regresion/` | Traza restringida de P01. En una pregunta libre, la operación del turno identifica la llamada a Ollama, modelo y digest, hecho seleccionado, versión, pasaje y regresión. | `shared/trazas.rs` | `localiza_origen_de_turno_lr` · E04, E18 |

## Nannu · P02

Directorio `backend/src/agentes/nannu/`. Archivo de pruebas `backend/tests/agente_nannu.rs`.

En el original figura Nanu. La paciente refiere dolor y el acompañante describe la tos productiva. Los dos hablantes pertenecen al mismo escenario. [Guion P02](<../../../datos/medsp1000/mep_2374-8265.10924-s001/scenario3/sp_actor/C. Pneumonia with Septic Shock.md>)

| Núcleo | Fuente, regla y motivo de revisión | Archivo de implementación | Caso de prueba y escenario |
| --- | --- | --- | --- |
| `01_rol_objetivo_modelo/` | Guion P02. El trabajador mantiene la identidad del caso y distingue las dos voces. | `agentes/nannu/perfil.rs` | `mantiene_paciente_y_acompanante` · E16 |
| `02_fuentes_recuperacion_memoria/` | Pasaje P02 y hablante autorizado. Dolor y tos productiva conservan sus dos atribuciones. | `shared/contexto.rs` | `atribuye_el_dolor_y_la_tos` · E16 |
| `03_herramientas_mcp/` | Consulta interna del relato P02 con hablante. Una declaración del familiar produce un turno con ese origen. | `shared/operaciones.rs` | `consulta_al_acompanante` · E16 |
| `04_identidad_acceso/` | Intento y asignación de P02. Cambiar de hablante conserva el propietario y la fase. | `shared/autorizacion.rs` | `cambiar_hablante_conserva_propietario` · E16, E17 |
| `05_controles_entrada_contexto_salida/` | Hecho, versión y hablante. Rechazar una frase de la paciente construida con un pasaje del familiar. | `agentes/nannu/entrevista.rs` | `rechaza_atribucion_incorrecta` · E16, E18 |
| `06_cache/` | Ficha aprobada de P02. Una sugerencia se resuelve usando el hablante activo, incluso cuando otro hablante usó un texto similar. | `shared/ficha_aprobada.rs` | `separa_sugerencias_por_hablante` · E16 |
| `07_enrutamiento/` | Destinatario y turno identificados. Llevar la pregunta a la voz autorizada y recuperar un turno interrumpido una sola vez. | `agentes/nannu/enrutamiento.rs` | `retoma_turno_del_familiar` · E16, E17 |
| `08_observabilidad_regresion/` | Traza P02. La revisión identifica el hablante esperado, el pasaje empleado y la prueba que detecta el cruce. | `shared/trazas.rs` | `localiza_fallo_de_atribucion` · E16, E18 |

## Dan Sparks · P03

Directorio `backend/src/agentes/dan_sparks/`. Archivo de pruebas `backend/tests/agente_dan_sparks.rs`.

El actor sitúa el inicio hace cuatro horas mientras trabajaba en el jardín. El controlador es la fuente separada de los informes. El JPG de ECG ausente se registra como medio indisponible. [Actor de Dan Sparks](<../../../datos/medsp1000/mededportal_3171/scenario1/sp_actor/Pulmonary Embolism Case Description.md>) · [Fuentes P03](../../../datos/medsp1000/mededportal_3171/scenario1/)

| Núcleo | Fuente, regla y motivo de revisión | Archivo de implementación | Caso de prueba y escenario |
| --- | --- | --- | --- |
| `01_rol_objetivo_modelo/` | Guion del actor P03. Dan sigue hablando como Dan después de entrevistar a LR. | `agentes/dan_sparks/perfil.rs` | `mantiene_identidad_despues_de_lr` · E16 |
| `02_fuentes_recuperacion_memoria/` | Hechos P03 por fase e intento. Recuperar las cuatro horas de su relato tras una pregunta sobre el inicio. | `shared/contexto.rs` | `recupera_cuatro_horas_de_p03` · E16 |
| `03_herramientas_mcp/` | Relato e informes del controlador. La consulta de un examen requiere la operación y el permiso de la fase. | `shared/operaciones.rs` | `protege_informe_del_controlador` · E10, E16 |
| `04_identidad_acceso/` | Asignación y caso del intento. Un identificador de P01 no habilita acceso al contexto de Dan. | `shared/autorizacion.rs` | `excluye_contexto_de_p01` · E16–E18 |
| `05_controles_entrada_contexto_salida/` | Pasaje P03 y disponibilidad del medio. Descartar hechos ajenos y conservar la incertidumbre ante una imagen ausente. | `agentes/dan_sparks/entrevista.rs` | `rechaza_hecho_ajeno_o_imagen_ausente` · E16, E18 |
| `06_cache/` | Ficha P03 versionada. Una sugerencia sobre el inicio recupera la cronología de Dan tras usar el mismo texto con LR. | `shared/ficha_aprobada.rs` | `sugerencia_literal_respeta_p03` · E06, E16 |
| `07_enrutamiento/` | Tipo de acción en P03. El relato se dirige al actor y los informes a la operación autorizada del entorno. | `agentes/dan_sparks/enrutamiento.rs` | `separa_relato_e_informe` · E16 |
| `08_observabilidad_regresion/` | Traza restringida P03. Se puede reproducir una confusión de cronología alternando versiones y casos identificados. | `shared/trazas.rs` | `rastrear_cronologia_de_dan` · E16, E18 |

## Evaluador formativo

Directorio `backend/src/agentes/evaluador/`. Archivo de pruebas `backend/tests/agente_evaluador.rs`. La recuperación se verifica en `backend/tests/recuperacion_e22.rs`.

El evaluador recibe la entrega confirmada, sus evidencias y las versiones clínica y de rúbrica fijadas al abrir el intento. Comprueba la huella de la rúbrica de la entrega, incluso después de un cambio de ponderaciones. La pauta original orienta la adaptación a pregrado avanzado. Las encuestas y los intentos ajenos quedan fuera de la entrada. [Pauta original P01](<../../../datos/medsp1000/mededportal_10181/scenario1/evaluator/Case A-Asthma.md>) · [Evaluación y publicación](../spec.md#historia-4--entregar-y-obtener-una-devolución-revisada-por-el-docente-p1)

| Núcleo | Fuente, regla y motivo de revisión | Archivo de implementación | Caso de prueba y escenario |
| --- | --- | --- | --- |
| `01_rol_objetivo_modelo/` | Entrega confirmada y rúbrica de pregrado avanzado. El agente propone observaciones por criterio. | `agentes/evaluador/perfil.rs` | `evalua_solo_entrega_confirmada` · E11, E16 |
| `02_fuentes_recuperacion_memoria/` | Copia inmutable y referencias del intento. Cada crédito apunta al registro disponible en esa entrega. | `modules/evaluacion/evidencia.rs` | `reconstruye_desde_una_entrega_y_su_rubrica_congeladas` · E11, E12 |
| `03_herramientas_mcp/` | Lectura interna de evidencias y rúbrica. El docente asignado decide qué publicar. | `modules/evaluacion/publicacion.rs` | `propuesta_no_publica` · E12, E13 |
| `04_identidad_acceso/` | Asignación de la entrega y del docente. Se restringe el contexto a ese trabajo, incluso si hay otros pendientes. | `shared/autorizacion.rs` | `excluye_entregas_y_encuestas_ajenas` · E10, E13, E16 |
| `05_controles_entrada_contexto_salida/` | Rúbrica, acuses y evidencias disponibles. Cotejar las partes de cada observación y derivar el registro ausente al docente. | `modules/evaluacion/criterios.rs` | `distingue_error_omision_y_acuse_sin_evento` · E12 |
| `06_cache/` | Trabajo ligado a entrega única. Releer el resultado y su revisión con los pesos guardados. | `modules/evaluacion/trabajos.rs` | `rectifica_con_motivo_y_reproduce_desglose_de_v1_despues_de_v2` · E11, E22 |
| `07_enrutamiento/` | Entrega pendiente y toma vigente. Recuperar el trabajo y descartar el resultado de una toma vencida. | `modules/evaluacion/trabajos.rs` | `descarta_resultado_tardio_de_toma_vencida_y_confirma_una_evaluacion` · E22 |
| `08_observabilidad_regresion/` | Criterio, evidencia, rúbrica y traza. La revisión docente llega al pasaje original y a la prueba de calibración o incidencia. | `shared/trazas.rs` | `rastrear_criterio_a_evidencia_y_regresion` · E12, E18 |

## Pruebas que deciden el paso al hito 4

**Entrevista e aislamiento.** Para LR se ejecutan por separado E03, E04 y E05. «¿Y eso?» sin referente produce una petición de aclaración. Al preguntar por la familia, LR responde que desconoce los antecedentes por adopción, según el guion. Al preguntar por el grupo sanguíneo, el sistema indica que el dato no está disponible en el escenario y deja la ficha sin hallazgo nuevo. La falta de registro nunca se traduce en una negativa clínica.

En E17 dos sesiones envían, antes de que termine la primera respuesta, una pregunta sobre el inicio y otra sobre los antecedentes familiares. El despachador separa las solicitudes por estudiante, intento, agente y operación. Una cola puede escalonar la inferencia. La traza debe demostrar que ambas solicitudes estuvieron pendientes a la vez, que se utilizaron los hechos correctos y que tras reanudar hay exactamente un par pregunta–respuesta en cada historial. Repetir la prueba alternando P02 y P03, e interrumpir el trabajador de Nannu para verificar la recuperación de su turno sin contaminar los otros.

**Calibración de pregrado avanzado.** El estudiante fundamenta una hipótesis principal y contrasta al menos una alternativa con evidencia del intento. Para cada criterio observable de entrevista, valoración, decisiones, uso de evidencias e hipótesis, el evaluador entrega cinco elementos. Debe identificar la actuación, citar el registro que la acredita, indicar qué esperaba la rúbrica, explicar el acierto o la brecha y proponer una mejora concreta para otra práctica. Un criterio fuera de las oportunidades del escenario se marca como no evaluable.

| Entrega preparada | Devolución esperada |
| --- | --- |
| Tres aciertos de cuatro | Reconocer por separado las tres hipótesis justificadas y citar sus evidencias. Para la cuarta, indicar el vínculo ausente y pedir que se contraste un hallazgo disponible o se explicite qué dato falta. |
| Un acierto de cuatro | Reconocer la primera hipótesis. Identificar la contradicción con el informe de ECG en la segunda, la alternativa sin justificar en la tercera y la confusión entre declaración y medición en la cuarta. La clave de prueba prioriza releer el ECG y corregir la segunda. |

Cuatro hipótesis es el tamaño de estas dos entregas de prueba. Una decisión equivocada tiene un acto documentado que discrepa del criterio. La omisión exige que el acto haya sido posible y que un registro íntegro muestre que faltó. Si existe un acuse pero falta el evento, la incidencia pasa a revisión docente sin descuento automático. El docente aprueba las ponderaciones antes de habilitar la actividad. Puede rectificar la evaluación con motivo y decide qué revisión publica. Las ponderaciones nuevas se cargan con otra versión de la rúbrica y se asignan a intentos posteriores. [Entrega, revisión y visibilidad](api.md)

**Seguimiento de fallos.** Partir de un turno incorrecto o de un criterio sin respaldo. El revisor deberá llegar al identificador de intento y operación, al agente y núcleo, a las versiones del caso, el modelo, las instrucciones y la rúbrica que corresponda, y al pasaje, hecho o evidencia utilizados. La traza indica razón del rechazo o diferencia con lo esperado, latencia, archivo y nombre de la regresión de la matriz. Una aclaración o incidencia técnica se rastrea hasta la decisión registrada sin inventar un hecho. Al salir del hito deben existir las 32 rutas de README y resolver cada enlace de implementación y prueba.
