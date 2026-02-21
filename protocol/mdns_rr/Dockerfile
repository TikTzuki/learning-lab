FROM rust:1.88.0-slim-bullseye AS builder

RUN apt-get update && apt-get install -y \
    libssl-dev \
    ca-certificates \
    g++ \
    cmake \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /workspace

COPY Cargo.toml Cargo.lock ./

COPY mdns_rr ./mdns_rr

RUN cargo build --release --bin mdns_rr

FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl1.1 \
    && rm -rf /var/lib/apt/lists/*

RUN useradd -m -u 1000 appuser

WORKDIR /app

COPY --from=builder /workspace/target/release/mdns_rr /app/mdns_rr

COPY mdns_rr/config.toml /app/config.toml

RUN mkdir "/app/logs"

RUN chown -R appuser:appuser /app

USER appuser

# Health check
#HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
#    CMD curl -f http://localhost:8080/health || exit 1

CMD ["./mdns_rr"]