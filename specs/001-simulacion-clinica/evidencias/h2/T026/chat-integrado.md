# T026 · Chat con Qwen y arranque del mismo origen

2026-11-25 · `feature/prototipo` · revisión base `fbc3f1ca608d34482d19d72c32bb06dce796a185`

## Procedimiento

Con Ollama de T013 activo y los pesos en su volumen, usar el archivo
`backend/target/demo.sqlite` con el intento de T020. Desde `backend/`, ejecutar
`cargo run --locked --bin sircle -- servir --db target/demo.sqlite`. Desde
`frontend/`, ejecutar `npm ci` si faltan las dependencias y `npm run dev:integrado`.
Las direcciones predeterminadas son 127.0.0.1:8081 para Axum y 127.0.0.1:8080
para el navegador. [Salidas de arranque](arranque.txt)

En este equipo el puerto 8080 del anfitrión lo ocupaba otro proyecto. La corrida
usó un espacio de red propio con `unshare -Urn` e `ip link set lo up`.
Para repetir ese aislamiento, crear un socket Unix privado y habilitar el
puente del anfitrión antes de entrar al espacio de red.

```sh
PUENTE_SIRCLE="$(mktemp -d)/ollama.sock"
socat "UNIX-LISTEN:$PUENTE_SIRCLE,mode=0600,fork" TCP4:127.0.0.1:11434 &
unshare -Urn env PUENTE_SIRCLE="$PUENTE_SIRCLE" bash
```

Dentro del espacio de red, preparar loopback y reenviar solo Ollama. Ejecutar
los dos comandos de arranque indicados arriba dentro de este mismo espacio.

```sh
ip link set lo up
socat TCP4-LISTEN:11434,bind=127.0.0.1,reuseaddr,fork "UNIX-CONNECT:$PUENTE_SIRCLE" &
curl -fsS http://127.0.0.1:11434/api/tags
```

Desde `frontend/`, ejecutar
`npm run test:navegador:chat -- ../specs/001-simulacion-clinica/evidencias/h2/T026`.
La prueba abre 8080 en Chromium con la identidad `estudiante-lr`, elige
«En curso → Continuar», coteja el turno de T015 y envía una nueva pregunta.
Consulta su traza como `docente-demo` en otra sesión, vuelve al navegador
del estudiante y recarga. [Entorno de prueba](entorno.json)

## Resultado

El origen integrado fue `http://127.0.0.1:8080` y Axum conservó su dirección
predeterminada. Se recuperó el intento `c1aaccc1b05db9c4cc6cb572ddad196e` con el
turno de T015 `15d81d67da3ce808602bfaaa840a20fa`. La pregunta libre pasó por el
mismo endpoint, sin ID de sugerencia y con una clave nueva. LR respondió que
empezó a notar la falta de aire hace dos días. El nuevo turno
`c7a1704021ba501f5cc18b133bf5c43b` quedó ligado a la operación de Ollama
`80797c47a5409bc0e8e35489e75b2394`. La traza docente cotejó Qwen 3.5:4b al
100 % por CPU y la línea 7 de la fuente de LR. Al recargar, la misma cookie
HttpOnly permitió recuperar ambos IDs. Se verificó que el otro servicio
conservó su puerto publicado. [Petición, traza y recarga](intercambio.json)

Chromium 156.0.8078.4 · 1280 × 900 · zoom 100 % · escala 1.

- [Selección para continuar](continuar.png)
- [Turno recuperado de T015](turno-t015.png)
- [Respuesta nueva de LR](respuesta-lr.png)
- [Recarga con la misma sesión](recarga.png)
