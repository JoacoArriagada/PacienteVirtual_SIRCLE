ALTER TABLE turnos ADD COLUMN fase TEXT NOT NULL DEFAULT 'inicial';

CREATE INDEX turnos_contexto ON turnos(intento_id, agente, fase, hablante, orden);
