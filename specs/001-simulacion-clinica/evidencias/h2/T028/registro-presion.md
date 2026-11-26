# T028 · Toma de presión, histórico y ficha

2026-11-25 · `feature/prototipo` · revisión base `fbc3f1ca608d34482d19d72c32bb06dce796a185`

## Procedimiento

Preparar una base de ensayo vacía con `target/debug/sircle preparar --db <ruta> --fuentes ../datos` desde `backend/`. Ejecutar Axum con esa base y `npm run dev:integrado` desde `frontend/` como en T026. Ejecutar `npm run test:navegador:presion -- <evidencia> <ruta-absoluta-de-base>`.

La prueba crea un intento como `estudiante-lr`, resuelve el pre técnico y reabre la práctica. Prepara una toma, la cancela tras un segundo simulado y adelanta el reloj para comprobar que no se envíe un POST. Completa otra toma y pulsa dos veces «Registrar en ficha». Coteja las respuestas con el histórico, el GET del intento y la ficha visible. Chromium 156.0.8078.4 a 1280 × 900, zoom 100 %, escala 1.

## Resultado

La cancelación produjo cero POST y cero lecturas. La toma completa obtuvo HTTP 201 y el ID `08fd0eef579ee6110a5cd3fc369eca5b` en el intento `bdb5df84a6a543ddd733a7609cb4e31f`. La lectura fue 148/85 mmHg. Los dos PUT devolvieron HTTP 200 con el mismo ID y acuse. El histórico, el GET y la ficha mostraron una sola referencia a la toma. Las suites de frontend y backend pasaron. [Peticiones, acuses y recuentos](registro.json)

- [Toma en curso](toma-en-curso.png)
- [Histórico tras cancelar](historico-tras-cancelar.png)
- [Resultado](resultado.png)
- [Registro en ficha](registro-en-ficha.png)
- [Histórico final](historico.png)
- [Ficha](ficha.png)
