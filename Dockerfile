FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    curl \
    ca-certificates \
    gcc \
    gcc-multilib \
    mingw-w64 \
    binutils \
    binutils-mingw-w64 \
    coreutils \
    && rm -rf /var/lib/apt/lists/*

ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | \
    sh -s -- -y --default-toolchain stable --profile minimal && \
    cargo install cargo-make

WORKDIR /build

COPY rust-toolchain.toml .
RUN rustup show

CMD ["cargo", "make", "--help"]
