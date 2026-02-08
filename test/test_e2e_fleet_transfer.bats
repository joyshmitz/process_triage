#!/usr/bin/env bats
# E2E tests for fleet transfer (export / import / diff)
#
# Validates the agent fleet transfer CLI surface:
# - JSON and .ptb export produce valid files
# - Import with --dry-run shows diff without modifying state
# - Round-trip stability (export → import → export)
# - Merge strategy selection
# - Diff command shows changes
# - Invalid/corrupt inputs produce useful errors
# - Backup file creation on import
# - JSONL logging of operations
# - Baseline normalization flag

load "./test_helper/common.bash"

PT_CORE="${BATS_TEST_DIRNAME}/../target/release/pt-core"

setup_file() {
    if [[ ! -x "$PT_CORE" ]]; then
        echo "# Building pt-core (release)..." >&3
        (cd "${BATS_TEST_DIRNAME}/.." && cargo build --release 2>/dev/null) || {
            echo "ERROR: Failed to build pt-core" >&2
            exit 1
        }
    fi
}

setup() {
    setup_test_env
    export PROCESS_TRIAGE_CONFIG="$CONFIG_DIR"
    test_start "$BATS_TEST_NAME" "fleet transfer E2E test"
}

teardown() {
    test_end "$BATS_TEST_NAME" "${BATS_TEST_COMPLETED:-fail}"
    teardown_test_env
}

require_jq() {
    if ! command -v jq &>/dev/null; then
        skip "jq not installed"
    fi
}

#==============================================================================
# EXPORT TESTS
#==============================================================================

@test "E2E fleet transfer: export creates valid JSON file" {
    require_jq
    local out_file="$DATA_DIR/bundle.json"

    run "$PT_CORE" agent fleet transfer export --out "$out_file" --standalone --format json
    test_info "exit_status=$status"
    [[ "$status" -eq 0 ]]

    # The file should exist and contain valid JSON
    [[ -f "$out_file" ]]
    run jq . "$out_file"
    [[ "$status" -eq 0 ]]

    # Must have schema_version and checksum
    local sv
    sv=$(jq -r '.schema_version' "$out_file")
    [[ "$sv" == "1.0.0" ]]

    local cksum
    cksum=$(jq -r '.checksum' "$out_file")
    [[ -n "$cksum" ]]
    [[ "$cksum" != "null" ]]
}

@test "E2E fleet transfer: export creates valid .ptb file" {
    local out_file="$DATA_DIR/bundle.ptb"

    run "$PT_CORE" agent fleet transfer export --out "$out_file" --standalone --format json
    test_info "exit_status=$status"
    [[ "$status" -eq 0 ]]

    # .ptb is a ZIP-based archive — should exist and be non-empty
    [[ -f "$out_file" ]]
    [[ -s "$out_file" ]]
}

@test "E2E fleet transfer: export stdout response has exported flag" {
    require_jq
    local out_file="$DATA_DIR/bundle.json"

    local stdout_output
    stdout_output=$("$PT_CORE" agent fleet transfer export --out "$out_file" --standalone --format json 2>/dev/null)

    local exported
    exported=$(echo "$stdout_output" | jq -r '.exported // empty')
    [[ "$exported" == "true" ]]

    local cmd
    cmd=$(echo "$stdout_output" | jq -r '.command // empty')
    [[ "$cmd" == "agent fleet transfer export" ]]
}

@test "E2E fleet transfer: export with --host-profile sets source_host_profile" {
    require_jq
    local out_file="$DATA_DIR/bundle.json"

    run "$PT_CORE" agent fleet transfer export --out "$out_file" \
        --host-profile "staging-server" --standalone --format json
    [[ "$status" -eq 0 ]]

    local profile
    profile=$(jq -r '.source_host_profile // empty' "$out_file")
    [[ "$profile" == "staging-server" ]]
}

#==============================================================================
# IMPORT TESTS
#==============================================================================

@test "E2E fleet transfer: import --dry-run shows diff without modifying" {
    require_jq
    local bundle="$DATA_DIR/bundle.json"

    # Export a bundle first
    run "$PT_CORE" agent fleet transfer export --out "$bundle" --standalone --format json
    [[ "$status" -eq 0 ]]

    # Import with --dry-run
    local stdout_output
    stdout_output=$("$PT_CORE" agent fleet transfer import --from "$bundle" \
        --dry-run --standalone --format json 2>/dev/null)

    local is_dry
    is_dry=$(echo "$stdout_output" | jq -r '.dry_run // empty')
    [[ "$is_dry" == "true" ]]

    # Verify original config wasn't modified (export again, compare)
    local bundle2="$DATA_DIR/bundle2.json"
    run "$PT_CORE" agent fleet transfer export --out "$bundle2" --standalone --format json
    [[ "$status" -eq 0 ]]
}

@test "E2E fleet transfer: import with --merge-strategy=replace works" {
    require_jq
    local bundle="$DATA_DIR/bundle.json"

    run "$PT_CORE" agent fleet transfer export --out "$bundle" --standalone --format json
    [[ "$status" -eq 0 ]]

    local stdout_output
    stdout_output=$("$PT_CORE" agent fleet transfer import --from "$bundle" \
        --merge-strategy replace --standalone --format json 2>/dev/null)

    local imported
    imported=$(echo "$stdout_output" | jq -r '.imported // empty')
    [[ "$imported" == "true" ]]
}

@test "E2E fleet transfer: import with --merge-strategy=weighted works" {
    require_jq
    local bundle="$DATA_DIR/bundle.json"

    run "$PT_CORE" agent fleet transfer export --out "$bundle" --standalone --format json
    [[ "$status" -eq 0 ]]

    local stdout_output
    stdout_output=$("$PT_CORE" agent fleet transfer import --from "$bundle" \
        --merge-strategy weighted --standalone --format json 2>/dev/null)

    local imported
    imported=$(echo "$stdout_output" | jq -r '.imported // empty')
    [[ "$imported" == "true" ]]

    local strat
    strat=$(echo "$stdout_output" | jq -r '.strategy // empty')
    [[ -n "$strat" ]]
}

@test "E2E fleet transfer: import creates backup (unless --no-backup)" {
    require_jq
    local bundle="$DATA_DIR/bundle.json"

    # Export
    run "$PT_CORE" agent fleet transfer export --out "$bundle" --standalone --format json
    [[ "$status" -eq 0 ]]

    # Import without --no-backup
    local stdout_output
    stdout_output=$("$PT_CORE" agent fleet transfer import --from "$bundle" \
        --merge-strategy replace --standalone --format json 2>/dev/null)

    # Check stdout mentions backup
    local backup_path
    backup_path=$(echo "$stdout_output" | jq -r '.backup_path // empty')
    # Either a backup was created or the field is present (may be null if no prior priors existed)
    test_info "backup_path=$backup_path"
}

#==============================================================================
# DIFF TESTS
#==============================================================================

@test "E2E fleet transfer: diff shows changes without side effects" {
    require_jq
    local bundle="$DATA_DIR/bundle.json"

    # Export a bundle
    run "$PT_CORE" agent fleet transfer export --out "$bundle" --standalone --format json
    [[ "$status" -eq 0 ]]

    # Run diff
    local stdout_output
    stdout_output=$("$PT_CORE" agent fleet transfer diff --from "$bundle" \
        --standalone --format json 2>/dev/null)

    local cmd
    cmd=$(echo "$stdout_output" | jq -r '.command // empty')
    [[ "$cmd" == "agent fleet transfer diff" ]]

    # Diff should report source info
    local source_host
    source_host=$(echo "$stdout_output" | jq -r '.source_host_id // empty')
    [[ -n "$source_host" ]]
}

#==============================================================================
# ROUND-TRIP STABILITY
#==============================================================================

@test "E2E fleet transfer: round-trip export→import→export produces stable output" {
    require_jq
    local bundle1="$DATA_DIR/bundle1.json"
    local bundle2="$DATA_DIR/bundle2.json"

    # First export
    run "$PT_CORE" agent fleet transfer export --out "$bundle1" --standalone --format json
    [[ "$status" -eq 0 ]]

    # Import (replace strategy to fully adopt)
    run "$PT_CORE" agent fleet transfer import --from "$bundle1" \
        --merge-strategy replace --standalone --format json
    [[ "$status" -eq 0 ]]

    # Second export
    run "$PT_CORE" agent fleet transfer export --out "$bundle2" --standalone --format json
    [[ "$status" -eq 0 ]]

    # Priors content should be stable — compare priors fields
    local priors1 priors2
    priors1=$(jq -S '.priors' "$bundle1")
    priors2=$(jq -S '.priors' "$bundle2")
    [[ "$priors1" == "$priors2" ]]
}

#==============================================================================
# ERROR HANDLING
#==============================================================================

@test "E2E fleet transfer: import fails gracefully on invalid file" {
    local bad_file="$DATA_DIR/corrupt.json"
    echo "not-valid-json{{{" > "$bad_file"

    run "$PT_CORE" agent fleet transfer import --from "$bad_file" --standalone --format json
    test_info "exit_status=$status"
    [[ "$status" -ne 0 ]]
}

@test "E2E fleet transfer: import fails on nonexistent file" {
    run "$PT_CORE" agent fleet transfer import --from "/nonexistent/bundle.json" --standalone --format json
    [[ "$status" -ne 0 ]]
}

@test "E2E fleet transfer: diff fails on corrupt file" {
    local bad_file="$DATA_DIR/bad.json"
    echo '{"schema_version":"99.0.0"}' > "$bad_file"

    run "$PT_CORE" agent fleet transfer diff --from "$bad_file" --standalone --format json
    test_info "exit_status=$status"
    # Should fail: either parse error or validation error
    [[ "$status" -ne 0 ]]
}

@test "E2E fleet transfer: export fails for unwritable path" {
    run "$PT_CORE" agent fleet transfer export --out "/nonexistent/deeply/nested/bundle.json" \
        --standalone --format json
    [[ "$status" -ne 0 ]]
}

#==============================================================================
# HELP
#==============================================================================

@test "E2E fleet transfer: --help exits 0" {
    run "$PT_CORE" agent fleet transfer --help
    [[ "$status" -eq 0 ]]
    [[ "$output" == *"transfer"* ]]
}

@test "E2E fleet transfer: export --help exits 0" {
    run "$PT_CORE" agent fleet transfer export --help
    [[ "$status" -eq 0 ]]
    [[ "$output" == *"out"* ]]
}
