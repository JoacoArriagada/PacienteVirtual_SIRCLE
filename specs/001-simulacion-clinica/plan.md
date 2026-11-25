# Plan técnico del módulo 01 · Paciente Virtual SIRCLE

**Rama de trabajo** `feature/prototipo` · **Especificación** `001-simulacion-clinica` · **Fecha** 2026-11-25

## Objetivo y contexto técnico

El avance técnico se demostrará con material de prueba identificado. La aceptación formal exigirá el recorrido completo de LR con la versión clínica aprobada en el computador del profesor. Ambas comprobaciones están previstas. [Especificación aclarada](spec.md)

El monorrepositorio usará HTML, CSS y TypeScript sin framework, Rust con Axum, una API REST con JSON, SQLite como única base y un motor Ollama local con `qwen3.5:4b`. La atribución al acompañante de Nannu responde a un requisito aprobado de P02. La precisión del principio I sigue pendiente de revisión por el procedimiento de enmienda. [Gobierno y enmiendas](../../.specify/memory/constitution.md#gobierno-y-enmiendas) · [Revisión de los seis principios](research.md#revisión-constitucional-antes-y-después-del-diseño)

## Hito 1. Preparar aplicaciones independientes y probar la conversación libre con LR

Crear `frontend/` y `backend/` en la raíz, con instalación, arranque y pruebas independientes. Agrupar en `src/modules/` acceso, intentos, entrevista, valoración, exámenes, razonamiento, encuestas y devolución. Compartir código cuando dos módulos lo necesiten. En valoración, el frontend representa el gesto del brazalete y envía la toma por HTTP. El backend valida propietario y fase. SQLite guarda lectura, unidad y momento. «Registrar en ficha» enlaza esa lectura. [Contrato HTTP](contracts/api.md)

Preparar las migraciones en `backend/migrations/` e importar los 14 originales de LR, Nannu y Dan Sparks (4, 4 y 6) con ruta, rol, huella y versión. Antes de extraer el chat, usar un intento ficticio con el pre cerrado y hechos de LR identificados por fuente, pasaje, versión, fase y hablante. Enviar a `POST /api/intentos/{id}/turnos` una pregunta propia, distinta de las sugerencias, por ejemplo «¿Cuándo empezaste a notar que te faltaba el aire?». El backend consultará a Qwen local y validará el hecho elegido antes de guardar el turno. [Modelo de datos](data-model.md)

**Criterio de avance técnico.** LR deberá responder en primera persona y situar el inicio hace dos días. La traza enlazará intento, turno, llamada real a Ollama y pasaje. Con inferencia detenida, reenviar la pregunta como operación nueva deberá indicar indisponibilidad. Registrar equipo, modelo y tiempos exploratorios en frío y en caliente. Si se ensaya en desarrollo, consignarlo y repetir en el computador del profesor en la primera oportunidad. Mostrar la toma con datos de ejemplo y reenviarla por HTTP. Al repetir la preparación deberán recuperarse tres casos, 14 fuentes y el intento con su turno y una única medición. [Ensayo de entrevista](quickstart.md#conversación-libre-con-lr-por-http)

## Hito 2. Extraer la interfaz conservando sus gestos

Tras el ensayo por HTTP del hito 1, copiar `docs/referencias/html-semana5/Paciente_Virtual.html` y `recursos/` a `frontend/referencia/semana5/`, de uso privado. Conservar el original y cotejar las 29 rutas relativas y sus SHA-256 antes de editar la copia. Conservar marcado, selectores, Roboto, paleta, iconos, expresiones, instrumentos y audio al extraer CSS y distribuir el comportamiento en TypeScript. El paquete público excluirá hechos reservados, claves y rúbricas.

**Se avanza** al comparar a igual resolución selección, chat, brazo ampliado, colocación, lectura e histórico, incluido el recorrido por teclado. Cancelar deja cero lecturas y reenviar el registro conserva una. Conectar el chat al mismo contrato de turnos del hito 1, recuperar la conversación y repetir la pregunta libre desde la interfaz con su traza. [Inventario de referencia](research.md#referencia-comprobada-en-el-clon)

## Hito 3. Integrar los cuatro agentes y calibrar la devolución

Con entrevista y valoración verificadas, integrar cuatro trabajadores independientes bajo `backend/src/agentes/` en `lr/`, `nannu/`, `dan_sparks/` y `evaluador/`. Cada uno conservará `01_rol_objetivo_modelo/`, `02_fuentes_recuperacion_memoria/`, `03_herramientas_mcp/`, `04_identidad_acceso/`, `05_controles_entrada_contexto_salida/`, `06_cache/`, `07_enrutamiento/` y `08_observabilidad_regresion/`. Comprobar los 32 README y sus enlaces a implementación y prueba, incluso al compartir código. Reconstruir el historial por estudiante, intento y agente. Nannu y Dan Sparks verifican identidad, hablante y aislamiento. [Matriz de agentes](contracts/agentes.md)

Preparar una rúbrica privada por versión en `datos/adaptaciones/lr/rubricas/`. El docente revisará criterios y ponderaciones de pregrado avanzado. Antes de habilitar la actividad, el preparador cargará en SQLite la versión, su aprobación fechada y la huella. Cada entrega conservará la versión fijada al abrir el intento. Cambiar ponderaciones creará otra versión y preservará el cálculo reproducible de entregas anteriores. Un editor gráfico queda para una necesidad posterior.

El evaluador revisará entrevista, valoración, decisiones, evidencias e hipótesis. Por criterio explicará actuación, evidencia, expectativa, juicio y mejora. Un registro técnico ausente requerirá revisión docente. En E12 reconocerá tres aciertos de cuatro y explicará la cuarta hipótesis en una entrega. En la de un acierto identificará tres brechas y priorizará una corrección. **Se avanza** al comprobar ambas calibraciones, rastrear un fallo hasta su fuente, aislar dos entrevistas simultáneas a LR (E17) y recuperar una sola evaluación tras registrar una medición nueva y confirmar la entrega durante la caída de Qwen (E22).

## Hito 4. Aceptar el recorrido en el computador del profesor

Superado el hito 3, `docker compose up --build` arrancará desde un clon limpio sin credenciales. `frontend`, `backend` e `inferencia` permanecerán activos. `sqlite-init` preparará la base y terminará. Datos y pesos tendrán volúmenes propios. Solo la interfaz publicará en localhost. Si faltan los pesos, se descargarán del registro oficial y se verificarán identidad y carga, con tres intentos por ciclo.

**Criterio de aceptación formal.** Antes de habilitar la actividad, el docente aprobará la ficha española, los hechos e instrumentos de la fase, la redacción del informe de ECG, la rúbrica, sus ponderaciones y las claves educativas.

En su computador completará con identidad ficticia el pre opcional de cinco preguntas, la entrevista, la valoración inicial (148/85 mmHg y pulso de 102/min), la orden de ECG y su informe textual de taquicardia sinusal, la hipótesis principal contrastada con al menos una alternativa, la entrega y el post opcional de siete preguntas. El acta enlazará la versión aprobada, las evidencias del intento, la devolución publicada y visible tras el post, el informe pre/post por forma y orden separado de la nota clínica y la recuperación E22 en Compose. El docente podrá rectificar con motivo y decidirá la publicación. Los ensayos de 20 usuarios, 100 sesiones abiertas y 100 preguntas simultáneas quedan para el estudio posterior. [Guía de aceptación](quickstart.md)
