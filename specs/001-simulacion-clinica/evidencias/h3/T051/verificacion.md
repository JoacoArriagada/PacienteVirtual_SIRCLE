# T051 · Alternar la cronología de LR y Dan Sparks

26 de noviembre de 2026, 07:39 CLST · `feature/prototipo` · revisión base `fbc3f1ca`

## Procedimiento

En `backend/`, ejecutar `cargo build --locked --offline` y
`cargo test --locked --offline --test agente_dan_sparks --test agente_nannu --test agente_lr --test contexto -- --nocapture`.
Preparar una base temporal e incluir P03 solo en la actividad de ensayo:

```sql
PRAGMA foreign_keys=ON;
UPDATE actividades SET habilitada=1 WHERE id='dan-ensayo';
INSERT INTO asignaciones VALUES ('dan-ensayo','estudiante-lr','docente-demo');
```

Arrancar Axum y Ollama como en T040. Con una sesión estudiantil, crear
los intentos de `lr-ensayo` y `dan-ensayo`, omitir sus pre de ensayo y
consultar ambos historiales iniciales. Enviar
`{"texto":"¿Desde cuándo comenzó a faltarte el aire?"}` primero a LR y
luego a Dan por su ruta de turnos. Usar en ambos la clave
`t051-misma-pregunta`. Leer cada traza con el docente y reabrir
los dos intentos desde una nueva sesión del mismo estudiante.

## Resultado

LR situó el inicio hace dos días. Dan lo situó hace cuatro horas,
mientras trabajaba en el jardín. Cada turno se confirmó con su voz,
caso y pasaje de actor versionado. Antes de sus preguntas los dos
historiales estaban vacíos. Al reabrir, cada intento conservó un solo
turno y el mismo ID confirmado. Las operaciones, turnos, hechos y
revisiones de fuente fueron distintos. Qwen respondió por CPU. Pasaron
31 pruebas de Rust.

[Cronologías e identificadores](cronologias.tsv) ·
[Acuses y trazas](cronologias.json) · [Regresiones Rust](rust.txt).
