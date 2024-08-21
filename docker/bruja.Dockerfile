ARG BUILDER_IMAGE=

FROM ${BUILDER_IMAGE} as builder
LABEL stage=builder

WORKDIR /dir
COPY . /dir

RUN cargo contract build --release --manifest-path crates/catalog/Cargo.toml
RUN cargo build --locked --release

###############################################################################
FROM docker.io/library/ubuntu:20.04
LABEL stage=app

COPY --from=builder /dir/target/ink/catalog/catalog.contract /

COPY --from=builder /dir/target/release/node /usr/local/bin
COPY --from=builder /dir/target/release/worker /usr/local/bin