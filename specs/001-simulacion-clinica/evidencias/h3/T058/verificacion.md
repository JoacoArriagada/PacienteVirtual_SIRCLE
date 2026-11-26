# T058 · Generar el borrador y vincularlo a su operación

26 de noviembre de 2026 · `feature/prototipo` · revisión base `fbc3f1ca`

## Procedimiento

```bash
SIRCLE_T056_DB="$PWD/backend/target/t056-calibracion.sqlite" \
  cargo test --manifest-path backend/Cargo.toml --locked --offline \
  --test agente_evaluador -- --nocapture
cargo test --manifest-path backend/Cargo.toml --locked --offline \
  --test entregas --test trabajadores --test mensajes --test autorizacion -- --nocapture
```

La primera ejecución abre una copia de T056 y la segunda prepara bases
temporales independientes. `rastrear_criterio_a_evidencia_y_regresion`
enciende un doble del protocolo HTTP de Ollama, inicia los trabajadores
reales y ejecuta la toma de la cola. El doble registra el encabezado de
operación y el cuerpo de la solicitud. El servicio guarda la respuesta,
la enlaza al borrador y permite releer ambos como docente asignado.
La prueba coteja todos los campos propuestos con el borrador guardado,
comprueba el origen de cada crédito y repite la lectura como titular y
docente ajeno. También intenta tomar otra vez el trabajo. La prueba
`escenario_sin_examen_no_evaluable` deja inhabilitado el ECG de P02 en
su base, ejecuta una entrega y coteja que el criterio de decisiones
quede como no evaluable sin enviarlo al modelo.

## Resultado

A produjo un solo borrador con los cinco ámbitos y su rúbrica V1 de
T056. El docente asignado obtuvo 200 al leer borrador y traza. El
titular y el docente ajeno obtuvieron 403 en ambos recursos. El
encabezado del adaptador, la invocación y la evaluación compartieron
el ID de operación. Los cinco juicios, sus citas y sus textos
coincidieron con la salida del doble. Los dos créditos citaron una
medición recuperable de A y ningún crédito apuntó fuera de la entrega.
El criterio de decisiones de P02 quedó como no evaluable, con
expectativa, explicación y mejora, y el trabajo terminó una sola vez
por entrega. Pasaron quince pruebas Rust en total.

[Evaluación docente, traza de ensayo y criterio sin oportunidad](criterios.json).
La corrida con Qwen local corresponde a [T059](../../../tasks.md#ev-t059).
