# T030 · Temperatura frontal y saturación

2026-11-25 · `feature/prototipo` · revisión base `fbc3f1ca608d34482d19d72c32bb06dce796a185`

## Procedimiento

Preparar una base nueva desde `backend/` con `target/debug/sircle preparar --db <ruta> --fuentes ../datos`.
Levantar Axum y `npm run dev:integrado` con esa base, como en T026. Desde
`frontend/`, ejecutar `npm run test:navegador:sensores -- <evidencia> <ruta-absoluta-de-base>`.
El ensayo crea un intento de `estudiante-lr`, resuelve el pre técnico y recorre
primero el termómetro frontal y después el pulsioxímetro. Para cada instrumento
comprueba los pasos, guarda una medición y la registra en ficha. Después de
ambos registros abre sus históricos y coteja el GET del intento.

## Resultado

Cada instrumento produjo un HTTP 201 y un solo registro en ficha. El
termómetro registró 36,6 °C y el pulsioxímetro 84 %. Ambos acuses incluyeron
origen y momento. El histórico de cada instrumento devolvió solo su ID. Los
valores corresponden a la versión de ensayo de LR. Para esta versión se
interpretó `Temp 97.9` del triaje original como °F y se convirtió a °C con
un decimal. La saturación conserva el 84 % en aire ambiente de la fuente.
[Acuses, históricos y ficha](lecturas.json)

Las dos colocaciones conservaron dimensiones y SHA-256 al cotejar el recorte
del dispositivo. Chromium 156.0.8078.4, 1280 × 900, zoom 100 %, escala 1.

| Instrumento | Original | Extracción | Resultado |
| --- | --- | --- | --- |
| Termómetro frontal | [Preparación](temperatura-original.png) | [Preparación](temperatura-extraccion.png) | [Lectura](temperatura-resultado.png) |
| Pulsioxímetro | [Preparación](saturacion-original.png) | [Preparación](saturacion-extraccion.png) | [Lectura](saturacion-resultado.png) |
