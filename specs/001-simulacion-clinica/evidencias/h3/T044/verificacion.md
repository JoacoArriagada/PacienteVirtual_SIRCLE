# T044 · Perfil de Nannu y atribución por hablante

26 de noviembre de 2026, 07:16 CLST · `feature/prototipo` · revisión base `fbc3f1ca`

## Procedimiento

Desde `backend/`, ejecutar `cargo build --locked --offline` y
`cargo test --locked --offline --test agente_nannu --test contexto --test preparacion --test agente_lr -- --nocapture`.
Cotejar las ocho filas del inventario con sus respectivos README. Cada
ficha lleva los seis campos del contrato y enlaces al guion, archivo
de implementación y prueba. La batería habilita y asigna P02 en su base
temporal para obtener un intento de ensayo. Construye la respuesta para
cada voz y compara el ID de hecho, caso, fase, versión, hablante, revisión
de fuente y fragmento. Intenta seleccionar un hecho del acompañante en el
contexto de la paciente, compara sus sugerencias y reabre SQLite.

## Resultado

Se verificaron las ocho fichas y sus enlaces. El dolor de pecho al
respirar quedó en la voz de Nannu y la tos con flema en la del acompañante.
Los dos candidatos pertenecen a P02, fase inicial y a una misma versión
de ensayo. Sus pasajes corresponden a fragmentos distintos de la línea 5
del guion del actor. La selección cruzada se rechazó. La ficha del
acompañante admitió la sugerencia de tos y la de la paciente conservó su
propia sugerencia. La actividad preparada sigue identificada como ensayo.
T050 comprobará su atribución en turnos completos por HTTP y T053 la
recuperación de una operación tras interrumpir al trabajador.

Se corrigió la conversión del número de línea de SQLite al índice usado en
la prueba. Después se repitió la batería y pasaron las 19 pruebas de Rust.

[Inventario de los ocho núcleos](nucleos-nannu.tsv) · [Hechos atribuidos y pasajes](atribucion.json) ·
[Salida Rust](rust.txt).
