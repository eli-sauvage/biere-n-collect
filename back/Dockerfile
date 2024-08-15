FROM rust:1-bookworm AS builder

ENV SQLX_OFFLINE true

WORKDIR /app/lhavrais-pay/

COPY Cargo.toml Cargo.toml
COPY src/ src/

#/!\ please make sure to run `cargo sqlx prepare` before
COPY .sqlx .sqlx
COPY migrations migrations

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/lhavrais-pay/target \
    cargo install --path .

FROM debian:bookworm-slim

RUN apt-get -y update &&  \ 
    apt-get install --no-install-recommends  \
    -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/local/cargo/bin/lhavrais-pay /usr/local/bin/lhavrais-pay

COPY Rocket.toml Rocket.toml

CMD ["lhavrais-pay"]
