FROM archlinux:latest

WORKDIR /workdir
EXPOSE 8080

ENV CARGO_HOME /opt
ENV PATH $PATH:/opt/bin

ENV WASMPACK_VERSION '0.8.1'

COPY package-lock.json .
COPY package.json .
COPY Cargo.toml .

RUN \
    pacman -Syy --noconfirm && \
    pacman -S --noconfirm \
        gcc npm rustup gzip tar && \
    # install wasm-pack
    curl -L "https://github.com/rustwasm/wasm-pack/releases/download/v${WASMPACK_VERSION}/wasm-pack-v${WASMPACK_VERSION}-x86_64-unknown-linux-musl.tar.gz" | tar xz && \
    cp wasm-pack-v${WASMPACK_VERSION}-x86_64-unknown-linux-musl/wasm-pack /usr/bin && \
    # rust setup
    rustup default stable && \
    rustup target add wasm32-unknown-unknown && \
    pacman -Rs --noconfirm gzip tar && \
    pacman -Sc --noconfirm && \
    npm i

RUN \
    mkdir src && \
    touch src/lib.rs && \
    cargo build --release && \
    rm -r src/
