CREATE TABLE rubricas_activas_ensayo (
    actividad_id TEXT PRIMARY KEY NOT NULL REFERENCES configuraciones_ensayo(actividad_id),
    rubrica_id INTEGER NOT NULL REFERENCES rubricas(id)
) STRICT;

ALTER TABLE intentos ADD COLUMN rubrica_id INTEGER REFERENCES rubricas(id);
ALTER TABLE intentos ADD COLUMN rubrica_sha256 TEXT CHECK (
    (rubrica_id IS NULL AND rubrica_sha256 IS NULL) OR
    (rubrica_id IS NOT NULL AND rubrica_sha256 IS NOT NULL AND
     length(rubrica_sha256)=64 AND rubrica_sha256 NOT GLOB '*[^0-9a-f]*')
);
