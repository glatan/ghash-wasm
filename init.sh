#! /usr/bin/env bash

# setup stable version of Rust
export CARGO_HOME='/cargo'

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs -o rustup-init.sh
sh rustup-init.sh -y \
    --default-host x86_64-unknown-linux-gnu \
    --default-toolchain stable \
    -t wasm32-unknown-unknown
mv -f /cargo/bin/* /usr/bin

# install trunk
TRUNK_VERSION='v0.19.1'
readonly TRUNK_VERSION

curl -L "https://github.com/trunk-rs/trunk/releases/download/${TRUNK_VERSION}/trunk-x86_64-unknown-linux-musl.tar.gz" | tar xz
mv -f trunk /usr/bin
