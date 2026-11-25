# Especificación de requerimientos del paciente virtual en SIRCLE

Universidad de La Frontera · Práctica en el Centro de Informática Biomédica

Versión 2.0 · 30 de octubre de 2026 · Propuesta para revisión con el profesor y el equipo de SIRCLE.

## Control de cambios

| Versión | Fecha | Cambio | Responsable |
| --- | --- | --- | --- |
| 1.0 | 30 de octubre de 2026 | Definición inicial del módulo y sus funciones. | Joaquín |
| 2.0 | 30 de octubre de 2026 | Especificación individual de casos de uso, requerimientos y pruebas; incorporación de tablas de usuarios y categorías técnicas; revisión de relaciones UML. | Joaquín |



## Índice

| Sección | Contenido |
| --- | --- |
| 1 | Introducción, propósito, alcance, contexto y definiciones |
| 2 | Descripción general, usuarios, ambiente y modelos |
| 2.5.2 | Especificaciones de los nueve casos principales y siete casos de apoyo o extensión |
| 3.1 | Funcionalidades esperadas, calidad y restricciones del usuario |
| 3.2 | Requerimientos funcionales y categorías técnicas |
| 3.3 | Trazabilidad entre necesidades, casos y software |
| 4 | Casos de prueba, trazabilidad y evaluación educativa |
| 5 | Referencias públicas |



## 1 Introducción

El proyecto propone incorporar a SIRCLE un módulo educativo de paciente virtual para estudiantes de pregrado del área de la salud de la Universidad de La Frontera. El estudiante podrá seleccionar un caso, entrevistar al paciente o cuidador, consultar hallazgos, registrar sus decisiones y observar la evolución del mismo paciente durante varios encuentros. El docente podrá preparar actividades y revisar el desempeño de los estudiantes a su cargo.

El objetivo es diseñar y evaluar este entorno de práctica para ejercitar el razonamiento clínico, la comunicación y el seguimiento. SIRCLE aporta la plataforma de registro; el nuevo módulo relacionará ese trabajo con una simulación que responda a las decisiones del estudiante y al tiempo transcurrido dentro del caso.

### 1.1 Propósito del sistema

Ofrecer un espacio controlado donde el estudiante aplique conocimientos, ensaye decisiones y reciba retroalimentación fundamentada antes de enfrentarse a situaciones semejantes en su formación práctica. La experiencia deberá permitir equivocarse, reconocer omisiones y volver a practicar. Su evaluación considerará la confianza clínica declarada, el desempeño observado y la participación en las actividades.

### 1.2 Alcance del proyecto

El módulo, denominado provisionalmente "Paciente virtual", se incorporará como una sección de SIRCLE. Estará orientado a distintas carreras y niveles del área de la salud, con casos y acciones adecuados al rol que desempeñe el estudiante en cada actividad. La disponibilidad para una carrera dependerá de que existan contenidos y criterios de evaluación revisados por sus docentes.

El alcance incluye la selección y asignación de casos, la entrevista, la valoración y solicitud de exámenes, el registro clínico, las intervenciones simuladas, el avance del tiempo, la continuidad entre sesiones y la evaluación. También comprende la vista docente y la habilitación de accesos por un administrador.

Se reutilizarán el ingreso y los componentes de registro de SIRCLE que resulten compatibles. El desarrollo no incluye una plataforma institucional nueva, una administración general de cuentas independiente ni un manual completo de SIRCLE. Tampoco pretende sustituir la práctica supervisada, atender pacientes reales o simular cualquier enfermedad y respuesta fisiológica. Cada caso tendrá posibilidades de actuación definidas y revisadas.

### 1.3 Contexto del proyecto

La UFRO identifica SIRCLE como una plataforma docente para la Simulación de Registros Clínicos Electrónicos en su [convocatoria de proyectos de docencia de 2025](https://docencia.ufro.cl/convocatoria-2025/). Su funcionamiento mostrado incluye acceso con credenciales, ingreso de pacientes ficticios, formularios clínicos, cálculos asociados a registros, anotaciones cronológicas y finalización de la ficha.

En ese flujo, el estudiante completa información de un caso clínico. El módulo propuesto incorporará una fuente de respuestas y hallazgos que evolucionen durante la actividad. Registrar un valor, una indicación o una observación deberá conservar su significado: escribir una intervención en una nota no equivale por sí solo a ejecutarla sobre el paciente simulado.

i-Human Patients es una referencia para el recorrido de entrevista, examen, razonamiento y retroalimentación. Su oferta abarca Enfermería, Medicina y otros programas de salud. La referencia orienta funciones educativas; no determina la tecnología que utilizará SIRCLE. [Descripción oficial de i-Human](https://kaplan.com/institutions-educators/higher-education/nursing-schools/i-human).

### 1.4 Definiciones

| Término | Significado en el proyecto |
| --- | --- |
| Caso clínico | Situación educativa con antecedentes, objetivos, hallazgos, acciones permitidas y criterios de evaluación. |
| Intento | Ejecución individual de un caso. Tiene su propio paciente simulado, historial y resultado. |
| Encuentro | Atención o reevaluación del mismo paciente dentro de un intento. |
| Tiempo simulado | Tiempo clínico del caso. Puede avanzar por decisión del estudiante sin esperar ese plazo en la vida real. |
| Rúbrica | Criterios y ponderaciones que permiten valorar las acciones, el razonamiento y la documentación. |
| Confianza clínica | Seguridad que el estudiante declara tener sobre sus conocimientos y decisiones. |
| Competencia clínica | Desempeño demostrado al integrar conocimientos y habilidades; en este proyecto se observa dentro de la simulación. |
| Compromiso estudiantil | Participación y continuidad en el aprendizaje, descritas mediante uso, finalización y opinión del estudiante. |

### 1.5 Referencias

Las referencias bibliográficas y enlaces públicos se reúnen al final, en la sección 5. Las funciones propuestas requieren validación académica y técnica; las investigaciones citadas no constituyen una aprobación de estos requerimientos.



## 2 Descripción general

### 2.1 Características de los usuarios

| Tipo de usuario | Descripción y conocimientos | Cantidad actual | Proyección a un año | Población prevista |
| --- | --- | --- | --- | --- |
| Estudiante | Estudiante de pregrado del área de la salud. Desarrolla casos acordes a su carrera y nivel; consulta únicamente sus registros. | Por confirmar | Por confirmar | Estudiantes UFRO incluidos en el piloto. |
| Docente | Profesional responsable de las actividades y de la revisión clínica. Prepara casos, supervisa grupos y evalúa evidencias. | Por confirmar | Por confirmar | Docentes responsables de los grupos participantes. |
| Administrador | Responsable de habilitar el módulo y gestionar permisos y relaciones docente-grupo. Requiere conocer la gestión de SIRCLE. | Por confirmar | Por confirmar | Personal que designe el equipo de SIRCLE. |



Las cantidades se fijarán con el equipo docente. Las cuentas usadas en las pruebas son datos de ensayo y no representan una matrícula real. Una persona podrá desempeñar varios roles cuando tenga autorización; el rol administrativo no concede por sí solo acceso a calificaciones.

![Interacción de estudiantes, docentes y administradores con los módulos](<diagramas/v2/usuarios_modulos.png>)

| Módulo | Estudiante | Docente | Administrador |
| --- | --- | --- | --- |
| Acceso mediante SIRCLE | Sí | Sí | Sí |
| Actividades, paciente y ficha del intento | Propios | Consulta para supervisión | Sin acceso académico automático |
| Evaluación y retroalimentación | Consulta de lo publicado | Revisión y publicación del grupo | Sin facultad de calificación automática |
| Preparación de casos | No | Con permiso de preparación | Habilita permisos |
| Grupos y accesos | No | Consulta de sus grupos | Gestión autorizada |



### 2.2 Perspectiva de los usuarios y clientes del producto

| Usuario | Visión del producto |
| --- | --- |
| Estudiante | Espera reconocer el objetivo del caso, entrevistar y valorar al paciente, tomar decisiones y retomar el trabajo. Necesita saber qué se guardó, qué acciones ejecutó y cómo se fundamenta su evaluación. La repetición debe conservar los intentos anteriores. |
| Docente | Espera preparar actividades adecuadas a la formación, observar la secuencia de decisiones y detectar omisiones. Necesita consultar solo sus grupos, revisar criterios de evaluación y entregar comentarios que el estudiante pueda relacionar con su desempeño. |
| Administrador | Espera habilitar o retirar accesos sin duplicar las cuentas de SIRCLE, asignar responsabilidades de grupo y conocer qué cambio realizó cada responsable. Necesita que la incorporación del módulo conserve las funciones de la plataforma. |



La confianza declarada en una encuesta y el desempeño observado se evaluarán por separado. La participación y el tiempo activo aportarán información sobre el uso de la actividad, sin convertirse por sí solos en una medida de competencia. [Estudio sobre calibración del razonamiento clínico](https://pmc.ncbi.nlm.nih.gov/articles/PMC6775028/).

### 2.3 Ambiente operacional del sistema

El usuario trabajará desde un navegador y accederá mediante SIRCLE. La integración deberá transmitir una identidad verificable y los permisos aplicables, sin pedir que el estudiante entregue su contraseña al módulo. El identificador de acceso seguirá el mecanismo vigente de SIRCLE; el uso obligatorio del correo electrónico queda sujeto a su confirmación.

| Componente | Situación de partida | Trabajo del proyecto |
| --- | --- | --- |
| Acceso | Se muestra ingreso con usuario y contraseña. | Reutilizar la sesión y comprobar autorización para el módulo. |
| Ficha de paciente ficticio | Se registran identidad, antecedentes y datos clínicos. | Vincular cada ficha con un intento y distinguir datos del caso de anotaciones del estudiante. |
| Formularios clínicos | Hay signos vitales, indicaciones, exámenes, valoración y evolución escrita. | Aprovechar los campos pertinentes y definir qué registro representa una acción ejecutada. |
| Cálculos y cierre | Se muestran cálculos de apoyo, revisión de secciones y almacenamiento final. | Mantener su funcionamiento y separar el cierre de la ficha del cierre de la actividad evaluada. |
| Simulación y evaluación | Su implementación no queda establecida por la demostración. | Incorporar conversación, evolución, continuidad y evaluación del aprendizaje. |

La conexión con la base de datos y los servicios de SIRCLE deberá acordarse con su equipo técnico. Todavía deben definirse los campos compartidos, la gestión de permisos y la forma de conservar varios intentos. No se fija un lenguaje, framework, proveedor de IA ni infraestructura sin esa revisión. El componente de conversación podrá ejecutarse en un servicio separado; su alojamiento se resolverá durante el diseño técnico.

### 2.4 Relación con otros proyectos

El desarrollo depende de la evolución de SIRCLE y de la coordinación con el equipo responsable de sus registros. Las modificaciones deberán probarse junto con los flujos existentes para evitar que afecten otras actividades docentes. i-Human, Body Interact y CyberPatient son referencias de mercado, sin que se plantee una conexión técnica con esos productos.



### 2.5 Descripción del modelo

El modelo conserva nueve objetivos principales del usuario. Los casos CU10 a CU16 describen operaciones compartidas o extensiones de esos objetivos; permiten precisar las relaciones UML sin convertir cada botón en una función independiente.

Una relación `<<include>>` incorpora un comportamiento requerido por el flujo. Una relación `<<extend>>` apunta desde el comportamiento opcional hacia el caso base e identifica su condición y punto de extensión. La autenticación se realiza en SIRCLE; las operaciones protegidas comprueban después la sesión y sus permisos. [Notación UML de Visual Paradigm](https://www.visual-paradigm.com/support/documents/vpuserguide/94/2575/84257_usecasediagr.html).

#### 2.5.1 Catálogo y relaciones de casos de uso

| ID | Caso de uso | Tipo |
| --- | --- | --- |
| CU01 | Acceder al módulo desde SIRCLE | Principal; acceso integrado |
| CU02 | Administrar accesos y grupos | Principal; administración |
| CU03 | Preparar y asignar una actividad | Principal; gestión académica |
| CU04 | Iniciar o retomar un caso | Principal; práctica individual |
| CU05 | Valorar al paciente virtual | Principal; valoración |
| CU06 | Registrar decisiones y realizar seguimiento | Principal; razonamiento y actuación |
| CU07 | Finalizar el intento y consultar su devolución | Principal; entrega académica |
| CU08 | Supervisar el aprendizaje del grupo | Principal; seguimiento académico |
| CU09 | Responder encuestas de la experiencia | Principal; evaluación educativa |
| CU10 | Verificar sesión y autorización | Incluido; servicio común |
| CU11 | Conservar el avance del intento | Incluido; persistencia |
| CU12 | Repetir un caso en un intento nuevo | Extensión de CU04 |
| CU13 | Solicitar y consultar un examen | Extensión de CU05 |
| CU14 | Avanzar el tiempo simulado | Extensión de CU06 |
| CU15 | Evaluar un intento entregado | Incluido por CU07; evaluación |
| CU16 | Completar o rectificar una evaluación | Extensión de CU08 |



![UML de acceso integrado y administración de permisos](<diagramas/v2/uml_acceso.png>)

![UML de práctica clínica con inclusiones y extensiones](<diagramas/v2/uml_practica.png>)

![UML de preparación y supervisión docente](<diagramas/v2/uml_docencia.png>)

Las vistas agrupan las funciones por responsabilidad. Las fichas individuales muestran también las verificaciones y el guardado compartidos. CU10 se aplica antes de cada operación protegida, aunque la sesión siga abierta.

| Origen | Relación | Destino | Condición o propósito |
| --- | --- | --- | --- |
| CU01 a CU09; CU12, CU13, CU14 y CU16 | <<include>> | CU10 | Comprobación de la sesión y del permiso de cada operación protegida. |
| CU04, CU05, CU06, CU07, CU12, CU13 y CU14 | <<include>> | CU11 | Conservación del cambio producido por el flujo, incluido el inicio o la reanudación de un intento. |
| CU07 | <<include>> | CU15 | Evaluar una entrega confirmada, con resultado completo o pendiente. |
| CU12 | <<extend>> | CU04 | PE04: el estudiante decide repetir y la actividad permite otro intento. |
| CU13 | <<extend>> | CU05 | PE05: el estudiante solicita o consulta una prueba complementaria. |
| CU14 | <<extend>> | CU06 | PE06: el estudiante confirma un avance temporal permitido. |
| CU16 | <<extend>> | CU08 | PE08: el docente revisa un criterio pendiente o una discrepancia justificada. |



#### 2.5.2 Especificación de los casos de uso

Las fichas distinguen alternativas válidas de errores que impiden continuar. Las postcondiciones describen el resultado del flujo completado. Si una excepción impide terminar, se conserva el último estado confirmado y se informa qué operación quedó pendiente. Los casos se proponen para validación; sus pruebas aún no se han ejecutado.

##### CU01 Acceder al módulo desde SIRCLE

| CU01 · Campo | Especificación |
| --- | --- |
| Nombre | Acceder al módulo desde SIRCLE |
| Versión y fecha | 2.0 · 30 de octubre de 2026 |
| Autor | Joaquín; revisión del profesor y equipo de SIRCLE pendiente. |
| Descripción | Permite abrir el módulo con la identidad validada por SIRCLE y acceder a las funciones autorizadas para el usuario. |
| Actores | Estudiante, docente o administrador; SIRCLE valida la identidad. |
| Tipo | Principal; acceso integrado |
| Referencias | RF01; pruebas PR01, PR02, PR03. |
| Disparador | El usuario abre Paciente virtual desde SIRCLE. |
| Precondición | Existe una cuenta institucional habilitada. La entrada al módulo está disponible en el ambiente configurado. |
| Postcondición | El usuario ve la sección correspondiente a su rol y a los grupos autorizados. No se crea una cuenta ni contraseña adicional. |
| Flujo principal | 1. El usuario ingresa a SIRCLE con las credenciales que utiliza la plataforma.<br>2. SIRCLE valida la cuenta y establece la sesión.<br>3. El usuario selecciona Paciente virtual.<br>4. El módulo ejecuta CU10 para comprobar la sesión y los permisos.<br>5. El sistema muestra actividades propias, supervisión docente o administración, según la autorización. |
| Subflujos y relaciones | Incluye CU10. |
| Flujos alternativos | A1, paso 1: si ya existe una sesión vigente, se continúa en el paso 3.<br>A2, paso 5: si la persona tiene varios roles, selecciona una vista habilitada; la selección no modifica sus permisos. |
| Excepciones y respuesta | E1, paso 2: cuenta inexistente o contraseña incorrecta. SIRCLE rechaza el ingreso y permite reintentar; se utiliza su recuperación o soporte disponible.<br>E2, paso 4: sesión vencida. Se solicita volver a ingresar, conservando el avance previamente confirmado.<br>E3, paso 4: falta de autorización o servicio de identidad no disponible. El módulo deniega el acceso; no muestra datos protegidos. |
| Puntos de extensión | No define puntos de extensión adicionales. |
| Casos de prueba | PR01, PR02, PR03. Las condiciones y resultados verificables se detallan en 4.1. |
| Diagrama | ![Diagrama de CU01 Acceder al módulo desde SIRCLE](<diagramas/v2/cu01.png>) |



##### CU02 Administrar accesos y grupos

| CU02 · Campo | Especificación |
| --- | --- |
| Nombre | Administrar accesos y grupos |
| Versión y fecha | 2.0 · 30 de octubre de 2026 |
| Autor | Joaquín; revisión del profesor y equipo de SIRCLE pendiente. |
| Descripción | Permite gestionar quién puede utilizar el módulo y qué grupos puede supervisar cada docente, a partir de las cuentas existentes. |
| Actores | Administrador; SIRCLE proporciona cuentas y grupos. |
| Tipo | Principal; administración |
| Referencias | RF02; pruebas PR03. |
| Disparador | El administrador solicita habilitar, modificar o revocar el acceso de una cuenta existente. |
| Precondición | Administrador autenticado con permiso de gestión. La cuenta y el grupo de destino existen en SIRCLE. |
| Postcondición | El permiso y la relación docente-grupo quedan actualizados, con autor y fecha. El historial académico se conserva. |
| Flujo principal | 1. El administrador abre la gestión del módulo; el sistema ejecuta CU10.<br>2. Busca la cuenta y consulta sus permisos y grupos actuales.<br>3. Selecciona el cambio de acceso o la relación docente-grupo.<br>4. El sistema valida la combinación y muestra el cambio antes de aplicarlo.<br>5. El administrador confirma.<br>6. El sistema guarda el cambio y su registro de auditoría; informa el resultado. |
| Subflujos y relaciones | Incluye CU10. |
| Flujos alternativos | A1, paso 3: revocar un permiso conserva los registros y debe impedir la siguiente operación protegida.<br>A2, paso 5: cancelar conserva la configuración anterior. |
| Excepciones y respuesta | E1, paso 2: usuario o grupo inexistente. Se pide corregir la selección.<br>E2, paso 4: el actor perdió el permiso administrativo. Se rechaza el cambio.<br>E3, paso 6: fallo de almacenamiento o modificación concurrente. No se informa éxito ni se dejan relaciones parciales. |
| Puntos de extensión | No define puntos de extensión adicionales. |
| Casos de prueba | PR03. Las condiciones y resultados verificables se detallan en 4.1. |
| Diagrama | ![Diagrama de CU02 Administrar accesos y grupos](<diagramas/v2/cu02.png>) |



##### CU03 Preparar y asignar una actividad

| CU03 · Campo | Especificación |
| --- | --- |
| Nombre | Preparar y asignar una actividad |
| Versión y fecha | 2.0 · 30 de octubre de 2026 |
| Autor | Joaquín; revisión del profesor y equipo de SIRCLE pendiente. |
| Descripción | Permite preparar el contenido clínico y las condiciones de una actividad antes de ponerla a disposición de un grupo. |
| Actores | Docente con permiso de preparación y asignación. |
| Tipo | Principal; gestión académica |
| Referencias | RF03; pruebas PR04, PR26. |
| Disparador | El docente prepara una actividad para un grupo a su cargo. |
| Precondición | Docente autenticado, grupo autorizado y acceso al catálogo. Para publicar debe existir una revisión clínica registrada. |
| Postcondición | Una versión revisada del caso y su rúbrica quedan asignadas al grupo. Los intentos ya iniciados mantienen su versión anterior. |
| Flujo principal | 1. El docente abre la preparación de actividades y el sistema ejecuta CU10.<br>2. Selecciona un caso revisado o prepara un borrador con identidad ficticia, objetivos, nivel y antecedentes.<br>3. Completa acciones admitidas, reglas de evolución y rúbrica; registra la revisión clínica correspondiente.<br>4. Selecciona el grupo, oportunidades de práctica y momento de publicación de la retroalimentación.<br>5. El sistema verifica integridad, revisión y relación con el grupo.<br>6. El docente confirma la publicación y asignación.<br>7. El sistema guarda la versión y habilita la actividad para los destinatarios. |
| Subflujos y relaciones | Incluye CU10. |
| Flujos alternativos | A1, paso 3: guardar como borrador permite continuar la preparación; no habilita intentos de estudiantes.<br>A2, paso 2: usar un caso revisado permite conservar sus reglas y rúbrica.<br>A3, paso 3: la IA puede ayudar a redactar contenido; el docente debe revisar el resultado antes de publicarlo. |
| Excepciones y respuesta | E1, paso 5: faltan datos, revisión o reglas consistentes. Se señalan los campos y se mantiene el borrador.<br>E2, paso 5: grupo ajeno o inexistente. Se rechaza la asignación.<br>E3, paso 6: se modifica un caso con intentos previos. Se exige otra versión; no se reemplaza la que esos intentos utilizaron. |
| Puntos de extensión | No define puntos de extensión adicionales. |
| Casos de prueba | PR04, PR26. Las condiciones y resultados verificables se detallan en 4.1. |
| Diagrama | ![Diagrama de CU03 Preparar y asignar una actividad](<diagramas/v2/cu03.png>) |



##### CU04 Iniciar o retomar un caso

| CU04 · Campo | Especificación |
| --- | --- |
| Nombre | Iniciar o retomar un caso |
| Versión y fecha | 2.0 · 30 de octubre de 2026 |
| Autor | Joaquín; revisión del profesor y equipo de SIRCLE pendiente. |
| Descripción | Permite comenzar una actividad asignada o continuar la atención del mismo paciente desde el último avance confirmado. |
| Actores | Estudiante. |
| Tipo | Principal; práctica individual |
| Referencias | RF04; pruebas PR05, PR06. |
| Disparador | El estudiante selecciona una actividad de su listado. |
| Precondición | Cuenta autenticada y actividad asignada. Para continuar debe existir un intento propio y recuperable. |
| Postcondición | Queda abierto un intento identificado, con la ficha correcta, estado clínico y reloj simulado. Otros intentos permanecen intactos. |
| Flujo principal | 1. El sistema ejecuta CU10 y muestra las actividades autorizadas.<br>2. El estudiante selecciona una actividad y consulta sus objetivos y antecedentes iniciales.<br>3. El sistema muestra si existe un intento en curso y las oportunidades disponibles.<br>4. El estudiante elige continuar el intento o iniciar el primero.<br>5. El sistema recupera el intento o crea uno nuevo con su versión de caso y ficha vinculada.<br>6. Ejecuta CU11 para registrar el inicio o la reanudación; retomar conserva el reloj y el estado clínico confirmado.<br>7. Muestra el paciente, la cronología y el estado de guardado. |
| Subflujos y relaciones | Incluye CU10, CU11. |
| Flujos alternativos | A1, paso 3, PE04: si ya existe un intento y el estudiante decide repetir, se ejecuta CU12 y se continúa en el paso 7 con el nuevo intento.<br>A2, paso 4: abrir un intento entregado permite consultar su registro; no permite modificarlo. |
| Excepciones y respuesta | E1, paso 1: actividad no asignada o intento ajeno. Se deniega el acceso.<br>E2, paso 5: límite de intentos o fallo de creación. Se mantiene el listado y se informa la causa.<br>E3, paso 5: doble clic o reenvío de una solicitud confirmada. Se devuelve el mismo intento creado, sin duplicarlo. |
| Puntos de extensión | PE04: tras consultar los intentos, repetir el caso cuando el usuario lo solicita y la actividad lo permite. |
| Casos de prueba | PR05, PR06. Las condiciones y resultados verificables se detallan en 4.1. |
| Diagrama | ![Diagrama de CU04 Iniciar o retomar un caso](<diagramas/v2/cu04.png>) |



##### CU05 Valorar al paciente virtual

| CU05 · Campo | Especificación |
| --- | --- |
| Nombre | Valorar al paciente virtual |
| Versión y fecha | 2.0 · 30 de octubre de 2026 |
| Autor | Joaquín; revisión del profesor y equipo de SIRCLE pendiente. |
| Descripción | Permite obtener antecedentes y hallazgos mediante la entrevista y las maniobras de valoración admitidas por el caso. |
| Actores | Estudiante; servicio de conversación cuando se utilice. |
| Tipo | Principal; valoración |
| Referencias | RF05, RF06; pruebas PR07, PR08. |
| Disparador | El estudiante decide recoger antecedentes o hallazgos del paciente. |
| Precondición | Intento propio en curso. El caso contiene respuestas y maniobras disponibles para el rol del estudiante. |
| Postcondición | Se muestran los hallazgos obtenidos y queda evidencia de la valoración. Las anotaciones del estudiante no sustituyen el estado del paciente. |
| Flujo principal | 1. El sistema ejecuta CU10 y recupera el contexto vigente del intento.<br>2. El estudiante pregunta al paciente o cuidador, o selecciona una maniobra de valoración.<br>3. El sistema determina la información disponible y la respuesta correspondiente al estado clínico.<br>4. Presenta la respuesta o el hallazgo con unidad y momento simulado cuando corresponda.<br>5. Ejecuta CU11 para conservar la interacción y su resultado.<br>6. El estudiante revisa lo obtenido y decide si continúa valorando o pasa al razonamiento clínico. |
| Subflujos y relaciones | Incluye CU10, CU11. |
| Flujos alternativos | A1, paso 2: una pregunta ambigua solicita aclaración antes de responder.<br>A2, paso 6, PE05: si necesita pruebas complementarias, ejecuta CU13 y vuelve a la valoración con el estado de la orden o el resultado disponible. |
| Excepciones y respuesta | E1, paso 3: dato o maniobra ausente. Se informa que no está disponible; no se inventa un valor normal.<br>E2, paso 3: servicio de diálogo interrumpido. Se permite reintentar sin cambiar antecedentes ni resultados.<br>E3, paso 2: una instrucción intenta cambiar el caso o revelar la rúbrica reservada. Se conserva el contexto autorizado y no se entrega esa información. |
| Puntos de extensión | PE05: al decidir si se requiere información adicional, solicitar o consultar un examen habilitado. |
| Casos de prueba | PR07, PR08. Las condiciones y resultados verificables se detallan en 4.1. |
| Diagrama | ![Diagrama de CU05 Valorar al paciente virtual](<diagramas/v2/cu05.png>) |



##### CU06 Registrar decisiones y realizar seguimiento

| CU06 · Campo | Especificación |
| --- | --- |
| Nombre | Registrar decisiones y realizar seguimiento |
| Versión y fecha | 2.0 · 30 de octubre de 2026 |
| Autor | Joaquín; revisión del profesor y equipo de SIRCLE pendiente. |
| Descripción | Permite documentar el razonamiento, ejecutar intervenciones representadas por el caso y revisar sus consecuencias. |
| Actores | Estudiante. |
| Tipo | Principal; razonamiento y actuación |
| Referencias | RF07, RF08; pruebas PR09, PR10, PR11. |
| Disparador | El estudiante registra una hipótesis, un plan o una intervención para el paciente. |
| Precondición | Intento propio en curso y estado vigente. Las acciones representadas y sus parámetros están definidos por el caso. |
| Postcondición | El razonamiento queda documentado. Las intervenciones confirmadas se registran una sola vez y el estado refleja sus efectos previstos. |
| Flujo principal | 1. El sistema ejecuta CU10 y presenta los datos actuales y los antecedentes del intento.<br>2. El estudiante registra diagnóstico o hipótesis, justificación y plan.<br>3. Si decide ejecutar una intervención, la selecciona e introduce los parámetros exigidos.<br>4. El sistema distingue lo propuesto de lo ejecutado y solicita confirmación de la intervención.<br>5. Tras la confirmación, valida la solicitud y aplica los efectos previstos para ese momento.<br>6. Ejecuta CU11 para conservar la documentación y los eventos.<br>7. El estudiante revisa el estado y decide continuar la atención, reevaluar después o finalizar. |
| Subflujos y relaciones | Incluye CU10, CU11. |
| Flujos alternativos | A1, paso 3: guardar solo el razonamiento conserva una nota, sin ejecutar lo escrito.<br>A2, paso 4: cancelar una intervención conserva la nota y no aplica efectos.<br>A3, paso 7, PE06: al solicitar una reevaluación posterior se ejecuta CU14 y se vuelve al paso 1 con el nuevo estado. |
| Excepciones y respuesta | E1, paso 5: parámetros incompletos o acción ambigua. Se solicita corrección y no se ejecuta.<br>E2, paso 5: acción fuera de las posibilidades del caso. Se informa el límite.<br>E3, paso 5: decisión clínicamente errónea pero contemplada. Se conserva y se aplican sus consecuencias definidas; no se trata como un fallo informático.<br>E4, paso 6: reenvío o fallo de guardado. Se evita duplicar la acción y se informa el estado pendiente. |
| Puntos de extensión | PE06: al decidir la siguiente reevaluación, avanzar un intervalo permitido del tiempo simulado. |
| Casos de prueba | PR09, PR10, PR11. Las condiciones y resultados verificables se detallan en 4.1. |
| Diagrama | ![Diagrama de CU06 Registrar decisiones y realizar seguimiento](<diagramas/v2/cu06.png>) |



##### CU07 Finalizar el intento y consultar su devolución

| CU07 · Campo | Especificación |
| --- | --- |
| Nombre | Finalizar el intento y consultar su devolución |
| Versión y fecha | 2.0 · 30 de octubre de 2026 |
| Autor | Joaquín; revisión del profesor y equipo de SIRCLE pendiente. |
| Descripción | Permite entregar el trabajo y consultar la evaluación cuando esté disponible para el estudiante. |
| Actores | Estudiante. |
| Tipo | Principal; entrega académica |
| Referencias | RF09; pruebas PR12, PR13. |
| Disparador | El estudiante elige entregar su intento. |
| Precondición | Intento propio en curso; criterios y versión de rúbrica asociados. La actividad define cuándo se publica la retroalimentación. |
| Postcondición | La entrega queda registrada y protegida contra cambios del estudiante. Se muestra la evaluación publicada o su estado pendiente. |
| Flujo principal | 1. El sistema ejecuta CU10 y muestra un resumen de la entrega.<br>2. Comprueba los campos documentales obligatorios e informa omisiones.<br>3. El estudiante confirma que desea finalizar.<br>4. CU11 conserva la entrega y su versión de evidencia.<br>5. El sistema ejecuta CU15 para evaluar el intento.<br>6. Muestra confirmación de entrega y, según su estado, resultado publicado o evaluación pendiente.<br>7. El estudiante consulta posteriormente la devolución disponible. |
| Subflujos y relaciones | Incluye CU10, CU11, CU15. |
| Flujos alternativos | A1, paso 3: cancelar permite continuar trabajando.<br>A2, paso 2: omitir una acción clínica evaluable no impide entregar; se registra para aplicar la rúbrica.<br>A3, paso 6: si el docente aún no publica los resultados, se informa esa condición sin mostrar respuestas reservadas. |
| Excepciones y respuesta | E1, paso 2: falta documentación obligatoria. Se señalan los campos y no se finaliza.<br>E2, paso 4: fallo al guardar. El intento no aparece entregado hasta confirmar la operación.<br>E3, paso 5: fallo del evaluador. La entrega se conserva y la evaluación queda pendiente, sin asignar cero.<br>E4, paso 3: confirmación repetida. Se devuelve la entrega existente. |
| Puntos de extensión | No define puntos de extensión adicionales. |
| Casos de prueba | PR12, PR13. Las condiciones y resultados verificables se detallan en 4.1. |
| Diagrama | ![Diagrama de CU07 Finalizar el intento y consultar su devolución](<diagramas/v2/cu07.png>) |



##### CU08 Supervisar el aprendizaje del grupo

| CU08 · Campo | Especificación |
| --- | --- |
| Nombre | Supervisar el aprendizaje del grupo |
| Versión y fecha | 2.0 · 30 de octubre de 2026 |
| Autor | Joaquín; revisión del profesor y equipo de SIRCLE pendiente. |
| Descripción | Permite al docente revisar la participación y las evidencias de sus estudiantes, añadir comentarios y publicar la devolución. |
| Actores | Docente. |
| Tipo | Principal; seguimiento académico |
| Referencias | RF10; pruebas PR03, PR13, PR14. |
| Disparador | El docente abre los resultados de una actividad a su cargo. |
| Precondición | Docente autenticado y vinculado al grupo. Existen actividades asignadas; pueden tener o no registros de estudiantes. |
| Postcondición | Se muestran resultados y evidencias autorizadas. Las publicaciones y comentarios confirmados quedan registrados. |
| Flujo principal | 1. El sistema ejecuta CU10 y muestra grupos y actividades del docente.<br>2. El docente selecciona filtros y consulta estudiantes, intentos y entregas.<br>3. Abre un intento para revisar cronología, decisiones y resultados por criterio.<br>4. Examina la evidencia y añade comentarios cuando corresponde.<br>5. Publica la retroalimentación de las entregas que están listas, según la política de la actividad.<br>6. El sistema guarda la publicación y actualiza la vista de resultados. |
| Subflujos y relaciones | Incluye CU10. |
| Flujos alternativos | A1, paso 2: si no hay registros, se muestra esa condición y no un promedio igual a cero.<br>A2, paso 3, PE08: si un criterio requiere revisión o existe una discrepancia fundada, se ejecuta CU16; después se vuelve al detalle del intento.<br>A3, paso 2: las evaluaciones pendientes se muestran aparte de los resultados completos. |
| Excepciones y respuesta | E1, paso 1 o 3: grupo o intento ajeno. Se rechaza el acceso, incluso por dirección directa.<br>E2, paso 5: hay criterios pendientes. Se identifica lo pendiente antes de publicar una evaluación definitiva.<br>E3, paso 6: fallo o modificación concurrente. Se conserva la última publicación confirmada y se pide recargar. |
| Puntos de extensión | PE08: al revisar un criterio, completar o rectificar su evaluación cuando exista justificación y permiso docente. |
| Casos de prueba | PR03, PR13, PR14. Las condiciones y resultados verificables se detallan en 4.1. |
| Diagrama | ![Diagrama de CU08 Supervisar el aprendizaje del grupo](<diagramas/v2/cu08.png>) |



##### CU09 Responder encuestas de la experiencia

| CU09 · Campo | Especificación |
| --- | --- |
| Nombre | Responder encuestas de la experiencia |
| Versión y fecha | 2.0 · 30 de octubre de 2026 |
| Autor | Joaquín; revisión del profesor y equipo de SIRCLE pendiente. |
| Descripción | Permite recoger la opinión y la confianza declarada antes y después de la actividad para estudiar la experiencia educativa. |
| Actores | Estudiante. |
| Tipo | Principal; evaluación educativa |
| Referencias | RF11; pruebas PR15. |
| Disparador | Se habilita el instrumento previo o posterior de una actividad. |
| Precondición | Estudiante autenticado y actividad identificada. El instrumento, su versión y las condiciones de participación están definidos. |
| Postcondición | Se conserva una respuesta por momento y versión, o se registra su ausencia. La encuesta no modifica la nota clínica. |
| Flujo principal | 1. El sistema ejecuta CU10 e identifica la actividad y el momento de respuesta.<br>2. Informa el propósito del instrumento y presenta las preguntas habilitadas.<br>3. El estudiante responde y decide enviar.<br>4. El sistema valida los campos exigidos por el instrumento.<br>5. Guarda las respuestas con versión, actividad y momento real.<br>6. Confirma la recepción y permite continuar con la actividad académica. |
| Subflujos y relaciones | Incluye CU10. |
| Flujos alternativos | A1, paso 3: el estudiante omite la encuesta de investigación; queda como dato faltante y continúa sin penalización.<br>A2, paso 1: una encuesta posterior se presenta al finalizar la actividad, conservando su relación con la previa cuando exista. |
| Excepciones y respuesta | E1, paso 1: se intenta responder la encuesta inicial después de comenzar. No se registra como medición previa.<br>E2, paso 4: respuesta incompleta. Se señalan los campos sin inventar valores.<br>E3, paso 5: reenvío o pérdida de conexión. Se evita duplicar una respuesta ya confirmada y se informa si sigue pendiente. |
| Puntos de extensión | No define puntos de extensión adicionales. |
| Casos de prueba | PR15. Las condiciones y resultados verificables se detallan en 4.1. |
| Diagrama | ![Diagrama de CU09 Responder encuestas de la experiencia](<diagramas/v2/cu09.png>) |



##### CU10 Verificar sesión y autorización

| CU10 · Campo | Especificación |
| --- | --- |
| Nombre | Verificar sesión y autorización |
| Versión y fecha | 2.0 · 30 de octubre de 2026 |
| Autor | Joaquín; revisión del profesor y equipo de SIRCLE pendiente. |
| Descripción | Comprueba la identidad y el permiso sobre el recurso solicitado antes de permitir una operación protegida. |
| Actores | Participación indirecta del usuario del caso invocador; SIRCLE valida la sesión. |
| Tipo | Incluido; servicio común |
| Referencias | RF01, RF02; pruebas PR01, PR02, PR03. |
| Disparador | Un caso de uso necesita acceder a una función o recurso protegido. |
| Precondición | La solicitud identifica la operación y el recurso. SIRCLE dispone del mecanismo acordado para validar la identidad. |
| Postcondición | La operación recibe un contexto autorizado o una denegación. No se expone información del recurso antes de comprobar el permiso. |
| Flujo principal | 1. Recibe la solicitud y la referencia verificable a la sesión.<br>2. Consulta la vigencia de la sesión según el contrato con SIRCLE.<br>3. Comprueba permiso de operación y pertenencia del recurso al estudiante o grupo docente.<br>4. Devuelve el contexto autorizado al caso invocador, o una denegación sin datos protegidos. |
| Subflujos y relaciones | Es incluido por CU01, CU02, CU03, CU04, CU05, CU06, CU07, CU08, CU09, CU12, CU13, CU14, CU16. |
| Flujos alternativos | A1, paso 3: una cuenta con varios roles utiliza únicamente el permiso aplicable al recurso solicitado. |
| Excepciones y respuesta | E1, paso 2: sesión inválida o expirada. Deniega y solicita autenticación.<br>E2, paso 3: permiso revocado o recurso ajeno. Deniega aunque el menú o enlace siga visible.<br>E3, paso 2: no se puede verificar la identidad. Se rechaza la operación protegida. |
| Puntos de extensión | No define puntos de extensión adicionales. |
| Casos de prueba | PR01, PR02, PR03. Las condiciones y resultados verificables se detallan en 4.1. |
| Diagrama | ![Diagrama de CU10 Verificar sesión y autorización](<diagramas/v2/cu10.png>) |



##### CU11 Conservar el avance del intento

| CU11 · Campo | Especificación |
| --- | --- |
| Nombre | Conservar el avance del intento |
| Versión y fecha | 2.0 · 30 de octubre de 2026 |
| Autor | Joaquín; revisión del profesor y equipo de SIRCLE pendiente. |
| Descripción | Confirma los cambios del intento y su evidencia, controlando reenvíos, conflictos y fallos de persistencia. |
| Actores | Participación indirecta del estudiante o proceso autorizado que modifica el intento. |
| Tipo | Incluido; persistencia |
| Referencias | RF04, RF05, RF06, RF07, RF08, RF09; pruebas PR05, PR06, PR09, PR11, PR16. |
| Disparador | El caso invocador necesita confirmar una creación, interacción, acción o entrega. |
| Precondición | La operación ya está autorizada. Se conocen el intento, la versión esperada y un identificador de solicitud. |
| Postcondición | El cambio y su evidencia quedan confirmados una sola vez. Si no se completa, se mantiene el último estado confirmado y se informa el resultado al invocador. |
| Flujo principal | 1. Recibe los datos del cambio, el intento y la identificación de la solicitud.<br>2. Comprueba si la operación ya se confirmó y si la versión del intento sigue vigente.<br>3. Valida consistencia y registra juntos los datos que deben cambiar como una unidad.<br>4. Actualiza la relación con la ficha según el contrato de SIRCLE.<br>5. Devuelve confirmación y la nueva versión; la interfaz informa que el avance quedó guardado. |
| Subflujos y relaciones | Es incluido por CU04, CU05, CU06, CU07, CU12, CU13, CU14. |
| Flujos alternativos | A1, paso 2: una solicitud repetida devuelve el resultado confirmado anteriormente. |
| Excepciones y respuesta | E1, paso 2: versión antigua. Se exige recuperar el estado vigente antes de volver a modificar.<br>E2, pasos 3 o 4: falla la persistencia o sincronización. Se conserva un estado pendiente o se revierte la operación según el contrato, sin afirmar que ambos registros están sincronizados.<br>E3: el usuario sale antes de confirmar. Solo se garantiza recuperar lo previamente confirmado. |
| Puntos de extensión | No define puntos de extensión adicionales. |
| Casos de prueba | PR05, PR06, PR09, PR11, PR16. Las condiciones y resultados verificables se detallan en 4.1. |
| Diagrama | ![Diagrama de CU11 Conservar el avance del intento](<diagramas/v2/cu11.png>) |



##### CU12 Repetir un caso en un intento nuevo

| CU12 · Campo | Especificación |
| --- | --- |
| Nombre | Repetir un caso en un intento nuevo |
| Versión y fecha | 2.0 · 30 de octubre de 2026 |
| Autor | Joaquín; revisión del profesor y equipo de SIRCLE pendiente. |
| Descripción | Permite practicar nuevamente una actividad sin reemplazar el trabajo realizado en intentos anteriores. |
| Actores | Estudiante. |
| Tipo | Extensión de CU04 |
| Referencias | RF04; pruebas PR05. |
| Disparador | En PE04, el estudiante elige repetir una actividad que ya desarrolló. |
| Precondición | Existe un intento previo propio y la actividad permite otra ejecución. |
| Postcondición | Se crea un intento independiente, con estado inicial y ficha propia. El intento previo y su evaluación permanecen intactos. |
| Flujo principal | 1. Ejecuta CU10 y verifica las oportunidades disponibles.<br>2. Muestra que se abrirá otro intento y solicita confirmación.<br>3. El estudiante confirma.<br>4. Crea un identificador nuevo a partir de la versión asignada al nuevo intento.<br>5. Ejecuta CU11 y devuelve a CU04 el nuevo contexto. |
| Subflujos y relaciones | Incluye CU10, CU11. Extiende CU04 en PE04: El estudiante pide repetir y la actividad admite otro intento. |
| Flujos alternativos | A1, paso 3: cancelar devuelve al listado sin crear ni borrar intentos. |
| Excepciones y respuesta | E1, paso 1: límite alcanzado o actividad retirada. Se rechaza la repetición.<br>E2, paso 5: reenvío. Devuelve el mismo intento creado para esa solicitud.<br>E3, paso 4: la operación de nuevo paciente de SIRCLE sobrescribiría la ficha anterior. Se bloquea la creación hasta contar con un vínculo independiente. |
| Puntos de extensión | PE04 en CU04 |
| Casos de prueba | PR05. Las condiciones y resultados verificables se detallan en 4.1. |
| Diagrama | ![Diagrama de CU12 Repetir un caso en un intento nuevo](<diagramas/v2/cu12.png>) |



##### CU13 Solicitar y consultar un examen

| CU13 · Campo | Especificación |
| --- | --- |
| Nombre | Solicitar y consultar un examen |
| Versión y fecha | 2.0 · 30 de octubre de 2026 |
| Autor | Joaquín; revisión del profesor y equipo de SIRCLE pendiente. |
| Descripción | Permite pedir pruebas complementarias y consultar su disponibilidad y sus resultados dentro de la valoración. |
| Actores | Estudiante. |
| Tipo | Extensión de CU05 |
| Referencias | RF06; pruebas PR08. |
| Disparador | En PE05, el estudiante necesita un examen complementario. |
| Precondición | Intento en curso y examen habilitado para ese caso. Para consultar un resultado debe existir una orden propia. |
| Postcondición | La orden queda registrada y se muestra como pendiente o disponible según el reloj simulado. No se altera el plazo por volver a consultarla. |
| Flujo principal | 1. Ejecuta CU10 y muestra los exámenes admitidos.<br>2. El estudiante selecciona un examen y confirma la solicitud.<br>3. El sistema registra orden y momento previsto de disponibilidad mediante CU11.<br>4. Muestra la orden y su estado.<br>5. Al consultarla, compara el reloj simulado con el plazo del caso.<br>6. Entrega el resultado con unidades y momento, si corresponde, o indica que sigue pendiente; devuelve el control a CU05. |
| Subflujos y relaciones | Incluye CU10, CU11. Extiende CU05 en PE05: El estudiante solicita o consulta una prueba complementaria. |
| Flujos alternativos | A1, paso 2: consultar una orden existente lleva al paso 5 sin emitir otra.<br>A2, paso 2: una nueva toma deliberada genera otra orden, diferenciada de un reenvío técnico. |
| Excepciones y respuesta | E1, paso 1: examen no representado. Se informa que no está disponible.<br>E2, paso 3: confirmación repetida. Se conserva una sola orden.<br>E3, paso 6: resultado pendiente o contenido que no carga. Se informa el estado y no se muestra un dato normal inventado. |
| Puntos de extensión | PE05 en CU05 |
| Casos de prueba | PR08. Las condiciones y resultados verificables se detallan en 4.1. |
| Diagrama | ![Diagrama de CU13 Solicitar y consultar un examen](<diagramas/v2/cu13.png>) |



##### CU14 Avanzar el tiempo simulado

| CU14 · Campo | Especificación |
| --- | --- |
| Nombre | Avanzar el tiempo simulado |
| Versión y fecha | 2.0 · 30 de octubre de 2026 |
| Autor | Joaquín; revisión del profesor y equipo de SIRCLE pendiente. |
| Descripción | Permite observar la evolución del paciente en un momento posterior sin esperar ese plazo en tiempo real. |
| Actores | Estudiante. |
| Tipo | Extensión de CU06 |
| Referencias | RF08; pruebas PR11. |
| Disparador | En PE06, el estudiante solicita una reevaluación posterior. |
| Precondición | Intento en curso y versión vigente. El caso define intervalos y reglas de evolución revisadas. |
| Postcondición | El reloj, los eventos pendientes y el estado del paciente avanzan de forma consistente. El nuevo encuentro conserva los antecedentes. |
| Flujo principal | 1. Ejecuta CU10 y muestra el momento actual y los intervalos admitidos.<br>2. El estudiante selecciona y confirma el avance.<br>3. El sistema calcula los eventos e intervenciones que corresponden al intervalo.<br>4. Comprueba consistencia de los resultados y del estado resultante.<br>5. Ejecuta CU11 para conservar juntos reloj, eventos y estado.<br>6. Devuelve a CU06 la situación para reevaluar al paciente. |
| Subflujos y relaciones | Incluye CU10, CU11. Extiende CU06 en PE06: El estudiante confirma un avance temporal permitido. |
| Flujos alternativos | A1, paso 2: cancelar conserva el momento actual.<br>A2, paso 3: si se alcanza un desenlace terminal previsto, se informa y se mantiene disponible el cierre académico. |
| Excepciones y respuesta | E1, paso 2: intervalo negativo, cero o fuera del periodo representado. Se rechaza sin modificar datos.<br>E2, paso 4: no existe una evolución definida o consistente. Se informa el límite y se conserva el estado previo.<br>E3, paso 5: fallo o versión desactualizada. No queda el reloj adelantado con datos antiguos. |
| Puntos de extensión | PE06 en CU06 |
| Casos de prueba | PR11. Las condiciones y resultados verificables se detallan en 4.1. |
| Diagrama | ![Diagrama de CU14 Avanzar el tiempo simulado](<diagramas/v2/cu14.png>) |



##### CU15 Evaluar un intento entregado

| CU15 · Campo | Especificación |
| --- | --- |
| Nombre | Evaluar un intento entregado |
| Versión y fecha | 2.0 · 30 de octubre de 2026 |
| Autor | Joaquín; revisión del profesor y equipo de SIRCLE pendiente. |
| Descripción | Relaciona la evidencia de una entrega con los criterios de su rúbrica y distingue resultados calculables de criterios pendientes de revisión. |
| Actores | Proceso de evaluación invocado por una entrega autorizada; docente revisa los criterios que lo requieran. |
| Tipo | Incluido por CU07; evaluación |
| Referencias | RF09; pruebas PR12, PR13. |
| Disparador | CU07 confirma la entrega y solicita su evaluación. |
| Precondición | Entrega persistida y evidencia identificada. La rúbrica corresponde a la versión usada por el intento. |
| Postcondición | Se conserva una evaluación trazable o un estado pendiente. Solo se publica según la política de la actividad. |
| Flujo principal | 1. Recupera la entrega y la rúbrica asociada.<br>2. Relaciona cada criterio con las acciones, omisiones y documentos del intento.<br>3. Calcula los criterios que pueden verificarse y marca los que necesitan revisión docente.<br>4. Prepara una explicación vinculada a la evidencia.<br>5. Guarda puntuaciones, criterios pendientes y versión de evaluación.<br>6. Devuelve a CU07 el estado y el resultado publicable. |
| Subflujos y relaciones | Es incluido por CU07. |
| Flujos alternativos | A1, paso 3: un criterio subjetivo queda pendiente para revisión en CU08 mediante CU16.<br>A2, paso 6: una evaluación completa permanece reservada hasta que el docente la publique. |
| Excepciones y respuesta | E1, paso 1: rúbrica incompatible o evidencia incompleta por fallo técnico. Se marca pendiente sin fabricar una nota.<br>E2, pasos 3 a 5: falla el evaluador. Se conserva la entrega y se permite reintentar.<br>E3: una segunda ejecución no genera dos evaluaciones vigentes para la misma entrega; una revisión posterior queda versionada. |
| Puntos de extensión | No define puntos de extensión adicionales. |
| Casos de prueba | PR12, PR13. Las condiciones y resultados verificables se detallan en 4.1. |
| Diagrama | ![Diagrama de CU15 Evaluar un intento entregado](<diagramas/v2/cu15.png>) |



##### CU16 Completar o rectificar una evaluación

| CU16 · Campo | Especificación |
| --- | --- |
| Nombre | Completar o rectificar una evaluación |
| Versión y fecha | 2.0 · 30 de octubre de 2026 |
| Autor | Joaquín; revisión del profesor y equipo de SIRCLE pendiente. |
| Descripción | Permite al docente resolver un criterio pendiente o corregir una valoración con fundamento en la evidencia conservada. |
| Actores | Docente responsable de la actividad. |
| Tipo | Extensión de CU08 |
| Referencias | RF09, RF10; pruebas PR13. |
| Disparador | En PE08, el docente identifica un criterio pendiente o una discrepancia respaldada por el registro. |
| Precondición | Intento entregado de un grupo autorizado y evaluación identificada. Existe evidencia para fundamentar la revisión. |
| Postcondición | Se conserva una nueva revisión con criterio, valor anterior y nuevo, motivo, docente y fecha. No se modifica la evidencia original del estudiante. |
| Flujo principal | 1. Ejecuta CU10 y recupera la evaluación vigente.<br>2. El docente consulta el criterio y la evidencia relacionada.<br>3. Introduce la valoración y su justificación.<br>4. El sistema verifica rango de puntuación y datos obligatorios.<br>5. El docente confirma la revisión.<br>6. El sistema conserva la trazabilidad y recalcula lo que corresponda; devuelve el control a CU08 para publicar cuando proceda. |
| Subflujos y relaciones | Incluye CU10. Extiende CU08 en PE08: Hay un criterio pendiente o discrepancia que requiere revisión justificada. |
| Flujos alternativos | A1, paso 3: añadir solo un comentario mantiene la puntuación.<br>A2, paso 5: cancelar conserva la evaluación anterior. |
| Excepciones y respuesta | E1, paso 1: grupo ajeno o permiso revocado. Se impide revisar.<br>E2, paso 4: puntuación fuera de rango o justificación vacía. Se pide corregir.<br>E3, paso 6: otro docente cambió la evaluación. Se muestra la versión nueva antes de aceptar otra modificación. |
| Puntos de extensión | PE08 en CU08 |
| Casos de prueba | PR13. Las condiciones y resultados verificables se detallan en 4.1. |
| Diagrama | ![Diagrama de CU16 Completar o rectificar una evaluación](<diagramas/v2/cu16.png>) |



#### 2.5.3 Bloques y flujos de datos

##### Diagrama de bloques

![Diagrama de bloques](<diagramas/bloques.png>)

Los bloques representan responsabilidades lógicas. El servicio de diálogo recibe el contexto permitido; el estado y la evidencia se conservan de forma persistente. El alojamiento y la tecnología se acordarán con el equipo de SIRCLE.

##### Diagrama de flujo de datos

![Diagrama de flujo de datos](<diagramas/flujo_datos.png>)

Las entidades intercambian información con procesos del módulo y almacenes lógicos. La relación entre ficha e intento debe conservarse en cada operación, de acuerdo con el contrato de integración.

##### Diagrama de secuencia

![Diagrama de secuencia](<diagramas/secuencia.png>)

El ejemplo utiliza un examen con disponibilidad a los tres días simulados, plazo propio del caso de prueba. Cerrar la sesión real conserva el reloj clínico; el resultado aparece tras un avance confirmado.

## 3 Requerimientos del sistema

Todos los requerimientos están propuestos para revisión. La prioridad crítica identifica condiciones sin las cuales no se acepta el módulo; la prioridad alta corresponde al recorrido educativo previsto y la media a funciones de apoyo. La estabilidad indica si el alcance y sus parámetros requieren acuerdo. Ninguna ficha declara una función implementada o una prueba aprobada.

### 3.1 Requerimientos de usuario

#### 3.1.1 Requerimientos de funcionalidades esperadas

##### RU01 Acceso según responsabilidad

| Atributo · RU01 | Descripción |
| --- | --- |
| Identificador | RU01 |
| Nombre | Acceso según responsabilidad |
| Descripción | El usuario necesita ingresar desde SIRCLE y utilizar únicamente funciones y registros correspondientes a su rol y grupo. El administrador necesita habilitar y retirar estos permisos. |
| Prioridad | Crítica |
| Fuente | Necesidad del proyecto propuesta para revisión con sus usuarios y el profesor. |
| Estabilidad | Pendiente de validación del alcance y de los parámetros indicados. |
| Estado | Propuesto; cumplimiento no verificado. |
| Listado de usuarios | Estudiante, docente y administrador |
| Criterio de aceptación | Una cuenta válida sin autorización no accede al módulo ni a datos de otro usuario. La revocación se aplica a la siguiente operación. |
| Requerimientos de software | RF01, RF02, RC02, RI01 |
| Casos de prueba | PR01, PR02, PR03 |



##### RU02 Actividades acordes a la formación

| Atributo · RU02 | Descripción |
| --- | --- |
| Identificador | RU02 |
| Nombre | Actividades acordes a la formación |
| Descripción | El docente necesita preparar y asignar casos ficticios con objetivos, nivel, acciones y criterios adecuados a la carrera y curso. |
| Prioridad | Alta |
| Fuente | Necesidad del proyecto propuesta para revisión con sus usuarios y el profesor. |
| Estabilidad | Pendiente de validación del alcance y de los parámetros indicados. |
| Estado | Propuesto; cumplimiento no verificado. |
| Listado de usuarios | Docente y estudiante |
| Criterio de aceptación | El estudiante inicia una versión revisada y asignada a su grupo; la preparación conserva borradores y versiones anteriores. |
| Requerimientos de software | RF03, RM01 |
| Casos de prueba | PR04, PR23, PR26 |



##### RU03 Valoración y seguimiento del paciente

| Atributo · RU03 | Descripción |
| --- | --- |
| Identificador | RU03 |
| Nombre | Valoración y seguimiento del paciente |
| Descripción | El estudiante necesita entrevistar, explorar, reunir resultados, documentar su razonamiento y observar consecuencias de sus decisiones durante varios encuentros del mismo paciente. |
| Prioridad | Alta |
| Fuente | Necesidad del proyecto propuesta para revisión con sus usuarios y el profesor. |
| Estabilidad | Pendiente de validación del alcance y de los parámetros indicados. |
| Estado | Propuesto; cumplimiento no verificado. |
| Listado de usuarios | Estudiante |
| Criterio de aceptación | Los hallazgos, intervenciones y avances temporales corresponden al caso y conservan una historia coherente. |
| Requerimientos de software | RF05, RF06, RF07, RF08, RI02 |
| Casos de prueba | PR07, PR08, PR09, PR10, PR11 |



##### RU04 Continuidad y repetición de la práctica

| Atributo · RU04 | Descripción |
| --- | --- |
| Identificador | RU04 |
| Nombre | Continuidad y repetición de la práctica |
| Descripción | El estudiante necesita salir, volver al mismo intento y, cuando esté permitido, repetir el caso sin borrar su trabajo previo. |
| Prioridad | Crítica |
| Fuente | Necesidad del proyecto propuesta para revisión con sus usuarios y el profesor. |
| Estabilidad | Pendiente de validación del alcance y de los parámetros indicados. |
| Estado | Propuesto; cumplimiento no verificado. |
| Listado de usuarios | Estudiante |
| Criterio de aceptación | Retomar recupera lo confirmado; repetir crea otra ficha e intento y conserva los anteriores. |
| Requerimientos de software | RF04, RO01, RC01, RI01, RE02, RM02, RIN01 |
| Casos de prueba | PR05, PR06, PR19 |



##### RU05 Evaluación comprensible y supervisión

| Atributo · RU05 | Descripción |
| --- | --- |
| Identificador | RU05 |
| Nombre | Evaluación comprensible y supervisión |
| Descripción | El estudiante necesita conocer las razones de su evaluación. El docente necesita revisar las decisiones, omisiones y evidencias del grupo, completar criterios y publicar comentarios. |
| Prioridad | Alta |
| Fuente | Necesidad del proyecto propuesta para revisión con sus usuarios y el profesor. |
| Estabilidad | Pendiente de validación del alcance y de los parámetros indicados. |
| Estado | Propuesto; cumplimiento no verificado. |
| Listado de usuarios | Estudiante y docente |
| Criterio de aceptación | Cada resultado remite a una rúbrica y evidencia; una revisión conserva autor, motivo y versión. |
| Requerimientos de software | RF09, RF10 |
| Casos de prueba | PR12, PR13, PR14 |



##### RU06 Evaluación de la experiencia educativa

| Atributo · RU06 | Descripción |
| --- | --- |
| Identificador | RU06 |
| Nombre | Evaluación de la experiencia educativa |
| Descripción | El equipo docente necesita observar confianza declarada, desempeño y participación por separado. El estudiante debe poder responder encuestas sin que estas alteren su calificación clínica. |
| Prioridad | Media |
| Fuente | Necesidad del proyecto propuesta para revisión con sus usuarios y el profesor. |
| Estabilidad | Pendiente de validación del alcance y de los parámetros indicados. |
| Estado | Propuesto; cumplimiento no verificado. |
| Listado de usuarios | Estudiante y docente |
| Criterio de aceptación | Se reconocen respuestas previas y posteriores, pares completos y datos faltantes; el reporte separa esos datos de la nota. |
| Requerimientos de software | RF10, RF11, RO02 |
| Casos de prueba | PR14, PR15 |



#### 3.1.2 Requerimientos de calidad

##### RUC01 Comprensión y facilidad de uso

| Atributo · RUC01 | Descripción |
| --- | --- |
| Identificador | RUC01 |
| Nombre | Comprensión y facilidad de uso |
| Descripción | El usuario necesita entender qué paciente atiende, qué momento simulado observa y si sus cambios se guardaron. Debe poder localizar y corregir los errores de entrada. |
| Prioridad | Alta |
| Fuente | Necesidad del proyecto propuesta para revisión con sus usuarios y el profesor. |
| Estabilidad | Pendiente de validación del alcance y de los parámetros indicados. |
| Estado | Propuesto; cumplimiento no verificado. |
| Listado de usuarios | Todos los roles |
| Criterio de aceptación | Las tareas principales se completan sin controles inaccesibles; los errores indican el campo afectado y conservan las otras entradas. |
| Requerimientos de software | RO01, RUS01, RUS02, RP01 |
| Casos de prueba | PR06, PR17, PR22 |



##### RUC02 Coherencia y conservación del trabajo

| Atributo · RUC02 | Descripción |
| --- | --- |
| Identificador | RUC02 |
| Nombre | Coherencia y conservación del trabajo |
| Descripción | El estudiante y el docente necesitan confiar en que los antecedentes, la evolución y las evidencias guardadas corresponden al mismo intento y versión. |
| Prioridad | Crítica |
| Fuente | Necesidad del proyecto propuesta para revisión con sus usuarios y el profesor. |
| Estabilidad | Pendiente de validación del alcance y de los parámetros indicados. |
| Estado | Propuesto; cumplimiento no verificado. |
| Listado de usuarios | Estudiante y docente |
| Criterio de aceptación | No se pierde un cambio confirmado ni se mezclan pacientes, y el tiempo simulado coincide con los eventos registrados. |
| Requerimientos de software | RF04, RF08, RC01, RIN01, RM01, RM02 |
| Casos de prueba | PR06, PR11, PR16, PR19 |



##### RUC03 Protección de información académica

| Atributo · RUC03 | Descripción |
| --- | --- |
| Identificador | RUC03 |
| Nombre | Protección de información académica |
| Descripción | El usuario necesita que sus respuestas y evaluaciones se consulten solo por personas autorizadas. La administración de accesos no concede automáticamente facultad para modificar notas. |
| Prioridad | Crítica |
| Fuente | Necesidad del proyecto propuesta para revisión con sus usuarios y el profesor. |
| Estabilidad | Pendiente de validación del alcance y de los parámetros indicados. |
| Estado | Propuesto; cumplimiento no verificado. |
| Listado de usuarios | Todos los roles |
| Criterio de aceptación | Las pruebas de acceso directo, grupo ajeno y rol incorrecto rechazan las operaciones sin exponer datos. |
| Requerimientos de software | RF01, RF02, RC02, RI02 |
| Casos de prueba | PR03, PR21 |



##### RUC04 Respuesta oportuna durante la práctica

| Atributo · RUC04 | Descripción |
| --- | --- |
| Identificador | RUC04 |
| Nombre | Respuesta oportuna durante la práctica |
| Descripción | El estudiante necesita que las esperas estén informadas y que las operaciones respondan dentro de tiempos aceptables para una sesión docente. |
| Prioridad | Alta |
| Fuente | Necesidad del proyecto propuesta para revisión con sus usuarios y el profesor. |
| Estabilidad | Pendiente de validación del alcance y de los parámetros indicados. |
| Estado | Propuesto; cumplimiento no verificado. |
| Listado de usuarios | Estudiante y docente |
| Criterio de aceptación | Se miden apertura, guardado, diálogo y avance por separado frente al perfil de RR01; no se comunica éxito durante una operación pendiente. |
| Requerimientos de software | RR01, RES01 |
| Casos de prueba | PR20, PR25 |



#### 3.1.3 Requerimientos de restricción

##### RUR01 Integración con la plataforma existente

| Atributo · RUR01 | Descripción |
| --- | --- |
| Identificador | RUR01 |
| Nombre | Integración con la plataforma existente |
| Descripción | El módulo deberá reutilizar la identidad y los registros compatibles de SIRCLE. El proyecto no incorpora otra plataforma institucional ni una administración independiente de contraseñas. |
| Prioridad | Crítica |
| Fuente | Necesidad del proyecto propuesta para revisión con sus usuarios y el profesor. |
| Estabilidad | Pendiente de validación del alcance y de los parámetros indicados. |
| Estado | Propuesto; cumplimiento no verificado. |
| Listado de usuarios | Todos los roles |
| Criterio de aceptación | La sesión y los registros se conectan mediante un contrato acordado; el flujo habitual de SIRCLE sigue funcionando. |
| Requerimientos de software | RF01, RI01, RIN01, RP01, RD01 |
| Casos de prueba | PR01, PR16, PR18 |



##### RUR02 Uso educativo de pacientes ficticios

| Atributo · RUR02 | Descripción |
| --- | --- |
| Identificador | RUR02 |
| Nombre | Uso educativo de pacientes ficticios |
| Descripción | Los casos deberán utilizar identidades ficticias. La revisión docente comprobará el contenido antes de publicarlo; las cuentas y resultados de estudiantes siguen siendo información académica protegida. |
| Prioridad | Crítica |
| Fuente | Necesidad del proyecto propuesta para revisión con sus usuarios y el profesor. |
| Estabilidad | Pendiente de validación del alcance y de los parámetros indicados. |
| Estado | Propuesto; cumplimiento no verificado. |
| Listado de usuarios | Docente y administrador |
| Criterio de aceptación | El caso publicado tiene revisión registrada de sus datos ficticios y los ambientes de prueba no usan fichas de pacientes reales. |
| Requerimientos de software | RF03, RE01, RC02 |
| Casos de prueba | PR21, PR26 |



##### RUR03 Límites clínicos y revisión docente

| Atributo · RUR03 | Descripción |
| --- | --- |
| Identificador | RUR03 |
| Nombre | Límites clínicos y revisión docente |
| Descripción | Cada caso deberá definir las acciones y evoluciones que representa. El contenido asistido por IA necesita revisión antes de su publicación; la simulación informa sus límites cuando no existe una respuesta sustentada. |
| Prioridad | Crítica |
| Fuente | Necesidad del proyecto propuesta para revisión con sus usuarios y el profesor. |
| Estabilidad | Pendiente de validación del alcance y de los parámetros indicados. |
| Estado | Propuesto; cumplimiento no verificado. |
| Listado de usuarios | Docente y estudiante |
| Criterio de aceptación | No se publica un caso sin revisión ni se inventan resultados para acciones fuera del caso. Las consecuencias erróneas previstas se pueden observar y evaluar. |
| Requerimientos de software | RF03, RF05, RF07, RF08, RD01 |
| Casos de prueba | PR07, PR10, PR11, PR26 |



##### RUR04 Dependencia del ambiente acordado

| Atributo · RUR04 | Descripción |
| --- | --- |
| Identificador | RUR04 |
| Nombre | Dependencia del ambiente acordado |
| Descripción | La solución deberá operar con la infraestructura y contratos autorizados por el equipo de SIRCLE. El alojamiento de la IA y los recursos del servidor se determinarán mediante pruebas, sin asumir que existe una GPU local. |
| Prioridad | Alta |
| Fuente | Necesidad del proyecto propuesta para revisión con sus usuarios y el profesor. |
| Estabilidad | Pendiente de validación del alcance y de los parámetros indicados. |
| Estado | Propuesto; cumplimiento no verificado. |
| Listado de usuarios | Equipo técnico y administrador |
| Criterio de aceptación | Se documenta el ambiente y se comprueba el perfil del piloto antes de declarar que la capacidad es suficiente. |
| Requerimientos de software | RE01, RE02, RR01, RES01, RI02 |
| Casos de prueba | PR16, PR20, PR21, PR25 |



### 3.2 Requerimientos de software

#### 3.2.1 Requerimientos funcionales

##### RF01 Acceder mediante SIRCLE y aplicar el rol

| Atributo · RF01 | Descripción |
| --- | --- |
| Identificador | RF01 |
| Nombre | Acceder mediante SIRCLE y aplicar el rol |
| Descripción | El módulo deberá aceptar usuarios autenticados por SIRCLE, verificar sus permisos y mostrar las funciones correspondientes. Al expirar o cerrarse la sesión deberá impedir nuevas operaciones protegidas, conservando los cambios ya confirmados. |
| Prioridad | Crítica |
| Fuente | RU01 |
| Estabilidad | Propuesto; integración y criterios clínicos sujetos a validación. |
| Estado | Pendiente de implementación y verificación. |
| Listado de usuarios | Estudiante, docente o administrador; SIRCLE valida la identidad. |
| Interfaz | acceso y navegación de SIRCLE; intercambio verificable de identidad, vigencia de sesión y permisos. El módulo no almacenará una segunda contraseña. |
| Excepciones y respuesta | una cuenta inexistente o una contraseña incorrecta impiden iniciar sesión en SIRCLE; se permite reintentar y recurrir al mecanismo de recuperación o soporte que la plataforma tenga habilitado. Una sesión vencida exige volver a ingresar. Un usuario válido sin autorización recibe una denegación de acceso al módulo, incluso si intenta abrirlo mediante una dirección directa. |
| Criterio de aceptación | cada cuenta obtiene únicamente las funciones y datos autorizados. |
| Casos de uso | CU01, CU10 |
| Casos de prueba | PR01, PR02, PR03 |



##### RF02 Administrar permisos y responsabilidad docente

| Atributo · RF02 | Descripción |
| --- | --- |
| Identificador | RF02 |
| Nombre | Administrar permisos y responsabilidad docente |
| Descripción | El administrador deberá habilitar o revocar el acceso al módulo para cuentas existentes y vincular docentes con los grupos que supervisan. Se registrarán el responsable, la fecha y el cambio. Se utilizará la gestión de SIRCLE cuando permita estas operaciones; la extensión necesaria se acordará con su equipo. |
| Prioridad | Crítica |
| Fuente | RU01 |
| Estabilidad | Propuesto; integración y criterios clínicos sujetos a validación. |
| Estado | Pendiente de implementación y verificación. |
| Listado de usuarios | Administrador; SIRCLE proporciona cuentas y grupos. |
| Interfaz | vista administrativa y mecanismo de consulta o actualización de usuarios, roles y grupos de SIRCLE. |
| Excepciones y respuesta | usuario o grupo inexistente, actor sin permiso o fallo al guardar. El sistema rechazará la operación sin cambios parciales. La revocación deberá surtir efecto en la siguiente operación protegida, aunque la pantalla siga abierta. |
| Criterio de aceptación | la asignación correcta habilita el acceso y su retiro lo impide sin borrar el historial. |
| Casos de uso | CU02, CU10 |
| Casos de prueba | PR03 |



##### RF03 Preparar y asignar casos revisados

| Atributo · RF03 | Descripción |
| --- | --- |
| Identificador | RF03 |
| Nombre | Preparar y asignar casos revisados |
| Descripción | El docente deberá preparar o seleccionar un caso con identidad ficticia, antecedentes, objetivos, nivel, acciones disponibles, reglas de evolución y rúbrica. Antes de habilitarlo deberá quedar registrada su revisión clínica. Podrá asignarlo a estudiantes de sus grupos, definir oportunidades de práctica y decidir cuándo se publica la retroalimentación. La preparación podrá apoyarse en IA, con revisión docente antes de su uso. |
| Prioridad | Alta |
| Fuente | RU02, RUR02, RUR03 |
| Estabilidad | Propuesto; integración y criterios clínicos sujetos a validación. |
| Estado | Pendiente de implementación y verificación. |
| Listado de usuarios | Docente con permiso de preparación y asignación. |
| Interfaz | editor o catálogo docente, grupos autorizados y repositorio de casos. El intercambio con la ficha deberá identificar el caso y su versión. |
| Excepciones y respuesta | contenido obligatorio incompleto, reglas incompatibles o falta de revisión mantienen el caso como borrador. Un grupo ajeno no puede recibir la asignación. Los cambios en un caso publicado generan otra versión y conservan la usada por intentos previos. |
| Criterio de aceptación | solo una versión revisada y asignada puede iniciarse. |
| Casos de uso | CU03 |
| Casos de prueba | PR04, PR26 |



##### RF04 Iniciar y recuperar intentos independientes

| Atributo · RF04 | Descripción |
| --- | --- |
| Identificador | RF04 |
| Nombre | Iniciar y recuperar intentos independientes |
| Descripción | El estudiante deberá consultar sus actividades y elegir entre iniciar un caso, continuar un intento o comenzar nuevamente cuando esté permitido. Cada intento tendrá un identificador propio, una versión del caso y una relación inequívoca con su ficha. Al retomarlo se recuperarán los registros confirmados, el estado del paciente y el reloj simulado. |
| Prioridad | Crítica |
| Fuente | RU04 |
| Estabilidad | Propuesto; integración y criterios clínicos sujetos a validación. |
| Estado | Pendiente de implementación y verificación. |
| Listado de usuarios | Estudiante. |
| Interfaz | listado de actividades e historial; almacenamiento de intentos y ficha vinculada en SIRCLE. |
| Excepciones y respuesta | actividad no asignada, límite de intentos alcanzado o intento de otro usuario bloquean el acceso. Si falla la creación, no se mostrará un intento incompleto como disponible. Un doble clic no creará dos intentos. La acción de "nuevo paciente" deberá proteger el historial previo, aunque el flujo actual de SIRCLE limpie campos para iniciar otra ficha. |
| Criterio de aceptación | continuar conserva el paciente; comenzar de nuevo crea otro intento sin sobrescribirlo. |
| Casos de uso | CU04, CU11, CU12 |
| Casos de prueba | PR05, PR06 |



##### RF05 Entrevistar y obtener hallazgos del paciente

| Atributo · RF05 | Descripción |
| --- | --- |
| Identificador | RF05 |
| Nombre | Entrevistar y obtener hallazgos del paciente |
| Descripción | El módulo deberá permitir preguntar al paciente o cuidador y realizar valoraciones disponibles en el caso, como observar, medir el peso o consultar signos vitales. Cada respuesta deberá corresponder a los antecedentes y al estado del paciente. El registro distinguirá el hallazgo obtenido de la interpretación escrita por el estudiante. |
| Prioridad | Alta |
| Fuente | RU03 |
| Estabilidad | Propuesto; integración y criterios clínicos sujetos a validación. |
| Estado | Pendiente de implementación y verificación. |
| Listado de usuarios | Estudiante; servicio de conversación cuando se utilice. |
| Interfaz | conversación y controles de exploración; servicio de diálogo, estado del caso y campos de valoración de SIRCLE. |
| Excepciones y respuesta | pregunta ambigua solicita aclaración; información o maniobra no disponible se informa como tal. Un fallo del servicio no genera respuestas ficticias ni cambia el estado. Las instrucciones del estudiante no podrán alterar los antecedentes ni revelar la pauta docente reservada. |
| Criterio de aceptación | el mismo hecho conserva su significado hasta que un evento del caso lo modifique. |
| Casos de uso | CU05, CU11 |
| Casos de prueba | PR07 |



##### RF06 Solicitar exámenes y consultar resultados

| Atributo · RF06 | Descripción |
| --- | --- |
| Identificador | RF06 |
| Nombre | Solicitar exámenes y consultar resultados |
| Descripción | El estudiante deberá solicitar los exámenes habilitados y consultar su estado: solicitado, pendiente o disponible. El resultado incluirá identificación, unidades cuando correspondan y momento simulado. Su disponibilidad respetará el plazo definido por el caso. |
| Prioridad | Alta |
| Fuente | RU03 |
| Estabilidad | Propuesto; integración y criterios clínicos sujetos a validación. |
| Estado | Pendiente de implementación y verificación. |
| Listado de usuarios | Estudiante; servicio de conversación cuando se utilice. |
| Interfaz | órdenes y resultados, campos de exámenes de SIRCLE y reloj de la simulación. |
| Excepciones y respuesta | examen fuera del caso informa su indisponibilidad; un resultado pendiente no se presenta como normal ni como cero. Si se pierde una confirmación, reintentar la misma solicitud no duplica la orden. Una nueva toma deliberada se registra como otra solicitud. |
| Criterio de aceptación | el resultado correcto aparece en el intento y momento previstos. |
| Casos de uso | CU05, CU11, CU13 |
| Casos de prueba | PR08 |



##### RF07 Registrar razonamiento y ejecutar intervenciones

| Atributo · RF07 | Descripción |
| --- | --- |
| Identificador | RF07 |
| Nombre | Registrar razonamiento y ejecutar intervenciones |
| Descripción | El estudiante deberá documentar hipótesis, diagnóstico, justificación y plan en los campos correspondientes. Las intervenciones deberán identificarse y confirmarse con sus parámetros requeridos antes de afectar la simulación. El historial distinguirá una propuesta, una indicación y una acción ejecutada. |
| Prioridad | Alta |
| Fuente | RU03 |
| Estabilidad | Propuesto; integración y criterios clínicos sujetos a validación. |
| Estado | Pendiente de implementación y verificación. |
| Listado de usuarios | Estudiante. |
| Interfaz | ficha clínica de SIRCLE y confirmación de acciones; registro de eventos y reglas del caso. |
| Excepciones y respuesta | una acción incompleta o ambigua exige corrección; una acción no representada por el caso informa el límite. Una decisión clínicamente equivocada, si el caso contempla sus consecuencias, se conserva para la evolución y evaluación. Repetir el envío de una misma confirmación no vuelve a ejecutar la acción. Un error de guardado se muestra como pendiente. |
| Criterio de aceptación | anotar un plan no lo ejecuta y cada confirmación válida produce un único evento. |
| Casos de uso | CU06, CU11 |
| Casos de prueba | PR09, PR10 |



##### RF08 Avanzar el tiempo y reevaluar al mismo paciente

| Atributo · RF08 | Descripción |
| --- | --- |
| Identificador | RF08 |
| Nombre | Avanzar el tiempo y reevaluar al mismo paciente |
| Descripción | El módulo deberá permitir un avance explícito del tiempo dentro de los intervalos admitidos por el caso. Aplicará los eventos pendientes y las consecuencias de las intervenciones, conservará antecedentes y mostrará el nuevo estado para continuar la atención. Guardará juntos el cambio temporal y su evolución asociada. |
| Prioridad | Alta |
| Fuente | RU03, RUC02 |
| Estabilidad | Propuesto; integración y criterios clínicos sujetos a validación. |
| Estado | Pendiente de implementación y verificación. |
| Listado de usuarios | Estudiante. |
| Interfaz | control de avance, cronología y vista clínica; reglas de evolución, solicitudes pendientes y almacenamiento. |
| Excepciones y respuesta | intervalo inválido o fuera del periodo representado se rechaza. Una actualización fallida no deja el reloj adelantado con datos antiguos. Si otra sesión ya cambió el intento, se exige actualizar la vista antes de operar. El cierre del navegador no adelanta el tiempo por sí mismo. |
| Criterio de aceptación | reloj, resultados y estado permanecen consistentes. |
| Casos de uso | CU06, CU11, CU14 |
| Casos de prueba | PR11 |



##### RF09 Finalizar y entregar retroalimentación fundamentada

| Atributo · RF09 | Descripción |
| --- | --- |
| Identificador | RF09 |
| Nombre | Finalizar y entregar retroalimentación fundamentada |
| Descripción | Al finalizar, el módulo deberá conservar la entrega y evaluarla con la versión de la rúbrica asociada al intento. La retroalimentación explicará los criterios cumplidos, las omisiones y los errores con evidencia del registro. El diagnóstico, la pertinencia de los exámenes, el seguimiento y la documentación tendrán las ponderaciones definidas por el docente, sin premiar la cantidad de acciones por sí sola. |
| Prioridad | Alta |
| Fuente | RU05 |
| Estabilidad | Propuesto; integración y criterios clínicos sujetos a validación. |
| Estado | Pendiente de implementación y verificación. |
| Listado de usuarios | Estudiante. |
| Interfaz | entrega, resultado y comentarios; historial del intento y rúbrica. Se relacionará el cierre académico con el cierre de ficha de SIRCLE sin confundirlos. |
| Excepciones y respuesta | documentación obligatoria ausente solicita completarla; una acción clínica omitida puede ser evaluada como omisión sin impedir la entrega. Una evaluación automática fallida conserva la entrega como pendiente, sin asignar cero. Los criterios que requieran juicio docente quedan por revisar. La retroalimentación se oculta hasta su publicación autorizada. |
| Criterio de aceptación | cada resultado puede rastrearse a la rúbrica y a la evidencia, con identificación de la revisión docente. |
| Casos de uso | CU07, CU11, CU15, CU16 |
| Casos de prueba | PR12, PR13 |



##### RF10 Supervisar resultados y participación

| Atributo · RF10 | Descripción |
| --- | --- |
| Identificador | RF10 |
| Nombre | Supervisar resultados y participación |
| Descripción | El docente deberá consultar a los estudiantes de sus grupos, sus intentos, decisiones, entregas y resultados por criterio. Podrá revisar criterios pendientes, añadir comentarios y publicar la retroalimentación. La vista resumirá casos iniciados y terminados, número de intentos y tiempo activo estimado, con filtros por actividad y estudiante. |
| Prioridad | Alta |
| Fuente | RU05, RU06 |
| Estabilidad | Propuesto; integración y criterios clínicos sujetos a validación. |
| Estado | Pendiente de implementación y verificación. |
| Listado de usuarios | Docente. |
| Interfaz | panel docente, detalle de intento, evaluación y registros de actividad. |
| Excepciones y respuesta | un grupo ajeno se bloquea; ausencia de datos se presenta como "sin registros". Una evaluación pendiente se identifica y se excluye de promedios de notas hasta completarse. Las rectificaciones conservarán responsable y motivo. Los intervalos de inactividad no deberán sumarse como trabajo continuo; el criterio de inactividad deberá acordarse y mostrarse en los reportes. |
| Criterio de aceptación | las cifras distinguen estudiantes de intentos y coinciden con sus registros. |
| Casos de uso | CU08, CU16 |
| Casos de prueba | PR13, PR14 |



##### RF11 Recoger encuestas antes y después de la actividad

| Atributo · RF11 | Descripción |
| --- | --- |
| Identificador | RF11 |
| Nombre | Recoger encuestas antes y después de la actividad |
| Descripción | El módulo deberá presentar los instrumentos definidos por el equipo docente en los momentos previstos y guardar respuestas, versión del instrumento y actividad. Permitirá relacionar las respuestas previas y posteriores mediante un identificador de estudio, según las condiciones acordadas para la evaluación. Las respuestas sobre confianza y experiencia se mantendrán separadas de la nota clínica. |
| Prioridad | Media |
| Fuente | RU06 |
| Estabilidad | Propuesto; integración y criterios clínicos sujetos a validación. |
| Estado | Pendiente de implementación y verificación. |
| Listado de usuarios | Estudiante. |
| Interfaz | formulario de encuesta y resultados autorizados; instrumentos y registro de participación. Podrá reutilizar una herramienta institucional si conserva estas relaciones. |
| Excepciones y respuesta | una encuesta no respondida se registra como dato faltante. La encuesta previa no podrá completarse retroactivamente después de iniciar la actividad y presentarse como medida inicial. Los envíos repetidos no generan respuestas duplicadas. Omitir una encuesta de investigación no bloqueará la práctica ni reducirá la nota. |
| Criterio de aceptación | los pares completos, datos faltantes y momentos de respuesta pueden identificarse correctamente. |
| Casos de uso | CU09 |
| Casos de prueba | PR15 |



#### 3.2.2 Requerimientos de interfaz

##### RI01 Contrato de identidad y datos

| Atributo · RI01 | Descripción |
| --- | --- |
| Identificador | RI01 |
| Nombre | Contrato de identidad y datos |
| Descripción | La integración deberá identificar usuario, rol, grupo, caso, versión, intento y ficha. Acordará campos, unidades, fechas, estados y respuestas de error. La autenticación utilizará la sesión verificable de SIRCLE, sin transmitir la contraseña al módulo. |
| Prioridad | Crítica |
| Fuente | RU01, RU04, RUR01 |
| Estabilidad | Parámetros propuestos; se ratificarán antes de la aceptación. |
| Estado | Propuesto; no verificado. |
| Listado de usuarios | Todos los roles |
| Criterio de aceptación | Un registro de prueba conserva sus identificadores y significado en ambos extremos; los datos incompatibles se rechazan de forma explícita. |
| Casos de prueba | PR01, PR16 |



##### RI02 Interfaz del servicio de conversación

| Atributo · RI02 | Descripción |
| --- | --- |
| Identificador | RI02 |
| Nombre | Interfaz del servicio de conversación |
| Descripción | El intercambio con el servicio de diálogo deberá enviar solo el contexto permitido del caso y distinguir respuesta, error y operación pendiente. Las credenciales del servicio se mantendrán del lado del servidor. El proveedor y alojamiento quedan por acordar. |
| Prioridad | Alta |
| Fuente | RU03, RUC03, RUR04 |
| Estabilidad | Parámetros propuestos; se ratificarán antes de la aceptación. |
| Estado | Propuesto; no verificado. |
| Listado de usuarios | Estudiante y equipo técnico |
| Criterio de aceptación | Un fallo del servicio comunica su estado y no modifica el paciente ni expone credenciales en el navegador. |
| Casos de prueba | PR07, PR16, PR21 |



#### 3.2.3 Requerimientos operacionales

##### RO01 Estados de la actividad y continuidad

| Atributo · RO01 | Descripción |
| --- | --- |
| Identificador | RO01 |
| Nombre | Estados de la actividad y continuidad |
| Descripción | La interfaz deberá distinguir intento en curso, pausado y entregado, además de evaluación pendiente o publicada. Mostrará identidad ficticia, actividad, tiempo simulado y guardado. Cerrar sesión no finaliza la actividad ni adelanta el reloj. |
| Prioridad | Alta |
| Fuente | RU04, RUC01 |
| Estabilidad | Parámetros propuestos; se ratificarán antes de la aceptación. |
| Estado | Propuesto; no verificado. |
| Listado de usuarios | Estudiante y docente |
| Criterio de aceptación | El recorrido de inicio, pausa, salida, recuperación y entrega conserva el estado correcto y evita modificar una entrega cerrada. |
| Casos de prueba | PR06, PR12 |



##### RO02 Registro de participación

| Atributo · RO02 | Descripción |
| --- | --- |
| Identificador | RO02 |
| Nombre | Registro de participación |
| Descripción | El módulo registrará inicio, reanudación, pausa y entrega, y distinguirá estudiantes de intentos. El tiempo activo será una estimación separada del reloj clínico; se excluirán pausas explícitas y periodos de inactividad según el criterio acordado antes del piloto. |
| Prioridad | Media |
| Fuente | RU06 |
| Estabilidad | Parámetros propuestos; se ratificarán antes de la aceptación. |
| Estado | Propuesto; no verificado. |
| Listado de usuarios | Estudiante y docente |
| Criterio de aceptación | El panel reproduce los conteos de su detalle y no suma una pausa explícita al tiempo activo. El reporte identifica el criterio utilizado para tratar inactividad. |
| Casos de prueba | PR14 |



#### 3.2.4 Requerimientos del entorno

##### RE01 Ambiente de pruebas y configuración

| Atributo · RE01 | Descripción |
| --- | --- |
| Identificador | RE01 |
| Nombre | Ambiente de pruebas y configuración |
| Descripción | El equipo deberá disponer de un ambiente separado con cuentas y fichas ficticias. Documentará sistema operativo, versiones de servicios, CPU, memoria, almacenamiento y conectividad utilizados, y comprobará que permiten ejecutar el perfil aprobado del piloto. |
| Prioridad | Alta |
| Fuente | RUR02, RUR04 |
| Estabilidad | Parámetros propuestos; se ratificarán antes de la aceptación. |
| Estado | Propuesto; no verificado. |
| Listado de usuarios | Equipo técnico y administrador |
| Criterio de aceptación | La configuración es reproducible y las pruebas se ejecutan sin modificar información de producción. La capacidad declarada se respalda con PR20. |
| Casos de prueba | PR20, PR21 |



##### RE02 Conectividad y operaciones pendientes

| Atributo · RE02 | Descripción |
| --- | --- |
| Identificador | RE02 |
| Nombre | Conectividad y operaciones pendientes |
| Descripción | El módulo requerirá conexión con SIRCLE y con los servicios que necesite cada función. Ante desconexión deberá identificar qué operaciones están confirmadas y cuáles pendientes. La primera versión no garantiza funcionamiento sin conexión. |
| Prioridad | Alta |
| Fuente | RU04, RUR04 |
| Estabilidad | Parámetros propuestos; se ratificarán antes de la aceptación. |
| Estado | Propuesto; no verificado. |
| Listado de usuarios | Estudiante y docente |
| Criterio de aceptación | Al recuperar la conexión se puede consultar o reintentar una solicitud con el mismo identificador, sin duplicarla. |
| Casos de prueba | PR16 |



#### 3.2.5 Requerimientos de usabilidad

##### RUS01 Claridad del registro y de los errores

| Atributo · RUS01 | Descripción |
| --- | --- |
| Identificador | RUS01 |
| Nombre | Claridad del registro y de los errores |
| Descripción | Las pantallas deberán usar etiquetas en español y mostrar unidades junto a los valores. Los errores señalarán el campo y la corrección necesaria, manteniendo las demás entradas. La interfaz distinguirá la observación obtenida de la interpretación del estudiante. |
| Prioridad | Alta |
| Fuente | RUC01 |
| Estabilidad | Parámetros propuestos; se ratificarán antes de la aceptación. |
| Estado | Propuesto; no verificado. |
| Listado de usuarios | Estudiante y docente |
| Criterio de aceptación | Los participantes pueden localizar errores y completar las tareas principales; se registra éxito, ayuda requerida y problemas observados. |
| Casos de prueba | PR17 |



##### RUS02 Acceso a controles y lectura

| Atributo · RUS02 | Descripción |
| --- | --- |
| Identificador | RUS02 |
| Nombre | Acceso a controles y lectura |
| Descripción | Las tareas principales deberán poder operarse con teclado, foco visible y etiquetas identificables. Las confirmaciones y mensajes no dependerán solo del color. La ampliación del texto no deberá ocultar acciones necesarias. |
| Prioridad | Alta |
| Fuente | RUC01 |
| Estabilidad | Parámetros propuestos; se ratificarán antes de la aceptación. |
| Estado | Propuesto; no verificado. |
| Listado de usuarios | Todos los roles |
| Criterio de aceptación | El recorrido por teclado permite iniciar, registrar, entregar y consultar resultados sin bloqueos; no hay controles indispensables recortados en la matriz admitida. |
| Casos de prueba | PR17, PR22 |



#### 3.2.6 Requerimientos de mantención

##### RM01 Versiones de contenido y pruebas de regresión

| Atributo · RM01 | Descripción |
| --- | --- |
| Identificador | RM01 |
| Nombre | Versiones de contenido y pruebas de regresión |
| Descripción | Los contenidos, reglas y rúbricas deberán identificarse por versión y mantenerse separados de la lógica común. Antes de publicar una actualización se comprobarán el caso cambiado y los flujos que puedan verse afectados. |
| Prioridad | Alta |
| Fuente | RU02, RUC02 |
| Estabilidad | Parámetros propuestos; se ratificarán antes de la aceptación. |
| Estado | Propuesto; no verificado. |
| Listado de usuarios | Docente y equipo técnico |
| Criterio de aceptación | Los intentos conservan su versión; una modificación no cambia los resultados esperados de otros casos ni obliga a reescribir sus contenidos. |
| Casos de prueba | PR04, PR23 |



##### RM02 Respaldo y procedimiento de recuperación

| Atributo · RM02 | Descripción |
| --- | --- |
| Identificador | RM02 |
| Nombre | Respaldo y procedimiento de recuperación |
| Descripción | La operación deberá disponer de un procedimiento documentado para respaldar y restaurar casos, intentos, evaluaciones y vínculos con SIRCLE. Se acordarán frecuencia de respaldo, retención y responsables antes del piloto. |
| Prioridad | Crítica |
| Fuente | RU04, RUC02 |
| Estabilidad | Parámetros propuestos; se ratificarán antes de la aceptación. |
| Estado | Propuesto; no verificado. |
| Listado de usuarios | Equipo técnico y administrador |
| Criterio de aceptación | Una restauración de prueba recupera los registros contenidos en la copia y permite seguir sus relaciones. Se informa su antigüedad y el tiempo medido de recuperación. |
| Casos de prueba | PR19 |



#### 3.2.7 Requerimientos de portabilidad

##### RP01 Compatibilidad del cliente web

| Atributo · RP01 | Descripción |
| --- | --- |
| Identificador | RP01 |
| Nombre | Compatibilidad del cliente web |
| Descripción | El módulo deberá funcionar en la matriz de navegadores y equipos acordada para docencia. Como matriz inicial se proponen Chrome, Edge y Firefox en sus versiones estables al ejecutar las pruebas; se registrarán las versiones exactas. No se exige instalar una aplicación propia del módulo. |
| Prioridad | Alta |
| Fuente | RUC01, RUR01 |
| Estabilidad | Parámetros propuestos; se ratificarán antes de la aceptación. |
| Estado | Propuesto; no verificado. |
| Listado de usuarios | Todos los roles |
| Criterio de aceptación | Cada combinación declarada compatible completa el recorrido principal y muestra correctamente controles y resultados. |
| Casos de prueba | PR22 |



#### 3.2.8 Requerimientos de confiabilidad

##### RC01 Integridad y recuperación del estado

| Atributo · RC01 | Descripción |
| --- | --- |
| Identificador | RC01 |
| Nombre | Integridad y recuperación del estado |
| Descripción | Una acción confirmada deberá producir un único efecto. El reloj, estado y eventos relacionados se actualizarán de forma consistente; una modificación concurrente deberá detectarse. El sistema conservará los cambios confirmados ante una interrupción de sesión. |
| Prioridad | Crítica |
| Fuente | RU04, RUC02 |
| Estabilidad | Parámetros propuestos; se ratificarán antes de la aceptación. |
| Estado | Propuesto; no verificado. |
| Listado de usuarios | Estudiante y docente |
| Criterio de aceptación | Las pruebas de reenvío, conflicto y fallo no producen duplicación, pérdida de datos confirmados ni mezcla entre intentos. |
| Casos de prueba | PR05, PR06, PR09, PR11, PR19 |



##### RC02 Control de acceso y protección de credenciales

| Atributo · RC02 | Descripción |
| --- | --- |
| Identificador | RC02 |
| Nombre | Control de acceso y protección de credenciales |
| Descripción | Los permisos deberán verificarse en el servidor para cada operación protegida, además de ocultar opciones en la interfaz. El intercambio deberá utilizar el canal protegido acordado con SIRCLE y los secretos de integración no llegarán al navegador. |
| Prioridad | Crítica |
| Fuente | RU01, RUC03 |
| Estabilidad | Parámetros propuestos; se ratificarán antes de la aceptación. |
| Estado | Propuesto; no verificado. |
| Listado de usuarios | Todos los roles |
| Criterio de aceptación | Los accesos ajenos y permisos revocados se rechazan; la revisión de archivos, respuestas y errores del cliente no expone claves del servicio. |
| Casos de prueba | PR03, PR21 |



#### 3.2.9 Requerimientos de interoperabilidad

##### RIN01 Correspondencia entre intento y ficha

| Atributo · RIN01 | Descripción |
| --- | --- |
| Identificador | RIN01 |
| Nombre | Correspondencia entre intento y ficha |
| Descripción | La integración deberá mantener una relación inequívoca entre intento y ficha de SIRCLE. Documentará qué sistema conserva cada dato, cuándo se sincroniza y cómo se recupera un fallo. La creación de otro paciente no sobrescribirá una ficha vinculada a un intento previo. |
| Prioridad | Crítica |
| Fuente | RU04, RUR01, RI01 |
| Estabilidad | Parámetros propuestos; se ratificarán antes de la aceptación. |
| Estado | Propuesto; no verificado. |
| Listado de usuarios | Estudiante, docente y equipo técnico |
| Criterio de aceptación | Las fichas de intentos distintos permanecen separadas. La integración conserva los registros y cálculos del flujo existente y distingue cierre clínico y académico. |
| Casos de prueba | PR05, PR16, PR18 |



#### 3.2.10 Requerimientos de rendimiento

##### RR01 Tiempos de respuesta del piloto

| Atributo · RR01 | Descripción |
| --- | --- |
| Identificador | RR01 |
| Nombre | Tiempos de respuesta del piloto |
| Descripción | Como objetivos iniciales por ratificar se proponen 20 usuarios concurrentes durante 15 minutos y p95 de 2 s para abrir una actividad, 3 s para guardar, 15 s para diálogo y 5 s para avanzar el tiempo. Se medirán al menos 100 operaciones por categoría y una finalización de al menos 99 % de solicitudes válidas. |
| Prioridad | Alta |
| Fuente | RUC04, RUR04 |
| Estabilidad | Parámetros propuestos; se ratificarán antes de la aceptación. |
| Estado | Propuesto; no verificado. |
| Listado de usuarios | Estudiante y docente |
| Criterio de aceptación | Se cumple el perfil acordado y se entrega el informe con configuración, tamaño de muestra, latencias y fallos. Estos valores son metas propuestas, no resultados medidos. |
| Casos de prueba | PR20 |



#### 3.2.11 Requerimientos de documentación

##### RD01 Documentación técnica y clínica del módulo

| Atributo · RD01 | Descripción |
| --- | --- |
| Identificador | RD01 |
| Nombre | Documentación técnica y clínica del módulo |
| Descripción | El proyecto entregará contrato de integración, configuración, reglas y rúbricas versionadas, procedimiento de respaldo y registro de pruebas. La ayuda de uso cubrirá las funciones nuevas que lo requieran; no se elaborará nuevamente el manual general de SIRCLE. |
| Prioridad | Alta |
| Fuente | RUR01, RUR03 |
| Estabilidad | Parámetros propuestos; se ratificarán antes de la aceptación. |
| Estado | Propuesto; no verificado. |
| Listado de usuarios | Equipo técnico, docente y administrador |
| Criterio de aceptación | Otro integrante puede reproducir la vinculación de una ficha de prueba, localizar una regla y revisar la evidencia de una prueba. |
| Casos de prueba | PR24 |



#### 3.2.12 Requerimientos de escalabilidad

##### RES01 Crecimiento controlado de la carga

| Atributo · RES01 | Descripción |
| --- | --- |
| Identificador | RES01 |
| Nombre | Crecimiento controlado de la carga |
| Descripción | Antes del piloto se probarán escalones de 5, 10 y 20 usuarios como perfil inicial propuesto. Se medirán recursos y tiempos, manteniendo aislamiento de intentos. El escalón máximo aprobado deberá cumplir RR01; no se declara capacidad ilimitada ni crecimiento automático. |
| Prioridad | Media |
| Fuente | RUC04, RUR04 |
| Estabilidad | Parámetros propuestos; se ratificarán antes de la aceptación. |
| Estado | Propuesto; no verificado. |
| Listado de usuarios | Equipo técnico y administrador |
| Criterio de aceptación | El informe muestra límites observados y el comportamiento de cada escalón. No aparecen datos cruzados ni acciones duplicadas al aumentar la concurrencia. |
| Casos de prueba | PR25 |



### 3.3 Relación entre requerimientos de usuario y de software

| Necesidad | Requerimientos de software | Pruebas |
| --- | --- | --- |
| RU01 | RF01, RF02, RC02, RI01 | PR01, PR02, PR03 |
| RU02 | RF03, RM01 | PR04, PR23, PR26 |
| RU03 | RF05, RF06, RF07, RF08, RI02 | PR07, PR08, PR09, PR10, PR11 |
| RU04 | RF04, RO01, RC01, RI01, RE02, RM02, RIN01 | PR05, PR06, PR19 |
| RU05 | RF09, RF10 | PR12, PR13, PR14 |
| RU06 | RF10, RF11, RO02 | PR14, PR15 |
| RUC01 | RO01, RUS01, RUS02, RP01 | PR06, PR17, PR22 |
| RUC02 | RF04, RF08, RC01, RIN01, RM01, RM02 | PR06, PR11, PR16, PR19 |
| RUC03 | RF01, RF02, RC02, RI02 | PR03, PR21 |
| RUC04 | RR01, RES01 | PR20, PR25 |
| RUR01 | RF01, RI01, RIN01, RP01, RD01 | PR01, PR16, PR18 |
| RUR02 | RF03, RE01, RC02 | PR21, PR26 |
| RUR03 | RF03, RF05, RF07, RF08, RD01 | PR07, PR10, PR11, PR26 |
| RUR04 | RE01, RE02, RR01, RES01, RI02 | PR16, PR20, PR21, PR25 |



| Caso de uso | Requerimientos funcionales | Pruebas |
| --- | --- | --- |
| CU01 | RF01 | PR01, PR02, PR03 |
| CU02 | RF02 | PR03 |
| CU03 | RF03 | PR04, PR26 |
| CU04 | RF04 | PR05, PR06 |
| CU05 | RF05, RF06 | PR07, PR08 |
| CU06 | RF07, RF08 | PR09, PR10, PR11 |
| CU07 | RF09 | PR12, PR13 |
| CU08 | RF10 | PR03, PR13, PR14 |
| CU09 | RF11 | PR15 |
| CU10 | RF01, RF02 | PR01, PR02, PR03 |
| CU11 | RF04, RF05, RF06, RF07, RF08, RF09 | PR05, PR06, PR09, PR11, PR16 |
| CU12 | RF04 | PR05 |
| CU13 | RF06 | PR08 |
| CU14 | RF08 | PR11 |
| CU15 | RF09 | PR12, PR13 |
| CU16 | RF09, RF10 | PR13 |



## 4 Pruebas del sistema

### 4.1 Pruebas de usuario y del software

El equipo de desarrollo ejecutará primero las pruebas técnicas. Después, estudiantes y docentes realizarán la aceptación de sus recorridos. La revisión clínica comprobará los resultados de los casos con una referencia aprobada. Estas actividades verifican el módulo; la evaluación educativa de 4.3 estudia su aporte al aprendizaje.

| Dato de prueba | Uso |
| --- | --- |
| EST01 y EST02 | Estudiantes ficticios del grupo G1. |
| EST03 | Estudiante ficticio del grupo G2. |
| DOC01 y DOC02 | Docentes de G1 y G2, respectivamente. |
| ADM01 | Cuenta autorizada para administrar accesos; sin permiso automático para calificar. |
| QA01 y L1 | Caso y examen de prueba con resultados esperados revisados. Los datos se prepararán antes de ejecutar las pruebas. |



Cada ejecución registrará fecha, responsable, versión del módulo, caso y rúbrica, datos utilizados, resultado obtenido y evidencia. El estado inicial es "pendiente de ejecución". Los valores de las pruebas son datos sintéticos para verificar comportamiento; no son recomendaciones clínicas ni resultados del piloto.

#### PR01 Acceso válido por rol

| Atributo · PR01 | Descripción |
| --- | --- |
| Identificador | PR01 |
| Nombre | Acceso válido por rol |
| Tipo de prueba | Integración y aceptación |
| Requerimientos y casos asociados | RF01; CU01, CU10; RI01 |
| Usuarios participantes | Estudiante, docente y administrador |
| Precondiciones y datos | Cuentas de prueba EST01, DOC01 y ADM01 habilitadas; cada una tiene un único rol y permisos conocidos. |
| Procedimiento | 1. Ingresar a SIRCLE como EST01 y abrir el módulo.<br>2. Cerrar sesión y repetir como DOC01.<br>3. Repetir como ADM01 y comprobar las opciones de cada vista. |
| Resultados esperados | 1. EST01 ve sus actividades e intentos.<br>2. DOC01 ve la supervisión del grupo G1.<br>3. ADM01 ve administración; no obtiene permiso de calificación por ser administrador.<br>4. El módulo reutiliza la sesión y no solicita una contraseña adicional. |
| Valor mínimo aceptable | Todos los resultados esperados se cumplen. Un incumplimiento deja la prueba no aceptada. |
| Resultado obtenido | No registrado: prueba pendiente de ejecución. |
| Evidencia requerida | Versión evaluada, entradas, registro de la operación y captura o reporte que permita comprobar cada resultado. |
| Estado | Pendiente de ejecución. |



#### PR02 Credenciales inválidas y sesión vencida

| Atributo · PR02 | Descripción |
| --- | --- |
| Identificador | PR02 |
| Nombre | Credenciales inválidas y sesión vencida |
| Tipo de prueba | Negativa e integración |
| Requerimientos y casos asociados | RF01; CU01, CU10 |
| Usuarios participantes | Usuario del módulo |
| Precondiciones y datos | Tres escenarios separados: cuenta inexistente, cuenta válida con contraseña incorrecta y sesión de EST01 invalidada después de abrir una actividad. |
| Procedimiento | 1. Probar cada combinación inválida en el ingreso de SIRCLE.<br>2. Comprobar que no se entra al módulo y que existe una vía de reintento o soporte de la plataforma.<br>3. Con la sesión invalidada, enviar una operación protegida desde una vista abierta.<br>4. Volver a ingresar y recuperar el intento. |
| Resultados esperados | 1. Ninguna credencial inválida concede acceso.<br>2. La sesión invalidada no permite leer ni modificar datos protegidos.<br>3. El usuario puede volver a autenticarse y conserva el último avance confirmado.<br>4. El módulo no implementa un sistema independiente de recuperación de contraseñas. |
| Valor mínimo aceptable | Todos los resultados esperados se cumplen. Un incumplimiento deja la prueba no aceptada. |
| Resultado obtenido | No registrado: prueba pendiente de ejecución. |
| Evidencia requerida | Versión evaluada, entradas, registro de la operación y captura o reporte que permita comprobar cada resultado. |
| Estado | Pendiente de ejecución. |



#### PR03 Permisos por recurso y revocación

| Atributo · PR03 | Descripción |
| --- | --- |
| Identificador | PR03 |
| Nombre | Permisos por recurso y revocación |
| Tipo de prueba | Autorización y prueba negativa |
| Requerimientos y casos asociados | RF01, RF02, RC02; CU02, CU08, CU10 |
| Usuarios participantes | ADM01, EST01, DOC01 |
| Precondiciones y datos | EST01 y EST02 pertenecen a G1; EST03 a G2. DOC01 supervisa G1; DOC02, G2. Cada estudiante tiene un intento distinto. |
| Procedimiento | 1. Como EST01, intentar abrir por identificador el intento de EST02.<br>2. Como DOC01, intentar consultar y modificar un intento de G2.<br>3. Como EST01, enviar una operación administrativa.<br>4. Como ADM01, revocar el acceso de EST01 mientras mantiene una vista abierta.<br>5. Desde esa vista, intentar guardar otra acción. |
| Resultados esperados | 1. Las tres operaciones fuera de permiso se rechazan sin exponer datos.<br>2. La revocación queda auditada con responsable y fecha.<br>3. La siguiente operación de EST01 se rechaza; su historial previo permanece. |
| Valor mínimo aceptable | Todos los resultados esperados se cumplen. Un incumplimiento deja la prueba no aceptada. |
| Resultado obtenido | No registrado: prueba pendiente de ejecución. |
| Evidencia requerida | Versión evaluada, entradas, registro de la operación y captura o reporte que permita comprobar cada resultado. |
| Estado | Pendiente de ejecución. |



#### PR04 Publicación y conservación de versiones

| Atributo · PR04 | Descripción |
| --- | --- |
| Identificador | PR04 |
| Nombre | Publicación y conservación de versiones |
| Tipo de prueba | Funcional, integración y versionado |
| Requerimientos y casos asociados | RF03, RM01; CU03 |
| Usuarios participantes | Docente |
| Precondiciones y datos | Caso ficticio QA01 en borrador; grupo G1; versión revisada V1 con rúbrica R1 y un intento iniciado después de publicarla. |
| Procedimiento | 1. Intentar publicar sin completar una regla obligatoria ni registrar revisión.<br>2. Completar el caso, registrar la revisión y asignar V1 a G1.<br>3. Iniciar un intento con EST01.<br>4. Modificar un criterio y publicar V2 para nuevos intentos.<br>5. Abrir el intento previo y crear otro con la nueva asignación. |
| Resultados esperados | 1. El borrador incompleto no se habilita.<br>2. La versión revisada se asigna al grupo autorizado.<br>3. El intento previo conserva V1 y R1; el nuevo usa la versión asignada V2.<br>4. No se recalifican automáticamente los intentos anteriores con la rúbrica nueva. |
| Valor mínimo aceptable | Todos los resultados esperados se cumplen. Un incumplimiento deja la prueba no aceptada. |
| Resultado obtenido | No registrado: prueba pendiente de ejecución. |
| Evidencia requerida | Versión evaluada, entradas, registro de la operación y captura o reporte que permita comprobar cada resultado. |
| Estado | Pendiente de ejecución. |



#### PR05 Creación y repetición sin sobrescritura

| Atributo · PR05 | Descripción |
| --- | --- |
| Identificador | PR05 |
| Nombre | Creación y repetición sin sobrescritura |
| Tipo de prueba | Persistencia e idempotencia |
| Requerimientos y casos asociados | RF04; CU04, CU11, CU12; RC01, RIN01 |
| Usuarios participantes | Estudiante |
| Precondiciones y datos | Actividad QA01 con dos intentos permitidos; una solicitud de inicio con identificador único y un intento previo con notas. |
| Procedimiento | 1. Enviar dos veces la misma solicitud de inicio.<br>2. Guardar una nota y volver al listado.<br>3. Elegir repetir y confirmar una solicitud diferente.<br>4. Consultar el intento anterior y el nuevo.<br>5. Intentar una tercera ejecución. |
| Resultados esperados | 1. El reenvío crea un solo intento.<br>2. La repetición crea otro identificador y otra ficha, con estado inicial.<br>3. Las notas y resultados del intento anterior siguen disponibles sin cambios.<br>4. La tercera ejecución se rechaza por el límite configurado. |
| Valor mínimo aceptable | Todos los resultados esperados se cumplen. Un incumplimiento deja la prueba no aceptada. |
| Resultado obtenido | No registrado: prueba pendiente de ejecución. |
| Evidencia requerida | Versión evaluada, entradas, registro de la operación y captura o reporte que permita comprobar cada resultado. |
| Estado | Pendiente de ejecución. |



#### PR06 Recuperación tras cierre de sesión

| Atributo · PR06 | Descripción |
| --- | --- |
| Identificador | PR06 |
| Nombre | Recuperación tras cierre de sesión |
| Tipo de prueba | Persistencia y aceptación |
| Requerimientos y casos asociados | RF04, RO01, RC01; CU04, CU11 |
| Usuarios participantes | Estudiante |
| Precondiciones y datos | Intento con conversación, nota, orden pendiente y tiempo simulado confirmados. Se conserva una captura de sus identificadores y valores. |
| Procedimiento | 1. Cerrar la sesión de SIRCLE y el navegador.<br>2. Volver a ingresar y elegir continuar.<br>3. Comparar los registros con la captura previa.<br>4. Consultar una operación que se hubiera intentado enviar sin recibir confirmación. |
| Resultados esperados | 1. Se recupera el mismo intento, ficha y versión de caso.<br>2. El reloj simulado no avanza por el tiempo real transcurrido fuera de la sesión.<br>3. Se recuperan todos los cambios confirmados.<br>4. La operación sin confirmar se identifica mediante su estado; no se afirma que se guardó si no existe evidencia. |
| Valor mínimo aceptable | Todos los resultados esperados se cumplen. Un incumplimiento deja la prueba no aceptada. |
| Resultado obtenido | No registrado: prueba pendiente de ejecución. |
| Evidencia requerida | Versión evaluada, entradas, registro de la operación y captura o reporte que permita comprobar cada resultado. |
| Estado | Pendiente de ejecución. |



#### PR07 Coherencia de respuestas y límites del diálogo

| Atributo · PR07 | Descripción |
| --- | --- |
| Identificador | PR07 |
| Nombre | Coherencia de respuestas y límites del diálogo |
| Tipo de prueba | Validación clínica y prueba negativa |
| Requerimientos y casos asociados | RF05, RUR03; CU05; RI02 |
| Usuarios participantes | Estudiante y docente revisor |
| Precondiciones y datos | Caso revisado con una lista de hechos conocidos, datos ausentes y pauta docente reservada; servicio de diálogo disponible y luego interrumpido. |
| Procedimiento | 1. Preguntar un hecho conocido usando dos formulaciones.<br>2. Solicitar un dato ausente y una maniobra no habilitada.<br>3. Pedir al paciente cambiar sus antecedentes o mostrar la pauta.<br>4. Interrumpir el servicio y repetir una pregunta. |
| Resultados esperados | 1. Las formulaciones conservan el mismo hecho, sin exigir texto idéntico.<br>2. Los datos ausentes se reconocen como no disponibles.<br>3. No se modifican antecedentes ni se entrega la pauta reservada.<br>4. El fallo se informa y no genera una respuesta clínica inventada. |
| Valor mínimo aceptable | Todos los resultados esperados se cumplen. Un incumplimiento deja la prueba no aceptada. |
| Resultado obtenido | No registrado: prueba pendiente de ejecución. |
| Evidencia requerida | Versión evaluada, entradas, registro de la operación y captura o reporte que permita comprobar cada resultado. |
| Estado | Pendiente de ejecución. |



#### PR08 Disponibilidad de exámenes y reenvío de órdenes

| Atributo · PR08 | Descripción |
| --- | --- |
| Identificador | PR08 |
| Nombre | Disponibilidad de exámenes y reenvío de órdenes |
| Tipo de prueba | Límites temporales e integración |
| Requerimientos y casos asociados | RF06; CU13 |
| Usuarios participantes | Estudiante |
| Precondiciones y datos | Examen de prueba L1 con resultado de referencia y disponibilidad a las 72 horas simuladas. No representa un plazo médico universal. |
| Procedimiento | 1. Solicitar L1 y reenviar la misma confirmación.<br>2. Consultar en t=0 y t=71 h 59 min.<br>3. Avanzar hasta t=72 h y consultar.<br>4. Solicitar una nueva toma con otro identificador. |
| Resultados esperados | 1. La primera solicitud y su reenvío producen una orden.<br>2. Antes de las 72 horas aparece pendiente, sin valor normal de reemplazo.<br>3. A las 72 horas aparece el resultado previsto con unidad y momento.<br>4. La toma deliberadamente nueva genera una segunda orden diferenciada. |
| Valor mínimo aceptable | Todos los resultados esperados se cumplen. Un incumplimiento deja la prueba no aceptada. |
| Resultado obtenido | No registrado: prueba pendiente de ejecución. |
| Evidencia requerida | Versión evaluada, entradas, registro de la operación y captura o reporte que permita comprobar cada resultado. |
| Estado | Pendiente de ejecución. |



#### PR09 Diferencia entre anotar y ejecutar

| Atributo · PR09 | Descripción |
| --- | --- |
| Identificador | PR09 |
| Nombre | Diferencia entre anotar y ejecutar |
| Tipo de prueba | Funcional e idempotencia |
| Requerimientos y casos asociados | RF07; CU06, CU11; RC01 |
| Usuarios participantes | Estudiante |
| Precondiciones y datos | Intervención de prueba A1 incluida en el caso y estado inicial S0; efecto inmediato S1 aprobado para la prueba. |
| Procedimiento | 1. Escribir A1 en una nota y guardarla.<br>2. Comprobar el estado del paciente.<br>3. Seleccionar A1, completar sus parámetros y confirmar.<br>4. Reenviar la misma confirmación. |
| Resultados esperados | 1. La nota queda guardada y el paciente continúa en S0.<br>2. La confirmación produce un evento y el estado previsto S1.<br>3. El reenvío no duplica el evento ni sus efectos.<br>4. La cronología distingue propuesta, indicación y ejecución. |
| Valor mínimo aceptable | Todos los resultados esperados se cumplen. Un incumplimiento deja la prueba no aceptada. |
| Resultado obtenido | No registrado: prueba pendiente de ejecución. |
| Evidencia requerida | Versión evaluada, entradas, registro de la operación y captura o reporte que permita comprobar cada resultado. |
| Estado | Pendiente de ejecución. |



#### PR10 Acciones inválidas y errores clínicos previstos

| Atributo · PR10 | Descripción |
| --- | --- |
| Identificador | PR10 |
| Nombre | Acciones inválidas y errores clínicos previstos |
| Tipo de prueba | Validación de entradas y clínica |
| Requerimientos y casos asociados | RF07, RUR03; CU06 |
| Usuarios participantes | Estudiante y docente revisor |
| Precondiciones y datos | Tres acciones de prueba: A1 sin un parámetro obligatorio, A2 fuera del caso y A3 clínicamente equivocada con consecuencia expresamente modelada. |
| Procedimiento | 1. Intentar confirmar A1 sin completar su parámetro.<br>2. Solicitar A2.<br>3. Confirmar A3 y consultar la evolución y evidencia de evaluación. |
| Resultados esperados | 1. A1 pide corrección y no se ejecuta.<br>2. A2 informa el límite del caso.<br>3. A3 se conserva como decisión del estudiante y aplica la consecuencia revisada.<br>4. El sistema no rechaza A3 como si fuera un error de formato ni la presenta como correcta. |
| Valor mínimo aceptable | Todos los resultados esperados se cumplen. Un incumplimiento deja la prueba no aceptada. |
| Resultado obtenido | No registrado: prueba pendiente de ejecución. |
| Evidencia requerida | Versión evaluada, entradas, registro de la operación y captura o reporte que permita comprobar cada resultado. |
| Estado | Pendiente de ejecución. |



#### PR11 Avance temporal consistente ante fallos

| Atributo · PR11 | Descripción |
| --- | --- |
| Identificador | PR11 |
| Nombre | Avance temporal consistente ante fallos |
| Tipo de prueba | Transacciones, concurrencia y validación clínica |
| Requerimientos y casos asociados | RF08, RC01; CU14, CU11 |
| Usuarios participantes | Estudiante y equipo de pruebas |
| Precondiciones y datos | Estado inicial conocido, orden pendiente y secuencia esperada revisada. Dos vistas del mismo intento y posibilidad de interrumpir el guardado en pruebas. |
| Procedimiento | 1. Intentar avanzar un intervalo negativo, cero y uno fuera del periodo representado.<br>2. Ejecutar un avance válido y comparar con la secuencia esperada.<br>3. Interrumpir una segunda actualización antes de confirmarla.<br>4. Desde la vista antigua, intentar otro cambio. |
| Resultados esperados | 1. Los intervalos inválidos no cambian el intento.<br>2. El avance válido modifica reloj, resultados y estado conforme a la referencia.<br>3. El fallo no deja una mezcla de tiempo nuevo y estado antiguo.<br>4. La vista desactualizada recibe un conflicto y debe recuperar la versión vigente. |
| Valor mínimo aceptable | Todos los resultados esperados se cumplen. Un incumplimiento deja la prueba no aceptada. |
| Resultado obtenido | No registrado: prueba pendiente de ejecución. |
| Evidencia requerida | Versión evaluada, entradas, registro de la operación y captura o reporte que permita comprobar cada resultado. |
| Estado | Pendiente de ejecución. |



#### PR12 Entrega y puntuación conocida

| Atributo · PR12 | Descripción |
| --- | --- |
| Identificador | PR12 |
| Nombre | Entrega y puntuación conocida |
| Tipo de prueba | Aceptación y cálculo |
| Requerimientos y casos asociados | RF09; CU07, CU15; RO01 |
| Usuarios participantes | Estudiante y docente |
| Precondiciones y datos | Rúbrica de prueba de 100 puntos con un criterio de 10 puntos; intento A cumple todos y B omite ese criterio. Estos valores verifican el cálculo, no fijan la rúbrica académica definitiva. |
| Procedimiento | 1. Entregar A y repetir la confirmación.<br>2. Entregar B con documentación obligatoria completa.<br>3. Consultar puntuaciones y explicación por criterio.<br>4. Intentar modificar la evidencia entregada como estudiante. |
| Resultados esperados | 1. A tiene una entrega y obtiene 100/100.<br>2. B obtiene 90/100; la omisión evaluable no impide entregar.<br>3. Cada descuento remite al criterio y evidencia correspondiente.<br>4. El estudiante no puede modificar la evidencia entregada. |
| Valor mínimo aceptable | Todos los resultados esperados se cumplen. Un incumplimiento deja la prueba no aceptada. |
| Resultado obtenido | No registrado: prueba pendiente de ejecución. |
| Evidencia requerida | Versión evaluada, entradas, registro de la operación y captura o reporte que permita comprobar cada resultado. |
| Estado | Pendiente de ejecución. |



#### PR13 Evaluación pendiente y revisión docente

| Atributo · PR13 | Descripción |
| --- | --- |
| Identificador | PR13 |
| Nombre | Evaluación pendiente y revisión docente |
| Tipo de prueba | Recuperación, autorización y auditoría |
| Requerimientos y casos asociados | RF09, RF10; CU08, CU15, CU16 |
| Usuarios participantes | Docente y estudiante |
| Precondiciones y datos | Una entrega con criterio manual pendiente y otra con fallo del evaluador; política de publicación docente; cuenta de docente ajeno al grupo. |
| Procedimiento | 1. Confirmar que ambas entregas permanecen registradas como pendientes.<br>2. Intentar publicar una nota definitiva con criterio pendiente.<br>3. Completar el criterio como DOC01, indicando puntuación y motivo.<br>4. Intentar un valor fuera de rango y una rectificación sin motivo.<br>5. Publicar la revisión válida y consultarla como estudiante.<br>6. Intentar modificarla como DOC02. |
| Resultados esperados | 1. No se asigna cero por el fallo ni se pierde la entrega.<br>2. La publicación definitiva exige resolver los criterios pendientes.<br>3. Se conservan versión anterior, nueva valoración, autor y motivo.<br>4. Los valores inválidos y el docente ajeno se rechazan.<br>5. El estudiante recibe solo la devolución autorizada. |
| Valor mínimo aceptable | Todos los resultados esperados se cumplen. Un incumplimiento deja la prueba no aceptada. |
| Resultado obtenido | No registrado: prueba pendiente de ejecución. |
| Evidencia requerida | Versión evaluada, entradas, registro de la operación y captura o reporte que permita comprobar cada resultado. |
| Estado | Pendiente de ejecución. |



#### PR14 Panel docente y tiempo activo

| Atributo · PR14 | Descripción |
| --- | --- |
| Identificador | PR14 |
| Nombre | Panel docente y tiempo activo |
| Tipo de prueba | Cálculo y aceptación |
| Requerimientos y casos asociados | RF10, RO02; CU08 |
| Usuarios participantes | Docente |
| Precondiciones y datos | G1 contiene dos estudiantes y tres intentos: uno evaluado con 80, otro con 60 y otro pendiente. Registro de actividad con dos intervalos activos de 30 y 45 segundos, separados por una pausa explícita. |
| Procedimiento | 1. Filtrar por G1 y por la actividad de prueba.<br>2. Consultar personas, intentos, entregas y media de notas completas.<br>3. Abrir los detalles que respaldan el resumen.<br>4. Consultar el tiempo activo y el tiempo clínico. |
| Resultados esperados | 1. Se muestran dos estudiantes y tres intentos.<br>2. La media de las dos evaluaciones completas es 70; el pendiente se informa aparte.<br>3. Los totales coinciden con el detalle.<br>4. El tiempo activo de la secuencia es 75 segundos; la pausa no se suma y el reloj clínico se presenta por separado. |
| Valor mínimo aceptable | Todos los resultados esperados se cumplen. Un incumplimiento deja la prueba no aceptada. |
| Resultado obtenido | No registrado: prueba pendiente de ejecución. |
| Evidencia requerida | Versión evaluada, entradas, registro de la operación y captura o reporte que permita comprobar cada resultado. |
| Estado | Pendiente de ejecución. |



#### PR15 Encuestas y mediciones emparejadas

| Atributo · PR15 | Descripción |
| --- | --- |
| Identificador | PR15 |
| Nombre | Encuestas y mediciones emparejadas |
| Tipo de prueba | Secuencia, persistencia y aceptación |
| Requerimientos y casos asociados | RF11; CU09 |
| Usuarios participantes | Estudiante y responsable de evaluación |
| Precondiciones y datos | Instrumento V1 previo y posterior; EST01 responde ambos y EST02 omite el previo. Una respuesta posterior se reenvía con la misma solicitud. |
| Procedimiento | 1. Responder el previo de EST01 antes de iniciar y el posterior después de entregar.<br>2. Omitir el previo de EST02 y continuar la actividad.<br>3. Intentar responder su previo después de comenzar.<br>4. Reenviar el posterior ya confirmado.<br>5. Consultar el conjunto de respuestas y las notas clínicas. |
| Resultados esperados | 1. EST01 forma un par completo con versión y momentos identificados.<br>2. EST02 queda con dato inicial faltante y puede practicar.<br>3. La respuesta tardía no se etiqueta como previa.<br>4. El reenvío no duplica respuestas y las encuestas no cambian notas clínicas. |
| Valor mínimo aceptable | Todos los resultados esperados se cumplen. Un incumplimiento deja la prueba no aceptada. |
| Resultado obtenido | No registrado: prueba pendiente de ejecución. |
| Evidencia requerida | Versión evaluada, entradas, registro de la operación y captura o reporte que permita comprobar cada resultado. |
| Estado | Pendiente de ejecución. |



#### PR16 Intercambio con SIRCLE y pérdida de conexión

| Atributo · PR16 | Descripción |
| --- | --- |
| Identificador | PR16 |
| Nombre | Intercambio con SIRCLE y pérdida de conexión |
| Tipo de prueba | Contrato de integración y recuperación |
| Requerimientos y casos asociados | RI01, RI02, RIN01, RE02; CU11 |
| Usuarios participantes | Equipo técnico y estudiante |
| Precondiciones y datos | Contrato de campos aprobado, ficha de prueba e intento vinculados; una unidad y un estado incompatibles preparados como entradas negativas. |
| Procedimiento | 1. Transferir un registro válido y comprobar identificadores, unidad, tiempo y autor en ambos extremos.<br>2. Enviar el registro incompatible.<br>3. Interrumpir la conexión después de enviar una operación y antes de recibir su confirmación.<br>4. Recuperar la conexión y consultar o reintentar usando el mismo identificador. |
| Resultados esperados | 1. El registro válido conserva su significado y se vincula a la ficha correcta.<br>2. El incompatible se rechaza con un error identificable.<br>3. La interfaz indica incertidumbre o pendiente mientras no exista confirmación.<br>4. La recuperación deja una operación confirmada, sin duplicación ni afirmaciones falsas de sincronización. |
| Valor mínimo aceptable | Todos los resultados esperados se cumplen. Un incumplimiento deja la prueba no aceptada. |
| Resultado obtenido | No registrado: prueba pendiente de ejecución. |
| Evidencia requerida | Versión evaluada, entradas, registro de la operación y captura o reporte que permita comprobar cada resultado. |
| Estado | Pendiente de ejecución. |



#### PR17 Tareas comprensibles y navegación por teclado

| Atributo · PR17 | Descripción |
| --- | --- |
| Identificador | PR17 |
| Nombre | Tareas comprensibles y navegación por teclado |
| Tipo de prueba | Usabilidad y accesibilidad funcional |
| Requerimientos y casos asociados | RUS01, RUS02; CU04, CU05, CU06, CU07, CU08 |
| Usuarios participantes | Estudiantes y docentes participantes del piloto |
| Precondiciones y datos | Tareas escritas: retomar un caso, consultar un hallazgo, guardar una nota, entregar y revisar un resultado. Muestra de participantes acordada antes del piloto. |
| Procedimiento | 1. Pedir a cada participante realizar las tareas correspondientes a su rol.<br>2. Repetir el recorrido usando teclado para controles y confirmaciones.<br>3. Provocar un campo obligatorio vacío y observar si puede localizarlo y corregirlo.<br>4. Registrar éxito, ayuda requerida, errores y comentarios por tarea. |
| Resultados esperados | 1. Los controles tienen etiqueta y foco visible; no existen bloqueos de teclado en las tareas principales.<br>2. Los errores identifican el campo y conservan lo ya introducido.<br>3. Se registra evidencia por participante y tarea, sin declarar usabilidad por opinión del desarrollador. |
| Valor mínimo aceptable | Cero bloqueos que impidan terminar las tareas principales. El porcentaje objetivo de realización sin ayuda se fija con el equipo docente antes del piloto y se informa junto con el tamaño de la muestra. |
| Resultado obtenido | No registrado: prueba pendiente de ejecución. |
| Evidencia requerida | Versión evaluada, entradas, registro de la operación y captura o reporte que permita comprobar cada resultado. |
| Estado | Pendiente de ejecución. |



#### PR18 Conservación de funciones existentes de SIRCLE

| Atributo · PR18 | Descripción |
| --- | --- |
| Identificador | PR18 |
| Nombre | Conservación de funciones existentes de SIRCLE |
| Tipo de prueba | Regresión e integración |
| Requerimientos y casos asociados | RIN01, RUR01; CU01, CU04, CU06, CU07 |
| Usuarios participantes | Usuario de SIRCLE y equipo técnico |
| Precondiciones y datos | Guion de referencia del registro actual: paciente ficticio, signos vitales, indicaciones, anotaciones y finalización de ficha. Ambiente de pruebas con y sin el módulo habilitado. |
| Procedimiento | 1. Completar el guion existente con el módulo deshabilitado y conservar resultados de referencia.<br>2. Habilitar el módulo y repetir con otra ficha de prueba.<br>3. Realizar una simulación y revisar qué ocurre al cerrar su ficha y al entregar la actividad. |
| Resultados esperados | 1. Los registros y cálculos existentes mantienen sus resultados para los mismos datos.<br>2. Las fichas de ambos recorridos permanecen separadas.<br>3. Cerrar la ficha no publica una evaluación por accidente y la entrega académica conserva su evidencia. |
| Valor mínimo aceptable | Todos los resultados esperados se cumplen. Un incumplimiento deja la prueba no aceptada. |
| Resultado obtenido | No registrado: prueba pendiente de ejecución. |
| Evidencia requerida | Versión evaluada, entradas, registro de la operación y captura o reporte que permita comprobar cada resultado. |
| Estado | Pendiente de ejecución. |



#### PR19 Restauración de registros y vínculos

| Atributo · PR19 | Descripción |
| --- | --- |
| Identificador | PR19 |
| Nombre | Restauración de registros y vínculos |
| Tipo de prueba | Recuperación y confiabilidad |
| Requerimientos y casos asociados | RC01, RM02; CU04 |
| Usuarios participantes | Equipo técnico |
| Precondiciones y datos | Copia de prueba que contiene casos versionados, intentos, fichas vinculadas y evaluaciones. Entorno de restauración aislado. |
| Procedimiento | 1. Registrar conteos e identificadores antes de respaldar.<br>2. Restaurar la copia en el entorno aislado.<br>3. Abrir un intento en curso y uno entregado.<br>4. Comparar vínculos, versiones, cronología y evaluación con la referencia. |
| Resultados esperados | 1. Se recuperan todos los registros presentes en la copia, con sus relaciones.<br>2. El intento en curso puede retomarse y el entregado conserva su evaluación.<br>3. Se documenta el tiempo de restauración medido y la fecha de la copia; no se afirma recuperar datos posteriores al respaldo. |
| Valor mínimo aceptable | Todos los resultados esperados se cumplen. Un incumplimiento deja la prueba no aceptada. |
| Resultado obtenido | No registrado: prueba pendiente de ejecución. |
| Evidencia requerida | Versión evaluada, entradas, registro de la operación y captura o reporte que permita comprobar cada resultado. |
| Estado | Pendiente de ejecución. |



#### PR20 Tiempos de respuesta del piloto

| Atributo · PR20 | Descripción |
| --- | --- |
| Identificador | PR20 |
| Nombre | Tiempos de respuesta del piloto |
| Tipo de prueba | Rendimiento |
| Requerimientos y casos asociados | RR01, RE01 |
| Usuarios participantes | Equipo técnico |
| Precondiciones y datos | Perfil propuesto: 20 usuarios concurrentes durante 15 minutos, datos de prueba y configuración del ambiente registrada. Al menos 100 operaciones por categoría medida. |
| Procedimiento | 1. Ejecutar aperturas, guardados, respuestas de diálogo y avances temporales con el perfil definido.<br>2. Medir de envío a respuesta visible; separar categorías y registrar dependencias externas.<br>3. Calcular percentil 95 y proporción de fallos.<br>4. Repetir después de corregir un incumplimiento. |
| Resultados esperados | 1. Objetivos propuestos de p95: apertura 2 s, guardado 3 s, diálogo 15 s y avance 5 s.<br>2. Al menos 99 % de operaciones de prueba completadas, excluyendo errores de entrada deliberados.<br>3. Ninguna operación fallida pierde datos confirmados ni comunica un éxito inexistente. |
| Valor mínimo aceptable | Cumplir el perfil y los umbrales propuestos una vez ratificados con el equipo. Estos valores son objetivos de aceptación, no mediciones ni capacidad demostrada de SIRCLE. |
| Resultado obtenido | No registrado: prueba pendiente de ejecución. |
| Evidencia requerida | Versión evaluada, entradas, registro de la operación y captura o reporte que permita comprobar cada resultado. |
| Estado | Pendiente de ejecución. |



#### PR21 Configuración del ambiente y secretos

| Atributo · PR21 | Descripción |
| --- | --- |
| Identificador | PR21 |
| Nombre | Configuración del ambiente y secretos |
| Tipo de prueba | Entorno y seguridad |
| Requerimientos y casos asociados | RE01, RC02, RI02 |
| Usuarios participantes | Equipo técnico y administrador |
| Precondiciones y datos | Ambiente separado de producción; configuración documentada; servicio de conversación de prueba y credenciales configuradas del lado del servidor. |
| Procedimiento | 1. Revisar URL, almacenamiento, versiones, CPU, memoria y conectividad del ambiente.<br>2. Inspeccionar respuestas y archivos enviados al navegador para comprobar que no contienen claves del servicio.<br>3. Invalidar la credencial del servicio en pruebas y solicitar una respuesta.<br>4. Comprobar que el acceso y el registro confirmado del estudiante siguen protegidos. |
| Resultados esperados | 1. El ambiente es reproducible y no utiliza fichas de pacientes reales.<br>2. Las claves del servicio no llegan al navegador ni aparecen en mensajes de error.<br>3. El fallo del servicio se informa sin perder el intento ni inventar una respuesta clínica. |
| Valor mínimo aceptable | Todos los resultados esperados se cumplen. Un incumplimiento deja la prueba no aceptada. |
| Resultado obtenido | No registrado: prueba pendiente de ejecución. |
| Evidencia requerida | Versión evaluada, entradas, registro de la operación y captura o reporte que permita comprobar cada resultado. |
| Estado | Pendiente de ejecución. |



#### PR22 Compatibilidad de interfaz

| Atributo · PR22 | Descripción |
| --- | --- |
| Identificador | PR22 |
| Nombre | Compatibilidad de interfaz |
| Tipo de prueba | Portabilidad y usabilidad |
| Requerimientos y casos asociados | RP01, RUS02 |
| Usuarios participantes | Equipo de pruebas |
| Precondiciones y datos | Matriz inicial propuesta: versiones estables de Chrome, Edge y Firefox disponibles al ejecutar la prueba, sobre los equipos docentes previstos; se registran versión y resolución exactas. |
| Procedimiento | 1. Ejecutar ingreso, selección, entrevista, registro, entrega y consulta de resultados en cada combinación admitida.<br>2. Comprobar campos, mensajes, tablas y navegación por teclado.<br>3. Repetir con una ventana reducida y con ampliación de texto. |
| Resultados esperados | 1. Las funciones principales se mantienen y no hay controles inaccesibles por recorte.<br>2. Se conservan etiquetas, unidades y mensajes de error.<br>3. Las combinaciones que fallen no se declaran compatibles hasta corregirlas o acordar explícitamente su exclusión. |
| Valor mínimo aceptable | Todos los resultados esperados se cumplen. Un incumplimiento deja la prueba no aceptada. |
| Resultado obtenido | No registrado: prueba pendiente de ejecución. |
| Evidencia requerida | Versión evaluada, entradas, registro de la operación y captura o reporte que permita comprobar cada resultado. |
| Estado | Pendiente de ejecución. |



#### PR23 Actualización mantenible de casos

| Atributo · PR23 | Descripción |
| --- | --- |
| Identificador | PR23 |
| Nombre | Actualización mantenible de casos |
| Tipo de prueba | Mantención y regresión |
| Requerimientos y casos asociados | RM01 |
| Usuarios participantes | Docente y equipo técnico |
| Precondiciones y datos | Caso V1 en uso y cambio previsto en una regla o rúbrica; pruebas de referencia con resultados esperados. |
| Procedimiento | 1. Crear V2 conservando V1.<br>2. Ejecutar las pruebas del caso actualizado y las de un caso no modificado.<br>3. Publicar V2 en el ambiente de pruebas.<br>4. Recuperar intentos de V1 y registrar los cambios introducidos. |
| Resultados esperados | 1. La modificación no obliga a cambiar reglas de otros casos.<br>2. Los resultados esperados de los casos no modificados se mantienen.<br>3. Las versiones y su documentación permiten conocer qué cambió y conservar intentos previos. |
| Valor mínimo aceptable | Todos los resultados esperados se cumplen. Un incumplimiento deja la prueba no aceptada. |
| Resultado obtenido | No registrado: prueba pendiente de ejecución. |
| Evidencia requerida | Versión evaluada, entradas, registro de la operación y captura o reporte que permita comprobar cada resultado. |
| Estado | Pendiente de ejecución. |



#### PR24 Documentación de la integración

| Atributo · PR24 | Descripción |
| --- | --- |
| Identificador | PR24 |
| Nombre | Documentación de la integración |
| Tipo de prueba | Inspección y reproducibilidad |
| Requerimientos y casos asociados | RD01 |
| Usuarios participantes | Equipo técnico receptor y docente revisor |
| Precondiciones y datos | Contrato de integración, configuración del ambiente, catálogo de reglas y rúbricas, y registro de pruebas entregados para revisión. |
| Procedimiento | 1. Seguir el procedimiento documentado para vincular una ficha de prueba y un intento.<br>2. Verificar campos, errores, unidades y significado de cierre.<br>3. Localizar la regla y rúbrica asociadas a un caso.<br>4. Revisar un registro de prueba con esperado, obtenido, versión y evidencia. |
| Resultados esperados | 1. Otro integrante puede identificar las operaciones y reproducir el recorrido.<br>2. La documentación permite rastrear los resultados del caso y sus cambios.<br>3. La ayuda describe el módulo cuando corresponde; no exige rehacer el manual general de SIRCLE. |
| Valor mínimo aceptable | Todos los resultados esperados se cumplen. Un incumplimiento deja la prueba no aceptada. |
| Resultado obtenido | No registrado: prueba pendiente de ejecución. |
| Evidencia requerida | Versión evaluada, entradas, registro de la operación y captura o reporte que permita comprobar cada resultado. |
| Estado | Pendiente de ejecución. |



#### PR25 Aumento de concurrencia con aislamiento

| Atributo · PR25 | Descripción |
| --- | --- |
| Identificador | PR25 |
| Nombre | Aumento de concurrencia con aislamiento |
| Tipo de prueba | Escalabilidad y carga |
| Requerimientos y casos asociados | RES01, RC01 |
| Usuarios participantes | Equipo técnico |
| Precondiciones y datos | Usuarios e intentos separados. Cargas escalonadas propuestas de 5, 10 y 20 usuarios, manteniendo el tipo de operación y los datos de referencia. |
| Procedimiento | 1. Ejecutar cada escalón y registrar recursos, errores y latencia por operación.<br>2. Repetir solicitudes simultáneas sobre intentos diferentes.<br>3. Comprobar que el incremento de carga no mezcla registros ni duplica acciones.<br>4. Comparar el escalón de 20 usuarios con RR01. |
| Resultados esperados | 1. Cada estudiante conserva su intento y sus resultados.<br>2. Se obtiene un informe del comportamiento ante cada carga, incluyendo límites observados.<br>3. El escalón de aceptación cumple RR01; si falla, se registra la capacidad observada y el trabajo necesario antes del piloto. |
| Valor mínimo aceptable | Todos los resultados esperados se cumplen. Un incumplimiento deja la prueba no aceptada. |
| Resultado obtenido | No registrado: prueba pendiente de ejecución. |
| Evidencia requerida | Versión evaluada, entradas, registro de la operación y captura o reporte que permita comprobar cada resultado. |
| Estado | Pendiente de ejecución. |



#### PR26 Datos ficticios y revisión de contenido

| Atributo · PR26 | Descripción |
| --- | --- |
| Identificador | PR26 |
| Nombre | Datos ficticios y revisión de contenido |
| Tipo de prueba | Inspección y validación docente |
| Requerimientos y casos asociados | RF03, RUR02, RUR03; CU03 |
| Usuarios participantes | Docente revisor |
| Precondiciones y datos | Borrador con ficha ficticia declarada, fuentes clínicas públicas, reglas de evolución y rúbrica; contenido preparado manualmente o con asistencia de IA. |
| Procedimiento | 1. Revisar que los identificadores y antecedentes corresponden a un paciente ficticio.<br>2. Contrastar objetivos, hallazgos y evolución con las referencias y criterios docentes.<br>3. Dejar sin aprobación un borrador con una regla no sustentada.<br>4. Intentar habilitarlo para estudiantes y después repetir tras corregir y registrar la revisión. |
| Resultados esperados | 1. La publicación exige revisión registrada y una ficha ficticia.<br>2. El borrador no revisado permanece fuera de las actividades disponibles.<br>3. La aprobación identifica quién revisó y qué versión se autorizó; no implica que la IA pueda publicar casos por sí sola. |
| Valor mínimo aceptable | Todos los resultados esperados se cumplen. Un incumplimiento deja la prueba no aceptada. |
| Resultado obtenido | No registrado: prueba pendiente de ejecución. |
| Evidencia requerida | Versión evaluada, entradas, registro de la operación y captura o reporte que permita comprobar cada resultado. |
| Estado | Pendiente de ejecución. |



Las pruebas de autorización, pérdida de registros o incoherencia clínica impiden aceptar el módulo mientras tengan fallos sin resolver. Cada resultado debe compararse con su criterio; la existencia de una pantalla no demuestra el cumplimiento del requerimiento.

### 4.2 Relación entre requerimientos de usuario y pruebas

| ID | Necesidad comprobada | Pruebas |
| --- | --- | --- |
| RU01 | Acceso según responsabilidad | PR01, PR02, PR03 |
| RU02 | Actividades acordes a la formación | PR04, PR23, PR26 |
| RU03 | Valoración y seguimiento del paciente | PR07, PR08, PR09, PR10, PR11 |
| RU04 | Continuidad y repetición de la práctica | PR05, PR06, PR19 |
| RU05 | Evaluación comprensible y supervisión | PR12, PR13, PR14 |
| RU06 | Evaluación de la experiencia educativa | PR14, PR15 |
| RUC01 | Comprensión y facilidad de uso | PR06, PR17, PR22 |
| RUC02 | Coherencia y conservación del trabajo | PR06, PR11, PR16, PR19 |
| RUC03 | Protección de información académica | PR03, PR21 |
| RUC04 | Respuesta oportuna durante la práctica | PR20, PR25 |
| RUR01 | Integración con la plataforma existente | PR01, PR16, PR18 |
| RUR02 | Uso educativo de pacientes ficticios | PR21, PR26 |
| RUR03 | Límites clínicos y revisión docente | PR07, PR10, PR11, PR26 |
| RUR04 | Dependencia del ambiente acordado | PR16, PR20, PR21, PR25 |



La relación detallada de software con pruebas está incluida en cada ficha de 3.2. Una misma prueba puede verificar varias condiciones relacionadas, pero debe conservar evidencia de cada resultado esperado.

### 4.3 Evaluación educativa



La evaluación deberá comprobar qué aporta la actividad al aprendizaje. El estudio de Padilha y colaboradores midió conocimiento y razonamiento, junto con satisfacción y autoeficacia; sus resultados fueron diferentes entre dimensiones. Esto respalda medirlas por separado, sin trasladar sus resultados directamente a este proyecto. [Ensayo de simulación clínica virtual](https://www.jmir.org/2019/3/e11529/).

| Dimensión | Información que se recogerá | Criterio de interpretación |
| --- | --- | --- |
| Confianza clínica | Respuestas a un mismo instrumento antes y después, con identificación de los pares completos. | Describe autopercepción; no sustituye la observación del desempeño. |
| Desempeño clínico simulado | Rúbrica de valoración, razonamiento, diagnóstico, pertinencia de acciones, seguimiento y documentación. | Un docente debe revisar los criterios y ponderaciones. Si se comparan actividades, deben tener dificultad y objetivos comparables. |
| Compromiso y experiencia | Finalización, intentos, tiempo activo estimado y comentarios del estudiante. | Completar más intentos o permanecer más tiempo no demuestra por sí solo un mejor aprendizaje. |

Se deberán acordar el instrumento, los participantes y las condiciones de uso académico o de investigación. No se establece una mejora porcentual anticipada. Para atribuir cambios a la intervención se necesitará un diseño de evaluación que contemple otras explicaciones, además de comparar el antes y el después.



### 4.4 Acuerdos necesarios antes de implementar



El profesor y el equipo de SIRCLE deberán resolver los siguientes puntos:

1. Mecanismo de sesión y permisos, responsabilidad de administrar grupos y funciones ya disponibles para docentes y administradores.
2. Relación entre ficha, paciente ficticio e intento, incluida la protección del historial al crear un nuevo paciente y el significado del cierre actual.
3. Campos y acciones que desencadenarán eventos, reglas temporales y responsable de revisar cada caso y rúbrica.
4. Momento de publicación de resultados, criterios de revisión docente e instrumentos de evaluación de la experiencia.
5. Ambiente de pruebas, navegadores, perfil de carga, umbrales de rendimiento y alojamiento del componente de IA.



Los objetivos cuantitativos de RR01 y RES01, el tratamiento de inactividad y la muestra de usabilidad deberán ratificarse antes de ejecutar el piloto. Una modificación posterior debe quedar registrada junto con el motivo y las pruebas afectadas.

## 5 Referencias públicas

1. Universidad de La Frontera. [Convocatoria 2025 de proyectos de docencia](https://docencia.ufro.cl/convocatoria-2025/). Identificación institucional de la plataforma SIRCLE.
2. Kaplan. [i-Human Patients Virtual Patient Simulation](https://kaplan.com/institutions-educators/higher-education/nursing-schools/i-human). Referencia de funciones educativas, seguimiento y programas destinatarios.
3. [First-year medical students' calibration bias and accuracy across clinical reasoning activities](https://pmc.ncbi.nlm.nih.gov/articles/PMC6775028/). Estudio sobre autovaloración y desempeño con pacientes virtuales.
4. Padilha, J. M. y colaboradores (2019). [Clinical Virtual Simulation in Nursing Education: Randomized Controlled Trial](https://www.jmir.org/2019/3/e11529/). DOI: 10.2196/11529.
5. Weston, J. y Zauche, L. H. (2021). [Comparison of Virtual Simulation to Clinical Practice for Prelicensure Nursing Students in Pediatrics](https://pmc.ncbi.nlm.nih.gov/articles/PMC8395966/). Referencia para el uso de i-Human acompañado de preparación y discusión docente.

6. Proyecto 2do Quatrimestre. [Casos de uso](https://gss-mtig.github.io/proy2Q/casos_uso/). Referencia de la estructura de especificación: actores, condiciones, flujo y relaciones con requerimientos.

7. Visual Paradigm. [UML Use Case Diagram Notations Guide](https://www.visual-paradigm.com/support/documents/vpuserguide/94/2575/84257_usecasediagr.html). Notación de actores, límites, inclusiones y extensiones.

8. Visual Paradigm. [What is Use Case Diagram](https://www.visual-paradigm.com/guide/uml-unified-modeling-language/what-is-use-case-diagram/). Referencia del modelo de interacción y sus relaciones.
