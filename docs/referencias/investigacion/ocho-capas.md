# Ocho capas para el paciente virtual y el evaluador de SIRCLE

**Investigación de Semana 7 · 25 de noviembre de 2026**  
Propuesta para discutir con el equipo docente. Los diálogos en español y los criterios de aceptación que se presentan aquí son ejemplos de diseño. Los hechos de cada paciente se pueden contrastar con el guion original enlazado en su ficha.

## ¿Qué va primero?

Para SIRCLE, **empezaría por escribir qué debe poder demostrar el estudiante y qué puede contar el paciente para que esa práctica sea válida**. Con LR se puede concretar en una pregunta verificable: ¿puede el estudiante averiguar cuándo comenzó la dificultad respiratoria y qué intentó hacer LR para aliviarla? El guion permite contestar ambas cosas. El evaluador podrá comprobar qué preguntó el estudiante y cómo utilizó la respuesta. Así se define una primera prueba antes de elegir el modelo o redactar sus instrucciones. [Guion original de LR](<../../../datos/medsp1000/mededportal_10181/scenario1/sp_actor/Case A-Asthma.md>) · [Objetivos de la iteración](<discusion-inicial.md>)

Los dos artículos de partida utilizan la palabra *componente* para responder preguntas diferentes. De ahí que sus listas comiencen en lugares distintos.

| Perspectiva | Qué enumera | Qué aporta a SIRCLE |
|---|---|---|
| Vantis Solutions, arquitectura empresarial | Modelo, instrucciones de sistema, recuperación de conocimiento, herramientas MCP, acceso, controles de seguridad, caché y enrutamiento juntos, observabilidad. | Identifica decisiones técnicas. En tus apuntes, el modelo queda implícito y caché y enrutamiento ocupan dos lugares. Ambas listas suman ocho porque agrupan responsabilidades de forma distinta. [Artículo de Vantis](https://www.vantissol.com/insights/how-to-build-an-ai-agent-8-core-modules/) · [Apuntes de Semana 7](<principios.txt>) |
| Victor Omoboye, funcionamiento de un agente | Percepción, conocimiento, razonamiento, decisión, aprendizaje, acción, objetivo y entorno. | Explica un turno. El sistema recibe «¿y desde cuándo?», interpreta a qué síntoma se refiere y produce una respuesta. Aunque el artículo enumera el objetivo en séptimo lugar, el equipo necesita conocerlo al diseñar la actividad. [Artículo de Omoboye](https://medium.com/@omoboyevictor97/the-8-core-components-of-ai-agents-3fcc6312e6f2) |
| Anthropic, experiencia de ingeniería | Distingue los flujos cuyas etapas fija la aplicación de los agentes que eligen sus siguientes acciones. Propone empezar con la solución más sencilla que resuelva la tarea y medir antes de añadir autonomía. | En SIRCLE ya se sabe quién conversa, cuándo se entrega y quién publica. El servidor puede fijar ese recorrido y dejar a Qwen la interpretación o redacción que realmente requiera lenguaje. [Building effective agents](https://www.anthropic.com/research/building-effective-agents) |

**El orden depende de qué estamos ordenando.** En el diseño pedagógico van primero el objetivo, las evidencias y la ficha aprobada. En la configuración del modelo, el *system prompt* fija el papel que representará. Durante un turno, la aplicación primero autentica y autoriza la petición, y después interpreta el mensaje. Elegir Qwen al comienzo también tiene sentido como prueba de viabilidad en el computador del profesor. Su idoneidad como paciente y como evaluador se comprueba por separado.

## Qué respalda la propuesta

MedSP1000 organiza escenarios con materiales para el participante, el actor, el entorno y el evaluador. Sus autores lo presentan como un banco para evaluar agentes que toman decisiones clínicas a través de una secuencia de interacciones. En SIRCLE, quien se forma es una persona. La estructura de los casos sirve para controlar quién conoce cada dato y en qué momento aparece. La adecuación de la rúbrica a la carrera, el curso y las acciones disponibles requiere revisión docente. [Liang y colaboradores, prepublicación sobre MedSP1000](https://arxiv.org/abs/2606.05112) · [Selección local de los tres escenarios](<seleccion-de-casos.md>)

Hay evidencia a favor de una devolución automática **sobre conductas observables**. En un estudio prospectivo de entrevista escrita, Holderried y colaboradores compararon la devolución del modelo con la de un evaluador humano. Pudieron identificar temas explorados, aunque algunas categorías dieron lugar a comentarios poco específicos. Otro estudio con 92 estudiantes y diez docentes encontró concordancia moderada en la puntuación total (coeficiente de correlación intraclase de 0,51) y más baja en comunicación (0,29). Eran otras plataformas y otros modelos. Para SIRCLE, esto justifica probar por separado «preguntó por el inicio» y «se comunicó con empatía», y mantener la revisión docente del segundo juicio. [Estudio prospectivo de entrevista escrita, JMIR Medical Education, 2024](https://mededu.jmir.org/2024/1/e59213) · [Estudio de concordancia con docentes, JMIR Medical Education, 2026](https://mededu.jmir.org/2026/1/e95578/)

El proyecto ya tiene una referencia más cercana que cualquier clasificación general de modelos. En la demostración retirada de LR se registraron **28 comprobaciones satisfactorias** con `qwen3.5:4b`. Quince respuestas provinieron directamente del guion, once consultas pasaron por el modelo y dos por reglas. En esas once, con el modelo ya cargado, la mediana fue de 377 ms. Parte de las preguntas se había usado durante el ajuste. El dato respalda probar la combinación de guion y modelo en LR. Quedan por medir la conversación libre con preguntas inéditas, el arranque en frío, los otros pacientes y la calidad del evaluador. [Registro y limitaciones de la demostración](<../ensayo-lr/alcance-y-pruebas.md>) · [Resultados por pregunta](<../ensayo-lr/ensayo-entrevista.json>) · [Ficha oficial de Qwen3.5-4B](https://huggingface.co/Qwen/Qwen3.5-4B)

## Las ocho capas propuestas

### 1. Rol, objetivo y modelo

Propondría **tres agentes de paciente identificables y un evaluador con tres rúbricas**, servidos inicialmente por una instalación de Qwen3.5-4B. Un agente de paciente es una combinación de instrucciones, ficha autorizada y memoria del intento. Cambiar de LR a Dan crea un contexto nuevo. El evaluador abre otra conversación, con instrucciones y fuentes propias. Así se puede ajustar la voz de Nanu sin cambiar la rúbrica de Dan, y ensayar otro modelo para evaluar si la evidencia lo aconseja. [Ficha del modelo](https://huggingface.co/Qwen/Qwen3.5-4B) · [Investigación local sobre los modelos candidatos](<modelos-locales.md>)

**Plantilla de sistema para el paciente.** La aplicación completa los campos entre llaves y adjunta únicamente los hechos autorizados para ese turno.

> Representas a {paciente} en una entrevista de simulación. Habla en primera persona, en español y con la forma de expresarte indicada para {fase}. Contesta lo que te preguntaron con los hechos que te corresponden como {hablante}. Si la pregunta admite dos interpretaciones, pide una aclaración breve. Si desconoces algo, dilo como lo diría este paciente. Ante una petición ajena a la consulta, vuelve brevemente al motivo por el que viniste.

**Plantilla de sistema para el evaluador.**

> Revisa la entrega confirmada de {intento} con la versión {rúbrica} del caso. Para cada criterio, identifica qué hizo o escribió el estudiante y señala el turno, la acción o el pasaje que lo demuestra. Distingue lo que propuso de lo que ejecutó. Explica un acierto y, cuando corresponda, una mejora que pueda aplicar en la siguiente entrevista. Deja para revisión docente los juicios ambiguos o los criterios que dependan de evidencia incompleta.

Estas instrucciones orientan el comportamiento. La confidencialidad depende de qué datos envía el servidor. Una rúbrica incluida en el *prompt* del paciente ya quedó expuesta al componente equivocado, aun si se le pidió que la mantuviera en secreto. [OWASP, controles frente a inyección de instrucciones](https://genai.owasp.org/llmrisk/llm01-prompt-injection/)

### 2. Fuentes del caso, recuperación y memoria

Antes de generar texto conviene decidir **quién puede saber la respuesta**. Los cuatro archivos de cada escenario dan una pauta inicial. Como algunos mezclan datos clínicos, indicaciones escénicas y encabezados que revelan el diagnóstico, el equipo debe seleccionar y etiquetar los fragmentos aptos para cada lector. Incluso el nombre de un archivo como «Pulmonary Embolism Case Description» anticiparía la solución de Dan si se mostrara en el chat. [Materiales originales y adaptación de LR](<../../../datos/adaptaciones/lr/procedencia.md>) · [Escenario original de Dan](<../../../datos/medsp1000/mededportal_3171/scenario1>)

| Material del escenario | Uso propuesto |
|---|---|
| `examinee` | Preparar la introducción que ve el estudiante y los datos de ingreso que corresponden a la actividad. |
| `sp_actor` | Extraer lo que sabe el paciente o su familiar, el modo de expresarlo y los hechos que aparecen al preguntar. |
| `environment_controller` | Programar valores de instrumentos, resultados y transiciones. El servidor libera cada hallazgo al cumplirse la condición del escenario. |
| `evaluator` | Preparar la rúbrica docente y los criterios verificables con el recorrido disponible. El evaluador los recibe junto con la evidencia de la entrega. |

Para tres fichas cortas empezaría con hechos aprobados y consulta por identificador o tema. Un registro mínimo de LR podría ser `P01 · inicio · dos días · hablante LR · fase inicial · a petición · sp_actor §3.2.1`. El intento conserva por separado la fase activa, quién habló, qué se reveló y los últimos turnos necesarios para entender referencias como «¿y después?». Lo que el estudiante supone en su pregunta se guarda como intervención del estudiante. Solo un hecho aprobado o un evento del escenario puede modificar el estado clínico.

**Cuándo introducir RAG.** La generación aumentada por recuperación aporta pasajes documentales al modelo en el momento de responder. Si aumentan los casos o se pierden paráfrasis, se puede probar búsqueda léxica o vectorial sobre fragmentos etiquetados. Primero se limita el conjunto por caso, versión, rol, hablante y fase. Después se buscan los pasajes relevantes y se registra cuál sustentó la respuesta. Para el evaluador, el conjunto recuperable es la rúbrica aplicable y la entrega confirmada. Conviene comprobar tanto si se recuperó el pasaje correcto como si la respuesta lo respetó. RAG aporta evidencia documental. Las decisiones de acceso y tratamiento de datos corresponden a la aplicación y a la institución. [Lewis y colaboradores, trabajo original sobre RAG](https://arxiv.org/abs/2005.11401)

Hay tres resultados distintos ante una pregunta. Si el hecho consta, el paciente lo expresa. Si el guion dice que el personaje lo desconoce, esa incertidumbre es la respuesta correcta. Si el guion guarda silencio, el sistema emplea una salida neutra aprobada y registra el vacío para revisión. Por ejemplo, ante «¿hizo ejercicios de fuerza en las últimas dos semanas?», LR podría decir «No sabría precisarlo». Inventarle una rutina o convertir el silencio en «nunca hago ejercicio» alteraría el caso. Una pregunta ambigua como «¿le duele ahí?» requiere primero identificar el lugar.

### 3. Herramientas y MCP

**Los dos chats pueden funcionar sin MCP en la primera iteración.** MCP es un protocolo para que una aplicación descubra e invoque herramientas o consulte recursos de otros servicios. En la entrevista, el servidor ya conoce el caso y puede leer su ficha directamente. En la entrega, puede preparar el paquete de evidencias antes de llamar al evaluador. Ninguno de esos pasos exige que el modelo elija o ejecute herramientas. Tampoco se necesitan cuatro instalaciones de MCP para representar cuatro roles. [Documentación oficial de la arquitectura MCP](https://modelcontextprotocol.io/docs/learn/architecture)

Un criterio para reconsiderarlo sería que SIRCLE ofreciera un servicio independiente de rúbricas versionadas que también usaran otros módulos. Una primera herramienta podría leer **una rúbrica autorizada para una entrega concreta** con una cuenta de servicio de alcance limitado. En el piloto, la confirmación de una medición, la entrega y la publicación siguen siendo acciones explícitas de la plataforma. Si el estudiante escribe «voy a medirle la presión», queda registrada una intención. El valor del instrumento y el evento de medición proceden del simulador. [Requisitos de SIRCLE, CU06, CU07 y CU15](<../../requisitos/Documento de requerimientos - Paciente virtual en SIRCLE.md>)

### 4. Identidad y acceso por rol

El «rol paciente» configura la voz del modelo. Los roles estudiante, docente y administrador conceden permisos a personas autenticadas. Si SIRCLE ofrece inicio de sesión institucional con OpenID Connect, se puede reutilizar su identidad. OAuth 2.0 permite delegar acceso mediante credenciales de alcance limitado. El servidor debe comprobar en cada petición la operación, el objeto y su relación con esa persona. Un permiso genérico de docente todavía requiere comprobar que el intento pertenece a uno de sus grupos. El mecanismo concreto de integración con SIRCLE está pendiente de confirmar. [OpenID Connect Core](https://openid.net/specs/openid-connect-core-1_0.html) · [IETF, OAuth 2.0](https://www.rfc-editor.org/rfc/rfc6749) · [OWASP, autorización por petición](https://cheatsheetseries.owasp.org/cheatsheets/Authorization_Cheat_Sheet.html)

| Identidad | Alcance que debe aplicar el servidor |
|---|---|
| Estudiante | Entrevista en sus intentos asignados, confirma su entrega y consulta su devolución una vez publicada. Una solicitud sobre las notas de otro estudiante se deniega antes de recuperar datos o consultar a Qwen. |
| Docente | Prepara casos y revisa, rectifica y publica entregas de sus grupos según sus permisos. Los servicios de mantenimiento de la base quedan fuera del chat. |
| Administrador | Gestiona el módulo, permisos y relaciones docente–grupo. Para ver información académica necesita la autorización académica correspondiente. |
| Paciente y evaluador | El servicio del paciente recibe solo la vista del caso activo. El evaluador recibe solo la entrega y la rúbrica que le correspondan. La publicación pertenece al flujo docente. |

Este reparto sigue la matriz de usuarios del proyecto. En una demostración local se pueden usar cuentas ficticias y probar el aislamiento entre dos intentos. Para un piloto con estudiantes identificables, la institución tendría que fijar finalidad, acceso, conservación y eliminación del material académico. En Chile, al **25 de noviembre de 2026**, la referencia vigente es la Ley 19.628. La Ley 21.855, publicada el 23 de noviembre, fijó el **15 de octubre de 2027** como fecha de entrada en vigor de las modificaciones indicadas en el artículo primero transitorio de la Ley 21.719. Que los pacientes sean ficticios no anonimiza las entregas de estudiantes identificables. [Matriz de usuarios de SIRCLE](<../../requisitos/Documento de requerimientos - Paciente virtual en SIRCLE.md>) · [Ley 19.628](https://www.bcn.cl/leychile/navegar?idNorma=141599) · [Ley 21.855 en el Diario Oficial](https://www.diariooficial.interior.gob.cl/publicaciones/2026/11/23/44606/01/2907953.pdf)

### 5. Controles de entrada, contexto y salida

El ejemplo del viaje a Santiago muestra por qué un filtro de palabras sería insuficiente. «¿Qué pasajes me recomienda?» se sale de la consulta. LR podría responder «Me cuesta respirar. ¿Seguimos?». En cambio, preguntar a Dan «¿ha viajado últimamente?» sí forma parte de la entrevista y su guion consigna que no hay antecedente de viaje. El sistema debe entender la intención. Tampoco debería redirigir una pregunta legítima solo porque menciona un diagnóstico que el estudiante está explorando. [Guion de Dan](<../../../datos/medsp1000/mededportal_3171/scenario1/sp_actor/Pulmonary Embolism Case Description.md>)

| Momento | Control defendible en el piloto |
|---|---|
| Antes de recuperar | Fijar caso, rol y fase a partir de la sesión. Tratar el texto del estudiante y los documentos que entrega como material a interpretar. Una orden escrita dentro de ellos carece de autoridad para cambiar permisos o la rúbrica. |
| Al construir el contexto | Entregar solo los campos autorizados. Mantener diagnósticos reservados, pautas docentes, datos de terceros y valores futuros fuera de la vista del paciente. |
| Antes de mostrar el resultado | Comprobar identidades, cifras, negaciones y hechos nuevos contra las fuentes del turno. Verificar que las citas del evaluador existen y respaldan el juicio. Si falla la comprobación, usar una respuesta aprobada, pedir aclaración o dejar el criterio pendiente. |

El filtrado complementa la separación de datos. El servidor puede impedir que se consulten las notas de un compañero aunque el modelo interprete mal una pregunta. En la conversación clínica, una validación demasiado rígida también tiene costo: rechazar una pregunta pertinente deteriora la entrevista. Ambas clases de error deben medirse. [OWASP, riesgos y mitigaciones de inyección de instrucciones](https://genai.owasp.org/llmrisk/llm01-prompt-injection/)

### 6. Caché y respuestas repetidas

«¿Cómo se llama?» y «¿cuántos años tiene?» pueden resolverse directamente desde la ficha aprobada del intento. El nombre es un dato de origen. La caché es una copia temporal que ahorra repetir una operación. Un *embedding* representa similitud y puede ayudar a reconocer dos maneras de preguntar lo mismo. **Para devolver «Tengo 34 años» en LR basta con leer su edad**. Las preguntas sugeridas con un identificador conocido también admiten respuesta directa, como mostró el ensayo de Semana 7. [Ensayo local y rutas de respuesta](<../ensayo-lr/alcance-y-pruebas.md>)

Para el piloto guardaría en caché solo plantillas o fragmentos aprobados y estables. La clave necesita, al menos, caso, versión, rol, hablante, idioma y tema. Si el contenido depende de fase, hace falta incluirla y volver a comprobar que se permite revelarlo. La historia y los datos ya revelados pertenecen a cada intento. Las respuestas personalizadas y las devoluciones académicas quedan fuera de una caché compartida. Una nueva versión de la ficha debe dejar inservibles las entradas antiguas.

Una prueba concreta es alternar «¿cuándo comenzó?» entre LR, Nanu y Dan: dos días, tres días y cuatro horas, respectivamente. Otra es preguntar a LR «¿cómo está ahora?» antes y después de un cambio de fase. Reutilizar la primera respuesta podría ocultar el cambio. También conviene distinguir «¿qué lo empeora?» de «¿qué lo mejora?»: la cercanía entre palabras o vectores puede ocultar que se preguntan cosas opuestas.

### 7. Enrutamiento con un solo modelo

Un *router* decide cómo atender una solicitud. Con un solo Qwen todavía hay una decisión valiosa: qué resolver de forma directa, cuándo redactar y cuándo pedir aclaración. El servidor fija primero el rol según la pantalla, el intento y su estado. El contenido del mensaje puede orientar el tema de una entrevista, pero cambiar de paciente a evaluador requiere una entrega confirmada. [Anthropic, patrones de enrutamiento](https://www.anthropic.com/research/building-effective-agents)

| Situación | Ruta propuesta |
|---|---|
| Dato estable o sugerencia identificada | Consultar el campo autorizado y devolver la frase aprobada. |
| Pregunta libre con evidencia suficiente | Seleccionar los hechos permitidos. Qwen puede identificar el tema o redactar una respuesta breve. Validar el resultado antes de mostrarlo. |
| Referencia ambigua o dato ausente | Pedir precisión o usar la salida de incertidumbre que corresponda. Registrar qué faltó. |
| Entrega confirmada | Abrir el contexto del evaluador, comprobar los eventos verificables y pedir un borrador por criterio. |
| Modelo lento o fuera de servicio | Conservar el turno o la entrega. Permitir reintento sin duplicar lo guardado. Usar una frase de guion únicamente cuando el tema ya esté identificado. |

Un segundo modelo tendría sentido si en el equipo objetivo Qwen cumple en entrevista y falla sistemáticamente al justificar evaluaciones, o si la espera del evaluador interfiere con turnos de pacientes. Primero compararía ambas rutas con los mismos intentos y mediría tiempo de respuesta, cola y concordancia docente. Un segundo modelo también puede repetir el mismo error de evidencia.

### 8. Observabilidad y pruebas de regresión

Para saber por qué LR contestó mal, interesa reconstruir **qué recibió la aplicación, qué hechos eligió y qué llegó a mostrarse**. Conviene enlazar las etapas con identificadores de traza. El historial académico conserva, con acceso restringido, los turnos y eventos necesarios para revisar la entrega. La telemetría técnica registra referencias, versiones, tiempos y errores. Esto permite investigar un turno sin copiar el texto íntegro de cada estudiante a los registros generales. Una explicación generada por el modelo tampoco permite reconstruir con certeza su razonamiento interno. [OpenTelemetry, trazas](https://opentelemetry.io/docs/concepts/signals/traces/) · [Atributos de IA generativa y contenido sensible](https://opentelemetry.io/docs/specs/semconv/registry/attributes/gen-ai/) · [Investigación sobre la fidelidad del razonamiento verbalizado](https://www.anthropic.com/research/reasoning-models-dont-say-think)

| Etapa | Evidencia que conviene registrar |
|---|---|
| Arranque | Versión del servicio, motor, modelo y huella de los pesos. Estado de disponibilidad, duración de carga, reintentos y último error. Si el despliegue usa Docker, distinguir servicio iniciado de modelo listo para responder. |
| Turno | Identificadores seudónimos de intento y turno, perfil, fase, versión del prompt, decisión de acceso, tema interpretado y ruta elegida. |
| Recuperación y respuesta | Identificadores y versiones de hechos, uso de caché o herramienta, duración de inferencia y resultado de la validación. Vincular la salida mostrada al evento académico correspondiente. |
| Entrega y revisión | Versión inmutable de la entrega y la rúbrica, evidencia por criterio, campos pendientes, corrección docente y momento de publicación. |

Por ejemplo, si LR dice «cuatro horas», la traza distingue tres fallos que requieren correcciones distintas. Si se seleccionó un hecho de Dan, hay que revisar el aislamiento. Si se eligió el hecho correcto de LR y el texto lo cambió, hay que corregir la generación o su validación. Si la búsqueda fue correcta y se sirvió una frase antigua, el problema es la caché. Esa explicación es comprobable aun usando respuestas completamente prefijadas. [Anthropic, diseño de evaluaciones y análisis de trayectorias](https://www.anthropic.com/research/demystifying-evals-for-ai-agents)

## Cómo personalizar los tres pacientes

El equipo puede usar una plantilla común y aprobar cada perfil por separado. Conviene fijar para cada uno su identificador, escenario de origen, versión de la adaptación española, fase inicial, hablantes, hechos de presentación espontánea, hechos que precisan pregunta, desconocimientos explícitos y criterios de evaluación. La conversación hereda el identificador del intento. Así, aunque dos estudiantes elijan a LR al mismo tiempo, cada uno conserva su propia entrevista y evolución. [Fichas de entrada de los tres pacientes](<fichas-para-estudiantes.md>)

| Capa | LR · P01 | Nanu · P02 | Dan Sparks · P03 | Evaluador |
|---|---|---|---|---|
| 1. Rol | Habla en frases cortas por la disnea. | Responde despacio. El familiar tiene voz identificada. | Habla con pausas por la falta de aire. | Redacta devoluciones por criterio. |
| 2. Fuentes | Relato de LR en la fase activa. | Hechos de Nanu y del familiar atribuidos por separado. | Relato y cronología que Dan conoce. | Rúbrica del caso y evidencia del intento entregado. |
| 3. Herramientas | El servidor aporta la ficha P01. | El servidor selecciona la vista del interlocutor. | El controlador gestiona los informes. | El servidor prepara la entrega. MCP queda como opción de integración. |
| 4. Acceso | Intento propio con P01 asignado. | Intento propio con P02 asignado. | Intento propio con P03 asignado. | Lectura circunscrita a la entrega. Publicación docente. |
| 5. Controles | Reservar cambios de fase futuros. | Evitar atribuir a Nanu lo dicho por el familiar. | Mantener el diagnóstico fuera de la respuesta del actor. | Verificar toda cita y detectar créditos sin respaldo. |
| 6. Caché | Identidad y edad de P01. | Identidad de P02 y plantilla por hablante. | Identidad y edad de P03. | Reutilizar la definición de rúbrica por versión. Mantener cada devolución en su entrega. |
| 7. Enrutamiento | Pregunta al actor o lectura del instrumento. | Pregunta a Nanu o al familiar. | Pregunta al actor o consulta del informe disponible. | Comprobación por evento, lectura de texto o revisión docente. |
| 8. Prueba distintiva | Expresa que desconoce la historia familiar. | Conserva quién describió la tos con flema. | Mantiene el inicio en cuatro horas al alternar casos. | Distingue intención escrita, ejecución confirmada y registro incompleto. |

### P01 · LR

LR tiene 34 años. El guion sitúa el comienzo de sus síntomas dos días antes, con rinorrea y tos, y describe dificultad para hablar en frases largas. Usa el inhalador frecuentemente y puede contarlo si le preguntan. También puede decir que tiene asma como antecedente conocido. Los antecedentes de su familia biológica sí tienen una respuesta explícita: los desconoce porque fue adoptada. [Guion del actor de LR](<../../../datos/medsp1000/mededportal_10181/scenario1/sp_actor/Case A-Asthma.md>)

| Pregunta o momento | Conducta esperada del agente |
|---|---|
| «¿Desde cuándo le cuesta respirar?» | «Hace dos días». Usar frases cortas acordes con el esfuerzo del ingreso. Si le preguntan cómo empezó, puede añadir el antecedente de resfrío. |
| «¿Su mamá también tenía asma?» | «No lo sé. Soy adoptada». Conservar esa incertidumbre en una repregunta. |
| «¿Cuánto marca su presión?» | Remitir al dato de triaje o al instrumento si está disponible. La cifra pertenece a esa fuente y a su momento de medición. |
| Revisión de la entrega | La pauta puede comprobar si exploró inicio y uso del inhalador. Si la actividad se limita al ingreso, dejar fuera los criterios de la complicación posterior. [Pauta original de LR](<../../../datos/medsp1000/mededportal_10181/scenario1/evaluator/Case A-Asthma.md>) |

**Decisión clínica pendiente para LR.** El original consigna presión de 148/85 y pulso 102 en triaje. La adaptación HTML muestra 128/78 y 112. Para los ejemplos de esta investigación tomé el original como referencia. Antes de usar el caso con estudiantes, el docente debe elegir la versión. La ficha, el instrumento, la caché y la rúbrica deben apuntar a esa misma decisión. [Comparación documentada de ambas versiones](<../../../datos/adaptaciones/lr/procedencia.md>)

### P02 · Nanu

Nanu tiene 61 años, consulta por tos y fiebre de tres días y responde lentamente. El guion asigna parte de la historia al familiar que la acompaña. Nanu puede describir dolor al respirar en el lado derecho del pecho, mientras que el familiar refiere tos con expectoración. La fuente original y el encargo usan **Nanu**. En los diseños de Semana 5 aparece **Nannu** como nombre de presentación. Conviene mantener P02 como identificador común y acordar qué grafía verá el estudiante. [Guion de Nanu y del familiar](<../../../datos/medsp1000/mep_2374-8265.10924-s001/scenario3/sp_actor/C. Pneumonia with Septic Shock.md>) · [Nota de los diseños de Semana 5](<../interfaz/figma.md>)

| Pregunta o momento | Conducta esperada del agente |
|---|---|
| A Nanu, «¿dónde le duele?» | «Aquí, a la derecha del pecho… cuando respiro». Permitir una respuesta breve y pausada. |
| Al familiar, «¿cómo ha sido la tos?» | Identificar la voz como **Familiar**. «Está tosiendo con flema». El historial guarda quién aportó el dato. |
| Cuando cambia su capacidad de responder | El controlador activa la fase prevista. Si la actividad solo ofrece entrevista con Nanu, se marcan como fuera de alcance los criterios que exijan hablar con el familiar. |
| Revisión de la entrega | Comprobar si el estudiante identificó la fuente de la historia y si comunicó el plan al familiar cuando la actividad dio esa oportunidad. Valorar una medición solo cuando conste en los eventos disponibles. [Pauta del escenario de Nanu](<../../../datos/medsp1000/mep_2374-8265.10924-s001/scenario3/evaluator/C. Pneumonia with Septic Shock.md>) |

Para representar al familiar basta un selector explícito del interlocutor dentro de P02 y un conjunto de hechos etiquetados para esa voz. Si la interfaz no deja claro a quién se dirige la pregunta, se puede pedir aclaración. Ello permite probar dos voces con el mismo modelo y conserva en la entrega una distinción relevante para el docente.

### P03 · Dan Sparks

Dan tiene 43 años. La disnea empezó de forma súbita cuatro horas antes de llegar y empeora con el esfuerzo. Refiere dolor leve en la pantorrilla derecha que él interpreta como muscular. El guion indica que la nebulización recibida durante el traslado no lo alivió y que no refiere viajes. Su explicación personal del dolor es una pista válida para conversar. El diagnóstico final está reservado para el escenario docente. [Guion del actor de Dan](<../../../datos/medsp1000/mededportal_3171/scenario1/sp_actor/Pulmonary Embolism Case Description.md>)

| Pregunta o momento | Conducta esperada del agente |
|---|---|
| «¿Qué hacía cuando empezó?» | «Trabajaba en el jardín. Tuve que parar». Mantener la cronología de cuatro horas cuando el estudiante repregunte. |
| «¿Le ha molestado alguna pierna?» | «Un poco la pantorrilla derecha. Pensé que era muscular». Presentarlo como percepción del paciente. |
| «¿Qué mostró la imagen del tórax?» | El paciente no adelanta el diagnóstico. Si el intento admite ese examen, el controlador entrega el informe según su estado. |
| Revisión de la entrega | Valorar si el estudiante caracterizó el inicio y justificó su hipótesis con lo obtenido. Una interpretación de imagen solo puede acreditarse cuando el informe estuvo disponible y la entrega muestra esa interpretación. [Pauta original de Dan](<../../../datos/medsp1000/mededportal_3171/scenario1/evaluator/Pulmonary Embolism Case Description.md>) |

## Qué entrega el evaluador

Al confirmar el envío, el servidor congela el trabajo del estudiante, la cronología y las versiones del caso y de la rúbrica. El evaluador recibe ese conjunto en una conversación nueva. Para una acción comprobable, como registrar una medición, se puede consultar el evento directamente. Para una justificación escrita, Qwen puede proponer una lectura y citar el pasaje. El docente puede corregir el criterio, dejar constancia del motivo y publicar cuando corresponda. Si falla el motor, la entrega permanece guardada y la evaluación queda pendiente. Reintentar debe recuperar la misma entrega y evitar dos evaluaciones vigentes. [Requisitos de entrega, evaluación y rectificación, CU07, CU15 y CU16](<../../requisitos/Documento de requerimientos - Paciente virtual en SIRCLE.md>)

**Ejemplo formativo con LR.** Supongamos que en el turno 3 el estudiante pregunta «¿desde cuándo?» y LR responde «hace dos días». En el turno 5 escribe «voy a medir la presión». En la entrega menciona el inicio, y el registro completo de instrumentos indica que la medición nunca se confirmó. Los identificadores son ilustrativos.

| Criterio de la actividad | Resultado del borrador | Evidencia y devolución al estudiante |
|---|---|---|
| Exploró y documentó el inicio | Observado | Turno 3 y síntesis entregada. «Preguntaste cuándo empezó la falta de aire y recuperaste ese dato en tu síntesis». |
| Ejecutó y registró la medición solicitada | Sin ejecución registrada | Turno 5 y registro íntegro de instrumentos de la sesión. «Dejaste escrita tu intención de medir. Para documentar un valor, completa la medición y confirma el registro en la ficha». |
| Mostró una comunicación adecuada a la dificultad respiratoria | Revisión docente | El docente puede leer si dio tiempo a LR y si formuló preguntas breves. Evitar convertir un solo saludo o la extensión del texto en un juicio global sobre empatía. |

Hay dos ausencias con consecuencias distintas. Si la actividad permitía medir y el registro está completo, se puede informar que la acción no consta. Si se perdió parte del registro por un fallo técnico, el criterio queda pendiente. Si la actividad solo ofrecía chat, el criterio de instrumentación queda fuera del alcance evaluable. Esta distinción protege al estudiante de recibir una mala evaluación por una limitación del prototipo.

## Cómo decidir si una versión está lista para el piloto

El evaluador juzga la **entrega del estudiante**. Las pruebas de regresión o *evals* juzgan **el comportamiento del software**, incluido el propio evaluador. Para empezar, reservaría preguntas inéditas redactadas por el docente y entregas simuladas anotadas independientemente por dos revisores cuando sea posible. Conviene conservar su desacuerdo antes de consensuar una referencia, especialmente en comunicación. El conjunto utilizado para ajustar instrucciones debe quedar separado del conjunto con que se decide aceptar una versión. [Antecedente de las pruebas de LR](<../ensayo-lr/alcance-y-pruebas.md>) · [Método de evaluación de sistemas con agentes](https://www.anthropic.com/research/demystifying-evals-for-ai-agents)

| Prueba propuesta | Qué se compara | Señal para decidir |
|---|---|---|
| Fidelidad por paciente | Preguntas nuevas, paráfrasis y repreguntas frente a hechos del actor | Proporción de afirmaciones respaldadas y de preguntas respondibles contestadas. Registrar aparte negaciones incorrectas, cifras inventadas y rechazos innecesarios. |
| Aislamiento y reserva | Alternar los tres perfiles, dos estudiantes y dos fases. Preguntar por el tiempo de inicio y después por otra persona o la rúbrica. | Cero cruces de caso, revelaciones prematuras o divulgaciones de terceros en la batería reservada. Un fallo basta para detener esa versión e investigar. |
| Incertidumbre y hablante | Antecedentes familiares de LR, ejercicio ausente del guion y dato aportado por el familiar de Nanu | Distingue desconocimiento explícito, información ausente y atribución. Un verificador humano contrasta cada respuesta con el pasaje autorizado. |
| Entrevista natural | Profesor o actor revisa diálogos de varios turnos, en español y con referencias ambiguas | Puntuar por separado pertinencia, coherencia entre turnos y adecuación de la voz. Aceptar paráfrasis fieles aunque no reproduzcan una frase literal. |
| Concordancia del evaluador | Entregas completas, parciales y con evidencia técnica faltante frente a revisión docente por criterio | Contar falsos créditos, omisiones y citas inexistentes. Informar acuerdo por caso y criterio. Para categorías ordinales se puede añadir kappa ponderado junto con recuentos y desacuerdos concretos. |
| Operación y cambios | Primera consulta en frío, turnos con modelo cargado, dos sesiones simultáneas, interrupción antes de recibir respuesta y reintento de la entrega | Medir mediana y percentil 95 cuando haya muestras suficientes. Verificar que se retoma el intento correcto y que existe una sola entrega vigente. |

Como punto de partida propondría **diez diálogos inéditos por paciente**, incluyendo al menos una repregunta y un caso ausente o ambiguo por diálogo, y **seis entregas contrastadas por caso** para el evaluador. Son tamaños iniciales para descubrir errores, no estimaciones de precisión poblacional. Repetir una parte con la misma configuración permite observar variabilidad. Antes de comparar versiones, fijar con el docente qué errores clínicos y de atribución bloquean el piloto y qué espera resulta aceptable en su computador. Anotar modelo, cuantización, versión de ficha, prompt y rúbrica junto con cada resultado.

## Orden de trabajo recomendado

1. **Cerrar el guion de LR con el docente.** Acordar el objetivo del curso, la fase que se practicará, los datos de triaje válidos y qué evidencias puede producir una entrevista de texto. Redactar las preguntas de aceptación antes de ajustar el prompt.
2. **Probar un recorrido completo con LR.** El estudiante entrevista, justifica su hipótesis y entrega. El evaluador produce un borrador con evidencias y el docente lo revisa. Usar Qwen3.5-4B como candidato inicial, con consulta directa de hechos, permisos por intento y trazas desde el arranque.
3. **Incorporar Nanu y Dan con los mismos controles.** En Nanu, probar el cambio de hablante. En Dan, probar que el texto y las fuentes visibles conservan la incertidumbre diagnóstica. Alternarlos con LR para detectar contaminación entre perfiles.
4. **Añadir complejidad con una pregunta medible.** Probar recuperación vectorial si falla la selección de hechos en preguntas nuevas, MCP si aparece un servicio externo que deba compartirse, y un segundo modelo si mejora un criterio concreto del evaluador. Repetir la misma batería después de cada cambio.

La primera demostración puede cerrarse con un ejemplo que el profesor pueda auditar de principio a fin: el estudiante pregunta a LR cuándo empezó a faltarle el aire, ella responde desde su guion, y la devolución muestra el turno y la frase de la entrega que justifican el criterio. Ese recorrido comprueba la personalización del paciente, la procedencia del dato y la utilidad del evaluador en una sola actividad.
