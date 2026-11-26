CREATE TABLE apuntes (
    id TEXT PRIMARY KEY NOT NULL,
    intento_id TEXT NOT NULL,
    autor_id TEXT NOT NULL,
    intencion TEXT NOT NULL CHECK (length(intencion) BETWEEN 1 AND 1500),
    momento TEXT NOT NULL,
    revision INTEGER NOT NULL,
    FOREIGN KEY (intento_id, autor_id) REFERENCES intentos(id, estudiante_id)
) STRICT;

CREATE UNIQUE INDEX intento_autor ON intentos(id, estudiante_id);
CREATE INDEX apuntes_intento ON apuntes(intento_id);

CREATE VIEW evidencias_registradas AS
SELECT t.intento_id, 'turno' AS tipo, t.id AS origen_id, 'declaracion' AS categoria,
       t.hablante AS autor, 'Entrevista' AS origen, t.respuesta AS texto, NULL AS instrumento,
       NULL AS valor, NULL AS unidad, NULL AS momento, t.revision
FROM turnos t WHERE t.tipo_respuesta='hecho'
UNION ALL
SELECT m.intento_id, 'medicion', m.id, 'medicion', i.estudiante_id,
       CASE WHEN m.procedencia_ensayo IS NULL THEN 'Valoración instrumental'
            ELSE 'Semana 5, material de ensayo' END,
       NULL, m.instrumento, m.valor, m.unidad, m.momento, m.revision
FROM mediciones m JOIN intentos i ON i.id=m.intento_id
UNION ALL
SELECT e.intento_id, 'hallazgo', h.id, 'observacion', i.estudiante_id,
       CASE WHEN h.procedencia_ensayo IS NULL THEN e.etiqueta
            ELSE 'Semana 5, material de ensayo · ' || e.etiqueta END,
       h.texto, e.instrumento, NULL, NULL, e.momento, e.revision
FROM exploraciones e JOIN hallazgos_exploracion h ON h.exploracion_id=e.id
JOIN intentos i ON i.id=e.intento_id
UNION ALL
SELECT o.intento_id, 'informe', f.id, 'informe', 'simulacion', 'Electrocardiograma inicial',
       f.resultado, NULL, NULL, NULL, f.consultado_en, o.revision
FROM ordenes o JOIN informes f ON f.orden_id=o.id WHERE f.consultado_en IS NOT NULL
UNION ALL
SELECT a.intento_id, 'apunte', a.id, 'interpretacion', a.autor_id, 'Apunte del estudiante',
       a.intencion, NULL, NULL, NULL, a.momento, a.revision
FROM apuntes a;
