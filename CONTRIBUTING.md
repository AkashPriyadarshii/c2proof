# Contributing

Rules first (from AGENTS.md, enforced):

1. **v0.1.0 scope is frozen**: flat C projects only. Non-goals: Makefile/cmake parsing, LLM refactor pass, Windows hosts, config files, safe-Rust claims, sqlite.
2. **Never build c2rust from source locally.** Real c2rust runs in GitHub Actions. Local testing = committed golden fixtures.
3. **No safe-Rust claims anywhere in output.** Mechanical translation, always labeled.
4. **Zero-cost stack**: Rust, c2rust (BSD-2), Docker, GHCR, GitHub Actions free tier.
5. Every feature must survive: *"does a stranger's tinyexpr migration work end-to-end?"*

## Pre-push gate (required, all green)

```bash
cargo fmt --check --manifest-path cli/Cargo.toml
cargo clippy --manifest-path cli/Cargo.toml -- -D warnings
cargo test  --manifest-path cli/Cargo.toml
cargo audit -f cli/Cargo.lock
cargo deny  --manifest-path cli/Cargo.toml check
```

CI (`check.yml`) runs the same set. Red CI = no merge.

## Workflow

1. Branch off `main`
2. Keep diffs minimal; deletion over addition
3. PR → gate must pass → squash-merge
