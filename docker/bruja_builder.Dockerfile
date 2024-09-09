FROM debian:bullseye-slim as builder

ENV PATH="/root/.cargo/bin:${PATH}"

RUN apt-get update
RUN apt-get install -y curl \
                       protobuf-compiler \
                       build-essential

RUN curl -sSf https://sh.rustup.rs/ | bash -s -- -y
RUN rustup target add wasm32-unknown-unknown
RUN rustup component add rust-src
RUN cargo install --force --locked cargo-contract