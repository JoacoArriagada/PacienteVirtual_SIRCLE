# T065 · Cierre de los 32 núcleos y recorridos de trazabilidad

26 de noviembre de 2026 · `feature/prototipo` · revisión base `fbc3f1ca`

## Procedimiento

Usar el contenedor CPU de T013 en 11434 y dejar libre el puerto 18081.
Desde la raíz:

```bash
EVIDENCIA=specs/001-simulacion-clinica/evidencias/h3/T065
cargo test --manifest-path backend/Cargo.toml --locked --offline \
  -- --nocapture > /tmp/sircle-t065-rust.log 2>&1
python3 backend/tests/fixtures/verificar-agentes-local.py "$EVIDENCIA"
cargo test --manifest-path backend/Cargo.toml --locked --offline \
  --test agente_evaluador -- --ignored --nocapture --test-threads=1 \
  > /tmp/sircle-t065-e12.log 2>&1
python3 "$EVIDENCIA/verificar-matriz.py" /tmp/sircle-t065-rust.log /tmp/sircle-t065-e12.log
```

El verificador recorre las filas del contrato, comprueba los seis
campos y los enlaces de cada README, localiza la función en su archivo
Rust y exige su resultado satisfactorio. Las dos pruebas externas de
la matriz se ejecutan contra Axum y Ollama. La prueba P02/P03 abre dos
intentos, envía las preguntas en paralelo y coteja los tiempos, los
hechos, la voz, el historial y la traza de cada uno.

Para el primer recorrido, el verificador sigue el rechazo de un
candidato del acompañante presentado como Nannu en una regresión
controlada. Comprueba el pasaje original y lo coteja con una respuesta
posterior de Qwen para el acompañante autorizado. Para el segundo
recorrido, sigue el error de la hipótesis 2 de E12 desde la devolución
docente hasta el informe, el pasaje del controlador, la rúbrica y el
resultado de la operación. Las huellas de instrucciones se calculan
sobre el mensaje de sistema exacto, en UTF-8 y sin reformatearlo.

E22 queda enlazado al ensayo con la caída real y a la regresión de
la toma vencida de T064.

## Resultado

Se comprobaron ocho núcleos por agente y 32 correspondencias
entre README, código y caso de prueba. Pasaron 64 pruebas Rust
ordinarias. Las baterías de LR, Nannu, Dan Sparks y el evaluador
aportaron 11, 10, 9 y 16 pruebas ordinarias, respectivamente.
Las pruebas externas de E17, la recuperación de Nannu y ambas
calibraciones de E12 pasaron por separado. Los ensayos externos
de H1 conservan su propia evidencia.

Las peticiones de LR coincidieron 92,069 segundos antes de la
primera confirmación. Las de P02 y P03 coincidieron 19,408
segundos y dejaron un turno de cada voz en su propio historial.
Nannu recuperó la misma operación después de reemplazar su
trabajador. E12 repitió los patrones de tres y un acierto con la
misma versión y huella, y conservó la prioridad del ECG cuando
correspondía. Las corridas externas confirmaron cero bytes de VRAM.
El recorrido del fallo enlazó el candidato rechazado con el pasaje
del acompañante y su regresión. El recorrido del criterio enlazó
el error de hipótesis con el informe de taquicardia sinusal y la
salida de Ollama que sustentó ese juicio.

[Matriz de los 32 núcleos](matriz.tsv) · [Regresiones Rust](rust.txt) ·
[Recorridos](recorridos.json) · [E12 repetido](e12-repetido.json) ·
[E17 de LR](lr-e17.json) · [Reinicio de Nannu](nannu-reinicio.json) ·
[E17 entre P02 y P03](p02-p03.json) · [E22 local](../T064/e22-local.json).
