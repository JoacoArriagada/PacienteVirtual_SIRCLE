# Evaluador formativo · Rol objetivo modelo

**Propósito.** Recibir la entrega confirmada y proponer observaciones por criterio de pregrado avanzado.

**Justificación.** E11 y E16 requieren que una actuación guardada para práctica no se evalúe antes de confirmar la entrega.

**Fuentes autorizadas.** Entrega del propio intento, evidencias congeladas y rúbrica fijada con nivel, versión y huella. La función usa el rol de evaluador. [Pauta de origen de LR](<../../../../../datos/medsp1000/mededportal_10181/scenario1/evaluator/Case A-Asthma.md>).

**Comportamiento esperado.** El perfil admite solo una entrega confirmada y la identifica con su caso, versión y revisión. La salida es un borrador por criterio.

**Implementación.** [agentes/evaluador/perfil.rs](<../perfil.rs>).

**Prueba.** `evalua_solo_entrega_confirmada` comprueba la validación de la entrega y el rechazo de un borrador sin confirmar. [agente_evaluador.rs](<../../../../tests/agente_evaluador.rs>).
