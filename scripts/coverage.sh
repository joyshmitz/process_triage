#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

OUT_DIR="coverage"
HTML_DIR="${OUT_DIR}/html"
LCOV_PATH="${OUT_DIR}/coverage.lcov"
JSON_PATH="${OUT_DIR}/coverage.json"
SUMMARY_PATH="${OUT_DIR}/coverage_summary.json"
CONFIG_PATH="coverage.toml"

if ! command -v cargo >/dev/null 2>&1; then
  echo "Error: cargo is required." >&2
  exit 2
fi

if ! cargo llvm-cov --version >/dev/null 2>&1; then
  echo "Error: cargo-llvm-cov is required. Install with:" >&2
  echo "  cargo install cargo-llvm-cov" >&2
  exit 2
fi

mkdir -p "$OUT_DIR" "$HTML_DIR"

# Generate LCOV
cargo llvm-cov --workspace \
  --lcov --output-path "$LCOV_PATH"

# Generate HTML report
cargo llvm-cov --workspace \
  --html --output-dir "$HTML_DIR"

# Generate JSON for programmatic parsing
cargo llvm-cov --workspace \
  --json --output-path "$JSON_PATH"

python3 - << 'PY'
import json
import os
import re
import sys
import tomllib
from datetime import datetime, timezone

ROOT = os.getcwd()
config_path = os.path.join(ROOT, "coverage.toml")
json_path = os.path.join(ROOT, "coverage", "coverage.json")
summary_path = os.path.join(ROOT, "coverage", "coverage_summary.json")

if not os.path.exists(config_path):
    print("Missing coverage.toml", file=sys.stderr)
    sys.exit(2)
if not os.path.exists(json_path):
    print("Missing coverage.json", file=sys.stderr)
    sys.exit(2)

with open(config_path, "rb") as fh:
    config = tomllib.load(fh)

with open(json_path, "r", encoding="utf-8") as fh:
    data = json.load(fh)

payloads = data.get("data", [])
if not payloads:
    print("coverage.json missing data", file=sys.stderr)
    sys.exit(2)

payload = payloads[0]
files = payload.get("files", [])
totals = payload.get("totals", {})

crate_stats = {}

crate_pattern = re.compile(r"/crates/([^/]+)/")

for f in files:
    filename = f.get("filename", "")
    match = crate_pattern.search(filename)
    if not match:
        continue
    crate = match.group(1)
    summary = f.get("summary", {})
    lines = summary.get("lines", {})
    branches = summary.get("branches", {})

    if crate not in crate_stats:
        crate_stats[crate] = {
            "lines": {"covered": 0, "count": 0},
            "branches": {"covered": 0, "count": 0},
        }

    crate_stats[crate]["lines"]["covered"] += int(lines.get("covered", 0))
    crate_stats[crate]["lines"]["count"] += int(lines.get("count", 0))
    if branches:
        crate_stats[crate]["branches"]["covered"] += int(branches.get("covered", 0))
        crate_stats[crate]["branches"]["count"] += int(branches.get("count", 0))


def percent(covered, count):
    if count <= 0:
        return 0.0
    return (covered / count) * 100.0

per_crate_summary = {}
for crate, stats in sorted(crate_stats.items()):
    line_cov = percent(stats["lines"]["covered"], stats["lines"]["count"])
    branch_cov = percent(stats["branches"]["covered"], stats["branches"]["count"])
    per_crate_summary[crate] = {
        "lines": {
            "covered": stats["lines"]["covered"],
            "count": stats["lines"]["count"],
            "percent": round(line_cov, 2),
        },
        "branches": {
            "covered": stats["branches"]["covered"],
            "count": stats["branches"]["count"],
            "percent": round(branch_cov, 2),
        },
    }

# Overall totals
line_totals = totals.get("lines", {})
branch_totals = totals.get("branches", {})

summary = {
    "generated_at": datetime.now(timezone.utc).isoformat(),
    "git_sha": os.environ.get("GITHUB_SHA"),
    "tool": {
        "cargo_llvm_cov_version": os.popen("cargo llvm-cov --version").read().strip(),
    },
    "totals": {
        "lines": {
            "covered": int(line_totals.get("covered", 0)),
            "count": int(line_totals.get("count", 0)),
            "percent": round(percent(line_totals.get("covered", 0), line_totals.get("count", 0)), 2),
        },
        "branches": {
            "covered": int(branch_totals.get("covered", 0)),
            "count": int(branch_totals.get("count", 0)),
            "percent": round(percent(branch_totals.get("covered", 0), branch_totals.get("count", 0)), 2),
        },
    },
    "per_crate": per_crate_summary,
}

with open(summary_path, "w", encoding="utf-8") as fh:
    json.dump(summary, fh, indent=2, sort_keys=True)

# Enforce thresholds
failures = []

cfg_global = config.get("global", {})
min_line = float(cfg_global.get("min_line", 0))
min_branch = float(cfg_global.get("min_branch", 0))

if summary["totals"]["lines"]["percent"] < min_line:
    failures.append(f"global line coverage {summary['totals']['lines']['percent']}% < {min_line}%")
if summary["totals"]["branches"]["percent"] < min_branch:
    failures.append(f"global branch coverage {summary['totals']['branches']['percent']}% < {min_branch}%")

crate_cfg = config.get("crates", {})
for crate, cfg in crate_cfg.items():
    enforce = bool(cfg.get("enforce", True))
    if not enforce:
        continue
    if crate not in per_crate_summary:
        failures.append(f"crate {crate} missing from coverage report")
        continue
    crate_line = per_crate_summary[crate]["lines"]["percent"]
    crate_branch = per_crate_summary[crate]["branches"]["percent"]
    min_line = float(cfg.get("min_line", 0))
    min_branch = float(cfg.get("min_branch", 0))
    if crate_line < min_line:
        failures.append(f"crate {crate} line coverage {crate_line}% < {min_line}%")
    if min_branch > 0 and crate_branch < min_branch:
        failures.append(f"crate {crate} branch coverage {crate_branch}% < {min_branch}%")

if failures:
    print("Coverage thresholds failed:")
    for failure in failures:
        print(f"- {failure}")
    sys.exit(1)

print("Coverage thresholds OK")
PY
