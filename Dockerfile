FROM node:20-bookworm AS builder-front

ARG VITE_BAR_NAME
ENV VITE_BAR_NAME=$VITE_BAR_NAME

WORKDIR /app

COPY front/package*.json .

RUN npm i 

COPY front/ .

RUN npm run build



FROM rust:1-bookworm AS builder-back

ENV SQLX_OFFLINE true

WORKDIR /app/biere-n-collect/

COPY back/Cargo.toml Cargo.toml
COPY back/src/ src/

#/!\ please make sure to run `cargo sqlx prepare` before
COPY back/.sqlx .sqlx
COPY back/migrations migrations

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/biere-n-collect/target \
    cargo install --path .



FROM debian:bookworm-slim

WORKDIR /app

RUN apt-get -y update &&  \ 
    apt-get install --no-install-recommends  \
    -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder-back /usr/local/cargo/bin/biere-n-collect /usr/local/bin/biere-n-collect
COPY --from=builder-front /app/dist/ dist/

CMD ["biere-n-collect"]

