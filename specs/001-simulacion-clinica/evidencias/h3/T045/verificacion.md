# T045 · Dan Sparks y procedencia del ECG

26 de noviembre de 2026, 07:20 CLST · `feature/prototipo` · revisión base `fbc3f1ca`

## Procedimiento

Desde `backend/`, ejecutar `cargo build --locked --offline` y
`cargo test --locked --offline --test agente_dan_sparks --test agente_nannu --test preparacion --test contexto -- --nocapture`.
La batería prepara SQLite desde las fuentes del repositorio, habilita P03
dentro de la base temporal, abre un intento y resuelve el pre técnico.
Alterna las fichas de P01 y P03, resuelve la misma sugerencia literal,
coteja la fuente del actor y consulta la disponibilidad del medio
declarado por el controlador. Comprueba la reserva HTTP del ECG y el
rechazo de una selección ajena. Revisar los seis campos y enlaces de
cada README con el inventario.

## Resultado

La respuesta de Dan situó el inicio hace cuatro horas mientras trabajaba
en el jardín. Su pasaje de origen fue la línea 29 del guion del actor
P03. La referencia al JPG del ECG quedó vinculada a la revisión del
controlador y se registró como `indisponible`. La consulta sin orden
devolvió 404 y omitió el medio. Usar la misma pregunta literal en
P01 y P03 recuperó un hecho de cada ficha. Se verificaron los ocho README
y pasaron las 18 pruebas Rust, incluidas las regresiones de Nannu,
preparación y memoria.

[Correspondencias de los núcleos](nucleos-dan.tsv) ·
[Pasaje y disponibilidad del medio](procedencia.json) · [Salida Rust](rust.txt).
