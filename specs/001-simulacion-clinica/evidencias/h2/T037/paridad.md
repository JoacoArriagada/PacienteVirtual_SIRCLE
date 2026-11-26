# T037 · Comparación de Semana 5

26 de noviembre de 2026, 00:11 CLST · `feature/prototipo` · revisión base `fbc3f1ca`

## Procedimiento

Desde `backend/`, preparar una base nueva con
`target/debug/sircle preparar --db "$DB" --fuentes ../datos` tras
`cargo build --locked --offline`. Usar una ruta absoluta para `DB`.
Arrancar `target/debug/sircle servir --db "$DB"` y, desde `frontend/`,
`npm run dev:integrado`. En este computador se ejecutaron en una red local
aislada con `unshare -Urn` e `ip link set lo up` por el uso previo del puerto 8080.
Desde `frontend/` ejecutar:

```bash
npm run test:navegador:paridad -- ../specs/001-simulacion-clinica/evidencias/h2/T037 "$DB"
```

El guion abre el original en `docs/referencias/html-semana5/` y la aplicación
interactiva con dos páginas de Chromium 156.0.8078.4. Verifica el SHA-256 del
HTML respecto de la copia privada y confirma en las dos ventanas 1280 × 900,
escala de dispositivo 1 y zoom 100 %. Filtra por LR en el original, compara el
único paciente asignado en la extracción y pregunta en ambas «¿Desde cuándo
comenzó?». Guarda una captura de la ventana y un recorte por cada estado.

## Resultado

| Estado | Original | Extracción | Comparación |
|---|---|---|---|
| Selección | [Pantalla](original-seleccion.png) | [Pantalla](extraccion-seleccion.png) | Tarjeta del mismo tamaño y controles de catálogo. La extracción lista la asignación de ensayo y reserva los datos clínicos antes del pre. |
| Chat | [Pantalla](original-chat.png) | [Pantalla](extraccion-chat.png) | Columna del mismo ancho, área de chat de 180 px y burbuja estudiantil alineada a la derecha en ambas. La redacción procede de cada versión. La extracción permite pregunta libre y muestra una tarjeta de entrevista propia. |
| Brazo ampliado | [Pantalla](original-brazo.png) | [Pantalla](extraccion-brazo.png) | Recortes de 540 × 360 px idénticos. |
| Preparación | [Pantalla](original-preparacion.png) | [Pantalla](extraccion-preparacion.png) | Escena idéntica y medición deshabilitada. |
| Brazalete colocado | [Pantalla](original-colocado.png) | [Pantalla](extraccion-colocado.png) | Escena idéntica y medición deshabilitada hasta confirmar el ajuste. |
| Toma en curso | [Pantalla](original-toma.png) | [Pantalla](extraccion-toma.png) | Monitor del mismo tamaño y avance de la señal simulada. Cada versión usa su señal de práctica. |
| Resultado | [Pantalla](original-resultado.png) | [Pantalla](extraccion-resultado.png) | Tarjetas de lectura con el mismo alto, unidades y foco visible en «Registrar en ficha». Se muestran los datos de cada versión. |
| Gráfica | [Pantalla](original-historico-grafica.png) | [Pantalla](extraccion-historico-grafica.png) | Histórico con pestañas, filtro y gráfica por unidad. La extracción muestra dos series de presión. |
| Tabla | [Pantalla](original-historico-tabla.png) | [Pantalla](extraccion-historico-tabla.png) | Tabla de tomas con columnas de presión, registro en ficha y referencias. La extracción usa identificadores persistidos e instantes UTC. |

El original mostró **128/78 mmHg y pulso de 112/min** en un solo procedimiento.
La versión de ensayo mostró **148/85 mmHg** y la toma separada de pulso devolvió
**102/min**. Las primeras cifras vienen del triaje del HTML y las segundas
coinciden con el triaje inicial de LR en su guion. [Datos de Semana 5](../../../../../docs/referencias/html-semana5/Paciente_Virtual.html) ·
[Guion de LR](<../../../../../datos/medsp1000/mededportal_10181/scenario1/sp_actor/Case A-Asthma.md>).

En ambas ventanas se comprobó el mismo recorrido con teclado. `Tab` alcanza
«Preparar el brazalete», activado con `Enter`. El foco pasa a la primera
casilla. Se confirma cada paso con `Espacio` y se avanza con `Tab` hasta
«Medir presión», activado con `Enter`. Al terminar se llega a «Registrar en
ficha» con un `Tab` y se confirma con `Enter`. Dos `Tab` llevan a «Histórico».
En la gráfica, tres `Tab` llevan a «Tabla» y `Espacio` la activa. En los ocho
hitos apareció `:focus-visible` con contorno sólido de 3 px.

La primera invocación de la prueba pasó las rutas de evidencia como archivos de
Node. Se ajustó el comando para pasarlas por variables de entorno. Se corrigió
la selección del desplegable de preguntas, que el original ya abre, y el alcance
del selector de histórico. La compilación de ejemplo comparte `dist/` con el
servidor integrado, por lo que las corridas finales se ejecutaron en serie.
Pasó la regresión T028. La toma cancelada dejó cero peticiones, los dos
reenvíos de guardado dieron el mismo acuse y ficha e histórico mostraron
una sola toma.

[Mediciones, huellas y foco por estado](comparacion.json).
