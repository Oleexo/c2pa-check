FROM rust:1.86 as builder
LABEL authors="maxim"

WORKDIR /build

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    build-essential \
    cmake \
    perl \
    curl

ENV OPENSSL_NO_VENDOR=1


# Make cache
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

COPY [ "src/", "src" ]

RUN cargo build --release

FROM debian:bookworm as final
WORKDIR /app
COPY --from=builder /build/target/release/c2pa-check .

ENTRYPOINT ["sleep", "1000000"]