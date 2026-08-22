# Changelog

All notable changes. Format: [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), semver.

## [Unreleased]

### Added
- CLI scaffold: `c2proof migrate <repo> [--fixture]` (clap, anyhow)
- Flat-dir scan gate with refusal reasons, exit code 2
- Pipeline wiring: clone → scan → transpile → cargo check
- Transpiler seam: fixture mode (offline) / c2rust docker mode (`ghcr.io/akashpriyadarshii/c2proof/runner:c2rust-0.20.0`)
- Docker runner image: rust:1-slim + pinned prebuilt c2rust 0.20.0, pushed to GHCR by CI only
- CI: check workflow (fmt, clippy -D warnings, test, audit, deny) + docker publish workflow
- Docs: README (SEO), INSTALL.md, CONTRIBUTING.md, LICENSE (MIT)

## [0.1.0] — unreleased

Planned for tag: e2e tinyexpr green in CI, REPORT.md generator, PR bot, composite action YAML.
