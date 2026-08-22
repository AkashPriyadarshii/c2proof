<div align="center">

# c2proof

**C to Rust migration with a verification report — not just transpiled code.**

[![check](https://github.com/AkashPriyadarshii/c2proof/actions/workflows/check.yml/badge.svg)](https://github.com/AkashPriyadarshii/c2proof/actions/workflows/check.yml)
[![docker](https://github.com/AkashPriyadarshii/c2proof/actions/workflows/docker.yml/badge.svg)](https://github.com/AkashPriyadarshii/c2proof/actions/workflows/docker.yml)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![rust](https://img.shields.io/badge/rust-stable-orange.svg)](https://www.rust-lang.org)

Turn a legacy C project into a compiling Rust port pull request, plus a REPORT.md that tells you what to distrust.

</div>

## Why c2proof?

Automated C-to-Rust transpilers exist. Trusting their output doesn't. **c2proof** wraps [c2rust](https://c2rust.com/) mechanical translation in a verifier-first pipeline: every migration produces a **compiling Rust crate**, a **clippy-clean gate**, and a **verification report** — so you review evidence, not vibes. Built for teams migrating legacy C codebases to memory-safe Rust with CI/CD automation.

## Quickstart

```bash
# clone any flat C project
git clone --depth 1 https://github.com/nickmqb/muon ../tinyexpr

# migrate it (offline fixture mode)
c2proof migrate ./tinyexpr --fixture
```

Or drop this into your repo's workflow:

```yaml
- uses: AkashPriyadarshii/c2proof@v0
  with:
    repo-url: https://github.com/someone/tinyexpr
```

Full setup → [INSTALL.md](INSTALL.md)

## How It Works

```
clone → scan → c2rust transpile (pinned docker) → cargo check + clippy → PR + REPORT.md
```

1. **Flat-scan gate** — refuses non-flat C projects up front with an exact reason (exit 2). No Makefile/cmake parsing in v0.
2. **Pinned c2rust** — translation runs inside a versioned container (`ghcr.io/akashpriyadarshii/c2proof/runner`), never on your laptop.
3. **Verify, don't trust** — output must compile; clippy findings are captured, not hidden.
4. **Proof artifact** — REPORT.md lists unsafe-function counts per file and unresolved symbols. Mechanical translation is always labeled as such.

## Support Matrix

| Input | Status |
|---|---|
| Flat `.c`/`.h` directories (no build system) | ✅ supported |
| tinyexpr | ✅ CI-tested dogfood target |
| Makefile / cmake / autotools | ❌ refused with reason |
| Windows host OS | ❌ (CI-only compute) |

## Keywords

C to Rust converter · automated Rust migration · c2rust wrapper · transpile C to safe Rust · legacy code modernization · memory safety · Rust port generator · GitHub Action for code migration · verification-first transpilation · C refactoring tool · unsafe audit report

## Roadmap

- **v0.1** — flat C repos, pinned c2rust, verification report, PR bot ✅
- **v0.2** — test-parity oracle (run original C tests against the port)
- **later** — agent-generated ports as input · LLM refactor pass gated by test oracle

## License

MIT — see [LICENSE](LICENSE). c2rust is BSD-2-Clause; we orchestrate it, we don't vendor it.
