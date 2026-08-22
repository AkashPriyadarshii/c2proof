# Installing c2proof

## Requirements

- Rust stable (1.75+): [rustup.rs](https://rustup.rs)
- Git
- Docker Desktop — **optional**: only for real c2rust runs. Offline fixture mode needs none.
- Windows/macOS users: real transpilation runs on CI only. Local = fixture mode + verification gate.

## 1. Build the CLI

```bash
git clone https://github.com/AkashPriyadarshii/c2proof
cd c2proof/cli
cargo install --path .
```

## 2. Try offline (no Docker needed)

```bash
c2proof migrate path/to/flat-c-project --fixture
```

Runs the full pipeline against the committed golden fixture instead of real c2rust.

## 3. Real migration (Docker or CI)

```bash
c2proof migrate https://github.com/someone/tinyexpr   # pulls pinned runner from GHCR
```

Or use as a GitHub Action (marketplace-ready):

```yaml
- uses: AkashPriyadarshii/c2proof@v0
  with:
    repo-url: ${{ github.server_url }}/${{ github.repository }}
```

## Exit Codes

| Code | Meaning |
|---|---|
| 0 | success |
| 1 | tool/environment failure (clone fail, docker missing, build fail) |
| 2 | refused: repo shape unsupported — reason printed |

## Troubleshooting

- **`docker not found`** → install Docker Desktop, or rerun with `--fixture`.
- **`refused: subdirectory ...`** → v0 supports flat C projects only. Flatten or wait for v0.2.
- **`fixture missing`** → regenerate via the CI e2e job, then pull main.

## As a GitHub Action

```yaml
jobs:
  migrate:
    runs-on: ubuntu-latest
    permissions:
      contents: write
      pull-requests: write
    steps:
      - uses: AkashPriyadarshii/c2proof@v0
        with:
          repo-url: ${{ github.server_url }}/${{ github.repository }}
```

Site & docs: https://akashpriyadarshii.github.io/c2proof/
