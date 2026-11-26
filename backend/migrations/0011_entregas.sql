CREATE TABLE entregas (
    id TEXT PRIMARY KEY NOT NULL,
    intento_id TEXT NOT NULL UNIQUE REFERENCES intentos(id),
    revision INTEGER NOT NULL CHECK (revision >= 0),
    caso TEXT NOT NULL REFERENCES escenarios(id),
    version_caso TEXT NOT NULL,
    rubrica_id INTEGER NOT NULL REFERENCES rubricas(id),
    version_rubrica TEXT NOT NULL,
    rubrica_sha256 TEXT NOT NULL CHECK (length(rubrica_sha256)=64 AND rubrica_sha256 NOT GLOB '*[^0-9a-f]*'),
    confirmada_en TEXT NOT NULL,
    instantanea TEXT NOT NULL CHECK (json_valid(instantanea))
) STRICT;

CREATE TABLE trabajos_evaluacion (
    entrega_id TEXT PRIMARY KEY NOT NULL REFERENCES entregas(id),
    estado TEXT NOT NULL CHECK (estado IN ('pendiente','en_curso','terminado')),
    creado_en TEXT NOT NULL
) STRICT;
