FROM ollama/ollama:0.40.5
RUN apt-get update && apt-get install -y --no-install-recommends curl jq nginx-light \
    && rm -rf /var/lib/apt/lists/*
COPY inferencia/preparar.sh /usr/local/bin/preparar-modelo
COPY inferencia/servidor.conf /etc/nginx/sircle.conf
RUN nginx -t -c /etc/nginx/sircle.conf
ENTRYPOINT ["/bin/sh", "/usr/local/bin/preparar-modelo"]
