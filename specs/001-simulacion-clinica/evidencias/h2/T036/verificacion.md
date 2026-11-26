# T036 · Hipótesis parciales y referencias al intento

26 de noviembre de 2026, 00:02 CLST · `feature/prototipo` · revisión base `fbc3f1ca`

## Procedimiento

Crear una base nueva mediante `target/debug/sircle preparar --db "$DB" --fuentes ../datos`
desde `backend/` tras `cargo build --locked --offline`. Usar una ruta absoluta.
Asignar temporalmente la segunda identidad ficticia a la actividad dentro de
esa base:

```bash
sqlite3 "$DB" "PRAGMA foreign_keys=ON; INSERT INTO asignaciones VALUES ('lr-ensayo','estudiante-sin-asignacion','docente-demo')"
```

Arrancar `target/debug/sircle servir --db "$DB"` y `npm run dev:integrado`
desde `frontend/`. En este computador se usó una red local aislada con
`unshare -Urn` e `ip link set lo up` por el uso previo del puerto 8080.
Desde `frontend/`:

```bash
npm run test:navegador:hipotesis -- ../specs/001-simulacion-clinica/evidencias/h2/T036 "$DB"
```

El guion abre dos intentos y obtiene un turno autorizado en cada uno. Registra
una toma de presión y un ECG consultado en el primero. Desde «Fundamentar una
hipótesis» guarda únicamente el título principal, vuelve a abrir la atención,
completa textos y marca referencias a turno, medición e informe. Envía una
referencia del otro intento, una escritura con la revisión antigua y el reenvío
idéntico del primer guardado. Comprueba la reapertura y coteja los vínculos
en SQLite.

## Resultado

En la revisión 5 se guardó un título principal sin fundamento ni citas. Tras
reabrir, la revisión 6 guardó la hipótesis principal con turno y medición y una
alternativa con el informe. Una referencia del otro estudiante devolvió 422.
La escritura con la revisión 5 devolvió 409. Reenviar la clave y el contenido
originales recuperó el acuse de la revisión 5 con HTTP 200. La vista y SQLite
conservaron la revisión 6 con sus tres vínculos, y al reabrir coincidieron
los dos títulos, las dos explicaciones y las selecciones. El intento de la
corrida final fue `808b751a54baa3787c853414475eab8e`. La regresión T035 pasó
con otros dos intentos tras compartir la ruta de escritura. Pasaron además
las nueve pruebas ordinarias de Rust y las dos pruebas de frontend. Las
cuatro pruebas de Ollama requieren ejecución explícita y constan en H1.

[Acuses y rechazos](hipotesis.json) · [Revisiones y vínculos en SQLite](sqlite.tsv) ·
[Borrador parcial](parcial.png) · [Formulario reabierto](reabierta.png).
