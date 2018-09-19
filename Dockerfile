FROM rust:latest as build

RUN apt-get update && apt-get install -y \
  apt-utils \
  libssl-dev openssl \
  pkg-config \
  clang \
  libclang-dev \
  apt-transport-https

RUN curl -sS https://dl.yarnpkg.com/debian/pubkey.gpg | apt-key add -
RUN echo "deb https://dl.yarnpkg.com/debian/ stable main" | tee /etc/apt/sources.list.d/yarn.list
RUN curl -sL https://deb.nodesource.com/setup_10.x | bash -
RUN apt-get update && apt-get install -y nodejs yarn

RUN cargo install diesel_cli --no-default-features --features postgres

RUN USER=root cargo new --bin app
WORKDIR /app

COPY ./ui ./ui
RUN cd ui && yarn install
RUN cd ui && yarn build

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./src ./src

RUN cargo build --release

FROM debian:stretch-slim
RUN apt update && apt install -y libssl-dev openssl libpq5 netcat-openbsd ca-certificates

WORKDIR /app
RUN mkdir ./ui
RUN mkdir ./ui/dist
COPY --from=build app/target/release/hermes .
COPY --from=build app/target/release/add_user .
COPY --from=build app/ui/dist ./ui/dist
COPY --from=build /usr/local/cargo/bin/diesel /usr/bin/diesel

COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
COPY ./migrations ./migrations
COPY ./docker-entrypoint.sh ./docker-entrypoint.sh

EXPOSE 3030
ENTRYPOINT ["./docker-entrypoint.sh"]
