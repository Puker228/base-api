FROM rust:1.94.1-alpine3.22 AS builder
WORKDIR /usr/src/app

COPY Cargo.toml Cargo.lock ./
COPY . .

RUN cargo build --release

FROM debian:trixie

WORKDIR /usr/src/app

COPY --from=builder /usr/src/app/target/release/base-api .

CMD ["./base-api"]