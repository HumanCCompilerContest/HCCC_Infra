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
RUN cargo build --release --bin judge_container

FROM gcc:12.2.0
COPY --from=builder /app/target/release/judge_container /work/
COPY --from=builder /app/testcase/ /work/
ENTRYPOINT ["/work/judge_container"]

