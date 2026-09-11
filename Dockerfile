FROM rust:alpine AS builder

LABEL maintainer="Mahmoud Harmouch <oss@wiseai.dev>"

RUN apk update && apk upgrade && \
    apk add --no-cache \
    build-base \
    pkgconfig \
    openssl-dev \
    openssl-libs-static \
    musl-dev \
    git

WORKDIR /bump2version

COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY benches ./benches/
COPY build.rs ./
COPY README.md ./
COPY RUST.md ./
COPY WASM.md ./

RUN RUSTFLAGS="-C target-cpu=native -C opt-level=3" \
    cargo build --release --features="rust-binary" && \
    strip target/release/bump

FROM alpine:3.24.1

RUN apk add --no-cache ca-certificates && \
    addgroup -g 10000 -S bump2version && \
    adduser -u 10000 -S -G bump2version bump2version

WORKDIR /workspace

COPY --from=builder /bump2version/target/release/bump /usr/local/bin/bump

RUN ln -s /usr/local/bin/bump /usr/local/bin/cargo-bump

USER 10000:10000

ENTRYPOINT ["/usr/local/bin/bump"]

CMD ["--help"]
