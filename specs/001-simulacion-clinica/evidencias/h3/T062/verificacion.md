# T062 · Rectificar y reproducir una evaluación con su rúbrica original

26 de noviembre de 2026 · `feature/prototipo` · revisión base `fbc3f1ca`

## Procedimiento

Elegir una ruta de salida todavía inexistente y recrear la base de
T056 si hace falta. Desde la raíz:

```bash
SIRCLE_T056_DB="$PWD/backend/target/t056-calibracion.sqlite" \
SIRCLE_T062_DB="$PWD/backend/target/t062-revisiones.sqlite" \
  cargo test --manifest-path backend/Cargo.toml --locked --offline \
  --test agente_evaluador --test intentos --test acceso --test rubricas -- --nocapture
npm --prefix frontend test
```

La serie nueva abre A con V1, entrega y guarda la revisión 1 con
el trabajador y un doble local del motor. Solo entonces activa V2 y
abre B. Lee la revisión inicial y recalcula su desglose con la
rúbrica de A. Rectifica la valoración como docente, reenvía con
la misma clave y prueba un envío nuevo con revisión obsoleta.
Consulta otra vez la revisión histórica y la vigente. En otra copia
se calculan los dos intentos retenidos de T055 y T056 con sus
respectivas versiones.

La regla de cálculo usa créditos enteros de 0 a 100 y el peso
fijado en la rúbrica. Por criterio se conserva el producto de
peso y crédito. El total divide la suma de productos por el
peso aplicable y se guarda en centésimas. Una revisión docente
pendiente conserva el desglose y deja el total definitivo sin valor.

Para el navegador, copiar la base guardada de T062. Arrancar
`sircle servir --db "$DB"` y `npm run dev:integrado`, con ambos
servicios en el mismo espacio de red que Chromium. La corrida
usó `unshare -Urn bash` e `ip link set lo up`, porque 8080 estaba
ocupado en el anfitrión. Desde la raíz ejecutar en esa red:

```bash
npm --prefix frontend run test:navegador:revision-docente -- "$EVIDENCIA"
```

El navegador entra como `docente-demo`, abre la entrega, cambia el
crédito de valoración, registra un motivo y reabre la revisión ya
guardada tras recargar la página.

## Resultado

La revisión 1 de A obtuvo 40 sobre 100 y estaba guardada antes de
activar V2. Releerla y recalcularla tras activar V2 devolvió las
mismas ponderaciones, juicios y total. La rectificación conservó al
docente, el motivo, la referencia a la revisión 1 y el cambio de
crédito de valoración de 100 a 75. La revisión 2 obtuvo 35. El reenvío
devolvió 200 y el intento de rectificar con una revisión obsoleta
devolvió 409. Una sesión ajena devolvió 403 y un motivo vacío o
una cita ajena devolvieron 422. Los intentos retenidos de T055 y
T056 dieron 40 en V1 y 30 en V2 con sus propios pesos.

Chromium guardó una revisión 3 con crédito 50 para valoración y
total 30. La vista recuperó la misma revisión al recargar. En una
copia de T061, la migración conservó las tres evaluaciones, incluida
la incidencia con cero descuento automático y sin total definitivo.
Pasaron dieciocho pruebas Rust y la compilación y pruebas de frontend.

[Revisiones y reproducción de V1](revisiones.json) ·
[Base T061 migrada](migracion-t061.json) · [Acuse del navegador](navegador.json) ·
[Desglose](desglose.png) · [Rectificación](rectificada.png).
