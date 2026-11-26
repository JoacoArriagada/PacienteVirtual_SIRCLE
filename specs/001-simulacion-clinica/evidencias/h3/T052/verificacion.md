# T052 · Dos preguntas libres de LR en paralelo

26 de noviembre de 2026, 07:42 CLST · `feature/prototipo` · revisión base `fbc3f1ca`

## Procedimiento

Preparar una base temporal y asignar al segundo estudiante ficticio dentro
de esa copia:

```sql
PRAGMA foreign_keys=ON;
INSERT INTO asignaciones VALUES ('lr-ensayo','estudiante-sin-asignacion','docente-demo');
```

Desde `backend/`, arrancar la API con `target/debug/sircle servir --db "$DB" --direccion 127.0.0.1:18081`,
redirigir stdout y stderr a un archivo nuevo y mantener Ollama local activo.
Ejecutar desde `backend/` con rutas absolutas:

```bash
SIRCLE_URL=http://127.0.0.1:18081 SIRCLE_T052_DB="$DB" SIRCLE_T052_LOG="$LOG_AXUM" \
  cargo test --locked --offline --test agente_lr aisla_dos_preguntas_simultaneas -- --ignored --nocapture
```

La prueba abre dos sesiones y dos intentos, resuelve por separado el pre
y verifica que estén vacíos. Despacha las dos peticiones con `tokio::join!`.
Compara las marcas de recepción y confirmación de Axum, las invocaciones
de Qwen, los acuses y los intentos reabiertos con sesiones nuevas. Cada
sesión intenta leer el otro intento. Verificar `size_vram=0` en el
inventario de procesos de Ollama.

## Resultado

Los dos ingresos a Axum precedieron a la primera confirmación.
El modelo atendió en el canal de LR y el solapamiento se midió con
las marcas del servidor, que incluyen el tiempo de cola. Entre el segundo
ingreso y la primera confirmación transcurrieron 22.361 ms. El estudiante
A recibió la cronología de dos días. B recibió el desconocimiento
familiar por adopción. Cada intento ganó un turno con identificadores
propios. Al reabrir coincidieron los acuses y las dos lecturas ajenas
devolvieron 403. La prueba real y la regresión ordinaria pasaron.

[Línea de tiempo e identificadores](simultaneas.tsv) ·
[Operaciones completas y trazas](simultaneidad.json) · [Regresión Rust](rust.txt).
