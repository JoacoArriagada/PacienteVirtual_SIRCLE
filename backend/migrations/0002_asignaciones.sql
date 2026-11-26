CREATE TABLE identidades (
    id TEXT PRIMARY KEY NOT NULL,
    rol TEXT NOT NULL CHECK (rol IN ('estudiante', 'docente'))
) STRICT;

CREATE TABLE actividades (
    id TEXT PRIMARY KEY NOT NULL,
    escenario_id TEXT NOT NULL REFERENCES escenarios(id),
    nombre TEXT NOT NULL,
    tipo TEXT NOT NULL CHECK (tipo IN ('ensayo', 'formal')),
    habilitada INTEGER NOT NULL CHECK (habilitada IN (0, 1)),
    UNIQUE (id, escenario_id)
) STRICT;

CREATE TABLE asignaciones (
    actividad_id TEXT NOT NULL REFERENCES actividades(id),
    estudiante_id TEXT NOT NULL REFERENCES identidades(id),
    docente_id TEXT NOT NULL REFERENCES identidades(id),
    PRIMARY KEY (actividad_id, estudiante_id),
    CHECK (estudiante_id <> docente_id)
) STRICT;

CREATE TABLE sesiones (
    huella TEXT PRIMARY KEY NOT NULL,
    identidad_id TEXT NOT NULL REFERENCES identidades(id),
    vence INTEGER NOT NULL
) STRICT;

CREATE INDEX sesiones_vence ON sesiones (vence);
CREATE INDEX asignaciones_estudiante ON asignaciones (estudiante_id);
