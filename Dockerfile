FROM node:20-bookworm AS builder-front

WORKDIR /app

COPY front/package*.json .

RUN npm i 

ARG VITE_SITE_URL
ENV VITE_SITE_URL=${VITE_SITE_URL}

ARG VITE_API_URL
ENV VITE_API_URL=${VITE_API_URL}

COPY front/ .

RUN npm run build



FROM rust:1-bookworm AS builder-back

ENV SQLX_OFFLINE true

WORKDIR /app/lhavrais-pay/

COPY back/Cargo.toml Cargo.toml
COPY back/src/ src/

#/!\ please make sure to run `cargo sqlx prepare` before
COPY back/.sqlx .sqlx
COPY back/migrations migrations

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/lhavrais-pay/target \
    cargo install --path .





FROM debian:bookworm-slim

WORKDIR /app

RUN apt-get -y update &&  \ 
    apt-get install --no-install-recommends  \
    -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder-back /usr/local/cargo/bin/lhavrais-pay /usr/local/bin/lhavrais-pay
COPY --from=builder-front /app/dist/ dist/

CMD ["lhavrais-pay"]

