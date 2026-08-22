# AGENTS.md — c2proof

## Session Bootstrap
New session? Read in order: `STATE.md` → `HANDOFF.md` → `PRD.md` → `tasks.md`. Current phase and next actions live there. Do not re-plan what STATE.md marks locked.

## What This Is
CLI + GitHub Action that turns a C repo into a compiling Rust port PR — with a verification report, not just transpiled code. Verifier-first positioning.

## Rules for Agents
1. Frozen scope: v0.1.0 supports flat C projects only (`.c`/`.h`, no build system). Anything else → graceful refusal with reason.
2. Never build c2rust from source. Real c2rust runs happen in GitHub Actions only. Local testing uses committed golden fixtures.
3. No safe-Rust claims anywhere in output. Mechanical translation, always labeled.
4. Zero cost stack only: Rust, c2rust (BSD-2), Docker, GHCR, GitHub Actions free tier.
5. Every feature must survive the test: "does a stranger's tinyexpr migration work end-to-end?" If not, it's scaffolding.
6. Docs live here, not in code comments.

## Key Decisions
- Name: **c2proof** (C→Rust with proof artifact; verifier-first moat)
- Dogfood target: tinyexpr (only CI-tested target in v0.1.0)
- c2rust = one input source. Agent-generated ports are future input #2.
- Success metric: stranger runs action → building Rust PR lands.

## Non-Goals (v0.1.0)
Makefile/cmake parsing · LLM refactor pass · Windows · config files · safe-Rust promises · sqlite

## Commands
- Pre-push gate: `cargo fmt --check && cargo clippy -- -D warnings && cargo test && cargo audit && cargo deny check`
- `cargo run -- migrate <repo-url> --fixture` — local run against golden fixture
- `cargo run -- migrate <repo-url>` — real c2rust (CI only)
- Action usage: `uses: <you>/c2proof@v0`
