# T032 · Inspección y palpación por zona

2026-11-25 · `feature/prototipo` · revisión base `fbc3f1ca608d34482d19d72c32bb06dce796a185`

## Procedimiento

Preparar una base vacía con `target/debug/sircle preparar --db <ruta> --fuentes ../datos`,
levantar Axum con esa base y `npm run dev:integrado` como en T026. Ejecutar desde
`frontend/` `npm run test:navegador:exploraciones -- <evidencia> <ruta-absoluta-de-base>`.
Con `estudiante-lr` y pre técnico resuelto, marcar la preparación y elegir una
zona cada vez. Comparar la selección en Chromium 156.0.8078.4 a 1280 × 900,
zoom 100 %, escala 1. Confirmar los cuatro gestos en la extracción y cotejar
el GET del intento con `exploraciones` y `hallazgos_exploracion` en SQLite.

## Resultado

Se esperaban cuatro acciones con instrumento, zona, fase, momento e ID,
y dos hallazgos citables. Los cuatro acuses 201 y el GET coincidieron. Tórax
anterior de inspección recuperó un hallazgo del original de P01. Tórax anterior
de palpación recuperó un hallazgo de ensayo de Semana 5. Tórax posterior y
pantorrilla derecha registraron su acción y quedaron pendientes, sin `hallazgo_id`.
SQLite conservó cuatro acciones y dos hallazgos con su procedencia.
[Acciones y acuses](acciones.json) · [Cotejo en SQLite](sqlite.json)

La primera comparación encontró el marcador de pantorrilla desplazado. Se
aplicó la coordenada 37 % de LR y se repitieron las cuatro zonas. Para cada
zona, el recorte de la escena de la referencia y el de la extracción tuvieron
las mismas dimensiones y huella.

| Gesto y zona | Original | Extracción | Registro |
| --- | --- | --- | --- |
| Inspección anterior | [Zona](frontal-original.png) | [Zona](frontal-extraccion.png) | [Hallazgo](frontal-resultado.png) |
| Inspección posterior | [Zona](posterior-original.png) | [Zona](posterior-extraccion.png) | [Pendiente](posterior-resultado.png) |
| Palpación anterior | [Zona](central-original.png) | [Zona](central-extraccion.png) | [Hallazgo](central-resultado.png) |
| Palpación de pantorrilla | [Zona](extremidad-original.png) | [Zona](extremidad-extraccion.png) | [Pendiente](extremidad-resultado.png) |
