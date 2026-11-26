# T011 · Brazalete de ejemplo

2026-11-25 21:05 -0300 · feature/prototipo · revisión base `fbc3f1ca608d34482d19d72c32bb06dce796a185`

## Procedimiento

Desde `frontend/`, ejecutar `npm ci`, `npm test` y `npm run build`.
Con Axum detenido, ejecutar `npm run dev:ejemplo` en una terminal y
`npm run test:navegador:brazalete -- ../specs/001-simulacion-clinica/evidencias/h1/T011`
en otra. La variable `SIRCLE_CHROMIUM` permite indicar el ejecutable del navegador disponible.
También se puede repetir manualmente en 4173 al preparar, colocar, tomar la presión, leer
el resultado y pulsar dos veces «Registrar en ficha».

## Resultado

Se esperaba un mismo encuadre durante el gesto y una sola entrada tras dos registros.
Con la interfaz activa, el puerto 8081 rechazó la conexión y el navegador realizó cero
peticiones a `/api/`. La toma recuperó `frontend/ejemplos/presion.json` por HTTP 200.
Apareció 148/85 mmHg con la etiqueta «Lectura de ensayo de LR, fase inicial». Antes
de registrar había cero filas y tras cada registro hubo una, con el ID
`presion-lr-ejemplo-v1`. Las dos pruebas unitarias pasaron.
La primera ejecución visual detectó correctamente la conexión rechazada, pero su
aserción trató el error anidado como un objeto exacto. Se corrigió la comparación
del código de error y se repitió el recorrido de principio a fin. [Sesión observada](sesion.json)

## Capturas

Chromium 156.0.8078.4 · 1280x900 · zoom 100 % · escala 1.
Marco del brazo fijo en x=256, y=339.42, 768 × 290 px.

- [01-preparado.png](01-preparado.png) · SHA-256 `241425f6e095db8277f63ca6cbadcf32138e07984bef08dc2b44cea4e987f680`
- [02-colocado.png](02-colocado.png) · SHA-256 `aedad8808530bbf1ceeb7fcb4cb7c0d3d7f0534dd6a3ecc0cf15fe89dae3cb8b`
- [03-resultado.png](03-resultado.png) · SHA-256 `e38bea66d66119adbd47f1c0e39077b709832801f75c7419fdd0cde9178ce010`
- [04-historico.png](04-historico.png) · SHA-256 `3c8fd5197ec31133c4b50e7c95a0d82b18502c65284e72b10151aa4a17c3f327`
