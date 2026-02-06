#!/usr/bin/env bats
# E2E: dormant daemon monitoring with JSONL logs + artifact manifest (bd-1tes).
#
# This test is Linux-only: it builds pt-core with the `daemon` feature and runs
# a short-lived daemon cycle, validating:
# - daemon start/stop/status lifecycle
# - state.json tick persistence
# - lock contention produces an inbox JSONL entry
# - a manifest.json describing logs/artifacts passes schema+checksum validation

load "./test_helper/common.bash"

PT_CORE="${BATS_TEST_DIRNAME}/../target/release/pt-core"
PROJECT_ROOT="$(cd "${BATS_TEST_DIRNAME}/.." && pwd)"

require_linux() {
    if [[ "$(uname -s 2>/dev/null || echo unknown)" != "Linux" ]]; then
        skip "daemon E2E currently Linux-only"
    fi
}

require_python3() {
    if ! command -v python3 >/dev/null 2>&1; then
        skip "python3 not installed"
    fi
}

require_jq() {
    if ! command -v jq >/dev/null 2>&1; then
        skip "jq not installed"
    fi
}

setup_file() {
    require_linux

    if [[ ! -x "$PT_CORE" ]] || ! "$PT_CORE" --help 2>/dev/null | grep -qE '^[[:space:]]+daemon[[:space:]]'; then
        echo "# Building pt-core (release, daemon feature)..." >&3
        (cd "$PROJECT_ROOT" && cargo build -p pt-core --release --features daemon >/dev/null) || {
            echo "ERROR: Failed to build pt-core with daemon feature" >&2
            exit 1
        }
    fi
}

setup() {
    require_linux
    require_python3
    require_jq

    setup_test_env

    export PROJECT_ROOT
    export PROCESS_TRIAGE_CONFIG="$CONFIG_DIR"
    export PROCESS_TRIAGE_DATA="$DATA_DIR"

    mkdir -p "$PROCESS_TRIAGE_DATA"
    cp "$PROJECT_ROOT/test/fixtures/config/valid_priors.json" "$CONFIG_DIR/priors.json"
    cp "$PROJECT_ROOT/test/fixtures/config/valid_policy.json" "$CONFIG_DIR/policy.json"

    # Suite workspace for manifest/logs/artifacts (matches fixture layout).
    export E2E_RUN_ID="e2e-$(date -u '+%Y%m%d%H%M%S')-daemon"
    export SUITE_DIR="$TEST_DIR/daemon"
    mkdir -p \
        "$SUITE_DIR/logs" \
        "$SUITE_DIR/telemetry" \
        "$SUITE_DIR/artifacts" \
        "$SUITE_DIR/snapshots"

    # Route helper JSONL to the daemon log path so it can be referenced in the manifest.
    export TEST_LOG_FILE="$SUITE_DIR/logs/daemon.jsonl"

    # Make daemon tick fast and always consider triggering.
    cat > "$CONFIG_DIR/daemon.json" <<'EOF'
{
  "tick_interval_secs": 1,
  "max_cpu_percent": 1000.0,
  "max_rss_mb": 4096,
  "triggers": {
    "ewma_alpha": 0.3,
    "load_threshold": -1.0,
    "memory_threshold": 2.0,
    "orphan_threshold": 999999,
    "sustained_ticks": 1,
    "cooldown_ticks": 100
  },
  "escalation": {
    "min_interval_secs": 0,
    "allow_auto_mitigation": false,
    "max_deep_scan_targets": 1
  },
  "notifications": {
    "enabled": false,
    "desktop": false,
    "notify_cmd": null,
    "notify_arg": []
  }
}
EOF

    test_start "$BATS_TEST_NAME" "daemon monitoring manifest + lock contention inbox"
}

teardown() {
    # Best-effort cleanup: stop daemon if it was started.
    "$PT_CORE" -f json --config "$CONFIG_DIR" daemon stop >/dev/null 2>&1 || true
    test_end "$BATS_TEST_NAME" "${BATS_TEST_COMPLETED:-fail}"
    teardown_test_env
}

wait_for_file() {
    local path="$1"
    local timeout_s="${2:-10}"
    local start
    start="$(date +%s)"
    while true; do
        if [[ -f "$path" ]]; then
            return 0
        fi
        if (( $(date +%s) - start >= timeout_s )); then
            test_error "Timed out waiting for file: $path"
            return 1
        fi
        sleep 0.1
    done
}

log_cmd_event() {
    local event="$1"
    local command="$2"
    local exit_code="$3"
    local duration_ms="$4"
    local artifact_path="${5:-}"

    local ts
    ts="$(date -u '+%Y-%m-%dT%H:%M:%SZ')"
    local command_esc
    command_esc=$(json_escape "$command")

    if [[ -n "$artifact_path" ]]; then
        local artifact_esc
        artifact_esc=$(json_escape "$artifact_path")
        printf '{"event":"%s","timestamp":"%s","phase":"e2e","run_id":"%s","command":"%s","exit_code":%s,"duration_ms":%s,"artifacts":[{"path":"%s","kind":"output"}]}\n' \
            "$event" \
            "$ts" \
            "$E2E_RUN_ID" \
            "$command_esc" \
            "$exit_code" \
            "$duration_ms" \
            "$artifact_esc" \
            >> "$TEST_LOG_FILE"
    else
        printf '{"event":"%s","timestamp":"%s","phase":"e2e","run_id":"%s","command":"%s","exit_code":%s,"duration_ms":%s,"artifacts":[]}\n' \
            "$event" \
            "$ts" \
            "$E2E_RUN_ID" \
            "$command_esc" \
            "$exit_code" \
            "$duration_ms" \
            >> "$TEST_LOG_FILE"
    fi
}

@test "daemon monitoring produces manifest + inbox lock contention and restarts cleanly" {
    # Ensure we start from a clean slate even if a previous run crashed.
    "$PT_CORE" -f json --config "$CONFIG_DIR" daemon stop >/dev/null 2>&1 || true

    # Hold the global lock so the daemon defers escalation and writes an inbox lock-contention item.
    python3 - <<'PY' &
import fcntl, os, time
data_dir = os.environ["PROCESS_TRIAGE_DATA"]
path = os.path.join(data_dir, ".pt-lock")
os.makedirs(os.path.dirname(path), exist_ok=True)
f = open(path, "a+")
fcntl.flock(f.fileno(), fcntl.LOCK_EX)
time.sleep(3)
PY
    lock_pid=$!

    local status_json="$SUITE_DIR/artifacts/daemon_status.json"
    local start_ms end_ms duration_ms
    local cmd_str

    # Start daemon (background mode; writes pid/state under PROCESS_TRIAGE_DATA).
    cmd_str="\"$PT_CORE\" -f json --config \"$CONFIG_DIR\" daemon start > \"$SUITE_DIR/artifacts/daemon_start.json\""
    start_ms=$(date +%s%3N)
    run bash -c "$cmd_str"
    local start_code=$status
    end_ms=$(date +%s%3N)
    duration_ms=$((end_ms - start_ms))
    log_cmd_event "daemon_start" "$cmd_str" "$start_code" "$duration_ms" "artifacts/daemon_start.json"
    if [[ "$start_code" -ne 0 ]]; then
        test_error "daemon start failed (exit=$start_code)"
        return 1
    fi

    # Wait for state + inbox artifacts.
    wait_for_file "$PROCESS_TRIAGE_DATA/daemon/state.json" 10
    wait_for_file "$PROCESS_TRIAGE_DATA/inbox/items.jsonl" 10

    # Snapshot state/inbox for artifact manifest.
    cp "$PROCESS_TRIAGE_DATA/daemon/state.json" "$SUITE_DIR/snapshots/state.json"
    cp "$PROCESS_TRIAGE_DATA/inbox/items.jsonl" "$SUITE_DIR/artifacts/inbox_items.jsonl"

    # Assert lock contention entry exists.
    if ! grep -q '"type":"lock_contention"' "$SUITE_DIR/artifacts/inbox_items.jsonl"; then
        test_error "expected lock_contention inbox item"
        return 1
    fi

    # Status should be callable while daemon runs.
    cmd_str="\"$PT_CORE\" -f json --config \"$CONFIG_DIR\" daemon status > \"$status_json\""
    start_ms=$(date +%s%3N)
    run bash -c "$cmd_str"
    local status_code=$status
    end_ms=$(date +%s%3N)
    duration_ms=$((end_ms - start_ms))
    log_cmd_event "daemon_status" "$cmd_str" "$status_code" "$duration_ms" "artifacts/daemon_status.json"
    [[ "$status_code" -eq 0 ]]
    jq -e '.command == "daemon status"' "$status_json" >/dev/null

    # Stop daemon cleanly.
    cmd_str="\"$PT_CORE\" -f json --config \"$CONFIG_DIR\" daemon stop > \"$SUITE_DIR/artifacts/daemon_stop.json\""
    start_ms=$(date +%s%3N)
    run bash -c "$cmd_str"
    local stop_code=$status
    end_ms=$(date +%s%3N)
    duration_ms=$((end_ms - start_ms))
    log_cmd_event "daemon_stop" "$cmd_str" "$stop_code" "$duration_ms" "artifacts/daemon_stop.json"
    [[ "$stop_code" -eq 0 ]]

    # Ensure lock holder finished.
    wait "$lock_pid" 2>/dev/null || true

    # Restart should succeed after stop.
    cmd_str="\"$PT_CORE\" -f json --config \"$CONFIG_DIR\" daemon start > \"$SUITE_DIR/artifacts/daemon_restart.json\""
    start_ms=$(date +%s%3N)
    run bash -c "$cmd_str"
    local restart_code=$status
    end_ms=$(date +%s%3N)
    duration_ms=$((end_ms - start_ms))
    log_cmd_event "daemon_restart" "$cmd_str" "$restart_code" "$duration_ms" "artifacts/daemon_restart.json"
    [[ "$restart_code" -eq 0 ]]

    # Stop again (best-effort assertion).
    "$PT_CORE" -f json --config "$CONFIG_DIR" daemon stop >/dev/null 2>&1 || true

    # Write a tiny telemetry placeholder so the manifest has at least one telemetry artifact.
    printf '{"event":"daemon_telemetry","timestamp":"%s","run_id":"%s","kind":"telemetry"}\n' \
        "$(date -u '+%Y-%m-%dT%H:%M:%SZ')" \
        "$E2E_RUN_ID" \
        > "$SUITE_DIR/telemetry/daemon_telemetry.jsonl"

    # Build a minimal manifest.json and validate it (schema + checksum + file sha256s).
    python3 - <<'PY'
import hashlib
import json
import os
from pathlib import Path
from datetime import datetime, timezone

root = Path(os.environ["PROJECT_ROOT"])
suite_dir = Path(os.environ["SUITE_DIR"])
run_id = os.environ["E2E_RUN_ID"]

def sha256_path(p: Path) -> str:
    h = hashlib.sha256()
    h.update(p.read_bytes())
    return h.hexdigest()

log_path = suite_dir / "logs" / "daemon.jsonl"
telemetry_path = suite_dir / "telemetry" / "daemon_telemetry.jsonl"

manifest = {
  "schema_version": "1.0.0",
  "run_id": run_id,
  "suite": "daemon",
  "test_id": "daemon-monitoring",
  "timestamp": datetime.now(timezone.utc).replace(microsecond=0).isoformat().replace("+00:00", "Z"),
  "env": {
    "os": "linux",
    "arch": os.uname().machine,
    "kernel": os.uname().release,
    "ci_provider": os.environ.get("CI_PROVIDER", "ci"),
    "pt_version": "dev",
    "runner": "bats"
  },
  "commands": [
    {
      "argv": ["pt-core", "daemon", "status"],
      "exit_code": 0,
      "duration_ms": 0
    }
  ],
  "logs": [
    {
      "path": "logs/daemon.jsonl",
      "kind": "jsonl",
      "sha256": sha256_path(log_path),
      "bytes": log_path.stat().st_size
    }
  ],
  "artifacts": [
    {
      "path": "telemetry/daemon_telemetry.jsonl",
      "kind": "telemetry",
      "sha256": sha256_path(telemetry_path),
      "bytes": telemetry_path.stat().st_size,
      "redaction_profile": "safe"
    },
    {
      "path": "snapshots/state.json",
      "kind": "snapshot",
      "sha256": sha256_path(suite_dir / "snapshots" / "state.json"),
      "bytes": (suite_dir / "snapshots" / "state.json").stat().st_size,
      "redaction_profile": "safe"
    },
    {
      "path": "artifacts/inbox_items.jsonl",
      "kind": "daemon",
      "sha256": sha256_path(suite_dir / "artifacts" / "inbox_items.jsonl"),
      "bytes": (suite_dir / "artifacts" / "inbox_items.jsonl").stat().st_size,
      "redaction_profile": "safe"
    }
  ],
  "metrics": {
    "timings_ms": { "total": 0, "setup": 0, "run": 0 },
    "counts": { "tests": 1, "failures": 0 },
    "flake_retries": 0
  }
}

clone = dict(manifest)
clone.pop("manifest_sha256", None)
canonical = json.dumps(clone, sort_keys=True, separators=(",", ":"), ensure_ascii=False).encode("utf-8")
manifest["manifest_sha256"] = hashlib.sha256(canonical).hexdigest()

out = suite_dir / "manifest.json"
out.write_text(json.dumps(manifest, indent=2), encoding="utf-8")
PY
    local manifest_path="$SUITE_DIR/manifest.json"
    run "$PROJECT_ROOT/scripts/validate_e2e_manifest.py" "$manifest_path"
    if [[ "$status" -ne 0 ]]; then
        test_error "validate_e2e_manifest.py failed (status=$status)"
        test_error "validator output: $output"
        return 1
    fi

    BATS_TEST_COMPLETED=pass
}
