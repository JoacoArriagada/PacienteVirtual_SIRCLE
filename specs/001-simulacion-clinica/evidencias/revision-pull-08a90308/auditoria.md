# Revisión del pull 08a90308

El reflog registró el pull que descargó `08a90308` en `origin/feature/prototipo`. La rama local todavía estaba en su padre `fbc3f1ca`. Se aplicó el avance directo sobre el árbol limpio. El commit trajo 628 archivos modificados y marcó T001 a T065 como completas.

Se cotejaron las 65 entradas y sus 65 filas de evidencia con los archivos del commit. Se repitieron las pruebas de integración en Windows con bases separadas. [Tareas y evidencias originales](../../tasks.md)

| Tareas | Comprobación en esta revisión | Ajustes |
| --- | --- | --- |
| T001–T020 | Preparación de 3 escenarios y 14 fuentes, migración, reenvío del alta, valoración y entrevista con Qwen real. Se repitieron el 503, el reenvío y la medición en frío y en caliente | Normalización de rutas en la prueba de preparación y recuperación del mismo intento en su estado actual |
| T021–T038 | 29 archivos de la referencia con huellas idénticas. Comparación en Chrome a 1280 × 900, 9 estados y 8 pasos de foco. La prueba de reserva examinó 79 archivos públicos y 112 respuestas sin encontrar material reservado | Rutas de módulos del frontend compatibles con Windows. La comparación conserva dimensiones y controles y registra diferencias de rasterización de hasta una unidad por canal en el interior |
| T039–T053 | Baterías de agentes, autorización, memoria y E17 con dos solicitudes reales y aisladas | `SystemRoot` para Winsock y una sola escritura por línea de traza. El reinicio de Nannu mediante `/proc` conserva su evidencia de Linux |
| T054–T065 | Versiones de rúbrica, entrega, revisión, publicación, recuperación y dos calibraciones E12 con Qwen en CPU. Se cotejaron los vínculos y regresiones de los 32 núcleos | Traza docente por criterio y consulta de rechazos de entrevista. El esquema de inferencia exige citar un registro admisible cuando existe |

Pasaron las 68 pruebas Rust de la suite normal y las 7 de ejecución explícita en Windows. Las latencias exploratorias de T019 fueron 22,0 s en frío y 8,2 s en caliente en la estación de desarrollo.

[Entrevista y recuperación](entrevista-windows.json) · [Latencias](latencias-t019-windows.json) · [Comparación visual](paridad-windows.json) · [Reserva de contenido](reserva-windows.json) · [E17 en Windows](e17-windows.json) · [E12 en CPU](e12-cpu-windows.json)

## Implementación del tramo pendiente

- T066–T069 quedaron verificadas con los dos recorridos de Chrome y las pruebas E14 y E15. E14 produjo tres invitados, dos pre, dos post, un par completo y un cambio de 50 puntos porcentuales en el razonamiento pareado. E15 conservó ambos órdenes tras altas concurrentes. Las pruebas de publicación comprobaron que el puntaje clínico se conserva al resolver el post. [Pre](../h4/T066/pre.md) · [Post](../h4/T067/post.md) · [Secuencias](../h4/T068/secuencias.tsv) · [Informe](../h4/T069/informe.tsv)
- T070–T077 cuentan con la configuración de Compose, inicialización, comprobación de modelo, respaldo y verificador E22. Se validó la configuración de Compose y se comprobó la disponibilidad de las cinco etiquetas de imágenes. El preparador confirmó tres intentos con un servidor simulado. La comprobación de generación se repitió con Qwen local. En procesos locales, E22 conservó la medición y la entrega y terminó una sola evaluación después de restablecer la conexión de Axum con Qwen. [Reintentos](../h4/T074/reintentos-local.txt) · [E22 local](../h4/T077/e22-local.json)
- T078–T080 requieren la aprobación docente de los tres materiales y la aceptación en su equipo. La prueba de integración verificó que el sistema bloquea el alta formal sin las constancias correctas y fija las versiones cuando las valida. [Procedimiento de aprobación y aceptación](../../quickstart.md#4-aceptar-la-demostración-en-el-equipo-del-profesor)
- T081–T084 conservan la dependencia de la aceptación formal. Los tres perfiles de carga están preparados. Se comprobó el ejecutor con cuatro sesiones ficticias y se dejó pendiente el estudio en el equipo docente.

La estación de desarrollo dispone de la CLI de Compose para validar la configuración. El arranque con un motor Docker, la restauración sobre volúmenes Docker y la aceptación quedan por comprobar en el equipo del profesor.
