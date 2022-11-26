FROM rust as builder
RUN rustup toolchain add nightly
RUN rustup default nightly
RUN cargo +nightly install -f cargo-fuzz

ADD . /syn
WORKDIR /syn/fuzz

RUN cargo fuzz build

# Package Stage
FROM ubuntu:20.04

COPY --from=builder /syn/fuzz/target/x86_64-unknown-linux-gnu/release/parse_file /
COPY --from=builder /syn/fuzz/target/x86_64-unknown-linux-gnu/release/create_token_buffer /