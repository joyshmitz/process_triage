#!/usr/bin/env bats

setup() {
    TEST_DIR="$( cd "$( dirname "$BATS_TEST_FILENAME" )" && pwd )"
    PROJECT_ROOT="$(dirname "$TEST_DIR")"
    export PROJECT_ROOT
}

@test "Capabilities manifest schema is valid JSON" {
    run python3 - <<'PY'
import json
import os
from pathlib import Path
root = Path(os.environ["PROJECT_ROOT"])
with open(root / "specs" / "schemas" / "capabilities-manifest.schema.json", "r", encoding="utf-8") as f:
    schema = json.load(f)
assert schema.get("$schema")
assert schema.get("$id")
assert schema.get("title")
PY
    [ "$status" -eq 0 ]
}

@test "Capabilities manifest validator accepts fixtures" {
    local manifest="$PROJECT_ROOT/test/fixtures/capabilities/capabilities.json"
    run "$PROJECT_ROOT/scripts/validate_capabilities_manifest.py" "$manifest"
    [ "$status" -eq 0 ]
}
