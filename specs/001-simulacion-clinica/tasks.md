# Tareas del módulo 01 · Paciente Virtual SIRCLE

**Rama** `feature/prototipo` · **Módulo** `001-simulacion-clinica` · **Fecha** 2026-11-25

Las tareas T001–T080 conservan los cuatro hitos del plan. T081–T084 corresponden al estudio posterior de capacidad. Todas las casillas y evidencias de ejecución están **pendientes**. La primera edición prevista de la especificación figura en T001. [Constitución vigente](../../.specify/memory/constitution.md) · [Especificación aclarada](spec.md) · [Plan técnico revisado](plan.md)

El frontend usará HTML, CSS y TypeScript sin framework. El backend usará Rust y Axum con REST/JSON, SQLite como única base y un solo motor Ollama local. LR tendrá el recorrido completo. Nannu y Dan Sparks se usarán para comprobar identidad, atribución y aislamiento. La precisión del principio I sobre las decisiones justificadas por otro requisito aprobado del módulo sigue el procedimiento de enmienda. [Contrato HTTP](contracts/api.md) · [Matriz de agentes](contracts/agentes.md) · [Modelo de datos](data-model.md) · [Decisiones de investigación](research.md) · [Guía de aceptación](quickstart.md)

| Tramo | Tareas | Qué permite comprobar el cierre | Registro |
| --- | --- | --- | --- |
| Hito 1 | T001–T020 · 20 | Conversación libre por HTTP, medición y persistencia con material de ensayo. | [Índice H1](#evidencias-del-hito-1) |
| Hito 2 | T021–T038 · 18 | El chat usa la ruta real de H1 y los gestos coinciden con la referencia. | [Índice H2](#evidencias-del-hito-2) |
| Hito 3 | T039–T065 · 27 | Cuatro trabajadores, 32 núcleos, calibración, aislamiento y recuperación local. | [Índice H3](#evidencias-del-hito-3) |
| Hito 4 | T066–T080 · 15 | El profesor completa y revisa la práctica con las versiones aprobadas en su computador. | [Índice H4](#evidencias-del-hito-4) |
| Capacidad posterior | T081–T084 · 4 | Tres perfiles medidos y un dictamen por perfil tras ratificar umbrales. | [Índice de capacidad](#evidencias-de-capacidad) |

## Cómo ejecutar y documentar una tarea

Las rutas de trabajo parten de la raíz del repositorio. Los archivos de implementación y evidencia son **previstos**. Las etiquetas US1–US8 identifican la historia principal. Las cinco tareas de preparación común citan el requisito aplicable. Cada lista de dependencias contiene prerrequisitos directos. La marca `[P]` se reserva para tareas que ya pueden iniciarse en paralelo y escriben archivos distintos. [Dependencias entre hitos e historias](#dependencias-y-paralelismo)

Al implementar, abrir el registro indicado en el índice y consignar **ID de tarea, revisión Git, comando o pasos reproducibles, resultado esperado, resultado observado y ubicación del respaldo**. Añadir fecha, identidad ficticia o rol, IDs de intento y operación cuando correspondan. Para inferencia o rendimiento, consignar equipo, modelo, digest, estado del motor y del modelo en memoria. Para comparación visual, anotar navegador, resolución, zoom, escala y estado del gesto en ambas capturas. Un respaldo compartido puede citarse desde varias tareas indicando la fila, el ID o la captura que verifica cada una. Las trazas y constancias docentes se conservarán bajo acceso restringido, fuera de la compilación y de las respuestas al estudiante.

Los enlaces de datos usan la notación descriptiva `Entidad.campo` de las tablas del modelo. Los códigos E identifican escenarios, FR requisitos y SC resultados medibles. E06 tiene criterio propio para el texto libre. La medición individual en frío/caliente y el perfil exploratorio de 20 usuarios se detallan en los supuestos de la especificación. [Escenarios](spec.md#escenarios-de-usuario-y-pruebas) · [Requisitos](spec.md#requisitos-funcionales) · [Resultados medibles](spec.md#resultados-medibles) · [Supuestos de medición](spec.md#supuestos-dependencias-y-límites)

## Pruebas independientes por historia

| Historia y prioridad | Tareas principales | Prueba independiente prevista |
| --- | ---: | --- |
| [US1 · Completar y retomar la práctica de LR](spec.md#historia-1--completar-y-retomar-la-práctica-de-lr-p1) · P1 | 5 | Reabrir el mismo intento y completar LR hasta leer la revisión publicada, con trazabilidad de la hipótesis principal y la alternativa. |
| [US2 · Entrevistar a LR e interpretar los límites de su relato](spec.md#historia-2--entrevistar-a-lr-e-interpretar-los-límites-de-su-relato-p1) · P1 | 14 | Probar una pregunta inédita con Qwen y, en diálogos independientes, ambigüedad, desconocimiento familiar y dato ausente. |
| [US3 · Explorar, pedir ayuda y fundamentar una hipótesis](spec.md#historia-3--explorar-pedir-ayuda-y-fundamentar-una-hipótesis-p1) · P1 | 18 | Comparar el brazalete y su histórico, confirmar el ECG y recuperar las hipótesis con evidencias del intento. |
| [US4 · Entregar y obtener una devolución revisada por el docente](spec.md#historia-4--entregar-y-obtener-una-devolución-revisada-por-el-docente-p1) · P1 | 12 | Entregar dos trabajos E12, inspeccionar créditos y citas, rectificar y verificar qué versión puede leer el titular. |
| [US5 · Interpretar el pre y el post por separado de la nota clínica](spec.md#historia-5--interpretar-el-pre-y-el-post-por-separado-de-la-nota-clínica-p1) · P1 | 4 | Obtener dos pre, dos post y un par en E14 y comprobar ambas secuencias en una serie distinta para E15. |
| [US6 · Conservar la identidad del paciente y del hablante](spec.md#historia-6--conservar-la-identidad-del-paciente-y-del-hablante-p1) · P1 | 11 | Solapar dos preguntas a LR, separar sus historias y alternar Nannu y Dan manteniendo la atribución a cada hablante. |
| [US7 · Preparar y recuperar la demostración local](spec.md#historia-7--preparar-y-recuperar-la-demostración-local-p1) · P1 | 11 | Arrancar desde un clon limpio, restaurar el intento y recuperar exactamente una evaluación tras E22. |
| [US8 · Medir la capacidad del equipo para una cohorte](spec.md#historia-8--medir-la-capacidad-del-equipo-para-una-cohorte-p2) · P2 | 4 | Informar por separado E23 y E24 en el equipo identificado y contrastar cada perfil con su propio umbral en un ensayo posterior. |

A estas historias se suman las 5 tareas comunes T001–T005. El primer avance técnico se cierra en T020. El recorrido clínico completo de US1 se acepta en T080.

## Hito 1 · Demostrar una medición y una conversación real con LR

El ensayo puede arrancar el backend y Ollama de forma independiente. Usar una identidad ficticia y una ficha de prueba identificada por versión. Conservar el intento de demostración para reabrirlo desde el chat en el hito 2. [Ensayo de pregunta libre](quickstart.md#conversación-libre-con-lr-por-http)

### Preparación común del proyecto

- [ ] T001 Alinear las dos filas de «Criterios de paso entre hitos» con el plan revisado. **Ruta** `specs/001-simulacion-clinica/spec.md`.

  **Depende de** ninguna tarea previa. **Referencias** FR-004, FR-012 · E06, E07, E19, E20 · SC-003, SC-006. **Cierre.** El cambio propuesto en la fila H1→H2 conserva la medición y la preparación repetible y exige la pregunta libre por HTTP con traza de Qwen. La fila H2→H3 conserva la paridad y exige repetir esa operación desde el chat. La revisión identifica las dos filas y su correspondencia con el plan. [Diferencias de las dos filas · pendiente](#ev-t001)

- [ ] T002 [P] Preparar el frontend HTML, CSS y TypeScript, con instalación, prueba, compilación y arranque propios. **Rutas** `frontend/package.json` · `frontend/tsconfig.json` · `frontend/index.html`.

  **Depende de** ninguna tarea previa. **Referencias** FR-005, FR-012 · E07, E19 · SC-003. **Cierre.** Desde frontend se completan `npm ci`, `npm test` y `npm run build`. Con el backend detenido, `npm run dev:ejemplo` sirve la pantalla de prueba en el navegador. [Comandos, códigos de salida y pantalla de arranque · pendiente](#ev-t002)

- [ ] T003 [P] Preparar el backend Rust con Axum y una prueba independiente del router HTTP/JSON. **Rutas** `backend/Cargo.toml` · `backend/src/app/router.rs` · `backend/src/bin/sircle.rs` · `backend/tests/router.rs`.

  **Depende de** ninguna tarea previa. **Referencias** FR-011 · E19. **Cierre.** Desde backend, `cargo build --locked` y `cargo test --locked` terminan correctamente. Una petición de la prueba al router devuelve el estado y el JSON esperados sin abrir el frontend ni requerir inferencia. El binario ofrece el modo `servir` para las peticiones HTTP directas del hito. [Compilación y respuesta del router · pendiente](#ev-t003)

- [ ] T004 Crear la migración inicial de escenarios, fuentes, revisiones y versiones clínicas en SQLite. **Ruta** `backend/migrations/0001_fuentes.sql`.

  **Depende de** T003. **Referencias** FR-010 · E20 · SC-006. **Cierre.** En una base vacía la migración se aplica una vez, las claves foráneas están activas y el alta de una revisión con fuente inexistente se rechaza. Consultar el esquema permite recorrer escenario → fuente → revisión → versión clínica. [Esquema y prueba de integridad referencial · pendiente](#ev-t004)

  **Datos** [`Escenario.identidad`, `Fuente.escenario`, `Revisión de fuente.fuente`, `Revisión de fuente.huella`, `Versión clínica.revisiones de fuente`](data-model.md#fuentes-y-versiones).

- [ ] T005 Importar los originales de los tres escenarios por ruta y rol. **Rutas** `backend/src/modules/escenarios/importacion.rs` · `backend/src/bin/sircle.rs` · `datos/medsp1000/`.

  **Depende de** T004. **Referencias** FR-010 · E20 · SC-006. **Cierre.** Desde backend, `cargo run --locked --bin sircle -- preparar --db target/demo.sqlite --fuentes ../datos` prepara la base para el modo `servir`. El inventario devuelve P01=4, P02=4 y P03=6, con contenido, ruta, rol y SHA-256 cotejados contra cada archivo. Rutas o roles distintos retienen identidades distintas aunque compartan bytes. [Catorce fuentes con sus identificadores y huellas · pendiente](#ev-t005)

  **Datos** [`Fuente.rol de origen`, `Fuente.ruta relativa`, `Fuente.contenido original`, `Fuente.SHA-256`, `Adaptación.pasaje`, `Medio.disponibilidad`](data-model.md#fuentes-y-versiones).

### Persistencia, sesión e intento de ensayo

- [ ] T006 [US7] Ensayar dos preparaciones idénticas y una importación inválida en una copia de prueba. **Ruta** `backend/tests/preparacion.rs`.

  **Depende de** T005. **Referencias** FR-010 · E20 · SC-006. **Cierre.** La segunda preparación devuelve los mismos tres casos, 14 fuentes, identificadores y versiones. Al retirar o invalidar una fuente en la copia, el preparador señala su ruta y la consulta posterior recupera íntegro el inventario válido anterior. [Inventarios antes y después y error identificado · pendiente](#ev-t006)

- [ ] T007 [US1] Habilitar la sesión local de estudiante ficticio y la actividad preasignada de ensayo. **Rutas** `backend/src/modules/acceso/sesiones.rs` · `backend/src/modules/acceso/actividades.rs` · `backend/migrations/0002_asignaciones.sql`.

  **Depende de** T003, T005. **Referencias** FR-001, FR-009 · E01, E13. **Cierre.** Después de `POST /api/sesiones`, `GET /api/actividades` devuelve únicamente la asignada al titular. Una segunda identidad no la puede abrir si carece de asignación. Antes del pre, la respuesta omite introducción, ficha y hechos. [Vistas autorizadas de dos identidades ficticias · pendiente](#ev-t007)

  **Datos** [`Asignación.actividad`, `Asignación.estudiante`, `Asignación.docente responsable`, `Identidad.rol`](data-model.md#actividad-e-intento).

- [ ] T008 [US1] Preparar un intento técnico de LR con el pre resuelto y versiones de ensayo identificadas. **Rutas** `backend/src/modules/intentos/intento.rs` · `backend/tests/fixtures/intento_lr.rs` · `backend/migrations/0003_intentos.sql`.

  **Depende de** T007. **Referencias** FR-001, FR-002, FR-010 · E01, E02, E20 · SC-001, SC-006. **Cierre.** Cerrar el pre mediante un estado de prueba. Salir y reabrir o reenviar el alta recupera el mismo identificador, estudiante, actividad, fase inicial y versiones de caso, rúbrica e instrumento. Se distingue de la actividad clínica formal. [Vista del intento antes y después de reabrir · pendiente](#ev-t008)

  **Datos** [`Intento.propietario`, `Intento.actividad`, `Intento.versión de caso`, `Intento.versión de rúbrica`, `Intento.versión de instrumento`, `Intento.estado clínico`, `Intento.revisión`](data-model.md#actividad-e-intento).

### Valoración independiente

- [ ] T009 [US3] Registrar en Axum una toma completa de presión y consultar su histórico por intento. **Rutas** `backend/src/modules/valoracion/mediciones.rs` · `backend/migrations/0004_mediciones.sql`.

  **Depende de** T008. **Referencias** FR-005 · E07, E19 · SC-003. **Cierre.** `POST /api/intentos/{id}/mediciones` devuelve identificador, valor de la versión de prueba, unidad mmHg y momento. La lectura aparece en el GET del mismo intento. Una toma incompleta o de fase no autorizada deja el recuento intacto. [Petición, lectura y consulta del histórico · pendiente](#ev-t009)

  **Datos** [`Medición.intento`, `Medición.instrumento`, `Medición.acción`, `Medición.fase`, `Medición.valor`, `Medición.unidad`, `Medición.momento`](data-model.md#actividad-e-intento).

- [ ] T010 [US3] Aplicar idempotencia a la medición y distinguir una toma nueva. **Rutas** `backend/src/modules/valoracion/mediciones.rs` · `backend/tests/valoracion.rs`.

  **Depende de** T009. **Referencias** FR-005 · E07, E19 · SC-003. **Cierre.** Con la misma clave y contenido, los dos acuses llevan el mismo ID y el histórico crece en una lectura. Con clave nueva, otra toma deliberada tiene otro ID y el histórico crece en una más. Reutilizar la primera clave con otro contenido da 409. Usar para esta serie un intento de prueba distinto del que T020 conservará para el chat. [Claves, acuses, IDs y recuentos de la prueba de valoración · pendiente](#ev-t010)

  **Datos** [`Operación.identidad`, `Operación.ámbito`, `Operación.clave`, `Operación.huella`](data-model.md#escrituras-caídas-y-reenvíos).

- [ ] T011 [US3] Representar el gesto del brazalete con respuestas JSON de ejemplo y el backend detenido. **Rutas** `frontend/src/modules/valoracion/brazalete.ts` · `frontend/tests/brazalete.test.ts`.

  **Depende de** T002. **Referencias** FR-005, FR-012 · E07, E19 · SC-003. **Cierre.** La pantalla permite preparar, colocar y tomar la presión y consultar la lectura. La secuencia de capturas mantiene el mismo encuadre. Registrar dos veces el resultado de ejemplo conserva una entrada. Se identifica la fuente de prueba. [Serie de capturas y registro de la sesión de ejemplo · pendiente](#ev-t011)

### Entrevista de LR por HTTP

- [ ] T012 [US2] Preparar candidatos de LR para la pregunta inicial y el desconocimiento familiar documentado. **Rutas** `backend/tests/fixtures/lr/hechos_iniciales.json` · `backend/src/modules/entrevista/hechos.rs` · `backend/migrations/0005_hechos.sql`.

  **Depende de** T005, T008. **Referencias** FR-004, FR-009 · E04, E06, E18 · SC-002. **Cierre.** Cada candidato incluye ID, P01, versión de ensayo, fase inicial, hablante LR, condición de revelación y pasaje de `sp_actor/Case A-Asthma.md`. El inicio remite a dos días y los antecedentes al pasaje de adopción. Ningún hecho de P02 o P03 integra este conjunto. [Candidatos y localización en la fuente de LR · pendiente](#ev-t012)

  **Datos** [`Hecho.contenido aprobado`, `Hecho.fuente`, `Hecho.pasaje`, `Hecho.hablante`, `Hecho.fase`, `Hecho.condiciones de revelación`](data-model.md#fuentes-y-versiones).

- [ ] T013 [US2] Conectar la pregunta libre del endpoint de turnos con Ollama local y `qwen3.5:4b`. **Rutas** `backend/src/modules/entrevista/turnos.rs` · `backend/src/modules/entrevista/ollama.rs`.

  **Depende de** T003, T008, T012. **Referencias** FR-004 · E06. **Cierre.** En el equipo de ensayo, dejar disponible Ollama con los pesos oficiales de `qwen3.5:4b` para la ejecución independiente. El adaptador envía texto libre y candidatos del intento. La respuesta efectiva del motor devuelve la selección solicitada. El registro vincula identificador de operación, inicio, fin, modelo y digest. Se conserva la selección devuelta por Qwen. [Petición y respuesta de una invocación real correlacionada · pendiente](#ev-t013)

- [ ] T014 [US2] Validar la selección de Qwen y la vigencia del contexto antes de confirmar el turno. **Rutas** `backend/src/agentes/lr/entrevista.rs` · `backend/tests/agente_lr.rs` · `backend/migrations/0006_turnos.sql`.

  **Depende de** T013. **Referencias** FR-004, FR-009 · E06, E18 · SC-002. **Cierre.** Un hecho permitido y pertinente puede guardarse. Al introducir candidatos de otro caso, fase o hablante, o al cerrar el intento mientras se responde, el servidor conserva el último turno confirmado y deja un motivo restringido. El estudiante recibe el estado correspondiente. [Casos válidos, rechazos y recuento de turnos · pendiente](#ev-t014)

  **Datos** [`Turno.estudiante`, `Turno.intento`, `Turno.agente`, `Turno.orden`, `Turno.hablante`, `Turno.pregunta`, `Turno.respuesta`, `Turno.tipo de respuesta`](data-model.md#actividad-e-intento).

- [ ] T015 [US2] Probar por HTTP una pregunta escrita por quien ensaya, distinta de todas las sugerencias literales. **Ruta** `backend/tests/entrevista_http.rs`.

  **Depende de** T001, T014. **Referencias** FR-004 · E06. **Cierre.** Con el pre cerrado, enviar a `POST /api/intentos/{id}/turnos` una paráfrasis inédita sobre el inicio, sin ID de sugerencia, por ejemplo «¿Cuándo empezaste a notar que te faltaba el aire?». LR responde en primera persona y sitúa el inicio hace dos días. El GET del intento devuelve ese turno. Ejecutable antes de la extracción visual y de los cuatro trabajadores. [Texto enviado, acuse y turno recuperado · pendiente](#ev-t015)

- [ ] T016 [US2] Exponer al revisor el recorrido del turno desde la pregunta al hecho seleccionado. **Rutas** `backend/src/shared/trazas.rs` · `backend/src/modules/entrevista/traza.rs` · `backend/migrations/0007_trazas.sql`.

  **Depende de** T015. **Referencias** FR-004, FR-009 · E06, E18 · SC-002. **Cierre.** La traza de T015 permite unir intento y turno con la misma operación enviada a Ollama, su respuesta, modelo/digest, fase, hablante, hecho, versión y pasaje. Consultarla con la identidad de estudiante deniega el detalle técnico. [Correlación del turno de T015 con la invocación y la fuente · pendiente](#ev-t016)

  **Datos** [`Turno.traza de revisión`, `Traza restringida.operación`, `Traza restringida.modelo`, `Traza restringida.digest`, `Traza restringida.inicio`, `Traza restringida.fin`, `Traza restringida.hecho seleccionado`](data-model.md#actividad-e-intento).

- [ ] T017 [US2] Detener la inferencia y enviar el mismo texto de T015 como operación nueva. **Ruta** `backend/tests/entrevista_indisponible.rs`.

  **Depende de** T016. **Referencias** FR-004, FR-011 · E06, E22. **Cierre.** Con una clave nueva, el backend devuelve 503 y una indicación de reintento. El historial antes y después tiene los mismos IDs y cero respuestas clínicas nuevas. La evidencia identifica el estado del motor y la clave fallida. [Respuesta 503 y comparación del historial · pendiente](#ev-t017)

- [ ] T018 [US2] Restablecer Qwen y reintentar la operación fallida con su misma clave. **Ruta** `backend/tests/entrevista_indisponible.rs`.

  **Depende de** T017. **Referencias** FR-004, FR-010 · E06, E22. **Cierre.** Después de una generación de comprobación, el reintento confirma un turno del intento original. Reenviar esa clave ya confirmada recupera el mismo ID. El recuento de pares confirmados para la operación fallida pasa de cero a uno. [Clave recuperada, confirmaciones y recuento final · pendiente](#ev-t018)

- [ ] T019 [US2] Medir la latencia HTTP de la pregunta en frío y en caliente con los pesos descargados. **Ruta** `backend/tests/entrevista_http.rs`.

  **Depende de** T018. **Referencias** FR-004 · E06. **Cierre.** Con operaciones nuevas, registrar desde el envío hasta la respuesta en ambos estados, modelo fuera de memoria y modelo residente. Identificar CPU, RAM, GPU si procede, contexto, modelo/digest y estación. Si es desarrollo, dejar registrada la repetición en el equipo docente en la primera oportunidad. Las dos duraciones son exploratorias. [Muestras por estado y equipo, con unidad de tiempo · pendiente](#ev-t019)

### Primer avance demostrable

- [ ] T020 [US7] Cerrar el primer avance recuperando la conversación y la medición de LR. **Ruta** `specs/001-simulacion-clinica/evidencias/h1/T020/acta.md`.

  **Depende de** T006, T010, T011, T016, T018, T019. **Referencias** FR-004, FR-005, FR-010, FR-011 · E06, E19, E20 · SC-003, SC-006. **Cierre.** El acta reúne las dos aplicaciones independientes, el gesto de ejemplo, los acuses de la misma toma y el turno con Qwen real. Tras repetir la preparación y reabrir, coinciden el ID de intento, el turno de T015 y el ID único de la toma reenviada, junto con los tres casos y 14 fuentes. Conservar este intento para T026. [Acta del primer avance e inventarios comparados · pendiente](#ev-t020)

## Hito 2 · Extraer el HTML y demostrar el recorrido desde la interfaz

Partir de T020 y de la copia privada verificada en T021. En la migración se conservan marcado, selectores, tipografía, paleta, iconos, expresiones e instrumentos. La comparación visual admite material de prueba. T026 exige una nueva invocación de Qwen. Las lecturas clínicas pertenecen a la versión habilitada por el backend. [Inventario del original](research.md#referencia-comprobada-en-el-clon) · [Gestos de los instrumentos](../../docs/referencias/interfaz/Funcionalidades_instrumentos.md)

### Copia y apariencia de Semana 5

- [ ] T021 [US3] Copiar íntegros el HTML de Semana 5 y su carpeta contigua de recursos al área privada del frontend. **Rutas** `docs/referencias/html-semana5/Paciente_Virtual.html` · `frontend/referencia/semana5/Paciente_Virtual.html` · `frontend/referencia/semana5/recursos/`.

  **Depende de** T020. **Referencias** FR-012 · E07, E10 · SC-003, SC-005. **Cierre.** Antes de editar la copia, ordenar ambos inventarios por ruta relativa y comparar SHA-256 sobre los bytes del mismo clon. Deben coincidir el HTML y los 28 recursos, con cero rutas faltantes, sobrantes o modificadas. El original conserva su huella. [Rutas y dos columnas de SHA-256 · pendiente](#ev-t021)

- [ ] T022 [US3] Extraer los estilos a CSS conservando marcado, selectores, Roboto, paleta y dimensiones. **Rutas** `frontend/index.html` · `frontend/src/app/semana5.css`.

  **Depende de** T021. **Referencias** FR-012 · E07 · SC-003. **Cierre.** Capturar la misma pantalla en original y extracción con navegador, zoom, resolución y escala fijos. Comparar selectores, tipografía, colores y dimensiones de los controles. Cada diferencia visible debe remitir a un cambio previsto en el plan o corregirse antes del cierre. [Par de capturas y estilos contrastados · pendiente](#ev-t022)

### Selección y reanudación

- [ ] T023 [US1] Migrar la selección de LR y «En curso → Continuar» a TypeScript. **Ruta** `frontend/src/modules/intentos/seleccion.ts`.

  **Depende de** T022, T008. **Referencias** FR-001, FR-010, FR-012 · E02, E07 · SC-001, SC-003. **Cierre.** Abrir el intento técnico, salir y continuar con el mismo estudiante. Coinciden el ID, la versión y los turnos previos. La pantalla de selección conserva los controles de referencia a igual resolución. [Capturas de selección y respuestas del mismo intento · pendiente](#ev-t023)

- [ ] T024 [US3] Migrar introducción, ficha, indicaciones y ayuda con visibilidad por fase. **Rutas** `frontend/src/modules/intentos/ficha.ts` · `backend/src/modules/intentos/vista.rs`.

  **Depende de** T023. **Referencias** FR-002, FR-006 · E08, E15 · SC-003, SC-004. **Cierre.** Con pre pendiente de ensayo, ni la vista ni su respuesta JSON muestran contenido clínico. Con pre de ensayo resuelto se abren la ficha y las indicaciones. El texto de ayuda identifica la orientación docente y queda distinguible de los hallazgos del estudiante. [Capturas y respuestas en cada fase · pendiente](#ev-t024)

### Chat y entrevista real

- [ ] T025 [US2] Migrar el desplegable de sugerencias y reconocer la edición de su texto. **Rutas** `frontend/src/modules/entrevista/sugerencias.ts` · `backend/src/shared/ficha_aprobada.rs`.

  **Depende de** T023, T014. **Referencias** FR-004 · E06. **Cierre.** «¿Desde cuándo comenzó?» intacta recupera el hecho de dos días desde la ficha, incluso con inferencia detenida. Editarla para preguntar por la familia activa la ruta libre. Con el motor disponible responde sobre la adopción y atribuye la voz a LR. [Capturas del texto intacto y editado, peticiones y rutas · pendiente](#ev-t025)

  **Datos** [`Turno.entrada literal o libre`](data-model.md#actividad-e-intento).

- [ ] T026 [US2] Conectar el chat al mismo endpoint de turnos verificado por HTTP en T015. **Ruta** `frontend/src/modules/entrevista/chat.ts`.

  **Depende de** T025, T016, T020. **Referencias** FR-004, FR-010, FR-012 · E02, E06, E07 · SC-001, SC-003. **Cierre.** Al reabrir el intento de T020 aparece el ID de turno de T015. Una nueva pregunta con palabras propias llega al mismo `POST /api/intentos/{id}/turnos` con clave nueva. La red del navegador y la traza del revisor enlazan un turno distinto, una nueva invocación real y el pasaje correcto. [Captura del chat, petición y correlación con Qwen · pendiente](#ev-t026)

### Valoración e instrumentos

- [ ] T027 [US3] Migrar la extremidad, su ampliación y la colocación del brazalete. **Rutas** `frontend/src/modules/valoracion/brazo.ts` · `frontend/src/modules/valoracion/brazalete.ts`.

  **Depende de** T022, T023, T011. **Referencias** FR-005, FR-012 · E07, E08 · SC-003. **Cierre.** En original y extracción se reconocen los mismos estados de preparación, brazo ampliado y brazalete colocado. La toma solo se habilita al completar los pasos de colocación y ajuste. Aportar un par de capturas por estado al mismo tamaño. [Pares de capturas de los estados intermedios · pendiente](#ev-t027)

- [ ] T028 [US3] Conectar la toma, el histórico y «Registrar en ficha» al contrato de valoración. **Rutas** `frontend/src/modules/valoracion/presion.ts` · `backend/src/modules/valoracion/ficha.rs`.

  **Depende de** T027, T010. **Referencias** FR-005, FR-012 · E07, E19 · SC-003. **Cierre.** Al cancelar antes de completar, el histórico crece en cero. Al completar, crece en una lectura. Dos `PUT` de registro sobre esa lectura dejan el mismo ID, una fila en el histórico y una referencia en la ficha. Contrastar el estado en pantalla con el GET del intento. [Secuencia visual, acuses y recuentos de ficha e histórico · pendiente](#ev-t028)

  **Datos** [`Medición.estado de registro`](data-model.md#actividad-e-intento).

- [ ] T029 [US3] Migrar pulso y conteo respiratorio y habilitar sus resultados por versión de prueba. **Rutas** `frontend/src/modules/valoracion/pulso.ts` · `frontend/src/modules/valoracion/respiracion.ts` · `backend/src/modules/valoracion/mediciones.rs`.

  **Depende de** T027, T009, T028. **Referencias** FR-005, FR-012 · E07, E08 · SC-003. **Cierre.** En dos ensayos independientes, el gesto de pulso produce su frecuencia y el conteo respiratorio conserva las pulsaciones del estudiante. Cada registro identifica instrumento, intento, origen, momento y unidad por minuto correspondiente. Comparar ambos gestos con el original. [Dos pares de capturas y sus lecturas identificadas · pendiente](#ev-t029)

- [ ] T030 [US3] Migrar termómetro frontal y sensor de saturación, con controles y registro independientes. **Rutas** `frontend/src/modules/valoracion/temperatura.ts` · `frontend/src/modules/valoracion/saturacion.ts` · `backend/src/modules/valoracion/mediciones.rs`.

  **Depende de** T022, T009. **Referencias** FR-005, FR-012 · E08 · SC-003. **Cierre.** Probar por separado la preparación y colocación del termómetro y del sensor. Registrar cada hallazgo habilitado una vez con origen, unidad y momento. El histórico del termómetro conserva solo sus tomas y el del pulsioxímetro solo las suyas. [Capturas comparables por instrumento y dos históricos · pendiente](#ev-t030)

- [ ] T031 [US3] Migrar la preparación de la báscula, la lectura y su registro. **Rutas** `frontend/src/modules/valoracion/peso.ts` · `backend/src/modules/valoracion/mediciones.rs`.

  **Depende de** T022, T009. **Referencias** FR-005, FR-012 · E08 · SC-003. **Cierre.** El mismo gesto de preparación que en el original habilita la toma. La medición de ejemplo muestra la unidad de peso de la versión, guarda un único ID y aparece en ficha con ese ID después de registrar. [Comparación visual y lectura con unidad · pendiente](#ev-t031)

- [ ] T032 [US3] Migrar inspección y palpación por zona y separar la acción de su hallazgo clínico. **Rutas** `frontend/src/modules/valoracion/inspeccion.ts` · `frontend/src/modules/valoracion/palpacion.ts` · `backend/src/modules/valoracion/hallazgos.rs`.

  **Depende de** T022, T009. **Referencias** FR-005, FR-012 · E08 · SC-003. **Cierre.** Para cada instrumento, comparar la selección de zona con la referencia y probar una zona habilitada y otra pendiente. Quedan identificados gesto, zona y momento. Solo la zona habilitada aporta un hallazgo citable de esa versión. [Pares de capturas por gesto y registros de las dos zonas · pendiente](#ev-t032)

- [ ] T033 [US3] Migrar la auscultación y reutilizar el audio autorizado de la referencia. **Rutas** `frontend/src/modules/valoracion/auscultacion.ts` · `backend/src/modules/valoracion/hallazgos.rs`.

  **Depende de** T022, T009. **Referencias** FR-005, FR-012 · E08 · SC-003. **Cierre.** Seleccionar el mismo punto en ambas interfaces y comprobar gesto, audio y resultado habilitado. Repetir con un hallazgo pendiente conserva el gesto, informa el estado y deja cero hallazgos clínicos nuevos. Consignar qué audio se oyó en cada caso. [Capturas por punto, comprobación auditiva y registro · pendiente](#ev-t033)

### Examen, apuntes e hipótesis

- [ ] T034 [US3] Conectar la orden confirmada del ECG inicial y la consulta de su informe. **Rutas** `frontend/src/modules/examenes/ecg.ts` · `backend/src/modules/examenes/ordenes.rs` · `backend/migrations/0008_ordenes.sql`.

  **Depende de** T024, T008. **Referencias** FR-006, FR-010 · E09 · SC-001, SC-003. **Cierre.** Antes de confirmar, la consulta del informe queda reservada. Al confirmar se libera el texto «taquicardia sinusal». Reenviar la confirmación conserva el mismo ID, una orden y el mismo informe citable del intento. La prueba se resuelve con el texto. [Consultas antes y después y acuses de una orden · pendiente](#ev-t034)

  **Datos** [`Orden.intento`, `Orden.identificador`, `Informe.resultado`, `Informe.fuente`](data-model.md#actividad-e-intento).

- [ ] T035 [US3] Conectar apuntes y evidencias, conservando autor, origen y referencias del propio intento. **Rutas** `frontend/src/modules/razonamiento/apuntes.ts` · `backend/src/modules/razonamiento/evidencias.rs` · `backend/migrations/0009_razonamiento.sql`.

  **Depende de** T026, T028, T034. **Referencias** FR-006, FR-007, FR-010 · E02, E09 · SC-001. **Cierre.** Guardar «solicitaría el ECG» sin ordenar nada en un intento nuevo mantiene el informe reservado. Reabrir otro intento con entrevista, medición e informe permite distinguir declaración, dato medido, orientación e interpretación y recuperar sus IDs de origen. [Apunte sin orden y evidencias recuperadas por origen · pendiente](#ev-t035)

  **Datos** [`Apunte.intención`, `Vínculo.intento`, `Vínculo.evidencia`](data-model.md#actividad-e-intento).

- [ ] T036 [US3] Guardar y reanudar hipótesis parciales y sus vínculos a las evidencias. **Rutas** `frontend/src/modules/razonamiento/hipotesis.ts` · `backend/src/modules/razonamiento/hipotesis.rs`.

  **Depende de** T035. **Referencias** FR-007, FR-010 · E02, E09 · SC-001. **Cierre.** Guardar un borrador incompleto y completarlo después. Al reabrir, coinciden principal, alternativa, explicaciones y referencias a turno, medición o informe propios. Una referencia ajena se rechaza y una revisión obsoleta devuelve 409 sin perder el último borrador guardado. [Dos revisiones, vínculos válidos y rechazos · pendiente](#ev-t036)

  **Datos** [`Hipótesis.rol`, `Hipótesis.explicación`](data-model.md#actividad-e-intento) · [`Intento.revisión esperada`](data-model.md#escrituras-caídas-y-reenvíos).

### Paridad y reserva

- [ ] T037 [US3] Comparar selección, chat, brazo ampliado y presión hasta su histórico a resolución fija. **Rutas** `frontend/tests/paridad_semana5.test.ts` · `docs/referencias/html-semana5/Paciente_Virtual.html`.

  **Depende de** T026, T028, T029, T030, T031, T032, T033. **Referencias** FR-005, FR-012 · E07, E08 · SC-003. **Cierre.** El informe empareja original y extracción por estado, navegador, resolución, escala y zoom. El teclado recorre preparación, colocación, lectura e histórico con foco visible. Identificar la diferencia documentada entre el HTML (128/78 y 112/min) y el triaje inicial (148/85 y 102/min) de LR. [Serie comparada, secuencia de teclas y diferencias justificadas · pendiente](#ev-t037)

- [ ] T038 [US3] Revisar la compilación pública y el tráfico de una sesión estudiantil antes de cerrar el hito. **Rutas** `frontend/tests/reserva_publica.test.ts` · `backend/tests/visibilidad.rs`.

  **Depende de** T024, T034, T036, T037. **Referencias** FR-009, FR-012 · E10 · SC-005. **Cierre.** Examinar archivos y JSON antes del pre, durante la atención y antes de publicar, usando testigos identificables del material reservado en datos de ensayo. Debe haber cero apariciones del HTML íntegro, claves, rúbricas, hechos reservados, trazas docentes o borradores. Registrar archivos y respuestas examinados. Repetir la revisión con las vistas reales en T079. [Inventario público y respuestas revisadas por fase · pendiente](#ev-t038)

## Hito 3 · Aislar los agentes y calibrar la devolución

Integrar los cuatro procesos sobre el contrato probado en H1 y H2. Para las pruebas educativas de este hito se usarán estados del post y aprobaciones de rúbrica **de ensayo** identificados. La autorización clínica real corresponde a T078 y el recorrido por la interfaz integrada a T067 y T079. [Contrato y regresiones por agente](contracts/agentes.md)

### Mensajes, autorización y memoria

- [ ] T039 [US6] Implementar el contrato JSON entre Axum y los trabajadores y reutilizar el adaptador de entrevista de H1. **Rutas** `backend/src/agentes/mensajes.rs` · `backend/src/modules/entrevista/turnos.rs`.

  **Depende de** T038. **Referencias** FR-004, FR-009 · E16, E18 · SC-002. **Cierre.** Una respuesta con operación, intento, agente y versión correctos se asocia al turno que la originó. Al cambiar uno de esos identificadores se descarta, se registra el motivo y el historial confirmado no cambia. Verificar también fase y hablante del mensaje antes de persistir. [Mensajes válidos y cruces rechazados · pendiente](#ev-t039)

- [ ] T040 [US6] Arrancar cuatro procesos con identidad y canal propios desde el backend. **Rutas** `backend/src/agentes/supervisor.rs` · `backend/src/app/trabajadores.rs`.

  **Depende de** T039. **Referencias** FR-004, FR-009 · E16 · SC-002. **Cierre.** El inventario identifica PID y canal de LR, Nannu, Dan Sparks y evaluador. Las cuatro configuraciones apuntan al mismo Ollama local y al digest de `qwen3.5:4b`. Cada solicitud llega al agente seleccionado por el backend. [PID, rol, canal, destino de inferencia y digest · pendiente](#ev-t040)

- [ ] T041 [US6] Autorizar propietario, rol, asignación y fase antes de construir contexto o devolver un acuse previo. **Rutas** `backend/src/shared/autorizacion.rs` · `backend/tests/autorizacion.rs`.

  **Depende de** T040, T007. **Referencias** FR-001, FR-009 · E13, E17 · SC-002, SC-005. **Cierre.** Un estudiante ajeno no puede leer el intento, recuperar su operación ni activar una llamada a Qwen. Una identidad autorizada recupera su propio intento. Comparar decisiones de acceso, número de invocaciones y recuento de turnos antes y después. [Matriz de actores, decisiones y recuentos · pendiente](#ev-t041)

- [ ] T042 [US6] Reconstruir la memoria desde SQLite para cada estudiante, intento y agente. **Rutas** `backend/src/shared/contexto.rs` · `backend/src/shared/operaciones.rs`.

  **Depende de** T041. **Referencias** FR-004, FR-009, FR-010 · E16, E17, E18 · SC-002. **Cierre.** Comparar los contextos de dos estudiantes y de dos casos. Cada conjunto contiene exclusivamente turnos propios y hechos permitidos por caso, versión, fase y hablante. Un cambio de intento o reinicio reconstruye la selección con las mismas restricciones. [IDs esperados y recuperados por intento y agente · pendiente](#ev-t042)

  **Datos** [`Traza restringida.intento`, `Traza restringida.agente`, `Traza restringida.caso`, `Traza restringida.versión`](data-model.md#actividad-e-intento).

### Ocho núcleos por agente

Cada agente tendrá un `README.md` en cada núcleo bajo `backend/src/agentes/{agente}/{núcleo}/`. Los agentes serán `lr`, `nannu`, `dan_sparks` y `evaluador`. Los núcleos son `01_rol_objetivo_modelo/`, `02_fuentes_recuperacion_memoria/`, `03_herramientas_mcp/`, `04_identidad_acceso/`, `05_controles_entrada_contexto_salida/`, `06_cache/`, `07_enrutamiento/`, `08_observabilidad_regresion/`. La ficha de cada núcleo documentará **propósito, justificación, fuentes autorizadas, comportamiento esperado, archivo de implementación y archivo y nombre de la prueba**. Los README pueden enlazar la misma implementación. Cada agente identificará su regla propia y cómo resuelve caché o MCP con la ficha u operaciones internas, cuando proceda. T043–T046 registran las correspondencias. T065 exige que los enlaces resuelvan y que las pruebas pasen. [Matriz de los 32 núcleos](contracts/agentes.md#cómo-localizar-una-decisión)

- [ ] T043 [US2] Preparar el perfil y el enrutamiento de LR y documentar sus ocho núcleos. **Rutas** `backend/src/agentes/lr/01_rol_objetivo_modelo/README.md` · `backend/tests/agente_lr.rs` · `backend/src/agentes/lr/perfil.rs` · `backend/src/agentes/lr/enrutamiento.rs`.

  **Depende de** T042. **Referencias** FR-004, FR-009 · E04, E06, E16, E18 · SC-002. **Cierre.** Hay ocho README en las rutas de la matriz. Cada uno identifica los seis campos de la ficha de núcleo del contrato, el archivo y el caso de prueba de LR. Sus ejemplos distinguen selección literal, texto libre y medición. El inventario enumera las ocho correspondencias. [Ocho núcleos de LR con fuente, archivo y prueba · pendiente](#ev-t043)

- [ ] T044 [US6] Documentar los ocho núcleos de Nannu y preparar su perfil, fuentes y reglas de hablante. **Rutas** `backend/src/agentes/nannu/01_rol_objetivo_modelo/README.md` · `backend/src/agentes/nannu/perfil.rs` · `backend/tests/agente_nannu.rs` · `backend/src/agentes/nannu/enrutamiento.rs`.

  **Depende de** T042. **Referencias** FR-004, FR-009 · E16, E18 · SC-002. **Cierre.** Los ocho README cumplen la ficha de núcleo. Desde el de fuentes se identifica el pasaje P02 que atribuye el dolor a la paciente y la tos productiva al acompañante. Las dos voces comparten escenario y mantienen su atribución al construir la respuesta. [Ocho correspondencias y pasajes de ambos hablantes · pendiente](#ev-t044)

- [ ] T045 [US6] Documentar los ocho núcleos de Dan Sparks y preparar las reglas de su perfil y procedencia. **Rutas** `backend/src/agentes/dan_sparks/01_rol_objetivo_modelo/README.md` · `backend/src/agentes/dan_sparks/perfil.rs` · `backend/tests/agente_dan_sparks.rs` · `backend/src/agentes/dan_sparks/enrutamiento.rs`.

  **Depende de** T042. **Referencias** FR-004, FR-009 · E16, E18 · SC-002. **Cierre.** Los ocho README cumplen la ficha de núcleo. La cronología remite al actor P03 y los informes al controlador. Una referencia a la imagen de ECG ausente conserva su estado de disponibilidad. Cada decisión identifica su prueba específica. [Ocho correspondencias y procedencia de relato e informe · pendiente](#ev-t045)

- [ ] T046 [US4] Preparar el perfil del evaluador y documentar sus ocho núcleos, entradas y operaciones. **Rutas** `backend/src/agentes/evaluador/01_rol_objetivo_modelo/README.md` · `backend/src/agentes/evaluador/perfil.rs` · `backend/tests/agente_evaluador.rs`.

  **Depende de** T042. **Referencias** FR-008, FR-009 · E11, E12, E16 · SC-005. **Cierre.** Los ocho README cumplen la ficha de núcleo. Identifican la entrega confirmada, sus evidencias y la rúbrica congelada como entrada, y un borrador por criterio como salida. La autorización de publicación pertenece al docente. T065 contrastará las ocho rutas con la implementación terminada. [Ocho correspondencias y permisos del evaluador · pendiente](#ev-t046)

### Límites de la entrevista de LR

- [ ] T047 [US2] Implementar y probar la aclaración de LR ante «¿Y eso?» sin referente. **Rutas** `backend/src/agentes/lr/entrevista.rs` · `backend/tests/agente_lr.rs`.

  **Depende de** T043. **Referencias** FR-004 · E03 · SC-002. **Cierre.** En una conversación nueva LR pide que se precise el referente. Se confirma un turno de aclaración y el número de hallazgos citables permanece igual. [Turno y recuento de hallazgos de E03 · pendiente](#ev-t047)

- [ ] T048 [US2] Probar el desconocimiento familiar documentado del caso LR. **Rutas** `backend/src/agentes/lr/entrevista.rs` · `backend/tests/agente_lr.rs`.

  **Depende de** T043. **Referencias** FR-004 · E04 · SC-002. **Cierre.** En otro intento nuevo, LR expresa en primera persona que desconoce los antecedentes por adopción. El tipo de respuesta y la traza enlazan el hecho y el pasaje del actor de esa versión. [Turno y cita de procedencia de E04 · pendiente](#ev-t048)

- [ ] T049 [US2] Resolver la consulta sobre un dato ausente de la ficha de LR. **Rutas** `backend/src/agentes/lr/entrevista.rs` · `backend/tests/agente_lr.rs`.

  **Depende de** T043. **Referencias** FR-004 · E05 · SC-002. **Cierre.** En un tercer diálogo nuevo, preguntar por el grupo sanguíneo produce un aviso identificable del sistema y cero hechos clínicos nuevos. No se atribuye a LR un grupo ni una negativa clínica. [Aviso, tipo de respuesta y ficha antes y después · pendiente](#ev-t049)

### Identidad, hablante y aislamiento

- [ ] T050 [US6] Integrar y probar la atribución al hablante en el caso de Nannu. **Rutas** `backend/src/agentes/nannu/entrevista.rs` · `backend/tests/agente_nannu.rs`.

  **Depende de** T044. **Referencias** FR-004, FR-009 · E16, E18 · SC-002. **Cierre.** Al alternar destinatarios, el dolor de P02 queda en la voz de Nannu y la tos productiva en la del acompañante. Los turnos y citas indican dos hablantes correctos del mismo intento. Una selección con hablante cruzado se rechaza. [Turnos con ID de hablante y pasaje · pendiente](#ev-t050)

- [ ] T051 [US6] Alternar Dan y LR y comprobar las dos cronologías. **Rutas** `backend/src/agentes/dan_sparks/entrevista.rs` · `backend/tests/agente_dan_sparks.rs`.

  **Depende de** T043, T045. **Referencias** FR-004, FR-009 · E16, E18 · SC-002. **Cierre.** Dan refiere el inicio hace cuatro horas mientras trabajaba en el jardín y LR hace dos días. Al alternarlos y reabrir ambos intentos no hay ID de hecho, fuente ni turno intercambiado. [Cronología y fuente por intento antes y después · pendiente](#ev-t051)

- [ ] T052 [US6] Ejecutar E17 con dos solicitudes simultáneas a LR desde dos sesiones e intentos distintos. **Ruta** `backend/tests/agente_lr.rs`.

  **Depende de** T041, T042, T048, T051. **Referencias** FR-001, FR-009, FR-010 · E17 · SC-002, SC-006. **Cierre.** `aisla_dos_preguntas_simultaneas` acredita la recepción de ambas solicitudes antes de terminar la primera respuesta. A obtiene los dos días y B el desconocimiento por adopción. Cada historial gana exactamente un par con IDs propios. Reanudar preserva esa separación y leer el historial ajeno deniega acceso. El solapamiento se mide en la API aunque el motor atienda las inferencias en serie. [Tiempos de recepción y fin, IDs y conteos de ambos historiales · pendiente](#ev-t052)

- [ ] T053 [US6] Interrumpir al trabajador de Nannu durante un turno y reanudarlo. **Rutas** `backend/src/agentes/supervisor.rs` · `backend/tests/agente_nannu.rs`.

  **Depende de** T040, T050, T051. **Referencias** FR-009, FR-010 · E17 · SC-002, SC-006. **Cierre.** Durante la interrupción se confirma una operación en otro trabajador. El proceso recuperado de Nannu deja un único turno confirmado para la misma operación y conserva hablante e intento. Comparar identificadores y actividad de los demás procesos. [PID, confirmaciones y continuidad de otros agentes · pendiente](#ev-t053)

### Rúbrica, entrega y evaluación

- [ ] T054 [US4] Preparar el formato privado y la carga de una rúbrica versionada de pregrado avanzado. **Rutas** `backend/src/modules/evaluacion/rubricas.rs` · `backend/tests/fixtures/rubricas/lr-v1.json` · `backend/migrations/0010_rubricas.sql`.

  **Depende de** T046, T008. **Referencias** FR-008 · E11, E12 · SC-005. **Cierre.** En calibración, una versión sin aprobación queda como borrador. Con una constancia de ensayo de docente responsable, fecha, motivo y SHA-256 del contenido evaluativo se cargan criterios, evidencias, expectativas, pesos y regla de cálculo. Repetir la carga mantiene el ID. Una huella distinta bajo igual versión se rechaza. T078 recogerá la aprobación formal. [Borrador, carga con constancia de prueba y rechazo de huella · pendiente](#ev-t054)

  **Datos** [`Rúbrica.actividad`, `Rúbrica.caso`, `Rúbrica.criterios observables`, `Rúbrica.evidencias exigibles`, `Rúbrica.expectativas`, `Rúbrica.ponderaciones`, `Rúbrica.regla de cálculo`](data-model.md#fuentes-y-versiones) · [`Rúbrica.versión`, `Rúbrica.huella`, `Rúbrica.constancia de aprobación`](data-model.md#preparación-y-aprobación-de-la-rúbrica).

- [ ] T055 [US4] Probar el cambio de ponderaciones al habilitar una rúbrica V2 de ensayo. **Rutas** `backend/src/modules/intentos/intento.rs` · `backend/tests/version_rubrica.rs`.

  **Depende de** T054, T008. **Referencias** FR-001, FR-008, FR-010 · E02, E11, E12 · SC-001, SC-005. **Cierre.** Abrir A con V1, aprobar en ensayo y habilitar V2 con otro peso y abrir B de otro estudiante. Al reabrir A y B, cada uno conserva el identificador, versión y huella esperados. Guardar ambos intentos para verificar la entrega y el cálculo en T056 y T062. [Asignación, peso y huellas de A y B · pendiente](#ev-t055)

  **Datos** [`Intento.versión de rúbrica`](data-model.md#actividad-e-intento).

- [ ] T056 [US4] Confirmar la entrega, congelar su contenido y encolar su evaluación en una transacción SQLite. **Rutas** `backend/src/modules/evaluacion/entregas.rs` · `backend/migrations/0011_entregas.sql` · `frontend/src/modules/razonamiento/entrega.ts`.

  **Depende de** T036, T055. **Referencias** FR-007, FR-008, FR-010, FR-011 · E11, E22 · SC-001, SC-005, SC-006. **Cierre.** Admitir también trabajo parcial. El reenvío devuelve el mismo acuse, una entrega y un trabajo pendiente. Un fallo antes del commit deja cero registros parciales. Entregar A tras habilitar V2 conserva la rúbrica V1 de A, su revisión y evidencias. Una respuesta tardía no modifica lo entregado. [Acuses, instantánea, IDs y recuentos transaccionales · pendiente](#ev-t056)

  **Datos** [`Entrega.intento`, `Entrega.revisión`, `Entrega.hipótesis`, `Entrega.evidencias`, `Entrega.identificador de rúbrica`, `Entrega.versión de rúbrica`, `Entrega.huella de rúbrica`, `Trabajo de evaluación.entrega`, `Trabajo de evaluación.estado`](data-model.md#actividad-e-intento).

- [ ] T057 [US4] Construir el contexto del evaluador desde una sola entrega confirmada. **Rutas** `backend/src/modules/evaluacion/evidencia.rs` · `backend/tests/agente_evaluador.rs`.

  **Depende de** T042, T046, T056. **Referencias** FR-008, FR-009 · E10, E11, E16 · SC-005. **Cierre.** Comparar la entrada con la copia congelada. Coinciden entrega, evidencias propias y rúbrica por ID, versión y huella. Un intento abierto no dispara evaluación. Las encuestas, claves educativas y registros de otro intento quedan fuera. [Inventario autorizado y comparación con la instantánea · pendiente](#ev-t057)

- [ ] T058 [US4] Generar y conservar el borrador formativo por criterio aplicable. **Rutas** `backend/src/modules/evaluacion/criterios.rs` · `backend/src/modules/evaluacion/trabajos.rs` · `backend/tests/agente_evaluador.rs` · `backend/migrations/0012_evaluacion.sql`.

  **Depende de** T057. **Referencias** FR-008 · E12 · SC-001, SC-005. **Cierre.** Revisar entrevista, valoración, decisiones, evidencias e hipótesis. Cada criterio explica actuación, cita de la entrega, expectativa, juicio motivado y mejora concreta. El 100 % de los créditos propuestos llega a una evidencia del intento. Un criterio fuera de la oportunidad formativa se marca no evaluable. [Desglose, citas y recorrido de una observación a su registro · pendiente](#ev-t058)

  **Datos** [`Evaluación.entrega`, `Criterio.actuación`, `Criterio.evidencia o incidencia`, `Criterio.expectativa`, `Criterio.juicio motivado`, `Criterio.mejora`](data-model.md#actividad-e-intento).

### Calibración y revisión docente

Las dos entregas de E12 tienen cuatro hipótesis para calibrar la calidad de la devolución. En la actividad ordinaria el estudiante fundamenta una hipótesis principal y la contrasta con al menos una alternativa. [Calibración de la devolución](contracts/agentes.md#pruebas-que-deciden-el-paso-al-hito-4)

- [ ] T059 [US4] Ejecutar la calibración E12 con tres hipótesis justificadas de cuatro. **Rutas** `backend/tests/fixtures/evaluacion/tres_de_cuatro.json` · `backend/tests/agente_evaluador.rs`.

  **Depende de** T058. **Referencias** FR-008 · E12 · SC-005. **Cierre.** El borrador reconoce por separado los tres aciertos y sus evidencias. Para la cuarta hipótesis identifica el vínculo que falta y una acción concreta para revisarla. Quedan identificadas entrega y versión de la rúbrica de calibración. [Cuatro juicios, citas y acción para la cuarta hipótesis · pendiente](#ev-t059)

- [ ] T060 [US4] Ejecutar la calibración E12 con una hipótesis justificada de cuatro. **Rutas** `backend/tests/fixtures/evaluacion/uno_de_cuatro.json` · `backend/tests/agente_evaluador.rs`.

  **Depende de** T058. **Referencias** FR-008 · E12 · SC-005. **Cierre.** Se reconoce un acierto y se identifican tres brechas diferentes. La clave de ensayo espera contradicción con el ECG, alternativa sin justificar y confusión entre declaración y medición. La primera corrección indicada es releer el ECG, con su evidencia y motivo. [Cuatro juicios y justificación de la mejora prioritaria · pendiente](#ev-t060)

- [ ] T061 [US4] Distinguir una decisión equivocada, una omisión y un acuse sin evento recuperable. **Rutas** `backend/src/modules/evaluacion/criterios.rs` · `backend/tests/agente_evaluador.rs`.

  **Depende de** T058. **Referencias** FR-008 · E12 · SC-005. **Cierre.** El acto documentado erróneo cita su evidencia. La omisión solo se atribuye si el registro íntegro y la oportunidad la acreditan. El acuse sin evento da incidencia, criterio pendiente de revisión docente y cero descuentos automáticos. El estado de incidencia se conserva para probar el bloqueo de publicación en T063. [Tres juicios contrastados y estado del criterio incompleto · pendiente](#ev-t061)

- [ ] T062 [US4] Habilitar la rectificación motivada del docente y conservar sus revisiones. **Rutas** `backend/src/modules/evaluacion/revisiones.rs` · `frontend/src/modules/devolucion/revision.ts` · `backend/migrations/0013_revisiones.sql`.

  **Depende de** T055, T058, T061. **Referencias** FR-008, FR-009, FR-010 · E11, E12, E13 · SC-005. **Cierre.** El docente asignado puede leer el desglose y rectificar con motivo. Quedan autor, revisión anterior, nueva revisión y diferencia por criterio. Una revisión obsoleta se rechaza. En una serie nueva, guardar una evaluación efectiva con V1 antes de activar V2. Tras activar V2, reproducir el total y el desglose originales con los pesos y juicios guardados de V1. [Revisiones, motivo y reproducción del desglose de V1 · pendiente](#ev-t062)

  **Datos** [`Revisión.responsable`, `Revisión.motivo`, `Revisión.revisión anterior`, `Revisión.créditos por criterio`, `Revisión.desglose ponderado`, `Revisión.total`](data-model.md#actividad-e-intento).

- [ ] T063 [US4] Implementar la publicación docente y la lectura de la devolución por el titular. **Rutas** `backend/src/modules/evaluacion/publicacion.rs` · `frontend/src/modules/devolucion/publicacion.ts`.

  **Depende de** T062. **Referencias** FR-002, FR-008, FR-009 · E12, E13, E15 · SC-004, SC-005. **Cierre.** Con estados de post identificados como ensayo, el titular obtiene solo la revisión publicada cuando el post está respondido u omitido. Publicar con post pendiente mantiene la devolución reservada. Estudiante ajeno y docente no asignado carecen de acceso. Un criterio técnico pendiente impide publicación definitiva. T067 y T079 repetirán la secuencia por la interfaz. [Matriz actor × estado del post × revisión · pendiente](#ev-t063)

  **Datos** [`Publicación.revisión elegida`, `Pre y post.estado`](data-model.md#actividad-e-intento).

### Recuperación y trazabilidad

- [ ] T064 [US7] Ejecutar E22 con procesos locales antes de Compose y recuperar el mismo trabajo de evaluación. **Rutas** `backend/src/modules/evaluacion/trabajos.rs` · `backend/tests/recuperacion_e22.rs`.

  **Depende de** T018, T028, T036, T056, T058, T063. **Referencias** FR-007, FR-010, FR-011 · E11, E22 · SC-006. **Cierre.** Con Qwen detenido, registrar una toma nueva, vincularla a la hipótesis, confirmar y reenviar la entrega y resolver el post de ensayo. Deben constar una medición nueva, un acuse, un trabajo pendiente y cero evaluaciones terminadas. Recuperar Qwen, comprobar una generación, reintentar el trabajo y su cierre. Persisten los IDs y se confirma exactamente una evaluación, incluso ante un resultado tardío de una toma vencida. [Estados, IDs, toma del trabajo y recuentos antes y después · pendiente](#ev-t064)

  **Datos** [`Trabajo de evaluación.identificador de toma`, `Trabajo de evaluación.vencimiento`](data-model.md#escrituras-caídas-y-reenvíos).

- [ ] T065 [US6] Cerrar el hito recorriendo los 32 núcleos y siguiendo un fallo de entrevista y un criterio hasta su regresión. **Rutas** `specs/001-simulacion-clinica/contracts/agentes.md` · `specs/001-simulacion-clinica/evidencias/h3/T065/matriz.tsv`.

  **Depende de** T043, T044, T045, T046, T047, T048, T049, T050, T051, T052, T053, T059, T060, T061, T062, T063, T064. **Referencias** FR-004, FR-008, FR-009, FR-010 · E12, E16, E17, E18, E22 · SC-002, SC-005, SC-006. **Cierre.** Localizar cuatro agentes × ocho README. En cada uno resuelven el archivo de implementación y el archivo y caso de prueba de la matriz. Las cuatro baterías pasan. Desde cada muestra se llega a intento, agente, núcleo, versión, fuente/pasaje o evidencia y regresión. Adjuntar E12, E17 y E22 local. [32 correspondencias y dos recorridos de trazabilidad · pendiente](#ev-t065)

  **Datos** [`Traza restringida.núcleo`, `Traza restringida.versiones del modelo e instrucciones`, `Traza restringida.rúbrica`, `Traza restringida.fuente y pasaje o evidencia`, `Traza restringida.tiempo de espera`, `Traza restringida.resultado`, `Traza restringida.prueba de regresión`](data-model.md#actividad-e-intento).

## Hito 4 · Aceptar el recorrido completo en el computador del profesor

Tras T065, sustituir los estados educativos de ensayo por el pre y el post operables. Los ensayos técnicos de Compose pueden usar versiones de prueba. La demostración formal requiere las versiones docentes registradas en T078. [Guía de aceptación formal](quickstart.md#4-aceptar-la-demostración-en-el-equipo-del-profesor)

### Encuestas e informe educativo

- [ ] T066 [US5] Implementar el pre opcional de cinco preguntas en la interfaz y la API. **Rutas** `frontend/src/modules/encuestas/pre.ts` · `backend/src/modules/encuestas/respuestas.rs` · `backend/migrations/0014_encuestas.sql`.

  **Depende de** T065, T024. **Referencias** FR-002, FR-003 · E01, E14, E15 · SC-004. **Cierre.** La forma contiene tres ítems de percepción con igual texto, orden y escala 1–5 que el post previsto, y dos de razonamiento. Antes de responder u omitir se reserva el caso. Cualquiera de las dos decisiones habilita la práctica, se conserva al reanudar y cierra la edición del pre. [Dos recorridos por interfaz, acuses y decisión reabierta · pendiente](#ev-t066)

  **Datos** [`Instrumento educativo.versión`, `Instrumento educativo.enunciados`, `Instrumento educativo.orden`, `Instrumento educativo.anclajes de percepción`, `Instrumento educativo.formas A/B`, `Instrumento educativo.alternativas`, `Instrumento educativo.claves`](data-model.md#fuentes-y-versiones) · [`Pre y post.intento`, `Pre y post.momento`, `Pre y post.versión`, `Pre y post.estado`, `Pre y post.forma asignada`](data-model.md#actividad-e-intento).

- [ ] T067 [US5] Implementar el post opcional de siete preguntas y conectarlo a la publicación real. **Rutas** `frontend/src/modules/encuestas/post.ts` · `backend/src/modules/encuestas/respuestas.rs`.

  **Depende de** T066, T056, T063. **Referencias** FR-002, FR-003, FR-008 · E13, E14, E15 · SC-004, SC-005. **Cierre.** Tras la entrega se ofrecen tres ítems de percepción, dos de razonamiento y dos de utilidad, aun con evaluación pendiente. Por interfaz, tanto responder como omitir habilita la lectura si ya hay publicación. Antes de entregar se rechaza el post y tras ver la devolución se rechaza un post retroactivo. Repetir el envío conserva un único acuse. [Capturas y acuses por orden de post y publicación · pendiente](#ev-t067)

  **Datos** [`Instrumento educativo.ítems exclusivos del post`](data-model.md#fuentes-y-versiones).

- [ ] T068 [US5] Asignar las dos secuencias A→B y B→A de forma alternada y persistente. **Rutas** `backend/src/modules/encuestas/formas.rs` · `backend/src/modules/intentos/intento.rs`.

  **Depende de** T066, T067. **Referencias** FR-002, FR-003, FR-010 · E02, E14, E15 · SC-004. **Cierre.** Al abrir dos intentos consecutivos de la misma actividad y versión de instrumento reciben órdenes opuestos. Una creación reenviada no consume otra asignación. Reanudar mantiene forma, orden y momento. Dos altas concurrentes conservan la alternancia transaccional. [IDs, contador, versiones y formas antes y después · pendiente](#ev-t068)

  **Datos** [`Intento.secuencia educativa`, `Intento.actividad`, `Intento.versión de instrumento`](data-model.md#actividad-e-intento).

- [ ] T069 [US5] Generar el informe educativo y comprobar E14 y E15 por separado de la nota clínica. **Rutas** `backend/src/modules/encuestas/informe.rs` · `frontend/src/modules/encuestas/informe.ts`.

  **Depende de** T067, T068, T062. **Referencias** FR-002, FR-003 · E14, E15 · SC-004. **Cierre.** En E14 el primero completa ambos y reenvía el post, el segundo omite el pre y el tercero reanuda y omite el post. Deben constar tres invitados, dos pre, dos post y un par completo. Variar la encuesta conserva la nota clínica. En un par, 1/2 a 2/2 equivale a +50 puntos porcentuales. E15 compara ambas secuencias completas, percepción por ítem, razonamiento por forma y orden, y utilidad aparte. Informar faltantes sin convertirlos en errores. Calcular el razonamiento sobre dos solo cuando se hayan respondido ambos ejercicios. [Recuentos, puntajes por medida y nota clínica contrastada · pendiente](#ev-t069)

  **Datos** [`Reporte.actividad`, `Reporte.estudiante`, `Reporte.intento`, `Reporte.versión`, `Reporte.pares completos`, `Reporte.faltantes`](data-model.md#reporte-educativo).

### Servicios, persistencia y recuperación en Compose

- [ ] T070 [US7] Preparar el servicio frontend para servir su compilación y reenviar `/api` al backend. **Rutas** `frontend/Dockerfile` · `frontend/servidor.conf` · `compose.yaml`.

  **Depende de** T065, T038. **Referencias** FR-009, FR-011 · E10, E19, E21 · SC-005, SC-006. **Cierre.** Con un backend de ensayo accesible en su red, la página abre en `127.0.0.1:8080` y una petición desde ella llega a Axum por el mismo origen. El servidor publica únicamente los archivos permitidos. La comprobación completa de servicios corresponde a T075. [Dirección local, captura y respuesta de red del mismo origen · pendiente](#ev-t070)

- [ ] T071 [US7] Preparar `sqlite-init` como servicio temporal con la imagen del backend. **Rutas** `backend/Dockerfile` · `backend/src/bin/sircle.rs` · `compose.yaml`.

  **Depende de** T070, T006, T054. **Referencias** FR-010, FR-011 · E20, E21 · SC-006. **Cierre.** Con volumen vacío se crea `/var/lib/sircle/sircle.sqlite`, se importan tres escenarios y 14 fuentes y el inicializador termina con código cero. Una segunda ejecución sobre `sircle-datos` recupera los mismos IDs. El modo de preparación y el de servicio usan la misma imagen Rust. [Estado de salida, imagen, volumen e inventarios · pendiente](#ev-t071)

- [ ] T072 [US7] Preparar el servicio backend para esperar la inicialización correcta de SQLite. **Rutas** `compose.yaml` · `backend/src/app/estado.rs`.

  **Depende de** T071, T064. **Referencias** FR-010, FR-011 · E20, E21, E22 · SC-006. **Cierre.** Con preparación fallida la API no arranca sobre una base incompleta. Tras salida correcta de `sqlite-init` se habilita aun con inferencia indisponible. `GET /api/estado` distingue base y modelo. Una toma y una entrega nuevas reciben acuse con la evaluación pendiente. [Dependencia de arranque, estados y dos acuses · pendiente](#ev-t072)

- [ ] T073 [US7] Preparar el servicio permanente de inferencia con su volumen de pesos y verificación del modelo. **Rutas** `inferencia/Dockerfile` · `inferencia/preparar.sh` · `compose.yaml`.

  **Depende de** T072, T013. **Referencias** FR-011 · E21, E22 · SC-006. **Cierre.** Con `sircle-pesos` en `/root/.ollama`, el preparador consulta la etiqueta pública `qwen3.5:4b`, descarga si falta, coteja el digest completo y exige una generación breve correcta antes de anunciar disponibilidad. Un digest inesperado deja incidencia. Ollama solo recibe tráfico en la red de Compose. [Etiqueta, digest esperado y local, prueba de generación y estado · pendiente](#ev-t073)

- [ ] T074 [US7] Ensayar la preparación sin pesos, el límite de reintentos y la recuperación. **Rutas** `inferencia/preparar.sh` · `specs/001-simulacion-clinica/evidencias/h4/T074/reintentos.txt`.

  **Depende de** T073. **Referencias** FR-011 · E21, E22 · SC-006. **Cierre.** Con volumen de prueba vacío y el registro inaccesible hay tres intentos totales y esperas de dos y cuatro segundos. El aviso identifica servicio, modelo, motivo y acción. Los sondeos no reinician la descarga. Al restablecer conectividad y reiniciar inferencia se verifica el digest y una generación, conservando el intento en el volumen de datos. [Conteo por ciclo, aviso, recuperación y mismo intento · pendiente](#ev-t074)

- [ ] T075 [US7] Arrancar desde un clon limpio en el equipo del profesor con `docker compose up --build`. **Rutas** `compose.yaml` · `specs/001-simulacion-clinica/evidencias/h4/T075/arranque.txt`.

  **Depende de** T070, T071, T072, T073, T074. **Referencias** FR-011 · E21 · SC-006. **Cierre.** Con imágenes y modelo públicos y sin `docker login`, cuenta del profesor, contraseñas, tokens ni `.env` secreto, abre la interfaz en localhost. `docker compose ps -a` muestra frontend, backend e inferencia activos y `sqlite-init` finalizado con código cero. Solo el frontend tiene un puerto publicado en loopback. [Revisión del clon, comando, estados de los cuatro servicios y puertos · pendiente](#ev-t075)

- [ ] T076 [US7] Comprobar reinicio y restaurar una copia consistente de SQLite en un volumen de prueba separado. **Rutas** `backend/tests/restauracion.rs` · `specs/001-simulacion-clinica/evidencias/h4/T076/restauracion.tsv`.

  **Depende de** T075, T062. **Referencias** FR-010, FR-011 · E02, E20, E21 · SC-006. **Cierre.** Con un intento guardado y una revisión publicada de ensayo, reiniciar Compose conserva datos y reutiliza los pesos verificados. Restaurar la copia en otro volumen recupera los mismos IDs de fuentes, intento, turnos, tomas, entrega y revisión publicada. Comparar ambas vistas con el inventario previo. [Inventarios antes, tras reinicio y en la copia restaurada · pendiente](#ev-t076)

- [ ] T077 [US7] Repetir E22 en Compose con la interfaz del post integrada. **Rutas** `compose.yaml` · `specs/001-simulacion-clinica/evidencias/h4/T077/e22-compose.json`.

  **Depende de** T064, T067, T075. **Referencias** FR-007, FR-010, FR-011 · E11, E22 · SC-006. **Cierre.** Con `docker compose stop inferencia`, hacer una medición nueva, citarla, entregar, reenviar y resolver el post. Quedan una medición nueva, una entrega, un trabajo pendiente y cero evaluaciones terminadas. Tras `docker compose start inferencia` y una generación de prueba, conservar los IDs y obtener exactamente una evaluación terminada incluso al reintentar el trabajo. [Comparación de IDs y recuentos durante y después de la caída · pendiente](#ev-t077)

### Autorización y aceptación formal

- [ ] T078 [US4] Registrar la autorización clínica y educativa del docente antes de habilitar la actividad formal. **Rutas** `datos/adaptaciones/lr/rubricas/<version>.json` · `specs/001-simulacion-clinica/evidencias/h4/T078/aprobaciones.md`.

  **Depende de** T054, T055, T068, T075. **Referencias** FR-003, FR-005, FR-006, FR-008 · E07, E09, E12, E14, E15 · SC-001, SC-003, SC-004, SC-005. **Cierre.** El docente responsable aprueba la ficha española, hechos e instrumentos de fase inicial, redacción del informe textual de ECG, rúbrica avanzada, ponderaciones y claves educativas. Revisa objetivos, distractores y equivalencia esperada de A/B. El acta recoge quién, cuándo, motivo, versiones y huellas cotejadas en SQLite. Si falta aprobación, la actividad formal sigue inhabilitada. [Acta docente restringida y correspondencia con las versiones cargadas · pendiente](#ev-t078)

  **Datos** [`Aprobación docente.autor`, `Aprobación docente.fecha`, `Aprobación docente.motivo`, `Aprobación docente.versión`](data-model.md#fuentes-y-versiones).

- [ ] T079 [US1] Ejecutar el recorrido completo de LR en el computador del profesor con la versión aprobada. **Rutas** `specs/001-simulacion-clinica/quickstart.md` · `specs/001-simulacion-clinica/evidencias/h4/T079/recorrido.md`.

  **Depende de** T069, T075, T077, T078. **Referencias** FR-001, FR-002, FR-004, FR-005, FR-006, FR-007, FR-008, FR-009 · E01, E02, E06, E07, E09, E10, E11, E12, E13, E15 · SC-001, SC-003, SC-004, SC-005. **Cierre.** Con identidad ficticia, responder u omitir el pre de cinco, reanudar, entrevistar libremente, medir 148/85 mmHg y comprobar 102/min en pulso. Confirmar el ECG y consultar «taquicardia sinusal». Citar evidencia para la principal y al menos una alternativa, entregar y responder u omitir el post de siete. El docente sigue una cita, revisa y publica. El titular ve esa revisión tras el post. Repetir el control de red de T038 con las versiones aprobadas de la actividad ficticia y registrar frío/caliente en ese equipo. [Secuencia por rol, IDs, captura de la devolución y tiempos del equipo docente · pendiente](#ev-t079)

- [ ] T080 [US1] Cerrar con el profesor el acta de aceptación formal de LR. **Ruta** `specs/001-simulacion-clinica/evidencias/h4/T080/acta.md`.

  **Depende de** T069, T076, T077, T078, T079. **Referencias** FR-002, FR-003, FR-007, FR-008, FR-009, FR-010, FR-011 · E01, E11, E12, E13, E14, E15, E21, E22 · SC-001, SC-004, SC-005, SC-006. **Cierre.** El acta une revisión Git, equipo, versiones aprobadas, intento, cita del evaluador, registro que la sustenta y revisión publicada visible. Adjunta informe pre/post separado de la nota y recuentos E22. Registra incidencias, efecto sobre el recorrido y decisión fechada del docente. La aceptación requiere que el recorrido y los criterios exigidos queden acreditados. [Decisión de aceptación y referencias a cada evidencia · pendiente](#ev-t080)

## Estudios posteriores de capacidad

Estos ensayos se harán después de T080 en el computador del profesor. Para cada corrida registrar hardware, modelo/digest y cuantización, longitud de contexto, concurrencia real, p50/p95 de espera y respuesta, RAM/VRAM, errores, contención de escritura en SQLite y recuperación. Ejecutar los perfiles en intervalos distintos para poder atribuir cada resultado. Los umbrales quedan **pendientes de medición exploratoria y ratificación**. En RR01, 99 % de solicitudes válidas completadas y p95 de 2 s al abrir, 3 s al guardar y 15 s en diálogo son valores propuestos para 20 usuarios. [Supuestos y RR01](spec.md#supuestos-dependencias-y-límites) · [Resultados medibles](spec.md#resultados-medibles)

- [ ] T081 [US8] Medir la referencia RR01 en el equipo del profesor con 20 usuarios durante 15 minutos. **Ruta** `pruebas/capacidad/rr01_20.ts`.

  **Depende de** T080. **Referencias** FR-013 · RR01. **Cierre.** Registrar volumen real de cada operación y contrastarlo con la propuesta de al menos 100 por categoría. Informar finalización de solicitudes válidas y p95 al abrir, guardar y dialogar. Identificar cualquier categoría con muestra insuficiente. Los valores propuestos de RR01 siguen pendientes de ratificación. [Perfil de 20, muestra por categoría y latencias · pendiente](#ev-t081)

- [ ] T082 [US8] Medir 100 sesiones abiertas con una mezcla declarada de lectura, medición, conversación y entrega. **Ruta** `pruebas/capacidad/sesiones_abiertas.ts`.

  **Depende de** T080. **Referencias** FR-013 · E23 · SC-007. **Cierre.** Acreditar 100 sesiones activas en el intervalo e informar el reparto de operaciones y las inferencias concurrentes observadas. Cotejar los 100 historiales por titular e intento e informar cruces, pérdidas y duplicados. El resultado describe únicamente ese perfil. [Perfil de sesiones, mezcla, concurrencia y aislamiento · pendiente](#ev-t082)

- [ ] T083 [US8] Medir una ráfaga de 100 preguntas simultáneas enviadas desde 100 intentos distintos. **Ruta** `pruebas/capacidad/preguntas_simultaneas.ts`.

  **Depende de** T080. **Referencias** FR-013 · E24 · SC-007. **Cierre.** Registrar cuándo se libera la ráfaga y cuándo llega cada petición. Para los 100 IDs distinguir cola, respuesta, fallo y recuperación. Conciliar el estado final de cada operación con su historial y contar pérdidas o turnos duplicados. Informar espera y latencia separadas. [Perfil de ráfaga, tiempos y conciliación de 100 operaciones · pendiente](#ev-t083)

- [ ] T084 [US8] Acordar los umbrales de cada perfil después de la exploración y contrastarlos en ensayos posteriores. **Ruta** `specs/001-simulacion-clinica/evidencias/capacidad/T084/dictamen.md`.

  **Depende de** T081, T082, T083. **Referencias** FR-013 · E23, E24 · SC-007. **Cierre.** El docente y el equipo documentan umbrales por perfil y fecha de ratificación antes de la siguiente corrida de aceptación. El dictamen relaciona cada umbral con nuevas mediciones del mismo equipo y perfil. Un umbral pendiente o una muestra insuficiente deja sin acreditar esa capacidad. [Ratificación por perfil, repetición y decisión de capacidad · pendiente](#ev-t084)

## Dependencias y paralelismo

Las flechas muestran dónde se reúnen los resultados. Un intervalo agrupa tareas del mismo tramo. Para ejecutar una tarea concreta, consultar sus dependencias directas.

| Secuencia | Qué recibe la tarea siguiente |
| --- | --- |
| T003 → T004 → T005 → T007 → T008 → T012 → T013 → T014 · T001 y T014 → T015 → T016 | Alinear el criterio de paso, disponer de sesión, intento y hechos de LR, y probar por HTTP la selección real de Qwen. T015 puede ejecutarse con el navegador cerrado y antes de T021 o T039–T046. Las dependencias directas exactas figuran en cada tarea. |
| T009 → T010 y T002 → T011 · T015–T019 → T020 | La demostración reúne la lectura persistida, el gesto visual de ejemplo, el turno rastreable, el 503 y la recuperación de la operación. |
| T020 → T021 → T025 → T026 → T037 → T038 | La extracción parte de la copia verificada. El chat recupera el intento técnico y realiza una nueva pregunta por la ruta de H1 antes de cerrar la comparación visual y de acceso. |
| T038 → T039–T042 → T043–T053 | El contrato, la autorización y la memoria sustentan las cuatro identidades, las pruebas del relato y E17. |
| T054 → T055 → T056 → T057 → T058 → T059–T064 → T065 | Las versiones de rúbrica y la entrega congelada sustentan la calibración, la revisión docente y E22 local. T065 reúne los resultados clínicos, de aislamiento y de recuperación sobre material de ensayo. |
| T065 → T066–T069 y T070–T077 · T078 → T079 → T080 | Encuestas por interfaz y Compose convergen con la autorización clínica. La aceptación requiere el recorrido en el computador docente, la devolución publicada y E22 en Compose. |
| T080 → T081, T082 y T083 → T084 | El equipo se estudia bajo tres cargas distintas. La ratificación y los ensayos de aceptación de capacidad requieren las mediciones exploratorias. |

**Paralelismo al inicio.** T002 y T003 pueden comenzar con sus respectivos archivos, instalaciones y comandos independientes. Las siguientes parejas son posibilidades para la implementación una vez cerradas sus dependencias. Antes de repartirlas, comprobar que tampoco coincidan el archivo de integración, la base de ensayo o el motor que vaya a detener una prueba.

| Historias | Trabajo que podría coincidir más adelante |
| --- | --- |
| US1 y US3 | Con T022, T008 y T009 cerradas, T023 trabaja en selección y T030 en temperatura y saturación. Usar intentos independientes para las comprobaciones. |
| US2 y US6 | Tras T042, T043 puede documentar LR mientras T044 o T045 prepara el perfil de otro caso. Cada agente tiene su directorio y su archivo de prueba. Un cambio común se integra antes de repartir los README que lo enlazan. |
| US4 y US6 | Con T044, T046 y T008 cerradas, T050 puede verificar el hablante de Nannu mientras T054 trabaja sobre la rúbrica de ensayo en su propia base. T059 y T060 comparten la batería del evaluador. |
| US5 y US7 | Al pasar T065 y los requisitos directos de T066 y T070, el flujo de pre y la configuración del servidor estático pueden desarrollarse en archivos separados. Serializar la integración si comparten compilación o puerto. |
| US8 | T081–T083 comparten el computador del profesor. Medir un perfil por vez. T084 espera los tres informes. |

## Índice de evidencias

La raíz propuesta es `specs/001-simulacion-clinica/evidencias/`. Cada ruta de la tabla se refiere a esa raíz. La implementación incorporará los archivos y enlazará los anexos de captura, consulta, traza o decisión desde su registro. En cada entrada se completarán revisión Git, pasos, esperado, observado y ubicación conforme al protocolo anterior. Todas las entradas están **pendientes** en esta edición documental.

### Evidencias del hito 1

| Tarea | Registro previsto | Observación que se archivará | Estado |
| --- | --- | --- | --- |
| <a id="ev-t001"></a>T001 | `h1/T001/pasos.diff` | Diferencias de las dos filas | **pendiente** |
| <a id="ev-t002"></a>T002 | `h1/T002/frontend.txt` | Comandos, códigos de salida y pantalla de arranque | **pendiente** |
| <a id="ev-t003"></a>T003 | `h1/T003/router.txt` | Compilación y respuesta del router | **pendiente** |
| <a id="ev-t004"></a>T004 | `h1/T004/esquema.txt` | Esquema y prueba de integridad referencial | **pendiente** |
| <a id="ev-t005"></a>T005 | `h1/T005/inventario.tsv` | Catorce fuentes con sus identificadores y huellas | **pendiente** |
| <a id="ev-t006"></a>T006 | `h1/T006/reimportacion.tsv` | Inventarios antes y después y error identificado | **pendiente** |
| <a id="ev-t007"></a>T007 | `h1/T007/sesiones.json` | Vistas autorizadas de dos identidades ficticias | **pendiente** |
| <a id="ev-t008"></a>T008 | `h1/T008/intento.json` | Vista del intento antes y después de reabrir | **pendiente** |
| <a id="ev-t009"></a>T009 | `h1/T009/toma.json` | Petición, lectura y consulta del histórico | **pendiente** |
| <a id="ev-t010"></a>T010 | `h1/T010/reenvios.json` | Claves, acuses, IDs y recuentos de la prueba de valoración | **pendiente** |
| <a id="ev-t011"></a>T011 | `h1/T011/brazalete.md` | Serie de capturas y registro de la sesión de ejemplo | **pendiente** |
| <a id="ev-t012"></a>T012 | `h1/T012/hechos.tsv` | Candidatos y localización en la fuente de LR | **pendiente** |
| <a id="ev-t013"></a>T013 | `h1/T013/invocacion.json` | Petición y respuesta de una invocación real correlacionada | **pendiente** |
| <a id="ev-t014"></a>T014 | `h1/T014/selecciones.json` | Casos válidos, rechazos y recuento de turnos | **pendiente** |
| <a id="ev-t015"></a>T015 | `h1/T015/pregunta-libre.json` | Texto enviado, acuse y turno recuperado | **pendiente** |
| <a id="ev-t016"></a>T016 | `h1/T016/traza-turno.json` | Correlación del turno de T015 con la invocación y la fuente | **pendiente** |
| <a id="ev-t017"></a>T017 | `h1/T017/caida.json` | Respuesta 503 y comparación del historial | **pendiente** |
| <a id="ev-t018"></a>T018 | `h1/T018/recuperacion.json` | Clave recuperada, confirmaciones y recuento final | **pendiente** |
| <a id="ev-t019"></a>T019 | `h1/T019/latencias.tsv` | Muestras por estado y equipo, con unidad de tiempo | **pendiente** |
| <a id="ev-t020"></a>T020 | `h1/T020/acta.md` | Acta del primer avance e inventarios comparados | **pendiente** |

### Evidencias del hito 2

| Tarea | Registro previsto | Observación que se archivará | Estado |
| --- | --- | --- | --- |
| <a id="ev-t021"></a>T021 | `h2/T021/inventarios.tsv` | Rutas y dos columnas de SHA-256 | **pendiente** |
| <a id="ev-t022"></a>T022 | `h2/T022/comparacion.md` | Par de capturas y estilos contrastados | **pendiente** |
| <a id="ev-t023"></a>T023 | `h2/T023/continuar.md` | Capturas de selección y respuestas del mismo intento | **pendiente** |
| <a id="ev-t024"></a>T024 | `h2/T024/vistas.md` | Capturas y respuestas en cada fase | **pendiente** |
| <a id="ev-t025"></a>T025 | `h2/T025/sugerencias.md` | Capturas del texto intacto y editado, peticiones y rutas | **pendiente** |
| <a id="ev-t026"></a>T026 | `h2/T026/chat-integrado.md` | Captura del chat, petición y correlación con Qwen | **pendiente** |
| <a id="ev-t027"></a>T027 | `h2/T027/brazo.md` | Pares de capturas de los estados intermedios | **pendiente** |
| <a id="ev-t028"></a>T028 | `h2/T028/registro-presion.md` | Secuencia visual, acuses y recuentos de ficha e histórico | **pendiente** |
| <a id="ev-t029"></a>T029 | `h2/T029/pulso-respiracion.md` | Dos pares de capturas y sus lecturas identificadas | **pendiente** |
| <a id="ev-t030"></a>T030 | `h2/T030/temperatura-saturacion.md` | Capturas comparables por instrumento y dos históricos | **pendiente** |
| <a id="ev-t031"></a>T031 | `h2/T031/peso.md` | Comparación visual y lectura con unidad | **pendiente** |
| <a id="ev-t032"></a>T032 | `h2/T032/exploracion.md` | Pares de capturas por gesto y registros de las dos zonas | **pendiente** |
| <a id="ev-t033"></a>T033 | `h2/T033/auscultacion.md` | Capturas por punto, comprobación auditiva y registro | **pendiente** |
| <a id="ev-t034"></a>T034 | `h2/T034/orden-ecg.json` | Consultas antes y después y acuses de una orden | **pendiente** |
| <a id="ev-t035"></a>T035 | `h2/T035/evidencias.json` | Apunte sin orden y evidencias recuperadas por origen | **pendiente** |
| <a id="ev-t036"></a>T036 | `h2/T036/hipotesis.json` | Dos revisiones, vínculos válidos y rechazos | **pendiente** |
| <a id="ev-t037"></a>T037 | `h2/T037/paridad.md` | Serie comparada, secuencia de teclas y diferencias justificadas | **pendiente** |
| <a id="ev-t038"></a>T038 | `h2/T038/visibilidad.md` | Inventario público y respuestas revisadas por fase | **pendiente** |

### Evidencias del hito 3

| Tarea | Registro previsto | Observación que se archivará | Estado |
| --- | --- | --- | --- |
| <a id="ev-t039"></a>T039 | `h3/T039/correlacion.json` | Mensajes válidos y cruces rechazados | **pendiente** |
| <a id="ev-t040"></a>T040 | `h3/T040/procesos.tsv` | PID, rol, canal, destino de inferencia y digest | **pendiente** |
| <a id="ev-t041"></a>T041 | `h3/T041/permisos.json` | Matriz de actores, decisiones y recuentos | **pendiente** |
| <a id="ev-t042"></a>T042 | `h3/T042/contextos.tsv` | IDs esperados y recuperados por intento y agente | **pendiente** |
| <a id="ev-t043"></a>T043 | `h3/T043/nucleos-lr.tsv` | Ocho núcleos de LR con fuente, archivo y prueba | **pendiente** |
| <a id="ev-t044"></a>T044 | `h3/T044/nucleos-nannu.tsv` | Ocho correspondencias y pasajes de ambos hablantes | **pendiente** |
| <a id="ev-t045"></a>T045 | `h3/T045/nucleos-dan.tsv` | Ocho correspondencias y procedencia de relato e informe | **pendiente** |
| <a id="ev-t046"></a>T046 | `h3/T046/nucleos-evaluador.tsv` | Ocho correspondencias y permisos del evaluador | **pendiente** |
| <a id="ev-t047"></a>T047 | `h3/T047/referente.json` | Turno y recuento de hallazgos de E03 | **pendiente** |
| <a id="ev-t048"></a>T048 | `h3/T048/familia.json` | Turno y cita de procedencia de E04 | **pendiente** |
| <a id="ev-t049"></a>T049 | `h3/T049/dato-ausente.json` | Aviso, tipo de respuesta y ficha antes y después | **pendiente** |
| <a id="ev-t050"></a>T050 | `h3/T050/hablantes.json` | Turnos con ID de hablante y pasaje | **pendiente** |
| <a id="ev-t051"></a>T051 | `h3/T051/cronologias.tsv` | Cronología y fuente por intento antes y después | **pendiente** |
| <a id="ev-t052"></a>T052 | `h3/T052/simultaneas.tsv` | Tiempos de recepción y fin, IDs y conteos de ambos historiales | **pendiente** |
| <a id="ev-t053"></a>T053 | `h3/T053/reinicio-nannu.tsv` | PID, confirmaciones y continuidad de otros agentes | **pendiente** |
| <a id="ev-t054"></a>T054 | `h3/T054/carga-rubrica.json` | Borrador, carga con constancia de prueba y rechazo de huella | **pendiente** |
| <a id="ev-t055"></a>T055 | `h3/T055/intentos-v1-v2.tsv` | Asignación, peso y huellas de A y B | **pendiente** |
| <a id="ev-t056"></a>T056 | `h3/T056/entrega.json` | Acuses, instantánea, IDs y recuentos transaccionales | **pendiente** |
| <a id="ev-t057"></a>T057 | `h3/T057/entrada-evaluador.json` | Inventario autorizado y comparación con la instantánea | **pendiente** |
| <a id="ev-t058"></a>T058 | `h3/T058/criterios.json` | Desglose, citas y recorrido de una observación a su registro | **pendiente** |
| <a id="ev-t059"></a>T059 | `h3/T059/tres-de-cuatro.json` | Cuatro juicios, citas y acción para la cuarta hipótesis | **pendiente** |
| <a id="ev-t060"></a>T060 | `h3/T060/uno-de-cuatro.json` | Cuatro juicios y justificación de la mejora prioritaria | **pendiente** |
| <a id="ev-t061"></a>T061 | `h3/T061/incidencia.json` | Tres juicios contrastados y estado del criterio incompleto | **pendiente** |
| <a id="ev-t062"></a>T062 | `h3/T062/revisiones.json` | Revisiones, motivo y reproducción del desglose de V1 | **pendiente** |
| <a id="ev-t063"></a>T063 | `h3/T063/publicacion.json` | Matriz actor × estado del post × revisión | **pendiente** |
| <a id="ev-t064"></a>T064 | `h3/T064/e22-local.json` | Estados, IDs, toma del trabajo y recuentos antes y después | **pendiente** |
| <a id="ev-t065"></a>T065 | `h3/T065/matriz.tsv` | 32 correspondencias y dos recorridos de trazabilidad | **pendiente** |

### Evidencias del hito 4

| Tarea | Registro previsto | Observación que se archivará | Estado |
| --- | --- | --- | --- |
| <a id="ev-t066"></a>T066 | `h4/T066/pre.md` | Dos recorridos por interfaz, acuses y decisión reabierta | **pendiente** |
| <a id="ev-t067"></a>T067 | `h4/T067/post.md` | Capturas y acuses por orden de post y publicación | **pendiente** |
| <a id="ev-t068"></a>T068 | `h4/T068/secuencias.tsv` | IDs, contador, versiones y formas antes y después | **pendiente** |
| <a id="ev-t069"></a>T069 | `h4/T069/informe.tsv` | Recuentos, puntajes por medida y nota clínica contrastada | **pendiente** |
| <a id="ev-t070"></a>T070 | `h4/T070/origen.md` | Dirección local, captura y respuesta de red del mismo origen | **pendiente** |
| <a id="ev-t071"></a>T071 | `h4/T071/sqlite-init.txt` | Estado de salida, imagen, volumen e inventarios | **pendiente** |
| <a id="ev-t072"></a>T072 | `h4/T072/backend-disponible.json` | Dependencia de arranque, estados y dos acuses | **pendiente** |
| <a id="ev-t073"></a>T073 | `h4/T073/modelo.json` | Etiqueta, digest esperado y local, prueba de generación y estado | **pendiente** |
| <a id="ev-t074"></a>T074 | `h4/T074/reintentos.txt` | Conteo por ciclo, aviso, recuperación y mismo intento | **pendiente** |
| <a id="ev-t075"></a>T075 | `h4/T075/arranque.txt` | Revisión del clon, comando, estados de los cuatro servicios y puertos | **pendiente** |
| <a id="ev-t076"></a>T076 | `h4/T076/restauracion.tsv` | Inventarios antes, tras reinicio y en la copia restaurada | **pendiente** |
| <a id="ev-t077"></a>T077 | `h4/T077/e22-compose.json` | Comparación de IDs y recuentos durante y después de la caída | **pendiente** |
| <a id="ev-t078"></a>T078 | `h4/T078/aprobaciones.md` | Acta docente restringida y correspondencia con las versiones cargadas | **pendiente** |
| <a id="ev-t079"></a>T079 | `h4/T079/recorrido.md` | Secuencia por rol, IDs, captura de la devolución y tiempos del equipo docente | **pendiente** |
| <a id="ev-t080"></a>T080 | `h4/T080/acta.md` | Decisión de aceptación y referencias a cada evidencia | **pendiente** |

### Evidencias de capacidad

| Tarea | Registro previsto | Observación que se archivará | Estado |
| --- | --- | --- | --- |
| <a id="ev-t081"></a>T081 | `capacidad/T081/rr01-20.tsv` | Perfil de 20, muestra por categoría y latencias | **pendiente** |
| <a id="ev-t082"></a>T082 | `capacidad/T082/sesiones-100.tsv` | Perfil de sesiones, mezcla, concurrencia y aislamiento | **pendiente** |
| <a id="ev-t083"></a>T083 | `capacidad/T083/preguntas-100.tsv` | Perfil de ráfaga, tiempos y conciliación de 100 operaciones | **pendiente** |
| <a id="ev-t084"></a>T084 | `capacidad/T084/dictamen.md` | Ratificación por perfil, repetición y decisión de capacidad | **pendiente** |

## Primer avance y aceptación formal

**Primer avance al profesor.** T009–T011 acreditarán que una toma de presión se conserva una sola vez al reenviarla. T012–T016 acreditarán la respuesta libre de LR con la invocación y el pasaje del caso. T017–T019 aportarán el control de indisponibilidad, la recuperación con la misma clave y las mediciones en frío y en caliente. T020 reunirá las capturas, los acuses y los mismos identificadores después de repetir la preparación. Si la primera medición se realiza en desarrollo, el registro indicará su origen y la repetición pendiente en el computador del profesor. [Acta prevista del primer avance](#ev-t020)

**Aceptación formal de LR.** T066–T069 aportarán el pre/post y su informe independiente de la nota clínica. T075 y T076 acreditarán el clon limpio y la recuperación. T077 aportará una medición y una entrega nuevas con Qwen detenido, los mismos identificadores tras el reinicio y exactamente una evaluación terminada. T078 recogerá la aprobación clínica y educativa del docente. T079 vinculará el recorrido real por la interfaz con las evidencias, la versión de rúbrica y la devolución publicada. T080 recogerá el dictamen del profesor y las incidencias que afecten a esa decisión. [Autorización prevista](#ev-t078) · [Recorrido previsto](#ev-t079) · [Acta prevista de aceptación](#ev-t080)
