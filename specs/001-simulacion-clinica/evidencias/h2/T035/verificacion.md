# T035 · Apuntes y procedencia de evidencias

25 de noviembre de 2026, 23:56 CLST · `feature/prototipo` · revisión base `fbc3f1ca`

## Procedimiento

Desde `backend/`, compilar con `cargo build --locked --offline` y preparar
una base temporal nueva mediante `target/debug/sircle preparar --db "$DB" --fuentes ../datos`.
Usar una ruta absoluta para `DB`. En esa copia de ensayo, añadir la segunda
asignación ficticia para abrir dos intentos en el mismo servicio:

```bash
sqlite3 "$DB" "PRAGMA foreign_keys=ON; INSERT INTO asignaciones VALUES ('lr-ensayo','estudiante-sin-asignacion','docente-demo')"
```

Arrancar `target/debug/sircle servir --db "$DB"` y `npm run dev:integrado`
desde `frontend/`. En este computador se ejecutaron en una red local aislada
con `unshare -Urn` e `ip link set lo up` por el uso previo del puerto 8080.
Desde `frontend/` ejecutar el guion:

```bash
npm run test:navegador:apuntes -- ../specs/001-simulacion-clinica/evidencias/h2/T035 "$DB"
```

El guion usa Chromium 156.0.8078.4 y resolvió el pre técnico por separado.
En el primer intento escribe «solicitaría el ECG» y consulta la disponibilidad
del informe sin entrar en la confirmación de orden. En el segundo registra
una respuesta literal autorizada de LR, una toma completa de presión, una
orden confirmada con su lectura y un apunte interpretativo. Reabre este
intento por «En curso → Continuar», coteja cada evidencia con su registro
de origen y comprueba el rechazo de lectura del intento ajeno.

## Resultado

El intento `88416e506c40efaf23bdb71720902788` guardó el apunte con autor y
momento. La consulta del ECG devolvió 404 y el recuento de órdenes fue cero.

El intento `1a4089db66d8b2dbbd0e562e668bf1e1` conservó cinco evidencias
al reabrir. El navegador mostró sus categorías, orígenes y referencias:

| Categoría | Autor u origen | ID de origen |
|---|---|---|
| Declaración | LR, entrevista | `153a6334834a4d0f43cdacabcbdf44e2` |
| Dato medido | Valoración instrumental, presión 148/85 mmHg | `0daaf60f472cbc5022b19df4dc6256b0` |
| Orientación | Docente, ayuda de la actividad | `orientacion-inicial-lr` |
| Informe | Electrocardiograma inicial consultado | `70286ad41bd18d8deb0ea4cf867c8439` |
| Interpretación | Apunte de la identidad ficticia del segundo intento | `a72edd2b8c3325e4cd745ac3df2132c7` |

La vista de SQLite confirmó un apunte en cada intento y una única orden en el
segundo. La orientación tiene su referencia estable dentro del intento y se
proyecta desde la misma ayuda que se muestra en ficha. El segundo estudiante
recibió 403 al consultar el primer intento.

[Acuses y orígenes](evidencias.json) · [Recuentos de SQLite](sqlite.tsv) ·
[Apunte sin orden](apunte-sin-orden.png) · [Evidencias reabiertas](evidencias-reabiertas.png).
