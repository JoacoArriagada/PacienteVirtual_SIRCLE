# T031 · Peso de ensayo

2026-11-25 · `feature/prototipo` · revisión base `fbc3f1ca608d34482d19d72c32bb06dce796a185`

## Procedimiento

Preparar una base vacía con `target/debug/sircle preparar --db <ruta> --fuentes ../datos`
desde `backend/`. Arrancar Axum y `npm run dev:integrado` como en T026 y ejecutar
`npm run test:navegador:sensores -- <evidencia> <ruta-absoluta-de-base> peso`
desde `frontend/`. En ambas interfaces abrir la báscula, comprobar cero,
confirmar calzado retirado y situar al paciente en la plataforma. En la
extracción completar la toma, registrar en ficha y recuperar el histórico.
Consultar `procedencia_ensayo` de la medición registrada y cotejar su SHA-256
contra el HTML original.

## Resultado

El botón «Medir peso» permaneció deshabilitado hasta el tercer paso en las dos
interfaces. El recorte del dispositivo tuvo las mismas dimensiones y huella
en Chromium 156.0.8078.4, 1280 × 900, zoom 100 %, escala 1. El intento
`246c268a71f26012613865859c63149f` devolvió una toma de 64,3 kg con ID
`889521da9f14a0b58eb370ba067b6ec2`. El histórico y la ficha conservaron el mismo
ID y una sola fila. El valor se identificó como dato de ejemplo de la versión
técnica, con su procedencia de Semana 5 archivada en SQLite.
[Acuses e histórico](lecturas.json) · [Procedencia cotejada](procedencia.json)

- [Original](peso-original.png)
- [Extracción](peso-extraccion.png)
- [Resultado](peso-resultado.png)
- [Registro en ficha](ficha.png)
