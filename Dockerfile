FROM rust:latest as rustbuilder

RUN apt-get update && apt-get install -y \
  apt-utils \
  libssl-dev openssl \
  pkg-config \
  clang \
  libclang-dev \
  apt-transport-https

RUN USER=root cargo new --bin hermes
WORKDIR /hermes

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

RUN cargo build --release


FROM node:10-alpine as jsbuilder

WORKDIR /ui

COPY ./ui ./

RUN yarn install
RUN yarn build


FROM debian:stretch-slim
RUN apt update && apt install -y libpq5 netcat-openbsd

WORKDIR /app
COPY --from=rustbuilder hermes/target/release/hermes .
COPY --from=jsbuilder ui/dist ./ui/dist

COPY ./docker-entrypoint.sh ./docker-entrypoint.sh

EXPOSE 3030
ENTRYPOINT ["./docker-entrypoint.sh"]
