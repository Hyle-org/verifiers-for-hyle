FROM rust:latest as builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY . .
RUN RUSTFLAGS='-C target-feature=+crt-static' cargo build --release --target x86_64-unknown-linux-gnu

FROM alpine:latest
WORKDIR /
COPY --from=builder /app/target/x86_64-unknown-linux-gnu/release/verifier .