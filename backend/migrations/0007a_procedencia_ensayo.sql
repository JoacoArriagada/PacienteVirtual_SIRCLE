ALTER TABLE lecturas_versionadas ADD COLUMN procedencia_ensayo TEXT
    CHECK (procedencia_ensayo IS NULL OR json_valid(procedencia_ensayo));

ALTER TABLE mediciones ADD COLUMN procedencia_ensayo TEXT
    CHECK (procedencia_ensayo IS NULL OR json_valid(procedencia_ensayo));
