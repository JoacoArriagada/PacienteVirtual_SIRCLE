CREATE TABLE configuraciones_ensayo (
    actividad_id TEXT PRIMARY KEY NOT NULL,
    escenario_id TEXT NOT NULL,
    version_caso_id INTEGER NOT NULL,
    version_rubrica TEXT NOT NULL,
    version_instrumento TEXT NOT NULL,
    FOREIGN KEY (actividad_id, escenario_id) REFERENCES actividades(id, escenario_id),
    FOREIGN KEY (escenario_id, version_caso_id) REFERENCES versiones_clinicas(escenario_id, id)
) STRICT;

CREATE TABLE intentos (
    id TEXT PRIMARY KEY NOT NULL,
    actividad_id TEXT NOT NULL,
    estudiante_id TEXT NOT NULL,
    escenario_id TEXT NOT NULL,
    version_caso_id INTEGER NOT NULL,
    version_rubrica TEXT NOT NULL,
    version_instrumento TEXT NOT NULL,
    pre TEXT NOT NULL CHECK (pre IN ('pendiente', 'omitido', 'respondido')),
    estado TEXT NOT NULL CHECK (estado IN ('pre_pendiente', 'atencion', 'entregado')),
    fase TEXT NOT NULL,
    revision INTEGER NOT NULL CHECK (revision >= 0),
    CHECK ((pre = 'pendiente') = (estado = 'pre_pendiente')),
    UNIQUE (actividad_id, estudiante_id),
    FOREIGN KEY (actividad_id, estudiante_id) REFERENCES asignaciones(actividad_id, estudiante_id),
    FOREIGN KEY (actividad_id, escenario_id) REFERENCES actividades(id, escenario_id),
    FOREIGN KEY (escenario_id, version_caso_id) REFERENCES versiones_clinicas(escenario_id, id)
) STRICT;
