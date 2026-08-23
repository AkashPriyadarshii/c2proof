# STATE — c2proof

## Phase
v0.1.0 is RELEASED. The pipeline is fully green, README is written, and the `v0.1.0` tag is cut and published.

## Locked Decisions
- Name: `c2proof` · repo: AkashPriyadarshii/c2proof (public, to be created)
- Product: CLI + GitHub Action; C repo → compiling Rust PR + verification report
- Positioning: verifier-first (translation is commoditized; trust artifact is the moat)
- Compute split: local = fixtures + fmt/clippy/test/audit/deny gate. GH Actions = real c2rust, Docker→GHCR, e2e
- No WSL, no Docker on dev laptop (8 GB RAM)
- Dogfood target v0.1.0: tinyexpr ONLY
- Scope frozen per PRD.md; non-goals listed there are final for v0.1.0

## Artifacts Present
Site live: https://akashpriyadarshii.github.io/c2proof/ (Pages from docs/, homepage set on repo). docs/: index.html (industrial-utilitarian, full SEO meta + JSON-LD SoftwareApplication/WebSite, sitemap, robots.txt, favicon.svg, .nojekyll). cli/ = lib (pipeline: clone→scan→transpile→verify→REPORT.md→PR) + thin bin. Transpiler seam: fixture mode (`C2PROOF_FIXTURE_DIR`/`C2PROOF_WORK_DIR`) / c2rust docker `ghcr.io/akashpriyadarshii/c2proof/runner:c2rust-0.20.0` (built from source in-image — upstream ships NO release assets). PR publish via C2PROOF_GITHUB_TOKEN. Workflows green: docker.yml, e2e.yml. 12 integration tests in cli/tests/pipeline.rs.

## Not Started
None. v0.1.0 is RELEASED.

## Open Items
- [ ] CI run on first push — verify check workflow green (incl. audit-check working-directory:cli)
