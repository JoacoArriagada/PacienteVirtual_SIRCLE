# T063 · Publicar una revisión y habilitar su lectura al cerrar el post

26 de noviembre de 2026 · `feature/prototipo` · revisión base `fbc3f1ca`

## Procedimiento

Usar las bases retenidas de T061 y T062. Cada prueba trabaja sobre
una copia y aplica las migraciones. Desde la raíz:

```bash
SIRCLE_T063_REVISION_DB="$PWD/backend/target/t062-revisiones.sqlite" \
SIRCLE_T063_INCIDENCIA_DB="$PWD/backend/target/t061-incidencia.sqlite" \
  cargo test --manifest-path backend/Cargo.toml --locked --offline \
  --test agente_evaluador --test visibilidad --test acceso -- --nocapture
cargo test --manifest-path backend/Cargo.toml --locked --offline \
  --test agente_evaluador publica -- --nocapture
```

La matriz publica la revisión 1 de A y verifica que el titular solo
ve el estado del post pendiente. Marca el post como respondido en
el modo de ensayo y comprueba que aparezca exclusivamente la revisión
1, aunque exista una rectificación. Publica la revisión 2 y vuelve a
enviar el primer acuse para verificar que el titular siga viendo la
segunda revisión. B omite el post de ensayo antes de la publicación.
La prueba coteja el acceso entre estudiantes y entre docentes.
En una copia de T061 intenta publicar el borrador con acuse sin evento.

Para verificar en Chromium, preparar otra copia de T062 con
`sircle preparar --db "$DB" --fuentes datos`. Arrancar `sircle servir`
sobre esa copia y `npm run dev:integrado` desde `frontend/`. Esta
corrida usó una red aislada `unshare -Urn bash` con loopback habilitado
porque 8080 estaba ocupado. Desde la raíz, en esa red:

```bash
npm --prefix frontend run test:navegador:publicacion -- "$EVIDENCIA" "$DB"
```

El navegador publica la revisión 2 como docente. Con otra sesión
abre la entrega del titular antes y después de omitir el post
mediante `sircle resolver-post-ensayo`. El formulario de post se
verificará por interfaz en T067.

## Resultado

El titular con post pendiente recibió solo `post_pendiente` antes
y después de publicar. Tras responder vio la revisión 1. La
rectificación guardada apareció solo al publicar la revisión 2.
Reenviar la primera publicación devolvió su acuse original y
conservó la revisión 2 como vigente. B, con el post omitido, vio
el estado pendiente hasta publicar su revisión 1. Las lecturas
cruzadas y los intentos de publicación ajenos devolvieron 403.
La evaluación con incidencia de T061 devolvió 409 y conservó cero
publicaciones.

Chromium publicó la revisión 2 y la mostró solo después de la
omisión de ensayo. Se comprobó además la entrega desde el navegador
en una base separada. Pasaron dieciocho pruebas Rust en la matriz
con bases retenidas, las tres pruebas autónomas y la compilación
con las pruebas de frontend.

[Matriz y bloqueo de publicación](publicacion.json) · [Respuestas del navegador](navegador.json) ·
[Publicación docente](publicada-docente.png) · [Post pendiente](post-pendiente.png) ·
[Devolución del titular](publicada-titular.png).
