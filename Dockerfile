FROM rust:latest as rustbuilder

RUN apt-get update && apt-get install -y \
  apt-transport-https \
  apt-utils \
  libpq5 \
  libssl-dev \
  openssl \
  pkg-config

RUN cargo install diesel_cli --no-default-features --features postgres

RUN USER=root cargo new --bin hermes
WORKDIR /hermes

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release

COPY ./src ./src
RUN touch src/main.rs

RUN cargo build --release


FROM node:10-alpine as jsbuilder

WORKDIR /ui

COPY ./ui ./

RUN apk add --no-cache \
    build-base \
    libpng-dev \
    lcms2-dev \
    bash

RUN yarn install
RUN yarn build


FROM debian:stretch-slim
RUN apt update && apt install -y libpq5 netcat-openbsd ca-certificates

WORKDIR /app
COPY --from=rustbuilder /usr/local/cargo/bin/diesel /usr/bin/diesel
COPY --from=rustbuilder hermes/target/release/hermes .
COPY ./migrations ./migrations
COPY --from=jsbuilder ui/dist ./ui/dist

COPY ./docker-entrypoint.sh ./docker-entrypoint.sh

EXPOSE 3030
ENTRYPOINT ["./docker-entrypoint.sh"]
