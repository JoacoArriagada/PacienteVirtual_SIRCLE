# T050 · Alternar las dos voces de P02

26 de noviembre de 2026, 07:36 CLST · `feature/prototipo` · revisión base `fbc3f1ca`

## Procedimiento

Desde `backend/`, ejecutar `cargo build --locked --offline` y
`cargo test --locked --offline --test agente_nannu --test agente_lr --test mensajes --test trabajadores -- --nocapture`.
Para la prueba de Qwen, preparar una base temporal nueva. Habilitar P02
y asignarlo solo en esa copia de ensayo:

```sql
PRAGMA foreign_keys=ON;
UPDATE actividades SET habilitada=1 WHERE id='nannu-ensayo';
INSERT INTO asignaciones VALUES ('nannu-ensayo','estudiante-lr','docente-demo');
```

Arrancar Axum y Ollama como en T040. Abrir el intento de `nannu-ensayo`,
resolver el pre técnico y enviar con claves distintas a
`POST /api/intentos/{id}/turnos`:

```json
{"texto":"¿Qué dolor sientes cuando respiras?","destinatario":"Nannu"}
{"texto":"¿La ha visto toser y expulsar flema?","destinatario":"Acompañante"}
```

Consultar cada traza con el docente y reabrir el intento con una sesión
estudiantil nueva. También se envía al validador un mensaje de ensayo
que identifica a la paciente pero contiene el candidato del acompañante.
Cotejar SQLite y la vista estudiantil tras el rechazo.

## Resultado

El relato del dolor quedó en la voz de Nannu y la tos con flema en la del
acompañante. Ambas respuestas 201 conservaron el mismo intento, versión
P02 y sus propios IDs. Las trazas y los contextos enviados a Ollama
conservaron los dos fragmentos de origen y excluyeron a la otra voz
respectivamente. Reabrir devolvió los mismos dos turnos y dos declaraciones.
Qwen se ejecutó por CPU. La selección cruzada se descartó con el motivo
`seleccion_fuera_de_hablante`, conservó la revisión y dejó cero turnos
en el intento de esa prueba. Pasaron 23 pruebas Rust.

En la primera corrida, el turno de Nannu se confirmó y la pregunta al
acompañante devolvió `sin_hecho`. La API rechazó la selección. Se incorporó
el destinatario autorizado al mensaje de Qwen y se aclararon el tratamiento
de pronombres y las condiciones de la tos. Se repitieron las dos preguntas
sobre otra base nueva. Los turnos finales fueron
`4018483b955f57a234504d928584743f` y `d56e590672496bc7238a0b02a240dd59`.

[Turnos y trazas](hablantes.json) · [Selección cruzada](cruce.json) · [Salida Rust](rust.txt).
