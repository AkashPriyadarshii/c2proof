# Architecture — c2proof v0.1.0

## Pipeline
```
┌──────────┐   ┌─────────────┐   ┌────────────┐   ┌─────────────────┐   ┌───────────┐
│ clone    │ → │ scan (.c/.h)│ → │ c2rust     │ → │ cargo build +   │ → │ git push  │
│ repo     │   │ flat-dir only│   │ (pinned,   │   │ clippy (output  │   │ + PR +    │
│          │   │ refuse else │   │ dockerized)│   │ crate)          │   │ REPORT.md │
└──────────┘   └─────────────┘   └────────────┘   └─────────────────┘   └───────────┘
```

## Components
1. **cli/** — Rust binary (`clap` for args, nothing heavier)
   - `migrate` subcommand only
   - shells out to docker + git + cargo; orchestrates, doesn't reimplement
2. **docker/** — runner image
   - Base: debian-slim + rustup stable + prebuilt c2rust binary (pinned vX.Y.Z)
   - Published GHCR, tagged by c2rust version
3. **ci/** — GitHub Actions workflows
   - `check` (every push): fmt, clippy, test (fixtures), audit, deny
   - `e2e` (main + release tags): real c2rust in container, regenerates fixture if pinned version changed
   - `docker`: builds runner image → GHCR
4. **action/** — composite action, ~40 lines YAML
   - Installs CLI (or uses image), runs `migrate`, opens PR via `gh` or GITHUB_TOKEN

## Compute Split (8 GB laptop reality)
- **Local = verify everything before push.** No WSL, no Docker, no c2rust locally.
  - Golden fixtures: tinyexpr's transpiled output (generated once by CI) is committed to `tests/fixtures/`. Local "transpile" step reads the fixture → full pipeline testable offline.
  - Pre-push gate (all required):
    ```
    cargo fmt --check && cargo clippy -- -D warnings && cargo test && cargo audit && cargo deny check
    ```
- **GitHub Actions = all heavy work.** Real c2rust runs, Docker image build/push to GHCR, e2e on ubuntu-latest, fixture regeneration when c2rust version bumps.
- Laptop stays a thin client. Rust toolchain is the only local install.

## Key Decisions
- **Golden-fixture seam**: transpile step is a trait/trait-like boundary (`Transpiler`); prod impl shells out to c2rust, test impl reads fixture. Enables full-pipeline local testing without Linux.
- **Orchestrate via subprocess** (docker/git/cargo), not library bindings. Boring, debuggable, swappable.
- **Pinned everything**: c2rust version, container base, action inputs default-pinned. Upstream churn is risk #1.
- **Flat-scan gate first**: if repo has `configure*`, `Makefile`, `CMakeLists.txt`, or nested dirs beyond depth 2 → refuse with exact reason. Gate runs before any heavy work.
- **Output crate layout**: one lib crate per migration, `build.rs` empty, deps minimal.

## Verification (v0.1.0 level)
- Build success + clippy warnings + unresolved symbol list
- Unsafe-fn count per file (parse transpiled source, regex-level, not syn-level)
- NOT in scope: test parity harness, formal checks (v0.2+)

## Failure Modes & Handling
| failure | behavior |
|---|---|
| unsupported repo shape | refuse, print reason, exit 2 |
| c2rust fails | capture stderr tail into issue comment |
| cargo build fails | still open PR, report marks ❌ with error excerpt |
| docker missing | instruct install, exit 1 |

## Growth Path (post-v0.1)
1. Test-parity harness: run original C tests against port (oracle)
2. Accept agent-generated Rust as alternate input source
3. LLM refactor pass, each rewrite gated by test oracle
