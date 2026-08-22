# Implementation Plan — c2proof v0.1.0

## Weekend 1 — prove by hand (spec extraction)
1. Fresh clone tinyexpr → hand-run full pipeline: transpile → fix → cargo build
2. Log every manual step + every breakage verbatim
3. That log = acceptance test for the CLI. Freeze it.

## Weekend 2 — build the machine
Day 1:
- [ ] scaffold cli/ with clap, `migrate` subcommand skeleton
- [ ] Transpiler seam: `C2RustTranspiler` (subprocess) + `FixtureTranspiler` (reads tests/fixtures/tinyexpr/)
- [ ] implement scan gate (flat-dir check, refusal reasons)

Day 2:
- [ ] wire pipeline: clone → scan → transpile → build/clippy (works against fixture locally)
- [ ] REPORT.md generator + unit tests for it
- [ ] git push + PR creation (GITHUB_TOKEN path)
- [ ] ci/check workflow: fmt+clippy+test+audit+deny on every push
- [ ] REPORT.md generator
- [ ] git push + PR creation (GITHUB_TOKEN path)
- [ ] action/ composite YAML wrapping CLI

Day 3 (half):
- [ ] e2e CI job on ubuntu runner: real c2rust transpiles tinyexpr → green; commit refreshed fixture
- [ ] docker runner image build in CI → GHCR (never built locally)
- [ ] action/ composite YAML wrapping CLI
- [ ] end-to-end CI job: fresh tinyexpr → green pipeline
- [ ] README with asciinema + support matrix
- [ ] tag v0.1.0, publish action to marketplace

## Acceptance (all must pass, nothing else counts)
- [ ] fresh tinyexpr clone → `c2pr migrate` → PR opens, crate builds
- [ ] non-flat repo → clean refusal, exit 2, reason printed
- [ ] docker missing → instructive exit 1
- [ ] total stranger time-to-first-PR < 10 min following README

## Post-v0.1 backlog (do NOT touch until traction signal)
test-parity oracle → agent-output ingestion → LLM refactor gated by tests
