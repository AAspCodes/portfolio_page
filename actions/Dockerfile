FROM ubuntu:jammy

RUN apt update && apt upgrade -y
RUN apt install -y build-essential curl 
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y

ENV PATH="/root/.cargo/bin:${PATH}"
RUN cargo --help
RUN cargo install trunk 
RUN rustup target add wasm32-unknown-unknown

