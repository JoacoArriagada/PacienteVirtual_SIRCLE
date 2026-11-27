CREATE TABLE turnos (
    id TEXT PRIMARY KEY NOT NULL,
    intento_id TEXT NOT NULL REFERENCES intentos(id),
    orden INTEGER NOT NULL CHECK (orden > 0),
    agente TEXT NOT NULL,
    hablante TEXT NOT NULL,
    pregunta TEXT NOT NULL,
    respuesta TEXT NOT NULL,
    tipo_respuesta TEXT NOT NULL,
    operacion_id TEXT NOT NULL UNIQUE,
    revision INTEGER NOT NULL,
    traza TEXT NOT NULL CHECK (json_valid(traza)),
    UNIQUE (intento_id, orden)
) STRICT;
