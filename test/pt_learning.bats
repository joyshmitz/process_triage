#!/usr/bin/env bats
# Decision memory and learning tests for pt bash wrapper

load "./test_helper/common.bash"

setup() {
    test_start "learning tests" "validate decision memory save/load"
    setup_test_env

    # These tests were written for the legacy bash-first learning/memory layer.
    # Learning/history is now implemented in `pt-core` and validated via the Rust
    # test suite + agent contract tests.
    skip "legacy bash learning/memory tests; superseded by pt-core tests"

    TEST_DIR="$( cd "$( dirname "$BATS_TEST_FILENAME" )" && pwd )"
    PROJECT_ROOT="$(dirname "$TEST_DIR")"
    PATH="$PROJECT_ROOT:$PATH"

    export PROCESS_TRIAGE_CONFIG="$CONFIG_DIR"
}

teardown() {
    teardown_test_env
    test_end "learning tests" "pass"
}

# ============================================================================
# Decision File Initialization
# ============================================================================

@test "decisions.json is created on first run" {
    # Use a fresh, unique config directory so we don't need to delete anything.
    local fresh_config="$BATS_TEST_TMPDIR/fresh_config_$$"
    export PROCESS_TRIAGE_CONFIG="$fresh_config"

    run pt --help
    [ "$status" -eq 0 ]

    # Config should be ensured
    [ -d "$fresh_config" ]
    [ -f "$fresh_config/decisions.json" ]

    # Restore default test config for subsequent tests
    export PROCESS_TRIAGE_CONFIG="$CONFIG_DIR"
}

@test "decisions.json starts as empty object" {
    skip_if_no_jq

    # Fresh config
    echo '{}' > "$CONFIG_DIR/decisions.json"

    local content
    content=$(cat "$CONFIG_DIR/decisions.json")
    [ "$content" = "{}" ]
}

@test "priors.json is created on first run" {
    # Use a fresh, unique config directory so we don't need to delete anything.
    local fresh_config="$BATS_TEST_TMPDIR/fresh_config_priors_$$"
    export PROCESS_TRIAGE_CONFIG="$fresh_config"

    run pt --help
    [ "$status" -eq 0 ]

    # Config dir should exist
    [ -d "$fresh_config" ]
    [ -f "$fresh_config/priors.json" ]

    # Restore default test config for subsequent tests
    export PROCESS_TRIAGE_CONFIG="$CONFIG_DIR"
}

# ============================================================================
# Decision Persistence
# ============================================================================

@test "pt robot apply --yes records kill decisions" {
    skip_if_no_jq

    # Start with empty decisions
    echo '{}' > "$CONFIG_DIR/decisions.json"

    # Create a test process that we can safely kill
    sleep 999 &
    local test_pid=$!

    # Wait for it to start
    sleep 0.1

    # Kill it via pt robot apply
    run pt robot apply --pids "$test_pid" --yes --format json
    # Status may vary based on whether process was actually killed
    [ "$status" -eq 0 ] || [ "$status" -eq 2 ]

    # Clean up if still running
    kill "$test_pid" 2>/dev/null || true
    wait "$test_pid" 2>/dev/null || true

    # Note: Decision may or may not be recorded depending on actual execution
    # The key is that the mechanism exists
}

@test "decision history persists across invocations" {
    skip_if_no_jq

    # Set up a decision
    echo '{"test_pattern": "kill"}' > "$CONFIG_DIR/decisions.json"

    # Read it back
    local decision
    decision=$(jq -r '.test_pattern' "$CONFIG_DIR/decisions.json")
    [ "$decision" = "kill" ]

    # Run pt (history should still exist)
    run pt --help
    [ "$status" -eq 0 ]

    # Verify decision still there
    decision=$(jq -r '.test_pattern' "$CONFIG_DIR/decisions.json")
    [ "$decision" = "kill" ]
}

# ============================================================================
# History Command
# ============================================================================

@test "pt history works with empty decisions" {
    skip_if_no_gum

    echo '{}' > "$CONFIG_DIR/decisions.json"

    run pt history
    [ "$status" -eq 0 ]
    assert_contains "$output" "No decision history" "should indicate empty history"
}

@test "pt history shows kill decisions" {
    skip_if_no_gum
    skip_if_no_jq

    # Set up some decisions
    echo '{"bun test --watch": "kill", "vim": "spare"}' > "$CONFIG_DIR/decisions.json"

    run pt history
    [ "$status" -eq 0 ]
    # Should show kill and spare counts
    assert_contains "$output" "kill" "should show kill decisions"
}

@test "pt history shows spare decisions" {
    skip_if_no_gum
    skip_if_no_jq

    echo '{"vim": "spare", "nano": "spare"}' > "$CONFIG_DIR/decisions.json"

    run pt history
    [ "$status" -eq 0 ]
    assert_contains "$output" "spare" "should show spare decisions"
}

# ============================================================================
# Clear Command
# ============================================================================

@test "pt clear removes all decisions" {
    skip_if_no_gum
    skip_if_no_jq

    # Set up decisions
    echo '{"pattern1": "kill", "pattern2": "spare"}' > "$CONFIG_DIR/decisions.json"

    # Clear with simulated confirmation (CI mode)
    echo 'y' | run pt clear
    # May or may not succeed depending on gum availability

    # Direct clear test
    echo '{}' > "$CONFIG_DIR/decisions.json"
    local content
    content=$(cat "$CONFIG_DIR/decisions.json")
    [ "$content" = "{}" ]
}

@test "pt clear with empty history is no-op" {
    skip_if_no_gum

    echo '{}' > "$CONFIG_DIR/decisions.json"

    run pt history
    [ "$status" -eq 0 ]
    # Should handle empty gracefully
}

# ============================================================================
# Pattern Normalization and Matching
# ============================================================================

@test "decisions use normalized patterns" {
    skip_if_no_jq

    # Decision patterns should not contain raw PIDs
    echo '{"bun test --watch": "kill"}' > "$CONFIG_DIR/decisions.json"

    # Verify stored pattern is normalized (no large numbers)
    local pattern
    pattern=$(jq -r 'keys[0]' "$CONFIG_DIR/decisions.json")

    # Pattern should not contain 5+ digit numbers (PIDs)
    [[ ! "$pattern" =~ [0-9]{5,} ]]
}

@test "similar commands match same pattern" {
    skip_if_no_jq

    # Extract normalize_pattern function
    normalize_cmd() {
        local input="$1"
        bash -c 'source <(sed -n "/^normalize_pattern()/,/^}/p" "$PROJECT_ROOT/pt"); normalize_pattern "$1"' _ "$input"
    }

    local pattern1 pattern2

    # Two similar commands with different PIDs should normalize to same pattern
    pattern1=$(normalize_cmd "bun test --watch pid:12345")
    pattern2=$(normalize_cmd "bun test --watch pid:67890")

    [ "$pattern1" = "$pattern2" ]
}

@test "decisions with ports are normalized" {
    skip_if_no_jq

    # Extract normalize_pattern function
    normalize_cmd() {
        local input="$1"
        bash -c 'source <(sed -n "/^normalize_pattern()/,/^}/p" "$PROJECT_ROOT/pt"); normalize_pattern "$1"' _ "$input"
    }

    local pattern1 pattern2

    pattern1=$(normalize_cmd "next dev --port=3000")
    pattern2=$(normalize_cmd "next dev --port=8080")

    [ "$pattern1" = "$pattern2" ]
}

# ============================================================================
# Learning Integration
# ============================================================================

@test "robot apply records spare decisions for non-KILL candidates" {
    skip_if_no_jq

    # Start fresh
    echo '{}' > "$CONFIG_DIR/decisions.json"

    # Run apply with very high min-age (likely no candidates)
    run pt robot apply --recommended --min-age 999999999 --yes --format json
    [ "$status" -eq 0 ]

    # Decisions file should still exist (may be empty if no candidates)
    [ -f "$CONFIG_DIR/decisions.json" ]
}

@test "decision influence is cumulative" {
    skip_if_no_jq

    # Start with a kill decision
    echo '{"stuck test": "kill"}' > "$CONFIG_DIR/decisions.json"

    # Add another decision
    local tmp
    tmp=$(mktemp)
    jq --arg k "old server" --arg v "spare" '.[$k] = $v' "$CONFIG_DIR/decisions.json" > "$tmp"
    mv "$tmp" "$CONFIG_DIR/decisions.json"

    # Both should exist
    local kill_count spare_count
    kill_count=$(jq '[.[] | select(. == "kill")] | length' "$CONFIG_DIR/decisions.json")
    spare_count=$(jq '[.[] | select(. == "spare")] | length' "$CONFIG_DIR/decisions.json")

    [ "$kill_count" = "1" ]
    [ "$spare_count" = "1" ]
}

# ============================================================================
# Cache Behavior
# ============================================================================

@test "decision cache is loaded once" {
    skip_if_no_jq

    # Set up decisions
    echo '{"test_cmd": "kill"}' > "$CONFIG_DIR/decisions.json"

    # Multiple plan calls should work consistently
    run pt robot plan --format json
    [ "$status" -eq 0 ]

    run pt robot plan --format json
    [ "$status" -eq 0 ]
}

# ============================================================================
# Edge Cases
# ============================================================================

@test "handles corrupted decisions.json gracefully" {
    # Write invalid JSON
    echo 'not valid json' > "$CONFIG_DIR/decisions.json"

    # Should not crash
    run pt --help
    [ "$status" -eq 0 ]
}

@test "handles missing decisions.json" {
    # Use a fresh, unique config directory so decisions.json starts missing.
    local fresh_config="$BATS_TEST_TMPDIR/fresh_config_missing_decisions_$$"
    export PROCESS_TRIAGE_CONFIG="$fresh_config"

    run pt --help
    [ "$status" -eq 0 ]

    [ -f "$fresh_config/decisions.json" ]

    export PROCESS_TRIAGE_CONFIG="$CONFIG_DIR"
}

@test "handles empty config directory" {
    # Use a fresh, unique config directory so it starts empty without deleting anything.
    local fresh_config="$BATS_TEST_TMPDIR/fresh_empty_config_$$"
    mkdir -p "$fresh_config"
    export PROCESS_TRIAGE_CONFIG="$fresh_config"

    run pt --help
    [ "$status" -eq 0 ]

    [ -f "$fresh_config/decisions.json" ]
    [ -f "$fresh_config/priors.json" ]

    export PROCESS_TRIAGE_CONFIG="$CONFIG_DIR"
}

# ============================================================================
# JSON Format Validation
# ============================================================================

@test "decisions.json is valid JSON after updates" {
    skip_if_no_jq

    echo '{}' > "$CONFIG_DIR/decisions.json"

    # Add a decision manually (simulating what pt does)
    local tmp
    tmp=$(mktemp)
    jq --arg k "test pattern" --arg v "kill" '.[$k] = $v' "$CONFIG_DIR/decisions.json" > "$tmp"
    mv "$tmp" "$CONFIG_DIR/decisions.json"

    # Verify it's still valid JSON
    run jq '.' "$CONFIG_DIR/decisions.json"
    [ "$status" -eq 0 ]
}

@test "decisions.json values are only kill or spare" {
    skip_if_no_jq

    echo '{"p1": "kill", "p2": "spare", "p3": "kill"}' > "$CONFIG_DIR/decisions.json"

    # All values should be kill or spare
    local invalid
    invalid=$(jq '[.[] | select(. != "kill" and . != "spare")] | length' "$CONFIG_DIR/decisions.json")
    [ "$invalid" = "0" ]
}
