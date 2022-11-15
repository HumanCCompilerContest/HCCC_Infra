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
RUN cargo build --release --bin judge_system

FROM gcc:12.2.0
COPY --from=builder /app/target/release/judge_system /work/
RUN ls -l /
RUN file /work/judge_system
ENTRYPOINT ["/work/judge_system"]

