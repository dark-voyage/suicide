FROM rust:1.63.0 as builder

RUN apt-get update && apt-get -y install ca-certificates cmake musl-tools libssl-dev && rm -rf /var/lib/apt/lists/*

COPY . .
RUN rustup target add x86_64-unknown-linux-musl
ENV PKG_CONFIG_ALLOW_CROSS=1
RUN cargo build --target x86_64-unknown-linux-musl --release --bin suicide-registry


FROM alpine:3.8

RUN apk --no-cache add ca-certificates
COPY --from=builder /target/x86_64-unknown-linux-musl/release/suicide-registry .

CMD ["/suicide-registry"]