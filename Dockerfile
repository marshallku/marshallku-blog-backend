FROM rust:1.87.0-alpine AS base

WORKDIR /usr/src/marshallku_blog_backend

RUN set -eux; \
    apk add --no-cache musl-dev pkgconfig libressl-dev; \
    rm -rf $CARGO_HOME/registry

COPY Cargo.* .

RUN mkdir src && \
    echo 'fn main() {println!("Hello, world!");}' > src/main.rs && \
    cargo build --release && \
    rm target/release/marshallku_blog_backend* && \
    rm target/release/deps/marshallku_blog_backend* && \
    rm -rf src

FROM base AS builder

COPY src src
RUN cargo build --release

FROM alpine:3.20.2

WORKDIR /usr/local/bin

COPY --from=builder /usr/src/marshallku_blog_backend/target/release/marshallku_blog_backend .

EXPOSE ${PORT}

CMD ["./marshallku_blog_backend"]