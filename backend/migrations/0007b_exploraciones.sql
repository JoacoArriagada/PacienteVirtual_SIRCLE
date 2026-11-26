CREATE TABLE zonas_exploracion (
    version_caso_id INTEGER NOT NULL REFERENCES versiones_clinicas(id),
    fase TEXT NOT NULL,
    instrumento TEXT NOT NULL,
    zona TEXT NOT NULL,
    etiqueta TEXT NOT NULL,
    resultado TEXT,
    revision_fuente_id INTEGER REFERENCES revisiones_fuente(id),
    procedencia_ensayo TEXT CHECK (procedencia_ensayo IS NULL OR json_valid(procedencia_ensayo)),
    CHECK ((resultado IS NULL) = (revision_fuente_id IS NULL)),
    PRIMARY KEY (version_caso_id, fase, instrumento, zona)
) STRICT;

CREATE TABLE exploraciones (
    id TEXT PRIMARY KEY NOT NULL,
    intento_id TEXT NOT NULL REFERENCES intentos(id),
    instrumento TEXT NOT NULL,
    zona TEXT NOT NULL,
    etiqueta TEXT NOT NULL,
    fase TEXT NOT NULL,
    momento TEXT NOT NULL,
    revision INTEGER NOT NULL
) STRICT;

CREATE TABLE hallazgos_exploracion (
    id TEXT PRIMARY KEY NOT NULL,
    exploracion_id TEXT NOT NULL UNIQUE REFERENCES exploraciones(id),
    texto TEXT NOT NULL,
    revision_fuente_id INTEGER NOT NULL REFERENCES revisiones_fuente(id),
    procedencia_ensayo TEXT CHECK (procedencia_ensayo IS NULL OR json_valid(procedencia_ensayo))
) STRICT;

CREATE INDEX exploraciones_intento ON exploraciones(intento_id);
