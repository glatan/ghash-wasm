#! /usr/bin/env bash

export WASMPACK_VERSION='0.10.0'
export CARGO_HOME='/cargo'

# setup stable version of Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs -o rustup-init.sh
sh rustup-init.sh -y \
    --default-host x86_64-unknown-linux-gnu \
    --default-toolchain stable \
    -t wasm32-unknown-unknown
mv -f /cargo/bin/* /usr/bin

# install wasm-pack
curl -L "https://github.com/rustwasm/wasm-pack/releases/download/v${WASMPACK_VERSION}/wasm-pack-v${WASMPACK_VERSION}-x86_64-unknown-linux-musl.tar.gz" | tar xz
mv -f wasm-pack-v${WASMPACK_VERSION}-x86_64-unknown-linux-musl/wasm-pack /usr/bin
