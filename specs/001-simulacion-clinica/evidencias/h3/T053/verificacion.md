# T053 · Interrumpir y recuperar el trabajador de Nannu

26 de noviembre de 2026, 07:46 CLST · `feature/prototipo` · revisión base `fbc3f1ca`

## Procedimiento

Preparar una base nueva y habilitar la actividad P02 únicamente en ese
ensayo. El procedimiento de asignación consta en T050. Desde `backend/`,
arrancar `target/debug/sircle servir --db "$DB" --direccion 127.0.0.1:18081`
con stdout y stderr en un archivo nuevo. Mantener Ollama local en CPU.
Ejecutar desde `backend/` con las rutas absolutas:

```bash
SIRCLE_URL=http://127.0.0.1:18081 SIRCLE_T053_DB="$DB" SIRCLE_T053_LOG="$LOG_AXUM" \
  cargo test --locked --offline --test agente_nannu retoma_turno_del_familiar -- --ignored --nocapture
```

La prueba abre los intentos de Nannu y LR, resuelve ambos pre y comprueba
que el PID de Nannu corresponde al subcomando trabajador de esa corrida.
Envía una pregunta libre al acompañante y espera la marca de operación del
trabajador. Cuando la petición aún espera su respuesta, termina solo ese
PID. Comprueba el historial vacío, envía una pregunta libre a LR y coteja
el PID que la atendió. Reenvía la pregunta de P02 con la misma clave,
compara el PID nuevo y vuelve a reenviar el acuse. Verifica el historial
de P02 y la permanencia de los otros procesos.

## Resultado

Interrumpir el trabajador de Nannu devolvió 503 y dejó cero turnos en P02.
Mientras ese proceso seguía detenido, LR confirmó un turno en su PID
original. Al repetir el envío original de P02, el supervisor inició un
nuevo proceso para Nannu y la misma operación confirmó un turno del
acompañante. El siguiente reenvío devolvió 200 con el mismo acuse.
El PID de Nannu cambió de 1276080 a 1277815 y el ID de su operación se conservó.
Los otros tres trabajadores conservaron sus PID. El motor ejecutó Qwen
en CPU. Pasaron además las once pruebas ordinarias de Nannu y supervisión.

[Comparación de procesos y acuses](reinicio-nannu.tsv) ·
[Marcas, acuses y recuperación](reinicio-nannu.json) · [Regresión Rust](rust.txt).
