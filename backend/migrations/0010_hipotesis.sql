CREATE TABLE borradores_hipotesis (
    intento_id TEXT NOT NULL REFERENCES intentos(id),
    revision INTEGER NOT NULL,
    PRIMARY KEY (intento_id, revision)
) STRICT;

CREATE TABLE hipotesis (
    intento_id TEXT NOT NULL,
    revision INTEGER NOT NULL,
    posicion INTEGER NOT NULL,
    rol TEXT NOT NULL CHECK (rol IN ('principal', 'alternativa')),
    titulo TEXT NOT NULL CHECK (length(titulo) <= 180),
    explicacion TEXT NOT NULL CHECK (length(explicacion) <= 1400),
    PRIMARY KEY (intento_id, revision, posicion),
    FOREIGN KEY (intento_id, revision) REFERENCES borradores_hipotesis(intento_id, revision)
) STRICT;

CREATE UNIQUE INDEX hipotesis_principal ON hipotesis(intento_id, revision) WHERE rol='principal';

CREATE TABLE vinculos_hipotesis (
    intento_id TEXT NOT NULL,
    revision INTEGER NOT NULL,
    posicion INTEGER NOT NULL,
    orden INTEGER NOT NULL,
    tipo TEXT NOT NULL,
    origen_id TEXT NOT NULL,
    PRIMARY KEY (intento_id, revision, posicion, orden),
    UNIQUE (intento_id, revision, posicion, tipo, origen_id),
    FOREIGN KEY (intento_id, revision, posicion) REFERENCES hipotesis(intento_id, revision, posicion)
) STRICT;

CREATE TRIGGER vinculo_evidencia_del_intento BEFORE INSERT ON vinculos_hipotesis
WHEN NOT EXISTS (
    SELECT 1 FROM evidencias_registradas
    WHERE intento_id=NEW.intento_id AND tipo=NEW.tipo AND origen_id=NEW.origen_id
)
BEGIN
    SELECT RAISE(ABORT, 'La evidencia no pertenece al intento');
END;
