FROM lukemathwalker/cargo-chef:latest-rust-1.65.0 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json
RUN cat recipe.json

FROM chef AS builder 
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef --release --recipe-path recipe.json
COPY . .
RUN cargo build --config net.git-fetch-with-cli=true --release --bin test_runner --features riscv

FROM archlinux
RUN pacman-key --init \
    && pacman -Sy --noconfirm archlinux-keyring \
    && pacman -Syyu --noconfirm autoconf automake curl python3 libmpc mpfr git gmp gawk base-devel bison flex texinfo gperf libtool patchutils bc zlib expat dtc
RUN git clone https://github.com/riscv/riscv-gnu-toolchain \
    && cd /riscv-gnu-toolchain \
    && ./configure --prefix=/opt/riscv && make -j$(nproc)
RUN git clone https://github.com/riscv-software-src/riscv-isa-sim.git \
    && mkdir -p /riscv-isa-sim/build \
    && cd /riscv-isa-sim/build \
    && ../configure --prefix=/opt/riscv \
    && make && make install 
RUN git clone https://github.com/riscv-software-src/riscv-pk.git \
    && mkdir -p /riscv-pk/build \
    && cd /riscv-pk/build \
    && ../configure --prefix=/opt/riscv --host=riscv64-unknown-elf \
    && make && make install 
ENV PATH $PATH:/opt/riscv/bin:$HOME/.cargo/bin
COPY --from=builder /app/target/release/test_runner /work/
ENTRYPOINT ["/work/test_runner"]

