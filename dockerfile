FROM ubuntu:resolute-20260413

USER root

RUN apt-get update && \
    apt-get install build-essential -y && \
    apt-get install curl -y && \
    apt-get install mingw-w64 -y && \
    apt install android-sdk -y && \
    apt install sdkmanager -y && \
    apt update && \
    sdkmanager --install "ndk;r29" && \
    yes | sdkmanager --licenses && \
    apt-get install libssl-dev -y && \
    apt install pkg-config -y

RUN mkdir $HOME/Projects

USER ubuntu

ENV HOME=/home/ubuntu/ ANDROID_HOME=/usr/lib/android-sdk/ NDK_HOME=/usr/lib/android-sdk/build-tools/29.0.3
ENV PATH=$ANDROID_HOME:$PATH

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --default-toolchain 1.96.0 -y

ENV PATH=$HOME".cargo/bin:${PATH}"

RUN rustup target add x86_64-unknown-linux-gnu && \
    rustup target add x86_64-pc-windows-gnu && \
    rustup target add arm-linux-androideabi && \
    rustup target add armv7-linux-androideabi

WORKDIR $HOME/Projects/rustris
