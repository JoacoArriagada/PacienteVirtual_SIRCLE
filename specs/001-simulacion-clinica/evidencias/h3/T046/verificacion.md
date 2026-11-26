# T046 · Entrada y salida del evaluador formativo

26 de noviembre de 2026, 07:23 CLST · `feature/prototipo` · revisión base `fbc3f1ca`

## Procedimiento

Desde `backend/`, ejecutar `cargo build --locked --offline` y
`cargo test --locked --offline --test agente_evaluador --test trabajadores --test visibilidad -- --nocapture`.
Cotejar las ocho fichas con sus seis campos y sus enlaces. Usar el
ejemplo de contrato para serializar y deserializar un mensaje del
evaluador. Alterar por separado la confirmación, la huella, el ID
de entrega, la cita, las partes de una observación y el rol autorizado.
Verificar que el contrato de salida produzca una observación por criterio
y que la publicación requiera la asignación docente.

## Resultado

Las ocho fichas y sus enlaces se verificaron. La entrada confirmó entrega,
revisión, caso, rúbrica congelada con huella, hipótesis y registros propios.
La salida conservó una observación por criterio con su expectativa, cita
y explicación. Las referencias ajenas, la huella distinta y el intento
sin confirmar se rechazaron. Los campos de encuestas o publicación no
pertenecen a los mensajes del trabajador. La autorización identificó al
docente asignado y rechazó la sesión estudiantil. Pasaron diez pruebas
de Rust. La prueba usa un contrato de ensayo. T057 y T058 conectarán la
instantánea y el trabajo persistido. T065 comprobará las rutas con la
calibración y revisión completas.

[Inventario de fichas](nucleos-evaluador.tsv) · [Ejemplo de contrato](contrato.json) ·
[Salida Rust](rust.txt).
