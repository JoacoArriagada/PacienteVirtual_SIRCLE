# Spec Kit · Módulo 01 de Paciente Virtual SIRCLE

Usar estos bloques por separado, revisando el resultado antes de la etapa siguiente. El diseño y los insumos se leen desde la raíz del repositorio. [Diseño acordado para discusión](diseno.md) · [Cuestionario y claves docentes](instrumento-pre-post.md) · [Materiales del repositorio](../../README.md)

En Codex, la invocación de una habilidad de Spec Kit lleva `$speckit-`. En una futura sesión la inicialización para el repositorio existente sería `specify init --here --integration codex` desde `feature/prototipo`. La constitución rige el proyecto. Después se especifica el módulo, se aclaran las decisiones pendientes, se planifica y se definen las tareas. La implementación y la verificación final se reservan para la etapa de desarrollo. [Integración oficial de Codex](https://github.github.io/spec-kit/integrations/codex.html) · [Flujo oficial de Spec Kit](https://github.github.io/spec-kit/quickstart.html)

## 1. Constitución

```text
$speckit-constitution

Redacta en español la constitución de Paciente Virtual SIRCLE en .specify/memory/constitution.md. El software comienza desde cero. El repositorio ya contiene los requisitos, el HTML de referencia, los recursos visuales y los casos clínicos. Lee primero README.md, docs/modulo-01/diseno.md y docs/modulo-01/instrumento-pre-post.md. Usa esos acuerdos para fijar las reglas que guiarán la especificación y la implementación. El entregable de esta etapa es la constitución. Detente al terminarla.

1. Propósito y alcance. Define como mínimo demostrable un recorrido completo con LR. Un estudiante entra con una identidad ficticia, responde u omite el cuestionario previo, entrevista a la paciente, realiza una medición, vincula la evidencia a una hipótesis y entrega. Después puede responder u omitir el cuestionario posterior antes de consultar la devolución publicada por el docente. El instrumento tendrá cinco preguntas antes y siete después. Sus resultados se registran por separado de la nota clínica. Este recorrido orienta las decisiones de arquitectura del módulo 01.

2. Arquitectura y lenguajes. Establece un monorrepositorio con frontend/ y backend/ en la raíz. Cada aplicación debe poder instalarse, ejecutarse y probarse por separado. El frontend usará HTML, CSS y TypeScript. El backend usará Rust con Axum. Se comunicarán mediante una API REST con JSON. Organiza cada aplicación por funcionalidades, como entrevista, valoración, encuestas y devolución. Por ejemplo, el gesto de colocar el brazalete pertenece a valoración en el frontend. Validar la medición y guardar su resultado pertenece a valoración en el backend. Crea una abstracción compartida cuando dos módulos necesiten el mismo comportamiento.

3. Reutilización y fidelidad. Conserva el HTML original como referencia de comparación. Reutiliza del repositorio la tipografía, la paleta, los iconos, las expresiones y los instrumentos. Comprueba la equivalencia visual y funcional reproduciendo un mismo gesto en ambas interfaces. El navegador recibirá solo la información visible para el estudiante. Los hechos reservados, las claves de las preguntas y las rúbricas estarán bajo control del backend. Si el HTML contradice al caso original, registra la discrepancia y deja la decisión clínica al docente.

4. Agentes y persistencia. Define cuatro trabajadores independientes, uno para LR, otro para Nannu, otro para Dan Sparks y otro para evaluación. Cada trabajador tiene identidad, fuentes y operaciones autorizadas propias. Pueden compartir los pesos de qwen3.5:4b mediante Ollama local. SQLite será la única base de datos. Guardará los tres escenarios con sus fuentes y versiones, además de los intentos, turnos, mediciones, encuestas y entregas. La memoria se recupera por estudiante, intento y agente. Dos estudiantes que entrevisten a LR deben conservar historias separadas. El evaluador recibe una entrega confirmada y propone observaciones vinculadas a evidencias. El docente puede corregirlas y decide su publicación.

5. Arranque local sin credenciales. El futuro punto de entrada será docker compose up --build desde un clon limpio. Define tres servicios permanentes, frontend, backend e inferencia, y un cuarto servicio temporal, sqlite-init, que prepara la base, importa los casos y termina. Las imágenes y el modelo serán accesibles públicamente. El arranque debe funcionar sin docker login, cuentas del profesor, contraseñas, tokens ni archivos .env con secretos. Publica la interfaz en localhost y mantén los servicios internos en la red de Compose. Conserva la base y los pesos en volúmenes. Si falta el modelo, contempla su descarga oficial y verificación. Si la carga falla, exige reintentos limitados y un mensaje comprensible que permita recuperar el servicio sin perder el intento.

6. Límites y gobierno. Para el módulo 01 quedan fuera el acceso institucional real a SIRCLE, un framework de frontend, una segunda base de datos, modelos adicionales y servicios en la nube. Cualquier ampliación debe identificar el problema observado, la alternativa más simple y la prueba que justificaría el cambio. La capacidad para cien estudiantes es una meta de ensayo. Distingue cien sesiones abiertas de cien preguntas simultáneas y exige mediciones antes de afirmar que el equipo las soporta.

Formula cada principio como una regla verificable e indica qué evidencia permitiría revisarlo. Cierra con un procedimiento breve para aprobar cambios y registrar su motivo. Reserva el código, los Dockerfiles, las migraciones y las tareas de implementación para sus etapas de Spec Kit.
```

## 2. Especificación funcional del módulo 01

```text
$speckit-specify

Redacta en español la especificación funcional del módulo 01 de Paciente Virtual SIRCLE. Lee .specify/memory/constitution.md, docs/modulo-01/diseno.md, el instrumento pre/post y el documento de requisitos en docs/requisitos/. Trabaja con los materiales del repositorio. Produce la especificación y su lista de comprobación. Describe los hitos en el orden en que deberán implementarse. La creación de directorios de aplicación, la copia de trabajo del HTML y la escritura de código corresponden a la implementación posterior.

Resuelve explícitamente dos ambigüedades. Primera: el recorrido completo de LR es el mínimo demostrable. Cada decisión debe vincularse a ese recorrido o a un requisito aprobado del módulo, como atribuir al acompañante de Nannu lo que este declara. Registra esta precisión respecto del principio I para su revisión constitucional, sin modificar la constitución desde Specify. Segunda: distingue tres comportamientos de entrevista. Ante «¿y eso?» sin referente claro, la paciente pide aclaración. Si a LR le preguntan por antecedentes familiares, expresa que los desconoce conforme a su guion. Si le preguntan por un dato que la fuente omite, el sistema indica que esa información no está disponible en el escenario. Nunca convierte la ausencia de registro en una respuesta clínica negativa. Exige una prueba independiente por situación.

Primer hito: establecer la estructura mínima. El futuro monorrepositorio tendrá frontend/ y backend/ en la raíz, instalables, ejecutables y comprobables por separado. Agrupa acceso, intentos, entrevista, valoración, exámenes, razonamiento, encuestas y devolución por funcionalidad. Sitúa los trabajadores en backend/src/agentes/, las migraciones en backend/migrations/ y las fuentes versionadas en datos/. HTML, CSS y TypeScript, Rust con Axum, REST con JSON, SQLite y Ollama con qwen3.5:4b son decisiones ya acordadas. Remite sus detalles técnicos al plan. Como prueba de preparación, el frontend debe poder demostrar una medición con datos de ejemplo y el backend debe poder registrar esa operación sin navegador. SQLite debe importar los 14 documentos de los tres escenarios y conservar casos e intentos al repetir el arranque.

Segundo hito: migrar la interfaz por extracción. Parte de una copia exacta de docs/referencias/html-semana5/Paciente_Virtual.html y de su carpeta recursos/. Conserva el original intacto para comparar. En la copia, mantén el marcado, los selectores y los gestos existentes siempre que sea posible. Extrae estilos a CSS y traslada el comportamiento a TypeScript por funcionalidad. Aplica responsabilidad única: el control del brazalete muestra el gesto, el cliente envía la operación y el backend valida y registra la medición. Introduce componentes compartidos cuando exista reutilización concreta. Comprueba la paridad a igual resolución en preparación, colocación, lectura e histórico, incluyendo imágenes, unidades y navegación por teclado. Excluye del paquete público las claves, rúbricas y hechos clínicos reservados.

Tercer hito: habilitar los trabajadores independientes de LR, Nannu, Dan Sparks y evaluación. Para cada una de las ocho capas, documenta su responsabilidad y un resultado comprobable:

1. Rol, objetivo y modelo. Cada trabajador conserva su identidad al compartir el motor.
2. Fuentes, recuperación y memoria. Recupera hechos autorizados por caso, versión, fase, hablante e intento.
3. Herramientas y MCP. Una medición exige una acción registrada. Bastan operaciones internas para esta iteración.
4. Identidad y acceso. Dos estudiantes que entrevistan a LR recuperan historiales distintos.
5. Controles de entrada, contexto y salida. Valida el hecho seleccionado y trata una sugerencia editada como pregunta libre.
6. Caché. Resuelve sugerencias exactas desde la ficha aprobada y evita reutilizar conversaciones entre intentos.
7. Enrutamiento. Distingue entrevista, instrumento y evaluación. Si falla Qwen, conserva avances y entregas pendientes de evaluación.
8. Observabilidad y regresión. Permite rastrear una respuesta hasta su fuente y detectar cruces al alternar pacientes.

Cuarto hito: demostrar el recorrido completo. El estudiante selecciona LR, responde u omite cinco preguntas antes de conocer el caso, abre la introducción y puede retomar desde «En curso». Entrevista mediante texto libre o sugerencias. Usa presión, pulso, respiración, temperatura, saturación, peso, inspección, palpación, auscultación, vista del brazo y ampliación. Consulta ficha, indicaciones y ayuda, solicita un examen adicional, guarda apuntes y vincula evidencias a su hipótesis. Tras confirmar la entrega puede responder u omitir siete preguntas antes de ver la devolución. El evaluador cita evidencias del intento. El docente rectifica y publica. La encuesta queda separada de la nota clínica.

Define la aceptación desde un clon limpio mediante Docker Compose sin credenciales. Frontend, backend e inferencia permanecen activos. sqlite-init prepara la base y termina. Comprueba persistencia tras reiniciar, reintentos sin duplicados y recuperación si falta el modelo. Distingue la demostración en el equipo del profesor de los ensayos de cien sesiones abiertas y cien preguntas simultáneas.

Entrega historias priorizadas y escenarios Dado–Cuando–Entonces vinculados a los CU, RF y PR existentes, incluido CU09/RF11/PR15. Señala el triaje de LR como decisión docente pendiente. Cierra con los criterios que permiten pasar de cada hito al siguiente y las decisiones que requieren aprobación.
```

## 3. Aclaración

```text
$speckit-clarify

Prioriza decisiones que modifican el aprendizaje o la evidencia. Confirma con el docente el triaje de LR, la fase evaluable, la prueba adicional habilitada y su condición de disponibilidad. Revisa las formas A/B del instrumento, el momento del post, las claves y qué se puede acreditar en la rúbrica. Para dimensionamiento aclara el equipo disponible, el escenario de uso de 100 estudiantes y la espera tolerable. Registra qué está resuelto y qué queda abierto. Mantén los materiales ausentes del dataset identificados.
```

## 4. Plan técnico

```text
$speckit-plan

Diseña un monorrepositorio con frontend/ y backend/ en la raíz, cada uno instalable, ejecutable y verificable por separado. El frontend usa HTML, CSS y TypeScript con módulos por funcionalidad. El HTML archivado y sus recursos son la referencia de paridad 1:1. El backend usa Rust con Axum y módulos por funcionalidad que agrupan manejadores, DTO, reglas y persistencia. Justifica los elementos compartidos y fija el contrato REST, control de acceso, errores, claves de idempotencia y pruebas entre aplicaciones.

Define cuatro trabajadores independientes en procesos separados, sus canales, permisos y fallos recuperables. Diseña memoria durable por estudiante, intento, agente, caso, versión, fase y hablante. El modelo es qwen3.5:4b servido por Ollama. Describe cómo se construye y descarta cada contexto. Planifica la validación de hechos, colas acotadas y un ensayo de interferencia y de carga de cien sesiones, incluyendo una ráfaga de peticiones. Mide memoria y latencia antes de definir réplicas. Las evoluciones MCP, caché semántica u otros motores deben responder a una necesidad observada.

Describe cómo incorporar a SQLite las catorce fuentes de los tres escenarios, sus roles, versiones y huellas, y cómo aprobar hechos y rúbricas. Especifica migración y semilla repetibles y protección de intentos existentes. Relaciona eventos y entregas con el cuestionario pre/post y su versión. Produce un diagrama de Compose con cuatro servicios definidos: frontend, backend, ia y sqlite-init. El cuarto prepara la base y termina. Tres quedan activos. El backend accede directamente al archivo en el volumen. Incluye validaciones de archivos, instalación inicial de pesos si faltan, verificación y reintentos acotados de Ollama, modo degradado, mensajes accionables y recuperación tras reinicio. Identifica los requisitos del host y las condiciones de una presentación sin red. Planifica la aceptación desde un clon limpio.
```

## 5. Tareas

```text
$speckit-tasks

Ordena las tareas por recorridos demostrables y responsable de frontend, backend o contenido docente. Primero verifica que el clon contenga HTML, fuentes, activos y licencias. Prepara migraciones y semilla SQLite de los tres pacientes y define contratos que permitan trabajo independiente. Entrega después acceso y pre, reanudación y entrevista de LR, medición con paridad al HTML, orden adicional y registro de evidencia, hipótesis y entrega, post y devolución revisable. Incluye los otros tres trabajadores, atribución del familiar, aislamiento entre intentos y evaluación sin acceso a casos ajenos. Asocia a cada incremento una prueba funcional y su CU. Añade un clon limpio, dos arranques consecutivos, una caída de inferencia, restauración de datos y ensayos de carga en la estación y con la cohorte. Explicita las dependencias entre tareas y el material aún pendiente de revisión.
```

## 6. Comprobación de coherencia

```text
$speckit-analyze

Contrasta constitución, especificación, plan y tareas. Comprueba que frontend y backend son independientes, que la semilla importa las fuentes de los tres pacientes en SQLite, que los cuatro agentes tienen límites y memoria comprobables, y que la evaluación educativa está dentro del recorrido. Busca inconsistencias entre número de servicios definidos y persistentes, y entre 100 sesiones y 100 inferencias concurrentes. Señala cada historia sin prueba, requisito sin responsable y dato clínico todavía sin aprobar.
```

Cuando el equipo autorice el desarrollo, `$speckit-implement` ejecutará las tareas acordadas y `$speckit-converge` permitirá cotejar los resultados con el plan y la especificación. [Guía de Spec Kit](https://github.github.io/spec-kit/quickstart.html)
