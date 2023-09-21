FROM rust:1.72.1 AS builder

# Build judge_server
WORKDIR /app
COPY . .
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    cargo build --release && cp target/release/judge_server /app/judge_server

FROM debian:bookworm-slim AS runner
WORKDIR /app

# Install docker client(for running test_runner)
RUN apt-get update && apt-get install -y \
  curl \
  && rm -rf /var/lib/apt/lists/*

ENV DOCKER_CLIENT_VERSION=24.0.6
ENV DOCKER_API_VERSION=1.43
RUN curl -fsSL https://download.docker.com/linux/static/stable/x86_64/docker-${DOCKER_CLIENT_VERSION}.tgz \
  | tar -xzC /usr/local/bin --strip=1 docker/docker

# Copy judge_server
COPY --from=builder /app/judge_server ./judge_server

CMD ["/app/judge_server"]
