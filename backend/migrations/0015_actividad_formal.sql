CREATE TABLE aprobaciones_clinicas (
    version_id INTEGER PRIMARY KEY REFERENCES versiones_clinicas(id),
    contenido TEXT NOT NULL CHECK (json_valid(contenido)),
    sha256 TEXT NOT NULL CHECK (length(sha256)=64 AND sha256 NOT GLOB '*[^0-9a-f]*'),
    constancia TEXT NOT NULL CHECK (json_valid(constancia))
) STRICT;

CREATE TABLE configuraciones_formales (
    actividad_id TEXT PRIMARY KEY NOT NULL REFERENCES actividades(id),
    version_caso_id INTEGER NOT NULL REFERENCES aprobaciones_clinicas(version_id),
    version_instrumento TEXT NOT NULL REFERENCES instrumentos_educativos(version),
    rubrica_id INTEGER NOT NULL REFERENCES rubricas(id)
) STRICT;
