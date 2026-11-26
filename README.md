# Paciente Virtual SIRCLE

El módulo 01 implementa la práctica de LR con entrevista local, valoración, razonamiento, evaluación docente y encuestas opcionales. El repositorio conserva las fuentes clínicas y la referencia visual de Semana 5.

| Para revisar | Abrir |
|---|---|
| Objetivo docente, ocho capas, aislamiento de agentes, arquitectura y Docker Compose | [Diseño del módulo 01](docs/modulo-01/diseno.md) |
| Preguntas, claves, medición pre/post y ejemplo de interpretación | [Instrumento del piloto](docs/modulo-01/instrumento-pre-post.md) |
| Constitución, especificación, aclaración, plan, tareas y análisis | [Prompts para Spec Kit en Codex](docs/modulo-01/prompts-spec-kit.md) |
| Interfaz de referencia | [HTML de Semana 5](docs/referencias/html-semana5/Paciente_Virtual.html) y [wireframes e instrumentos](docs/referencias/interfaz/) |
| Imágenes de los tres pacientes, escenas, instrumentos, iconos, paleta, tipografía y licencias | [Recursos audiovisuales](recursos-audiovisuales/) |
| Catálogo completo de MedSP1000, incluido al clonar | [Guía del dataset](dataset/README.md) · [Índice de escenarios](dataset/medsp1000/scenario_index.json) |
| Material del actor, estudiante, entorno y evaluador | [LR · P01](datos/medsp1000/mededportal_10181/scenario1/) · [Nannu · P02](datos/medsp1000/mep_2374-8265.10924-s001/scenario3/) · [Dan Sparks · P03](datos/medsp1000/mededportal_3171/scenario1/) · [Ficha del dataset](datos/medsp1000/README.md) |
| Traducción y decisiones de adaptación de LR | [Adaptación de la entrevista](datos/adaptaciones/lr/) |
| Requisitos, diagramas y pruebas de aceptación | [Documento de requisitos](<docs/requisitos/Documento de requerimientos - Paciente virtual en SIRCLE.md>) |
| Antecedentes | [Investigación sobre ocho capas](docs/referencias/investigacion/ocho-capas.md) · [Ensayo histórico de LR](docs/referencias/ensayo-lr/alcance-y-pruebas.md) |

Desde una copia local se puede abrir el HTML en el navegador conservando a su lado la carpeta `recursos/`. La referencia utiliza datos de demostración. El diseño del módulo identifica cuáles debe aprobar el docente antes de activar un caso. Las referencias históricas que pertenecen a iteraciones anteriores se identifican como tales.

Para la demostración técnica, ejecutar `docker compose up --build` desde la raíz y abrir `http://127.0.0.1:8080`. La guía explica cómo comprobar la preparación, ensayar la recuperación y registrar la aprobación docente que habilita la actividad formal. [Guía de arranque y aceptación](specs/001-simulacion-clinica/quickstart.md#4-aceptar-la-demostración-en-el-equipo-del-profesor)
