# Propuesta de paciente virtual en Figma

Archivo de trabajo [SIRCLE](https://www.figma.com/design/Zoe7plD3BeVL3r7Cyccv7a).

## Organización

- Propuesta visual Circl reúne la copia de la propuesta y el catálogo de funcionalidades.
- LOW-FI WIREFRAMES contiene las 16 pantallas, 40 estados de ocho instrumentos y cuatro ampliaciones de presión arterial.
- HI-FI WIREFRAMES desarrolla la propuesta con los pacientes de la investigación y la cabecera de escritorio original.
- COMPONENTES PACIENTE VIRTUAL incorpora estados clínicos y variantes emocionales.
- FLUJO PACIENTE VIRTUAL documenta las decisiones, recorridos y retornos.

## Criterios de adaptación

Se reutiliza la cabecera del componente original de SIRCLE y su tipografía Roboto. Los low-fi conservan las ilustraciones y datos demostrativos de la propuesta anterior. Los hi-fi introducen LR, Nannu y Dan Sparks a partir de la investigación de Semana4.

La fuente llama Nanu al segundo perfil. Se utiliza Nannu en las nuevas imágenes y pantallas siguiendo la solicitud del autor. La apariencia es ficticia. Cada expresión representa un estado comunicativo y requiere vinculación al guion clínico en la implementación.

Los valores de las demostraciones de instrumentos proceden del prototipo anterior. No se asignan como constantes clínicas a los tres escenarios. El resultado real debe proceder del estado del caso y del intento.

Los diagnósticos del material de investigación se reservan a la configuración docente. Las pantallas de estudiante muestran el motivo de consulta y la evidencia disponible en cada etapa.

Las pantallas son propuestas de diseño. La navegación de Figma representa transiciones demostrativas. La conversación, medición, persistencia y evaluación requieren implementación en la aplicación.

## Fuentes locales

- `../Propuesta_visual_SIRCLE_v2/recursos/datos_propuesta.json`
- `../Propuesta_visual_SIRCLE_v2/recursos/instrumentos/recursos/datos.js`
- `../Propuesta_visual_SIRCLE_v2/Funcionalidades_16_wireframes_SIRCLE.md`
- `../Propuesta_visual_SIRCLE_v2/Funcionalidades_instrumentos.md`
- `../../Semana4/Flujos y Piloto Paciente Virtual/Investigacion/Analisis_dataset_y_seleccion.md`

Las imágenes se entregan por separado en `../Imágenes serie de emociones pacientes virtual/` y se incorporan al archivo de Figma.

## Entrega verificada

Reedición del 10 de noviembre: 67 láminas y estados low-fi por página, 128 pantallas y estados hi-fi, 16 iconos SVG propios y 31 imágenes incorporadas. Las imágenes comprenden diez estados de cada paciente y un estado adicional de tos febril para Nannu.

La navegación demostrativa de procedimientos utiliza LR. Nannu y Dan tienen fichas, atención, preguntas guiadas, respuestas y fuentes propias. Las ramas compartidas de instrumentos, notas y entrega no conservan un estado independiente por paciente.

La auditoría actual encontró 934 enlaces sin destinos inexistentes, sin texto fuera de los límites exteriores de las pantallas hi-fi y sin solapamientos entre sus 128 marcos. Se revisaron capturas de selección, chat, sesión, evidencia, peso, low-fi e iconos. Los resultados y respuestas son ejemplos fijos.

El detalle de correspondencia con el HTML, las diferencias y las comprobaciones está en Revision_lofi_hifi_2026-11-10.md (antecedente histórico: `Revision_lofi_hifi_2026-11-10.md`). Los SVG y su catálogo autónomo están en [Iconos_SIRCLE](<../../../recursos-audiovisuales/iconos>).
