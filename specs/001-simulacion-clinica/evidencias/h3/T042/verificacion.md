# T042 · Memoria por estudiante, intento y agente

26 de noviembre de 2026, 00:32 CLST · `feature/prototipo` · revisión base `fbc3f1ca`

## Procedimiento

Desde `backend/` ejecutar:

```bash
cargo build --locked --offline
cargo test --locked --offline --test contexto --test mensajes --test hechos_lr -- --nocapture
```

La prueba abre dos intentos de LR con dos identidades y registra un turno
literal en cada uno usando la misma clave. Añade un intento de P02 de
ensayo con turnos de Nannu y su acompañante y dos hechos apoyados en
pasajes del actor. Agrega un hecho de otra fase, un hecho de otra versión,
un turno posterior y un turno de agente ajeno. Alterna peticiones
entre los cuatro contextos, reconstruye el router sobre la misma base,
reabre SQLite y compara el resultado con el previo al reinicio.

Para comprobar la salida al motor real, abrir un intento nuevo en Axum
como en T040. Resolver el pre, enviar la sugerencia literal con
`{"texto":"¿Desde cuándo comenzó?","sugerencia_id":"inicio"}` y luego una
pregunta libre de inicio con otra clave. Consultar la traza docente del
segundo turno y buscar `intercambios_confirmados` en la invocación.
Consultar `/api/ps` de Ollama para confirmar CPU. Como regresión de
migración, copiar en frío la base de H1 con `sqlite3 .backup`, preparar
la copia y comparar IDs de turnos antes y después.

## Resultado

Las dos identidades de LR conservaron turnos propios con identificadores
distintos. En el mismo estudiante, cambiar de P01 a P02 recuperó la
memoria del segundo intento. Cambiar entre Nannu y acompañante seleccionó
el turno y el hecho de la voz solicitada. Los cuatro registros testigo
ajenos a fase, versión o agente quedaron fuera. Después de reconstruir
el router y abrir otra conexión SQLite, los cuatro contextos
coincidieron. La misma clave siguió aislada por identidad e intento.

La prueba real añadió un turno de Qwen local al turno literal previo.
La invocación de CPU recibió exactamente un intercambio de LR. El
resultado persistido fue el turno `53e9658eea81ee5b570e0565cfe52311`.
La copia de H1 conservó sus cinco identificadores y su fase inicial
tras migrar. Las tres pruebas de Rust pasaron. En el primer intento,
la preparación del test de P02 buscaba un estado que el catálogo no
usa. Se corrigió el filtro para copiar las versiones originales por su
identificador y se repitió la batería.

[Contextos e identificadores](contextos.tsv) · [Detalles antes y después](contextos.json) ·
[Memoria enviada con Qwen local](ollama-real.json) · [Migración de turnos previos](migracion.tsv).
