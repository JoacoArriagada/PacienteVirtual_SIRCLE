# T049 · Aviso de dato ausente

26 de noviembre de 2026, 07:29 CLST · `feature/prototipo` · revisión base `fbc3f1ca`

## Procedimiento

En `backend/`, ejecutar `cargo build --locked --offline` y
`cargo test --locked --offline --test agente_lr --test mensajes --test visibilidad -- --nocapture`.
Preparar otra base nueva con `target/debug/sircle preparar --db "$DB" --fuentes ../datos`.
Arrancar Axum y `npm run dev:integrado` con Ollama local accesible en el puerto 11434.
En este computador se usó `unshare -Urn` y un puente Unix a Ollama, como en
T026, por el puerto 8080 ocupado por otro proyecto. Desde `frontend/`:

```bash
npm run test:navegador:dato-ausente -- ../specs/001-simulacion-clinica/evidencias/h3/T049 "$DB"
```

El guion abre un intento con historial vacío, envía «¿Cuál es su grupo
sanguíneo?» y comprueba el aviso del sistema en el chat. Compara las dos
versiones de ficha y evidencias, lee la traza con el docente asignado y
comprueba el digest y el uso de CPU en Ollama.

## Resultado

Axum confirmó un turno del **Sistema** con el tipo `dato_ausente` y
«El grupo sanguíneo no está disponible en este escenario». La interfaz lo
mostró en una burbuja de sistema. La ficha y las evidencias se conservaron
iguales y hubo cero hechos citables nuevos. Qwen eligió `sin_hecho` y la
traza registró `dato_no_registrado` sin asociarle un hecho clínico.
La regresión distingue este caso de la aclaración de LR y del
desconocimiento familiar documentado. El turno fue `6fb229e2c345d2743bd5954d31d02f1c`.
Pasaron las trece pruebas de Rust y la compilación de frontend.

[Acuse, traza y ficha antes y después](dato-ausente.json) ·
[Captura del aviso](aviso.png) · [Regresiones Rust](rust.txt).
