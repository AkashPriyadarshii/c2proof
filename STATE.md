# STATE — c2proof

## Phase
T07+T08 DONE → next: T09 action YAML, T10 e2e fixture-regen CI job

## Locked Decisions
- Name: `c2proof` · repo: AkashPriyadarshii/c2proof (public, to be created)
- Product: CLI + GitHub Action; C repo → compiling Rust PR + verification report
- Positioning: verifier-first (translation is commoditized; trust artifact is the moat)
- Compute split: local = fixtures + fmt/clippy/test/audit/deny gate. GH Actions = real c2rust, Docker→GHCR, e2e
- No WSL, no Docker on dev laptop (8 GB RAM)
- Dogfood target v0.1.0: tinyexpr ONLY
- Scope frozen per PRD.md; non-goals listed there are final for v0.1.0

## Artifacts Present
Repo live: github.com/AkashPriyadarshii/c2proof. cli/ = lib (pipeline: clone→scan→transpile→cargo check) + thin bin. Transpiler seam: fixture mode (`C2PROOF_FIXTURE_DIR` override) / c2rust docker `ghcr.io/akashpriyadarshii/c2proof/runner:c2rust-0.20.0`. GHCR image pushed (c2rust 0.20.0 built from source in-image — upstream ships NO release assets). Workflows: check.yml green, docker.yml green. Docs: README(SEO)/INSTALL/CONTRIBUTING/CHANGELOG/LICENSE. Tests: cli/tests/pipeline.rs 11 integration tests incl exit-code mapping + GHCR lowercase invariant.

## Not Started
T09 action YAML, T10 e2e fixture-regen job, T11 README asciinema, T12 tag v0.1.0

## Open Items
- [ ] CI run on first push — verify check workflow green (incl. audit-check working-directory:cli)
