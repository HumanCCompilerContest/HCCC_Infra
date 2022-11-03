# Dockerfile for judge server
FROM gcc:12.2.0
ENTRYPOINT ["/work/judge_system"]
WORKDIR /work

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

COPY Cargo.toml /work/build
COPY src/ /work/build/src/
COPY testcase/ /work/testcase/
RUN cd /work/build && \
    $HOME/.cargo/bin/cargo build --release
RUN mv /work/build/target/release/judge_system /work
