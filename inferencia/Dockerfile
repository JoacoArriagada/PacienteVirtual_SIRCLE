FROM ollama/ollama:0.40.5
RUN apt-get update && apt-get install -y --no-install-recommends curl jq socat \
    && rm -rf /var/lib/apt/lists/*
COPY inferencia/preparar.sh /usr/local/bin/preparar-modelo
ENTRYPOINT ["/bin/sh", "/usr/local/bin/preparar-modelo"]
