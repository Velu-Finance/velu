#---------------------------------- builder -----------------------------------
FROM --platform=linux/amd64 rust:1.80 AS builder

WORKDIR /app

# Кэш зависимостей
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo 'fn main() {}' > src/main.rs
RUN cargo build --release
RUN rm -rf src target

# Реальный код
COPY src ./src
RUN cargo build --release --bin velu-backend

# ------------------------------ server (prod) ------------------------------
FROM --platform=linux/amd64 debian:bookworm-slim AS server

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates libssl3 wget curl \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/velu-backend /app/velu-backend

# Health check
HEALTHCHECK --interval=20s --timeout=3s --start-period=5s --retries=3 \
    CMD wget --no-verbose --tries=1 --spider http://localhost:8000/api/health || exit 1

EXPOSE 8000
CMD ["/app/velu-backend"]
