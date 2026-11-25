# Plan técnico del módulo 01 · Paciente Virtual SIRCLE

**Rama de trabajo** `feature/prototipo` · **Especificación** `001-simulacion-clinica` · **Fecha** 2026-11-25

## Objetivo y contexto técnico

El profesor podrá iniciar la demostración desde un clon limpio, entrevistar a LR, medir su presión y seguir una hipótesis hasta la evidencia que sustenta la devolución. La entrega se construirá en cuatro hitos. El cliente usará HTML, CSS y TypeScript. Rust con Axum expondrá una API REST con JSON. SQLite conservará las fuentes clínicas, el progreso y las evaluaciones. Ollama ejecutará `qwen3.5:4b` localmente. La primera aceptación se realizará en el computador del profesor. [Especificación y acuerdos docentes](spec.md)

## Hito 1. Preparar aplicaciones independientes y persistencia

Crear `frontend/` y `backend/` en la raíz. Dentro de cada aplicación, `src/modules/` agrupará acceso, intentos, entrevista, valoración, exámenes, razonamiento, encuestas y devolución o evaluación según su responsabilidad. El arranque y la composición de dependencias residirán en `src/app/`. Extraer código a `shared/` cuando dos funcionalidades compartan comportamiento. El navegador hablará con `/api` mediante el mismo origen. El servicio frontend reenviará esas peticiones al backend por la red interna.

Preparar migraciones en `backend/migrations/`. Importar los catorce documentos de LR, Nannu y Dan desde `datos/`, con contenido, rol, ruta y SHA-256. Separar fuente original, adaptación española y versión aprobada. Al abrir un intento, fijar las versiones de caso, rúbrica e instrumento. **Paso al hito 2:** demostrar una medición con respuestas de ejemplo en el frontend, registrar esa misma operación directamente en la API y repetir la preparación de SQLite conservando tres escenarios y un intento previo. [Modelo de datos](data-model.md) · [Contrato de la API](contracts/api.md)

## Hito 2. Extraer la interfaz sin perder sus interacciones

Copiar `docs/referencias/html-semana5/Paciente_Virtual.html` y su carpeta `recursos/` a un área privada de trabajo del frontend. Cotejar las 29 rutas y sus huellas antes de extraer estilos a CSS y comportamiento a TypeScript. Mantener marcado, selectores, imágenes, audio y navegación. Migrar primero selección y reanudación, después entrevista y valoración, y finalmente notas, hipótesis y entrega. Usar una lista explícita de archivos públicos para que el navegador solo reciba material permitido en su fase.

**Paso al hito 3:** comparar capturas a igual resolución y recorrer con teclado preparación, colocación del brazalete, lectura e histórico. Cancelar deja cero mediciones. Reenviar el registro confirmado conserva una. La referencia visual mantiene su cifra histórica. La ficha de LR usará el triaje inicial seleccionado, 148/85 mmHg y pulso de 102/min, al habilitar la versión validada por el docente.

## Hito 3. Integrar los cuatro agentes y la evaluación

Ejecutar cuatro trabajadores independientes bajo `backend/src/agentes/`, en `lr/`, `nannu/`, `dan_sparks/` y `evaluador/`. Cada uno tendrá ocho directorios numerados para rol, fuentes y memoria, herramientas, acceso, controles, caché, enrutamiento y observabilidad. Los 32 README enlazarán requisito, justificación, fuentes permitidas, comportamiento, implementación y prueba. La matriz de la especificación fija los nombres de directorio y la conducta por agente. [Matriz de los ocho núcleos](spec.md#hito-3--habilitar-los-cuatro-trabajadores-y-comprobar-sus-ocho-capas)

Axum autorizará al usuario antes de recuperar hechos por intento, versión, fase y hablante. Enviará al trabajador el contexto mínimo. Una sugerencia intacta recupera su respuesta aprobada. El texto libre puede requerir inferencia. Validar el identificador elegido antes de mostrar una respuesta. Distinguir petición de aclaración, desconocimiento documentado, dato ausente y fallo técnico. Cada traza deberá permitir encontrar la fuente y la regresión correspondiente.

Al confirmar la entrega, guardar en una transacción la copia inmutable y la evaluación pendiente. El evaluador revisará entrevista, valoración, decisiones, evidencia e hipótesis según la rúbrica de pregrado avanzado. Por criterio explicará actuación, evidencia, expectativa, juicio y mejora concreta. **Paso al hito 4:** superar E12 con las dos entregas de calibración, E17 con peticiones simultáneas y E22 con medición y entrega nuevas durante una caída. Al recuperarse Qwen debe quedar una evaluación vigente. [Decisiones de integración](research.md)

## Hito 4. Demostrar el recorrido y comprobar recuperación

Distribuir tres servicios permanentes mediante Docker Compose: frontend, backend e inferencia. Un cuarto servicio temporal, `sqlite-init`, preparará la base. Publicar solo la interfaz en localhost, usar volúmenes para datos y pesos y permitir el primer arranque desde fuentes públicas sin credenciales. Ante un modelo ausente, descargarlo, verificarlo y probar la carga con reintentos limitados.

La aceptación incluirá pre de cinco preguntas antes de mostrar la ficha, entrevista, medición, ECG textual tras confirmar la orden, hipótesis principal contrastada con una alternativa, entrega, post de siete preguntas y publicación docente. Ambas encuestas serán omitibles. Comparar percepción por ítem y razonamiento por forma y secuencia A→B/B→A. Medir en frío y caliente y recuperar el intento tras reiniciar. RR01 y los dos perfiles de cien estudiantes corresponden al posterior ensayo de cohorte. [Guía de validación](quickstart.md)

## Revisión constitucional y decisiones pendientes

Los controles de entrada y salida del diseño cubren los principios I–VI mediante E01, E07, E12, E17, E21 y E22. El docente conserva la aprobación de la ficha española, la redacción del ECG, las claves y las ponderaciones antes de activar el contenido. El responsable del proyecto resolverá por enmienda la precisión pendiente del principio I. La preparación de infraestructura puede avanzar con datos de prueba. [Constitución vigente](../../.specify/memory/constitution.md)
