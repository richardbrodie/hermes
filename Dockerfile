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

COPY ./react-ui ./react-ui
RUN cd react-ui && yarn install
RUN cd react-ui && yarn build

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./src ./src

RUN cargo build --release

FROM debian:stretch-slim
RUN apt update && apt install -y libssl-dev openssl libpq5 netcat-openbsd ca-certificates

WORKDIR /app
RUN mkdir ./react-ui
RUN mkdir ./react-ui/build
COPY --from=build app/target/release/feeds .
COPY --from=build app/target/release/add_user .
COPY --from=build app/react-ui/build ./react-ui/build
COPY --from=build /usr/local/cargo/bin/diesel /usr/bin/diesel

COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
COPY ./migrations ./migrations
COPY ./docker-entrypoint.sh ./docker-entrypoint.sh
ENTRYPOINT ["./docker-entrypoint.sh"]
