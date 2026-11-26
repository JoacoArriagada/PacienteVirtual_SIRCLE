CREATE TABLE lecturas_versionadas (
    version_caso_id INTEGER NOT NULL REFERENCES versiones_clinicas(id),
    fase TEXT NOT NULL,
    instrumento TEXT NOT NULL,
    valor TEXT NOT NULL CHECK (json_valid(valor)),
    unidad TEXT NOT NULL,
    revision_fuente_id INTEGER NOT NULL REFERENCES revisiones_fuente(id),
    PRIMARY KEY (version_caso_id, fase, instrumento)
) STRICT;

CREATE TABLE mediciones (
    id TEXT PRIMARY KEY NOT NULL,
    intento_id TEXT NOT NULL REFERENCES intentos(id),
    instrumento TEXT NOT NULL,
    accion TEXT NOT NULL,
    fase TEXT NOT NULL,
    valor TEXT NOT NULL CHECK (json_valid(valor)),
    unidad TEXT NOT NULL,
    revision_fuente_id INTEGER NOT NULL REFERENCES revisiones_fuente(id),
    momento TEXT NOT NULL,
    en_ficha INTEGER NOT NULL DEFAULT 0 CHECK (en_ficha IN (0, 1)),
    revision INTEGER NOT NULL
) STRICT;

CREATE INDEX mediciones_intento_instrumento ON mediciones (intento_id, instrumento);
