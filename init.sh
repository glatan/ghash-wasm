#! /usr/bin/env bash

TRUNK_VERSION='0.14.0'
readonly TRUNK_VERSION
CARGO_HOME='/cargo'
# shellcheck disable=SC2034
readonly CARGO_HOME

# setup stable version of Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs -o rustup-init.sh
sh rustup-init.sh -y \
    --default-host x86_64-unknown-linux-gnu \
    --default-toolchain stable \
    -t wasm32-unknown-unknown
mv -f /cargo/bin/* /usr/bin

# install trunk
curl -L "https://github.com/thedodd/trunk/releases/download/v${TRUNK_VERSION}/trunk-v${TRUNK_VERSION}-x86_64-unknown-linux-gnu.tar.gz" | tar xz
mv -f trunk-v${TRUNK_VERSION}-x86_64-unknown-linux-musl/wasm-pack /usr/bin
