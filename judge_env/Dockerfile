# Dockerfile for judge server
FROM gcc:12.2.0
ENTRYPOINT ["sh", "/work/judge_env"]
RUN mkdir /work
COPY ./scripts/judge.sh /work

RUN mkdir /work/system
COPY ./Cargo.toml /work/system
COPY ./src /work/system
COPY ./src /work/testcase
RUN cargo build --release && mv /work/system/target/judge_env /work
