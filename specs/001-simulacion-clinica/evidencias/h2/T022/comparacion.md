# T022 · Estilos de Semana 5

2026-11-25 21:48 -0300 · feature/prototipo · revisión base `fbc3f1ca608d34482d19d72c32bb06dce796a185`

## Procedimiento

Comparar el bloque `<style>` de la copia privada de T021 con
`frontend/src/app/semana5.css`. La extracción reemplaza el bloque por una hoja
externa y conserva el marcado visible de «Mis pacientes», las clases e iconos.
Los recursos binarios visibles se guardan en `frontend/recursos/semana5/`. Dentro
de `frontend/`, ejecutar `npm test` y `npm run dev:ejemplo`. En otra terminal,
ejecutar `npm run test:navegador:semana5 -- ../specs/001-simulacion-clinica/evidencias/h2/T022`.
La comparación abre el original privado y la extracción en `/semana5`.

## Resultado

Se esperaba conservar estilos, cajas y apariencia con una pantalla de ejemplo.
Con Chromium 156.0.8078.4, resolución 1280x900, zoom 100 %
y escala 1, Roboto estuvo cargada. Coincidieron los
selectores, clases, tipografía, tonos y dimensiones de 15
elementos comprobados. Las capturas tuvieron idéntico SHA-256 `591cb681ba1c51c82509627ccef60bd450fa2f152ce721fb1d5c27b2beac8ce7`.
La pantalla de ensayo mantiene las tres tarjetas de la referencia. La carga
autorizada y los gestos interactivos corresponden a las tareas siguientes.
[Cotejo de estilos](estilos.json)

- [Captura original](original.png)
- [Captura de la extracción](extraccion.png)

El detector estático no resolvió la URL de la hoja externa desde el archivo
fuente. El cotejo de Chromium cargó la hoja servida en esa URL y verificó los
valores tipográficos de la página renderizada.
