# T033 · Auscultación por punto

25 de noviembre de 2026, 23:48 CLST · `feature/prototipo` · revisión base `fbc3f1ca`

## Procedimiento

Crear una base temporal desde `backend/` con `cargo build --locked --offline` y
`target/debug/sircle preparar --db "$DB" --fuentes ../datos`, con `DB` en una ruta absoluta.
Arrancar el backend con `target/debug/sircle servir --db "$DB"` y en `frontend/`
usar `npm run dev:integrado`. En este computador se ejecutaron los servidores del
ensayo en una red local aislada con `unshare -Urn` e `ip link set lo up` para conservar
el servicio de otro proyecto que usa el puerto 8080. Ejecutar desde `frontend/`:

```bash
npm run test:navegador:auscultacion -- ../specs/001-simulacion-clinica/evidencias/h2/T033 "$DB"
```

La prueba abre el HTML privado y la aplicación con Chromium 156.0.8078.4,
1280 × 900, zoom 100 % y escala 1. En cada interfaz confirma los dos pasos,
selecciona vista y punto y verifica que solo entonces se habilite «Auscultar».
Durante la acción activa «Escuchar audio de práctica». Instrumenta Web Audio para
capturar la señal mono entregada a la salida, el estado del contexto, el bucle y
su parada. Usa la misma semilla en ambas ventanas únicamente para comparar el
ruido de la prueba. Los WAV exportan un ciclo de señal con la ganancia aplicada.
La prueba observa la salida de Web Audio y permite escuchar los WAV en un
reproductor. No se realizó una escucha humana por los parlantes.

## Resultado

| Vista y punto | Audio de práctica | Resultado en el intento |
|---|---|---|
| Espalda, Base izquierda | Ruido respiratorio sintético de envolvente suave, 2,4 s a 48 kHz. Señal no nula, RMS 0,01694. | Acción registrada y un hallazgo citable de la versión de ensayo de LR, con revisión de fuente 1. |
| Corazón, Foco mitral | Dos pulsos cardíacos sintéticos amortiguados, 0,82 s a 48 kHz. Señal no nula, RMS 0,00826. | Acción registrada, interfaz «Hallazgo pendiente», cero hallazgos nuevos. |

Coincidieron los recortes de la escena seleccionada y la huella de cada WAV
entre original y extracción. Ambos reproductores llegaron a estado `running`
con ganancia 0,24 y bucle conectado a la salida. El original se detuvo desde el
control y la extracción al terminar la toma. El intento
`97b07dcf74319fdf35a3cd113e9cbff4` recuperó por HTTP las dos acciones. La unión
en SQLite confirmó exactamente un hallazgo.

El primer recorrido del original había dejado abierta la confirmación de salida
al volver con la toma en curso. Se corrigió el guion para completar la toma con el
reloj simulado y usar «Repetir toma» al volver a acceder a Auscultación mediante
el identificador estable del botón, porque su nombre incluye el contador. Se
repitieron ambos puntos con una base nueva.

## Evidencia

- Base izquierda. [Gesto original](postLeft-original.png), [gesto extraído](postLeft-extraccion.png), [audio en curso](postLeft-reproduccion.png), [señal WAV](postLeft.wav), [resultado original](postLeft-resultado-original.png), [resultado del ensayo](postLeft-resultado.png).
- Foco mitral. [Gesto original](mitral-original.png), [gesto extraído](mitral-extraccion.png), [audio en curso](mitral-reproduccion.png), [señal WAV](mitral.wav), [resultado original](mitral-resultado-original.png), [estado pendiente](mitral-resultado.png).
- Huellas de escenas y audio, acuses HTTP y filas de la base. [JSON](acciones.json), [SQLite](sqlite.tsv).
