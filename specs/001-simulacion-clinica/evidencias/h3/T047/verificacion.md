# T047 · Aclarar una pregunta sin referente

26 de noviembre de 2026, 07:25 CLST · `feature/prototipo` · revisión base `fbc3f1ca`

## Procedimiento

Desde `backend/`, ejecutar `cargo build --locked --offline` y
`cargo test --locked --offline --test agente_lr --test mensajes -- --nocapture`.
La prueba `pide_referente_sin_crear_hallazgo` prepara un diálogo vacío,
confirma la selección `sin_hecho` con el validador del turno y compara
ficha, evidencias y traza. También comprueba que el caso especial se
limite a un diálogo sin intercambios.

Para la prueba real, arrancar Ollama y la API como en T040, con una
base nueva. Crear sesión estudiantil e intento LR, resolver el pre
técnico y verificar que `GET /api/intentos/{id}` tenga cero turnos.
Enviar `{"texto":"¿Y eso?"}` a `POST /api/intentos/{id}/turnos` con
`Idempotency-Key: t047-ollama`. Volver a consultar el intento y comparar
ficha y evidencias. Leer la traza con la identidad docente asignada y
cotejar la selección de Qwen. Contar los hallazgos citables sin contar
la orientación disponible desde el pre.

## Resultado

La operación confirmó un turno de LR con «¿A qué te refieres con eso?».
El tipo de respuesta fue `aclaracion`. La traza registró `sin_hecho`
como selección de Qwen, `referente_ambiguo` como motivo y ningún hecho
asociado. La ficha y las evidencias se mantuvieron iguales. El recuento
de hallazgos citables fue cero antes y cero después. El turno fue
`81ec87874af9e7ebab69ec94f953d944`. Axum devolvió 201, Ollama
confirmó CPU y pasaron las once pruebas Rust.

[Acuse HTTP, recuentos y traza](referente.json) · [Prueba y regresiones Rust](rust.txt).
