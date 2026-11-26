# T029 · Pulso y conteo respiratorio

2026-11-25 · `feature/prototipo` · revisión base `fbc3f1ca608d34482d19d72c32bb06dce796a185`

## Procedimiento

Para cada instrumento, preparar una base nueva desde `backend/` con
`target/debug/sircle preparar --db <ruta> --fuentes ../datos`. Levantar Axum
con la base y `npm run dev:integrado` como en T026. Desde `frontend/`, ejecutar
`npm run test:navegador:ritmos -- <evidencia> <ruta-absoluta-de-base> pulso`
y repetir con `respiracion` en otra base. La prueba entra como `estudiante-lr`,
crea el intento, resuelve el pre técnico y compara el gesto con la referencia.
Tras completar el procedimiento consulta el histórico del instrumento.

## Resultado

Los dos ensayos devolvieron HTTP 201 y conservaron una toma con ID de intento,
instrumento, origen, unidad y momento. El pulso registró 102 lpm con origen en
el pulsioxímetro simulado. La respiración registró por separado la frecuencia
de 26 resp/min del escenario y los tres ciclos marcados por el estudiante. Los
intentos fueron distintos. Ambos históricos devolvieron su lectura exacta.
[Pulso](pulso.json) · [Respiración](respiracion.json)

En el original el sensor de pulso se usa desde «Signos vitales». Aquí el mismo
gesto abre la lectura independiente de pulso. Los recortes de ambas escenas
coincidieron en dimensiones y SHA-256 por estado. Chromium 156.0.8078.4,
1280 × 900, zoom 100 %, escala 1.

| Gesto | Original | Extracción | Lectura |
| --- | --- | --- | --- |
| Colocar sensor | [Pulso](pulso-original.png) | [Pulso](pulso-extraccion.png) | [Resultado](pulso-resultado.png) |
| Marcar ciclo | [Respiración](respiracion-original.png) | [Respiración](respiracion-extraccion.png) | [Resultado](respiracion-resultado.png) |
