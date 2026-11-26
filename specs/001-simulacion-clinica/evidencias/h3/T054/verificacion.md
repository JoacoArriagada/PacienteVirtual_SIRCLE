# T054 · Cargar una rúbrica privada con constancia de ensayo

26 de noviembre de 2026 · `feature/prototipo` · revisión base `fbc3f1ca`

## Procedimiento

```bash
cd backend
cargo build --locked --offline
cargo test --locked --offline --test rubricas --test perfil_evaluador --test preparacion -- --nocapture
DB="$(mktemp -d)/intentos.sqlite"
target/debug/sircle preparar --db "$DB" --fuentes ../datos
target/debug/sircle cargar-rubrica-ensayo --db "$DB" --archivo tests/fixtures/rubricas/lr-v1.json
```

En una copia de `lr-v1.json`, añadir al objeto raíz `aprobacion` con el
tipo `ensayo`, docente `docente-demo`, fecha `2026-11-26T00:00:00Z`,
un motivo ficticio de calibración y los campos `codigo`, `version` y
`sha256` devueltos por la carga inicial. Cargar esa copia dos veces.
Modificar otra copia sin aprobación cambiando los dos primeros pesos de
20 y 20 a 25 y 15. Ejecutar el mismo subcomando con esa copia. Consultar
`SELECT id, codigo, version, estado, sha256, contenido, constancia FROM rubricas`
y recalcular SHA-256 sobre los bytes de `contenido`.

## Resultado

La primera carga registró el ID 1 como borrador. La constancia de ensayo lo
pasó a aprobado y su reenvío conservó el acuse completo. Cambiar pesos bajo
la misma versión devolvió código 1. SQLite conservó una fila y el cálculo
externo sobre su contenido dio la huella registrada. Las once pruebas
pasaron, incluidas las de constancia con huella distinta, fecha inválida,
docente sin asignación y tipo formal por el cargador de ensayo. La aprobación
docente formal corresponde a T078.

[Acuses, criterio y huella observada](carga-rubrica.json).
