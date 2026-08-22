# PRD — c2proof v0.1.0

## Problem
Everyone generates Rust from C (agents, c2rust). Nobody trusts the output. Translation is commoditized; verification is scarce.

## Product
One command: C repo in → pull request out containing a compiling Rust crate + parity report.

## Users
1. Maintainers of small/old C libs curious about Rust
2. Teams running AI C→Rust migrations who need a trust gate
3. Vibecoders shipping portfolio-grade OSS tooling

## Core Flow
```
c2pr migrate https://github.com/x/tinyexpr
  → scan .c/.h files
  → transpile (c2rust, pinned)
  → cargo build + clippy on output crate
  → generate REPORT.md (LOC, unsafe count, status, unresolved symbols)
  → branch `rust-port` → push → open PR
```

## v0.1.0 Scope (FROZEN)
- Flat C projects only; no configure/make/cmake. Refuse others gracefully.
- One dogfooded target: tinyexpr, tested in CI.
- CLI flag-only (`--out`, `--crate-name`). No config file.
- Report = markdown. Unsafe-line metrics included.
- Docker image on GHCR, prebuilt c2rust binary.

## Explicit Non-Goals
LLM refactoring · Makefile/cmake support · Windows · safe-Rust claims · multi-target matrix

## Success Metric
A stranger runs the action on their C repo, gets a building Rust PR, posts about it.

## Failure Signal
Support issues > usage stars. Means scope lied about what works.
