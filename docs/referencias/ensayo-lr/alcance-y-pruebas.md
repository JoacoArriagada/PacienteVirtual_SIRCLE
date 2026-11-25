# Alcance de la demostración y pruebas

**Registro histórico.** Las pruebas descritas corresponden a la demostración retirada de Semana 7. Se conservan sus observaciones y materiales como referencia.

## Relación con los 16 casos de uso

La especificación v2.0 define nueve objetivos principales y siete operaciones incluidas o extensiones. La numeración de los casos de uso y la numeración de las dieciséis pantallas pertenecen a catálogos distintos. [Requisitos completos](<../../requisitos/Documento de requerimientos - Paciente virtual en SIRCLE.md>) · [Catálogo de pantallas](<../interfaz/Funcionalidades_16_wireframes_SIRCLE.md>)

| CU | Decisión para LR | Cómo observarlo o qué queda para después |
|---|---|---|
| 01 · Acceder | Simulado | Entrada local como estudiante. La sesión institucional requiere el contrato de SIRCLE. |
| 02 · Administrar accesos | Posterior | Gestión de roles y grupos en la plataforma. |
| 03 · Preparar y asignar | Posterior | LR se entrega configurada. El profesor revisa primero su contenido. |
| 04 · Iniciar o retomar | En la demo | Un intento P01 en curso, introducción, consulta, pausa y reanudación. |
| 05 · Valorar | En la demo | Texto libre, ejemplos, retrato, brazo, ampliación, exploración y medición. |
| 06 · Registrar decisiones | Parcial | Apuntes, evidencia, hipótesis, indicaciones y solicitud de apoyo. El efecto y la confirmación dependen del guion. |
| 07 · Finalizar y consultar | Heredado | Pantallas locales de revisión y resumen. La entrega institucional queda para otra iteración. |
| 08 · Supervisar grupo | Posterior | Requiere usuarios y relación docente–grupo. |
| 09 · Encuestas | Heredado | Encuesta local opcional. Su uso académico requiere acordar el instrumento. |
| 10 · Verificar autorización | Simulado | Perfil de estudiante de demostración. La autorización real depende de SIRCLE. |
| 11 · Conservar avance | En la demo | SQLite guarda el estado y su revisión. Recargar o reiniciar recupera el mismo intento. |
| 12 · Repetir | Heredado | La pantalla de reanudación permite otro intento según los límites del prototipo. |
| 13 · Solicitar y consultar examen | En la demo | Orden pendiente, plazo simulado, informe y registro en ficha. |
| 14 · Avanzar tiempo | En la demo | Avances confirmados de 30 minutos con cambio de escena y disponibilidad de informes. |
| 15 · Evaluar | Heredado | Resumen formativo local basado en reglas. La validación de rúbrica y publicación sigue pendiente. |
| 16 · Rectificar evaluación | Posterior | Necesita identidad docente y trazabilidad académica. |

## Qué probar en una sesión con el profesor

| Acción | Resultado observable |
|---|---|
| Entrar, filtrar En curso y continuar | Aparece una sola tarjeta, LR, P01, 34 años. Antes de explorar se pueden abrir la información inicial y los objetivos. |
| Escribir una pregunta o editar una sugerencia | Se registra lo que escribió el estudiante y LR responde desde el guion. Una frase con hallazgo conserva referencia al mensaje de origen. |
| Preguntar por una dosis ausente o por un grupo sanguíneo | El sistema expresa incertidumbre. El contador de temas y la evidencia se mantienen. |
| Ver brazo, ampliar y cerrar | La consulta vuelve a la misma atención. |
| Elegir una maniobra | Presión arterial, signos vitales, temperatura, respiración, saturación, peso, inspección, palpación y auscultación disponen de preparación, acción, resultado e histórico. En la exploración por regiones se elige una zona. |
| Registrar una medición | Valor o hallazgo, unidad cuando corresponde y momento simulado quedan en la ficha. El triaje y una nueva toma tienen orígenes distintos. |
| Pedir un examen adicional | Radiografía de tórax, hemograma, ECG y gasometría quedan disponibles a los 30 minutos simulados. El panel viral, a los 60. |
| Pedir ayuda | La acción admite prioridad y fundamento, requiere confirmación y deja constancia en el intento. |
| Guardar apunte e hipótesis | Se puede volver desde un hallazgo a su origen. La hipótesis conserva texto, fundamento y referencias. |
| Recargar o reiniciar la API | El intento recupera conversación, tomas, órdenes, apunte e hipótesis. Un intento nuevo tiene otro identificador. |

## Verificación del 25 de noviembre de 2026

Se ejecutó el recorrido en Chromium de escritorio, a 1440 × 1024. Se completaron las nueve maniobras desde sus controles, se registraron y se abrieron sus históricos. Se abrieron el brazo y la ampliación, la ficha clínica y las indicaciones. Se solicitaron los cinco adicionales y se comprobó su disponibilidad tras dos avances de 30 minutos. Se confirmó la solicitud de ayuda y se conservó una hipótesis vinculada a evidencia. En la base de prueba, la revisión 33 se recuperó con el mismo número después de detener el proceso y volver a iniciar la API.

**Fidelidad visual.** Se abrió el HTML original de Semana 5 junto al piloto con igual tamaño de ventana. Se comparó la captura del área de imagen del instrumento, en fase de preparación, para cada una de las nueve maniobras. Las nueve comparaciones fueron idénticas. La comprobación cubre esa región y ese estado. [Captura de la entrevista en el piloto](<../interfaz/piloto-atencion-lr.png>) · [Referencia exportada de Figma](<../interfaz/figma-atender-lr.png>)

**Despliegue.** El ejecutable Windows x64 se compiló en modo release y se probó con Ollama nativo. La CLI oficial aceptó `docker compose config` y resolvió los cuatro servicios. En esta estación faltaba un motor Docker disponible. Quedan por ejecutar el arranque de los contenedores, la primera descarga y la recuperación de los volúmenes después de recrearlos.

**Recuperación de errores.** Ante un HTTP 503 inducido en el navegador, el chat conservó la pregunta para reintentar y la conversación mantuvo el mismo número de mensajes. Una escritura enviada a la API con una revisión obsoleta recibió HTTP 409. Se comprobó además que el origen de un hallazgo mostraba la paráfrasis que escribió el estudiante.

## Criterios de interpretación

El ensayo de entrevista pasó **28 de 28 comprobaciones** en la versión entregada. Los quince ejemplos literales se recuperaron del guion. Entre las trece preguntas libres, once pasaron por Qwen y dos comprobaron reglas de dosis o aclaración. Las once consultas al modelo cargado tardaron entre **360 y 414 ms**, con mediana de **377 ms**, medidas de extremo a extremo desde un cliente HTTP local. No incluyen un primer arranque en frío. Ollama informó 4.096 tokens de contexto y 3.140.495.604 bytes en VRAM. Se usó `qwen3.5:4b` Q4_K_M, con digest `d8b0f5e9760cd1682034f292d7ef72ec46f432149be0df7574bf2d6e92e38c04`. [Preguntas, respuestas, origen y tiempos individuales](<ensayo-entrevista.json>)

**Qué cambió durante las pruebas.** En el primer ensayo de 25 preguntas, «¿Qué cosas hacen que le cueste más respirar?» activó preocupación en vez de factores agravantes. Añadir el nombre del hallazgo y distinguir esfuerzo, evolución y emoción corrigió esa paráfrasis. Al repetir el ensayo, el modelo desvió una sugerencia literal sobre hacer pausas. Como cada sugerencia ya tiene un identificador estable, el servidor ahora recupera directamente el par aprobado cuando coincide el texto. Esa decisión evita gastar inferencia en una correspondencia conocida. Parte de las preguntas se reutilizó para ajustar el piloto. La exactitud con preguntas nuevas del profesor sigue abierta.

El ensayo consultó la API sin escribir en el intento del estudiante. Se conservan las preguntas y respuestas en el [registro del ensayo](<ensayo-entrevista.json>).

Las cifras clínicas y los plazos reproducen un escenario educativo de Semana 5. El profesor debe revisar su coherencia antes de usarlo con estudiantes. La especificación propone para RR01 veinte usuarios simultáneos durante quince minutos y, entre otros objetivos, un p95 de diálogo de 15 s. Este recorrido usa una sola estación. Repetir las preguntas del guion sirve como prueba de regresión. Para medir generalización hay que reservar preguntas nuevas y revisar su significado sin enseñar la respuesta esperada al sistema.
