# T061 · Distinguir error, omisión e incidencia de registro

26 de noviembre de 2026 · `feature/prototipo` · revisión base `fbc3f1ca`

## Procedimiento

Elegir una ruta de base que aún no exista. Desde la raíz:

```bash
SIRCLE_T061_DB="$PWD/backend/target/t061-incidencia.sqlite" \
  cargo test --manifest-path backend/Cargo.toml --locked --offline \
  --test agente_evaluador --test entregas --test rubricas -- --nocapture
```

La regresión prepara tres intentos de ensayo con una misma rúbrica
de integridad. En A registra y consulta el ECG, y guarda un apunte
con la decisión equivocada. En B entrega sin confirmar el ECG,
aunque la versión clínica lo ofrece. En C confirma una orden y
retira artificialmente su informe y evento mediante SQLite dentro
de la prueba. La API conserva el acuse al reenviar la misma clave
y devuelve 404 al consultar el evento.

Tras las tres entregas, el trabajador se conecta a un doble local
de Ollama. El servicio compara los acuses propios hasta la revisión
confirmada con los eventos de la instantánea. La falta de un evento
se congela antes de ejecutar la evaluación. Se comprueba la lectura
docente, la justificación del juicio, los IDs y el criterio pedido
al motor. La copia retenida permite reproducir en T063 el intento de
publicar una revisión que todavía tiene la incidencia.

## Resultado

A conservó el error con las citas del apunte y del informe textual.
B conservó la omisión con el ECG habilitado, registro íntegro y
cero acuses pendientes. C conservó el acuse aunque faltaba la orden
en la instantánea. El criterio de decisiones de C quedó como
`revision_docente`, con incidencia `acuse_sin_evento` y
`descuento_automatico: 0`. El criterio y el ID ausente quedaron fuera
de la solicitud al motor. Los tres trabajos terminaron una sola vez.
La copia pasó `PRAGMA integrity_check` y contiene la evaluación
pendiente de revisión para T063. Pasaron quince pruebas Rust.

[Tres juicios, acuses e incidencia persistida](incidencia.json).
