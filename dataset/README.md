# Catálogo completo de MedSP1000

Al clonar Paciente Virtual SIRCLE, los casos quedan disponibles en `dataset/medsp1000/`. Para consultarlos basta un navegador en GitHub o un editor de texto en el equipo. El catálogo se puede leer sin instalar Python, Hugging Face, Docker ni el modelo de IA.

## Cómo encontrar un caso

1. Abre el [índice original de escenarios](medsp1000/scenario_index.json). Cada entrada indica una ruta relativa al directorio `medsp1000/`.
2. Abre esa carpeta y elige el rol. Los documentos están en inglés, en formato Markdown.
3. Por ejemplo, en [LR, escenario 1](medsp1000/mededportal_10181/scenario1/) puedes comparar el [guion de la paciente](<medsp1000/mededportal_10181/scenario1/sp_actor/Case A-Asthma.md>) con la [pauta del evaluador](<medsp1000/mededportal_10181/scenario1/evaluator/Case A-Asthma.md>).

| Carpeta del escenario | Qué contiene |
| --- | --- |
| `sp_actor/` | Guion del paciente estandarizado o de su acompañante. |
| `examinee/` | Material para quien realiza la atención. |
| `environment_controller/` | Información del entorno y hallazgos que puede revelar el escenario. |
| `evaluator/` | Pautas y documentos para evaluar la actuación. |

## Versión y procedencia

La copia conserva las rutas y el contenido de los **9.961 archivos** de la revisión `55e3e55efd08c73baab912ba0c5b42637114fbc8` (43.066.584 bytes). Incluye 9.958 documentos de escenarios, el índice, la ficha original y el archivo de atributos de Git. Se cotejaron las rutas y las huellas de los archivos con el inventario de esa revisión. [Revisión exacta en Hugging Face](https://huggingface.co/datasets/byrLLCC/MedSP1000/tree/55e3e55efd08c73baab912ba0c5b42637114fbc8)

El índice distribuido enumera **1.639 rutas de escenario en 613 carpetas de origen**. La ficha de los autores reporta **1.638 casos**. Aquí se conserva el índice tal como se publicó. La ficha declara licencia MIT para la colección, identifica a MedEdPORTAL como fuente y contiene la cita bibliográfica de los autores. [Ficha original, licencia y cita](medsp1000/README.md)

## Uso en el módulo 01

Este catálogo sirve para consultar y seleccionar escenarios. La preparación de SQLite del módulo 01 utiliza los **14 documentos** de los tres escenarios seleccionados en [datos/medsp1000](../datos/medsp1000/). Las [adaptaciones en español de LR](../datos/adaptaciones/lr/) se mantienen junto a los datos del módulo. El catálogo completo pertenece al material de referencia del repositorio y queda fuera de los archivos que la aplicación sirve al estudiante.
