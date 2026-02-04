#!/usr/bin/env bats

setup() {
    TEST_DIR="$( cd "$( dirname "$BATS_TEST_FILENAME" )" && pwd )"
    PROJECT_ROOT="$(dirname "$TEST_DIR")"
    export PROJECT_ROOT
}

@test "E2E manifest schema is valid JSON" {
    run python3 - <<'PY'
import json
import os
from pathlib import Path
root = Path(os.environ["PROJECT_ROOT"])
with open(root / "specs" / "schemas" / "e2e-artifact-manifest.schema.json", "r", encoding="utf-8") as f:
    schema = json.load(f)
assert schema.get("$schema")
assert schema.get("$id")
assert schema.get("title")
PY
    [ "$status" -eq 0 ]
}

@test "E2E manifest validator accepts fixtures" {
    local manifests=(
        "$PROJECT_ROOT/test/fixtures/manifest_examples/tui/manifest.json"
        "$PROJECT_ROOT/test/fixtures/manifest_examples/install/manifest.json"
        "$PROJECT_ROOT/test/fixtures/manifest_examples/daemon/manifest.json"
        "$PROJECT_ROOT/test/fixtures/manifest_examples/bundle_report/manifest.json"
    )

    for manifest in "${manifests[@]}"; do
        run "$PROJECT_ROOT/scripts/validate_e2e_manifest.py" "$manifest"
        [ "$status" -eq 0 ]
    done
}

@test "E2E manifest validator accepts minor schema bump" {
    run python3 - <<'PY'
import json
import hashlib
import os
from pathlib import Path
from tempfile import mkdtemp
root = Path(os.environ["PROJECT_ROOT"])
source = root / "test" / "fixtures" / "manifest_examples" / "tui" / "manifest.json"
workspace = Path(mkdtemp(prefix="e2e_manifest_"))
manifest_path = workspace / "manifest.json"
manifest = json.loads(source.read_text(encoding="utf-8"))
manifest["schema_version"] = "1.1.0"
clone = dict(manifest)
clone.pop("manifest_sha256", None)
canonical = json.dumps(clone, sort_keys=True, separators=(",", ":"), ensure_ascii=False).encode("utf-8")
manifest["manifest_sha256"] = hashlib.sha256(canonical).hexdigest()
manifest_path.write_text(json.dumps(manifest, indent=2), encoding="utf-8")
print(manifest_path)
PY
    [ "$status" -eq 0 ]
    manifest_path="$output"
    run "$PROJECT_ROOT/scripts/validate_e2e_manifest.py" "$manifest_path"
    [ "$status" -eq 0 ]
}

@test "E2E manifest validator rejects checksum mismatch" {
    run python3 - <<'PY'
import json
import os
from pathlib import Path
from shutil import copytree
from tempfile import mkdtemp
root = Path(os.environ["PROJECT_ROOT"])
source_dir = root / "test" / "fixtures" / "manifest_examples" / "daemon"
workspace = Path(mkdtemp(prefix="e2e_manifest_badhash_"))
copytree(source_dir, workspace, dirs_exist_ok=True)
log_path = workspace / "logs" / "daemon.jsonl"
log_path.write_text("tampered\n", encoding="utf-8")
print(workspace / "manifest.json")
PY
    [ "$status" -eq 0 ]
    manifest_path="$output"
    run "$PROJECT_ROOT/scripts/validate_e2e_manifest.py" "$manifest_path"
    [ "$status" -ne 0 ]
}

@test "E2E manifest validator requires redaction_profile" {
    run python3 - <<'PY'
import json
import hashlib
import os
from pathlib import Path
from shutil import copytree
from tempfile import mkdtemp
root = Path(os.environ["PROJECT_ROOT"])
source_dir = root / "test" / "fixtures" / "manifest_examples" / "tui"
workspace = Path(mkdtemp(prefix="e2e_manifest_noredact_"))
copytree(source_dir, workspace, dirs_exist_ok=True)
manifest_path = workspace / "manifest.json"
manifest = json.loads(manifest_path.read_text(encoding="utf-8"))
for entry in manifest.get("artifacts", []):
    if entry.get("kind") == "telemetry":
        entry.pop("redaction_profile", None)
        break
clone = dict(manifest)
clone.pop("manifest_sha256", None)
canonical = json.dumps(clone, sort_keys=True, separators=(",", ":"), ensure_ascii=False).encode("utf-8")
manifest["manifest_sha256"] = hashlib.sha256(canonical).hexdigest()
manifest_path.write_text(json.dumps(manifest, indent=2), encoding="utf-8")
print(manifest_path)
PY
    [ "$status" -eq 0 ]
    manifest_path="$output"
    run "$PROJECT_ROOT/scripts/validate_e2e_manifest.py" "$manifest_path"
    [ "$status" -ne 0 ]
}
