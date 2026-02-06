#!/usr/bin/env bats
# Error handling tests for pt bash wrapper

load "./test_helper/common.bash"

setup() {
    test_start "error tests" "validate error handling and edge cases"
    setup_test_env

    # This file was written against an older bash-first wrapper interface.
    # The authoritative CLI/error surface is now enforced by pt-core contract tests:
    # `test/pt_agent_contract.bats` plus Rust unit/integration tests.
    skip "legacy bash wrapper error-surface tests; superseded by pt-core contract tests"

    TEST_DIR="$( cd "$( dirname "$BATS_TEST_FILENAME" )" && pwd )"
    PROJECT_ROOT="$(dirname "$TEST_DIR")"
    PATH="$PROJECT_ROOT:$PATH"

    export PROCESS_TRIAGE_CONFIG="$CONFIG_DIR"
}

teardown() {
    teardown_test_env
    test_end "error tests" "pass"
}

# ============================================================================
# Invalid Commands
# ============================================================================

@test "pt unknown_command fails with error" {
    run pt unknown_xyz_123
    [ "$status" -ne 0 ]
    assert_contains "$output" "Unknown command" "should indicate unknown command"
}

@test "pt invalid_subcommand shows help hint" {
    run pt invalid_subcommand
    [ "$status" -ne 0 ]
    assert_contains "$output" "help" "should mention help"
}

@test "pt robot invalid_subcommand shows robot help" {
    run pt robot invalid_xyz
    # Should either fail or show help
    [ "$status" -eq 0 ] || [ "$status" -eq 1 ]
}

# ============================================================================
# Missing Arguments
# ============================================================================

@test "pt robot explain without --pid fails" {
    run pt robot explain
    [ "$status" -eq 1 ]
}

@test "pt robot apply --pids without value handles gracefully" {
    run pt robot apply --pids
    # May fail or treat as empty, but shouldn't crash
    [ "$status" -eq 0 ] || [ "$status" -eq 1 ] || [ "$status" -eq 2 ]
}

@test "pt robot plan --limit without value uses default" {
    skip_if_no_jq

    run pt robot plan --limit --format json
    # Should either use default or fail gracefully
    [ "$status" -eq 0 ] || [ "$status" -eq 1 ]
}

@test "pt robot plan --min-age without value uses default" {
    skip_if_no_jq

    run pt robot plan --min-age --format json
    # Should either use default or fail gracefully
    [ "$status" -eq 0 ] || [ "$status" -eq 1 ]
}

# ============================================================================
# Invalid Argument Values
# ============================================================================

@test "pt robot plan --min-age negative is handled" {
    skip_if_no_jq

    run pt robot plan --min-age -100 --format json
    # Bash arithmetic treats negative as valid, but may produce odd results
    # Key is it shouldn't crash
    [ "$status" -eq 0 ] || [ "$status" -eq 1 ]
}

@test "pt robot plan --limit negative is handled" {
    skip_if_no_jq

    run pt robot plan --limit -5 --format json
    # Should handle gracefully
    [ "$status" -eq 0 ] || [ "$status" -eq 1 ]
}

@test "pt robot plan --only invalid_value is handled" {
    skip_if_no_jq

    run pt robot plan --only invalid_filter --format json
    # Should either ignore or use default
    [ "$status" -eq 0 ]
}

@test "pt robot explain --pid non_numeric is handled" {
    skip_if_no_jq

    run pt robot explain --pid "not_a_pid" --format json
    [ "$status" -eq 0 ]

    # Should return error or not_found
    assert_contains "$output" "error" "should indicate error or not_found"
}

@test "pt robot apply --pids with invalid PIDs is handled" {
    skip_if_no_jq

    run pt robot apply --pids "abc,def,ghi" --yes --format json
    # Should handle gracefully (skip invalid)
    [ "$status" -eq 0 ]
}

# ============================================================================
# Format Argument Handling
# ============================================================================

@test "pt robot plan --format invalid uses default json" {
    run pt robot plan --format xyz
    # Should either use default or fail gracefully
    [ "$status" -eq 0 ] || [ "$status" -eq 1 ]
}

@test "pt robot plan with both --json and --md uses last" {
    run pt robot plan --json --md
    [ "$status" -eq 0 ]
    # Should output markdown (last flag wins)
    assert_contains "$output" "# pt robot plan" "should output markdown"
}

# ============================================================================
# Edge Case PIDs
# ============================================================================

@test "pt robot explain --pid 1 works (init)" {
    skip_if_no_jq

    run pt robot explain --pid 1 --format json
    [ "$status" -eq 0 ]
    # May or may not have permission to inspect PID 1
}

@test "pt robot explain --pid 0 is handled" {
    skip_if_no_jq

    run pt robot explain --pid 0 --format json
    [ "$status" -eq 0 ]
    # PID 0 doesn't exist, should return not_found
}

@test "pt robot explain --pid $$ works (current shell)" {
    skip_if_no_jq

    run pt robot explain --pid $$ --format json
    [ "$status" -eq 0 ]
}

@test "pt robot apply --pids $$ is blocked (self-protection)" {
    skip_if_no_jq

    run pt robot apply --pids $$ --yes --format json
    [ "$status" -eq 0 ]

    # Should skip self
    assert_contains "$output" "skipped" "should skip self"
}

# ============================================================================
# File System Errors
# ============================================================================

@test "handles unwritable config directory" {
    skip_if_root  # Root can write anywhere

    local readonly_config="$BATS_TEST_TMPDIR/readonly_config"
    mkdir -p "$readonly_config"
    chmod 555 "$readonly_config"
    export PROCESS_TRIAGE_CONFIG="$readonly_config"

    # Should fail gracefully or work in read-only mode
    run pt robot plan --format json
    # May succeed (read-only) or fail
    [ "$status" -eq 0 ] || [ "$status" -eq 1 ]

    # Cleanup
    chmod 755 "$readonly_config"
}

@test "handles corrupted decisions.json" {
    echo 'this is not json {{{' > "$CONFIG_DIR/decisions.json"

    run pt robot plan --format json
    # Should not crash, may produce warnings
    [ "$status" -eq 0 ]
}

@test "handles binary garbage in decisions.json" {
    # Write some binary data
    printf '\x00\x01\x02\x03\xff\xfe' > "$CONFIG_DIR/decisions.json"

    run pt robot plan --format json
    # Should handle gracefully
    [ "$status" -eq 0 ]
}

@test "handles very large decisions.json" {
    skip_if_no_jq

    # Generate large but valid JSON
    local large_json='{'
    for i in $(seq 1 100); do
        [ $i -gt 1 ] && large_json+=','
        large_json+="\"pattern_$i\": \"kill\""
    done
    large_json+='}'
    echo "$large_json" > "$CONFIG_DIR/decisions.json"

    run pt robot plan --format json
    [ "$status" -eq 0 ]
}

# ============================================================================
# Process Access Errors
# ============================================================================

@test "handles process that exits during scan" {
    # Start a short-lived process
    sleep 0.1 &
    local pid=$!

    # Wait for it to finish
    sleep 0.2

    run pt robot explain --pid $pid --format json
    [ "$status" -eq 0 ]
    # Should return not_found since process exited
}

@test "handles permission denied for process inspection" {
    skip_if_root  # Root can inspect anything

    # Try to inspect a root process (if any exist)
    local root_pid
    root_pid=$(ps -u root -o pid= 2>/dev/null | head -1 | tr -d ' ')

    if [ -n "$root_pid" ]; then
        run pt robot explain --pid "$root_pid" --format json
        # Should handle gracefully
        [ "$status" -eq 0 ]
    else
        skip "No root processes found"
    fi
}

# ============================================================================
# Signal Handling
# ============================================================================

@test "pt handles interrupt gracefully" {
    # This is hard to test automatically, but we can verify
    # the script doesn't leave zombie processes
    run pt --help
    [ "$status" -eq 0 ]
}

# ============================================================================
# Dependency Handling
# ============================================================================

@test "pt works without jq installed" {
    # Save original PATH
    local orig_path="$PATH"

    # Simulate a minimal PATH that excludes common jq install locations (e.g., /usr/bin).
    export PATH="$PROJECT_ROOT:$MOCK_BIN:/bin:/usr/sbin:/sbin"

    run pt --help
    [ "$status" -eq 0 ]

    # Restore PATH
    export PATH="$orig_path"
}

@test "pt works without gum in CI mode" {
    export CI=true

    # Save original PATH
    local orig_path="$PATH"

    # Simulate a minimal PATH that excludes common gum install locations (e.g., /usr/bin).
    export PATH="$PROJECT_ROOT:$MOCK_BIN:/bin:/usr/sbin:/sbin"

    run pt --help
    [ "$status" -eq 0 ]

    # Restore PATH
    export PATH="$orig_path"
}

# ============================================================================
# Output Robustness
# ============================================================================

@test "JSON output is always valid" {
    skip_if_no_jq

    run pt robot plan --format json
    [ "$status" -eq 0 ]

    # Verify JSON is parseable
    echo "$output" | jq '.' > /dev/null
    [ $? -eq 0 ]
}

@test "JSON output handles special characters in commands" {
    skip_if_no_jq

    run pt robot plan --format json
    [ "$status" -eq 0 ]

    # Output should be valid JSON even with special chars
    echo "$output" | jq '.' > /dev/null
    [ $? -eq 0 ]
}

@test "markdown output is well-formed" {
    run pt robot plan --format md
    [ "$status" -eq 0 ]

    # Should have header
    assert_contains "$output" "# pt robot plan" "should have markdown header"

    # Should have table
    assert_contains "$output" "|" "should have table delimiter"
}

# ============================================================================
# Empty Results
# ============================================================================

@test "empty candidate list is valid" {
    skip_if_no_jq

    # Use very restrictive filter
    run pt robot plan --min-age 999999999 --format json
    [ "$status" -eq 0 ]

    local count
    count=$(echo "$output" | jq '.candidates | length')
    [ "$count" = "0" ]
}

@test "empty kill list is valid" {
    skip_if_no_jq

    # Filter to only spare (likely empty or all spare)
    run pt robot plan --only spare --format json
    [ "$status" -eq 0 ]

    # kill_pids should be empty
    local kill_count
    kill_count=$(echo "$output" | jq '.recommended.kill_pids | length')
    [ "$kill_count" = "0" ]
}

# ============================================================================
# Concurrent Access
# ============================================================================

@test "parallel plan calls don't corrupt state" {
    skip_if_no_jq

    # Start multiple plan calls in background
    pt robot plan --format json > /dev/null &
    local pid1=$!
    pt robot plan --format json > /dev/null &
    local pid2=$!
    pt robot plan --format json > /dev/null &
    local pid3=$!

    # Wait for all
    wait $pid1 $pid2 $pid3

    # Decisions file should still be valid JSON
    if [ -f "$CONFIG_DIR/decisions.json" ]; then
        run jq '.' "$CONFIG_DIR/decisions.json"
        [ "$status" -eq 0 ]
    fi
}
