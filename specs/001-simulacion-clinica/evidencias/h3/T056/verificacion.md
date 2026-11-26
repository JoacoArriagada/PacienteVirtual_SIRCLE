# T056 · Confirmar una entrega parcial y encolarla de forma atómica

26 de noviembre de 2026 · `feature/prototipo` · revisión base `fbc3f1ca`

## Procedimiento

Recrear la base de T055 si es necesario y elegir para T056 una ruta de
salida aún inexistente. La prueba trabaja sobre una copia consistente.

```bash
export SIRCLE_T055_DB="$PWD/backend/target/t055-calibracion.sqlite"
export SIRCLE_T056_DB="$PWD/backend/target/t056-calibracion.sqlite"
cargo test --manifest-path backend/Cargo.toml --locked --offline \
  --test entregas --test intentos --test valoracion --test visibilidad --test acceso -- --nocapture
npm --prefix frontend test
```

La prueba registra en A una toma de presión, una orden de ECG aún sin
consulta y una hipótesis parcial con el ID de la toma. Un trigger de
prueba aborta el registro del acuse después de insertar entrega y trabajo
y de cambiar el intento, justo antes de confirmar la transacción. Se
comparan los recuentos y la revisión después del error. Sin el trigger,
un adaptador de prueba en loopback retiene una respuesta válida del
trabajador de LR. Mientras esa respuesta espera, se envía la entrega
dos veces en paralelo. Se guarda la instantánea, se libera la respuesta
y se cotejan los bytes guardados y los recuentos. La prueba conserva la
base resultante para continuar con el evaluador.

Para la prueba de navegador, copiar la base de T055 a una ruta temporal
y ejecutar las migraciones con `backend/target/debug/sircle preparar
--db "$DB" --fuentes datos`, donde `DB` es la copia. Arrancar
`backend/target/debug/sircle servir --db "$DB"` y, desde `frontend/`,
`npm run dev:integrado`. Los dos servicios de esta corrida se levantaron
en el mismo espacio de red aislado con `unshare -Urn bash` y
`ip link set lo up`, porque el puerto 8080 del anfitrión estaba ocupado.
En ese mismo espacio ejecutar:

```bash
npm --prefix frontend run test:navegador:entrega -- "$EVIDENCIA" "$DB"
```

El navegador entra a A, revisa el trabajo vacío y simula la pérdida de
la primera respuesta tras llegar a Axum. Repite con la misma clave
desde el botón y reabre el acuse en «Finalizados».

## Resultado

El fallo de SQLite devolvió 500 y dejó cero entregas, trabajos y
acuses, con A aún en atención y en revisión 4. El reintento confirmó una
entrega, un trabajo pendiente y un acuse. La revisión 4 congelada retuvo
el ID, versión y huella de V1, además de la hipótesis parcial, la medición
y la orden. V2 siguió activa y B permaneció abierto con V2. La respuesta
retenida y la primera consulta tardía del informe devolvieron 409. Hubo
cero turnos nuevos y la instantánea conservó los mismos bytes. Pasaron
las seis pruebas Rust. La base retenida pasó `PRAGMA integrity_check`.

[Acuse, instantánea y recuentos de SQLite](entrega.json).

Chromium recibió 201 en el primer envío simulado y 200 tras el
reintento, con contenido y clave idénticos. La revisión mostró el trabajo
vacío y permitió seguir editando. «Finalizados» reabrió el mismo acuse
en una vista sin botón de revisión. Se reutilizaron los iconos, clases
y textos de la pantalla original de Semana 5. La compilación de ejemplo
y sus dos pruebas ordinarias pasaron. Las pruebas de paridad y reserva
usadas en T037 y T038 requieren sus propios datos de navegador, que no
se configuraron en esa ejecución de `npm test`.

[Acuses del navegador](navegador.json) · [Revisión](revisar.png) ·
[Confirmación](confirmada.png).
