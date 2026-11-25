# MedSP1000: análisis para el piloto respiratorio

Fecha: 6 de noviembre de 2026. Estado: investigación y propuesta para revisión docente; no se ha implementado ni validado clínicamente el piloto.

## 1. Qué se revisó y dónde estaba el análisis anterior

El antecedente detallado está en la propuesta de Semana 2 (antecedente histórico: `../../../Semana2/Propuesta_implementacion_paciente_virtual_MedSP1000.md`) y en su carpeta de investigación (antecedente histórico: `../../../Semana2/Investigacion_MedSP1000/README.md`). Las cinco propuestas de Semana 4 (antecedente histórico: `../../Propuestas/README.md`) comparan arquitecturas apoyadas en ese trabajo. Por ello, esta entrega une ambos antecedentes, sin atribuir a Semana 4 una auditoría distinta que no encontré allí.

Se inventariaron los Markdown de la copia local completa y se recalcularon las rúbricas del ZIP previamente fijado. Para la selección clínica se buscaron materiales respiratorios, se contrastaron perfiles alternativos y se inspeccionaron los guiones y materiales de entorno/examinado de los tres candidatos elegidos. Es una selección intencional, no una revisión clínica exhaustiva de los 1.639 escenarios. No se ejecutó el simulador de terceros ni se entrenó un modelo.

El registro reproducible (antecedente histórico: `Auditoria_local_2026-11-06.json`) contiene rutas, tamaños y SHA-256 de los archivos seleccionados. La revisión del código sigue siendo `2806984cb331d6fedc2c0555e3c1c4a54171c77a`; el SHA-256 del ZIP coincide con el conservado en Semana 2. El inventario actual identifica la copia local por sus archivos, sin certificar que todo el directorio corresponda a una única revisión remota.

## 2. Qué es el dataset y qué aporta

MedSP1000 reúne escenarios derivados de materiales de pacientes estandarizados. Su investigación evalúa agentes que actúan como clínicos mediante interacción y criterios docentes. El artículo anuncia 1.638 casos, 17 especialidades y 24.602 entradas de evaluación. Es evidencia sobre un benchmark de agentes; no demuestra por sí misma aprendizaje de estudiantes humanos usando nuestro sistema. [Artículo original](https://arxiv.org/abs/2606.05112).

La unidad práctica es una carpeta `case_id/scenarioN`, con cuatro grupos de documentos. `sp_actor` contiene el guion del paciente; `examinee`, materiales para el examinado; `environment_controller`, información del entorno y evolución; `evaluator`, materiales de evaluación. El repositorio se distribuye como árbol de archivos, no como una tabla de diálogos lista para entrenar. [Descripción del dataset](https://huggingface.co/datasets/byrLLCC/MedSP1000).

Para SIRCLE se propone que el estudiante humano ocupe el lugar del examinado. El actor conversacional representa al paciente; el sistema conserva eventos y evidencia; un motor de reglas revisadas determina cambios; el evaluador compara la actuación con una rúbrica. Esta separación es una decisión de nuestro proyecto. El código original consulta un agente de entorno y aplica su respuesta; no debe presentarse como un motor fisiológico determinista ya disponible. [Controlador auditado](https://github.com/MAGIC-AI4Med/MedSP1000/blob/2806984cb331d6fedc2c0555e3c1c4a54171c77a/src/simulate/controller.py).

El diagrama DS-01 (antecedente histórico: `../Diagramas/DS-01.svg`) representa origen, cuatro roles, selección, controles, aprobación y uso durante un intento. Separa visualmente la estructura observada del proceso que proponemos construir.

## 3. Resultados de la revisión local

| Comprobación | Resultado | Interpretación |
| --- | ---: | --- |
| Archivos con extensión `.md` | 9.960 | Incluye documentos fuera de los cuatro roles. |
| Archivos dentro de los cuatro roles | 9.958 | 2.645 entorno; 3.549 evaluación; 1.852 examinado; 1.912 actor. |
| Rutas de escenarios | 1.639 | Una más que las 1.638 anunciadas en el artículo. |
| Directorios superiores con escenarios | 613 | No equivale a 613 publicaciones únicas deduplicadas. |
| Escenarios con los cuatro roles | 1.639 | Existencia de carpetas no demuestra suficiencia clínica. |
| Rúbricas en el ZIP de código | 1.638 | Recurso separado de los guiones. |
| Objetos JSON legibles | 1.634 | Cuatro archivos están vacíos. |
| Objetos válidos sin entradas de categorías | 86 | JSON válido no garantiza una evaluación utilizable. |
| Entradas sumadas de categorías | 24.602 | Reproduce el conteo anterior; no son 24.602 conversaciones. |

El análisis anterior contaba 9.961 archivos listados de todos los tipos en el repositorio remoto. Aquí se cuentan 9.960 Markdown locales: son denominadores distintos y no evidencia de pérdida de un caso. La discrepancia de escenarios sí permanece. Debe registrarse, sin inventar una causa ni mezclar revisiones de datos y código.

Para reproducir esta revisión, ejecutar `Fuentes/auditar_dataset.py` desde esta entrega. Lee el corpus local y el ZIP; solo escribe el nuevo JSON de investigación.

## 4. Área clínica y criterio de selección

Propongo la unidad didáctica **Evaluación inicial del adulto con síntomas respiratorios**. Es una denominación del piloto, no una unidad curricular oficial de la universidad. Se sitúa en semiología y razonamiento clínico, con contenidos de neumología y contexto de medicina interna/urgencias. La adscripción curricular exacta debe confirmarla el profesor.

La conexión entre los casos será el problema que enfrenta el estudiante: dificultad para respirar, tos o dolor asociado a la respiración. Los mecanismos son distintos: obstrucción de la vía aérea, infección pulmonar y obstrucción vascular pulmonar. Así se reutilizan las mismas pantallas sin enseñar que todos los síntomas respiratorios tienen una única explicación. GINA describe el carácter variable de síntomas como sibilancias, disnea, opresión y tos; NICE contempla la evaluación de neumonía y de sospecha de embolia pulmonar por síntomas que pueden solaparse. [GINA 2026](https://ginasthma.org/wp-content/uploads/2026/07/GINA-Summary-Guide-2026-WEB-WMS.pdf), [NICE NG250](https://www.nice.org.uk/guidance/ng250), [NICE NG158](https://www.nice.org.uk/guidance/ng158/chapter/recommendations).

La selección prioriza trazabilidad, entrevista, observación, razonamiento y una primera reevaluación. Los originales contienen urgencias graves: no se presentan como tres casos clínicamente sencillos. El piloto debe dirigirse a estudiantes de Medicina con semiología y evaluación inicial ya trabajadas, bajo supervisión; el curso concreto queda pendiente.

## 5. Tres candidatos comprobados

Esta sección es material de diseño y contiene soluciones. Para estudiantes se preparó un [archivo separado sin diagnósticos](<fichas-para-estudiantes.md>).

| Prioridad | Caso del corpus | Perfil original | Problema de entrada | Diagnóstico reservado / alcance |
| --- | --- | --- | --- | --- |
| 1 | `mededportal_10181/scenario1` | LR, mujer de 34 años | Falta de aire; evolución de dos días | Exacerbación asmática; el original agrega una taquicardia supraventricular. |
| 2 | `mep_2374-8265.10924-s001/scenario3` | Nanu, mujer de 61 años | Fiebre y tos de tres días; decaimiento | Neumonía con shock séptico en el original. |
| 3 | `mededportal_3171/scenario1` | Dan Sparks, hombre de 43 años | Disnea de comienzo agudo, cuatro horas | Embolia pulmonar; caso original de simulación para estudiantes de tercer año. |

Son tres edades y dos sexos descritos en los guiones. No se infiere identidad de género más allá de la información de las fuentes. La diversidad resulta de seleccionar perfiles existentes, no de cambiar arbitrariamente edad o sexo de una historia. Pueden utilizarse nombres ficticios en español, pero eso debe quedar documentado como adaptación. Las fichas propuestas conservan las iniciales o nombres originales.

**Primero: caso de 34 años.** Permite probar entrevista, antecedentes, exploración, hipótesis, acción confirmada y reevaluación. El guion contiene signos de compromiso respiratorio; no basta con sostener una conversación extensa. El entorno introduce una arritmia a los cinco minutos independientemente del tratamiento. Para la primera demostración propongo recortar el objetivo a evaluación inicial y solicitud de ayuda, dejando esa complicación fuera de la versión piloto, con aprobación docente explícita. Es una modificación pedagógica, no una reproducción íntegra. [Publicación de origen](https://www.mededportal.org/doi/10.15766/mep_2374-8265.10181).

**Segundo: caso de 61 años.** Amplía la entrevista a paciente y familiar, incorpora alergias, un examen solicitado y el reconocimiento de deterioro sistémico. El material original ya inicia con hipotensión y otros signos de gravedad; no corresponde renombrarlo “neumonía leve” ni mantener a la paciente conversando como si estuviera estable. El piloto puede terminar cuando el estudiante reconoce la urgencia, explica su hipótesis y escala la atención. La simulación de reanimación y manejo intensivo quedaría para una fase posterior. [Publicación de origen](https://www.mededportal.org/doi/10.15766/mep_2374-8265.10924).

**Tercero: caso de 43 años.** Introduce el contraste entre disnea y una explicación vascular. El guion aporta comienzo súbito, molestia torácica al respirar y dolor en una pantorrilla al preguntar; esos datos apoyan el razonamiento, pero ninguno confirma aisladamente el diagnóstico. El ejercicio original exige integrar entrevista, exploración y pruebas. Su publicación es de 2009: las listas de pruebas y manejo necesitan revisión actual, no copiarse como protocolo vigente. [Caso original](https://www.mededportal.org/doi/10.15766/mep_2374-8265.3171), [evaluación actual de sospecha de embolia: NICE NG158](https://www.nice.org.uk/guidance/ng158/chapter/recommendations).

## 6. Hallazgos que afectan directamente al diseño

Las rúbricas JSON de estos candidatos tienen, respectivamente, **9, 15 y 6 entradas**. No equivalen a nueve, quince o seis conductas atómicas. Por ejemplo, una entrada puede agrupar varias acciones. El profesor debe descomponerlas en criterios observables, vincularlos a evidencias y definir qué no se evalúa en el recorte del piloto. No se recomienda convertir automáticamente cada entrada en un punto.

La carpeta `examinee` tampoco garantiza ausencia de soluciones. En el caso de embolia, el título del documento dirigido al examinado ya revela el diagnóstico. Los nombres de los tres archivos son igualmente reveladores. La importación necesita una vista pública construida y revisada, no una publicación directa del Markdown original.

En el caso de asma se encontraron marcadores de imagen del tipo `data:image/jpeg;base64...`, que no constituyen imágenes clínicas recuperables. También aparecen referencias a recursos externos con condiciones específicas. El piloto deberá usar informes textuales redactados y revisados, o conseguir recursos reales con procedencia y permisos comprobados. No debe presentar una imagen inventada como resultado del dataset.

El guion de asma contiene una indicación para contestar “normal” ante ciertos hallazgos no previstos. Para nuestro sistema se propone distinguir “no disponible/no modelado” de “normal”: inventar normalidad ocultaría una limitación del escenario. Esta adaptación debe mantenerse visible en la revisión clínica.

## 7. Decisión recomendada

Construir **un solo proyecto piloto**, con un caso inicial y dos ampliaciones del mismo bloque respiratorio. Empezar con P01, demostrar el recorrido completo y corregirlo; luego incorporar P02 y P03. Su mérito inicial será que la información aparece en el momento correcto, las acciones tienen trazabilidad y se puede retomar el intento. La eficacia educativa requerirá un estudio posterior.

Cada versión debe conservar procedencia, cambios respecto al original, contenido público/reservado, acciones representables, tiempos, resultados, condiciones de cierre y revisión docente. La etiqueta MIT del repositorio agregado no sustituye la revisión de permisos de anexos e imágenes originales. Las fuentes web se consultaron el 6 de noviembre de 2026; algunas páginas de MedEdPORTAL bloquearon la apertura completa, por lo que la inspección de los guiones se fundamenta en los archivos locales identificados en el manifiesto.

Continuar con el plan del piloto (antecedente histórico: `../Piloto/Propuesta_piloto.md`) y las fichas docentes (antecedente histórico: `../Piloto/Fichas_docentes_reservadas.md`).
