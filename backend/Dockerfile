FROM rust:1.100-bookworm AS compilacion
WORKDIR /app
COPY backend/ ./
RUN cargo build --release --locked --bin sircle

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends curl jq \
    && rm -rf /var/lib/apt/lists/*
COPY --from=compilacion /app/target/release/sircle /usr/local/bin/sircle
COPY datos/medsp1000/ /datos/medsp1000/
ENTRYPOINT ["sircle"]
