FROM debian:buster-slim

WORKDIR /workdir

ENV CARGO_HOME "/cargo"
ENV PATH "/cargo/bin:$PATH"

ENV WASMPACK_VERSION '0.9.1'

COPY Cargo.toml .

RUN \
    apt update -y && \
    apt install -y \
        curl gcc gzip tar npm && \
    # rust setup
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs -o rustup-init.sh && \
    sh rustup-init.sh -y \
        --default-host x86_64-unknown-linux-gnu \
        --default-toolchain stable \ 
        -t wasm32-unknown-unknown && \
    # install wasm-pack
    curl -L "https://github.com/rustwasm/wasm-pack/releases/download/v${WASMPACK_VERSION}/wasm-pack-v${WASMPACK_VERSION}-x86_64-unknown-linux-musl.tar.gz" | tar xz && \
    cp wasm-pack-v${WASMPACK_VERSION}-x86_64-unknown-linux-musl/wasm-pack /usr/bin && \
    # cache clear
    apt clean -y && \
    rm -rf /var/lib/apt/lists/*

RUN \
    mkdir src && \
    touch src/lib.rs && \
    cargo build && \
    cargo build --release && \
    rm -r src/
