FROM node:14-buster-slim

WORKDIR /workdir

COPY init.sh .

RUN \
    apt update -y && \
    apt install -y \
        curl gcc gzip tar && \
    # init
    ./init.sh && \
    # cache clear
    apt clean -y && \
    rm -rf /var/lib/apt/lists/*
