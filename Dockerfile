FROM node:20-bookworm AS builder-front

ARG VITE_BAR_NAME
ENV VITE_BAR_NAME=$VITE_BAR_NAME

WORKDIR /app

COPY front/package*.json .

RUN npm i 

COPY front/ .

RUN npm run build



FROM rust:1-bookworm AS builder-back


WORKDIR /app/biere-n-collect/

ARG DATABASE_URL
ENV DATABASE_URL=$DATABASE_URL
ARG DATABASE_FILE

COPY back/Cargo.toml Cargo.toml
COPY back/src/ src/

COPY back/$DATABASE_FILE $DATABASE_FILE
COPY back/migrations migrations

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/biere-n-collect/target \
     cargo build --release && cp target/release/biere-n-collect biere-n-collect



FROM debian:bookworm-slim

WORKDIR /app

RUN apt-get -y update &&  \ 
    apt-get install --no-install-recommends  \
    -y ca-certificates curl && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder-back /app/biere-n-collect/biere-n-collect /app
COPY --from=builder-front /app/dist/ dist/

CMD ["/app/biere-n-collect"]

