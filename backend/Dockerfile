FROM rust:1.100-bookworm AS compilacion
WORKDIR /app
COPY backend/ ./
RUN cargo build --release --locked --bin sircle

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends curl jq \
    && rm -rf /var/lib/apt/lists/*
COPY --from=compilacion /app/target/release/sircle /usr/local/bin/sircle
COPY dataset/medsp1000/mededportal_10181/scenario1/ /datos/medsp1000/mededportal_10181/scenario1/
COPY dataset/medsp1000/mep_2374-8265.10924-s001/scenario3/ /datos/medsp1000/mep_2374-8265.10924-s001/scenario3/
COPY dataset/medsp1000/mededportal_3171/scenario1/ /datos/medsp1000/mededportal_3171/scenario1/
ENTRYPOINT ["sircle"]
