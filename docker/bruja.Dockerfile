ARG BUILDER_IMAGE

FROM ${BUILDER_IMAGE} as builder
LABEL stage=intermediate

WORKDIR /dir
COPY . /dir

RUN cargo contract build --manifest-path crates/catalog/Cargo.toml
RUN cargo build --locked

##############################################################################
FROM docker.io/library/ubuntu:20.04
LABEL stage=app

COPY --from=builder /dir/target/ink/catalog/catalog.contract /

COPY --from=builder /dir/target/debug/node /usr/local/bin
COPY --from=builder /dir/target/debug/worker /usr/local/bin