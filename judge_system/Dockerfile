# Dockerfile for judge server
FROM gcc:12.2.0
ENTRYPOINT ["sh", "/work/judge_system"]
RUN mkdir /work
RUN mkdir /work/system
COPY ./Cargo.toml /work/system
COPY ./src /work/system
COPY ./testcase /work
RUN cd /work/system && cargo build --release && mv /work/system/target/judge_system /work
