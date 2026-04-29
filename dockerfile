FROM ubuntu:resolute-20260413

RUN useradd -ms /bin/bash/ abstract

USER root

RUN apt-get update && \
    apt-get install build-essential -y && \
    apt-get install curl -y && \
    apt-get install mingw-w64 -y 

ENV HOME="/home/abstract/"

RUN mkdir -p $HOME/Projects

USER abstract

WORKDIR $HOME 

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --default-toolchain 1.93.0 -y

ENV PATH=$HOME".cargo/bin:${PATH}"

RUN rustup target add x86_64-unknown-linux-gnu && \
    rustup target add x86_64-pc-windows-gnu && \
    rustup target add x86_64-apple-darwin




