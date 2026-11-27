DROP VIEW evidencias_registradas;
CREATE VIEW evidencias_registradas AS
SELECT t.intento_id, 'turno' AS tipo, t.id AS origen_id, 'declaracion' AS categoria,
       t.hablante AS autor, 'Entrevista' AS origen,
       coalesce(json_extract(tr.detalle, '$.declaracion'), t.respuesta) AS texto, NULL AS instrumento,
       NULL AS valor, NULL AS unidad, NULL AS momento, t.revision
FROM turnos t LEFT JOIN trazas_turno tr ON tr.turno_id=t.id WHERE t.tipo_respuesta='hecho'
UNION ALL
SELECT m.intento_id, 'medicion', m.id, 'medicion', i.estudiante_id,
       CASE WHEN m.procedencia_ensayo IS NULL THEN 'Valoración instrumental'
            ELSE 'Material de ensayo del prototipo' END,
       NULL, m.instrumento, m.valor, m.unidad, m.momento, m.revision
FROM mediciones m JOIN intentos i ON i.id=m.intento_id
UNION ALL
SELECT e.intento_id, 'hallazgo', h.id, 'observacion', i.estudiante_id,
       CASE WHEN h.procedencia_ensayo IS NULL THEN e.etiqueta
            ELSE 'Material de ensayo del prototipo · ' || e.etiqueta END,
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

UPDATE lecturas_versionadas
SET procedencia_ensayo = json_set(procedencia_ensayo, '$.ruta',
    'docs/referencias/prototipo-html/Paciente_Virtual.html')
WHERE instrumento='peso'
  AND json_extract(procedencia_ensayo, '$.sha256')='ff9dea8958759550c5241ab9f0560c46f2726bf04ba1d46f927fb9febfeee032'
  AND json_extract(procedencia_ensayo, '$.ruta')<>'docs/referencias/prototipo-html/Paciente_Virtual.html';

UPDATE zonas_exploracion
SET procedencia_ensayo = json_set(procedencia_ensayo, '$.ruta',
    'docs/referencias/prototipo-html/Paciente_Virtual.html')
WHERE instrumento='palpacion' AND zona='central'
  AND json_extract(procedencia_ensayo, '$.sha256')='ff9dea8958759550c5241ab9f0560c46f2726bf04ba1d46f927fb9febfeee032'
  AND json_extract(procedencia_ensayo, '$.ruta')<>'docs/referencias/prototipo-html/Paciente_Virtual.html';
