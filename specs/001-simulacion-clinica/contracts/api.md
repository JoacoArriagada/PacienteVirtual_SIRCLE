# Contrato HTTP previsto para el módulo 01

La interfaz llama a `/api` en su propio origen. El proxy reenvía a Axum. El servidor establece una sesión de demostración con cookie HttpOnly y comprueba rol, propietario, actividad y fase en cada operación protegida. Las identidades se preparan localmente. Cada escritura reintentable recibe `Idempotency-Key`.

| Operación | Petición y resultado esperado |
|---|---|
| `POST /api/sesiones` y `GET /api/actividades` | Seleccionar una identidad ficticia preconfigurada y listar sus actividades. La lista anterior al pre omite datos clínicos. |
| `POST /api/intentos` y `GET /api/intentos/{id}` | Iniciar con una actividad asignada o recuperar la vista autorizada, su revisión y el estado del pre/post. |
| `GET/PUT /api/intentos/{id}/encuestas/{momento}` | Leer la forma asignada y confirmar respuestas u omisión. El pre tardío se rechaza. Un reenvío recupera el mismo registro. |
| `POST /api/intentos/{id}/turnos` | Enviar texto y, si procede, ID de sugerencia. Devuelve turno y tipo de respuesta. La sugerencia editada sigue la ruta de texto libre. La traza de procedencia completa queda para el revisor autorizado. |
| `POST /api/intentos/{id}/mediciones` y `POST /api/intentos/{id}/ordenes` | Recibir instrumento/gesto confirmado u orden. El servidor resuelve valor, unidad y fase. Confirmar el ECG inicial devuelve la referencia de su informe textual. Un apunte carece de ese efecto. |
| `PUT /api/intentos/{id}/razonamiento` | Guardar apuntes, hipótesis, alternativa, evidencia y revisión esperada. Rechazar referencias a otro intento o una revisión obsoleta. |
| `POST /api/intentos/{id}/entrega` | Congelar el intento y responder con el mismo acuse en los reenvíos, incluso con Ollama caído. Estado inicial de evaluación pendiente. |
| `GET /api/intentos/{id}/devolucion` | Estudiante titular con post cerrado recibe la versión publicada. El estado visible puede indicar que espera revisión docente. |
| `GET/PATCH /api/evaluaciones/{id}` y `POST /api/evaluaciones/{id}/publicacion` | Docente asignado consulta, corrige con motivo y publica la revisión elegida. La API valida las citas por criterio y conserva el historial. |
| `GET /api/actividades/{id}/resultados-educativos` | Docente asignado consulta invitados, pares completos, faltantes, percepción por ítem y razonamiento por forma y orden. |

Usar 401 para sesión ausente, 403 para acceso denegado, 409 para conflicto de estado y 422 para entradas inválidas. Un fallo temporal de inferencia produce 503 con un mensaje recuperable. Una evaluación pendiente se comunica en el acuse de entrega. La API de diagnóstico distingue disponibilidad de persistencia e inferencia.
