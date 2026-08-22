# STATE — c2proof

## Phase
SCAFFOLD COMPLETE (T03, T05, T10 partial) → next: T04/T06 pipeline wiring

## Locked Decisions
- Name: `c2proof` · repo: AkashPriyadarshii/c2proof (public, to be created)
- Product: CLI + GitHub Action; C repo → compiling Rust PR + verification report
- Positioning: verifier-first (translation is commoditized; trust artifact is the moat)
- Compute split: local = fixtures + fmt/clippy/test/audit/deny gate. GH Actions = real c2rust, Docker→GHCR, e2e
- No WSL, no Docker on dev laptop (8 GB RAM)
- Dogfood target v0.1.0: tinyexpr ONLY
- Scope frozen per PRD.md; non-goals listed there are final for v0.1.0

## Artifacts Present
Repo live: github.com/AkashPriyadarshii/c2proof (main). cli/ crate (clap migrate cmd, scan_gate + 3 tests), .github/workflows/check.yml (fmt/clippy/test/audit/deny), deny.toml. Local gate green.

## Not Started
T04 docker image, T06 pipeline wiring, T07 REPORT.md, T08 PR push, T09 action YAML, T10 e2e fixture job

## Open Items
- [ ] CI run on first push — verify check workflow green (incl. audit-check working-directory:cli)
