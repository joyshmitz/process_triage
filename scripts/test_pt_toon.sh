#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
LOG_DIR="${SCRIPT_DIR}/../test_logs/pt_toon_${TIMESTAMP}"
mkdir -p "$LOG_DIR"

log() { echo "[$(date +%H:%M:%S)] $*" | tee -a "$LOG_DIR/test.log"; }
pass() { log "PASS: $*"; PASS_COUNT=$((PASS_COUNT+1)); }
fail() { log "FAIL: $*"; FAIL_COUNT=$((FAIL_COUNT+1)); FAILURES+=("$*"); }
skip() { log "SKIP: $*"; SKIP_COUNT=$((SKIP_COUNT+1)); }

PASS_COUNT=0
FAIL_COUNT=0
SKIP_COUNT=0
declare -a FAILURES=()

log "=========================================="
log "pt TOON Integration E2E Test Suite"
log "Log directory: $LOG_DIR"
log "=========================================="

find_pt_core() {
  local candidates=(
    "$SCRIPT_DIR/../target/release/pt-core"
    "$SCRIPT_DIR/../target/debug/pt-core"
    "$SCRIPT_DIR/../pt-core"
  )
  for c in "${candidates[@]}"; do
    if [ -x "$c" ]; then
      echo "$c"
      return 0
    fi
  done

  if command -v pt-core >/dev/null 2>&1; then
    command -v pt-core
    return 0
  fi

  return 1
}

log ""
log "Phase 1: Prerequisites"
PT_BIN="$(find_pt_core || true)"
if [ -z "$PT_BIN" ]; then
  fail "pt-core binary not found (build with: cargo build -p pt-core)"
  exit 1
fi
pass "pt-core binary found: $PT_BIN"

PT_VERSION=$($PT_BIN --version 2>&1 | head -1 || echo "unknown")
log "  pt-core version: $PT_VERSION"

if $PT_BIN --help 2>&1 | grep -qi "toon"; then
  pass "TOON mentioned in --help"
else
  fail "TOON not mentioned in --help"
fi

CMD_BASE=("$PT_BIN" config show)

log ""
log "Phase 2: Format Flag Tests"

log "Test: config show --format json"
if "${CMD_BASE[@]}" --format json > "$LOG_DIR/config_json.out" 2>"$LOG_DIR/config_json.err"; then
  if head -1 "$LOG_DIR/config_json.out" | grep -q "^{"; then
    pass "--format json produces JSON"
  else
    fail "--format json output does not start with '{'"
  fi
else
  fail "--format json command failed"
fi

log "Test: config show --format toon"
if "${CMD_BASE[@]}" --format toon > "$LOG_DIR/config_toon.out" 2>"$LOG_DIR/config_toon.err"; then
  if head -1 "$LOG_DIR/config_toon.out" | grep -qv "^{"; then
    pass "--format toon produces non-JSON output"
  else
    fail "--format toon output looks like JSON"
  fi
else
  fail "--format toon command failed"
fi

log ""
log "Phase 3: Environment Variable Tests"

log "Test: PT_OUTPUT_FORMAT=toon"
if PT_OUTPUT_FORMAT=toon "${CMD_BASE[@]}" > "$LOG_DIR/env_pt_toon.out" 2>"$LOG_DIR/env_pt_toon.err"; then
  if head -1 "$LOG_DIR/env_pt_toon.out" | grep -qv "^{"; then
    pass "PT_OUTPUT_FORMAT=toon produces TOON output"
  else
    fail "PT_OUTPUT_FORMAT=toon not honored"
  fi
else
  fail "PT_OUTPUT_FORMAT=toon command failed"
fi

log "Test: CLI --format json overrides PT_OUTPUT_FORMAT=toon"
if PT_OUTPUT_FORMAT=toon "${CMD_BASE[@]}" --format json > "$LOG_DIR/env_cli_override.out" 2>"$LOG_DIR/env_cli_override.err"; then
  if head -1 "$LOG_DIR/env_cli_override.out" | grep -q "^{"; then
    pass "CLI --format json overrides PT_OUTPUT_FORMAT"
  else
    fail "CLI override failed (expected JSON)"
  fi
else
  fail "CLI override test command failed"
fi

log "Test: TOON_DEFAULT_FORMAT=toon"
if TOON_DEFAULT_FORMAT=toon "${CMD_BASE[@]}" > "$LOG_DIR/env_default_toon.out" 2>"$LOG_DIR/env_default_toon.err"; then
  if head -1 "$LOG_DIR/env_default_toon.out" | grep -qv "^{"; then
    pass "TOON_DEFAULT_FORMAT=toon produces TOON output"
  else
    fail "TOON_DEFAULT_FORMAT=toon not honored"
  fi
else
  fail "TOON_DEFAULT_FORMAT=toon command failed"
fi

log "Test: PT_OUTPUT_FORMAT=json overrides TOON_DEFAULT_FORMAT=toon"
if PT_OUTPUT_FORMAT=json TOON_DEFAULT_FORMAT=toon "${CMD_BASE[@]}" > "$LOG_DIR/env_precedence.out" 2>"$LOG_DIR/env_precedence.err"; then
  if head -1 "$LOG_DIR/env_precedence.out" | grep -q "^{"; then
    pass "Env precedence honored (json)"
  else
    fail "Precedence failed: expected JSON"
  fi
else
  fail "Precedence test command failed"
fi

log ""
log "Phase 4: Round-Trip Verification"
if command -v tru >/dev/null 2>&1 && command -v jq >/dev/null 2>&1; then
  if [ -s "$LOG_DIR/config_toon.out" ] && [ -s "$LOG_DIR/config_json.out" ]; then
    if tru --decode < "$LOG_DIR/config_toon.out" > "$LOG_DIR/decoded.json" 2>"$LOG_DIR/decoded.err"; then
      jq -S . "$LOG_DIR/config_json.out" > "$LOG_DIR/config_json_sorted.json" 2>/dev/null || true
      jq -S . "$LOG_DIR/decoded.json" > "$LOG_DIR/decoded_sorted.json" 2>/dev/null || true
      if diff -q "$LOG_DIR/config_json_sorted.json" "$LOG_DIR/decoded_sorted.json" >/dev/null 2>&1; then
        pass "TOON round-trip preserves data"
      else
        fail "TOON round-trip data mismatch"
      fi
    else
      fail "tru --decode failed"
    fi
  else
    skip "Round-trip skipped (missing outputs)"
  fi
else
  skip "Round-trip skipped (tru/jq not available)"
fi

log ""
log "Phase 5: Token Savings"
if [ -s "$LOG_DIR/config_json.out" ] && [ -s "$LOG_DIR/config_toon.out" ]; then
  JSON_SIZE=$(wc -c < "$LOG_DIR/config_json.out")
  TOON_SIZE=$(wc -c < "$LOG_DIR/config_toon.out")
  if [ "$JSON_SIZE" -gt 0 ]; then
    SAVINGS=$(( 100 - (TOON_SIZE * 100 / JSON_SIZE) ))
    log "  JSON size: $JSON_SIZE bytes"
    log "  TOON size: $TOON_SIZE bytes"
    log "  Savings: ${SAVINGS}%"
    pass "Token savings computed"
  else
    skip "Token savings skipped (empty JSON output)"
  fi
else
  skip "Token savings skipped (missing outputs)"
fi

log ""
log "=========================================="
log "TEST SUMMARY"
log "=========================================="
log "Passed: $PASS_COUNT"
log "Failed: $FAIL_COUNT"
log "Skipped: $SKIP_COUNT"
log "Log directory: $LOG_DIR"

if [ "$FAIL_COUNT" -gt 0 ]; then
  log ""
  log "FAILURES:"
  for f in "${FAILURES[@]}"; do
    log "  - $f"
  done
  exit 1
fi

log ""
log "All tests passed!"
exit 0
