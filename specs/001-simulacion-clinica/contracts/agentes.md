# Contrato interno y mapa de los cuatro agentes

El backend iniciará cuatro procesos trabajadores. Cada proceso tendrá una entrada y un canal local propios. El mensaje JSON por turno llevará identificador de operación, intento, agente, caso y versión, fase, hablante y únicamente los hechos o evidencias ya autorizados. La respuesta incluirá el mismo identificador de operación, tipo de resultado y hecho o citas seleccionados. El backend validará esa relación antes de persistir. Los trabajadores comparten los pesos de `qwen3.5:4b` por Ollama y carecen de acceso directo a SQLite. [Especificación de los trabajadores](../spec.md#hito-3--habilitar-los-cuatro-trabajadores-y-comprobar-sus-ocho-capas)

## Cómo localizar una decisión

Cada fila define el futuro `README.md` ubicado dentro del directorio del agente y el núcleo indicado. Las rutas de implementación de las tablas parten de `backend/src/`. El archivo de prueba aparece bajo el título del agente y cada fila nombra el caso que se podrá ejecutar con Cargo. Son rutas **previstas**. Los directorios, el código y las pruebas se crearán durante la implementación.

Cada README documentará propósito, justificación y requisito, fuentes autorizadas con versión y fase, comportamiento esperado, ruta de implementación y archivo y nombre de prueba. Por ejemplo, `backend/src/agentes/nannu/02_fuentes_recuperacion_memoria/README.md` explicará el origen de la tos productiva, enlazará al módulo compartido de contexto y a la prueba específica de Nannu. Los directorios numerados organizan la documentación. Los archivos Rust y sus módulos pueden tener nombres convencionales.

Cuando dos agentes compartan un archivo, cada README enlazará la misma ruta y describirá su regla y prueba propias. Para el núcleo de MCP bastan las operaciones internas de la API. Para caché se documentará la consulta directa a la ficha o a la entrega que satisface esa necesidad. Las sugerencias exactas conservan caso, versión, fase y hablante. El historial personal siempre se reconstruye para el intento vigente. El despachador del backend aplica las reglas de enrutamiento antes de enviar un mensaje al trabajador. Una medición o una orden confirmada se resuelve en valoración o exámenes aun si la entrevista libre está indisponible.

## LR · P01

Directorio previsto `backend/src/agentes/lr/`. Archivo de pruebas previsto `backend/tests/agente_lr.rs`.

El ensayo por HTTP del hito 1 y el chat del hito 2 usarán la misma operación de entrevista de P01. Qwen recibirá el texto libre y candidatos del actor en la versión y fase del intento. El backend validará el hecho elegido y su atribución antes de guardar la respuesta. La integración de los cuatro procesos y la comprobación de sus 32 núcleos corresponden al hito 3. El actor de LR aporta el relato y el desconocimiento de su familia por adopción. Los hallazgos del instrumento y el ECG pertenecen al controlador. La adaptación española debe enlazar el pasaje y la versión aprobada. [Actor de LR](<../../../datos/medsp1000/mededportal_10181/scenario1/sp_actor/Case A-Asthma.md>) · [Fuentes y adaptación](../../../datos/adaptaciones/lr/procedencia.md)

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

Directorio previsto `backend/src/agentes/nannu/`. Archivo de pruebas previsto `backend/tests/agente_nannu.rs`.

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

Directorio previsto `backend/src/agentes/dan_sparks/`. Archivo de pruebas previsto `backend/tests/agente_dan_sparks.rs`.

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

Directorio previsto `backend/src/agentes/evaluador/`. Archivo de pruebas previsto `backend/tests/agente_evaluador.rs`.

El evaluador recibe la entrega confirmada, sus evidencias y las versiones clínica y de rúbrica fijadas al abrir el intento. Comprueba la huella de la rúbrica de la entrega, incluso después de un cambio de ponderaciones. La pauta original orienta la adaptación a pregrado avanzado. Las encuestas y los intentos ajenos quedan fuera de la entrada. [Pauta original P01](<../../../datos/medsp1000/mededportal_10181/scenario1/evaluator/Case A-Asthma.md>) · [Evaluación y publicación](../spec.md#historia-4--entregar-y-obtener-una-devolución-revisada-por-el-docente-p1)

| Núcleo | Fuente, regla y motivo de revisión | Archivo de implementación | Caso de prueba y escenario |
| --- | --- | --- | --- |
| `01_rol_objetivo_modelo/` | Entrega confirmada y rúbrica de pregrado avanzado. El agente propone observaciones por criterio. | `agentes/evaluador/perfil.rs` | `evalua_solo_entrega_confirmada` · E11, E16 |
| `02_fuentes_recuperacion_memoria/` | Copia inmutable y referencias del intento. Cada crédito apunta al registro disponible en esa entrega. | `modules/evaluacion/evidencia.rs` | `cita_la_entrega_y_su_version` · E12 |
| `03_herramientas_mcp/` | Lectura interna de evidencias y rúbrica. El borrador carece de operación para fijar ponderaciones o publicar. | `shared/operaciones.rs` | `propuesta_no_publica` · E12, E13 |
| `04_identidad_acceso/` | Asignación de la entrega y del docente. Se restringe el contexto a ese trabajo, incluso si hay otros pendientes. | `shared/autorizacion.rs` | `excluye_entregas_y_encuestas_ajenas` · E10, E13, E16 |
| `05_controles_entrada_contexto_salida/` | Rúbrica, acuses y evidencias disponibles. Contrastar las cinco partes de cada observación y derivar el registro técnico ausente al docente. | `modules/evaluacion/criterios.rs` | `calibrar_aciertos_omisiones_e_incidencias` · E12 |
| `06_cache/` | Trabajo ligado a entrega única. Releer el resultado de esa entrega y preservar las rectificaciones al reintentar. | `modules/evaluacion/trabajos.rs` | `recupera_resultado_de_la_misma_entrega` · E11, E22 |
| `07_enrutamiento/` | Entrega pendiente y toma vigente del trabajo. Al recuperarse Qwen se confirma una sola evaluación. | `modules/evaluacion/trabajos.rs` | `persiste_una_evaluacion_tras_recuperacion` · E22 |
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
