CREATE TABLE rechazos_mensajes (
    id INTEGER PRIMARY KEY,
    intento_id TEXT NOT NULL REFERENCES intentos(id),
    operacion_id TEXT NOT NULL,
    agente TEXT NOT NULL,
    motivo TEXT NOT NULL,
    momento TEXT NOT NULL
) STRICT;

CREATE INDEX rechazos_mensajes_operacion ON rechazos_mensajes(operacion_id);
