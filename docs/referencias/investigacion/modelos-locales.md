# Modelos de inteligencia artificial en CPU para el Paciente Virtual de SIRCLE

**Informe de investigación aplicada · Práctica de Bioinformática · Universidad de La Frontera**  
**Fecha de corte documental:** 17 de noviembre de 2026.  
**Escenario:** estación de laboratorio con un procesador de la clase AMD Ryzen 5 serie 5000, gráficos integrados y 16 GB de memoria DDR4. Inferencia de texto íntegramente en CPU.

## Resumen ejecutivo

**El primer candidato recomendado es Qwen3-4B-Instruct-2507, en el archivo de 2,50 GB cuantizado como Q4_K_M, ejecutado mediante llama.cpp.** Se propone cargar una sola copia y asignarle dos tareas por separado. Durante la entrevista representa al paciente ficticio y responde exclusivamente con los hechos que puede conocer en ese momento. Al entregar un intento, una solicitud nueva le proporciona la rúbrica y el registro de acciones para que redacte una evaluación formativa respaldada por evidencias. El modelo es multilingüe y su variante 2507 produce respuestas directas. Esas propiedades justifican el orden de prueba. Su fidelidad al guion, su utilidad docente en español y su velocidad en el laboratorio siguen pendientes de medición. [01–03, 19, 21]

La comparación más informativa tiene tres pasos. Primero, enfrentar Qwen3-4B-Instruct-2507 a Gemma 3 4B IT con idénticos casos y una restricción parecida de memoria. Sus archivos pesan casi lo mismo y pertenecen a familias diferentes. Segundo, contrastar ambos con MedGemma 1.5 4B en la revisión de una entrega ya cerrada. Google advierte que esta versión médica carece de optimización y evaluación específicas para conversaciones de varios turnos. Tercero, reservar Qwen3-8B para los intentos de evaluación en que el modelo menor se equivoque de manera repetible. Qwen2.5-3B-Instruct y Qwen3.5-4B permiten investigar, respectivamente, una opción más compacta y una arquitectura posterior dentro del mismo fabricante. [04–16]

El tamaño de los archivos respalda ensayar estos modelos en 16 GB. La memoria instalada también debe alojar el sistema operativo, la aplicación, el contexto de inferencia y los búferes del motor. El borrador de requerimientos, además, propone que el percentil 95 del diálogo quede dentro de 15 segundos bajo una carga de 20 usuarios concurrentes. Una respuesta fluida en una demostración individual y la aceptación de ese requisito requieren ensayos diferentes. Hay una medición pública de otro modelo cuantizado sobre un Ryzen 5 5600G, útil como referencia de orden de magnitud. Se hizo con 32 GB y carece de una evaluación docente. Ninguna velocidad de este informe se presenta como resultado del computador de la Universidad. [22, 28, 41, P1]

**Decisión educativa.** El docente conserva el control sobre hechos, evolución, criterios y publicación de resultados. El valor añadido del modelo consiste en comprender preguntas formuladas de distintas maneras, contestarlas con naturalidad y relacionar evidencia escrita con un descriptor de rúbrica. El puntaje calculable, la disponibilidad de exámenes y el tiempo del caso se pueden verificar directamente en los registros. Así se puede aceptar un modelo para interpretar al paciente aunque todavía necesite revisión exhaustiva como evaluador, o seleccionar modelos distintos para ambos roles. La decisión debe explicitar qué tareas aprobaron, con qué versión y ante qué ejemplos fallaron. [29–35, P1–P2]

### Guía de lectura

| Decisión que debe tomar el equipo | Sección |
|---|---|
| Qué descargar y cuánto ocupa | 3 y 4 |
| Qué evidencia existe sobre un Ryzen comparable | 2 y 5 |
| Cómo ejecutar exclusivamente en CPU y repetir el ensayo | 5 |
| Cómo incorporar el paciente y el evaluador en SIRCLE | 6 y 7 |
| Cómo interpretar la literatura educativa y preparar casos | 8 y 9 |
| Cómo decidir con mediciones y resultados docentes | 10 a 12 |
| Dónde verificar modelos, archivos y evidencia | Bibliografía al final |

## 1. Problema de investigación y método de selección

El Paciente Virtual amplía una plataforma en la que estudiantes y docentes ya utilizan una ficha clínica. En la práctica interesa unir tres actividades que suelen evaluarse por separado. El estudiante conversa para recoger antecedentes, documenta lo que considera relevante y toma decisiones cuyo resultado aparece después en la evolución del caso. Al cierre, el docente necesita reconstruir por qué el estudiante llegó a una conclusión y qué información tenía disponible. Una aplicación puede generar una conversación convincente y, sin embargo, dejar pocas huellas para esa revisión. Por eso la selección considera tanto la calidad del diálogo como la capacidad de relacionar una afirmación con el evento que la sustenta. [P1–P2]

La pregunta técnica es acotada. ¿Qué modelos de pesos descargables permiten construir ese circuito con inferencia local de texto en una estación con 16 GB de DDR4 y sin delegar cálculo a una GPU? La pregunta pedagógica añade una condición importante. ¿En qué tareas resulta admisible confiar en la salida generada y qué evidencia tendría que observar un docente antes de aceptarla? En este entorno, una respuesta veloz pierde utilidad si anticipa resultados de laboratorio, cambia la biografía del paciente o acredita un examen que el estudiante solo pensó solicitar. A la inversa, una evaluación lenta puede seguir siendo útil al término de la clase si respeta el registro y reduce la carga de revisión.

Se realizó una revisión documental orientada a una decisión de ingeniería. La búsqueda cubrió fichas de fabricantes, archivos publicados en Hugging Face, documentación de llama.cpp y Ollama, documentación del procesador, estudios sobre evaluación con modelos de lenguaje y publicaciones sobre simulación con pacientes estandarizados. Se distinguió entre fabricante del modelo, responsable de la conversión GGUF y autor de la medición de rendimiento. Un mismo repositorio puede hospedar varias cuantizaciones, un proyector visual y pesos de precisión alta. Para evitar atribuir al candidato el tamaño de todo el repositorio, se verificó la entrada del archivo específico en su árbol de distribución. Los números entre corchetes remiten a las fuentes consultadas. [01–30, 37–42]

La inclusión exigió una tarea identificable en SIRCLE, una variante instruida utilizable en conversación o evaluación y un archivo identificable para un motor con ejecución en CPU. Se prefirieron candidatos que permitieran contrastes interpretables. Dos modelos de tamaño parecido y entrenamiento diferente ayudan a investigar el valor de la especialización. Dos tamaños de una familia permiten explorar qué se obtiene a cambio de una espera mayor, aunque cambien también la versión y el ajuste instructivo. Un modelo comunitario usado para una medición sobre un Ryzen semejante aparece como evidencia de hardware y queda fuera de la lista clínica priorizada. [01–16, 41]

La revisión es selectiva. Los seis modelos representan opciones justificables para este encargo y la fecha indicada. La literatura de referencia incluye pruebas realizadas con otros idiomas, pesos y equipos, así como estudios de educación sanitaria que no utilizaban ninguno de los seis modelos. Se informa la naturaleza de esa evidencia al aplicarla. Un ensayo educativo puede orientar el diseño de las mediciones sin demostrar que un archivo determinado mejora el aprendizaje. Una ficha oficial puede confirmar la arquitectura y las modalidades sin acreditar la idoneidad de la entrevista en Temuco. Un repositorio comunitario puede verificar la disponibilidad de una conversión sin transformar sus instrucciones en aval del fabricante.

En las secciones siguientes, **dato publicado** designa una característica documentada del modelo, un tamaño listado o un resultado atribuido a sus autores. **Estimación** designa una operación explícita sobre supuestos identificados. **Propuesta de piloto** designa una decisión del presente informe, como empezar con 4.096 tokens o reservar una entrega para revisión. Esta distinción permite actualizar la recomendación si cambia la máquina o si un candidato inicialmente secundario obtiene mejores resultados con docentes. No se entrenó un modelo, no se instalaron pesos en el laboratorio y no se recogieron respuestas de estudiantes para este informe.

## 2. Qué puede hacer el computador descrito

### 2.1 Identidad del procesador y memoria realmente disponible

La expresión «Ryzen 5 de quinta generación» se interpreta aquí como una referencia aproximada a la serie Ryzen 5000. AMD identifica, por ejemplo, el Ryzen 5 5600G de escritorio como un procesador Zen 3 de seis núcleos y doce hilos, con gráficos Radeon integrados, dos canales de memoria y soporte DDR4 de hasta 3.200 MT/s. Es una referencia pertinente porque combina CPU y gráficos en el mismo equipo. El procesador concreto de la práctica podría tener otro límite de potencia, otra cantidad de caché o encontrarse en un portátil. Para el informe experimental se debe copiar el identificador que informa el sistema operativo y anotar el tipo de equipo. [28]

Dos instalaciones con 16 GB pueden comportarse de forma distinta. La cantidad y disposición de los módulos, la velocidad negociada y la memoria que el sistema reserva para gráficos cambian el margen. También importa si la interfaz, el motor de base de datos y el navegador comparten esa estación. Conviene registrar la memoria disponible tras iniciar la sesión habitual y repetir la lectura con el servicio de inferencia encendido. Si un proceso ya estaba abierto antes del ensayo, cerrarlo solo para obtener una cifra más favorable describe otra configuración. Un informe defendible conserva tanto el ensayo aislado como una segunda medición con los componentes que realmente se utilizarán en clase. [19–22, 28]

Un archivo de 2,50 GB representa aproximadamente 2,33 GiB, al dividir sus bytes decimales por 1.073.741.824. El de 5,03 GB representa alrededor de 4,68 GiB. Estas conversiones explican diferencias entre lo que muestra el sitio de descarga y algunas utilidades del sistema. Al cargar un modelo se suman los búferes de trabajo, las estructuras de seguimiento del contexto y la memoria del programa. El sistema operativo puede mapear el archivo y cargar páginas a medida que se usan. Por eso sumar dos lecturas de memoria sin conocer su definición puede contar dos veces páginas compartidas. El usuario final, en cambio, percibirá enseguida los efectos de la paginación si cada turno tarda más que el anterior. [02, 12, 19–22, 24]

### 2.2 Lectura del contexto y generación de la respuesta

El motor atraviesa dos fases medibles. Primero procesa el texto de entrada, que incluye instrucciones, parte del caso y conversación previa. Luego genera la respuesta token a token. Un token es una unidad del vocabulario interno y puede representar una palabra, una parte de ella o un signo. Dos modelos pueden producir cantidades diferentes de tokens para la misma frase en español. Por eso interesa comparar segundos para una tarea completa además de tokens por segundo. El microensayo de llama.cpp informa por separado procesamiento de entrada y generación. Esta separación permite distinguir una ficha de contexto excesiva de un paciente que se extiende demasiado al contestar. [21–22]

Un ejemplo aritmético aclara los tamaños en juego. Si un sistema hipotético procesara 50 tokens de entrada por segundo y generara 10 por segundo, leer 500 tokens nuevos tomaría unos diez segundos. Entregar 40 tokens adicionales tomaría otros cuatro. Antes de considerar cola, red interna o interfaz, el intercambio rondaría catorce segundos. Si la misma pregunta arrastrara 2.000 tokens nuevos, el tiempo de lectura subiría a cuarenta segundos. Son supuestos ilustrativos. La reutilización de un prefijo ya procesado, la arquitectura del modelo y el tamaño de lote pueden cambiar sustancialmente el resultado. La prueba debe distinguir una primera consulta de otra que aprovecha contexto reutilizable. [21–22]

### 2.3 Un antecedente experimental comparable

El mantenedor de una conversión de Nanbeige4.1-3B publicó un ensayo con Ryzen 5 5600G, Windows 11 Pro, seis hilos y 32 GB DDR4-3200. Su tabla de CPU para Q4_K_M informa **67,80 ± 1,14 tokens/s de procesamiento de entrada** y **14,35 ± 0,52 tokens/s de generación**, con cargas de 512 y 128 tokens respectivamente, tres repeticiones y una compilación de llama.cpp identificada. El equipo también tenía una tarjeta dedicada. El autor diferencia el ensayo de CPU de su ensayo CUDA, por lo que se tomó exclusivamente la tabla del primero. Es un reporte comunitario, con un modelo diferente y el doble de memoria instalada. [41]

Este resultado permite defender una afirmación limitada pero útil para planificar la compra o reutilización de equipos. Hay un ejemplo documentado de generación interactiva en CPU sobre un procesador de la clase descrita. El dato deja abiertas la latencia de primer token con una ficha larga, la tasa obtenida con Qwen y la capacidad de atender un curso. También faltan observaciones sobre adecuación al español clínico y exactitud del evaluador. El ensayo tiene más valor como patrón para documentar el laboratorio que como pronóstico. Repetir sus dos longitudes ofrece un punto de comparación y añadir un guion realista hace que la medición sea pertinente para SIRCLE. [22, 41]

## 3. Archivos seleccionados y condiciones de distribución

Todos los tamaños de la tabla son decimales, redondeados por el repositorio y correspondientes a un único GGUF de pesos. Los enlaces abren los directorios en los que figura el nombre exacto. IT e Instruct identifican variantes ajustadas para seguir instrucciones. Un modelo base de la misma familia puede tener dimensiones parecidas y comportarse de modo distinto ante la plantilla de conversación. La selección apunta al uso textual. Un proyector de imágenes constituiría otro archivo y requeriría una prueba separada.

| Modelo y autor original | Distribuidor del GGUF y nombre exacto | Tamaño | Condiciones de los pesos | Primera tarea propuesta |
|---|---|---:|---|---|
| Qwen3-4B-Instruct-2507 · Qwen, Alibaba | [Unsloth](https://huggingface.co/unsloth/Qwen3-4B-Instruct-2507-GGUF/tree/main) · `Qwen3-4B-Instruct-2507-Q4_K_M.gguf` | 2,50 GB | Apache 2.0 | Paciente y evaluación por turnos |
| Gemma 3 4B IT · Google | [ggml-org](https://huggingface.co/ggml-org/gemma-3-4b-it-GGUF/tree/main) · `gemma-3-4b-it-Q4_K_M.gguf` | 2,49 GB | Términos de Gemma | Comparación independiente de ambos roles |
| MedGemma 1.5 4B IT · Google | [Unsloth](https://huggingface.co/unsloth/medgemma-1.5-4b-it-GGUF/tree/main) · `medgemma-1.5-4b-it-Q4_K_M.gguf` | 2,49 GB | Health AI Developer Foundations | Revisión de evidencia al cierre |
| Qwen2.5-3B-Instruct · Qwen, Alibaba | [Qwen](https://huggingface.co/Qwen/Qwen2.5-3B-Instruct-GGUF/tree/main) · `qwen2.5-3b-instruct-q4_k_m.gguf` | 2,10 GB | Qwen Research License | Entrevista breve con menos memoria |
| Qwen3.5-4B · Qwen, Alibaba | [ggml-org](https://huggingface.co/ggml-org/Qwen3.5-4B-GGUF/tree/main) · `Qwen3.5-4B-Q4_K_M.gguf` | 3,03 GB | Apache 2.0 | Alternativa para casos extensos o fallos del 2507 |
| Qwen3-8B · Qwen, Alibaba | [Qwen](https://huggingface.co/Qwen/Qwen3-8B-GGUF/tree/main) · `Qwen3-8B-Q4_K_M.gguf` | 5,03 GB | Apache 2.0 | Segunda lectura de evaluaciones pendientes |

### 3.1 Por qué importa distinguir al fabricante del distribuidor

La ficha del fabricante permite comprobar el propósito del ajuste, la arquitectura, las modalidades y los límites declarados. El árbol de archivos permite comprobar qué bytes se pueden descargar para el ensayo. La combinación es particularmente importante en MedGemma. La ficha original explica qué entrenó y evaluó Google, mientras que el GGUF seleccionado procede de Unsloth. En Gemma 3 y Qwen3.5 la conversión seleccionada la distribuye ggml-org. En Qwen2.5-3B y Qwen3-8B los archivos elegidos aparecen en la propia organización Qwen. Conservar esta cadena de procedencia permite atribuir correctamente un fallo de plantilla, conversión o ejecución, sin imputarlo automáticamente al conocimiento del modelo original. [01–18, 37]

Hay una diferencia práctica entre disponer de pesos y poder redistribuirlos dentro de otra aplicación. Si la práctica comparte solo el código de integración, el docente puede descargar el modelo siguiendo las condiciones del publicador. Si en cambio se distribuye un equipo preconfigurado, una imagen de disco o un paquete que incluye los pesos, la distribución debe documentar las condiciones correspondientes. El texto del acuerdo de Qwen2.5-3B define su alcance no comercial en términos de investigación o evaluación. Los modelos Gemma 3 y MedGemma remiten a instrumentos distintos. Para reproducir una decisión institucional conviene archivar junto al manifiesto del experimento el enlace a los términos de la versión seleccionada y la fecha de consulta. [04, 08, 14, 17–18, 36]

### 3.2 Cuantización y margen de contexto

GGUF es un formato de distribución de tensores y metadatos que utilizan motores como llama.cpp. Q4_K_M identifica una estrategia de cuantización que reduce el tamaño de los pesos, con tratamiento de precisión que puede variar entre tensores. No corresponde multiplicar el número nominal de parámetros por cuatro bits y tomar el producto como tamaño íntegro del archivo. Existen metadatos, tensores con tratamiento diferente y particularidades de cada conversión. Por la misma razón, el rótulo «4B» no determina una cifra de memoria. La tabla muestra una diferencia de más de medio gigabyte entre dos archivos de la familia Qwen descritos como 4B. [01–02, 06–07, 24, 37]

La cuantización permite escoger otro punto de comparación sin cambiar de modelo. Para Qwen3-4B-Instruct-2507 el mismo repositorio lista una versión Q5_K_M de 2,89 GB y otra Q8_0 de 4,28 GB. Si el candidato falla de manera persistente en negaciones o condiciones temporales, se puede repetir una muestra con Q5_K_M manteniendo plantilla y caso. Un resultado diferente orientaría la investigación hacia la sensibilidad a la representación de pesos, aunque una sola respuesta acertada sería insuficiente. Si ambos archivos producen el mismo error al recibir los mismos hechos, conviene investigar el diseño del caso o la instrucción antes de asignar más memoria. [02, 24]

Para el primer candidato se puede calcular un componente de memoria con mayor precisión. Su configuración especifica 36 capas, ocho cabezas de clave y valor y una dimensión de 128 por cabeza. Si el motor mantiene claves y valores densos de 16 bits para toda la secuencia, una aproximación es multiplicar dos tensores por 36 capas, ocho cabezas, 128 componentes y dos bytes por componente. Se obtienen 147.456 bytes por token. Para 4.096 posiciones el total teórico es **576 MiB**, y para 8.192 posiciones es **1.152 MiB**. El cálculo excluye pesos, alineación, búferes y otros consumos. Cambiar el tipo de caché o la arquitectura cambia la estimación. [01, 21, 43]

La suma aproximada de los pesos Q4 de ese Qwen y la caché teórica a 4.096 posiciones ronda 2,89 GiB. Es una referencia parcial para una instancia y una secuencia. Falta observar cuánta memoria queda disponible con el modelo caliente, el servidor, la base de datos y el navegador, y qué sucede al incrementar contexto o concurrencia. Para Qwen3.5, que combina bloques con distintas formas de atención y estado, aplicar la misma fórmula produciría una falsa precisión. Los registros del motor al reservar memoria y las mediciones del sistema constituyen mejores referencias para ese candidato. [02, 06–07, 19–22, 43]

## 4. Investigación de los seis modelos

### 4.1 Qwen3-4B-Instruct-2507 como primera instalación

La ficha distingue esta edición de Qwen3-4B original y de la variante especializada en razonamiento. Declara cuatro mil millones de parámetros, 36 capas y una ventana nativa de 262.144 tokens. La característica determinante para esta práctica es más sencilla: la edición Instruct-2507 responde directamente, sin activar un modo separado de razonamiento visible. En una entrevista de preguntas breves facilita reservar la mayor parte de la salida a lo que diría el personaje. La ventana anunciada describe una capacidad arquitectónica. Configurar esa longitud en un Ryzen con 16 GB introduciría un costo considerable y aportaría poco a un personaje que solo debe acceder al estado vigente. [01–03, 43]

La ventaja para el piloto es poder formular dos problemas con el mismo archivo. En el primero se evalúa si entiende una pregunta del estudiante y formula una respuesta que suene a paciente. En el segundo recibe un intercambio cerrado y debe señalar si existe respaldo textual para un criterio. Esto mantiene constantes modelo, memoria y motor mientras se evalúa cuánto depende la calidad de la tarea. Una estación pequeña puede alternar las instrucciones de ambos roles en solicitudes independientes y eliminar el costo de cambiar los pesos en cada interacción. La evidencia del desempeño de un rol se conserva por separado de la del otro.

La prueba decisiva para este candidato incluye preguntas ambiguas y negativas. Si el guion dice que el personaje desconoce el nombre de un examen, una respuesta adecuada puede reconocer que no lo recuerda. Inventar un nombre más preciso sería un deterioro aunque la frase resulte profesional. Como evaluador, el mismo modelo debe distinguir el caso en que el estudiante preguntó por el examen del caso en que realmente lo solicitó. En ambos ejemplos se puede determinar si acertó consultando un guion breve y eventos identificados. Ninguna tabla general de habilidades multilingües resuelve por adelantado estas dos preguntas. [01, 29–32, P1]

### 4.2 Gemma 3 4B IT como comparación generalista

Google documenta para Gemma 3 capacidades multilingües amplias y una versión instruida de cuatro mil millones de parámetros que admite texto e imágenes como entrada. Para una entrevista escrita interesa exclusivamente la ruta textual. El archivo de 2,49 GB en ggml-org permite mantener casi constante el presupuesto de pesos frente a Qwen3-4B-Instruct-2507. Si aparecen diferencias en los resultados, el tamaño en disco será una explicación poco satisfactoria por sí sola. También intervienen el entrenamiento, la tokenización del español, la plantilla y la forma en que cada modelo resuelve una instrucción que entra en tensión con un diálogo previo. [08–10, 18]

Un ensayo especialmente informativo consiste en preparar dos guiones con la misma estructura narrativa, pero con antecedentes distintos. En uno, el personaje recuerda una fecha exacta. En el otro solo recuerda si el hecho ocurrió antes o después de una visita previa. El estudiante usa la misma pregunta insistente en ambas entrevistas. El candidato obtiene crédito por ajustar el grado de precisión al conocimiento del personaje. Si Gemma preserva mejor ese límite y sus respuestas siguen siendo oportunas, podría ocupar el rol de paciente aunque Qwen obtenga mejor resultado al redactar devoluciones formativas. La comparación debe conservar ambos resultados, incluso cuando el equipo prefiera mantener un solo archivo en operación.

La familia Gemma también permite aislar parcialmente el valor de la especialización de MedGemma. Las dos opciones de Google proceden de una línea arquitectónica relacionada y los GGUF elegidos tienen el mismo tamaño redondeado. Con una entrega idéntica se puede investigar si la adaptación médica ayuda a interpretar terminología o induce al modelo a suministrar conocimiento que el estudiante nunca demostró. Se compararán en especial las situaciones donde el nombre técnico correcto aparece en una nota copiada, frente a aquellas donde el alumno expresa un razonamiento equivalente con palabras corrientes. Así se examina una diferencia que tendrá efecto directo en la justicia de la retroalimentación. [08–10, 13–16]

### 4.3 MedGemma 1.5 4B IT como lector especializado

MedGemma 1.5 es un modelo multimodal de Google desarrollado para tareas con información médica. Su informe técnico describe trabajos con imágenes, documentos clínicos y datos de distintas modalidades. Para SIRCLE interesa una capacidad más acotada: leer un pequeño conjunto de datos de una ficha ficticia y producir una explicación sustentada en esos datos. El laboratorio puede explorar esa hipótesis sin añadir imágenes. Google señala que el modelo requiere validación para cada aplicación y declara expresamente la falta de evaluación y optimización en interacciones de varios turnos. Esta advertencia afecta la propuesta de paciente conversacional más que una revisión puntual de evidencia. [13–17]

El valor de un especialista se aprecia mejor en contrastes difíciles. Supóngase una entrega en la que el estudiante documentó el momento de la reevaluación, pero dejó indeterminada su relación con la intervención. Se le pide al modelo identificar qué está documentado y qué parte necesita revisión docente. El vocabulario médico puede ayudarle a entender equivalencias. También puede llevarlo a completar el vínculo causal porque una secuencia le parece habitual. El ensayo debe puntuar ambos aspectos de forma independiente. Identificar bien el concepto y atribuirle al estudiante una conclusión ausente son resultados que pueden coexistir en una misma salida.

La ficha informa que ciertas evaluaciones, entre ellas las de seguridad, utilizaron predominantemente instrucciones en inglés. Conviene diseñar en español los ejemplos originales, mantener las expresiones espontáneas de estudiantes y verificar la calidad de la devolución en ese idioma. Traducir un caso al inglés para hacer que MedGemma funcione mejor alteraría la tarea de comparación. Si se investiga la traducción, debe consignarse como una configuración distinta con tiempo y errores propios. Para el primer piloto la alternativa más interpretable es darle el mismo texto en español que reciben Qwen y Gemma, con las mismas referencias numeradas y los mismos criterios de aceptación. [13–16]

### 4.4 Qwen2.5-3B-Instruct como alternativa compacta

La ficha de Qwen2.5-3B-Instruct declara 3,09 mil millones de parámetros y menciona expresamente el español entre sus idiomas compatibles. La organización Qwen publica el archivo Q4_K_M de 2,10 GB. Es el menor del grupo y sirve para responder una pregunta concreta: si la entrevista solo necesita reconocer una intención y verbalizar un hecho disponible, ¿cuánto se pierde al utilizar el modelo más compacto? Su licencia particular de investigación es relevante para documentar el piloto. No corresponde extenderle la etiqueta Apache 2.0 de otros modelos que llevan la misma marca. [04–05, 36]

La diferencia de 0,40 GB frente a la primera opción es modesta en relación con los 16 GB instalados. Por tanto, su selección se justificaría mejor con una reducción observada de latencia, estabilidad superior bajo carga o desempeño equivalente en los casos seleccionados. Las respuestas de dos frases pueden favorecer a un modelo pequeño porque concentran su tarea en una sola acción verbal. La revisión de una entrevista larga plantea un problema distinto de seguimiento de referencias. Si este candidato cumple adecuadamente el rol de paciente y pierde evidencia al calificar, se le puede asignar únicamente la primera función y conservar las evaluaciones para otro momento. [04–05, 21–22]

### 4.5 Qwen3.5-4B como alternativa de arquitectura

Qwen3.5-4B introduce una arquitectura híbrida. La configuración publicada intercala bloques Gated DeltaNet con bloques de atención, lo que cambia la forma de mantener y procesar información contextual. La conversión de ggml-org seleccionada ocupa 3,03 GB. La documentación del modelo también distingue configuraciones de generación con y sin razonamiento. Para comparar un diálogo breve interesa fijar y registrar el modo utilizado, verificar la plantilla y observar lo que realmente entrega el motor. Si se habilita razonamiento en la revisión docente, el tiempo total y el presupuesto de salida deben medirse como condiciones distintas. [06–07, 21]

La hipótesis más valiosa es que esta alternativa podría manejar de manera diferente los antecedentes repartidos en una entrevista extensa. Para probarla se pueden preparar tres versiones de un mismo historial donde la pieza decisiva aparezca al comienzo, en medio y al final. Se mantiene constante el resto del caso, incluida la pregunta del estudiante. Cuando el modelo omite una pieza que estaba disponible, se documenta su posición, la cantidad de tokens y si se aplicó recorte. Si el antecedente desapareció antes de que el motor recibiera el texto, el fallo pertenece a la construcción de contexto. Si llegó correctamente, corresponde estudiar la respuesta del candidato.

### 4.6 Qwen3-8B como segunda lectura

Qwen publica directamente el GGUF Q4_K_M de Qwen3-8B, de 5,03 GB. La ficha del modelo original explica cómo alternar modos de razonamiento mediante la plantilla. En la estación descrita tiene sentido ensayar este tamaño después de establecer una referencia con 4B. Una configuración que logra cargar pesos todavía debe dejar memoria para el contexto y sostener una espera aceptable. Para un evaluador que trabaja después de la entrega, la demora tiene un costo distinto al de interrumpir la entrevista entre dos preguntas. El archivo mayor puede permanecer descargado en disco y cargarse al terminar una tanda de prácticas. [11–12, 21–22]

La inversión en un evaluador mayor tiene una condición verificable. Debe reducir errores que importan al docente, por ejemplo acreditar acciones inexistentes, confundir quién hizo una afirmación o redactar observaciones difíciles de sostener con una cita. Para comprobarlo se prepara una muestra de errores del modelo menor y otra muestra aleatoria de intentos ordinarios. La primera indica si el cambio resuelve el problema que motivó la comparación. La segunda evita elegir un modelo que mejora situaciones excepcionales a costa de empeorar la mayoría de los casos. Si el beneficio aparece solo en una clase de criterio, puede reservarse para esa revisión específica.

## 5. Ejecución reproducible en CPU y sin conexión durante la práctica

### 5.1 Motor elegido y preparación de archivos

llama.cpp proporciona un motor de inferencia local, servidor HTTP, herramientas de medición y documentación para obtener modelos GGUF. En una estación x86 se puede usar una distribución de CPU compatible con el procesador o compilarla siguiendo las instrucciones del proyecto. Ollama es una alternativa si el equipo ya lo utiliza y prefiere administrar modelos por nombre. Para resolver la pregunta inicial se favorece llama.cpp porque el nombre del archivo, el número de hilos, el contexto y la ausencia de descarga de capas a GPU quedan visibles en el comando de arranque. La aplicación puede invocarlo mediante HTTP local sin implementar inferencia dentro de SIRCLE. [19–26, 37–38]

La preparación con conexión incluye el motor y el archivo de pesos. El ensayo sin conexión empieza una vez completada la descarga y verificada la integridad. La siguiente secuencia de referencia usa una terminal Bash en Linux, la utilidad `hf` disponible y un paquete de llama.cpp que expone `llama-server` y `llama-bench` en el entorno de comandos. Se conserva el nombre exacto del GGUF distribuido por Unsloth. El valor SHA-256 procede de la página del archivo consultada al cierre del informe y permite detectar una descarga incompleta o un cambio posterior de contenido. El proyecto del motor publica también instrucciones para Windows. [19–22, 27, 44]

```bash
hf download unsloth/Qwen3-4B-Instruct-2507-GGUF \
  Qwen3-4B-Instruct-2507-Q4_K_M.gguf --local-dir modelos

MODELO=modelos/Qwen3-4B-Instruct-2507-Q4_K_M.gguf
echo "3605803b982cb64aead44f6c1b2ae36e3acdb41d8e46c8a94c6533bc4c67e597  $MODELO" | sha256sum -c -
```

La comprobación debe indicar que el archivo coincide antes de utilizar los resultados siguientes. Si se actualiza el modelo, se registra el nuevo hash junto con la revisión del repositorio y se conserva la identidad del anterior para poder reproducir las comparaciones. En PowerShell, `Get-FileHash -Algorithm SHA256` permite comparar el archivo con la misma huella. Conviene registrar también la salida de `llama-server --version` y comprobar con `--help` que la distribución instalada admite los argumentos utilizados. Esto evita interpretar como fallo clínico una incompatibilidad entre el formato de la plantilla y una compilación antigua. [19–21, 27, 44]

```bash
llama-server -m "$MODELO" --alias paciente-local \
  --device none -ngl 0 -t 6 -c 4096 -np 1 \
  --host 127.0.0.1 --port 8080
```

Seis hilos son una condición inicial para un ejemplo de seis núcleos físicos. El equipo real puede justificar otro valor. La dirección de escucha restringe el servicio a la propia estación. Si SIRCLE reside en otro servidor, la integración requiere definir explícitamente qué componente se ejecutará junto al motor y cómo se autorizará la comunicación interna. Para demostrar que la inferencia es local basta con iniciar la prueba en la misma estación, desconectar la salida a Internet tras obtener los archivos y completar una entrevista y una evaluación. El registro de arranque debe identificar los búferes del modelo en CPU. La mera presencia de una Radeon en el inventario del sistema carece de valor como indicador de dónde se ejecutó el cálculo. [19–21]

### 5.2 Primer intercambio y microensayo

Desde una segunda terminal se consulta el estado del servidor y se realiza una solicitud de prueba. El siguiente caso inventado no requiere conocimientos clínicos. Solo comprueba que el modelo responde en el idioma y el rol adecuados. Un resultado admisible expresa que la molestia comenzó ayer por la tarde, en primera persona y sin añadir una causa. Se debe conservar el texto real para comparar ejecuciones. La aplicación de laboratorio debe considerar que una respuesta HTTP satisfactoria acredita que el servidor contestó, mientras que la concordancia del contenido requiere revisar el texto. [01, 21]

```bash
curl -sS http://127.0.0.1:8080/health
curl -sS http://127.0.0.1:8080/v1/chat/completions \
  -H 'Content-Type: application/json' \
  -d '{"model":"paciente-local","messages":[
    {"role":"system","content":"Representa a Elena, paciente ficticia. Responde en español y en primera persona. La molestia comenzó ayer por la tarde. Usa solo los datos del guion."},
    {"role":"user","content":"¿Desde cuándo se siente así?"}
  ],"max_tokens":64,"temperature":0,"stream":false}'
```

Para medir el motor de manera aislada se detiene antes el servidor y se usa el mismo archivo, en una compilación solo CPU. Si la segunda terminal no conserva la variable del primer ejemplo, se define de nuevo. El comando propone cinco repeticiones en lugar de una y seis hilos como punto de partida. La documentación de la herramienta explica que las pruebas de procesamiento del texto inicial y generación usan cargas sintéticas. Conservar las salidas completas facilita detectar diferencias de versión o de unidades cuando se revisen los resultados varias semanas después. [20–22]

```bash
MODELO=modelos/Qwen3-4B-Instruct-2507-Q4_K_M.gguf
llama-bench -m "$MODELO" -ngl 0 -t 6 -p 512 -n 128 -r 5
```

La comparación entre cuatro, seis y el número de hilos lógicos disponible se hace en tandas separadas con la máquina en condiciones equivalentes. Utilizar todos los hilos puede dejar menos recursos a la interfaz o al guardado de la ficha. En la entrevista puede darse el caso de que la generación aislada mejore ligeramente y al mismo tiempo empeore la demora que experimenta el estudiante. Por ello se conserva una segunda serie de mediciones de extremo a extremo con la aplicación abierta. Un ensayo que solo informa la mejor cifra de tokens por segundo omite exactamente las operaciones que condicionan la experiencia del curso. [21–22, P1]

### 5.3 Ollama y comportamiento local

Ollama documenta cómo importar un archivo GGUF mediante una instrucción `FROM` en un Modelfile y crear un nombre local. Permite solicitar salidas estructuradas conforme a un esquema y ofrece un modo local mediante `OLLAMA_NO_CLOUD=1` aplicado al proceso servidor. Su documentación también describe cómo consultar con `ollama ps` qué parte del modelo se encuentra en CPU o GPU. Si se elige esta ruta para acreditar el requisito de hardware, se debe verificar que el modelo cargado figura íntegramente en CPU. Desactivar servicios de nube y desactivar aceleración gráfica son dos controles independientes. [25–26, 38]

La ruta con Ollama resulta útil al integrar una interfaz que ya utilice su API. El equipo debe identificar el archivo que hay detrás del nombre local y registrar parámetros efectivos de contexto y generación. Si cambia a una etiqueta de catálogo, conviene volver a comprobar la procedencia y el tamaño, pues una etiqueta parecida puede apuntar a otro ajuste instructivo. Durante una prueba comparativa se mantiene fijo el motor. Cambiar simultáneamente pesos, plantilla y sistema de ejecución haría difícil atribuir la diferencia observada. Una vez elegido el modelo se puede estudiar qué motor mantiene mejor la latencia y la facilidad de operación en el laboratorio. [19, 21, 25–27, 38]

## 6. Diseño del paciente virtual y manejo de la evolución

### 6.1 Qué información recibe quien interpreta al paciente

El primer insumo es un guion docente organizado por etapa y accesibilidad. Una parte recoge la identidad ficticia, la situación inicial y el registro lingüístico esperado. Otra parte contiene información que el personaje puede revelar cuando se le pregunta. Una tercera describe datos disponibles únicamente en la ficha o después de un evento. La rúbrica pertenece al material del evaluador. Este reparto reduce los fallos de revelación y la cantidad de texto que debe leer el procesador. Para el caso de Elena, por ejemplo, se puede indicar que reconoce el momento de inicio y que desconoce el nombre de una evaluación anterior. En la primera pregunta no necesita recibir el desenlace del caso. [29–30, 33, P1–P2]

Antes de responder, la aplicación obtiene el intento activo y su estado confirmado. Selecciona los hechos relevantes para esa etapa y un fragmento del diálogo que mantenga la continuidad. Si el contenido depende del tipo de pregunta, una selección determinista o una búsqueda en el catálogo del caso puede reducir información impertinente. El modelo realiza la tarea generativa de comprender la formulación y hablar como el personaje. Cuando el guion carece de un dato, la respuesta esperada depende de la naturaleza del vacío. Una persona puede desconocer qué examen se tomó. Si falta una característica esencial del propio guion, corresponde registrarlo para el autor y evitar que cada estudiante reciba una historia distinta. [P1–P2, 29, 33]

La naturalidad se juzga en relación con el personaje. Es razonable que una persona recuerde una expresión cotidiana y desconozca el término técnico. Si el estudiante pregunta «¿desde cuándo le pasa?» y el modelo contesta con una explicación enciclopédica, el intercambio pierde valor aun cuando contenga vocabulario correcto. A su vez, algunos personajes pueden conocer términos precisos por su historia o formación. El guion debe decirlo. Aplicar una regla universal que obligue a todos a hablar de forma simplificada empobrece la simulación. La revisión docente marcará cuándo una expresión es coherente con ese personaje y cuándo solo reproduce el estilo habitual del asistente.

### 6.2 Acciones, reloj simulado y persistencia

El profesor describió tres etapas para un primer caso manejable: presentación de síntomas, indicaciones del estudiante y respuesta a las indicaciones con un cierre. El modelo puede interpretar una frase como propuesta de acción y verbalizar el resultado narrativo de un cambio. La aplicación registra por separado la indicación propuesta, su confirmación y la transición prevista por los autores del caso. Esta decisión técnica preserva la función del agente y permite explicar al docente por qué el paciente evolucionó de determinada forma. Cuando el caso no contempla una rama, se registra la limitación y se deriva a revisión en lugar de inventar una trayectoria para completar el relato. [P1–P2, 29–30]

Una pregunta sobre un examen ilustra el problema. Si el estudiante menciona que consideraría solicitarlo, el paciente puede responder a la conversación, pero el catálogo de órdenes sigue igual. Si se confirma la solicitud en la ficha, aparece un evento de orden emitida. Si el resultado tiene una demora simulada, solo se hace visible cuando corresponde. Dos estudiantes con idéntico guion pueden llegar al cierre con información diferente porque tomaron decisiones diferentes. El evaluador debe poder reconstruir ese recorrido. Permitir que el personaje anuncie un resultado antes de su publicación rompería el vínculo entre entrevista, ficha y razonamiento, y afectaría también la evaluación del intento. [P1–P2]

La continuidad tampoco depende de dejar permanentemente abierta una conversación con el motor. Se conserva el identificador de intento, la versión del caso, la etapa, los hechos divulgados y los eventos confirmados. Al reanudar, la aplicación reconstruye el contexto imprescindible y crea otra solicitud. Si el estudiante inicia una bifurcación para explorar una decisión diferente, se copia una referencia al punto de partida y se asigna un identificador propio a las acciones posteriores. Con este diseño es viable descargar los pesos para liberar memoria o usar el evaluador sin perder la historia del paciente. También permite reproducir un fallo alimentando al motor únicamente el contexto correspondiente al instante cuestionado. [P1–P2, 21]

### 6.3 Ensayo longitudinal de ejemplo

| Momento del caso sintético | Hecho observable | Qué debe comprobar el ensayo |
|---|---|---|
| Inicio | Elena conoce cuándo comenzó su molestia | Responde en primera persona y preserva el intervalo del guion |
| Pregunta sobre examen previo | Elena desconoce el nombre | Mantiene la incertidumbre aunque se repita la pregunta |
| Nota del estudiante | Escribe «consideraría pedir el examen» | La nota queda identificada como intención |
| Confirmación posterior | Se registra una orden en la ficha | Solo desde este evento existe una solicitud ejecutada |
| Avance de tiempo | Se habilita el resultado definido por el caso | El estudiante puede consultarlo después de su liberación |
| Entrega | Queda cerrada la secuencia de acciones | El evaluador cita el evento correcto al acreditar cada criterio |

La tabla es una prueba de consistencia, de modo que el resultado puede ser una cadena sintética tan simple como «informe disponible». No requiere simular un tratamiento real. En una segunda pasada se interrumpe la sesión antes de confirmar la orden y se vuelve a abrir el intento. En la tercera se bifurca exactamente en ese punto y se deja una rama sin orden. La misma pregunta final permite observar si el sistema respeta estados distintos sin mezclar evidencia. Estas variaciones son más exigentes que preguntar tres veces al mismo personaje por su nombre y, al mismo tiempo, pueden revisarse sin equipamiento adicional ni transcripciones extensas.

## 7. Diseño del evaluador y límites de la puntuación

### 7.1 De un texto plausible a una observación comprobable

La evaluación necesita una unidad de análisis que un docente pueda inspeccionar. Se propone presentar un criterio, su descriptor de cumplimiento y un conjunto de eventos con identificadores estables. El modelo debe devolver una categoría, citar el identificador y explicar brevemente la relación entre el texto y el criterio. Una categoría de evidencia insuficiente permite derivar dudas. La interfaz puede mostrar la frase del estudiante junto a la explicación generada. Así el docente puede corregir una asociación en segundos y distinguir si el error provino de una redacción ambigua del criterio o de una lectura incorrecta del modelo. [21, 23, 26, 31–32, P1]

Por ejemplo, el criterio «solicitó el examen» admite comprobar la existencia de una orden confirmada. El criterio «explicó por qué solicitaba el examen» exige interpretar lenguaje natural. Un estudiante puede haber ejecutado la orden sin documentar su motivo, o haber razonado correctamente sin ejecutarla. La distinción importa para enseñarle qué mejorar. Si un único mensaje de devolución afirma que «faltó el examen», se puede cometer una injusticia en cualquiera de las dos direcciones. Una rúbrica dividida por conductas observables permite al evaluador identificar el matiz y a la aplicación calcular puntos una vez establecida la categoría de cada criterio. [P1–P2, 29, 31–32]

Un ejemplo de salida para la rama donde solo hubo una nota se muestra a continuación. Los identificadores y el texto son ficticios. En la entrada completa se indica que el listado de órdenes está disponible y vacío hasta el cierre. Esto permite distinguir una omisión evaluable de una pérdida de datos durante la exportación. Si faltara el listado, la categoría defendible sería «requiere revisión». El servidor puede aplicar un esquema JSON para controlar tipos y categorías. Después, la aplicación debe contrastar los identificadores con la sesión y comprobar si la evidencia realmente expresa lo que afirma el modelo. [21, 23, 26]

```json
{
  "criterio": "C_ORDEN",
  "estado": "no_cumple",
  "evidencias": ["NOTA_04", "CIERRE_09"],
  "justificacion": "La nota registra la intención. El listado de órdenes al cierre está completo y no contiene una solicitud confirmada.",
  "retroalimentacion": "Quedó registrada tu intención de pedir el examen. Para acreditar la solicitud, debes confirmarla en la ficha."
}
```

Una gramática puede garantizar que la salida incluya un arreglo de identificadores; incluso podría restringir los valores a una lista determinada. Sin embargo, acertar el identificador exige entender una relación de significado. Si se menciona una nota válida que pertenece al intento correcto pero sustenta otra conducta, la salida será sintácticamente impecable y educativamente errónea. En el piloto se registran por separado formato válido, identificadores existentes, pertinencia de la cita y correspondencia del estado con el patrón docente. Esta separación permite priorizar fallos. Reintentar una salida malformada puede resolver el formato; repetir una inferencia con un criterio ambiguo puede limitarse a producir otra explicación igualmente convincente. [21, 23, 26, 31–32]

### 7.2 Supervisión y tratamiento de desacuerdos

El borrador de requerimientos ya aporta una prueba de cálculo. En PR12, dos intentos de ejemplo se valoran con una rúbrica ilustrativa de cien puntos. Uno cumple todo y otro omite un criterio de diez puntos. Las salidas esperadas son cien y noventa y cada descuento debe quedar asociado a su evidencia. Este ensayo es barato y conviene aplicarlo antes de pedir a profesores que lean veinte devoluciones. Si la aplicación calcula mal una suma o duplica una entrega al recibir una segunda confirmación, la calidad lingüística del modelo tiene poca importancia. Tras comprobar ese circuito se incorporan criterios para los cuales la interpretación del lenguaje sí aporta valor. [P1]

En PR13 se contempla tanto una revisión pendiente como un fallo del evaluador. La entrega se conserva y la publicación definitiva espera a que se resuelvan los criterios. Un error técnico no se traduce en una calificación de cero. La propuesta respeta ese comportamiento y añade información para estudiar qué modelo genera más trabajo. Por ejemplo, un candidato que deriva tres criterios difíciles al docente puede ser preferible a otro que aparenta resolverlos todos y acredita conductas inexistentes. Además del porcentaje de acuerdo interesa medir cuántas correcciones son sustantivas, cuánto tarda el revisor y qué clase de explicación le facilita tomar una decisión. [P1, 31–32]

Una segunda lectura por otro modelo puede detectar discrepancias, aunque los dos pueden compartir un error. Interesa conservar las justificaciones y mostrar al docente los extractos originales en forma neutral. Si el segundo candidato recibe primero la conclusión del anterior, la comparación incorpora una influencia adicional. Para un ensayo de independencia se entrega a ambos el mismo material y se recogen sus respuestas por separado. La revisión docente posterior puede ocultar el nombre del modelo y alternar el orden de presentación. Esto reduce el peso de expectativas sobre el prestigio del fabricante o sobre el supuesto valor automático de la especialización médica. [13, 31–32]

## 8. Qué aporta la investigación sobre simulación y evaluación

### 8.1 MedSP1000 como referencia para construir casos

Liang y colaboradores presentan MedSP1000 como un banco de evaluación interactiva derivado de casos de pacientes estandarizados. La publicación y la ficha pública consignan **1.638 casos y 24.602 ítems de rúbrica**. El registro interno de la reunión mencionaba preliminarmente 1.639 escenarios. Para describir el recurso público corresponde utilizar la cifra que declaran los autores y conservar la observación interna como antecedente. El conjunto organiza archivos para el personaje, el evaluador y el entorno. Su ficha proporciona una correspondencia con los artículos originales en MedEdPORTAL. Esta trazabilidad facilita revisar de dónde procede un escenario antes de adaptarlo a otra disciplina o nivel formativo. [29–30, 42, P2]

En el estudio, un agente que ocupa el rol clínico interactúa con un paciente y con un controlador del entorno. La evaluación examina qué proporción de ítems de una trayectoria se completó. El resultado publicado de 60,4 % para el modelo que obtuvo el mejor desempeño se refiere a esa fracción de ítems en el experimento. Interpretarlo como una tasa de aprobación de estudiantes o como acierto al representar pacientes cambiaría la unidad de análisis. Tampoco describe las capacidades de los GGUF de este informe. Para la práctica importa principalmente la manera de dividir los papeles y de evaluar un proceso a lo largo del tiempo. [29–30, 42]

Una adaptación local podría partir de un escenario cuyos materiales permitan reconstruir qué información tiene el personaje al comienzo, qué puede pedir el estudiante y qué sucede después. Si un caso ofrece una entrevista valiosa pero deja abierta la evolución, el equipo docente deberá completar y versionar ese tramo antes de usarlo para calificar. Un posible procedimiento consiste en extraer solo las condiciones necesarias para tres etapas y redactarlas en español con el mismo significado pedagógico. Un segundo revisor comprueba el sentido de la adaptación y el nivel de vocabulario. Se conserva la procedencia del material, el alcance de las modificaciones y el enlace al caso original. [29–30, 33, 42, P2]

El recurso público se presenta como un árbol de archivos y el proyecto incluye código de simulación. Descargar los datos no instala un modelo de IA, y disponer del código de evaluación tampoco certifica que su configuración por defecto opere sin servicios externos. La ruta aquí propuesta reutiliza las ideas de organización y los casos que el equipo decida adaptar. El componente que genera el diálogo y la evaluación es el GGUF local escogido. Para demostrar independencia de servicios alojados se debe ejecutar la prueba sin salida a Internet una vez terminada la preparación. Es una propiedad comprobable del sistema completo, no una consecuencia automática de que el conjunto de datos sea descargable. [19–21, 29–30, 42]

### 8.2 Qué miden las evaluaciones estáticas y qué se pierde en ellas

HEAD-QA reúne preguntas de opción múltiple procedentes de exámenes de acceso a formación sanitaria especializada en España. Ofrece una referencia explícita de comprensión biomédica en español. Se puede usar una muestra para explorar si un modelo confunde términos o si una conversión produce salidas anómalas. Acertar una alternativa requiere capacidades distintas de mantener una versión consistente de los hechos durante una entrevista. Un candidato puede reconocer la respuesta de examen y, como paciente, revelar información que el alumno todavía no ha obtenido. Esa comparación suplementaria se debe informar en otra columna y evitar que mejore artificialmente la puntuación del rol conversacional. [29, 39–40]

La investigación sobre preguntas médicas complejas ha mostrado que la evaluación puede incluir más dimensiones que la exactitud literal. En una práctica de razonamiento clínico son especialmente importantes el origen del dato, el momento de disponibilidad y la acción que lo generó. Al leer una transcripción, un experto distingue una conjetura del alumno, un hallazgo validado y una respuesta del personaje. Un indicador agregado de aciertos elimina gran parte de esas distinciones. Por ello los ejemplos que discriminan entre candidatos deben incluir al menos una situación en la que todos reconozcan el término médico, pero solo algunos atribuyan correctamente su fuente y su momento. [29, 39–40, P1]

### 8.3 Evidencia educativa y revisión humana

Los estándares de la Association of Standardized Patient Educators orientan el desarrollo de casos y la preparación del trabajo con pacientes simulados. Una aplicación local puede tomar de esa tradición la consistencia del personaje y la relación explícita entre actividades y objetivos de aprendizaje. Los docentes ayudan a determinar qué información debe proporcionar el paciente de manera espontánea, cuál exige una pregunta pertinente y qué incertidumbre forma parte del caso. Es un trabajo de autoría que antecede al ensayo del modelo. Si dos revisores esperan respuestas incompatibles para la misma pregunta, habrá que resolver primero qué pretende enseñar el guion. [33]

En un ensayo sobre simulación clínica virtual en enfermería, Padilha y colaboradores observaron diferencias favorables en medidas de conocimiento y satisfacción, y no encontraron una diferencia estadística en autoeficacia percibida. El estudio se desarrolló en otro contexto y precede a los modelos aquí examinados. Sirve para decidir qué medir por separado. La revisión de alcance sobre pacientes virtuales basados en modelos de lenguaje también describe una literatura con abundancia de pilotos y dificultades de fidelidad. Para SIRCLE resulta razonable recoger percepción del estudiante, concordancia de la evaluación y resultados de aprendizaje como variables independientes. Ninguna permite sustituir automáticamente a las otras. [34–35, P1–P2]

Los estudios sobre evaluadores de lenguaje, entre ellos MT-Bench y G-Eval, documentan problemas de preferencia por posición, extensión o estilo en determinadas tareas. Ese antecedente sugiere un ensayo local concreto. Se redactan dos entregas con el mismo contenido evaluable. Una es concisa y contiene una abreviatura aceptada. La otra se expresa con frases más largas y tono académico. Un criterio de «registró el momento de reevaluación» debería conservar su decisión en ambas. Si la rúbrica incluye además un criterio de comunicación, se examina por separado. De ese modo se detecta si el modelo premia la apariencia general de una entrega al puntuar otra habilidad. [31–32]

## 9. Diseño de la muestra y referencia docente

Se propone comenzar con tres casos ficticios de tres etapas cada uno, elegidos por disponibilidad de guion y objetivos de aprendizaje. Es una cifra de diseño para obtener información durante la práctica. Un caso sencillo puede concentrarse en el relato del motivo de consulta. Un segundo puede requerir recuperar información después de una pausa y un tercero distinguir una nota, una acción y un resultado. Es preferible empezar con un número de casos que dos docentes puedan revisar con detenimiento. La variedad posterior debe incorporar guiones redactados de manera independiente para evitar que el modelo solo parezca consistente porque todas las preguntas siguen una frase idéntica. [29–30, 33, P2]

Para evaluar al paciente se prepara, por ejemplo, un conjunto de doce preguntas por caso, distribuidas entre hechos espontáneos, hechos condicionados, información desconocida y datos todavía ocultos. Son 36 situaciones iniciales. Dos formulaciones naturales por situación dan 72 entradas comparables, antes de considerar repeticiones. Las formulaciones podrían alternar «¿cuándo partió?» y «¿desde cuándo lo nota?», o una pregunta abierta y otra que presupone una fecha equivocada. La respuesta patrón define el núcleo factual y qué añadidos son inadmisibles. Evaluar con una única redacción literal penalizaría diferencias expresivas que tienen valor en una conversación real.

Para el evaluador se preparan entregas completas y pares casi idénticos. En un par solo cambia si la orden fue confirmada. En otro cambia quién formuló la explicación. Un tercero conserva los eventos pero elimina la información necesaria para decidir un criterio. Dos docentes pueden valorar de manera independiente una muestra inicial y registrar el motivo de sus desacuerdos antes de acordar una referencia común. Si el desacuerdo se concentra en una palabra de la rúbrica, reescribirla probablemente mejore tanto la consistencia humana como la automática. La versión acordada se cierra antes de comparar candidatos en los intentos reservados. [29, 31–33, P1]

La separación entre desarrollo y comprobación final se hace por caso o por familia de intentos. Si una variante apenas cambia el nombre del paciente, conserva demasiada semejanza con el ejemplo utilizado para ajustar el mensaje de sistema. Reservar familias completas permite estudiar si la mejora se mantiene frente a material nuevo. En una muestra pequeña se deben comunicar cantidades absolutas y denominadores. Informar «dos revelaciones prematuras en doce oportunidades de revelar un dato oculto» permite al lector juzgar mejor la gravedad que anunciar un porcentaje de fidelidad sobre todos los turnos, entre los que quizá predominen preguntas muy fáciles.

La revisión del español abarca algo más que gramática. El docente observa si el paciente responde con un grado de formalidad verosímil, si entiende preguntas breves y si reconoce expresiones locales sin convertirlas en caricaturas. También conviene ensayar escritura con pequeños errores de digitación y preguntas dobles en las que solo una parte puede contestarse. En evaluación, varias expresiones coloquiales pueden satisfacer el mismo criterio que una palabra técnica. La anotación conserva esos equivalentes aceptables y explica cuándo la precisión terminológica sí es parte explícita del objetivo. Esto ayuda a evitar que el sistema mida el estilo personal del estudiante en lugar de la conducta prevista.

## 10. Protocolo de rendimiento y calidad para la estación UFRO

### 10.1 Mediciones de una sesión y de un curso

Se registran cuatro tiempos con relojes comparables: envío de la pregunta, aceptación por el motor, aparición del primer texto útil y finalización visible de la respuesta. La diferencia entre envío y aceptación identifica espera en cola o en la aplicación. El primer texto útil aproxima la espera que percibe el alumno si se presenta la respuesta progresivamente. La finalización indica cuándo puede leerla completa. También se conserva la cantidad de tokens de entrada y salida. Si el motor devuelve la respuesta de una sola vez, la medición de primer texto visible coincidirá con la presentación final aunque internamente haya calculado el primer token antes. [21–22, P1]

El borrador RR01 y la prueba PR20 proponen 20 usuarios concurrentes durante quince minutos, al menos cien operaciones por categoría, p95 de dos segundos en apertura, tres en guardado, quince en diálogo y cinco en avance temporal, y al menos 99 % de finalización de solicitudes válidas. Son metas por ratificar. Una fase inicial con una sesión permite detectar problemas de instalación. Después pueden ensayarse dos y cuatro sesiones, y finalmente el perfil acordado. Veinte personas con sesiones abiertas y veinte generaciones simultáneas representan cargas diferentes. El informe de medición debe declarar el intervalo entre acciones, la mezcla de operaciones y qué evento visible se tomó como final del diálogo. [P1]

| Medida | Registro mínimo | Consecuencia para la elección |
|---|---|---|
| Latencia en frío | Tiempo de carga más primera respuesta | Viabilidad de iniciar el servicio al comienzo de una clase |
| Latencia en caliente | Mediana y p95 hasta primer texto y hasta cierre | Fluidez en entrevista y cumplimiento del perfil |
| Memoria | Disponible antes y durante, pico del proceso y actividad de paginación | Margen con interfaz y base de datos abiertas |
| Estabilidad | Intentos completados, errores, reintentos y esperas en cola | Capacidad de sostener sesiones reales |
| Fidelidad del paciente | Hechos correctos, invenciones, contradicciones y fugas por oportunidad | Aptitud para trabajo sin supervisión continua |
| Calidad de evaluación | Aciertos por criterio, falsas acreditaciones, derivaciones y citas pertinentes | Volumen de corrección antes de publicar |
| Tiempo docente | Minutos por entrega revisada y tipo de cambio requerido | Beneficio operativo frente a una revisión manual |

Como ejemplo de interpretación, si una generación completa necesita doce segundos en una estación que procesa solo una por vez, su capacidad teórica sería de cinco respuestas por minuto antes de sumar otras operaciones. Si veinte estudiantes envían una pregunta simultáneamente, la última respuesta del grupo podría terminar alrededor de cuatro minutos después bajo esas hipótesis. Esta aritmética permite distinguir rendimiento individual de capacidad compartida. Los valores son ficticios. El procesamiento en lotes, la mezcla de consultas y los momentos reales en que los estudiantes escriben pueden cambiar el comportamiento. La prueba de carga debe medirlo y también confirmar que los tiempos de guardado permanecen aceptables durante la generación. [21–22, P1]

### 10.2 Indicadores de evaluación y decisión de aceptación

Para cada criterio se construye una tabla de resultados frente a la referencia docente. En un criterio binario se cuentan verdaderos cumplimientos, omisiones reales, créditos indebidamente concedidos y cumplimientos que el modelo pasó por alto. Una tasa de acuerdo global puede ser muy alta si casi todos los estudiantes cumplen y el modelo responde siempre que sí. Mostrar la precisión del crédito concedido y la sensibilidad para detectar cumplimiento ayuda a interpretar esa situación. Los casos derivados a revisión se informan en una categoría propia. La cobertura automática indica qué proporción quedó resuelta y debe presentarse junto a la exactitud dentro de esa proporción. [31–32, P1]

Por ejemplo, si de veinte decisiones diez reciben crédito automático, ocho de esas diez coinciden con los docentes, seis se clasifican como incumplimiento y cuatro se derivan, la precisión del crédito otorgado es ocho de diez y la cobertura es dieciséis de veinte. Con esos dos datos aún se desconoce cuántos de los seis incumplimientos son correctos. Conservar la tabla completa evita interpretar los resultados como si se conociera una exactitud que todavía no se puede calcular. También ayuda a comparar dos candidatos cuando uno prefiere derivar y el otro toma más decisiones arriesgadas. Las cifras de este ejemplo ilustran el cálculo y carecen de valor experimental.

Se propone que una invención que cambie la trayectoria del caso o una acreditación sin respaldo en un criterio esencial impida aprobar automáticamente ese uso hasta revisar el fallo. Para otras dimensiones, el equipo docente debe fijar de antemano la tolerancia y el procedimiento ante dudas. Un modelo que supera un conjunto limitado de ejemplos puede avanzar a un piloto supervisado. Generalizarlo a todos los ramos, cohortes o disciplinas exigiría nueva evidencia. Al comunicar el resultado se indica el número de casos, idiomas, longitud típica de sesión y distribución de criterios que el ensayo realmente cubrió. [13, 29, 31–35, P1]

## 11. Decisiones de implementación según los resultados

| Observación en el laboratorio | Decisión fundada | Verificación posterior |
|---|---|---|
| Qwen 2507 mantiene el rol y cita correctamente | Reutilizar una instancia para las dos tareas | Probar dos solicitudes aisladas y alternadas, incluyendo retorno al rol de paciente |
| Gemma 3 conserva mejor el secreto del caso | Adoptarlo como paciente si la latencia es aceptable | Repetir con casos nuevos y preguntas que presuponen el desenlace |
| MedGemma interpreta mejor las notas, pero añade hechos | Limitarlo a criterios donde conserve respaldo verificable | Medir correcciones y reincidencia en notas reservadas |
| El candidato de 4B tarda demasiado en diálogos cortos | Medir Qwen2.5-3B con la misma tarea y revisar la longitud de entrada | Comparar segundos por respuesta completa y fidelidad |
| Qwen3.5 mejora la recuperación de antecedentes | Usarlo en el rol donde se observó la mejora | Repetir con igual contexto útil y costos registrados |
| Qwen3-8B reduce falsas acreditaciones y tarda más | Ejecutar la revisión diferida después de entregar | Medir la cola al cierre de una clase y el tiempo docente ahorrado |
| Un usuario cumple la meta y veinte exceden el p95 | Declarar la capacidad demostrada y revisar el despliegue del curso | Repetir PR20 con el perfil acordado, sin reetiquetar el ensayo individual |

Hay dos decisiones de diseño particularmente sensibles al hardware. La primera es elegir entre mantener dos modelos residentes o alternar la carga. Para la configuración inicial se recomienda una sola instancia. Si la evaluación diferida utiliza otro archivo, el cambio se puede agrupar por entregas y medir el costo real de recarga. La segunda decisión es cuánto diálogo anterior se envía. Un resumen manualmente contrastable de los hechos confirmados puede reducir entrada y demoras. La transcripción íntegra se conserva para el docente y se recuperan eventos concretos cuando lo requiere un criterio. Así se puede reducir el contexto de inferencia sin perder evidencia académica. [19–22, P1]

La determinación de qué información se resume merece atención especial. Un resumen que transforma «tal vez pediría un examen» en «pidió un examen» cambia el desempeño atribuido al estudiante. Para el paciente se pueden conservar hechos simples del guion y el estado actual. Para la evaluación se anteponen las notas y los eventos originales al resumen, especialmente en criterios disputados. Si falta espacio, se divide la evaluación por bloques de criterios con evidencia pertinente y se ensamblan las decisiones mediante sus identificadores. Los criterios que dependen de toda la trayectoria, como el orden o la oportunidad de una reevaluación, necesitan una vista común fechada aunque las notas se presenten por bloques.

Para escoger entre modelos también se debe contabilizar el trabajo que aparece después de generar el texto. Una devolución de diez líneas puede exigir más revisión que una de tres si contiene afirmaciones nuevas sin respaldo. Si los docentes tardan menos en revisar un caso con el modelo pequeño y la corrección final mantiene la misma calidad, esa ventaja tiene valor incluso si el otro redacta mejor. Si todos necesitan reescribir íntegramente la explicación, el sistema aún puede servir para localizar evidencia y dejar la redacción a la persona responsable. Declarar esa función acotada permite aprovechar resultados parciales y diseñar el siguiente ensayo alrededor del problema observado. [31–35, P1–P2]

## 12. Conclusiones

Hay una ruta concreta para instalar y probar IA generativa en la estación descrita. Los seis modelos cuentan con archivos de pesos descargables adecuados para un motor con ejecución en CPU. Por tamaño, procedencia identificable y generación directa, Qwen3-4B-Instruct-2507 ofrece el punto de partida más sencillo para ensayar ambos roles con una sola instalación. Gemma 3 4B IT permite contrastar esa elección a un costo de almacenamiento semejante. MedGemma 1.5 4B merece una evaluación dirigida a la lectura clínica y a la devolución sobre evidencia, con una cautela documentada respecto de su uso conversacional de varios turnos. [01–02, 08–09, 13–16, 19]

La decisión académica más importante será comprobar si el sistema preserva el significado de las acciones del estudiante. Describir una intención, confirmarla en la ficha y observar su resultado son tres acontecimientos distintos. Un buen paciente mantiene esa secuencia al responder. Un buen evaluador identifica qué parte ocurrió, cita la evidencia y deja al docente una explicación que pueda corregir. Las pruebas propuestas permiten elegir candidatos distintos para esas tareas y fijar un límite claro a la automatización cuando la evidencia del intento sea incompleta. [29–35, P1–P2]

El estado alcanzado por esta investigación es una **selección documentada y reproducible para un piloto individual supervisado**. La velocidad en el equipo UFRO, la utilidad formativa de los seis GGUF en español y la capacidad con veinte usuarios siguen siendo resultados pendientes. Para resolverlos se dispone de nombres de archivo, una referencia de integridad, comandos de ejecución, ejemplos de comportamiento esperado y criterios de medición ligados a los requerimientos del proyecto. La elección definitiva podrá sostenerse así en respuestas observadas, recursos medidos y decisiones docentes que se puedan reconstruir. [19–23, 41, 43–44, P1–P2]

## Bibliografía

### Fuentes externas consultadas

Las referencias [01] a [44] identifican páginas consultables por separado. Cuando una entrada corresponde a los pesos, otra al informe y otra a la conversión, se ha indicado la función de cada una. Varias proceden del mismo proyecto y describen niveles distintos de evidencia. Fecha de consulta: 17 de noviembre de 2026.

- **[01]** Qwen Team. *Qwen3-4B-Instruct-2507*. Ficha oficial de la variante instructiva, arquitectura, idiomas y comportamiento de generación. [Consultar fuente](https://huggingface.co/Qwen/Qwen3-4B-Instruct-2507)
- **[02]** Unsloth. *Qwen3-4B-Instruct-2507-GGUF*. Inventario de la conversión comunitaria y comparación de precisiones. [Consultar fuente](https://huggingface.co/unsloth/Qwen3-4B-Instruct-2507-GGUF/tree/main)
- **[03]** Qwen Team. *Qwen3 Technical Report*. Informe de la familia, entrenamiento y evaluación multilingüe. arXiv:2505.09388. [Consultar fuente](https://arxiv.org/abs/2505.09388)
- **[04]** Qwen Team. *Qwen2.5-3B-Instruct*. Ficha oficial, soporte del español y licencia específica de la variante de 3B. [Consultar fuente](https://huggingface.co/Qwen/Qwen2.5-3B-Instruct)
- **[05]** Qwen Team. *Qwen2.5-3B-Instruct-GGUF*. Distribución oficial de los archivos cuantizados y tamaños. [Consultar fuente](https://huggingface.co/Qwen/Qwen2.5-3B-Instruct-GGUF/tree/main)
- **[06]** Qwen Team. *Qwen3.5-4B*. Arquitectura híbrida, modalidades de entrada y control de la generación. [Consultar fuente](https://huggingface.co/Qwen/Qwen3.5-4B)
- **[07]** ggml-org. *Qwen3.5-4B-GGUF*. Archivo Q4_K_M y variantes de precisión publicadas para llama.cpp. [Consultar fuente](https://huggingface.co/ggml-org/Qwen3.5-4B-GGUF/tree/main)
- **[08]** Google. *Gemma 3 4B IT*. Ficha oficial de la versión instruida, idiomas, modalidades y límites. [Consultar fuente](https://huggingface.co/google/gemma-3-4b-it)
- **[09]** ggml-org. *gemma-3-4b-it-GGUF*. Árbol de archivos del modelo textual y del proyector separado. [Consultar fuente](https://huggingface.co/ggml-org/gemma-3-4b-it-GGUF/tree/main)
- **[10]** Gemma Team. *Gemma 3 Technical Report*. Informe técnico sobre diseño y evaluación de la familia. arXiv:2503.19786. [Consultar fuente](https://arxiv.org/abs/2503.19786)
- **[11]** Qwen Team. *Qwen3-8B*. Ficha oficial, modos de razonamiento y uso de plantillas. [Consultar fuente](https://huggingface.co/Qwen/Qwen3-8B)
- **[12]** Qwen Team. *Qwen3-8B-GGUF*. Inventario oficial de pesos cuantizados, incluida la variante de 5,03 GB. [Consultar fuente](https://huggingface.co/Qwen/Qwen3-8B-GGUF/tree/main)
- **[13]** Google Health AI Developer Foundations. *MedGemma 1.5 model card*. Capacidades, evaluación y limitación explícita de uso multivuelta. [Consultar fuente](https://developers.google.com/health-ai-developer-foundations/medgemma/model-card)
- **[14]** Google. *medgemma-1.5-4b-it*. Ficha de los pesos originales, condiciones de acceso y alcance de las evaluaciones lingüísticas. [Consultar fuente](https://huggingface.co/google/medgemma-1.5-4b-it)
- **[15]** Unsloth. *medgemma-1.5-4b-it-GGUF*. Archivos de conversión y tamaños según precisión. [Consultar fuente](https://huggingface.co/unsloth/medgemma-1.5-4b-it-GGUF/tree/main)
- **[16]** Sellergren y colaboradores. *MedGemma 1.5 Technical Report*. Evaluación y ampliaciones de la familia médica. arXiv:2604.05081. [Consultar fuente](https://arxiv.org/abs/2604.05081)
- **[17]** Google. *Health AI Developer Foundations Terms of Use*. Texto de las condiciones a las que remite MedGemma. [Consultar fuente](https://developers.google.com/health-ai-developer-foundations/terms)
- **[18]** Google. *Gemma Terms of Use*. Condiciones aplicables a los pesos Gemma distribuidos bajo este acuerdo. [Consultar fuente](https://ai.google.dev/gemma/terms)
- **[19]** ggml-org. *llama.cpp*. Repositorio principal del motor local y plataformas de ejecución. [Consultar fuente](https://github.com/ggml-org/llama.cpp)
- **[20]** ggml-org. *Building llama.cpp locally*. Instrucciones de compilación y opciones de plataforma. [Consultar fuente](https://github.com/ggml-org/llama.cpp/blob/master/docs/build.md)
- **[21]** ggml-org. *llama.cpp HTTP Server*. Opciones de dispositivo, contexto, sesiones, API y presentación de respuestas. [Consultar fuente](https://github.com/ggml-org/llama.cpp/blob/master/tools/server/README.md)
- **[22]** ggml-org. *llama-bench*. Metodología de microensayos y separación entre procesamiento de entrada y generación. [Consultar fuente](https://github.com/ggml-org/llama.cpp/blob/master/tools/llama-bench/README.md)
- **[23]** ggml-org. *GBNF Guide*. Gramáticas y conversión de esquemas para restringir la forma de la salida. [Consultar fuente](https://github.com/ggml-org/llama.cpp/blob/master/grammars/README.md)
- **[24]** Hugging Face. *GGUF*. Documentación del formato, metadatos y consulta de pesos cuantizados. [Consultar fuente](https://huggingface.co/docs/hub/gguf)
- **[25]** Ollama. *FAQ*. Funcionamiento local, desactivación de funciones de nube y comprobación del procesador utilizado. [Consultar fuente](https://docs.ollama.com/faq)
- **[26]** Ollama. *Structured outputs*. Salidas controladas por esquema y validación posterior. [Consultar fuente](https://docs.ollama.com/capabilities/structured-outputs)
- **[27]** Hugging Face. *Command Line Interface*. Descarga selectiva de archivos, revisiones y directorios locales. [Consultar fuente](https://huggingface.co/docs/huggingface_hub/guides/cli)
- **[28]** AMD. *AMD Ryzen 5 5600G — Specifications*. Datos del procesador de escritorio usado como referencia aproximada. [Consultar fuente](https://www.amd.com/en/support/downloads/drivers.html/processors/ryzen/ryzen-5000-series/amd-ryzen-5-5600g.html)
- **[29]** Liang y colaboradores. *Evaluating Large Language Models in Dynamic Clinical Decision-Making with Standardized Patient Cases*. Diseño y resultados de MedSP1000. arXiv:2606.05112. [Consultar fuente](https://arxiv.org/abs/2606.05112)
- **[30]** Liang y colaboradores. *MedSP1000*. Ficha del conjunto de datos, estructura de publicación y procedencia de los casos. [Consultar fuente](https://huggingface.co/datasets/byrLLCC/MedSP1000)
- **[31]** Zheng y colaboradores. *Judging LLM-as-a-Judge with MT-Bench and Chatbot Arena*. Investigación sobre tendencias y limitaciones de los jueces automáticos. arXiv:2306.05685. [Consultar fuente](https://arxiv.org/abs/2306.05685)
- **[32]** Liu y colaboradores. *G-Eval: NLG Evaluation using GPT-4 with Better Human Alignment*. Evaluadores generativos contrastados con valoración humana. EMNLP, 2023. [Consultar fuente](https://aclanthology.org/2023.emnlp-main.153/)
- **[33]** Lewis y colaboradores. *The Association of Standardized Patient Educators (ASPE) Standards of Best Practice (SOBP)*. Desarrollo de casos y práctica con pacientes estandarizados. Advances in Simulation, 2017. [Consultar fuente](https://advancesinsimulation.biomedcentral.com/articles/10.1186/s41077-017-0043-4)
- **[34]** Padilha y colaboradores. *Clinical Virtual Simulation in Nursing Education: Randomized Controlled Trial*. Comparación de resultados de aprendizaje, satisfacción y autoeficacia. JMIR, 2019. [Consultar fuente](https://www.jmir.org/2019/3/e11529/)
- **[35]** *Large Language Model-Based Virtual Patients for Simulated Clinical Learning: A Scoping Review*. Revisión de alcance sobre implementación y evaluación educativa de pacientes generativos. [Consultar fuente](https://www.mdpi.com/3042-6707/1/1/7)
- **[36]** Qwen Team. *Qwen Research License Agreement*. Texto de licencia incluido en el repositorio Qwen2.5-3B-Instruct. [Consultar fuente](https://huggingface.co/Qwen/Qwen2.5-3B-Instruct/blob/main/LICENSE)
- **[37]** ggml-org. *Obtaining and quantizing models*. Descarga o conversión de pesos para llama.cpp y uso de archivos locales. [Consultar fuente](https://github.com/ggml-org/llama.cpp/blob/master/docs/models.md)
- **[38]** Ollama. *Importing a Model*. Procedimiento para importar un archivo GGUF mediante Modelfile. [Consultar fuente](https://docs.ollama.com/import)
- **[39]** Singhal y colaboradores. *Towards Expert-Level Medical Question Answering with Large Language Models*. Evaluación de preguntas médicas y dimensiones de calidad. arXiv:2305.09617. [Consultar fuente](https://arxiv.org/abs/2305.09617)
- **[40]** Vilares y Gómez-Rodríguez. *HEAD-QA: A Healthcare Dataset for Complex Reasoning*. Prueba sanitaria de opción múltiple en español. ACL, 2019. [Consultar fuente](https://aclanthology.org/P19-1092/)
- **[41]** tantk. *Nanbeige4.1-3B-GGUF, CPU Benchmark*. Medición comunitaria sobre Ryzen 5 5600G, con modelo, hilos, memoria, repeticiones y backend declarados. [Consultar fuente](https://huggingface.co/tantk/Nanbeige4.1-3B-GGUF)
- **[42]** MAGIC-AI4Med. *MedSP1000*. Repositorio de los autores para simulación y evaluación por criterios. [Consultar fuente](https://github.com/MAGIC-AI4Med/MedSP1000)
- **[43]** Qwen Team. *Qwen3-4B-Instruct-2507, config.json*. Parámetros exactos de capas y cabezas empleados en la estimación de caché. [Consultar fuente](https://huggingface.co/Qwen/Qwen3-4B-Instruct-2507/raw/main/config.json)
- **[44]** Unsloth. *Qwen3-4B-Instruct-2507-Q4_K_M.gguf*. Página del archivo seleccionado, tamaño y huella SHA-256. [Consultar fuente](https://huggingface.co/unsloth/Qwen3-4B-Instruct-2507-GGUF/blob/main/Qwen3-4B-Instruct-2507-Q4_K_M.gguf)

### Documentación interna de la práctica

- **[P1]** Joaquín Arriagada. *Documento para la Especificación de Requerimientos Paciente Virtual*. Versión 0.1, 30 de octubre de 2026. En particular, CU15–CU16, RR01, PR07–PR13, PR20 y PR25. Fuente de las metas propuestas y del comportamiento esperado ante revisiones y fallos.
- **[P2]** *Registro de reunión de práctica con el docente Andrés Santos*. 6 de noviembre de 2026. Fuente de los tres componentes de SIRCLE, evolución en etapas, revisión docente y encargo de explorar inferencia sin GPU.
