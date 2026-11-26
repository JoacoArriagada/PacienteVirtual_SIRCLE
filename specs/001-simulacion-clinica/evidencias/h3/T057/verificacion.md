# T057 · Reconstruir la entrada privada de una entrega

26 de noviembre de 2026 · `feature/prototipo` · revisión base `fbc3f1ca`

## Procedimiento

```bash
SIRCLE_T056_DB="$PWD/backend/target/t056-calibracion.sqlite" \
  cargo test --manifest-path backend/Cargo.toml --locked --offline \
  --test agente_evaluador --test contexto --test rubricas -- --nocapture
```

La prueba abre una copia de la base retenida de T056. Si se omite la
variable, prepara una base nueva con el mismo contraste V1/V2. Guarda
un apunte en B y verifica que el intento abierto siga sin trabajo de
evaluación. Introduce cuatro testigos ficticios, uno en ese apunte,
dos en encuestas y uno en claves educativas. Las dos tablas temporales
de instrumentos son fixtures de aislamiento. La implementación del
instrumento corresponde a las tareas educativas del hito siguiente.

Recupera A por el ID de su entrega confirmada y compara la entrada
serializada con la revisión, versión clínica, rúbrica y referencias de
la instantánea en SQLite. Cambia el valor de la medición viva en la copia
de ensayo y repite la lectura. Al final altera los bytes de la rúbrica
en la copia y comprueba que el constructor rechace la huella alterada.

## Resultado

La entrada recuperó la revisión 4 de A con V1 y la misma huella de
T056. Conservó dos referencias propias, la medición y la orden de
ECG, y la hipótesis citó la medición. El informe sin consultar quedó
fuera. Alterar el valor vivo dejó intacta la entrada reconstruida.
Alterar la rúbrica devolvió «La huella de la rúbrica congelada no coincide».
Los cuatro testigos quedaron fuera y B conservó cero trabajos de
evaluación. Pasaron las doce pruebas Rust y la repetición autónoma
que genera su propia base.

[Entrada efectiva e instantánea comparada](entrada-evaluador.json).
