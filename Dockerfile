FROM amazonlinux:2

WORKDIR /workdir

COPY init.sh .

RUN \
    yum update -y && \
    yum install -y \
        gcc gzip nodejs tar && \
    ./init.sh
