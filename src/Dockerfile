FROM rust:1.86 AS builder
LABEL authors="oleexo"

WORKDIR /build

RUN apt-get update && apt-get install -y \
    build-essential \
    cmake \
    perl \
    curl

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

COPY [ "src/", "src" ]
RUN touch src/main.rs
RUN cargo build --release

FROM debian:bookworm-slim AS final
EXPOSE 8080
WORKDIR /app

RUN apt-get update && apt-get install -y \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /build/target/release/c2pa-check .

CMD ["/app/c2pa-check"]
