FROM debian:bullseye-slim as builder
LABEL stage=builder

ENV PATH="/root/.cargo/bin:${PATH}"

RUN apt-get update
RUN apt-get install -y build-essential \
                        curl \
                        protobuf-compiler \
                        libclang-dev \
                        git

RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
RUN rustup target add wasm32-unknown-unknown
RUN rustup component add rust-src --toolchain stable-aarch64-unknown-linux-gnu
RUN cargo install --force --locked cargo-contract