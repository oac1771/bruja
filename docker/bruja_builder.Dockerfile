FROM ghcr.io/cross-rs/x86_64-unknown-linux-musl:latest as builder
LABEL stage=builder

ENV PATH="/root/.cargo/bin:${PATH}"

RUN apt-get update
RUN apt-get install -y build-essential \
                        curl \
                        protobuf-compiler \
                        libclang-dev

RUN curl -sSf https://sh.rustup.rs/ | bash -s -- --default-toolchain=1.77.2 -y
RUN rustup target add wasm32-unknown-unknown --toolchain 1.77.2-x86_64-unknown-linux-gnu

# RUN rustup component add rust-src --toolchain stable-aarch64-unknown-linux-gnu
# RUN cargo install --force --locked cargo-contract