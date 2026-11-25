<!--
Informe de impacto de la constitución
Cambio de versión de plantilla sin ratificar a 1.0.0 por adopción inicial.
Se incorporan los principios I a VI y las secciones de decisiones clínicas,
revisión en Spec Kit y gobierno. No había principios anteriores que retirar.
La elección del triaje de LR corresponde al docente antes de activar el caso.
-->

# Constitución de Paciente Virtual SIRCLE

## Principios del módulo 01

### I. El recorrido completo de LR guía la arquitectura

El módulo debe permitir observar cómo un estudiante obtiene un hallazgo y lo usa para justificar
una hipótesis inicial. Cada decisión de arquitectura debe indicar qué parte de este recorrido
hace posible o protege.

1. El estudiante entra con una identidad ficticia, elige la actividad de LR y responde u omite
   las **cinco preguntas del pre** antes de ver la introducción clínica, la ficha o el chat.
   Al abrir el caso se cierra el pre. Al reanudar se conserva la decisión.
2. Entrevista a LR, realiza una medición con un instrumento y vincula la evidencia a su
   hipótesis. El historial distingue lo que dijo la paciente, lo que midió el instrumento
   y lo que interpretó el estudiante. Al confirmar la entrega se fija una copia de ese trabajo.
3. Tras la confirmación, responde u omite las **siete preguntas del post**, incluso si la
   evaluación sigue pendiente. Solo después de esa decisión puede consultar su propia devolución
   publicada por el docente. Ver la devolución cierra la posibilidad de registrar un post comparable.

El pre y el post comparten tres ítems de percepción con igual redacción, orden y escala de 1 a 5.
Cada momento incluye dos ejercicios de razonamiento en formas equivalentes. El post añade dos
ítems de utilidad. El docente revisa las claves y la equivalencia antes del piloto. Los resultados
se registran por estudiante, actividad, intento, momento y versión del instrumento, separados de
la nota clínica. La omisión se registra como dato faltante y permite continuar sin penalización.

**Evidencia de revisión.** Para CU09 / RF11 / PR15, un intento ficticio completa ambos momentos
y reenvía el post, otro omite el pre y completa el post, y un tercero completa el pre, reanuda
y omite el post. El informe debe mostrar dos pre, dos post y un par completo. En el primer
intento, el docente debe poder seguir la hipótesis hasta el turno y la medición citados y
explicar la nota clínica sin recurrir a la encuesta.

### II. Dos aplicaciones independientes, organizadas por funcionalidad

El monorrepositorio debe tener `frontend/` y `backend/` en la raíz. Cada aplicación se instala,
ejecuta y prueba por separado. El frontend usa HTML semántico, CSS y TypeScript. El backend usa
Rust con Axum. Se comunican por una API REST con JSON.

Entrevista, valoración, encuestas y devolución agrupan el código de cada funcionalidad en la
aplicación que la atiende. Por ejemplo, colocar el brazalete es un gesto de valoración en el
frontend. Validar la medición y guardar su valor, unidad y momento corresponde a valoración en
el backend. Se extrae una abstracción compartida cuando al menos dos módulos necesitan el mismo
comportamiento y se pueden identificar ambos usos.

**Evidencia de revisión.** Desde cada directorio debe poder comprobarse su instalación, arranque
y pruebas. El frontend debe permitir ensayar el brazalete con respuestas JSON de ejemplo.
La misma operación, enviada directamente a la API con el navegador cerrado, debe producir una
única medición válida aunque se reenvíe la petición.

### III. Fidelidad a la interfaz y control de la información clínica

El HTML de Semana 5 y su carpeta de recursos se conservan íntegros como referencia de
comparación. La interfaz debe reutilizar la tipografía Roboto, la paleta, los iconos, las
expresiones de los pacientes y los instrumentos del repositorio. El backend autoriza los datos
según el rol, el propietario del intento y la fase. Al navegador del estudiante solo llegan
recursos y datos visibles en ese momento. El HTML de demostración, los hechos reservados, las
claves, las rúbricas y los borradores docentes quedan fuera de los archivos públicos y de las
respuestas de la API al estudiante. [HTML de Semana 5](../../docs/referencias/html-semana5/Paciente_Virtual.html) · [Recursos audiovisuales](../../recursos-audiovisuales/)

**Evidencia de revisión.** A igual resolución, repetir en ambas interfaces el gesto de colocar
el brazalete sobre el brazo ampliado y tomar la presión. Comparar las capturas, los estados,
las etiquetas y la unidad desde la preparación hasta el resultado y su registro en el historial.
Toda diferencia clínica debe remitir a una decisión docente. Examinar los archivos que descarga
el navegador y las respuestas de red de una sesión estudiantil para verificar el límite de acceso.

### IV. Cuatro trabajadores con memoria aislada y una sola base de datos

El servicio backend debe ejecutar cuatro procesos trabajadores independientes. Cada uno tiene
identidad fija, fuentes propias y operaciones autorizadas. Pueden compartir los pesos de
`qwen3.5:4b` mediante Ollama local. El backend selecciona el trabajador a partir del intento,
le entrega el contexto autorizado para ese turno y valida su respuesta antes de guardarla.

| Trabajador | Fuente y operación autorizada |
|---|---|
| LR (P01) | Responde en primera persona con el relato aprobado de LR para la fase del intento. Los valores de los instrumentos proceden de valoración. |
| Nannu (P02) | Recupera solo su escenario y atribuye cada respuesta a Nannu o a su acompañante según el guion. |
| Dan Sparks (P03) | Responde con la historia aprobada de Dan. Los informes de exámenes se consultan por la operación que los habilita. |
| Evaluador | Recibe solo una entrega confirmada, su rúbrica y las evidencias de ese intento. Propone observaciones revisables por criterio, con cita del registro que las sustenta. |

SQLite es la única base de datos. Debe conservar los tres escenarios y sus 14 documentos de
origen, con rol, ruta y huella de cada fuente. Cada versión clínica enlaza sus fuentes y su
estado de aprobación. SQLite guarda también intentos, turnos, mediciones, hipótesis,
encuestas, entregas y devoluciones. La memoria se recupera por **estudiante, intento y agente**,
con la versión clínica fijada al abrir el caso. Cada llamada al modelo reconstruye exclusivamente
ese contexto. Confirmar la entrega registra en una misma transacción la copia entregada y su
evaluación pendiente. El docente puede corregir las observaciones y debe registrar el motivo
de sus correcciones. Solo el docente decide si publica la devolución y qué versión publica.

**Evidencia de revisión.** Alternar dos estudiantes que entrevistan a LR, intercalar turnos de
Nannu y Dan y reiniciar un trabajador. Cada estudiante debe recuperar únicamente su historia,
con el hablante correcto y una sola copia del turno reintentado. Enviar al evaluador una entrega
que diga «mediría la presión» sin un evento de medición. Debe señalar la falta de registro.
La revisión debe mostrar el criterio, la evidencia disponible, la corrección docente y la
versión que finalmente pudo leer el autor de la entrega.

### V. Arranque local sin credenciales y recuperación del intento

Desde un clon limpio, `docker compose up --build` debe preparar la demostración con imágenes y
modelo de acceso público, sin `docker login`, cuentas del profesor, contraseñas, tokens ni
archivos `.env` con secretos. Debe haber tres servicios permanentes, `frontend`, `backend` e
`inferencia`. Un cuarto servicio temporal, `sqlite-init`, prepara la base, importa las fuentes
de los tres escenarios y termina. El backend comprueba que esa preparación concluyó con éxito.
Solo la interfaz publica un puerto en `localhost`. La API y Ollama se comunican por la red
interna de Compose. La base y los pesos se conservan en volúmenes persistentes.

Si faltan los pesos, inferencia debe descargarlos del registro oficial de Ollama, verificar la
identidad del modelo y completar una prueba de carga antes de anunciar que está disponible.
Ante un fallo, el arranque debe limitar los reintentos y mostrar el servicio, el motivo y cómo
reintentarlo. Mientras se recupera el motor, el estudiante debe poder retomar el mismo intento,
registrar mediciones, responder encuestas y confirmar su entrega. La evaluación puede quedar
pendiente.

**Evidencia de revisión.** En el equipo del profesor, arrancar con volúmenes vacíos y comprobar
los tres casos, la salida correcta de `sqlite-init`, la respuesta del modelo y el único puerto
publicado. Repetir el arranque con un intento guardado. Provocar un fallo de carga y comprobar
el mensaje de recuperación, el fin de los reintentos y la reanudación del mismo intento sin
duplicar casos, respuestas ni entregas.

### VI. Alcance del módulo y capacidad demostrada

Quedan fuera del módulo 01 el acceso institucional real a SIRCLE, los frameworks de frontend,
una segunda base de datos, los modelos adicionales y los servicios en la nube. Toda ampliación
debe identificar el problema observado, la alternativa más simple y la prueba que justificaría
el cambio.

La capacidad para cien estudiantes es una **meta de ensayo**. Deben medirse por separado
**100 sesiones abiertas** con una mezcla declarada de lectura, medición, chat y entrega, y una
ráfaga de **100 preguntas simultáneas**. El docente y el equipo deben acordar el umbral de
aceptación antes de atribuir esa capacidad a un equipo concreto.

**Evidencia de revisión.** Para cada ensayo, informar hardware, modelo y cuantización, longitud
del contexto, inferencias realmente concurrentes, percentiles 50 y 95 de espera y respuesta,
RAM/VRAM, errores, esperas de escritura en SQLite y recuperación. La conclusión debe identificar
qué perfil alcanzó qué umbral y en qué equipo.

## Decisiones clínicas y versiones de caso

Si el HTML y el caso original difieren, se deben registrar ambas fuentes, sus valores, la fase
y la decisión docente con su motivo. En LR, el triaje original consigna **148/85 mmHg y pulso
de 102/min**. El HTML muestra **128/78 mmHg y 112/min**. La elección queda pendiente de
aprobación docente. Solo la versión aprobada podrá alimentar la ficha, el instrumento y la
rúbrica de un intento. Una corrección posterior crea una nueva versión y conserva la que
sustenta cada entrega previa. [Diseño del módulo 01](../../docs/modulo-01/diseno.md)

## Revisión en Spec Kit

La aplicación parte de cero. Cada historia de la especificación debe enlazar su caso de uso
y una prueba observable. En conjunto, las pruebas deben cubrir el recorrido de LR y el aislamiento
entre estudiantes y agentes. Al revisar el plan, se debe identificar qué módulo responde por cada regla y qué evidencia
permitirá comprobarla. El código, los Dockerfiles, las migraciones y las tareas de implementación
corresponden a sus etapas posteriores. [Materiales del repositorio](../../README.md) · [Instrumento pre/post y prueba de aceptación](../../docs/modulo-01/instrumento-pre-post.md)

## Gobierno y enmiendas

1. Quien propone un cambio identifica el principio afectado, el problema observado, la
   alternativa más simple y la prueba que permitiría decidir. Registra el efecto sobre los
   intentos en curso y las entregas anteriores cuando corresponda.
2. El responsable del proyecto revisa la evidencia y registra la decisión, su motivo, la fecha
   y quién la aprobó. Los cambios de contenido clínico, instrumento o rúbrica requieren además
   la aprobación del docente antes de usarse en una actividad.
3. Al aprobar una enmienda, se actualizan esta constitución y su historial en el repositorio y
   se identifican las especificaciones o planes afectados. La versión mayor cambia al retirar
   o redefinir una regla de forma incompatible, la menor al añadir o ampliar una regla y el
   parche al aclarar la redacción sin modificar su alcance.

**Versión** 1.0.0 · **Ratificación inicial** 2026-11-25 · **Última enmienda** 2026-11-25
