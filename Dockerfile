FROM amazonlinux:2

WORKDIR /workdir

COPY init.sh .

RUN \
    curl -sL https://rpm.nodesource.com/setup_16.x | bash - && \
    yum update -y && \
    yum install -y \
        gcc gzip nodejs tar && \
    ./init.sh
