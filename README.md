# Paciente Virtual SIRCLE

El módulo 01 permite practicar la entrevista y la valoración de LR, registrar hallazgos, justificar una hipótesis y revisar la devolución docente. Incluye una aplicación integrada y una demo estática.

## Aplicación integrada

Desde la raíz del repositorio, ejecuta `docker compose up --build` y abre `http://127.0.0.1:8080`. La guía de arranque explica cómo comprobar SQLite y Ollama y cómo cargar material aprobado. [Guía de arranque](docs/arranque.md)

## Demo local

Desde `frontend/`, ejecuta `npm ci` y `npm run dev:ejemplo`. Abre `http://127.0.0.1:4173/simulacion`. La demo y la aplicación integrada cargan `/app/styles.css`.

## Material del módulo

- [Diseño del módulo 01](docs/modulo-01/diseno.md)
- [Instrumento de evaluación educativa antes y después](docs/modulo-01/instrumento-pre-post.md)
- [Catálogo MedSP1000 y fuentes de los casos](dataset/README.md)
- [Recursos audiovisuales y licencias](docs/recursos-audiovisuales/README.md)
