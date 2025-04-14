FROM rust:1.86 as builder
LABEL authors="maxim"

WORKDIR /build

RUN apt-get update && apt-get install -y \
    build-essential \
    cmake \
    perl \
    curl

#ENV OPENSSL_NO_VENDOR=1

# Make cache
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

COPY [ "src/", "src" ]

RUN cargo build --release

FROM debian:bookworm-slim as final
WORKDIR /app

# Install only the necessary runtime dependencies
RUN apt-get update && apt-get install -y \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /build/target/release/c2pa-check .

# Expose the port your application uses
EXPOSE 8080

# Run the application
CMD ["/app/c2pa-check"]
