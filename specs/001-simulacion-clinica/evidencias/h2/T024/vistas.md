# T024 · Visibilidad según el pre y la fase

2026-11-25 21:56 -0300 · feature/prototipo · revisión base `fbc3f1ca608d34482d19d72c32bb06dce796a185`

## Procedimiento

Desde `backend/`, preparar una base vacía con
`target/debug/sircle preparar --db <ruta> --fuentes ../datos` y usar
`target/debug/sircle servir --db <ruta>`. Desde `frontend/`,
iniciar `npm run dev:ejemplo` y ejecutar
`npm run test:navegador:ficha -- <evidencia> <ruta-absoluta-de-base>`.
El navegador entra como `estudiante-lr`, crea un intento y comprueba
la vista y el GET antes del pre. La prueba ejecuta `resolver-pre-ensayo`,
reabre el mismo intento por «Continuar» y abre las tres secciones.

## Resultado

Se esperaba que la vista y el GET reservaran el contenido hasta resolver el pre.
El intento `fe03b25bd45a7f43265a600c3b9baa98` pasó de pendiente a omitido.
En pre pendiente no aparecieron introducción, ficha, indicaciones, ayuda, turnos
ni mediciones en JSON. En fase inicial, los controles abrieron la ficha de ensayo
de LR con 148/85 mmHg y las indicaciones. «Orientación docente» mostró su origen
`docente` mientras las mediciones del estudiante seguían vacías. Pasaron las
suites Rust y TypeScript. [Respuestas de las dos fases](respuestas.json)

El selector de prueba de «Indicaciones» encontró el botón del encabezado y
el de la ficha en la primera corrida. Se acotó a la región principal y
se repitió el recorrido con otra base vacía.

Chromium 156.0.8078.4 · 1280 × 900 · zoom 100 % · escala 1.

- [Pre pendiente](pre-pendiente.png)
- [Pre resuelto y secciones abiertas](pre-resuelto.png)
