# T039 · Correlación de mensajes de entrevista

26 de noviembre de 2026, 00:19 CLST · `feature/prototipo` · revisión base `fbc3f1ca`

## Procedimiento

Desde `backend/`, ejecutar:

```bash
cargo build --locked --offline
cargo test --locked --offline --test mensajes --test agente_lr --test visibilidad -- --nocapture
```

La prueba serializa una solicitud con cabecera de LR y candidatos de su
intento. Deserializa una respuesta válida y la envía al mismo validador
que ejecuta Axum antes de persistir. Después altera individualmente cada
campo, vuelve a deserializar y compara el recuento, contenido y revisión
del historial por HTTP. Consulta en SQLite la fila de cada rechazo y
recupera la cabecera de la respuesta válida con la sesión docente.

Para la comprobación real, mantener Ollama de T013 en
`http://127.0.0.1:11434`. Preparar una base nueva con
`target/debug/sircle preparar --db "$DB" --fuentes ../datos` y arrancar
`target/debug/sircle servir --db "$DB" --direccion 127.0.0.1:18081` desde
`backend/`. Crear sesión `estudiante-lr` y un intento para `lr-ensayo`.
Resolver el pre desde la consola con
`target/debug/sircle resolver-pre-ensayo --db "$DB" --intento "$ID"`.
Enviar a `POST /api/intentos/$ID/turnos` una clave `t039-ollama` y
`{"texto":"¿Hace cuánto empezaste a sentir que te faltaba el aire?"}`.
Consultar el intento y la traza por su `turno_id` con `docente-demo`.
Comparar operación, intento, agente, versión, fase, hablante y digest con
el acuse, la vista del intento y `GET /api/tags` de Ollama. Comprobar CPU
mediante `GET /api/ps` y `size_vram=0`.

## Resultado

La respuesta válida dejó un turno y su cabecera correlacionada en la
traza docente. Los cambios de operación, intento, agente, caso, versión,
fase, hablante, tipo e ID interno de la invocación quedaron registrados
como nueve descartes en SQLite. Tras cada descarte, el historial y la
revisión del intento siguieron iguales. Pasaron las tres pruebas Rust.

La pregunta libre por HTTP devolvió 201 con la respuesta de LR sobre el
inicio hace dos días. La cabecera y la invocación coincidieron con el turno
que se recuperó al reabrir. La petición real final tardó 7.854 ms. Ollama confirmó el digest de Qwen de T013 y que
el proceso tenía `size_vram=0`. T040 conectará este contrato a los cuatro
procesos supervisados.

[Solicitud, respuesta y descartes](correlacion.json) ·
[Acuse y traza de Qwen por CPU](ollama-real.json).
