CREATE TABLE rubricas (
    id INTEGER PRIMARY KEY,
    codigo TEXT NOT NULL,
    version TEXT NOT NULL,
    actividad_id TEXT NOT NULL REFERENCES actividades(id),
    caso TEXT NOT NULL REFERENCES escenarios(id),
    nivel TEXT NOT NULL,
    sha256 TEXT NOT NULL CHECK (length(sha256)=64 AND sha256 NOT GLOB '*[^0-9a-f]*'),
    contenido BLOB NOT NULL,
    estado TEXT NOT NULL CHECK (estado IN ('borrador','aprobada')),
    constancia TEXT CHECK (constancia IS NULL OR json_valid(constancia)),
    CHECK ((estado='aprobada')=(constancia IS NOT NULL)),
    UNIQUE (codigo, version)
) STRICT;
