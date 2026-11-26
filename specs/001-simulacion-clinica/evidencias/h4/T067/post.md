# T067 · Post y publicación

Chrome mostró siete preguntas después de confirmar la entrega. El recorrido A omitió el post y el B lo respondió. Antes de resolverlo, la API devolvió `post_pendiente`. La decisión se guardó con la forma, la versión y el instante.

| Recorrido | Decisión | Captura | Acuse y estado |
| --- | --- | --- | --- |
| A | Omitido | [Post A](post-a.png) | [Registro A](recorrido-a.json) |
| B | Respondido | [Post B](post-b.png) | [Registro B](recorrido-b.json) |

La prueba del evaluador publicó una revisión antes de resolver el post. Verificó que la revisión siguiera reservada, que apareciera al responder y que el puntaje coincidiera con la revisión docente. Un intento de cambiar la encuesta después de leer la devolución recibió 409. El reenvío con la misma clave recuperó el acuse. [Pruebas de publicación](../../../../../backend/tests/agente_evaluador.rs) · [Pruebas de encuestas](../../../../../backend/tests/encuestas.rs)
