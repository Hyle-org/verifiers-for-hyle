FROM rust:latest as builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY hyle-contract hyle-contract
COPY risc0-verifier risc0-verifier
COPY sp1-verifier sp1-verifier
RUN rustup override set nightly-2024-04-17
RUN RUSTFLAGS='-C target-feature=+crt-static' cargo build --release --target x86_64-unknown-linux-gnu

FROM alpine:latest
WORKDIR /
COPY --from=builder /app/target/x86_64-unknown-linux-gnu/release/risc0-verifier risc0-verifier
COPY --from=builder /app/target/x86_64-unknown-linux-gnu/release/sp1-verifier sp1-verifier
