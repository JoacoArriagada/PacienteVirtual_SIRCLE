# T023 · Continuar el intento de LR

2026-11-25 21:51 -0300 · feature/prototipo · revisión base `fbc3f1ca608d34482d19d72c32bb06dce796a185`

## Procedimiento

Arrancar `npm run dev:ejemplo` desde `frontend/` y
`target/debug/sircle servir --db target/demo.sqlite` desde `backend/`.
Desde `frontend/`, ejecutar
`npm run test:navegador:seleccion -- ../specs/001-simulacion-clinica/evidencias/h2/T023`.
La prueba abre `/aplicacion` y reenvía por Playwright las rutas relativas
`/api` a Axum. Entra con `estudiante-lr`, abre «En curso → Continuar», sale
de la sesión, entra con la misma identidad y continúa otra vez. Compara la
tarjeta y los controles con «Mis pacientes» del original filtrado por LR.

## Resultado

Se esperaba recuperar el mismo intento sin modificarlo y conservar el diseño
de selección. Las dos consultas HTTP devolvieron el intento
`c1aaccc1b05db9c4cc6cb572ddad196e`, versión `ensayo-originales-e4bbd7046ad5752ef50a0df4212be7d6873220bc1a3773029806ebdde49fdd69` y los mismos
4 turnos del acta de T020. La salida de sesión devolvió 204.
La prueba cotejó clases, Roboto, tamaños, anchos y colores de los
5 elementos de selección a 1280 × 900, zoom 100 %, escala 1.
El estado y la acción se rotularon «En curso» y «Continuar» para el intento guardado.
Se presenta una sola tarjeta autorizada y la edad se reserva para la ficha de fase.
[Comparación y respuestas](respuestas.json)

- [Referencia filtrada](referencia.png)
- [Selección En curso](en-curso.png)
- [Intento recuperado](reabierto.png)
