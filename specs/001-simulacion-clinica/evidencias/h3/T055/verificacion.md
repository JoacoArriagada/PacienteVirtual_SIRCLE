# T055 · Fijar la rúbrica al abrir cada intento

26 de noviembre de 2026 · `feature/prototipo` · revisión base `fbc3f1ca`

## Procedimiento

Desde la raíz, usar una ruta que aún no exista para la copia SQLite.

```bash
SIRCLE_T055_DB="$PWD/backend/target/t055-calibracion.sqlite" \
  cargo test --manifest-path backend/Cargo.toml --locked --offline \
  --test version_rubrica --test intentos --test rubricas --test contexto --test visibilidad -- --nocapture
```

La prueba genera constancias ficticias de ensayo para `docente-demo`,
activa V1 con `sircle habilitar-rubrica-ensayo` y abre A por la API.
Carga V2 como borrador, comprueba que su activación se rechaza y coteja
que V1 sigue activa. Carga la constancia de V2, la activa y abre B con
otro estudiante. Recrea el router sobre el mismo almacén, autentica a
cada titular y compara las lecturas y los reenvíos con las altas.
Consulta la rúbrica privada por su ID, versión y huella, y vuelve a
calcular SHA-256. Comprueba también las lecturas cruzadas. SQLite crea
la copia consistente al final mediante `VACUUM INTO`.

## Resultado

A reabrió con ID de rúbrica 1, versión 1 y peso 20 para entrevista.
B reabrió con ID 2, versión 2 y peso 30. Cada lectura y reenvío del
titular devolvió 200 y cada lectura cruzada devolvió 403. Los criterios
permanecieron en el almacén privado. La activación del borrador devolvió
código 1. Pasaron las seis pruebas Rust. La copia
`backend/target/t055-calibracion.sqlite` conserva ambos intentos y
V2 activa, con `PRAGMA integrity_check` igual a `ok`.

También se migró una copia de la base técnica previa con
`sircle preparar --db <copia> --fuentes datos`. Conservó su intento y
los cinco turnos originales. La versión técnica previa conservó su
etiqueta, sin asignación retroactiva de una rúbrica.

[Asignaciones, respuestas y pesos](intentos-v1-v2.tsv) · [Detalle de SQLite](asignaciones.json).
