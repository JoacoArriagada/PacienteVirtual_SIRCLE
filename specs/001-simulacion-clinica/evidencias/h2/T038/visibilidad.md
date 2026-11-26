# T038 · Reserva del paquete público y de las respuestas

26 de noviembre de 2026, 00:14 CLST · `feature/prototipo` · revisión base `fbc3f1ca`

## Procedimiento

Desde `backend/`, compilar con `cargo build --locked --offline` y ejecutar las
pruebas de acceso con `cargo test --locked --offline --test visibilidad -- --nocapture`.
Preparar una base nueva por corrida con
`target/debug/sircle preparar --db "$DB" --fuentes ../datos`, usando una ruta
absoluta para `DB`. Arrancar `target/debug/sircle servir --db "$DB"` y
`npm run dev:integrado` desde `frontend/`. En este computador se usó
`unshare -Urn` con la interfaz local activa por el uso previo del puerto 8080.
Desde `frontend/` ejecutar:

```bash
npm run test:navegador:reserva -- ../specs/001-simulacion-clinica/evidencias/h2/T038 "$DB"
```

El guion lee cada archivo generado en `dist/`, verifica su contenido y compara
su SHA-256 con el recurso descargado por la sesión real. Examina los cuerpos
de las respuestas HTTP y el DOM antes del pre, durante la atención y en el
estado entregado. Usa una clave, una ponderación y un borrador docente en una
tabla auxiliar de ensayo de la base temporal, un borrador real de otro
estudiante, un hecho de entrevista todavía reservado, el texto fuente de una
traza docente y el HTML íntegro con su huella. El apunte del titular y un
turno autorizado son controles positivos. El ECG se mantiene sin ordenar
durante este ensayo.

## Resultado

Se cotejaron **69 archivos y 96 respuestas HTTP** con Chromium 156.0.8078.4.
Hubo **cero coincidencias de material reservado** y cada descarga pública
coincidió con la huella del archivo inventariado.

| Fase | Respuestas | Controles observados |
|---|---:|---|
| Antes del pre | 81 | La vista omite secciones clínicas. Las cinco URL de archivos privados devuelven 404 y la vista ajena devuelve 403. |
| Atención | 9 | La sesión muestra el turno autorizado y el apunte propio. La traza del turno y el intento ajeno devuelven 403. El informe sin confirmar devuelve 404. |
| Antes de publicar | 6 | La vista conserva el turno y el apunte del titular. Las consultas de traza y de intento ajeno devuelven 403. Las rutas de devolución y evaluación devuelven 404 en este hito. |

En Rust, el estudiante recibió 403 al pedir la traza y el docente asignado
recibió 200 con el pasaje testigo, lo que confirmó que el material restringido
existía. La prueba de navegador simuló el estado `entregado` en SQLite. En
este hito la encuesta, la evaluación y la publicación todavía no cuentan con
rutas activas. T079 repetirá la inspección con las pantallas y el flujo
completo de la versión aprobada.

[Inventario y huellas](inventario.tsv) · [Respuestas por fase](respuestas.json) ·
[Prueba de servidor](servidor.txt).
