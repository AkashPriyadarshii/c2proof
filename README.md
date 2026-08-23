# c2proof

[![ci](https://github.com/AkashPriyadarshii/c2proof/actions/workflows/e2e.yml/badge.svg)](https://github.com/AkashPriyadarshii/c2proof/actions/workflows/e2e.yml)

**C to Rust with a proof artifact. Verifier-first migration.**

Everyone generates Rust from C (agents, `c2rust`). Nobody trusts the output. Translation is commoditized; verification is scarce.

`c2proof` is a CLI and GitHub Action that takes a C repository and emits a **pull request** containing a compiling Rust crate + a detailed parity report.

---

## 🚀 Quickstart

### As a GitHub Action (Recommended)
Add `c2proof` to your C project to automatically generate a Rust port PR:

```yaml
name: Rust Port
on: [workflow_dispatch]

permissions:
  contents: write
  pull-requests: write

jobs:
  migrate:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Generate Rust Port
        uses: AkashPriyadarshii/c2proof@v0.1.0
        with:
          github_token: ${{ secrets.GITHUB_TOKEN }}
```

### As a CLI
```bash
# Clone the transpiler CLI
git clone https://github.com/AkashPriyadarshii/c2proof
cd c2proof

# Migrate a C repository
cargo run --release -- migrate https://github.com/codeplea/tinyexpr
```

## 📊 The "Proof Artifact"

It`s not just transpiled code. `c2proof` generates a `REPORT.md` that is attached to the PR, detailing:
- 🟢 Build status (compiles cleanly)
- 🦀 Lines of Code (C vs Rust)
- ⚠️ `unsafe` block count
- 🔍 Unresolved external symbols

## 💻 Support Matrix (v0.1.0)

For `v0.1.0`, scope is **strictly frozen** to:
- **Project Structure**: Flat C projects only (`.c`/`.h` files). No `configure`, `make`, or `cmake` parsing.
- **Dogfood Target**: Built and tested heavily against [`tinyexpr`](https://github.com/codeplea/tinyexpr).
- **Environment**: Linux (Ubuntu latest) via Docker container containing pinned `c2rust`.
- **Target OS**: No Windows support in v0.1.0.

## 🎥 Demo
<!-- Asciinema terminal recording placeholder -->
[![asciicast](https://asciinema.org/a/xxxxxx.svg)](https://asciinema.org/a/xxxxxx)

## ⚖️ License
MIT / Apache-2.0
