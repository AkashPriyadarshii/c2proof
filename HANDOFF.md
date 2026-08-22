# SESSION HANDOFF — c2proof

## Read First (in order)
1. STATE.md — phase, locked decisions
2. AGENTS.md — rules
3. PRD.md — scope (frozen)
4. tasks.md — T01–T12

## This Session's Job: SCAFFOLD ONLY (tasks T03–T05 + T10 partial)
1. Verify name availability (`gh repo view AkashPriyadarshii/c2proof` fails = free); create public repo
2. Git init in `~/Desktop/c2proof`, move md files to repo root
3. Scaffold:
   - `cli/` Rust binary crate (clap, `migrate` subcommand skeleton)
   - `Transpiler` enum/trait: `Fixture` variant first (reads tests/fixtures/tinyexpr/)
   - scan gate: refuse non-flat repos with reason, exit 2
   - `tests/fixtures/` placeholder (real fixture generated later by CI job)
   - `.github/workflows/check.yml`: fmt + clippy -D warnings + test + audit + deny
4. Wire pre-push gate as git hook or documented command
5. Commit per logical unit, push main

## Do NOT
- Install WSL/Docker/c2rust locally
- Touch T06+ pipeline wiring beyond skeleton stubs
- Add deps beyond clap + anyhow (+ tempfile for tests)
- Revisit planning decisions in STATE.md

## Definition of Done (this session)
`cargo run -- migrate <any-flat-c-url> --fixture` prints refusal/success path correctly; check workflow green.
