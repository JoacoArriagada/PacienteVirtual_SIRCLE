# T013 · Inferencia local en CPU

2026-11-25 · `feature/prototipo` · revisión base `fbc3f1ca608d34482d19d72c32bb06dce796a185`

## Preparación de la estación

El equipo de desarrollo usa un Intel Core i3-7020U con dos núcleos y cuatro hilos,
12 423 127 040 bytes de RAM e Intel HD Graphics 620. Docker Engine 29.10.1 y
Compose v5.8.0 están instalados. Ejecutar el contenedor sin dispositivos GPU y
publicar su API solo en loopback. El volumen conserva los pesos entre reinicios.

```sh
docker pull ollama/ollama:latest
docker run -d --name sircle-ollama-h1 \
  -p 127.0.0.1:11434:11434 \
  -v sircle-h1-pesos:/root/.ollama \
  -e OLLAMA_NUM_PARALLEL=1 -e OLLAMA_CONTEXT_LENGTH=2048 \
  -e OLLAMA_VULKAN=0 ollama/ollama:latest
docker exec sircle-ollama-h1 ollama pull qwen3.5:4b
curl -fsS https://registry.ollama.ai/v2/library/qwen3.5/manifests/4b | sha256sum
curl -fsS http://127.0.0.1:11434/api/tags
```

Comparar el digest local de la etiqueta con los bytes del manifiesto oficial y
con el digest fijado por el adaptador. [Ollama en Docker](https://docs.ollama.com/docker)
· [Qwen 3.5:4b](https://ollama.com/library/qwen3.5:4b) · [Inventario de modelos](https://docs.ollama.com/api/tags)

## Petición desde el backend

Desde `backend/`, actualizar la base con el intento técnico de T008 y ejecutar
Axum en otra terminal. El modo `servir` usa Ollama en loopback por defecto.

```sh
cargo test --locked
cargo run --locked --bin sircle -- preparar --db target/demo.sqlite --fuentes ../datos
cargo run --locked --bin sircle -- servir --db target/demo.sqlite
```

Entrar por `POST /api/sesiones` con `{"identidad":"estudiante-lr"}`. Conservar la
cookie HttpOnly para enviar `POST /api/intentos/<intento_id>/turnos` con el ID de
T008, `Idempotency-Key: t013-ollama-cpu` y el siguiente JSON.

```json
{"texto":"¿En qué momento empezaste a tener problemas para respirar?"}
```

Se espera que la respuesta del backend identifique la misma operación que el
registro `SIRCLE_INFERENCIA`, con una selección real de `lr-inicio` y dos
candidatos en la solicitud a Ollama. Conservar identificador, inicio, fin,
modelo, digest y respuesta cruda. Comprobar `docker exec sircle-ollama-h1 ollama ps`
y `GET /api/ps`. El modelo debe registrar `100% CPU` y `size_vram=0`.
[Contrato de chat de Ollama](https://docs.ollama.com/api/chat)

## Resultado observado

La imagen oficial resolvió al manifiesto
`sha256:1cabd20e1762a7a8444bf4a7110b346d923cd77ab6c97c984434140638c913ec`.
Ollama 0.40.18 verificó la descarga de los pesos. El digest local coincidió con
el manifiesto de `qwen3.5:4b`. El ensayo por HTTP devolvió 200 y la operación
`5ec73cdcf80d474a3d88824532e08687`. La respuesta cruda de Qwen seleccionó
`lr-inicio` entre los dos candidatos enviados. La misma operación figura en el
acuse y en la traza. La primera inferencia informó 82,41 s de tiempo total del
motor. `ollama ps` informó `100% CPU`, contexto 2048 y `size_vram=0` desde la API.
La suite del backend pasó y el contenedor quedó disponible con el volumen de pesos.
[Petición, selección y traza de la estación](invocacion.json)
