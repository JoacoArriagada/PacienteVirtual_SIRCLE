# T059 · Calibración E12 con tres hipótesis justificadas de cuatro

26 de noviembre de 2026 · `feature/prototipo` · revisión base `fbc3f1ca`

## Procedimiento

Reutilizar el contenedor CPU de T013 y confirmar que su inventario
contiene `qwen3.5:4b` con la huella registrada. Desde la raíz:

```bash
docker inspect sircle-ollama-h1 --format '{{.State.Running}} {{json .HostConfig.DeviceRequests}}'
curl --silent --fail http://127.0.0.1:11434/api/tags
cargo test --manifest-path backend/Cargo.toml --locked --offline \
  --test agente_evaluador calibra_tres_de_cuatro_con_qwen -- --ignored --nocapture
```

La prueba activa la rúbrica de calibración `lr-calibracion-e12` V1
con constancia de ensayo. Por la API registra el turno de inicio, la
toma de presión y el informe del ECG. Guarda las cuatro hipótesis del
fixture, con los IDs efectivos de las tres primeras, y confirma la
entrega. El trabajador separado del evaluador invoca Ollama y guarda
el borrador. Un docente asignado recupera el borrador y su traza por
sus rutas de revisión. La prueba coteja los juicios y sus citas con
la respuesta real del motor y comprueba en `/api/ps` el digest y el
uso de VRAM. La clave del fixture se usa para comparar después de la
respuesta. La solicitud de inferencia lleva las expectativas de la
rúbrica, los registros y las hipótesis de la entrega.

## Primer intento

El cliente dejó de esperar después de 300 segundos. Ollama había
procesado la entrada y seguía generando a cerca de 3,5 tokens por
segundo. El motor registró 500 al cancelarse la petición y mostró cero
tokens de entrada truncados. El servicio devolvió el trabajo a pendiente.
Para repetir se acortaron los nombres de campos de la respuesta y
se pidió una frase breve por campo. El plazo del evaluador quedó en
520 segundos y su toma en 550 segundos. Estos valores corresponden a
la ejecución funcional local en CPU. Los estudios de capacidad siguen
programados después de T080.

[Registro de Ollama del primer intento](primer-tiempo-agotado.txt).

## Segundo intento

El modelo devolvió la respuesta dentro del plazo. El resultado repitió
un criterio en la lista y el validador impidió guardar el borrador.
El esquema de salida ahora exige un objeto con una clave obligatoria
por criterio. El servicio conserva las observaciones ordenadas por la
rúbrica y la traza conserva también el JSON crudo del motor.

[Rechazo de la segunda corrida](segundo-rechazado.txt).

## Tercer intento

La respuesta cubrió los criterios, pero incluyó una cita de un tipo
inadecuado para uno de los juicios. El validador rechazó esa evaluación
en 325 segundos. Para repetir, cada criterio recibió en su esquema solo
los IDs de evidencia de tipos que admite su rúbrica. Si el intento carece
de esos tipos, el esquema limita el juicio a omisión o revisión docente.

[Rechazo de la tercera corrida](tercero-rechazado.txt).

## Cuarto intento

La respuesta respetó los tipos de las citas que incluyó, pero propuso
un juicio que exigía respaldo para la cuarta hipótesis, cuyo vínculo
faltaba. Se rechazó antes de guardar. La rúbrica de calibración fija
ahora la posición de cada hipótesis y los tipos que admite cada
criterio. Cuando falta el fundamento, el esquema solicita una omisión
y una cita admisible para proponer un contraste. Antes de guardar un
acierto o error, el servicio comprueba el fundamento y una cita propia
de esa hipótesis. Las rúbricas anteriores mantienen su formato y huella
porque omiten el campo opcional.

[Rechazo de la cuarta corrida](cuarto-rechazado.txt).

## Quinto intento

La prueba obtuvo los tres aciertos y la omisión con las citas correctas.
En la cuarta hipótesis, el modelo identificó que el turno no mencionaba
un esfuerzo previo. La mejora señalaba el registro ausente de forma
descriptiva. El borrador incorpora ahora una instrucción concreta para
revisar la cita y anotar si documenta el dato que falta. El juicio, la
cita y el motivo originales quedan vinculados a la respuesta del motor.

[Cotejo de la quinta corrida](quinto-cotejo.json).

## Resultado final

El evaluador reconoció por separado el empeoramiento descrito en el
turno, el origen sinusal del ritmo según el ECG y la elevación
sistólica en la toma. Cada acierto citó su propio registro. La cuarta
hipótesis quedó como omisión y citó el turno de inicio. La devolución
indicó que faltaba vincular el esfuerzo previo con la disnea y pidió
revisar el turno y anotar si documentaba ese dato. Los cuatro juicios
y sus citas coincidieron con la clave, la salida cruda de Ollama y la
lectura del docente. Pasaron la calibración y las trece pruebas de
regresión del evaluador y las rúbricas.

La traza enlaza la entrega, V1 y su SHA-256, la operación de inferencia
y el ID del borrador recuperado. Ollama generó 927 tokens. La solicitud
de la corrida final tardó 368,688 segundos y `/api/ps` informó cero
bytes de VRAM.

[Cuatro juicios, acuse y cotejo](tres-de-cuatro.json) ·
[Traza real y estado del motor CPU](traza-inferencia.json).
