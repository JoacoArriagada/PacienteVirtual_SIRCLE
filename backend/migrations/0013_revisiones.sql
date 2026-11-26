CREATE TABLE revisiones_evaluacion (
    evaluacion_id TEXT NOT NULL REFERENCES evaluaciones(id),
    revision INTEGER NOT NULL CHECK (revision>=1),
    anterior INTEGER,
    autor_id TEXT REFERENCES identidades(id),
    motivo TEXT NOT NULL CHECK (length(motivo)>0),
    momento TEXT NOT NULL,
    resultado TEXT NOT NULL CHECK (json_valid(resultado)),
    puntaje TEXT NOT NULL CHECK (json_valid(puntaje)),
    diferencias TEXT NOT NULL CHECK (json_valid(diferencias)),
    PRIMARY KEY (evaluacion_id,revision),
    FOREIGN KEY (evaluacion_id,anterior) REFERENCES revisiones_evaluacion(evaluacion_id,revision),
    CHECK ((revision=1 AND anterior IS NULL AND autor_id IS NULL)
        OR (revision>1 AND anterior IS NOT NULL AND anterior=revision-1 AND autor_id IS NOT NULL))
) STRICT;
