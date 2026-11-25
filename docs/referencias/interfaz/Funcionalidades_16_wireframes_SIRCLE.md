# Funcionalidades de los 16 wireframes · Versión 2

Cada código identifica lámina y elemento: **01-1** es la primera llamada de la lámina 01. Los casos de uso oficiales permanecen indicados aparte. La numeración describe esta versión; no debe mezclarse con el catálogo anterior.

Los estados ampliados de presión pertenecen a la lámina 07 y continúan su secuencia de identificadores. Cada descripción contiene como máximo treinta palabras.

[Abrir galería](<recursos/wireframes>) · [Resumen](<resumen-visual.md>) · [Análisis](<analisis-visual.md>)

## 01. Elegir un paciente

**Usuario:** Estudiante. **Casos de uso:** CU04. **Requerimientos:** RF04.

![Elegir un paciente](<recursos/wireframes/01_elegir_un_paciente.png>)

[Ampliar SVG](<recursos/wireframes/01_elegir_un_paciente.svg>) · [Ver lámina](<recursos/wireframes/01_elegir_un_paciente.svg>)

| Código | Nombre de la llamada | Función |
| --- | --- | --- |
| 01-1 | Ficha paciente | Reúne retrato, nombre, edad y acción principal; la imagen conserva una proporción de cuatro a tres. |
| 01-2 | Buscar paciente | Localiza un paciente por su nombre; los filtros adicionales se abren cuando se necesitan. |
| 01-3 | Vistas por estado | Alterna todos los casos, los intentos en pausa y los finalizados sin mezclar progreso con calificación. |
| 01-4 | Detalles del estado | Muestra etapa, guardado y situación de la devolución al pulsar información; evita repetir estos datos en la tarjeta. |
| 01-5 | Estado breve | Usa Sin iniciar, En pausa o Finalizado, con punto de color y texto para reconocer la continuidad del caso. |
| 01-6 | Motivo bajo demanda | Revela motivo y familia al pulsar, enfocar o señalar el icono. El color pertenece al icono, no a toda la tarjeta. |
| 01-7 | Acción principal | Inicia el caso, reanuda el mismo intento o abre su resumen según el estado del paciente. |

## 02. Retomar o revisar

**Usuario:** Estudiante. **Casos de uso:** CU04, CU07, CU12. **Requerimientos:** RF04, RF09.

![Retomar o revisar](<recursos/wireframes/02_retomar_o_revisar.png>)

[Ampliar SVG](<recursos/wireframes/02_retomar_o_revisar.svg>) · [Ver lámina](<recursos/wireframes/02_retomar_o_revisar.svg>)

| Código | Nombre de la llamada | Función |
| --- | --- | --- |
| 02-1 | Intento en pausa | Destaca el caso pendiente y conserva el acceso directo al mismo paciente, conversación y registros. |
| 02-2 | Retomar atención | Continúa el intento existente; no crea una práctica nueva ni reinicia los hallazgos. |
| 02-3 | Detalle de continuidad | Despliega el punto de retorno, el guardado confirmado y el identificador del intento cuando el estudiante necesita consultarlos. |
| 02-4 | Resumen del caso | Abre el trabajo finalizado y los comentarios publicados; una devolución pendiente sigue siendo un estado explícito. |
| 02-5 | Historial compacto | Agrupa los casos finalizados con nombre, estado y acceso al resumen; reserva notas y tiempos para su detalle. |

## 03. Continuar una atención

**Usuario:** Estudiante. **Casos de uso:** CU04, CU11, CU12. **Requerimientos:** RF04.

![Continuar una atención](<recursos/wireframes/03_continuar_una_atencion.png>)

[Ampliar SVG](<recursos/wireframes/03_continuar_una_atencion.svg>) · [Ver lámina](<recursos/wireframes/03_continuar_una_atencion.svg>)

| Código | Nombre de la llamada | Función |
| --- | --- | --- |
| 03-1 | Paciente que retomas | Confirma la identidad antes de regresar a una atención que quedó en pausa. |
| 03-2 | Punto de retorno | Expresa la tarea siguiente con un título breve, sin desplegar permanentemente la lista de todos los registros recuperables. |
| 03-3 | Continuar | Vuelve al mismo intento con conversación, notas, hallazgos y momento simulado conservados. |
| 03-4 | Información recuperada | Muestra identificación del intento, etapa, último guardado y registros disponibles en un panel secundario. |
| 03-5 | Nueva práctica | Crea otro intento solo cuando la actividad lo permite; conserva intacta la práctica anterior. |

## 04. Conocer al paciente

**Usuario:** Estudiante. **Casos de uso:** CU04, CU09. **Requerimientos:** RF04, RF11.

![Conocer al paciente](<recursos/wireframes/04_conocer_al_paciente.png>)

[Ampliar SVG](<recursos/wireframes/04_conocer_al_paciente.svg>) · [Ver lámina](<recursos/wireframes/04_conocer_al_paciente.svg>)

| Código | Nombre de la llamada | Función |
| --- | --- | --- |
| 04-1 | Identidad y motivo | Muestra la información inicial al abrir el caso; distingue el motivo referido de un diagnóstico confirmado. |
| 04-2 | Tarea principal | Resume el propósito de la práctica con una frase; el detalle de objetivos permanece disponible en un apartado desplegable. |
| 04-3 | Ficha inicial | Despliega los datos autorizados al inicio; no revela antecedentes que deban obtenerse durante la entrevista. |
| 04-4 | Objetivos ampliados | Permite consultar los objetivos de aprendizaje cuando hacen falta sin mantenerlos todos abiertos durante la atención. |
| 04-5 | Comenzar atención | Abre el espacio clínico con el paciente, las acciones y la conversación disponibles. |
| 04-6 | Encuesta inicial | Abre una encuesta opcional separada del desempeño clínico; omitirla no impide iniciar la atención. |

## 05. Atender y conversar

**Usuario:** Estudiante. **Casos de uso:** CU05, CU13, CU11. **Requerimientos:** RF05, RF06, RF04.

![Atender y conversar](<recursos/wireframes/05_atender_y_conversar.png>)

[Ampliar SVG](<recursos/wireframes/05_atender_y_conversar.svg>) · [Ver lámina](<recursos/wireframes/05_atender_y_conversar.svg>)

| Código | Nombre de la llamada | Función |
| --- | --- | --- |
| 05-1 | Grupos de examen | Alterna examen general y búsqueda de exámenes adicionales; conserva el contexto y los resultados ya obtenidos. |
| 05-2 | Signos vitales | Reúne presión, pulso, temperatura, respiración, saturación y peso. Cada icono conserva un nombre visible. |
| 05-3 | Métodos de examen | Agrupa inspección, palpación y auscultación para localizar acciones por su propósito. |
| 05-4 | Vista del paciente | Permite acercarse al brazo o ampliar al paciente sin cambiar la etapa clínica ni borrar la conversación. |
| 05-5 | Paciente presente | Mantiene visible a la misma persona durante la atención y contextualiza los procedimientos realizados. |
| 05-6 | Información de sesión | Abre estado, intento, etapa y tiempos bajo demanda; esos datos dejan de ocupar permanentemente el encabezado. |
| 05-7 | Cuaderno por pestañas | Alterna chat, notas, evidencia e hipótesis y conserva el contenido y la posición de trabajo. |
| 05-8 | Conversación | Diferencia las intervenciones del paciente y del estudiante, con historial desplazable dentro del panel. |
| 05-9 | Enviar pregunta | Envía la pregunta escrita. La demostración responde a un guion breve y conserva lo no reconocido para corregirlo. |
| 05-10 | Ficha clínica | Abre el registro vinculado al intento; acceder a la ficha no entrega ni finaliza la actividad. |

## 06. Escribir notas por puntos

**Usuario:** Estudiante. **Casos de uso:** CU06, CU11. **Requerimientos:** RF07, RF04.

![Escribir notas por puntos](<recursos/wireframes/06_escribir_notas_por_puntos.png>)

[Ampliar SVG](<recursos/wireframes/06_escribir_notas_por_puntos.svg>) · [Ver lámina](<recursos/wireframes/06_escribir_notas_por_puntos.svg>)

| Código | Nombre de la llamada | Función |
| --- | --- | --- |
| 06-1 | Lista de apuntes | Presenta notas breves del estudiante como puntos independientes, conservando la separación respecto de lo dicho por el paciente. |
| 06-2 | Editar un punto | Abre el texto del apunte para corregirlo; cancelar conserva su versión anterior. |
| 06-3 | Quitar un punto | Retira el apunte seleccionado y ofrece deshacer para recuperarlo durante la sesión. |
| 06-4 | Nuevo apunte | Recibe un punto breve sin reemplazar las notas existentes ni perder el borrador al cambiar de pestaña. |
| 06-5 | Añadir punto | Incorpora el apunte a la lista y confirma el resultado. Un texto vacío no genera una nota. |
| 06-6 | Consultar evidencia | Abre los hallazgos obtenidos para fundamentar un apunte; la nota personal no se convierte automáticamente en evidencia clínica. |

## 07. Medir presión arterial

**Usuario:** Estudiante. **Casos de uso:** CU05, CU13, CU11. **Requerimientos:** RF06, RF07.

![Medir presión arterial](<recursos/wireframes/07_medir_presion_arterial.png>)

[Ampliar SVG](<recursos/wireframes/07_medir_presion_arterial.svg>) · [Ver lámina](<recursos/wireframes/07_medir_presion_arterial.svg>)

| Código | Nombre de la llamada | Función |
| --- | --- | --- |
| 07-1 | Paciente a la izquierda | Conserva identidad y postura visible durante la preparación y medición, con el instrumento representado en el brazo. |
| 07-2 | Monitor en espera | Muestra 000 como marcador de espera junto a Sin medición; esos ceros no se registran como resultados clínicos. |
| 07-3 | Preparar medición | Abre la preparación del brazalete antes de iniciar la simulación; todavía no produce ni registra valores. |
| 07-4 | Información adicional | Explica el alcance de esta acción y las unidades en un panel secundario accesible por teclado y pulsación. |
| 07-5 | Volver al paciente | Regresa a la conversación del mismo intento; una medición activa solicita confirmar su cancelación antes de salir. |
| 07-6 | Abrir histórico | Consulta las mediciones completadas del intento como gráfico o tabla; no incluye intentos cancelados. |

### Preparación de la medición

![Preparación de la medición](<recursos/estados/07A_preparacion.png>)

| Código | Nombre de la llamada | Función |
| --- | --- | --- |
| 07-7 | Preparar el brazalete | Marca la colocación del brazalete en la simulación; el dibujo ayuda a reconocer el paso sin sustituir una instrucción clínica validada. |
| 07-8 | Confirmar preparación | Comprueba colocación y ajuste mediante dos casillas. La medición se habilita al completar ambos pasos. |
| 07-9 | Iniciar medición | Inicia una espera demostrativa de veinte segundos después de confirmar la preparación; bloquea activaciones duplicadas. |
| 07-10 | Cancelar preparación | Cierra la preparación sin generar resultados ni registros; devuelve el foco al control que la abrió. |

### Medición en curso

![Medición en curso](<recursos/estados/07B_midiendo.png>)

| Código | Nombre de la llamada | Función |
| --- | --- | --- |
| 07-11 | Progreso de medición | Muestra segundos restantes y progreso de la simulación; conserva los valores sin resultado hasta que termine. |
| 07-12 | Cancelar medición | Detiene el proceso sin añadir valores al histórico ni modificar mediciones anteriores. |

### Medición completa

![Medición completa](<recursos/estados/07C_resultado.png>)

| Código | Nombre de la llamada | Función |
| --- | --- | --- |
| 07-13 | Resultado de la medición | Presenta SYS, DIA y pulso con sus unidades y el momento simulado; los valores del ejemplo son ficticios. |
| 07-14 | Registrar resultado | Añade una referencia de la medición a la ficha del intento una sola vez; evita duplicarla al volver a pulsar. |

### Histórico por unidades

![Histórico por unidades](<recursos/estados/07D_historico.png>)

| Código | Nombre de la llamada | Función |
| --- | --- | --- |
| 07-15 | Presión en el tiempo | Compara valores sistólicos y diastólicos en mmHg con etiquetas y trazos distintos; el eje horizontal indica momentos simulados. |
| 07-16 | Pulso separado | Muestra el pulso en latidos por minuto en una gráfica independiente para no mezclar unidades con la presión. |
| 07-17 | Tabla de mediciones | Alterna la gráfica con valores textuales, unidades, momento y estado de registro de cada medición. |

## 08. Registrar una acción

**Usuario:** Estudiante. **Casos de uso:** CU06, CU11. **Requerimientos:** RF07, RF08.

![Registrar una acción](<recursos/wireframes/08_registrar_una_accion.png>)

[Ampliar SVG](<recursos/wireframes/08_registrar_una_accion.svg>) · [Ver lámina](<recursos/wireframes/08_registrar_una_accion.svg>)

| Código | Nombre de la llamada | Función |
| --- | --- | --- |
| 08-1 | Tipo de registro | Separa documentación, acciones e historial; cambiar de apartado conserva el borrador. |
| 08-2 | Acción permitida | Selecciona una acción preparada para el caso; los parámetros se revisan antes de ejecutar. |
| 08-3 | Indicación escrita | Permite redactar y justificar una propuesta sin convertirla todavía en una acción ejecutada. |
| 08-4 | Estado de la propuesta | Distingue Borrador, Preparada y Ejecutada; cada estado refleja una consecuencia diferente. |
| 08-5 | Guardar borrador | Conserva lo escrito sin ejecutar la acción ni entregar la actividad. |
| 08-6 | Revisar y confirmar | Abre una revisión breve de acción, parámetros y consecuencia antes de ejecutar y añadir un evento al intento. |

## 09. Revisar la evolución

**Usuario:** Estudiante. **Casos de uso:** CU06, CU14. **Requerimientos:** RF08.

![Revisar la evolución](<recursos/wireframes/09_revisar_la_evolucion.png>)

[Ampliar SVG](<recursos/wireframes/09_revisar_la_evolucion.svg>) · [Ver lámina](<recursos/wireframes/09_revisar_la_evolucion.svg>)

| Código | Nombre de la llamada | Función |
| --- | --- | --- |
| 09-1 | Cronología breve | Agrupa síntomas, indicaciones y respuesta; cada evento puede ampliar sus datos y fuentes. |
| 09-2 | Eventos con momento | Ubica cada cambio en el tiempo clínico del caso; no confunde esos intervalos con la duración real de la sesión. |
| 09-3 | Respuesta pendiente | Indica que falta avanzar para observar la respuesta preparada; no muestra resultados futuros como hallazgos obtenidos. |
| 09-4 | Avanzar el caso | Propone un intervalo permitido y muestra su consecuencia antes de confirmar; cancelar conserva el momento y los registros existentes. |
| 09-5 | Volver a conversar | Regresa al paciente para reevaluar después del cambio sin reiniciar la atención. |
| 09-6 | Tiempo de sesión | Permite consultar la duración real y otros metadatos desde información, fuera del recorrido principal de eventos. |

## 10. Fundamentar una hipótesis

**Usuario:** Estudiante. **Casos de uso:** CU06, CU11. **Requerimientos:** RF07.

![Fundamentar una hipótesis](<recursos/wireframes/10_fundamentar_una_hipotesis.png>)

[Ampliar SVG](<recursos/wireframes/10_fundamentar_una_hipotesis.svg>) · [Ver lámina](<recursos/wireframes/10_fundamentar_una_hipotesis.svg>)

| Código | Nombre de la llamada | Función |
| --- | --- | --- |
| 10-1 | Evidencia consultable | Mantiene a la vista el hallazgo seleccionado mientras se escribe una interpretación; no muestra información todavía no obtenida. |
| 10-2 | Origen del hallazgo | Abre el mensaje o la acción que produjo la evidencia para verificar qué sustenta la interpretación. |
| 10-3 | Interpretación provisional | Permite expresar una hipótesis sin convertirla en diagnóstico confirmado ni ocultar la incertidumbre. |
| 10-4 | Fundamento y límites | Relaciona hallazgos con el razonamiento y permite señalar lo que todavía falta explorar. |
| 10-5 | Vincular evidencia | Selecciona evidencia obtenida y añade una referencia explícita a la hipótesis. |
| 10-6 | Guardar hipótesis | Conserva el razonamiento del estudiante, separado de los hallazgos y de la entrega académica. |
| 10-7 | Revisar el intento | Abre la revisión de entrega cuando el estudiante decide cerrar; guardar una hipótesis no entrega automáticamente. |

## 11. Revisar antes de entregar

**Usuario:** Estudiante. **Casos de uso:** CU07, CU15. **Requerimientos:** RF09.

![Revisar antes de entregar](<recursos/wireframes/11_revisar_antes_de_entregar.png>)

[Ampliar SVG](<recursos/wireframes/11_revisar_antes_de_entregar.svg>) · [Ver lámina](<recursos/wireframes/11_revisar_antes_de_entregar.svg>)

| Código | Nombre de la llamada | Función |
| --- | --- | --- |
| 11-1 | Revisión por apartados | Resume conversación, notas, evidencia y razonamiento sin mostrar de nuevo todo su contenido. |
| 11-2 | Abrir un apartado | Despliega el contenido seleccionado para revisarlo; permite volver al editor correspondiente antes de entregar. |
| 11-3 | Consecuencia de entrega | Aclara que confirmar cierra la edición; los comentarios y la evaluación pueden publicarse posteriormente. |
| 11-4 | Seguir editando | Regresa a la atención conservando lo escrito y los hallazgos disponibles. |
| 11-5 | Confirmar entrega | Finaliza el intento tras la revisión; una entrega pendiente de evaluación no recibe automáticamente una nota cero. |

## 12. Consultar el resumen

**Usuario:** Estudiante. **Casos de uso:** CU07, CU09, CU12, CU15. **Requerimientos:** RF09, RF11.

![Consultar el resumen](<recursos/wireframes/12_consultar_el_resumen.png>)

[Ampliar SVG](<recursos/wireframes/12_consultar_el_resumen.svg>) · [Ver lámina](<recursos/wireframes/12_consultar_el_resumen.svg>)

| Código | Nombre de la llamada | Función |
| --- | --- | --- |
| 12-1 | Resumen del caso | Identifica al paciente y expresa Finalizado; la situación de la devolución se consulta dentro del resumen. |
| 12-2 | Detalle de evaluación | Despliega estudiante, curso, intento, duración, rúbrica y resultado cuando están disponibles; no fabrica una calificación. |
| 12-3 | Áreas revisables | Organiza comentarios por habilidades de la rúbrica y muestra una categoría a la vez. |
| 12-4 | Comentario prioritario | Destaca lo realizado y una mejora concreta antes de mostrar listas extensas de criterios o métricas. |
| 12-5 | Omisión con contexto | Explica una acción esperada sin registro, diferenciándola de una acción ejecutada incorrectamente. |
| 12-6 | Revisar evidencia | Abre el fragmento del intento que sustenta el comentario docente. |
| 12-7 | Otra práctica | Crea un nuevo intento solo si está autorizado; conserva el resumen anterior para comparación. |
| 12-8 | Encuesta final | Recoge experiencia o confianza de forma opcional, separada de la calificación del intento. |

## 13. Asignar una actividad

**Usuario:** Docente. **Casos de uso:** CU03. **Requerimientos:** RF03.

![Asignar una actividad](<recursos/wireframes/13_asignar_una_actividad.png>)

[Ampliar SVG](<recursos/wireframes/13_asignar_una_actividad.svg>) · [Ver lámina](<recursos/wireframes/13_asignar_una_actividad.svg>)

| Código | Nombre de la llamada | Función |
| --- | --- | --- |
| 13-1 | Configuración por pasos | Distribuye caso, destinatarios y condiciones en etapas editables; cambiar de paso conserva las selecciones. |
| 13-2 | Buscar destinatario | Filtra estudiantes dentro del grupo autorizado por nombre o matrícula. |
| 13-3 | Seleccionar estudiantes | Marca destinatarios explícitamente; buscar no cambia por sí solo quién recibe la actividad. |
| 13-4 | Recuento de selección | Confirma cuántos estudiantes recibirían la actividad antes de revisar las condiciones. |
| 13-5 | Revisar condiciones | Abre disponibilidad, intentos permitidos y publicación; habilitar requiere revisar el resumen de asignación. |
| 13-6 | Contexto de asignación | Permite consultar curso, grupo, caso y versión elegidos sin repetir todos los datos en cada paso. |

## 14. Supervisar intentos

**Usuario:** Docente. **Casos de uso:** CU08, CU16. **Requerimientos:** RF10, RF09.

![Supervisar intentos](<recursos/wireframes/14_supervisar_intentos.png>)

[Ampliar SVG](<recursos/wireframes/14_supervisar_intentos.svg>) · [Ver lámina](<recursos/wireframes/14_supervisar_intentos.svg>)

| Código | Nombre de la llamada | Función |
| --- | --- | --- |
| 14-1 | Contexto académico | Filtra por curso, grupo y actividad autorizados antes de localizar un intento. |
| 14-2 | Buscar estudiante | Localiza al estudiante por nombre o matrícula sin desplegar métricas de todo el grupo. |
| 14-3 | Estado del intento | Distingue En curso, Por revisar, Publicado y Sin registros para anticipar la acción docente posible. |
| 14-4 | Revisión individual | Abre una vista dedicada a evidencia y comentarios del intento, con acciones separadas para guardar, publicar y rectificar. |
| 14-5 | Resumen del grupo | Despliega una gráfica de conteos y detalles académicos cuando el docente necesita comparar estados. |
| 14-6 | Intento sin registros | Muestra una ausencia de actividad explícita; no la convierte en nota cero ni inventa una entrega. |

## 15. Administrar el módulo

**Usuario:** Administrador. **Casos de uso:** CU02, CU03, CU10. **Requerimientos:** RF02, RF03.

![Administrar el módulo](<recursos/wireframes/15_administrar_el_modulo.png>)

[Ampliar SVG](<recursos/wireframes/15_administrar_el_modulo.svg>) · [Ver lámina](<recursos/wireframes/15_administrar_el_modulo.svg>)

| Código | Nombre de la llamada | Función |
| --- | --- | --- |
| 15-1 | Destinos administrativos | Separa catálogo, accesos y cambios para evitar mezclar contenido clínico con permisos o auditoría. |
| 15-2 | Nuevo borrador | Crea una versión de trabajo sin habilitarla automáticamente a estudiantes. |
| 15-3 | Versiones del caso | Mantiene identificables las versiones y su estado para conservar el contenido usado por intentos anteriores. |
| 15-4 | Acciones de versión | Abre editar, revisar historial o retirar del catálogo según permisos. Retirar solicita confirmación y conserva los intentos existentes. |
| 15-5 | Accesos y grupos | Gestiona los permisos del módulo dentro de los grupos autorizados por SIRCLE. |
| 15-6 | Límite de responsabilidad | Explica el permiso activo y sus límites; administrar accesos no concede automáticamente revisión de calificaciones. |

## 16. Preparar y revisar un caso

**Usuario:** Preparador autorizado. **Casos de uso:** CU03, CU10. **Requerimientos:** RF03.

![Preparar y revisar un caso](<recursos/wireframes/16_preparar_y_revisar_un_caso.png>)

[Ampliar SVG](<recursos/wireframes/16_preparar_y_revisar_un_caso.svg>) · [Ver lámina](<recursos/wireframes/16_preparar_y_revisar_un_caso.svg>)

| Código | Nombre de la llamada | Función |
| --- | --- | --- |
| 16-1 | Versión del contenido | Identifica la versión de trabajo; los cambios no modifican retrospectivamente casos ya asignados. |
| 16-2 | Secciones de preparación | Divide datos, diálogo, evolución y rúbrica; mantiene una sección activa y conserva los borradores de las demás. |
| 16-3 | Etapa abierta | Muestra únicamente los campos de la etapa clínica que se está editando. |
| 16-4 | Etapas plegadas | Permite abrir indicaciones o respuesta y cierre sin mostrar todos sus campos simultáneamente. |
| 16-5 | Revisión pendiente | Despliega qué apartados requieren revisión clínica y académica antes de habilitar el caso. |
| 16-6 | Guardar y continuar | Conserva la sección actual y permite seguir preparando el caso sin publicarlo. |
| 16-7 | Solicitar revisión | Envía la versión al responsable autorizado cuando cumple los requisitos mínimos; guardar no equivale a aprobación clínica. |

## Funciones compartidas

CU01 y CU10 acompañan acceso y autorización. CU11 conserva avances en el producto previsto; la demo solo mantiene datos mientras permanece abierta. Las funciones administrativas y docentes se representan para revisión y no actúan sobre SIRCLE.

**Conteo:** 16 láminas principales, 4 estados ampliados, 113 códigos únicos. Descripción más larga: 21 palabras.