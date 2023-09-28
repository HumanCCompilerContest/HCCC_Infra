FROM lukemathwalker/cargo-chef:latest-rust-1.65.0 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json
RUN cat recipe.json

FROM chef AS builder 
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --config net.git-fetch-with-cli=true --release --bin test_runner --features riscv

FROM ghcr.io/alignof/riscv_toolchain_docker:master
ENV PATH $PATH:/opt/riscv/bin:$HOME/.cargo/bin
COPY --from=builder /app/target/release/test_runner /work/
ENTRYPOINT ["/work/test_runner"]

