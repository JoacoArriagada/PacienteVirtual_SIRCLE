# T060 · Calibración E12 con una hipótesis justificada de cuatro

26 de noviembre de 2026 · `feature/prototipo` · revisión base `fbc3f1ca`

## Procedimiento

Usar el mismo contenedor local de Ollama en CPU que en T059. Desde la
raíz:

```bash
cargo test --manifest-path backend/Cargo.toml --locked --offline \
  --test agente_evaluador --test rubricas -- --nocapture
cargo test --manifest-path backend/Cargo.toml --locked --offline \
  --test agente_evaluador calibra_uno_de_cuatro_y_prioriza_ecg_con_qwen -- --ignored --nocapture
```

La prueba prepara otra entrega con el fixture `uno_de_cuatro.json`.
Comprueba la huella de la misma rúbrica de T059 antes de abrir el
intento. Registra por la API el relato inicial, una toma de presión
y el informe textual de ECG. Guarda y confirma las cuatro hipótesis
de esta segunda entrega. El trabajador evalúa la entrega en Qwen.
El docente asignado recupera el borrador y la traza. La prueba coteja
los cuatro juicios y sus citas, y comprueba que la primera corrección
sea releer el informe de ECG y revisar la segunda hipótesis.

El modelo redacta los juicios y sus motivos a partir de la entrega.
El servicio comprueba la relación con el informe y ordena primero
un error de hipótesis respaldado por un informe del intento. Para esa
prioridad conserva el ID de la cita, su resultado y el apartado que
el estudiante debe corregir.

## Resultado

La segunda entrega conservó V1 y el SHA-256 de T059. La primera
hipótesis fue acierto con su turno de inicio. La segunda fue error y
citó el informe que registra taquicardia sinusal frente a la afirmación
de fibrilación auricular. La tercera fue omisión y pidió contrastar la
hipótesis de hipertensión sostenida con un hallazgo, con la presión
medida como punto de partida. La cuarta fue error y citó el turno que
el estudiante había tomado como medición. La devolución indicó
distinguir la declaración de la lectura instrumental.

La primera corrección fue releer el ECG y corregir la segunda
hipótesis. El motivo cita la contradicción documentada y explica por
qué corregirla antes de completar los otros fundamentos. Los cuatro
juicios y citas se cotejaron con la clave. Pasaron la calibración real
y las trece pruebas de regresión. Qwen generó 824 tokens en 335,295
segundos y `/api/ps` informó cero bytes de VRAM.

[Cotejo, prioridad y traza de la segunda entrega](uno-de-cuatro.json).
