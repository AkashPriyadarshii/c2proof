# c2proof runner: rust toolchain + pinned c2rust built from source.
# Built/pushed by CI only (rule: no local c2rust builds).
# ponytail: full source build (~25 min CI); switch back to prebuilt tarball if upstream ships assets again.
ARG C2RUST_VERSION=0.20.0
FROM ubuntu:22.04

ARG DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get install -y --no-install-recommends \
    curl ca-certificates build-essential cmake git python3 \
    clang-14 llvm-14-dev libclang-14-dev libssl-dev pkg-config \
    && rm -rf /var/lib/apt/lists/*

ENV LLVM_SYS_140_PREFIX=/usr/lib/llvm-14
RUN curl -fsSL https://sh.rustup.rs | sh -s -- -y --default-toolchain stable --profile minimal
ENV PATH="/root/.cargo/bin:${PATH}"

# Pinned c2rust from crates.io (source build inside container = allowed; local builds are not).
ARG C2RUST_VERSION
RUN cargo install --locked --version ${C2RUST_VERSION} c2rust

ENTRYPOINT ["c2rust"]
