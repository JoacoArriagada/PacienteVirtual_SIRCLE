CREATE UNIQUE INDEX versiones_fuentes_revision
    ON versiones_fuentes (version_clinica_id, fuente_id, revision_fuente_id);

CREATE TABLE hechos (
    id TEXT NOT NULL,
    escenario_id TEXT NOT NULL,
    version_clinica_id INTEGER NOT NULL,
    fase TEXT NOT NULL,
    hablante TEXT NOT NULL,
    condicion TEXT NOT NULL,
    respuesta TEXT NOT NULL,
    fuente_id INTEGER NOT NULL,
    revision_fuente_id INTEGER NOT NULL,
    linea INTEGER NOT NULL CHECK (linea > 0),
    pasaje TEXT NOT NULL,
    PRIMARY KEY (version_clinica_id, fase, hablante, id),
    FOREIGN KEY (escenario_id, version_clinica_id) REFERENCES versiones_clinicas(escenario_id, id),
    FOREIGN KEY (version_clinica_id, fuente_id, revision_fuente_id)
        REFERENCES versiones_fuentes(version_clinica_id, fuente_id, revision_fuente_id)
) STRICT;
