CREATE TABLE operaciones (
    identidad_id TEXT NOT NULL REFERENCES identidades(id),
    ambito TEXT NOT NULL,
    clave TEXT NOT NULL CHECK (length(clave) BETWEEN 1 AND 128),
    huella TEXT NOT NULL CHECK (length(huella) = 64 AND huella NOT GLOB '*[^0-9a-f]*'),
    respuesta TEXT NOT NULL CHECK (json_valid(respuesta)),
    PRIMARY KEY (identidad_id, ambito, clave)
) STRICT;
