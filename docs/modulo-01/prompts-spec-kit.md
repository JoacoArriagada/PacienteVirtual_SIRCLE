# Spec Kit · Módulo 01 de Paciente Virtual SIRCLE

Usar estos bloques por separado, revisando el resultado antes de la etapa siguiente. El diseño y los insumos se leen desde la raíz del repositorio. [Diseño acordado para discusión](diseno.md) · [Cuestionario y claves docentes](instrumento-pre-post.md) · [Materiales del repositorio](../../README.md)

En Codex, la invocación de una habilidad de Spec Kit lleva `$speckit-`. En una futura sesión la inicialización para el repositorio existente sería `specify init --here --integration codex` desde `feature/prototipo`. La constitución rige el proyecto. Después se especifica el módulo, se aclaran las decisiones pendientes, se planifica y se definen las tareas. La implementación y la verificación final se reservan para la etapa de desarrollo. [Integración oficial de Codex](https://github.github.io/spec-kit/integrations/codex.html) · [Flujo oficial de Spec Kit](https://github.github.io/spec-kit/quickstart.html)

## 1. Constitución

```text
$speckit-constitution

Redacta en español los principios de ingeniería y docencia del repositorio PacienteVirtual_SIRCLE. Exige que las fuentes, los casos y las referencias necesarios se puedan recuperar desde un clon de la rama de trabajo. Distingue esos materiales de los requisitos externos de ejecución y de los datos generados en un intento. Establece arquitectura por funcionalidades con frontend/ y backend/ independientes y contrato explícito entre ambos. Describe cómo se verifica fidelidad visual, responsabilidad única, recuperación tras fallos y revisión de cambios de contrato.

Establece agentes independientes de LR, Nannu, Dan Sparks y evaluación, con fuentes autorizadas, memoria por intento y decisiones trazables. El docente aprueba el contenido y la rúbrica y puede revisar la evidencia y publicar la devolución. La medición pre/post pertenece a la evaluación educativa del programa y debe poder omitirse sin penalizar la actividad clínica. Define qué constituye una excepción, quién la puede aprobar y qué prueba demuestra su efecto. Limita el resultado a la constitución.
```

## 2. Especificación funcional del módulo 01

```text
$speckit-specify

Especifica en español el módulo 01 de Paciente Virtual SIRCLE para que un profesor y dos desarrolladores puedan acordar qué experiencia se va a demostrar. Parte de docs/modulo-01/diseno.md, del instrumento de evaluación pre y post en esa carpeta y de los requisitos institucionales en docs/requisitos. Usa el HTML y las láminas de docs/referencias para identificar comportamientos. Usa los tres escenarios en datos/medsp1000, las adaptaciones en datos/adaptaciones y los elementos visuales en recursos-audiovisuales. Un profesor que clone la rama debe poder revisar los materiales del proyecto. Una discrepancia entre la fuente clínica y la interfaz se registra con procedencia, impacto en el aprendizaje y persona que debe resolverla.

La experiencia principal empieza cuando una estudiante entra con una identidad local de demostración y selecciona la actividad de LR. Antes de mostrar información clínica, el sistema ofrece una evaluación inicial de cinco preguntas. Tres miden autoeficacia, calma y preparación percibida y dos solicitan una respuesta objetiva sobre cronología o interpretación de evidencia. Explica cómo se valida y guarda la respuesta, cómo se registra una omisión voluntaria y qué impide completar el pre después de conocer el caso. La estudiante puede consultar la introducción, comenzar su intento o usar «En curso» y «Continuar» para reanudarlo. Retomar debe conservar conversación, hallazgos y estado del cuestionario.

Reproduce la interacción del HTML de referencia en selección, entrevista, exploración, mediciones, evidencia y entrega. Especifica qué debe coincidir visualmente y qué gesto o transición demuestra esa coincidencia. Conserva cabecera, tipografía, paleta, iconos, expresiones y encuadres. Incluye presión arterial, pulso, respiración, temperatura, saturación, peso, inspección, palpación, auscultación, brazo y ampliación. La estudiante abre ficha, indicaciones y ayuda, solicita un examen adicional y distingue una orden pendiente de un informe disponible. Vincula una medida o turno concreto a la hipótesis y guarda sus apuntes. Separa el resultado de un instrumento, lo que dice el paciente y lo que el estudiante interpreta.

Diseña tres agentes de paciente independientes, uno para LR, otro para Nannu, a quien el dataset llama Nanu, y otro para Dan Sparks. El cuarto agente evalúa una entrega confirmada. Cada agente tiene identidad fija, instrucciones, fuentes, operaciones permitidas y límites de acceso propios. La memoria del estudiante debe reconstruirse solo a partir del intento autorizado. Dos alumnos que entrevistan al mismo paciente nunca deben recibir datos del otro. El uso compartido de los pesos de Qwen conserva el aislamiento de las conversaciones. Incluye una prueba concurrente con dos estudiantes en LR y otra alternando los tres pacientes. Preguntar por el inicio del cuadro debe recuperar la cronología de cada guion. En Nannu distingue las declaraciones del familiar. En Dan identifica los informes que siguen pendientes.

Una pregunta sugerida puede usarse literalmente o editarse. Al editarla se atiende al significado del texto enviado. Para preguntas libres, describe cómo observa la estudiante las diferencias entre una respuesta sustentada, desconocimiento documentado, falta de información y necesidad de aclaración. La voz de la paciente se conserva ante preguntas que corresponden al docente. Traduce las ocho capas del diseño en conductas observables. Explica cómo verifica un revisor la procedencia de la respuesta, la separación entre pacientes, la validez de una acción y el comportamiento ante saturación o caída del motor.

La estudiante entrega una hipótesis con evidencias. El sistema confirma una sola entrega y se puede responder u omitir el post antes de ver claves o devolución. Se repiten las tres afirmaciones de percepción con idénticos anclajes y se presentan dos ejercicios equivalentes. Se añaden dos ítems de transferencia percibida e identificación de brechas. Conserva en el registro el estudiante, actividad, intento, momento y versión del instrumento. Especifica el reporte de pares completos, cambios por ítem, porcentaje objetivo y faltantes. Una repetición del envío debe producir el mismo resultado. Distingue rendimiento del estudiante, confianza percibida y efecto atribuible al programa.

El evaluador abre la entrega congelada y la rúbrica aprobada para el caso. Produce un borrador por criterio con citas que conducen a un turno, una medición o una frase de la entrega. Contrasta una acción registrada con una intención escrita. El docente revisa, rectifica con motivo y publica. La estudiante solo consulta su devolución publicada. Si el motor falla, su entrega permanece recibida y el borrador se puede producir más tarde. Exige que la reanudación y el reinicio conserven tanto la atención como la medición educativa.

Devuelve historias priorizadas, escenarios Dado–Cuando–Entonces, trazabilidad CU/RF/PR y decisiones pedagógicas pendientes. Incluye CU09, RF11 y PR15 en la primera iteración. Declara LR como primer recorrido completo y las condiciones para habilitar los otros dos pacientes. Distingue la prueba en el equipo del profesor, la referencia de veinte usuarios del documento institucional y el objetivo universitario de cien estudiantes simultáneos. Indica qué medir si cien personas están conectadas y qué sucede si todas preguntan al mismo tiempo. La capacidad, el triaje de LR y la equivalencia de los cuestionarios deben verificarse antes de formular conclusiones. Entrega la especificación para discutirla con el profesor.
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
