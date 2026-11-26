#!/bin/sh
set -eu

modelo=qwen3.5:4b
digest=d8b0f5e9760cd1682034f292d7ef72ec46f432149be0df7574bf2d6e92e38c04
interno=127.0.0.1:11435
ollama_url=http://$interno
directorio=$(mktemp -d)
proxy=
motor=
trap 'exit 0' TERM INT
trap 'test -z "$proxy" || kill "$proxy" 2>/dev/null || :; test -z "$motor" || kill "$motor" 2>/dev/null || :; rm -f "$directorio/descarga"; rmdir "$directorio"' EXIT

OLLAMA_HOST=$interno OLLAMA_NO_CLOUD=1 ollama serve &
motor=$!

preparar() {
    causa='el servicio local no responde'
    inventario=$(curl -fsS --connect-timeout 4 --max-time 8 "$ollama_url/api/tags") || return 1
    causa='el inventario de modelos es inválido'
    local_digest=$(printf '%s' "$inventario" | jq -r --arg modelo "$modelo" '.models[] | select(.name==$modelo) | .digest') || return 1
    if [ -z "$local_digest" ]; then
        causa='falló la descarga del registro oficial o dejó de avanzar'
        curl -fsS --no-buffer --connect-timeout 8 --speed-limit 1 --speed-time 90 \
            -H 'Content-Type: application/json' -d "{\"model\":\"$modelo\",\"stream\":true}" \
            "$ollama_url/api/pull" > "$directorio/descarga" || return 1
        if ! jq -se 'any(.[]; .status=="success") and all(.[]; has("error")|not)' "$directorio/descarga" >/dev/null; then
            causa=$(jq -r 'select(.error) | .error' "$directorio/descarga" | tail -n 1)
            causa=${causa:-el registro no confirmó la descarga}
            return 1
        fi
        inventario=$(curl -fsS --connect-timeout 4 --max-time 8 "$ollama_url/api/tags") || return 1
        local_digest=$(printf '%s' "$inventario" | jq -r --arg modelo "$modelo" '.models[] | select(.name==$modelo) | .digest') || return 1
    fi
    causa="huella inesperada: esperada $digest, disponible ${local_digest:-ninguna}"
    [ "$local_digest" = "$digest" ] || return 2

    causa='no se pudo verificar una generación breve con el modelo'
    generacion=$(curl -sS --fail-with-body --connect-timeout 4 --max-time 150 -H 'Content-Type: application/json' \
        -d "{\"model\":\"$modelo\",\"prompt\":\"Responde únicamente OK\",\"stream\":false,\"think\":false,\"options\":{\"temperature\":0,\"num_ctx\":2048,\"num_predict\":8}}" \
        "$ollama_url/api/generate") || {
        detalle=$(printf '%s' "$generacion" | jq -r '.error // empty' 2>/dev/null) || detalle=
        causa="$causa${detalle:+ · $detalle}"
        return 1
    }
    printf '%s' "$generacion" | jq -e --arg modelo "$modelo" '.done == true and .model == $modelo and (.response | test("^[[:space:]]*OK[[:space:]]*$"; "i"))' >/dev/null
}

for espera in 1 2 3 4 5 6 7 8 9 10; do
    if curl -fsS --connect-timeout 1 --max-time 2 "$ollama_url/api/version" >/dev/null 2>&1; then break; fi
    sleep 1
done

for intento in 1 2 3; do
    printf 'inferencia %s · intento %s/3\n' "$modelo" "$intento"
    if preparar; then
        printf 'inferencia %s · disponible · sha256 %s\n' "$modelo" "$digest"
        socat TCP-LISTEN:11434,reuseaddr,fork TCP:"$interno" &
        proxy=$!
        wait "$proxy"
        exit $?
    else
        codigo=$?
    fi
    printf 'inferencia %s · %s\n' "$modelo" "$causa" >&2
    if [ "$codigo" -eq 2 ] || [ "$intento" -eq 3 ]; then break; fi
    sleep "$((intento * 2))"
done

printf 'inferencia %s · no disponible. Verifica el acceso a registry.ollama.ai y que sircle-pesos contenga la versión indicada. Después ejecuta docker compose restart inferencia.\n' "$modelo" >&2
wait "$motor"
