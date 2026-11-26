ALTER TABLE rechazos_mensajes ADD COLUMN detalle TEXT CHECK (detalle IS NULL OR json_valid(detalle));
CREATE INDEX rechazos_mensajes_intento ON rechazos_mensajes(intento_id, id);
