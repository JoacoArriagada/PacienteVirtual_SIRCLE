# Contrato HTTP previsto para el módulo 01

La interfaz llama a `/api` en su propio origen. El servicio frontend reenvía a Axum por la red de Compose. La API intercambia JSON y mantiene una sesión local mediante una cookie HttpOnly. La identidad ficticia, el rol, la asignación docente y el propietario efectivo proceden de la sesión. El ensayo independiente del hito 1 podrá usar HTTP directo al backend. El chat extraído en el hito 2 usará el mismo contrato. Los controles se aplican antes de devolver un recurso, un acuse previo o datos para inferencia. [Modelo de datos](../data-model.md)

## Operaciones

| Operación | Petición y resultado observable |
| --- | --- |
| `POST/GET /api/sesiones` · `GET /api/actividades` | Seleccionar una identidad ficticia preconfigurada o recuperar la sesión local. Listar solo sus actividades habilitadas y el estado necesario para «En curso → Continuar». Antes del pre, la tarjeta de LR carece de introducción, ficha y hechos clínicos. |
| `POST /api/intentos` · `GET /api/intentos/{id}` | La actividad clínica asignada solo admite nuevas altas con sus versiones aprobadas y cargadas. El intento fija su rúbrica al abrirse. El ensayo con rúbrica habilitada devuelve `rubrica_id`, `version_rubrica` y `rubrica_sha256`, con los criterios en el almacén privado. El modo técnico anterior sigue disponible para pruebas sin evaluación. La respuesta devuelve fase, revisión y estado de las encuestas. Tras cerrar el pre, la vista autorizada expone introducción, ficha, indicaciones, ayuda, turnos, apuntes y evidencias disponibles. La ayuda indica su origen docente. |
| `GET` y `PUT /api/intentos/{id}/encuestas/{momento}` | `momento` vale `pre` o `post`. Leer la forma asignada y confirmar respuestas u omisión. La respuesta incluye acuse, versión y momento. Se rechaza un pre tardío, un post anterior a la entrega y un cambio retroactivo. Los ítems llegan sin claves. |
| `POST /api/intentos/{id}/turnos` | Enviar texto y, si procede, identificador de sugerencia. El servidor coteja la sugerencia con la ficha de la versión del intento. Una pregunta propia o una sugerencia editada pasa al agente de LR en P01 y requiere una llamada a Qwen local. Devuelve el turno, hablante y tipo de respuesta. Un hecho de otro caso, fase o hablante se descarta. La traza docente no forma parte del turno público. |
| `POST /api/intentos/{id}/mediciones` · `GET /api/intentos/{id}/mediciones` | Confirmar una toma completa o consultar el histórico del instrumento. El servidor autoriza propietario y fase, valida la acción y calcula el valor desde la versión del caso. Devuelve identificador, valor o hallazgo, unidad, momento y estado de registro. Una cancelación previa a la toma completa no genera lectura. |
| `PUT /api/intentos/{id}/mediciones/{medicion}/ficha` | Registrar en ficha una lectura existente del propio intento. La lectura conserva su identificador, el histórico conserva una fila y los reenvíos recuperan el mismo estado. |
| `POST /api/intentos/{id}/exploraciones` · `GET /api/intentos/{id}/exploraciones` | Confirmar el gesto completo sobre una zona del instrumento y recuperar sus acciones. La respuesta conserva zona, fase y momento. Solo la zona habilitada incluye un hallazgo con ID citable. En una zona pendiente se confirma la acción y se informa el estado. |
| `POST /api/intentos/{id}/ordenes` · `GET /api/intentos/{id}/ordenes/{orden}` | Confirmar una orden del ECG inicial habilita el informe textual «taquicardia sinusal». Un apunte o una orden sin confirmar mantiene el informe reservado. La orden confirmada es citable y su reenvío recupera el mismo identificador. |
| `PUT /api/intentos/{id}/razonamiento` | Guardar apuntes, hipótesis principal, alternativas, explicación, vínculos a registros y revisión esperada. Una evidencia de otro intento o una revisión obsoleta se rechazan. Admite guardar razonamientos parciales. |
| `POST /api/intentos/{id}/entrega` | Confirmar por `revision_esperada` el trabajo guardado, incluso si está parcial, y congelarlo con el identificador, la versión y la huella de la rúbrica del intento. Responde con el acuse y el estado de evaluación al confirmar, aun si Qwen está caído. El reenvío recupera el mismo acuse y el titular puede releerlo en la vista del intento. El post queda disponible. |
| `GET /api/intentos/{id}/devolucion` | El titular ve el estado permitido y, una vez resuelto el post, el contenido de la revisión publicada. Una devolución ya publicada sigue reservada mientras el post esté pendiente. |
| `GET /api/actividades/{id}/entregas` · `GET/PATCH /api/evaluaciones/{id}` · `POST /api/evaluaciones/{id}/publicacion` | El docente asignado puede leer la versión y huella de la rúbrica congelada, el desglose y las observaciones por criterio. Puede rectificar la evaluación con motivo y revisión esperada y publicar la revisión elegida cuando se resuelvan los criterios pendientes. Se conservan las revisiones anteriores. |
| `GET /api/evaluaciones/{id}/traza` | El docente asignado recupera la entrega, la rúbrica, la operación del evaluador, la invocación del modelo y el borrador guardado. |
| `GET /api/docente/actividades` · `GET /api/evaluaciones/{id}/revisiones/{revision}` | La supervisión lista las actividades a cargo del docente. Una revisión histórica conserva su propio autor, motivo, juicios, citas, créditos y total. |
| `GET /api/evaluaciones/{id}/criterios/{criterio}/traza` · `GET /api/intentos/{id}/turnos/{turno}/traza` | El revisor autorizado obtiene la cadena desde la cita hasta el evento y la versión de la fuente, el agente y la regresión asociada. Una incidencia indica qué evidencia se esperaba y permite localizar su registro técnico. |
| `GET /api/intentos/{id}/trazas-rechazadas` | El docente asignado recupera los rechazos del intento con su operación, agente, motivo y los datos de inferencia y procedencia disponibles. |
| `GET /api/actividades/{id}/resultados-educativos` | El docente asignado recibe invitados, pares completos, faltantes, percepción por ítem y razonamiento por forma, secuencia y momento. Las ponderaciones de la nota clínica proceden exclusivamente de la rúbrica. |
| `GET /api/estado` | Comunica por separado disponibilidad de la base y del modelo, junto con una indicación comprensible de recuperación. El servicio mantiene las operaciones respaldadas por SQLite durante un fallo de inferencia. |

## Pregunta libre por HTTP y desde el chat

Con sesión ficticia, caso P01 y pre cerrado, el ensayo enviará a `POST /api/intentos/{id}/turnos` un cuerpo como `{"texto":"¿Cuándo empezaste a notar que te faltaba el aire?"}`, sin identificador de sugerencia y con `Idempotency-Key` nueva. Se espera un turno de ese intento, atribuido a LR en primera persona, que sitúe el inicio hace dos días. El revisor correlacionará la operación, la llamada completada a Ollama, el modelo y el hecho con su versión y pasaje. La vista estudiantil mostrará la pregunta, la respuesta autorizada y el hablante. El detalle de la invocación quedará en la traza restringida. [Ensayo de avance técnico](../quickstart.md#conversación-libre-con-lr-por-http)

Durante la caída, repetir el mismo texto como **otra operación**, con clave nueva, debe devolver 503 y dejar intacto el historial confirmado. El acuse de una operación ya completada seguiría disponible por idempotencia. Al reintentar la operación fallida tras la recuperación, su clave deberá producir un solo turno. Esta regla se aplica tanto a la petición directa como al chat.

Si llega un reenvío mientras el turno sigue en curso, el servidor devuelve HTTP 202 con `{"estado":"en_curso","operacion_id":"<id>"}` y `Retry-After: 1`. Pasado ese intervalo, el cliente consulta el acuse con el mismo POST, cuerpo y clave. La misma clave con contenido diferente devuelve 409.

En un intento P02 de ensayo, `destinatario` admite `Nannu` o `Acompañante`. Omitirlo dirige la pregunta a Nannu. El servidor filtra hechos y memoria por la voz elegida y conserva el hablante en el turno. La clave de operación identifica también el destinatario cuando se envía.

## Medición de referencia

Para una toma de presión completa del intento ficticio `i1`, el cliente envía `{"instrumento":"presion","accion":"toma_completa"}` con una clave de operación. En la versión de valoración inicial acordada, la respuesta contiene, entre otros campos, `{"medicion_id":"m1","intento_id":"i1","valor":{"sistolica":148,"diastolica":85},"unidad":"mmHg","en_ficha":false}`. La acción «Registrar en ficha» cambia `en_ficha` a verdadero sobre `m1`. El pulso de 102/min corresponde a la vista de pulso. El docente debe poder llegar a `m1` desde una observación que cite esa medición. Los identificadores del ejemplo son ficticios.

## Orden del ECG inicial

La confirmación envía `{"examen":"ecg-inicial","accion":"confirmar"}` con `Idempotency-Key`. El acuse devuelve `orden_id`, `intento_id`, `fase`, `momento` y revisión. El intento recupera las órdenes confirmadas tras resolver el pre. El informe se consulta por `orden_id` o por el alias `ecg-inicial` dentro del propio intento. Sin orden confirmada, la consulta devuelve 404 con el estado reservado. Tras confirmarla, la consulta devuelve `informe_id`, `orden_id`, texto y origen.

## Apuntes de la atención

Para añadir un apunte, `PUT /api/intentos/{id}/razonamiento` recibe `{"revision_esperada":4,"apunte":"solicitaría el ECG"}` con una clave de operación. Devuelve el identificador del apunte, autor, intención, momento y revisión confirmada. La acción escribe el apunte y mantiene la disponibilidad del informe según el estado de la orden. El intento devuelve sus `apuntes` y sus `evidencias` tras cerrar el pre. Cada evidencia conserva `intento_id`, `tipo`, `origen_id`, categoría, autor y origen. El informe integra esta lista tras su primera consulta.

## Borrador de hipótesis

La misma ruta recibe `revision_esperada` y `hipotesis` al guardar el borrador. Cada hipótesis lleva `rol` (`principal` o `alternativa`), `titulo`, `explicacion` y una lista de `evidencias` con pares `tipo` y `origen_id`. Los vínculos se resuelven dentro del intento y una referencia ajena o inexistente devuelve 422. Se puede guardar una lista vacía o hipótesis parciales. El acuse contiene la revisión confirmada y las hipótesis, y la vista del intento devuelve la última revisión confirmada en `borrador`. Una revisión esperada obsoleta devuelve 409. El reenvío de una operación confirmada recupera su acuse original.

## Confirmación y errores

Las mutaciones académicas reintentables llevan `Idempotency-Key`. La misma intención conserva la clave al perder la respuesta, mientras una nueva toma deliberada recibe otra. Tras verificar el acceso actual, la API revisa la clave antes de rechazar por cambio de fase o revisión. Repetir una operación ya confirmada devuelve el mismo acuse e identificadores. Una operación todavía en curso devuelve su estado y una referencia para consultarlo o reintentarlo, sin iniciarla por segunda vez.

| Situación | Respuesta prevista |
| --- | --- |
| Creación confirmada · 201 | Acuse con identificadores y revisión. Una repetición ya confirmada puede responder 200 con los mismos identificadores. La entrega se confirma aunque el cálculo de su evaluación siga pendiente. |
| Solicitud en curso · 202 | Indica procesamiento pendiente. La interfaz muestra éxito solo al recibir el acuse de confirmación. |
| Sesión ausente o permiso insuficiente · 401/403 | Indica cómo volver al acceso disponible. La respuesta omite datos del recurso ajeno. |
| Estado incompatible, clave con otro contenido o revisión obsoleta · 409 | Indica la operación en conflicto y permite recargar la vista autorizada. La entrega y sus evidencias ya confirmadas se conservan. |
| Entrada inválida · 422 | Identifica el campo o el gesto que requiere corrección sin guardar un evento inválido. |
| Fallo temporal del diálogo libre · 503 | Informa indisponibilidad y opción de reintento. Una selección no válida del modelo no se convierte en un hecho clínico. Una solicitud fallida puede recuperarse sin duplicar el turno. |

La pregunta ambigua da lugar a una aclaración de la paciente. El desconocimiento familiar de LR se expresa en primera persona y el dato ausente genera un aviso del escenario. Son resultados válidos de entrevista, distinguibles de un error técnico. [E03 a E06](../spec.md#historia-2--entrevistar-a-lr-e-interpretar-los-límites-de-su-relato-p1)
