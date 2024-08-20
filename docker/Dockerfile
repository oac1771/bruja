FROM debian:bullseye-slim as builder

WORKDIR /dir
COPY . /dir

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

RUN cargo build --locked --release

###############################################################################
FROM docker.io/library/ubuntu:20.04

COPY --from=builder /dir/target/release/node /usr/local/bin
COPY --from=builder /dir/target/release/worker /usr/local/bin