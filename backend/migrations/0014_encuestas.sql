CREATE TABLE instrumentos_educativos (
    version TEXT PRIMARY KEY NOT NULL,
    contenido TEXT NOT NULL CHECK (json_valid(contenido)),
    sha256 TEXT NOT NULL CHECK (length(sha256)=64 AND sha256 NOT GLOB '*[^0-9a-f]*'),
    constancia TEXT CHECK (constancia IS NULL OR json_valid(constancia))
) STRICT;

CREATE TABLE secuencias_educativas (
    intento_id TEXT PRIMARY KEY NOT NULL REFERENCES intentos(id),
    actividad_id TEXT NOT NULL REFERENCES actividades(id),
    version TEXT NOT NULL,
    ordinal INTEGER NOT NULL CHECK (ordinal > 0),
    UNIQUE (actividad_id, version, ordinal)
) STRICT;

INSERT INTO secuencias_educativas
SELECT id, actividad_id, version_instrumento,
       row_number() OVER (PARTITION BY actividad_id, version_instrumento ORDER BY rowid)
FROM intentos;

CREATE TABLE respuestas_educativas (
    intento_id TEXT NOT NULL REFERENCES secuencias_educativas(intento_id),
    etapa TEXT NOT NULL CHECK (etapa IN ('pre', 'post')),
    estado TEXT NOT NULL CHECK (estado IN ('respondido', 'omitido')),
    instante TEXT NOT NULL,
    respuestas TEXT NOT NULL CHECK (json_valid(respuestas) AND json_type(respuestas)='object'),
    PRIMARY KEY (intento_id, etapa)
) STRICT;
