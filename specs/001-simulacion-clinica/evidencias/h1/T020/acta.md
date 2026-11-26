# T020 · Primer avance técnico de LR

2026-11-25 21:43 -0300 · feature/prototipo · revisión base `fbc3f1ca608d34482d19d72c32bb06dce796a185`

## Procedimiento reproducible

1. Desde `frontend/`, ejecutar `npm ci`, `npm test` y `npm run build`. Con Axum
   apagado, usar `npm run dev:ejemplo` para preparar, colocar y tomar la presión
   y registrar dos veces la lectura. [Arranque](../T002/frontend.txt) · [Brazalete](../T011/brazalete.md)
2. Desde `backend/`, ejecutar `cargo build --locked`, `cargo test --locked` y
   `cargo run --locked --bin sircle -- preparar --db target/demo.sqlite --fuentes ../datos`.
   Con Ollama local, arrancar `target/debug/sircle servir --db target/demo.sqlite`.
   Entrar en `/api/sesiones` como `estudiante-lr` y recuperar el intento de T008.
   Cotejar el turno real de T015 con su traza de T016. [Pregunta y acuse](../T015/pregunta-libre.json)
   · [Traza docente](../T016/traza-turno.json)
3. En el mismo intento, enviar dos veces `POST /api/intentos/c1aaccc1b05db9c4cc6cb572ddad196e/mediciones`
   con `Idempotency-Key: t020-toma-demostracion` y
   `{"instrumento":"presion","accion":"toma_completa"}`.
   Guardar el GET de `/api/intentos/c1aaccc1b05db9c4cc6cb572ddad196e` y detener Axum.
4. Repetir el comando de preparación, reiniciar Axum, entrar otra vez con la
   misma identidad y repetir el GET y el POST con la misma clave. Comparar
   inventarios con `cmp` y verificar `PRAGMA integrity_check` en SQLite.

## Resultado observado

- El frontend y el backend se ejecutaron y probaron por separado. El brazalete
  de ejemplo conservó una sola entrada con Axum detenido.
- El intento `c1aaccc1b05db9c4cc6cb572ddad196e` recuperó el turno T015 `15d81d67da3ce808602bfaaa840a20fa`.
  Qwen 3.5:4b se ejecutó por CPU con el digest descrito en T013. El mismo
  historial de 4 turnos reapareció tras reabrir.
- La toma de presión `0fccd2ee381de9720e814a3a2caa8b08` obtuvo 201, 200 al reenviar y
  200 al repetir después de reabrir, con el mismo acuse. Solo se guardó una
  lectura de 148/85 mmHg. [Acuses y vistas](persistencia.json)
- Las dos preparaciones devolvieron los mismos tres casos y 14 fuentes y
  cotejaron byte a byte con T005. SHA-256 del inventario `57f08e310c348ae37ede034a25ece430836011b5614c0253223623a4413fb3b4`.
  SQLite devolvió `integrity_check=ok`. [Antes](inventario-antes.tsv) · [Después](inventario-despues.tsv)
- Con el motor detenido no se agregaron turnos. Al recuperarlo, una sola
  llamada confirmó la operación fallida. Las muestras fría y caliente son de
  este equipo de desarrollo y quedan pendientes de repetición en el computador
  del profesor. [Caída](../T017/caida.json) · [Recuperación](../T018/recuperacion.json)
  · [Latencias](../T019/latencias.tsv)

La demostración técnica cumple el criterio del hito 1 con material de ensayo.
El intento y la base `backend/target/demo.sqlite` quedan conservados para T026.
Los pesos de Qwen siguen en el volumen Docker `sircle-h1-pesos`.
