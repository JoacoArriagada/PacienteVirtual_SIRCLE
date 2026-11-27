# Arranque y operación local

## Aplicación con Docker Compose

Necesitas Docker con Compose. La primera preparación descarga las imágenes, sus dependencias y el modelo `qwen3.5:4b`. Puedes preparar estos artefactos con antelación para una demostración sin conexión.

Ejecuta los siguientes comandos desde la raíz del repositorio.

```sh
docker compose up --build -d
docker compose ps -a
docker compose logs sqlite-init inferencia
```

`sqlite-init` termina con código cero después de migrar SQLite e importar las 14 fuentes. Abre `http://127.0.0.1:8080` para entrar con una identidad ficticia. Consulta `http://127.0.0.1:8080/api/estado` y comprueba que `base.disponible` y `modelo.disponible` valen `true` antes de usar la entrevista libre.

Si la preparación de Ollama falla, consulta `docker compose logs inferencia`. Tras resolver la causa indicada, ejecuta `docker compose restart inferencia` y vuelve a comprobar el estado. `docker compose stop` detiene la aplicación y conserva los volúmenes `sircle-datos` y `sircle-pesos`.

Si el estado indica que el modelo está disponible y una pregunta libre devuelve 503, consulta `docker compose logs backend inferencia`. Un mensaje de tiempo de espera en `/api/chat` indica que la generación superó el plazo configurado.

## Demo estática y desarrollo del frontend

Con Node.js 24, ejecuta estos comandos desde `frontend/`.

```sh
npm ci
npm run dev:ejemplo
```

La demo abre en `http://127.0.0.1:4173/simulacion` y usa datos de demostración locales. La hoja compartida se sirve en `/app/styles.css`. Los iconos e imágenes se sirven en `/recursos/simulacion/`.

Para trabajar con el backend nativo, prepara y sirve SQLite desde `backend/` con Rust instalado. Configura `OLLAMA_URL` si Ollama usa otro endpoint local.

```sh
cargo run --locked --bin sircle -- preparar --db target/demo.sqlite --fuentes ../dataset --rubrica-ensayo
cargo run --locked --bin sircle -- servir --db target/demo.sqlite
```

En otra terminal, ejecuta `npm ci` y `npm run dev:integrado` desde `frontend/`. Este servidor abre en `http://127.0.0.1:8080` y reenvía la API al backend local en el puerto 8081.

## Actividad formal de LR

1. Obtén el borrador con `docker compose exec -T backend sircle material-formal --db /var/lib/sircle/sircle.sqlite`.
2. Guarda el objeto `actividad` en `privado/actividad-formal.local.json` y la rúbrica en `privado/rubricas/`, con el nombre de archivo que indica la actividad. El espacio `privado/` se usa para el material de aprobación local.
3. Después de revisar ficha, instrumento y rúbrica, copia los archivos al contenedor y ejecuta `huellas-formales` para obtener sus huellas definitivas. Registra la aprobación docente en `clinica.aprobacion`, `instrumento.aprobacion` y `aprobacion` de la rúbrica. Cada constancia incluye `tipo` con valor `formal`, `docente_id`, `fecha` en UTC, `motivo`, `codigo`, `version` y `sha256`.
4. Copia de nuevo los archivos con las constancias y ejecuta `preparar` para validar y cargar la actividad.

```sh
docker compose exec -T backend mkdir -p /tmp/aprobacion-lr/rubricas
docker compose cp privado/actividad-formal.local.json backend:/tmp/aprobacion-lr/
docker compose cp privado/rubricas/. backend:/tmp/aprobacion-lr/rubricas/
docker compose exec -T backend sircle huellas-formales --archivo /tmp/aprobacion-lr/actividad-formal.local.json
docker compose exec -T backend sircle preparar --db /var/lib/sircle/sircle.sqlite --fuentes /datos --actividad-formal /tmp/aprobacion-lr/actividad-formal.local.json
```

El preparador fija las versiones aprobadas para los intentos nuevos. Los intentos previos conservan las versiones con las que se abrieron.
