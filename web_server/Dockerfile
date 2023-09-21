FROM rust:1.72.1 AS builder

WORKDIR /app
COPY . .
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    cargo build --release && cp target/release/web_server /app/web_server

FROM debian:bookworm-slim AS runner
EXPOSE 55301
WORKDIR /app

RUN apt-get update && apt-get install -y \
  libssl-dev \
  && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/web_server ./web_server

CMD ["/app/web_server"]
