# T064 · Recuperar una evaluación después de detener Qwen

26 de noviembre de 2026 · `feature/prototipo` · revisión base `fbc3f1ca`

## Procedimiento

Desde la raíz, con el contenedor de ensayo `sircle-ollama-h1` de
T013 activo y el puerto 18081 libre:

```bash
cargo test --manifest-path backend/Cargo.toml --locked --offline \
  --test recuperacion_e22 -- --nocapture
D="$(mktemp -d backend/target/e22-t064.XXXXXX)"
python3 backend/tests/fixtures/ejecutar-e22.py \
  --db "$D/intentos.sqlite" --destino specs/001-simulacion-clinica/evidencias/h3/T064
```

La prueba Rust usa un doble de Ollama con una barrera. Retiene su
primera respuesta, adelanta el vencimiento en una base temporal,
espera que el mismo trabajo tenga una nueva toma y libera la respuesta
anterior. Comprueba los recuentos y qué resultado se guardó.

El ensayo con el motor real detiene únicamente el contenedor de
Ollama de SIRCLE y comprueba que el puerto deja de responder. Con Axum
local crea el intento, resuelve el pre técnico, toma una presión y
vincula la medición con la hipótesis. Confirma y reenvía la entrega y
omite el post de ensayo. Inspecciona SQLite tras el fallo real de la
primera toma del trabajo. Detiene el proceso de API, restaura el
contenedor y verifica una generación breve. Levanta Axum sobre la
misma base y deja que la cola reintente. Compara IDs, recuentos y la
traza de evaluación accesible como docente, y coteja VRAM en `/api/ps`.

## Reinicio transitorio

El primer sondeo recibió `ConnectionResetError` al reiniciar Docker.
La base ya tenía el acuse y el trabajo pendiente. Se corrigió el
sondeo para reintentar errores de conexión y se ejecutó el mismo
comando con `--reanudar` sobre esa base. La reanudación comprueba
los IDs y recuentos ya observados y se limita a recuperar el trabajo.

[Salida del primer sondeo](reinicio-transitorio.txt) ·
[Copia observada durante la caída](en-caida.json).

## Resultado

Con Ollama detenido, el reenvío de la entrega devolvió 200 con el
mismo acuse. La medición nueva quedó vinculada a la hipótesis.
SQLite conservó una medición, una entrega, un acuse, un post de ensayo
omitido y un trabajo pendiente, con cero evaluaciones. El primer
intento de tomar el trabajo registró conexión rechazada.

Al restaurar el motor, la primera generación breve respondió
«Listo». El proceso de API reabierto tomó el mismo trabajo con un
nuevo ID de toma y confirmó una evaluación. Se conservaron los IDs
de medición, entrega y trabajo. El modelo ejecutó la evaluación en
153,565 segundos con cero bytes de VRAM. SQLite conservó una sola
traza y revisión, y pasó `PRAGMA integrity_check`.

La prueba de la toma vencida devolvió el resultado anterior tarde.
El resultado se descartó, se confirmó el de la segunda toma y
quedaron una evaluación y una traza para la entrega.

[Recuentos e IDs de E22](e22-local.json) · [Traza real](recuperacion-real.json) ·
[Respuesta tardía y cierre único](toma-vencida.json).
