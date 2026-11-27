CREATE TABLE reservas_operaciones (
    identidad_id TEXT NOT NULL REFERENCES identidades(id),
    ambito TEXT NOT NULL REFERENCES intentos(id),
    clave TEXT NOT NULL CHECK (length(clave) BETWEEN 1 AND 128),
    huella TEXT NOT NULL CHECK (length(huella) = 64 AND huella NOT GLOB '*[^0-9a-f]*'),
    toma_id TEXT NOT NULL UNIQUE,
    vence INTEGER NOT NULL CHECK (vence >= 0),
    PRIMARY KEY (identidad_id, ambito, clave)
) STRICT;
