FROM rust:buster

RUN apt-get update \
  && apt-get install -y --no-install-recommends build-essential libicu-dev pkg-config libssl-dev clang llvm cmake zlib1g-dev libev-dev libevent-dev libpcsclite-dev unzip zip gcc-mingw-w64-x86-64 g++-mingw-w64-x86-64 clang gcc g++ zlib1g-dev libmpc-dev libmpfr-dev libgmp-dev git libxml2-dev libpcre++-dev curl

COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock
COPY .cargo/ .cargo/
COPY src/ src/

RUN cargo build