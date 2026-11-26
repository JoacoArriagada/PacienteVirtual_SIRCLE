# T040 · Cuatro trabajadores supervisados

26 de noviembre de 2026, 00:24 CLST · `feature/prototipo` · revisión base `fbc3f1ca`

## Procedimiento

Desde `backend/`, ejecutar:

```bash
cargo build --locked --offline
cargo test --locked --offline --test trabajadores --test mensajes --test visibilidad -- --nocapture
```

El supervisor crea los cuatro procesos y recibe de cada uno su ficha de
arranque por un par de tuberías de entrada y salida estándar. La prueba
envía una solicitud de diagnóstico con operación distinta a cada canal.
Comprueba PID, etiqueta de agente y operación. Envía una solicitud de LR
por el canal de Nannu para verificar que se rechace antes de inferir.
Detiene el supervisor y comprueba que su inventario queda vacío.

Para comprobar el arranque de Axum y el motor real, preparar una base
nueva y usar el puerto local 18081 con Ollama de T013 activo, como en T039.
Las líneas `SIRCLE_TRABAJADOR` contienen las fichas. Para cada PID,
consultar `/proc/<pid>/status` y `/proc/<pid>/cmdline`. Los cuatro deben
tener por padre el proceso `sircle servir` y recibir solo el identificador
del agente y la dirección de Ollama como argumentos de su comando interno.
Comparar el digest con `GET /api/tags` en Ollama. Abrir un intento de ensayo
de LR, resolver el pre técnico y enviar una pregunta libre por HTTP.
Comparar la traza, el acuse y el inventario. Al terminar Axum, verificar que
desaparezcan los cuatro PID.

La primera compilación offline indicó que faltaba `signal-hook-registry`
para gestionar los procesos. Se descargó la dependencia de Tokio una vez
y se reconstruyó con `--locked --offline`.

## Resultado

Los cuatro procesos respondieron por canales distintos y las respuestas de
diagnóstico conservaron su operación, agente y PID. El canal de Nannu
rechazó la solicitud de LR. Pasaron además las regresiones de mensajes y
visibilidad. Las cuatro fichas apuntaron al mismo Ollama local y al digest
de Qwen fijado en T013. En la corrida por HTTP, Axum tuvo PID 1130413.
El turno de LR `0fc3a894e8350363fd32110c748262ca`
se confirmó por su trabajador, con una latencia de 4832 ms y Qwen en CPU.
Tras detener el servicio quedaron cero PID de los cuatro trabajadores.

[Proceso padre, PID, canales y destino](procesos.tsv) ·
[Prueba de enrutamiento](enrutamiento.json) ·
[Pregunta libre y trazas de Axum y Ollama](ollama-real.json).
