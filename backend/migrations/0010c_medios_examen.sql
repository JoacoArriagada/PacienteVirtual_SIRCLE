CREATE TABLE medios_referidos (
    version_caso_id INTEGER NOT NULL REFERENCES versiones_clinicas(id),
    fase TEXT NOT NULL,
    examen TEXT NOT NULL,
    medio TEXT NOT NULL,
    revision_fuente_id INTEGER NOT NULL REFERENCES revisiones_fuente(id),
    disponibilidad TEXT NOT NULL CHECK (disponibilidad IN ('indisponible','localizado')),
    PRIMARY KEY (version_caso_id, fase, examen, medio)
) STRICT;
