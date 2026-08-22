# c2proof runner: rust toolchain + pinned c2rust. Built/pushed by CI only.
ARG C2RUST_VERSION=0.20.0
FROM rust:1-slim

RUN apt-get update && apt-get install -y --no-install-recommends curl ca-certificates clang libclang-dev llvm-dev python3 && rm -rf /var/lib/apt/lists/*

# Prebuilt release binary, pinned. No source build.
ARG C2RUST_VERSION
RUN curl -fsSL "https://github.com/immunant/c2rust/releases/download/v${C2RUST_VERSION}/c2rust-${C2RUST_VERSION}-x86_64-unknown-linux-gnu.tar.gz" \
    | tar xz -C /usr/local --strip-components=1 --wildcards '*/bin/c2rust'

ENTRYPOINT ["c2rust"]
