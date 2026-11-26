# T041 · Autorizar antes de recuperar un acuse o inferir

26 de noviembre de 2026, 00:28 CLST · `feature/prototipo` · revisión base `fbc3f1ca`

## Procedimiento

Desde `backend/` ejecutar:

```bash
cargo build --locked --offline
cargo test --locked --offline --test autorizacion --test acceso --test intentos --test visibilidad -- --nocapture
```

La regresión modela cinco actores con sesiones DB reales, cuatro procesos
trabajadores y un servidor HTTP local de prueba que cuenta las peticiones
efectivas a `/api/chat`, incluida la operación que recibió. Abre un intento
con pre pendiente y prueba la entrevista. Luego resuelve el pre y confirma
una pregunta propia antes de medir el estado inicial de la matriz. Intenta
leer el recurso y su traza, reenviar la clave confirmada e iniciar una
operación nueva con las identidades ajenas. En esa copia de SQLite cambia
por separado la disponibilidad de la actividad, el rol y el estado del
intento. Tras cada petición compara el número de llamadas y turnos.

La comprobación adicional usa Ollama real en CPU y un intento nuevo en
Axum como en T040. Crear un turno libre con `Idempotency-Key: t041-ollama`.
Hacer GET de la vista y POST del mismo texto y clave desde
`estudiante-sin-asignacion`. Repetir desde esa misma sesión con la clave
`t041-ajena-nueva`. Reabrir desde el titular y reenviar su clave original.
Contar las líneas `SIRCLE_INFERENCIA` de Axum y comparar los turnos
recuperados antes y después.

## Resultado

La pregunta con pre pendiente devolvió 409 y dejó cero invocaciones y
cero turnos. Tras resolverlo, la pregunta autorizada devolvió 201 y dejó
una invocación y un turno. Leer o reenviar como estudiante ajeno devolvió
403. Su operación nueva también devolvió 403. El titular leyó el intento
y recuperó su acuse con 200. Solo el docente asignado leyó la traza con
200. Cambiar el rol o deshabilitar la actividad dejó el acuse reservado.
El intento entregado devolvió 409 ante una pregunta nueva y conservó el
acuse histórico del mismo titular. En toda la matriz quedaron una
llamada y un turno.

En la comprobación real se contó una invocación confirmada y un turno
tanto antes como después de los rechazos. La diferencia en cada recuento
fue cero. El turno real recuperado fue `2f943944ce347f043666a3f5ed5c9bed`.
Pasaron las cuatro pruebas de Rust.

La primera compilación detectó que la rutina de mediciones seguía usando
el tipo `Intento` tras sustituir las consultas de autorización. Se restituyó
su referencia y se repitió la compilación y la matriz.

[Matriz y recuentos HTTP](permisos.json) · [Comprobación con Qwen local](ollama-real.json).
