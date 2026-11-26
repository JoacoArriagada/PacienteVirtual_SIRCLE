# T048 · Desconocimiento familiar documentado de LR

26 de noviembre de 2026, 07:27 CLST · `feature/prototipo` · revisión base `fbc3f1ca`

## Procedimiento

Ejecutar `cargo test --locked --offline --test agente_lr cita_desconocimiento_familiar -- --nocapture`
desde `backend/`. Para la comprobación real, iniciar Ollama y Axum como en
T040 con otra base nueva. Abrir un intento de LR, omitir el pre técnico y
verificar un historial vacío. Enviar
`{"texto":"¿Conoces los antecedentes de salud de tu familia biológica?"}` con
la clave `t048-ollama` a `POST /api/intentos/{id}/turnos`. Recuperar la vista
estudiantil y la traza del docente asignado. Comparar la respuesta, el
hablante, el hecho seleccionado, el pasaje, su línea y versión con el
guion del actor. Verificar el digest y CPU con el inventario de Ollama.

## Resultado

LR respondió «No conozco los antecedentes de mi familia biológica porque fui
adoptada». El tipo `hecho` identifica un desconocimiento comunicado por
la paciente. La vista recuperó el turno como una declaración citable.
La traza enlazó `lr-antecedentes-familiares` con su pasaje de la línea 17
del actor y la versión exacta del intento. Qwen eligió ese hecho en CPU. El turno fue `043d0b080cb6122fd03ade598a59fdf4`.

El primer guion de comprobación esperaba por error la línea 8. La respuesta
y el pasaje coincidieron. Se cotejó la numeración original, se corrigió el
aserto a la línea 17 y se repitió la llamada sobre una base nueva.

[Turno, selección y procedencia](familia.json) ·
[Guion del actor de LR](<../../../../../datos/medsp1000/mededportal_10181/scenario1/sp_actor/Case A-Asthma.md>).
