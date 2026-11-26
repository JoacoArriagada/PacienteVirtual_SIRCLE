# T034 · Confirmar la orden y consultar el ECG inicial

25 de noviembre de 2026, 23:51 CLST · `feature/prototipo` · revisión base `fbc3f1ca`

## Procedimiento

Desde `backend/` ejecutar `cargo build --locked --offline` y crear una base de
ensayo nueva con `target/debug/sircle preparar --db "$DB" --fuentes ../datos`,
usando una ruta absoluta. Arrancar `target/debug/sircle servir --db "$DB"` y
`npm run dev:integrado` desde `frontend/`. En este computador se usó
`unshare -Urn` con la interfaz local activa para conservar el servicio externo
al módulo que ocupa el puerto 8080. Desde `frontend/` ejecutar:

```bash
npm run test:navegador:ecg -- ../specs/001-simulacion-clinica/evidencias/h2/T034 "$DB"
```

El guion consulta el informe con el pre pendiente, durante la atención y después
de cancelar una solicitud. Intenta enviar una orden sin confirmar y vuelve a
consultar. Confirma en la interfaz, abre el informe por su orden, reenvía la
confirmación con la misma clave y con otra, reabre el intento y vuelve a leer.
Se coteja la procedencia con la unión de tablas:

```sql
SELECT o.intento_id, o.id AS orden_id, f.id AS informe_id, f.resultado,
       fu.rol, r.sha256, f.consultado_en
FROM ordenes o
JOIN informes f ON f.orden_id = o.id
JOIN revisiones_fuente r ON r.id = f.revision_fuente_id
JOIN fuentes fu ON fu.id = r.fuente_id;
```

## Resultado

Las consultas previas devolvieron 404 con «El informe está reservado hasta
confirmar la orden de este intento». La solicitud sin confirmar devolvió 422 y
la lista de órdenes permaneció vacía. La confirmación devolvió 201. El GET
posterior devolvió 200 con el texto «taquicardia sinusal» y la vista pudo
mostrarlo sin recursos de imagen. Los dos reenvíos devolvieron 200 con el mismo
acuse. La lista final y SQLite conservaron una sola orden y un solo informe.
La reanudación devolvió el mismo informe y se guardó el instante de la primera
consulta. La fuente enlazada es el controlador del escenario de LR. Se verificó
la versión de ensayo, cuya aprobación formal corresponde a T078.

La regresión de alta y reanudación comparaba los metadatos con la vista completa.
Se amplió el aserto para comprobar que la nueva lista esté vacía al abrir y
excluirla de esa comparación. El reintento de las pruebas de acceso, intentos
y router pasó, con tres pruebas exitosas. También pasaron las dos de frontend.

[Avisos y respuestas HTTP](orden-ecg.json) · [Fila de SQLite](sqlite.tsv) ·
[Informe reservado](reservado.png) · [Confirmación](confirmacion.png) · [Vista textual](informe.png).
