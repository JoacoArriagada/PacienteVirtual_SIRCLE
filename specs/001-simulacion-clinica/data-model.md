# Modelo de datos y reglas de integridad

Propuesta SQLite. Las claves foráneas se habilitan en cada conexión. Tiempos del sistema y momento clínico se guardan por separado. El servidor obtiene identidad y permisos de la sesión local.

| Entidad | Campos y relaciones relevantes |
|---|---|
| Identidad y asignación | Identificador ficticio, rol, actividad, estudiante y docente responsable. La asignación determina quién puede acceder. |
| Fuente, versión clínica y hecho | Caso P01/P02/P03, rol de origen, ruta, contenido y huella. Una versión publicada fija los hechos aprobados, cada uno con identificador, pasaje, hablante, fase y regla de disponibilidad. La semilla es repetible. Referencias a medios ausentes se conservan como tales. |
| Rúbrica e instrumento | Versiones inmutables con criterios, evidencia exigida y ponderaciones docentes. El instrumento guarda enunciados, formas A/B, claves y anclajes. Solo el servidor recibe las claves. |
| Intento | Propietario, actividad, versiones fijadas, fase, revisión y estado en curso o entregado. La transacción inicial asigna y conserva la secuencia A→B o B→A por actividad. |
| Evento e hipótesis | ID de intento, tipo, operación, instante, actor, estado y origen. El turno referencia hablante y hecho; la medición conserva valor y unidad; la orden confirmada de ECG habilita el informe textual. La hipótesis enlaza eventos del mismo intento. |
| Cuestionario | Un registro por intento y momento, con versión, forma, respuestas por ítem u omisión. El pre se cierra antes del acceso clínico. Tras la entrega, la devolución publicada se habilita al responder u omitir el post. |
| Entrega y evaluación | Una entrega por intento, con copia de hipótesis y evidencias. Un trabajo de evaluación por entrega, con estado pendiente, en curso, revisable o publicado. Las revisiones conservan rúbrica, criterio, cita, versiones del modelo, motivo de rectificación y docente. |

## Operaciones que deben ser atómicas

- Cada escritura reintentable lleva un identificador de operación único por intento. Un reenvío idéntico recupera el resultado. Un identificador reutilizado con otro contenido produce conflicto. Dos tomas iniciadas deliberadamente tienen identificadores distintos.
- La entrega y su trabajo pendiente se confirman juntos. La inferencia ocurre fuera de la transacción. Una toma de trabajo con vencimiento permite recuperar un trabajador caído. El cierre comprueba que la toma siga vigente y conserva un solo resultado válido.
- Una entrega congela su revisión. Solo se publican revisiones autorizadas por el docente asignado. Un cambio de contenido crea una versión nueva. Los intentos y publicaciones anteriores conservan sus vínculos.
- Para el reporte se agrupan pre y post del mismo intento y versión. Las omisiones permanecen como faltantes. El indicador clínico y el educativo se calculan de forma independiente.
