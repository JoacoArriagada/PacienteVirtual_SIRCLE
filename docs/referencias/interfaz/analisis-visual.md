# Análisis del rediseño de SIRCLE · Versión 2

9 de noviembre de 2026. Propuesta de baja fidelidad para revisión con el autor y Andrés Santos. El objetivo de esta versión es orientar la atención del estudiante y reducir la información simultánea, conservando las funciones del recorrido anterior.

**Conteo de palabras:** 7293. El método se describe en Verificación de entrega (antecedente histórico: `verificacion/Verificacion_entrega.md`).

## 1. Qué cambia y qué problema se intenta resolver

La primera propuesta hizo visibles las funciones, incorporó pacientes reconocibles y permitió discutir la distribución de las pantallas. La observación del autor precisa un problema diferente: esa misma información funcional ocupaba demasiado espacio dentro de la interfaz del estudiante. Una tarjeta mostraba identidad, motivo, estado, etapa, disponibilidad de comentarios, familia y acción. Cada dato podía justificarse por separado, pero la suma obligaba a leer más de lo necesario antes de decidir qué caso abrir. La versión dos revisa esa prioridad, no solamente el tamaño de las letras.

El criterio central consiste en distinguir información para actuar ahora de información para consultar después. Nombre, edad, estado breve y acción principal ayudan a elegir un paciente. El identificador del intento, el detalle del guardado, la etapa y la situación de la devolución responden a preguntas posteriores. En la atención, las herramientas y la conversación son parte del trabajo; un cronómetro permanente no necesariamente lo es. La propuesta conserva el acceso a esos datos mediante controles claros y evita que su existencia obligue a mantenerlos siempre desplegados.

Ocultar información tampoco resuelve cualquier problema. Una opción necesaria para continuar no debería quedar enterrada en varias ventanas. Por eso se mantienen visibles las acciones frecuentes y se limita el detalle adicional a un nivel reconocible: un panel de información, un apartado desplegable o una vista de procedimiento. Si el estudiante necesita conocer una consecuencia antes de actuar, esa consecuencia aparece junto a la decisión. El cierre de edición al entregar y el cambio temporal al avanzar son ejemplos de información que permanece explícita.

El pedido actual también cambia la profundidad de la demostración. La presión arterial deja de ser un clic que produce un número inmediatamente y pasa a una secuencia de preparación, espera, resultado e histórico. Esa secuencia permite evaluar si la interfaz comunica lo que está pasando, diferencia lo pendiente de lo obtenido y preserva al paciente como referencia. Los veinte segundos de espera representan una decisión demostrativa de interacción. No se presentan como duración clínica universal ni como una validación del procedimiento.

Se mantuvo el lenguaje visual sobrio de la propuesta anterior: fondos neutros, líneas sencillas, ilustraciones vectoriales y un acento principal. Los colores de familia se limitan a iconos y superficies pequeñas. Los estados usan color junto con texto. La jerarquía se construye principalmente con tamaño, posición y agrupación, de modo que las decisiones sigan siendo comprensibles en una impresión monocroma o cuando una persona no distinga los colores utilizados.

La entrega se creó en una carpeta nueva, `Propuesta_visual_SIRCLE_v2`. La versión anterior permanece en su ubicación y se incluye una copia completa de su paquete en Antecedentes (antecedente histórico: `antecedentes/Propuesta_visual_SIRCLE_v1.zip`). Las imágenes proporcionadas para los cambios también se conservaron sin modificaciones. Las nuevas ilustraciones, diagramas, controles y textos son una reformulación del recorrido; no se usaron capturas como sustituto de una interfaz editable o de un boceto con texto legible.

## 2. Fuentes consultadas y límites de la interpretación

La fuente principal es la explicación del autor en el texto adjunto de esta conversación. Allí se pide simplificar tarjetas, desplazar información de sesión a un cuadro aparte, enriquecer los exámenes generales, incorporar búsqueda de adicionales, desarrollar la interacción de presión y aplicar el criterio a todas las pantallas. La explicación también solicita trabajar sobre una copia y conservar la entrega anterior. Estas instrucciones prevalecen sobre las decisiones de distribución y cantidad de información documentadas en la primera versión.

Se inspeccionaron individualmente las ocho imágenes de la carpeta «Propuestas de cambio». El índice de referentes (antecedente histórico: `recursos/referentes_cambio/indice.json`) registra sus nombres, rutas de copia y huellas SHA-256. Una captura permite observar composición, texto, estados dibujados y jerarquías. No permite concluir que un botón ya tenga cierto comportamiento, que una validación esté implementada o que un flujo haya sido probado con estudiantes. Las interacciones de esta versión se definen como propuestas y se distinguen de lo directamente visible.

| Referencia del autor | Resolución | Aporte principal |
| --- | --- | --- |
| Cambio Cuadro informativo paciente virtual.png | 1536 × 1024 | Tres estados breves, nombre dominante, edad secundaria, motivo asociado al icono. |
| vista caso 1.png | 1672 × 941 | Herramientas agrupadas, paciente central y cuaderno a la derecha. |
| vista caso 2.png | 1672 × 941 | Panel de información de sesión desplegado. |
| presion arterial/1.png | 1448 × 1086 | Monitor inicial, ausencia de resultado y acción principal. |
| presion arterial/2.png | 1448 × 1086 | Preparación mediante pasos antes de comenzar. |
| presion arterial/3.png | 1448 × 1086 | Medición en curso, espera y representación del instrumento. |
| presion arterial/4.png | 1448 × 1086 | Resultado completado y diferencia entre consultar y registrar. |
| presion arterial/5.png | 1448 × 1086 | Histórico con gráfica, tabla y series identificadas. |

Los requerimientos y la trazabilidad de la primera propuesta siguen siendo el marco funcional. Los dieciséis wireframes no se confundieron con los dieciséis casos de uso oficiales: el número de una lámina identifica una representación, mientras un caso de uso puede estar presente en varias pantallas. Seleccionar o reanudar un paciente continúa relacionado con CU04. El acceso y la autorización acompañan el recorrido sin exigir una nueva pantalla de inicio de sesión dentro del módulo.

La investigación adicional se concentró en patrones que ayudan a revisar este cambio: divulgación progresiva, controles desplegables, diálogos y comunicación mediante color. Se consultaron fuentes de NN/g y W3C, vinculadas en la sección de accesibilidad. No se hizo una búsqueda clínica para convertir los ejemplos de presión en recomendaciones. Tampoco se incorporaron escalas de aprobación, diagnósticos o reglas de evaluación que no estén acordados en la actividad. Los valores numéricos conservan una identificación explícita como datos de demostración.

## 3. Análisis de las ocho imágenes nuevas

### 3.1. Tarjeta y tres estados

La referencia muestra tres tarjetas con la misma paciente, Elena, y permite comparar Sin iniciar, En pausa y Finalizado sin que cambie la identidad. El nombre ocupa mayor tamaño que la edad. El botón cambia de acuerdo con la continuidad del caso y el motivo se consulta desde un icono. Esa organización reduce líneas permanentes respecto de la versión anterior. El estado sigue acompañado de texto, de modo que el punto de color aporta reconocimiento rápido sin ser la única fuente de significado.

La proporción mencionada como cuatro a tres se interpretó para la imagen del paciente. Esa lectura es coherente con el dibujo: el área de retrato es horizontal y la tarjeta completa necesita espacio debajo para identidad, estado y acción. Aplicar cuatro a tres al contenedor total obligaría a comprimir demasiado el retrato o a apretar los controles. En esta versión el HTML fija la imagen en 4:3 y permite que la altura total resulte del contenido. Esta interpretación queda visible para que el autor pueda revisarla.

Se adapta la estructura, pero se evita repetir exactamente sus adornos y tamaños. La familia musculoesquelética utiliza un icono sobre una superficie rojiza suave; gastrointestinal, una superficie naranja suave. La etiqueta emergente describe motivo y familia. Ninguna de esas señales convierte el motivo en diagnóstico confirmado. Una persona puede consultar «Dolor lumbar» sin que la tarjeta afirme una enfermedad causal todavía desconocida. La edad permanece legible aunque tenga menor contraste y tamaño que el nombre.

### 3.2. Vista de atención con grupos de herramientas

La primera vista del caso reúne seis controles de signos vitales, tres métodos y dos herramientas de visualización. Su aporte principal es la agrupación. Una colección más grande de iconos puede seguir siendo localizable si responde a categorías que el estudiante reconoce. La propuesta anterior mostraba pocos instrumentos y dejaba demasiado implícita la variedad de acciones. La nueva distribución mantiene tres zonas y hace explícita la diferencia entre medir un signo, aplicar un método de examen y modificar cómo se observa al paciente.

En el rediseño aparecen presión, pulso, temperatura, respiración, saturación y peso. Inspección, palpación y auscultación forman otro grupo. Para la vista del paciente se proponen Ver brazo y Ampliar, porque esas funciones pueden representarse de manera concreta en esta demostración. No se promete un cambio tridimensional de postura que el material disponible no implementa. La ficha clínica y la evolución mantienen accesos cercanos, pero separados de los instrumentos para evitar que todos los controles parezcan acciones del mismo tipo.

El chat conserva su campo de escritura y su envío dentro del panel derecho. El historial tiene desplazamiento propio; el paciente permanece en la zona central. Las proporciones se ajustan al contenido en lugar de fijar tres porcentajes rígidos. El panel izquierdo necesita que quepan etiquetas, y el derecho necesita una conversación legible. La ilustración central conserva presencia suficiente para identificar a la persona sin obligar a encoger los controles hasta volverlos ambiguos.

### 3.3. Información de sesión desplegada

La segunda vista repite la composición de atención y añade un cuadro con estado, intento, etapa, día simulado y duración. Esa comparación deja claro que los datos no desaparecen: cambian de nivel de acceso. El control de información permanece visible en una ubicación estable del encabezado. El panel se abre sobre la interfaz, presenta las parejas de etiqueta y valor y ofrece cierre explícito. Al cerrarse, el espacio vuelve a estar disponible para la conversación y las herramientas.

La adaptación usa un panel secundario con estado expandido anunciado y cierre mediante Escape. La información de sesión no contiene acciones clínicas ni se convierte en un menú general del producto. Su alcance es responder dónde está el estudiante y qué intento está usando. El tiempo real se presenta con minutos y segundos; el tiempo del caso se presenta como día y desplazamiento simulado. La distinción evita que veinte segundos de una animación de medición se interpreten automáticamente como veinte minutos de evolución clínica.

El panel no debería interrumpir una respuesta ni borrar un borrador. En la demo su apertura es una consulta y no cambia el contenido de chat o notas. En ventanas pequeñas se limita su anchura al espacio disponible. Se mantiene una única entrada reconocible en lugar de distribuir pequeños indicadores por todas las esquinas. Esta decisión responde a la observación del autor sobre saturación, aunque su comprensión deberá comprobarse con estudiantes que no hayan recibido una explicación previa del prototipo.

### 3.4. Presión arterial en estado inicial

La primera imagen de presión presenta un monitor sin resultado y un botón destacado para medir. El autor corrige expresamente la posición del paciente: solicita que esté a la izquierda y el dispositivo a la derecha. La nueva lámina sigue esa instrucción textual incluso cuando la captura inicial sitúa los elementos en otro orden. El objetivo es mantener una relación visual simple entre la persona, el instrumento y la acción, con menos elementos compitiendo que en la mesa de atención completa.

El monitor muestra SYS, DIA y Pulso, con unidades. Los ceros iniciales se mantienen como marcador visual porque el autor los pidió, pero se acompañan de Sin medición y No explorado. El sistema no los registra como presión o pulso cero. Esta distinción es una regla de estado de la interfaz: no existe un resultado hasta completar la medición. El histórico empieza vacío y la ficha también permanece sin una entrada de esa medición.

Medir presión es el control principal, situado debajo del monitor. Información adicional tiene menor peso y abre una explicación breve. Volver al paciente permite recuperar la conversación del mismo intento. El histórico es una consulta separada, disponible incluso para mostrar que todavía no hay registros. No se añaden valores anteriores ficticios para llenar una gráfica vacía; los ejemplos con varios puntos se identifican como tales en las láminas estáticas.

### 3.5. Preparación de la medición

La segunda imagen añade una preparación con brazalete, ajuste y comienzo. La propuesta conserva la idea de pasos previos, pero transforma dos de ellos en comprobaciones visibles que habilitan la acción final. El estudiante puede marcar la colocación y confirmar el ajuste dentro de la simulación. Iniciar medición permanece deshabilitado hasta completar ambas comprobaciones. Ese estado anticipa por qué todavía no se puede continuar y evita que un simple clic produzca valores sin representar el procedimiento.

La ilustración del brazo ayuda a reconocer a qué se refiere el paso. Se reutiliza la identidad del paciente y se añade un brazalete vectorial coherente con la baja fidelidad. No se pretende enseñar una técnica validada mediante la posición exacta de ese dibujo. Los términos de preparación y la disponibilidad del procedimiento deberán revisarse con el responsable clínico antes de usar el diseño en una actividad real. La demo permite evaluar la secuencia de controles y su comprensión, no la corrección clínica de la técnica.

Cancelar y cerrar tienen el mismo efecto sobre los datos: no producen una medición ni añaden una fila al histórico. Al volver a preparar, las casillas se presentan de nuevo sin asumir que una comprobación anterior sigue vigente. El foco regresa al elemento que abrió la ventana cuando se cancela. La preparación ocupa una ventana puntual porque concentra una decisión previa, mientras que la medición y sus resultados permanecen en la vista de procedimiento.

### 3.6. Medición en curso

La tercera imagen representa espera, un contador y una señal gráfica. El rediseño conserva esos elementos, pero les asigna funciones distintas. El contador informa cuánto falta. La barra indica progreso. La señal ilustra actividad sin presentarse como una curva fisiológica real ni como un dato interpretable. Los campos numéricos permanecen sin resultado durante la espera. Esto evita sugerir que valores parciales ya estén disponibles o que el estudiante pueda copiarlos antes de terminar.

La demo utiliza una espera de veinte segundos y un brazalete visible en el brazo. El inicio repetido se bloquea mientras la medición está activa. Cancelar permanece disponible y descarta únicamente el proceso incompleto; las mediciones anteriores se conservan. Si la persona intenta volver a la atención durante ese intervalo, se explica que saldrá cancelando la medición actual y se solicita confirmar esa consecuencia. La confirmación tiene una razón concreta y no aparece en cada consulta secundaria.

La animación no es necesaria para comprender el estado. Con movimiento reducido se conserva el texto, la barra y el contador. Al completar, la interfaz anuncia el resultado y habilita la acción siguiente. No se utilizan mensajes que sugieran que hubo una conexión con un dispositivo físico. La simulación sirve para discutir espera, control y continuidad dentro de un caso, manteniendo visible el límite entre un prototipo de interacción y una medición clínica real.

### 3.7. Medición completada y registro

La cuarta imagen separa el resultado de la acción de registrarlo. Esa diferencia es central para SIRCLE: obtener un hallazgo, redactar una nota, guardar un borrador y entregar un intento no son la misma operación. En la propuesta, una medición completa entra en el histórico del intento y aparece como evidencia obtenida. Registrar en ficha añade una referencia documental a esa medición. El botón cambia a Registrado en ficha y se deshabilita para evitar que el mismo resultado se copie varias veces.

El resultado muestra SYS y DIA en mmHg y pulso en latidos por minuto. También identifica el momento simulado y el número de medición. El prototipo no usa una fecha real de atención clínica ni inventa una certificación del dato. Los valores de ejemplo, como 118/76 y 78 lpm, son ficticios y se utilizan para evaluar la presentación. La interfaz no los califica como normales o anormales ni deriva de ellos una recomendación.

Medir de nuevo conserva los registros anteriores y vuelve a la preparación. Una nueva medición tiene una identidad distinta. Si el estudiante cancela esa repetición, el resultado previo sigue disponible. Esa continuidad evita que las acciones de prueba destruyan la evidencia que ya se había obtenido. La separación también permite revisar posteriormente cuáles mediciones están en el histórico y cuáles fueron registradas explícitamente en la ficha, sin depender de inferencias a partir del número de clics.

### 3.8. Histórico, gráfico y tabla

La quinta imagen aporta una vista temporal y una alternativa de tabla. La adaptación conserva ambas, pero corrige una ambigüedad de unidades: el pulso no debe compartir un eje rotulado únicamente como presión en mmHg. La propuesta muestra una gráfica para SYS y DIA y otra para pulso en lpm. Las dos utilizan la misma secuencia de mediciones y sus momentos simulados. Las series de presión se distinguen por nombre y trazo, además del color.

En la demo el histórico comienza vacío y solo incorpora resultados completados. Las mediciones canceladas no generan puntos. La gráfica etiqueta las mediciones en orden y debajo muestra su momento del caso; repetir sin avanzar conserva ese momento. Por tanto, la distancia horizontal representa la secuencia de registros, no una duración física continua. Esa decisión se declara para evitar que una línea entre puntos se interprete como observación continua del paciente.

La tabla muestra número de medición, momento, SYS, DIA, pulso y presencia en ficha. Proporciona valores exactos y una alternativa a la lectura visual de las curvas. En una ventana estrecha, la tabla puede desplazarse dentro de su contenedor sin ensanchar toda la página. En las láminas estáticas se incluye un ejemplo de tres registros para discutir composición; en el HTML el usuario produce sus propios registros demostrativos al completar el procedimiento.

## 4. Información principal, secundaria y contextual

La simplificación se aplica mediante una decisión por dato, no mediante una reducción indiscriminada de texto. Información principal es la necesaria para elegir la acción de ese momento. Información secundaria responde a dudas de contexto que no bloquean la mayoría de las tareas. Información contextual aparece solo cuando cambia una decisión. Por ejemplo, el identificador E-01 no necesita repetirse en la tarjeta, el encabezado y el pie; sí debe poder consultarse cuando el estudiante verifica que reanudó el intento correcto.

En la tarjeta, la identidad ocupa el primer nivel. La edad acompaña al nombre con menor peso. El estado breve permite distinguir empezar, continuar o revisar. El motivo se asocia a su icono y se muestra al consultar. En cambio, al entrar en la ficha inicial, ese motivo pasa a ser principal porque ya contextualiza la atención. La información cambia de prioridad según la tarea y no por una regla fija que obligue a ocultar siempre determinados campos.

En la atención, los instrumentos y la conversación permanecen abiertos porque el estudiante alterna entre ellos. Notas, evidencia e hipótesis comparten el panel derecho porque no necesitan cuatro editores completos simultáneos. Cambiar de pestaña conserva el contenido. La información de sesión se consulta desde el encabezado y no sustituye el acceso a la ficha clínica. Esa separación evita un icono de información que termine acumulando funciones heterogéneas y se vuelva difícil de predecir.

En el procedimiento de presión, unidades y estado son principales. El tiempo restante también lo es mientras existe una espera activa. Una vez completado, ese contador desaparece y el momento del resultado toma su lugar. El mismo principio se aplica a la entrega: antes de confirmar importa revisar el trabajo y conocer que se cerrará su edición; después importa saber que fue recibido y si hay una devolución disponible. Mostrar todas esas etapas a la vez produciría justamente la saturación que se busca corregir.

La divulgación progresiva propone mantener visibles las opciones habituales y ofrecer las especializadas cuando se solicitan. Su utilidad depende de escoger bien la separación y de hacer reconocible el acceso al detalle. En esta propuesta ese criterio se aplica a sesión, filtros y evaluación; no justifica esconder el botón de enviar ni la consecuencia de entregar. Es una aplicación de diseño que debe comprobarse con tareas, no un resultado de usabilidad ya medido. [NN/g: Progressive Disclosure](https://www.nngroup.com/articles/progressive-disclosure/).

## 5. Estado de tarjeta y continuidad del intento

Sin iniciar indica que todavía no se ha comenzado una práctica de ese caso. Iniciar caso abre primero la presentación del paciente. Esa entrada permite conocer la información inicial y la tarea antes de pasar a la mesa de atención. No se presentan antecedentes reservados ni un diagnóstico resuelto. El objetivo es que el estudiante reconozca a quién atenderá y qué debe producir, sin convertir la tarjeta del catálogo en una ficha clínica completa.

En pausa corresponde a un intento iniciado que puede continuarse. En la demostración, Pausar devuelve al catálogo y conserva los datos en la memoria de la página. Al reanudar se recuperan conversación, notas, hallazgos, solicitudes y momento simulado. Esta persistencia durante la sesión permite probar continuidad, pero no implica que exista guardado en un servidor. El producto integrado necesitará confirmaciones de guardado y recuperación que la maqueta solo representa conceptualmente.

Finalizado significa que el intento está cerrado para edición. No significa por sí solo que tenga calificación, que haya sido aprobado o que la devolución ya esté publicada. Esas diferencias permanecen en el resumen y en el detalle del estado. Clara representa un caso finalizado con devolución pendiente; Rosa representa uno con devolución publicada. Ambas tarjetas pueden conservar un estado breve común porque la acción inicial es consultar el resumen, donde la diferencia pasa a ser relevante.

El color acompaña esta clasificación y no codifica desempeño. Un punto verde en Finalizado no equivale a aprobación clínica. Un punto rojizo en Sin iniciar no expresa un error ni una urgencia del paciente. El texto evita esa confusión y permite interpretar el estado sin recordar una clave cromática. Del mismo modo, el color de familia pertenece al motivo del caso y utiliza un icono diferente del punto de continuidad. Son dos conceptos separados, con posiciones y formas distintas.

La ficha conserva proporciones comunes para que la comparación entre pacientes sea sencilla. Los rostros, nombres y botones ocupan lugares estables. En escritorio se muestran cuatro tarjetas cuando el ancho permite mantenerlas legibles; en ventanas menores pasan a dos o una columna. La proporción 4:3 permanece en el retrato, mientras los controles mantienen un tamaño utilizable. No se fuerza una cuadrícula de cuatro columnas cuando produciría textos cortados o botones demasiado estrechos.

## 6. Organización de las acciones y los exámenes

El panel izquierdo distingue las acciones que obtienen información del paciente de los controles que modifican su representación. Los signos vitales y los métodos de examen tienen grupos propios. Ver brazo y Ampliar no se presentan como mediciones: ayudan a observar la figura. Esta separación permite que un estudiante encuentre una acción por su propósito y no solo por recordar la forma de un icono. Las etiquetas cortas permanecen visibles; el nombre completo de la acción está disponible para teclado y tecnologías de asistencia.

La presión dispone del procedimiento desarrollado porque fue el ejemplo que el autor describió con mayor precisión. Las otras acciones muestran resultados ilustrativos acotados y los incorporan a Evidencia. No se construyeron nueve simuladores clínicos distintos ni se inventaron audios de auscultación. Cuando no hay contenido clínico configurado, se expresa ese límite en el resultado de demostración. Antes de integrar el módulo, cada acción necesitará una definición de disponibilidad, respuesta, unidad, momento y relación con los objetivos de la actividad.

Los exámenes adicionales utilizan búsqueda por nombre o categoría y una lista de resultados compactos. Cada resultado mantiene icono, nombre, familia y acción de solicitud. La selección no entrega un resultado inmediato: primero se confirma qué se solicita para el paciente activo y después aparece Pendiente. La solicitud queda asociada al intento y no se duplica si se vuelve a consultar la misma entrada. Esta secuencia permite revisar la diferencia entre elegir una prueba y recibir su información.

En la demo, una solicitud se resuelve tras avanzar treinta minutos simulados. El intervalo es una regla local para mostrar el concepto, no una promesa sobre tiempos de laboratorio o imagenología. Cuando se resuelve, se informa que hay un documento de demostración sin valores clínicos configurados. Esa limitación evita llenar un resultado con datos arbitrarios que podrían parecer parte de un caso validado. El lugar de la información y el cambio de estado sí quedan disponibles para discutir la interfaz.

La evolución mantiene un acceso propio. Allí el momento del caso es relevante porque el estudiante está decidiendo avanzar. La confirmación muestra origen, destino y consecuencia del salto. Cancelar conserva el estado anterior. En la mesa de atención, en cambio, esos tiempos no necesitan ocupar constantemente el encabezado. La propuesta usa el contexto de la tarea para decidir cuándo un dato temporal debe aparecer, en lugar de clasificar todos los tiempos como irrelevantes o todos como permanentes.

## 7. Notas, evidencia, hipótesis y registro

La petición sobre notas se traduce en una lista de puntos independientes. Añadir crea un apunte nuevo; editar modifica uno existente; quitar retira el seleccionado y ofrece Deshacer. Esas acciones resultan más claras que un único bloque de texto para quien quiere construir una lista durante la entrevista. El contenido puede seguir siendo libre: el formato por puntos no obliga a usar frases prefabricadas ni convierte los apuntes en respuestas de una rúbrica.

Al editar, la lista mantiene la identidad del punto y el formulario cambia su acción a Guardar cambio. Cancelar edición conserva el texto que ya estaba guardado. Al quitar, la recuperación permite corregir una eliminación accidental durante la sesión. Los botones de edición y eliminación llevan nombres accesibles que incluyen el apunte al que pertenecen. Esta relación es importante cuando existen varios puntos: una serie de iconos de papelera sin contexto sería ambigua para quien utiliza un lector de pantalla.

La conversación se conserva como una fuente distinta. Un apunte del estudiante puede resumir o interpretar lo escuchado, pero no se trata automáticamente como una expresión literal del paciente. La evidencia identifica hallazgos obtenidos y su origen. En la demo se diferencia el motivo referido en el primer mensaje de los resultados producidos por acciones. Una herramienta no utilizada permanece sin explorar, y una solicitud todavía no resuelta permanece pendiente. La ausencia de un dato nunca se convierte por defecto en un hallazgo negativo.

La hipótesis utiliza dos campos principales: interpretación y fundamento con límites. Junto al editor se presenta una evidencia vinculada. El estudiante puede cambiar esa referencia mediante una selección de hallazgos obtenidos. El diseño busca que fundamentar sea una acción visible y no una recomendación escondida en un texto introductorio. Guardar hipótesis conserva ese razonamiento, pero no modifica los hallazgos ni entrega el caso. Revisar entrega permanece como un acceso posterior y de menor peso mientras se está escribiendo.

La ficha clínica conserva una responsabilidad documental. En el procedimiento de presión, registrar añade una referencia al resultado completado. En el apartado de acciones, guardar borrador conserva una indicación todavía no ejecutada. Revisar acción abre sus parámetros y consecuencia antes de confirmar. La interfaz evita usar un único botón Guardar para todas estas operaciones porque produciría una expectativa ambigua: el estudiante no sabría si solo almacenó texto, si aplicó una acción al paciente o si cerró la actividad académica.

El prototipo preserva estos datos dentro de la memoria de la página. La continuidad es comprobable al cambiar de pestaña, volver al catálogo y reanudar un paciente. Recargar reinicia los ejemplos. Esta regla se comunica fuera de la interfaz clínica, en la nota de la demo. No se introduce en cada control una explicación técnica del almacenamiento; el estudiante necesita saber que su acción se realizó y cuál es su consecuencia, mientras el revisor necesita conocer el alcance de la demostración.

## 8. Reformulación del conjunto de dieciséis pantallas

La revisión cubre todas las láminas principales y conserva la organización general de la primera propuesta. El cambio no consiste en aplicar un encabezado nuevo a pantallas intactas. Cada vista reduce información simultánea de acuerdo con una tarea: elegir, continuar, conversar, escribir, medir, revisar o gestionar. Los cuatro estados ampliados de presión pertenecen a la lámina 07 y no sustituyen ninguna de las otras funciones. Sus identificadores continúan la secuencia de esa lámina en el catálogo.

| Lámina | Tarea | Reformulación principal |
| --- | --- | --- |
| 01 | Elegir un paciente | Tarjeta esencial, retrato 4:3 y motivo bajo demanda. |
| 02 | Retomar o revisar | Un intento en pausa destacado y lista breve de finalizados. |
| 03 | Continuar una atención | Punto de retorno breve, una acción principal y detalles del intento. |
| 04 | Conocer al paciente | Identidad y motivo al entrar; objetivos y datos iniciales desplegables. |
| 05 | Atender y conversar | Herramientas agrupadas, paciente central y sesión bajo demanda. |
| 06 | Escribir notas | Puntos editables, eliminación recuperable y borrador conservado. |
| 07 | Medir presión | Paciente izquierdo, monitor derecho y cinco estados del procedimiento. |
| 08 | Registrar una acción | Un editor, borrador explícito y revisión previa a ejecutar. |
| 09 | Revisar evolución | Cronología breve y avance temporal contextual. |
| 10 | Fundamentar hipótesis | Evidencia cercana y dos campos de razonamiento. |
| 11 | Revisar entrega | Apartados resumidos y consecuencia clara al confirmar. |
| 12 | Consultar resumen | Comentarios y evidencia antes que métricas de evaluación. |
| 13 | Asignar actividad | Caso, destinatarios y condiciones en pasos editables. |
| 14 | Supervisar intentos | Tabla operativa y revisión individual en una vista separada. |
| 15 | Administrar módulo | Destinos por responsabilidad y acciones contextuales de versión. |
| 16 | Preparar caso | Secciones y etapas plegables con revisión pendiente consultable. |

Las primeras cuatro vistas reducen decisiones repetidas. La selección ya permite diferenciar empezar, reanudar y consultar. La presentación inicial aporta el motivo y la tarea cuando el estudiante entra al caso. Reanudar no obliga a leer un inventario completo de todo lo recuperado; ofrece un punto de retorno y un acceso a sus detalles. El historial conserva las prácticas finalizadas sin convertir la portada en un panel de calificaciones, tiempos y porcentajes.

Las pantallas de trabajo clínico concentran los elementos que cambian durante la atención. La conversación tiene su espacio; los apuntes se manipulan como puntos; las acciones muestran estados; la cronología explica el momento del cambio. Las hipótesis mantienen una referencia de evidencia. No se añaden pantallas por cada dato obtenido. El procedimiento de presión se abre porque necesita concentración y pasos propios, y conserva una salida clara hacia la atención que estaba en curso.

La revisión de entrega utiliza apartados que se pueden abrir antes de confirmar. Sus cantidades describen contenido, no calidad: tres intervenciones de chat no significan una entrevista correcta. Una hipótesis guardada tampoco implica que esté bien fundamentada. La consecuencia de entregar se mantiene visible porque sí afecta una decisión. La propuesta no bloquea automáticamente la entrega por toda posible omisión clínica; ese criterio deberá definirse según la actividad y su rúbrica, para no impedir evaluar justamente lo que el estudiante omitió.

La devolución prioriza un comentario útil y una mejora concreta. Las áreas evaluadas permiten cambiar de categoría sin desplegar un informe completo. Identificación del intento, duración, rúbrica y calificación se consultan en el detalle cuando están disponibles. Pendiente de revisión permanece explícito. El prototipo no presenta un porcentaje de aprobación ficticio para completar un espacio. La encuesta final se conserva como actividad opcional, separada del desempeño clínico y de la decisión de repetir una práctica.

La asignación docente pasa a un recorrido por etapas. El docente elige un caso, define destinatarios y revisa condiciones. Los datos ya seleccionados se pueden consultar desde el contexto sin repetirlos como bloques completos en cada paso. La habilitación requiere revisar el resumen final. Esta secuencia intenta reducir errores por omisión sin hacer que todas las decisiones académicas compitan simultáneamente. No sustituye la definición pendiente de reglas de repetición, disponibilidad o publicación.

La supervisión separa grupo e intento. La tabla del grupo muestra estudiante, estado y acción inmediata; las métricas y el resumen gráfico quedan disponibles cuando se necesitan. Abrir un intento conduce a la pieza de revisión individual, con evidencia a un lado y comentario al otro. Guardar revisión, publicar devolución y rectificar siguen siendo operaciones diferentes. La diferencia visual evita que una corrección todavía en borrador parezca publicada al estudiante antes de que el docente lo decida.

La administración y la preparación de contenido también se benefician de la reducción de simultaneidad. El catálogo administra versiones y ofrece acciones contextuales. Los accesos y grupos tienen un destino separado. El preparador trabaja por sección y abre una etapa clínica a la vez. Las versiones usadas por intentos anteriores deben conservarse. El estado Borrador no se confunde con aprobación clínica, y solicitar revisión no equivale a habilitar automáticamente la actividad para estudiantes.

## 9. Trazabilidad funcional y controles explicativos

Los números de lámina siguen siendo identificadores de presentación. El catálogo utiliza códigos como 01-1 y 07-14 para localizar una funcionalidad en un globo y su descripción. Los casos de uso oficiales aparecen en los encabezados de cada lámina y se relacionan con el recorrido. Esta separación impide interpretar la nueva distribución como una renumeración de los requerimientos. La versión dos tiene su propio catálogo, porque reducir, agrupar o ampliar controles cambia la correspondencia de algunos identificadores respecto de la versión anterior.

| Caso de uso | Representación en la versión 2 |
| --- | --- |
| CU01 · Acceder desde SIRCLE | Contexto de acceso compartido; no se dibuja otro inicio de sesión. |
| CU02 · Administrar permisos y grupos | Destino administrativo de la lámina 15. |
| CU03 · Preparar y asignar | Asignación 13, administración de versiones 15 y preparación 16. |
| CU04 · Iniciar y reanudar | Selección, continuidad y presentación en 01–04. |
| CU05 · Valorar al paciente | Conversación y exploración en 05 y 07. |
| CU06 · Registrar y seguir | Notas, acciones, evolución e hipótesis en 06 y 08–10. |
| CU07 · Finalizar y consultar devolución | Entrega y resumen en 11 y 12. |
| CU08 · Supervisar | Grupo en 14 y revisión individual en B08. |
| CU09 · Encuestas | Accesos opcionales en 04 y 12. |
| CU10 · Verificar autorización | Responsabilidad transversal; límites de administración y preparación. |
| CU11 · Conservar avances | Continuidad y estados de guardado a lo largo del intento. |
| CU12 · Repetir práctica | Intento nuevo autorizado desde continuidad o resumen. |
| CU13 · Solicitar exámenes | Grupos, búsqueda y solicitudes pendientes en la atención. |
| CU14 · Avanzar tiempo | Evolución y confirmación del cambio en 09. |
| CU15 · Evaluar | Entrega, revisión y publicación de resultados. |
| CU16 · Rectificar | Revisión individual docente con acción diferenciada. |

Las flechas quedan fuera del área simulada de la interfaz. Su función es explicar el diseño al revisor, no aumentar la carga de lectura del estudiante durante una sesión. El HTML ofrece un recorrido interactivo sin esos globos y un apartado de dieciséis wireframes anotados. Esta distinción permite evaluar por separado la interfaz de uso y la documentación de sus funciones. La leyenda mantiene los mismos nombres y códigos que los dibujos, generados desde una fuente de datos común.

Cada descripción del catálogo tiene un máximo de treinta palabras. Ese límite obliga a explicar la acción y su consecuencia inmediata con precisión. Las condiciones más amplias, dependencias de roles y decisiones pendientes se documentan aquí, donde existe espacio para desarrollarlas. No se sobrecarga cada globo con una miniatura del requerimiento completo. Los estados ampliados de presión incluyen sus propias llamadas para que preparación, cancelación, registro e histórico no queden descritos únicamente en un párrafo distante.

## 10. Accesibilidad aplicada y jerarquía visual

La prioridad visual se expresa mediante contraste de tamaño y posición. El nombre de paciente domina la identidad y la edad acompaña en un nivel secundario. El botón principal es mayor o tiene el acento más fuerte de su área. Las consultas complementarias utilizan controles de menor peso, manteniendo un área de activación utilizable. Reducir el protagonismo de un dato no significa hacerlo ilegible. La edad, las unidades y los estados siguen teniendo contraste suficiente para leerse en la pantalla prevista.

Los colores de estado y familia se acompañan de palabras o símbolos diferenciados. Esta decisión conserva el reconocimiento rápido que pide el autor y permite comprender la tarjeta sin distinguir rojo, ámbar y verde. En las gráficas, SYS y DIA también tienen trazos diferentes. La alternativa textual de tabla permite consultar cada valor con su unidad. El uso redundante de señales responde al criterio de no transmitir significado exclusivamente mediante color. [W3C: Understanding Use of Color](https://www.w3.org/WAI/WCAG22/Understanding/use-of-color.html).

Los controles que revelan información comunican si están abiertos o cerrados y se pueden activar con teclado. El panel de sesión tiene un nombre claro, cierre visible y retorno al control que lo abrió. Los apartados nativos de detalle conservan un resumen reconocible. La finalidad es que una persona descubra el contenido secundario sin depender de pasar el cursor por una zona invisible. Este comportamiento sigue el patrón de control y contenido expandible descrito por W3C. [W3C: Disclosure Pattern](https://www.w3.org/WAI/ARIA/apg/patterns/disclosure/).

Los diálogos de preparación y confirmación concentran una decisión puntual. Al abrirse, el foco entra en la ventana; el fondo deja de ser interactivo; cerrar devuelve el foco a una ubicación coherente. Escape permite cancelar una consulta o preparación. La confirmación de salida durante una medición explica la pérdida concreta antes de actuar. Estas decisiones se apoyan en el patrón de diálogo modal de W3C; la comprobación del prototipo no reemplaza una auditoría completa con lectores de pantalla. [W3C: Dialog Modal Pattern](https://www.w3.org/WAI/ARIA/apg/patterns/dialog-modal/).

Las pestañas usan controles reales y permiten navegación mediante flechas, Inicio y Fin. La selección permanece anunciada y solo un panel del cuaderno se muestra a la vez. En móvil, Paciente, Acciones y Cuaderno permiten acceder a las tres zonas sin encogerlas hasta perder legibilidad. Al obtener un hallazgo, la demo abre Evidencia y hace visible el cuaderno cuando el ancho requiere paneles. La identidad permanece en el encabezado para conservar contexto aunque el retrato grande esté en otra pestaña.

Se respeta la preferencia de movimiento reducido. El contador y los estados textuales permanecen aunque se detenga la animación decorativa de señal. Los mensajes de estado informan del resultado de una acción sin insertar una explicación extensa en cada pantalla. En las notas, las acciones de editar y quitar tienen nombres específicos. En el histórico, la gráfica incluye un nombre accesible y se acompaña de una tabla. Estas medidas son verificaciones parciales; no se afirma conformidad integral WCAG a partir de unas pocas pruebas automáticas.

## 11. Qué permite probar el HTML

La galería abre en una vista de prototipo para que el revisor pueda experimentar primero la reducción de información. El catálogo permite buscar pacientes, filtrar familia, consultar motivo y estado, iniciar un caso, reanudar y abrir un resumen. La ficha inicial muestra el motivo dentro del contexto de atención. La separación de vistas evita que la explicación metodológica ocupe el lugar de los controles que el estudiante necesita. Los fundamentos y los referentes están en un apartado propio de la galería.

La atención permite abrir información de sesión, usar el cuaderno y recorrer preguntas guiadas. El chat no contiene un agente de IA ni produce respuestas libres. Si se escribe una pregunta fuera del guion, la demo informa del límite y conserva el borrador. Las notas permiten añadir, editar, quitar y deshacer. La hipótesis puede vincular una evidencia disponible. Estas funciones se mantienen durante la sesión para comprobar que cambiar de pestaña o reanudar no requiere reconstruir el trabajo manualmente.

La medición de presión implementa los estados descritos en las imágenes. Las comprobaciones de preparación habilitan el comienzo; la espera dura veinte segundos; cancelar no produce un registro; completar añade una medición con identidad propia. El histórico representa las mediciones del paciente activo y la ficha evita duplicar una entrada. Los resultados están predefinidos y señalados como ilustrativos. Esta implementación permite probar transiciones y consecuencias sin introducir una dependencia de dispositivos, servidores o modelos de lenguaje.

La búsqueda de exámenes adicionales muestra resultados por nombre o categoría y confirma solicitudes. La evolución resuelve esas solicitudes de demostración según su regla temporal. La entrega local cierra el intento y muestra una devolución pendiente. Las vistas de administración, asignación y preparación se entregan como wireframes para revisar, no como operaciones sobre SIRCLE. Esta diferencia se registra para que un botón dibujado en una lámina no se interprete como una integración ya implementada.

El HTML usa recursos locales y no necesita conexión a Internet para funcionar. Se incluyen imágenes, estilos, código de interacción y documentos en el mismo paquete. Los archivos SVG permiten ampliar las láminas sin perder nitidez; los PNG facilitan compartirlas en otros documentos. El PDF reúne las dieciséis pantallas principales. Los estados ampliados de presión permanecen disponibles como imágenes individuales y dentro de la galería, para revisar la secuencia con mayor detalle que en una sola página.

## 12. Validación, decisiones pendientes y siguiente revisión

La revisión automática comprueba que existan las dieciséis láminas, que los códigos sean únicos, que sus llamadas estén presentes en el recurso correspondiente y que las descripciones respeten el límite de palabras. También revisa la carga de imágenes y la ausencia de dependencias de red. Estas comprobaciones reducen errores de entrega, pero no garantizan por sí solas que una flecha señale con claridad un control o que una composición sea fácil de interpretar. Por eso se complementan con inspección de capturas y exportaciones.

Las pruebas de interacción cubren búsqueda, estados, preguntas guiadas, notas, preparación de presión, espera, cancelación, registro e histórico. También comprueban la conservación de contenido al reanudar y la resolución de solicitudes al avanzar. Las ventanas de 320, 375, 414, 768, 1280 y 1440 píxeles permiten revisar el cambio de disposición. Los resultados concretos y las limitaciones se registran en el documento de verificación. No se reportan mejoras porcentuales de tiempo, aprendizaje o satisfacción porque no se han medido con usuarios.

La primera prueba con estudiantes debería pedir que elijan un caso concreto sin explicar previamente los iconos. Se observaría si reconocen nombre, edad, estado y acción, y si encuentran el motivo cuando lo necesitan. Después se pediría reanudar un intento y comprobar su identidad. Si el panel de información resulta difícil de descubrir o el estado breve genera dudas sobre la evaluación, habría que ajustar etiqueta y posición antes de añadir nuevamente más líneas a todas las tarjetas.

Una segunda tarea debería incluir conversación, nota y obtención de un hallazgo. Interesa observar si el estudiante entiende dónde quedó su borrador al cambiar de pestaña, si diferencia un apunte de una evidencia y si logra vincular esa evidencia con una hipótesis. La prueba no debería premiar la cantidad de iconos utilizados. El objetivo es revisar si la interfaz apoya el razonamiento previsto y permite recuperar información sin obligar a memorizar la estructura completa de la aplicación.

La secuencia de presión requiere revisión clínica antes de utilizarse como práctica formativa. Deben acordarse disponibilidad de la acción, pasos de preparación, representación del instrumento, respuesta del paciente, duración de la simulación y contenido del resultado. La prueba de interfaz puede preguntar qué espera el estudiante al cancelar, qué diferencia encuentra entre histórico y ficha y si interpreta los ceros iniciales como ausencia de medición. Esas observaciones permitirán decidir si el marcador 000 sigue siendo conveniente o necesita otro tratamiento visual.

La revisión docente debe definir qué información necesita permanecer visible para localizar intentos y qué puede abrirse después. La tabla compacta propuesta no elimina la calificación ni la última actividad del modelo: las desplaza al detalle del intento o al resumen solicitado. Será necesario comprobar si ese acceso adicional resulta adecuado para las tareas habituales del docente. La separación entre guardar, publicar y rectificar debe revisarse con ejemplos de borradores y correcciones, manteniendo historial y responsable cuando exista integración real.

Siguen pendientes las rúbricas, los porcentajes de aprobación, las reglas de repetición, las condiciones de publicación y la distribución definitiva de responsabilidades de administración y preparación. También deberá resolverse la persistencia real de intentos y su relación con la ficha de SIRCLE. La propuesta no llena esos vacíos con métricas inventadas. Reserva lugares para la información acordada y documenta qué conducta se está sugiriendo, de modo que la revisión académica pueda modificarla sin perder las funciones ya identificadas.

El material permite discutir una pregunta concreta: si el estudiante puede reconocer la siguiente acción sin perder acceso al contexto que necesita. Las dieciséis láminas, los estados de presión y la demo ofrecen distintas formas de revisar esa pregunta. El catálogo explica las funciones con brevedad; este análisis conserva los motivos, los límites y las decisiones pendientes. La versión anterior queda disponible para comparar qué información se retiró de la vista inicial y dónde puede encontrarse ahora.
