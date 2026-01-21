#!/usr/bin/env bash
# E2E runner harness: executes BATS E2E suites with JSONL metadata + artifacts.
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
ARTIFACT_ROOT="${ARTIFACT_ROOT:-$ROOT_DIR/target/test-logs/e2e}"
LOG_DIR="$ARTIFACT_ROOT/logs"

mkdir -p \
    "$ARTIFACT_ROOT" \
    "$ARTIFACT_ROOT/artifacts" \
    "$ARTIFACT_ROOT/snapshots" \
    "$ARTIFACT_ROOT/plans" \
    "$ARTIFACT_ROOT/telemetry" \
    "$LOG_DIR"

export BATS_TEST_TMPDIR="${BATS_TEST_TMPDIR:-$ARTIFACT_ROOT/bats-tmp}"
export TEST_LOG_LEVEL="${TEST_LOG_LEVEL:-info}"
mkdir -p "$BATS_TEST_TMPDIR"

json_escape() {
    local s="$1"
    s=${s//\\/\\\\}
    s=${s//\"/\\\"}
    s=${s//$'\n'/\\n}
    s=${s//$'\r'/\\r}
    s=${s//$'\t'/\\t}
    printf '%s' "$s"
}

BATS_ARGS=()
if [[ "$#" -eq 0 ]]; then
    BATS_ARGS=("$ROOT_DIR/test/pt_e2e_real.bats")
else
    BATS_ARGS=("$@")
fi

start_ts=$(date -u '+%Y-%m-%dT%H:%M:%SZ')
start_epoch=$(date +%s)
git_sha=$(git -C "$ROOT_DIR" rev-parse --short HEAD 2>/dev/null || echo "")
os_name=$(uname -s 2>/dev/null || echo "unknown")
arch_name=$(uname -m 2>/dev/null || echo "unknown")
args_joined=$(printf '%s ' "${BATS_ARGS[@]}")
args_joined=${args_joined% }
args_esc=$(json_escape "$args_joined")
git_sha_esc=$(json_escape "$git_sha")
os_esc=$(json_escape "$os_name")
arch_esc=$(json_escape "$arch_name")
artifact_root_esc=$(json_escape "$ARTIFACT_ROOT")
log_dir_esc=$(json_escape "$LOG_DIR")
printf '{"ts":"%s","event":"bats_start","git_sha":"%s","os":"%s","arch":"%s","args":"%s","artifact_root":"%s","log_dir":"%s"}\n' \
    "$start_ts" \
    "$git_sha_esc" \
    "$os_esc" \
    "$arch_esc" \
    "$args_esc" \
    "$artifact_root_esc" \
    "$log_dir_esc" \
    >> "$LOG_DIR/e2e_runner.jsonl"

set +e
tap_path="$LOG_DIR/bats.tap"
stderr_path="$LOG_DIR/bats.stderr"
bats --tap "${BATS_ARGS[@]}" > "$tap_path" 2> "$stderr_path"
status=$?
set -e

end_epoch=$(date +%s)
end_ts=$(date -u '+%Y-%m-%dT%H:%M:%SZ')

duration_s=$((end_epoch - start_epoch))
tap_esc=$(json_escape "$tap_path")
stderr_esc=$(json_escape "$stderr_path")
tap_bytes=$(wc -c < "$tap_path" 2>/dev/null | tr -d ' ' || echo "0")
stderr_bytes=$(wc -c < "$stderr_path" 2>/dev/null | tr -d ' ' || echo "0")

printf '{"ts":"%s","event":"bats_complete","status":%s,"duration_s":%s,"start_ts":"%s","tap":"%s","stderr":"%s","tap_bytes":%s,"stderr_bytes":%s}\n' \
    "$end_ts" \
    "$status" \
    "$duration_s" \
    "$start_ts" \
    "$tap_esc" \
    "$stderr_esc" \
    "$tap_bytes" \
    "$stderr_bytes" \
    >> "$LOG_DIR/e2e_runner.jsonl"

printf '{"ts":"%s","event":"bats_metadata","git_sha":"%s","os":"%s","arch":"%s","args":"%s"}\n' \
    "$end_ts" \
    "$git_sha_esc" \
    "$os_esc" \
    "$arch_esc" \
    "$args_esc" \
    >> "$LOG_DIR/e2e_runner.jsonl"
echo "E2E run completed at $end_ts (status=$status, duration=${duration_s}s)"
exit "$status"
