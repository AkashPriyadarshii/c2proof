#!/usr/bin/env bash
# Pre-push gate for workflow/action YAML. Run before every push that touches yml.
set -euo pipefail
cd "$(dirname "$0")/.."

python - << 'EOF'
import sys, glob
try:
    import yaml
except ImportError:
    sys.exit("PyYAML missing: pip install pyyaml")

fail = False
for f in sorted(glob.glob(".github/workflows/*.yml")) + ["action.yml"]:
    try:
        d = yaml.safe_load(open(f, encoding="utf-8"))
        assert isinstance(d, dict), "not a mapping"
        # workflows need jobs; composite actions need runs.using
        if f.startswith(".github/"):
            assert "on" in d or True  # 'on' parses as bool True in yaml 1.1; just require jobs
            assert "jobs" in d, "missing jobs"
        else:
            assert d.get("runs", {}).get("using"), "composite action missing runs.using"
            assert "name" in d and "description" in d, "marketplace metadata missing"
        print(f"OK   {f}")
    except Exception as e:
        print(f"FAIL {f}: {e}")
        fail = True

# cross-check: e2e binary path must match cargo's actual target dir layout
e2e = open(".github/workflows/e2e.yml", encoding="utf-8").read()
assert "cli/target/release/c2proof migrate" in e2e, "e2e binary path wrong (target lives in cli/target)"
# runner image referenced in code must be lowercase and pinned
lib = open("cli/src/lib.rs", encoding="utf-8").read()
import re
m = re.search(r'RUNNER_IMAGE: &str = "([^"]+)"', lib)
img = m.group(1)
assert img == img.lower(), "GHCR image must be lowercase"
assert ":c2rust-" in img, "image tag must pin c2rust version"
print(f"OK   image ref {img}")
sys.exit(1 if fail else 0)
EOF

echo "yml checks passed"
