CREATE TABLE examenes_versionados (
    version_caso_id INTEGER NOT NULL REFERENCES versiones_clinicas(id),
    fase TEXT NOT NULL,
    examen TEXT NOT NULL,
    resultado TEXT NOT NULL,
    revision_fuente_id INTEGER NOT NULL REFERENCES revisiones_fuente(id),
    PRIMARY KEY (version_caso_id, fase, examen)
) STRICT;

CREATE TABLE ordenes (
    id TEXT PRIMARY KEY NOT NULL,
    intento_id TEXT NOT NULL REFERENCES intentos(id),
    examen TEXT NOT NULL,
    fase TEXT NOT NULL,
    momento TEXT NOT NULL,
    revision INTEGER NOT NULL,
    UNIQUE (intento_id, examen)
) STRICT;

CREATE TABLE informes (
    id TEXT PRIMARY KEY NOT NULL,
    orden_id TEXT NOT NULL UNIQUE REFERENCES ordenes(id),
    resultado TEXT NOT NULL,
    revision_fuente_id INTEGER NOT NULL REFERENCES revisiones_fuente(id),
    consultado_en TEXT
) STRICT;
