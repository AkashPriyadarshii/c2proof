# Changelog

All notable changes. Format: [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), semver.

## [0.1.0] — 2026-08-22

### Added
- `c2proof migrate <repo> [--fixture]` — clone → scan → transpile → verify → REPORT.md → PR
- Flat-scan gate: refuses non-flat C repos with exact reason, exit code 2; inert metadata (Makefile, README, LICENSE) allowed
- Transpiler seam: real c2rust via pinned GHCR container (`c2rust-0.20.0`, source-built in-image), offline golden-fixture mode
- Verification report: build status, clippy warning count, per-file `unsafe fn` table, permanent not-safe-Rust label
- PR publish via `C2PROOF_GITHUB_TOKEN`: branch `c2proof/port` + REST PR with report committed inside
- GitHub Action (`action.yml`): composite, marketplace-ready
- CI: check (fmt/clippy/test/audit/deny), docker publish to GHCR, e2e tinyexpr acceptance + fixture refresh
- Marketing site: https://akashpriyadarshii.github.io/c2proof/ (docs/, full SEO meta + JSON-LD)
- Docs: README, INSTALL, CONTRIBUTING; 14 integration tests incl. exit-code mapping and GHCR lowercase invariant

### Notes
- Mechanical translation only. Output is NOT safe Rust and is labeled as such everywhere.
