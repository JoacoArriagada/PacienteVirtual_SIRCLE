CREATE TABLE escenarios (
    id TEXT PRIMARY KEY NOT NULL,
    paciente TEXT NOT NULL CHECK (length(paciente) > 0)
) STRICT;

CREATE TABLE fuentes (
    id INTEGER PRIMARY KEY,
    escenario_id TEXT NOT NULL REFERENCES escenarios(id),
    rol TEXT NOT NULL CHECK (rol IN ('sp_actor', 'examinee', 'evaluator', 'environment_controller')),
    ruta TEXT NOT NULL CHECK (length(ruta) > 0),
    UNIQUE (escenario_id, rol, ruta),
    UNIQUE (escenario_id, id)
) STRICT;

CREATE TABLE revisiones_fuente (
    id INTEGER PRIMARY KEY,
    fuente_id INTEGER NOT NULL REFERENCES fuentes(id),
    sha256 TEXT NOT NULL CHECK (length(sha256) = 64 AND sha256 NOT GLOB '*[^0-9a-f]*'),
    contenido BLOB NOT NULL,
    UNIQUE (fuente_id, sha256),
    UNIQUE (fuente_id, id)
) STRICT;

CREATE TABLE versiones_clinicas (
    id INTEGER PRIMARY KEY,
    escenario_id TEXT NOT NULL REFERENCES escenarios(id),
    version TEXT NOT NULL CHECK (length(version) > 0),
    estado TEXT NOT NULL CHECK (estado IN ('pendiente', 'ensayo', 'aprobada')),
    autorizada_por TEXT,
    autorizada_en TEXT,
    motivo_autorizacion TEXT,
    CHECK (
        (estado = 'aprobada' AND autorizada_por IS NOT NULL AND autorizada_en IS NOT NULL AND motivo_autorizacion IS NOT NULL)
        OR (estado <> 'aprobada' AND autorizada_por IS NULL AND autorizada_en IS NULL AND motivo_autorizacion IS NULL)
    ),
    UNIQUE (escenario_id, version),
    UNIQUE (escenario_id, id)
) STRICT;

CREATE TABLE versiones_fuentes (
    escenario_id TEXT NOT NULL,
    version_clinica_id INTEGER NOT NULL,
    fuente_id INTEGER NOT NULL,
    revision_fuente_id INTEGER NOT NULL,
    PRIMARY KEY (version_clinica_id, fuente_id),
    FOREIGN KEY (escenario_id, version_clinica_id) REFERENCES versiones_clinicas(escenario_id, id),
    FOREIGN KEY (escenario_id, fuente_id) REFERENCES fuentes(escenario_id, id),
    FOREIGN KEY (fuente_id, revision_fuente_id) REFERENCES revisiones_fuente(fuente_id, id)
) STRICT;
