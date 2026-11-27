ALTER TABLE hechos ADD COLUMN tema TEXT NOT NULL DEFAULT '';
ALTER TABLE hechos ADD COLUMN variantes TEXT NOT NULL DEFAULT '[]' CHECK (json_valid(variantes));
