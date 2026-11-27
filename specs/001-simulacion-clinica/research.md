# Decisiones técnicas que sustentan el plan

El diseño parte de las cinco aclaraciones del módulo. El triaje pertenece a la valoración inicial, el ECG se libera por una orden confirmada y la evaluación corresponde a pregrado avanzado. Las formas A/B se alternan por intento. La capacidad de una cohorte requiere medición posterior. [Especificación aclarada](spec.md#aclaraciones) · [Diseño del módulo 01](../../docs/modulo-01/diseno.md)

## Decisiones de integración

| Decisión | Fundamento y alternativa considerada |
| --- | --- |
| Servir el frontend y reenviar `/api` por el mismo origen. | La interfaz publica únicamente su compilación y los recursos permitidos. Dos instalaciones y pruebas independientes permiten desarrollar valoración con datos de ejemplo. Compilar ambas aplicaciones en un único proceso acoplaría sus ciclos de prueba. [Constitución](../../.specify/memory/constitution.md) · [Router de Axum](https://docs.rs/axum/latest/axum/) |
| Mantener una copia privada de extracción en `frontend/referencia/semana5/`. | La referencia incrusta datos de demostración. Se conservarán sus gestos y recursos extraídos a partir de una copia idéntica y se construirá el paquete público con una lista explícita de archivos. Una copia pública completa expondría material reservado. [HTML de Semana 5](../../docs/referencias/html-semana5/Paciente_Virtual.html) · [Funciones de los instrumentos](../../docs/referencias/interfaz/Funcionalidades_instrumentos.md) |
| Usar la misma imagen Rust para el servicio backend y el trabajo temporal `sqlite-init`. | Dos modos de arranque permiten que el inicializador importe datos y termine. El backend espera `service_completed_successfully` del inicializador. La biblioteca SQLite abre el archivo en el volumen. No hace falta un servidor de base de datos. [Dependencias de Compose](https://docs.docker.com/compose/how-tos/startup-order/) |
| Confirmar en transacciones breves y exigir claves únicas por operación. | La entrega y el trabajo pendiente se recuperan juntos. Las llamadas al modelo se realizan fuera de la transacción. Un identificador de toma vigente protege el resultado tras la caída de un trabajador. Para los respaldos se elige una copia consistente de SQLite, restaurada en un volumen distinto. [Transacciones](https://www.sqlite.org/lang_transaction.html) · [Claves foráneas](https://www.sqlite.org/foreignkeys.html) · [Copia de respaldo](https://www.sqlite.org/backup.html) |
| Iniciar cuatro procesos trabajadores desde el backend y conectar cada uno por un canal local propio de mensajes JSON. | La API resuelve identidad, permisos y fase antes de formar el mensaje. Un contexto autocontenido permite compartir los pesos de Ollama y reiniciar a un trabajador mientras los demás continúan. La selección de un identificador por el modelo requiere validación de pertenencia, significado, hablante y fase antes de producir un turno. [Cuatro agentes](../../docs/modulo-01/diseno.md#cuatro-agentes-independientes) · [Salidas estructuradas de Ollama](https://docs.ollama.com/capabilities/structured-outputs) |
| Probar la pregunta libre de LR por HTTP en el hito 1 y reutilizar esa operación desde el chat del hito 2. | La ficha resuelve las sugerencias literales. En texto libre, Qwen selecciona entre los hechos autorizados y el backend valida pertinencia, cronología y hablante. Una respuesta de ejemplo permite ensayar la vista. Para acreditar el diálogo se exige una llamada real asociada al turno y un 503 al formular la misma pregunta como operación nueva durante la caída. [Ensayo con LR](quickstart.md#conversación-libre-con-lr-por-http) |
| Preparar la rúbrica en un archivo privado por versión e importarla con su aprobación. | La actividad preasignada permite que el docente revise criterios y pesos antes de habilitarla. Una huella une el contenido revisado, el intento y la entrega. Se podrá reproducir una evaluación anterior después de cambiar las ponderaciones. Un editor gráfico se considerará si surge una necesidad docente posterior. [Aprobación y conservación](data-model.md#preparación-y-aprobación-de-la-rúbrica) |
| Preparar y verificar el modelo desde el servicio `inferencia`. | Un único responsable consulta el inventario, descarga la etiqueta oficial si falta y comprueba una respuesta real. Una comprobación que solo detectase el proceso confundiría motor disponible con modelo cargado. La API sigue aceptando operaciones respaldadas por SQLite. [Distribución oficial](https://ollama.com/library/qwen3.5:4b) · [Inventario y digest](https://docs.ollama.com/api/tags) · [Descarga](https://docs.ollama.com/api/pull) · [Generación de prueba](https://docs.ollama.com/api/generate) |
| Mantener separados el flujo educativo y la nota clínica. | El docente compara cada ítem de percepción y ambas formas de razonamiento por secuencia y momento. El evaluador recibe solo la entrega clínica con su rúbrica. Una omisión voluntaria de encuesta conserva el acceso a la práctica y a la devolución publicada. [Instrumento del piloto](../../docs/modulo-01/instrumento-pre-post.md) |

La implementación fijará las versiones de las dependencias en `Cargo.lock` y `package-lock.json`. Para reproducir la demostración se registrarán las imágenes utilizadas, el digest completo de `qwen3.5:4b`, la cuantización, el contexto y el equipo. El digest del modelo se fijará en la configuración pública de la entrega tras verificar el artefacto oficial.

## Preparación del modelo

| Servicio previsto en `compose.yaml` | Arranque, exposición y persistencia |
| --- | --- |
| `frontend` | Publicará la interfaz en `127.0.0.1:8080`. Servirá la compilación y reenviará las llamadas del mismo origen bajo `/api` al backend. |
| `backend` | Esperará la salida correcta de `sqlite-init`, abrirá `/var/lib/sircle/sircle.sqlite` en el volumen `sircle-datos` e iniciará los cuatro trabajadores. La preparación de Qwen es independiente de la salud de la API. |
| `inferencia` | Ejecutará Ollama en la red de Compose. Tendrá el volumen `sircle-pesos` en `/root/.ollama` y acceso saliente al registro oficial durante la preparación. La API y Ollama carecerán de puertos publicados al host. |
| `sqlite-init` | Usará la imagen del backend en modo de preparación, con las fuentes y las versiones de rúbrica en modo lectura y acceso de escritura al volumen de datos. Terminará tras la migración e importación. Un error impedirá que la API opere sobre una base incompleta. |

El punto de entrada de `inferencia` será el único encargado de preparar el modelo. Consultará `GET /api/tags` y cotejará nombre y digest completo con el manifiesto público de la versión preparada para la demo. Si falta `qwen3.5:4b`, solicitará la descarga oficial mediante `POST /api/pull` y volverá a verificar el inventario. Antes de habilitar entrevista libre y evaluación se exigirá una generación breve y correcta sin datos clínicos. Tras esa verificación, el proxy HTTP interno aceptará las peticiones a `inferencia:11434` y las reenviará a Ollama con el encabezado `Host` de su dirección de loopback. La comprobación de salud usará el mismo nombre de red que el backend. Una etiqueta con digest inesperado queda como incidencia de versión y el aviso debe indicar cómo recuperar los pesos aprobados. [Inventario de modelos](https://docs.ollama.com/api/tags) · [Descarga](https://docs.ollama.com/api/pull) · [Prueba de generación](https://docs.ollama.com/api/generate) · [Volumen de Ollama](https://docs.ollama.com/docker)

**Política inicial de recuperación.** Se proponen tres intentos totales por ciclo, con esperas de dos y cuatro segundos tras fallos transitorios. El plazo de inactividad de una descarga se renueva al recibir progreso. Los sondeos de salud observan el estado sin disparar nuevos ciclos de descarga. Al agotarse los intentos, el aviso identificará servicio, modelo, causa y acción de recuperación. Tras corregir la conectividad o el volumen, reiniciar solo `inferencia` iniciará otro ciclo. El backend seguirá atendiendo operaciones de SQLite y recogerá el trabajo pendiente tras recuperar el modelo. [Orden de arranque](https://docs.docker.com/compose/how-tos/startup-order/) · [Red de Compose](https://docs.docker.com/compose/how-tos/networking/)

## Revisión constitucional antes y después del diseño

| Principio | Condición de partida | Cómo la conserva el diseño |
| --- | --- | --- |
| I. Recorrido de LR | Identidad ficticia, pre, práctica, entrega, post y publicación docente. | El contrato controla ambos momentos y la visibilidad de la devolución. E01 y E14 siguen el flujo y sus omisiones. La atribución en P02 distingue el origen de las respuestas al alternar los casos en E16. E17 comprueba que los intentos de LR conservan sus respuestas y continúan cuando se reinicia el trabajador de P02. |
| II. Aplicaciones independientes | Dos instalaciones, ejecuciones y pruebas. | E19 comprueba el gesto con datos de ejemplo y la operación directa al servidor. La paráfrasis de E06 se adelantará al hito 1 y se repetirá desde el chat del hito 2. |
| III. Fidelidad y reserva | Un original íntegro y solo material permitido en el navegador. | Hito 2 exige inventario exacto, cuatro estados del brazalete y revisión conjunta de archivos y tráfico en E07 y E10. |
| IV. Memoria y autoridad docente | Tres escenarios, cuatro trabajadores y una sola base. | El intento fija propietario y rúbrica aprobada. La entrega conserva esa versión y sus evidencias. E12, E17 y E22 comprobarán calibración, aislamiento y un único resultado confirmado. |
| V. Arranque y recuperación | Tres servicios permanentes y un preparador temporal. | El backend depende de la base preparada. Ante una caída de inferencia conserva medición, entrega y encuestas. E21 y E22 lo comprueban. |
| VI. Alcance y capacidad | La aceptación inicial es en el computador del profesor. | La guía reserva RR01, E23 y E24 para el posterior estudio de capacidad. El primer resultado de cada perfil será exploratorio. |

## Referencia comprobada en el clon

El 2026-11-25 se leyeron el HTML y los 28 archivos de `recursos/`. La tabla contiene las 29 rutas relativas al directorio de referencia y los SHA-256 de los bytes leídos. Durante el hito 2 se generarán dos inventarios **antes de editar la copia**. Se cotejarán rutas y huellas por igualdad exacta en el mismo clon, incluyendo faltantes y sobrantes. La tabla permite además reconocer una revisión diferente de la referencia. [Directorio de Semana 5](../../docs/referencias/html-semana5/)

<details>
<summary>Ver las 29 rutas y sus huellas SHA-256</summary>

| Ruta relativa | SHA-256 |
| --- | --- |
| `Paciente_Virtual.html` | `ff9dea8958759550c5241ab9f0560c46f2726bf04ba1d46f927fb9febfeee032` |
| `recursos/escenas/dan/espalda.webp` | `2aff840aed10630e613e7a98957822f55c3f7b6b4ccccee55b521f0988e963a4` |
| `recursos/escenas/dan/frente.webp` | `54db9eb0e0412f15ed7189e84aa2fd6ba35737a4425f2a6469dd15a2731b448f` |
| `recursos/escenas/dan/mano.webp` | `90445592cedda08d0ea5413164e54becfbcd8d2ba3bf817bbf1554fd80e44b53` |
| `recursos/escenas/dan/pantorrilla.webp` | `dd3e326fa7fb279da6a7806a765d53908187ca4f51a3e29a3321397c584f4dd0` |
| `recursos/escenas/dan/peso-listo.webp` | `44e298d9bb5d81b95afd46a53699035a2f7e3aa56c0faeb23be1bfa9c90d87fe` |
| `recursos/escenas/dan/peso.webp` | `77c078b1625617de426065caf0d1b92e9a5ca94de659d5a6616a68a2c7dd5d59` |
| `recursos/escenas/dan/presion-colocada.webp` | `3bf0f5e2ac16b6c250e1143c5a3c1f2221ef503b603ddf8146a0cd157abe9b56` |
| `recursos/escenas/dan/presion.webp` | `31154a58c38b9807d04208134b8cfef7e5e9db2df4ad6dba5061fc5bc07d8b07` |
| `recursos/escenas/lr/espalda.webp` | `0b0fffcfc5f9f4f20a87bdbb2a76a336e68e12c53d15c7dbae92c073fbbe5075` |
| `recursos/escenas/lr/frente.webp` | `47346caaffc06c533c946a81b272a80983ddfb9a0fc4f690dbc9ae05f194c6c6` |
| `recursos/escenas/lr/mano.webp` | `96c1808aa12a97cc9a2e481ec9ea7ff4c50ec43d5522a2fbbb981d53297c5298` |
| `recursos/escenas/lr/pantorrilla.webp` | `3ced790115d049801d38bbbc3f365d866f7c2ae65e31a02b0a56a681b7e57f35` |
| `recursos/escenas/lr/peso-listo.webp` | `056931204b5ed5360c45104a12e05ae4549087ccdce72c7f1503b64ea6d3fb93` |
| `recursos/escenas/lr/peso.webp` | `66a4ceb4a3b763e070a13296ab98fb39edf042fbc0e992d9feaccc320605fcf9` |
| `recursos/escenas/lr/presion-colocada.webp` | `b8183e439c2112efac32f0b50d96258829eca180f2e8c85a642d8016837c2c81` |
| `recursos/escenas/lr/presion.webp` | `467a31706e8237da812d92e7298fca99e93d8d917e13069e552458b2f503162f` |
| `recursos/escenas/nannu/espalda.webp` | `2039914306340efa83396fa206aefdec3d7ca6d8c4459f40210a3e7baea3b009` |
| `recursos/escenas/nannu/frente.webp` | `e9e0fc77ecb495b7cad8d135794531321d323a7bdea2a06336c1e00415b4ed0f` |
| `recursos/escenas/nannu/mano.webp` | `2a04f5ad03d91999525fb3f9c98b9c1f6d83ff3c44308cdb7e0e2a8246532010` |
| `recursos/escenas/nannu/pantorrilla.webp` | `9d12d3f0a286a639bcfb080478943eada89adb23e0b814a0a9a4386eec73a59b` |
| `recursos/escenas/nannu/peso-listo.webp` | `5c558e38abb1fac8b0cae5ea2b9c3b6b56d52d6aa32817957ab59e96f22867e7` |
| `recursos/escenas/nannu/peso.webp` | `854a5dfff882e650044f000f1acb13be99fd0772b674d7bd780c5648b1501118` |
| `recursos/escenas/nannu/presion-colocada.webp` | `e02b2d2526388f590820b5c11371259acfe444abe1491c4e399a6e5991fea715` |
| `recursos/escenas/nannu/presion.webp` | `990950dcbcbb4a5680724e4ee2791af96c86c89d0d14018fcbcb70b314afc087` |
| `recursos/instrumentos/fonendoscopio.webp` | `85948f274da56b17791671b2b4d6fc794b9d7af5dfb23336265ac049c28ac16b` |
| `recursos/instrumentos/palpacion.webp` | `794d660e58f03ec060cd9cb02678a28da0ef3faee181dc34b091cf91e8d2c0b9` |
| `recursos/instrumentos/pulsioximetro.webp` | `f0b829ed85922986c7abac0290c32262f8ed8b6846108a6d897f0b9e9efd059c` |
| `recursos/instrumentos/termometro.webp` | `46adc492df369620cf056a3da96ec00a6230b8878385239f5d274a362c6e3328` |

</details>
