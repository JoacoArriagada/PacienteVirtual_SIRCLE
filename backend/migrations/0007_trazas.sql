CREATE UNIQUE INDEX turnos_id_operacion ON turnos (id, operacion_id);

CREATE TABLE trazas_turno (
    turno_id TEXT PRIMARY KEY NOT NULL,
    operacion_id TEXT NOT NULL UNIQUE,
    detalle TEXT NOT NULL CHECK (json_valid(detalle)),
    FOREIGN KEY (turno_id, operacion_id) REFERENCES turnos(id, operacion_id)
) STRICT;

INSERT INTO trazas_turno (turno_id, operacion_id, detalle)
    SELECT id, operacion_id, traza FROM turnos;

ALTER TABLE turnos DROP COLUMN traza;
