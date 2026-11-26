# T025 · Sugerencia intacta y editada

2026-11-25 22:00 -0300 · feature/prototipo · revisión base `fbc3f1ca608d34482d19d72c32bb06dce796a185`

## Procedimiento

Preparar y servir una base temporal nueva desde `backend/` con
`target/debug/sircle preparar --db <ruta> --fuentes ../datos` y
`target/debug/sircle servir --db <ruta>`. Desde `frontend/` ejecutar
`npm run dev:ejemplo`. Detener el contenedor local `sircle-ollama-h1`.
Ejecutar `npm run test:navegador:sugerencias -- <evidencia> <ruta-absoluta-de-base>`.
Chromium entra como `estudiante-lr`, selecciona una sugerencia, la envía
intacta con el motor detenido, reinicia Ollama y luego edita esa sugerencia para
preguntar por los antecedentes familiares. El docente coteja las dos trazas.

## Resultado

Se esperaba una confirmación directa y una por inferencia real. El intento
de prueba `65a648ceaa6cd6959dd9d6b6c9693dde` preservó ambos turnos. «¿Desde cuándo comenzó?»
viajó con `sugerencia_id: inicio` y obtuvo HTTP 201 con LR situando el inicio
hace dos días. Ollama estaba detenido y la traza de ese turno identificó
`ficha` sin invocación. La pregunta editada viajó sin identificador de sugerencia
y obtuvo HTTP 201. LR respondió en primera persona que fue adoptada y que
desconoce los antecedentes. La única llamada real seleccionó
`lr-antecedentes-familiares`, con Qwen 3.5:4b y el digest de T013.
[Petición, acuses y trazas](rutas.json)

Chromium a 1280 × 900, zoom 100 %, escala 1. Equipo de desarrollo i3-7020U.
Ollama se recuperó en Docker y el modelo quedó residente al 100 % por CPU.

- [Selección intacta](texto-intacto.png)
- [Texto tras editar](texto-editado.png)
- [Respuesta de LR](respuesta-de-lr.png)
