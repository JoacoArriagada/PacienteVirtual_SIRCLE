# LR · P01 · Fuentes recuperacion memoria

**Propósito.** Recuperar la información de LR y los intercambios confirmados del intento.

**Justificación.** E04 requiere que el desconocimiento familiar por adopción conserve el pasaje que lo sustenta.

**Fuentes autorizadas.** La versión inicial de ensayo enlaza el guion sp_actor para LR. La memoria se lee con el propietario e intento autenticados. [Guion del actor de LR](<../../../../../datos/medsp1000/mededportal_10181/scenario1/sp_actor/Case A-Asthma.md>).

**Comportamiento esperado.** Una pregunta por antecedentes familiares puede seleccionar el hecho de adopción. Un candidato de otro intento, versión, voz o fase queda fuera del contexto.

**Implementación.** [shared/contexto.rs](<../../../shared/contexto.rs>).

**Prueba.** `cita_desconocimiento_familiar` comprueba la respuesta en primera persona y el pasaje en la versión del turno. [agente_lr.rs](<../../../../tests/agente_lr.rs>).
