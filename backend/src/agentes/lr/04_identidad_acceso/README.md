# LR · P01 · Identidad acceso

**Propósito.** Asociar cada turno a la identidad y al intento que lo originaron.

**Justificación.** E17 requiere preservar dos historiales distintos incluso cuando sus preguntas se reciben juntas.

**Fuentes autorizadas.** Sesión estudiantil, asignación habilitada, fase inicial y versión fijada. La revisión docente se autoriza mediante la asignación correspondiente. [Guion del actor de LR](<../../../../../datos/medsp1000/mededportal_10181/scenario1/sp_actor/Case A-Asthma.md>).

**Comportamiento esperado.** Dos identidades autorizadas reciben turnos de su intento. Consultar el recurso ajeno o reutilizar su clave produce rechazo antes de preparar contexto.

**Implementación.** [shared/autorizacion.rs](<../../../shared/autorizacion.rs>).

**Prueba.** `aisla_dos_preguntas_simultaneas` comprueba dos preguntas libres en curso al mismo tiempo y cero accesos cruzados. [agente_lr.rs](<../../../../tests/agente_lr.rs>).

La prueba `aisla_dos_preguntas_simultaneas` verifica el solapamiento en Axum y el aislamiento con Qwen real. T052 conserva las marcas y los turnos.
