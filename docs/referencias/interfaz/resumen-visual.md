# SIRCLE · Propuesta visual 02

9 de noviembre de 2026 · Para revisión con el autor y Andrés Santos.

**Conteo verificado:** 617 palabras. Método y resultados (antecedente histórico: `verificacion/Verificacion_entrega.md`).

La segunda versión replantea los dieciséis wireframes con una prioridad: que el estudiante reconozca qué hacer ahora y pueda consultar los detalles cuando los necesite. Se creó en una carpeta nueva y conserva una copia completa de la propuesta anterior.

La primera entrega hizo visibles muchas funciones, pero también repetía datos de estado, etapa, guardado, tiempo y evaluación en espacios que debían ayudar a actuar. Esta revisión usa las ocho imágenes del autor para reducir esa carga, sin eliminar las funciones del recorrido.

## Cambios principales

Las tarjetas muestran retrato 4:3, nombre, edad, estado breve y acción principal. La edad tiene menor peso que el nombre. Sin iniciar, En pausa y Finalizado combinan punto de color y texto. El motivo se consulta desde un icono de familia con color suave; el detalle del estado se abre desde información. La tarjeta completa ajusta su altura al contenido.

Al entrar al caso aparecen el motivo y la tarea. Los objetivos y los datos iniciales se pueden desplegar. La atención mantiene herramientas a la izquierda, paciente al centro y chat a la derecha. Intento, etapa y tiempos pasan a un panel de sesión, disponible sin ocupar permanentemente el encabezado.

Los exámenes generales se agrupan en seis signos vitales y tres métodos. Ver brazo y Ampliar se separan de las mediciones. Los adicionales se buscan por nombre o categoría; una solicitud queda pendiente y no se confunde con un resultado obtenido. [Vista de atención](<recursos/bocetos/B03_atencion.svg>) y [panel de sesión](<recursos/bocetos/B04_sesion.svg>).

La presión arterial tiene paciente a la izquierda y monitor a la derecha. El recorrido incluye estado inicial, preparación del brazalete, medición de veinte segundos, resultado e histórico. Los ceros iniciales son marcadores de espera, no valores clínicos. Cancelar no genera resultados. Registrar en ficha es posterior a medir y evita duplicados.

El histórico separa presión en mmHg y pulso en lpm. Ofrece también una tabla. La demo empieza sin mediciones y añade solo las completadas; las láminas muestran ejemplos identificados como ficticios. [Estado inicial](<recursos/wireframes/07_medir_presion_arterial.svg>) e [histórico ampliado](<recursos/estados/07D_historico.svg>).

Las notas funcionan como puntos que se pueden añadir, editar, quitar y recuperar con Deshacer. Las hipótesis mantienen una evidencia cercana. La entrega resume apartados y explica que confirmar cierra su edición. La devolución destaca comentarios útiles antes que métricas. [Notas editables](<recursos/bocetos/B06_notas.svg>).

La vista docente separa la tabla del grupo de la revisión individual. Guardar revisión, publicar devolución y rectificar siguen siendo acciones distintas. Asignar una actividad y preparar un caso se organizan por pasos y secciones, con un solo apartado de trabajo abierto. [Revisión individual](<recursos/bocetos/B08_revision_docente.svg>).

## Qué se puede revisar

La [galería HTML](<recursos/wireframes>) permite probar el recorrido, consultar los dieciséis wireframes y comparar las piezas con las imágenes del autor. El [catálogo funcional](<Funcionalidades_16_wireframes_SIRCLE.md>) explica 113 identificadores en descripciones de hasta treinta palabras; el máximo observado es veintiuna. Los cuatro estados ampliados de presión continúan la numeración de la lámina 07.

La demo conserva el trabajo mientras la página permanece abierta. Recargar lo reinicia. No implementa IA, conexión con SIRCLE ni mediciones reales. Los valores y tiempos son ilustrativos. Las pruebas automáticas y la revisión visual comprueban los artefactos; no demuestran mejoras de aprendizaje ni sustituyen pruebas con estudiantes.

Quedan por acordar el protocolo clínico de cada acción, las rúbricas, los permisos, las reglas de repetición y la persistencia real. Para la siguiente reunión conviene recorrer tarjeta, atención y presión, y después revisar si el mismo criterio de información bajo demanda funciona en entrega, evaluación y supervisión.

El [análisis técnico](<analisis-visual.md>) desarrolla las fuentes, las decisiones y sus límites. La comparación por pantalla (antecedente histórico: `Cambios_v1_a_v2.md`) permite localizar qué cambió y dónde quedó disponible la información que se retiró de la vista inicial.
