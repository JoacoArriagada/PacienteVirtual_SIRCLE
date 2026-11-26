CREATE TABLE publicaciones_evaluacion (
    evaluacion_id TEXT NOT NULL,
    numero INTEGER NOT NULL CHECK (numero>=1),
    revision INTEGER NOT NULL,
    docente_id TEXT NOT NULL REFERENCES identidades(id),
    publicada_en TEXT NOT NULL,
    PRIMARY KEY (evaluacion_id,numero),
    FOREIGN KEY (evaluacion_id,revision) REFERENCES revisiones_evaluacion(evaluacion_id,revision)
) STRICT;

CREATE TABLE post_ensayo (
    intento_id TEXT PRIMARY KEY NOT NULL REFERENCES intentos(id),
    estado TEXT NOT NULL CHECK (estado IN ('respondido','omitido')),
    momento TEXT NOT NULL
) STRICT;
