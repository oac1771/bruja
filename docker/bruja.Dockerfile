ARG BUILDER_IMAGE

FROM ${BUILDER_IMAGE} as builder
LABEL stage=intermediate

COPY . .

RUN cargo contract build --manifest-path crates/catalog/Cargo.toml --release
RUN cargo build --exclude scripts --exclude catalog --exclude tests --workspace --release

##############################################################################
FROM docker.io/library/ubuntu:20.04
LABEL stage=app

COPY --from=builder /target/ink/catalog/catalog.contract /

COPY --from=builder /target/release/node /usr/local/bin
COPY --from=builder /target/release/worker /usr/local/bin
