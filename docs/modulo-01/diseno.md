# Módulo 01 · Entrevista, valoración y devolución

El módulo permite observar si un estudiante puede entrevistar a LR, obtener hallazgos con los instrumentos de SIRCLE y justificar una hipótesis que el profesor pueda revisar. La medición educativa empieza antes de mostrar la información clínica y se repite después de entregar, antes de ver la devolución. [Instrumento de evaluación educativa](instrumento-pre-post.md)

## Recorrido de la actividad

1. El estudiante elige LR y responde u omite el instrumento inicial. Puede retomar su intento desde «En curso».
2. Durante la atención conversa con LR, usa los instrumentos, registra hallazgos y solicita los exámenes disponibles para el caso. La ficha distingue el relato, las mediciones, los informes y los apuntes del estudiante.
3. Al entregar, el estudiante fundamenta una hipótesis principal y la contrasta con una alternativa. El sistema fija la versión de la rúbrica y las evidencias del intento para la revisión.
4. El docente inspecciona la evidencia de cada criterio, puede rectificar con motivo y publica la devolución. El estudiante la recibe cuando se resuelve el instrumento posterior y la revisión está publicada.

## Casos y fuentes

| Caso | Uso actual |
|---|---|
| **P01 · LR** | Recorrido integrado de entrevista, valoración y devolución docente. |
| **P02 · Nannu** | Caso disponible en el backend para ensayar su agente y distinguir el relato de Nannu del de su acompañante. |
| **P03 · Dan Sparks** | Caso disponible en el backend para ensayar su agente y gestionar los hallazgos del escenario. |

Los tres casos proceden de MedSP1000. SQLite importa los documentos de actor, estudiante, entorno y evaluador, con seis fuentes en total para Dan por los dos documentos adicionales del ECG. El ECG de Dan referencia una imagen ausente en el catálogo, por lo que el backend registra su disponibilidad a partir de la fuente. [Catálogo y fuentes de los casos](../../dataset/README.md)

El módulo conserva la separación entre material de ensayo y actividad formal. El HTML de referencia conserva la procedencia de recursos y hallazgos del prototipo, incluidos el peso y la palpación. Los materiales formales requieren constancia docente y se fijan por versión al abrir cada intento. [Prototipo de referencia](../referencias/prototipo-html/Paciente_Virtual.html) · [Activación formal](../arranque.md#actividad-formal-de-lr)

## Componentes

| Componente | Responsabilidad |
|---|---|
| **Frontend TypeScript y Nginx** | Presenta el recorrido de LR, sirve los recursos visuales y reenvía la API. |
| **Backend Rust y SQLite** | Autoriza las operaciones, prepara fuentes y versiones, guarda los registros y separa los procesos de los agentes. |
| **Ollama** | Ejecuta localmente `qwen3.5:4b` para la entrevista y la evaluación. El backend valida las respuestas contra el contexto del intento. |
| **Docker Compose** | Prepara SQLite antes de iniciar el backend y conecta la aplicación con el modelo local. |

[Arranque y operación](../arranque.md) · [Recursos audiovisuales y licencias](../recursos-audiovisuales/README.md)
