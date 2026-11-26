ALTER TABLE trabajos_evaluacion ADD COLUMN toma_id TEXT;
ALTER TABLE trabajos_evaluacion ADD COLUMN vence INTEGER;
ALTER TABLE trabajos_evaluacion ADD COLUMN reintentar_desde INTEGER NOT NULL DEFAULT 0;
ALTER TABLE trabajos_evaluacion ADD COLUMN ultimo_error TEXT;

CREATE TABLE evaluaciones (
    id TEXT PRIMARY KEY NOT NULL,
    entrega_id TEXT NOT NULL UNIQUE REFERENCES entregas(id),
    operacion_id TEXT NOT NULL UNIQUE,
    borrador TEXT NOT NULL CHECK (json_valid(borrador)),
    momento TEXT NOT NULL,
    UNIQUE (id,operacion_id)
) STRICT;

CREATE TABLE trazas_evaluacion (
    evaluacion_id TEXT PRIMARY KEY NOT NULL,
    operacion_id TEXT NOT NULL UNIQUE,
    detalle TEXT NOT NULL CHECK (json_valid(detalle)),
    FOREIGN KEY (evaluacion_id,operacion_id) REFERENCES evaluaciones(id,operacion_id)
) STRICT;
