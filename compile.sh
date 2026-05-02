#!/bin/sh

# script used to compile for Linux and Windows

docker build -t rustris-compiler:local .

docker run -u $UID:abstract -v .:/home/abstract/Projects/rustris rustris-compiler:local \
    cd Projects/rustris && \
    cargo build --release && \
    cargo build --release --target=x86_64-pc-windows-gnu

docker image rm rustris-compiler:local -f

