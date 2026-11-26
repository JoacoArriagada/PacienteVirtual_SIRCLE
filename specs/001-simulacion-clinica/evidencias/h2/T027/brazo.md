# T027 · Brazo ampliado y colocación del brazalete

2026-11-25 · `feature/prototipo` · revisión base `fbc3f1ca608d34482d19d72c32bb06dce796a185`

## Procedimiento

Levantar las dos aplicaciones con la base de T020 como en T026. En `frontend/`,
ejecutar `npm test` y `npm run test:navegador:brazo -- <ruta-de-evidencia>`.
La prueba compara Chromium 156.0.8078.4 a 1280 × 900, escala 1 y zoom 100 %.
Abre «Ver brazo», luego «Presión» y «Preparar el brazalete». Activa por orden
la selección, la colocación y la confirmación de la posición. Compara encuadre,
pasos y habilitación de «Medir presión» en el original y en el intento de ensayo.

## Resultado

Coincidieron los pasos y las imágenes de los cuatro estados. El brazo ampliado
midió 540 × 360 px y la escena del procedimiento 461,86 × 307,91 px en ambas
aplicaciones. Los recortes de la escena de cada estado tuvieron el mismo SHA-256.
«Medir presión» permaneció deshabilitado al preparar y al colocar, y se habilitó
solo al confirmar el ajuste. Al mover el modo de ejemplo, la primera compilación
detectó un tipo faltante. Se corrigió la referencia de tipo y pasaron la
compilación, las dos pruebas de ejemplo y todo el recorrido en Chromium.
[Encuadres, huellas y estados](estados.json)

| Estado | Original | Extracción |
| --- | --- | --- |
| Brazo ampliado | [Brazo](original-brazo.png) | [Brazo](extraccion-brazo.png) |
| Preparación | [Preparación](original-preparacion.png) | [Preparación](extraccion-preparacion.png) |
| Brazalete colocado | [Colocado](original-colocado.png) | [Colocado](extraccion-colocado.png) |
| Ajuste confirmado | [Ajustado](original-ajustado.png) | [Ajustado](extraccion-ajustado.png) |
