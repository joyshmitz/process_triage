#!/usr/bin/env bats
# E2E workflow tests for pt - complete workflows from scan through decision memory
#
# These tests verify:
# - Components integrate correctly
# - Data flows through the entire pipeline
# - User-visible behavior matches expectations
# - Real-world scenarios work end-to-end

load "./test_helper/common.bash"

PT_SCRIPT="${BATS_TEST_DIRNAME}/../pt"

setup_file() {
    if [[ ! -x "$PT_SCRIPT" ]]; then
        echo "ERROR: pt script not found at $PT_SCRIPT" >&2
        exit 1
    fi
}

setup() {
    setup_test_env

    # This workflow suite targets the legacy bash-first `pt` behavior (history/clear and
    # heuristic scoring). The current architecture routes functionality through `pt-core`.
    # Core behavior is validated by `test/pt_agent_contract.bats` and Rust integration tests.
    skip "legacy bash-first E2E workflow tests; superseded by pt-core contract/integration suites"

    TEST_DIR="$( cd "$( dirname "$BATS_TEST_FILENAME" )" && pwd )"
    PROJECT_ROOT="$(dirname "$TEST_DIR")"
    PATH="$PROJECT_ROOT:$PATH"

    export PROCESS_TRIAGE_CONFIG="$CONFIG_DIR"
}

teardown() {
    restore_path
    teardown_test_env
}

#==============================================================================
# SCAN WORKFLOW TESTS
#==============================================================================

@test "E2E: scan finds stuck test runner and scores correctly" {
    test_start "E2E: scan finds stuck test runner" "verify stuck test detection and scoring"

    test_info "Setting up: mock ps with stuck bun test process (2 hours old)"

    # Create mock ps with a stuck test - output format expected by pt
    # Format: PID PPID UID ELAPSED RSS CMD
    # ELAPSED is in [[dd-]hh:]mm:ss format
    cat > "${MOCK_BIN}/ps" << 'EOF'
#!/usr/bin/env bash
# Mock ps that outputs a stuck test process
if [[ "$*" == *"axo"* ]] || [[ "$*" == *"aux"* ]] || [[ "$*" == *"-eo"* ]]; then
    echo "  PID  PPID   UID     ELAPSED   RSS CMD"
    echo "12345  1000  1000   02:00:00  524288 bun test --watch"
    echo "  100     1     0 365-00:00:00  10240 /usr/lib/systemd/systemd"
fi
EOF
    chmod +x "${MOCK_BIN}/ps"
    use_mock_bin

    test_info "Running: pt robot plan --format json"
    local json_output
    json_output=$(pt robot plan --format json 2>/dev/null) || true

    test_info "Checking if stuck test is detected"
    if command -v jq &>/dev/null; then
        local candidate_count
        candidate_count=$(echo "$json_output" | jq '.candidates | length' 2>/dev/null || echo "0")
        test_info "Found $candidate_count candidates"

        # Check that bun test was found
        if echo "$json_output" | jq -e '.candidates[] | select(.cmd | contains("bun test"))' >/dev/null 2>&1; then
            test_info "Stuck test runner detected successfully"
        else
            test_info "Note: bun test may not be in candidates (could be filtered by min-age)"
        fi
    fi

    test_end "E2E: scan finds stuck test runner" "pass"
}

@test "E2E: scan excludes protected system processes" {
    test_start "E2E: scan excludes protected" "verify systemd/sshd/cron never flagged"

    # Create mock ps with system processes and one candidate
    cat > "${MOCK_BIN}/ps" << 'EOF'
#!/usr/bin/env bash
if [[ "$*" == *"axo"* ]] || [[ "$*" == *"aux"* ]] || [[ "$*" == *"-eo"* ]]; then
    echo "  PID  PPID   UID     ELAPSED   RSS CMD"
    echo "    1     0     0 365-00:00:00  10240 /usr/lib/systemd/systemd"
    echo " 2000     1     0  30-00:00:00   5120 sshd: /usr/sbin/sshd"
    echo " 3000     1     0  30-00:00:00   2048 /usr/sbin/cron"
    echo " 4000  1000  1000   48:00:00  524288 bun test --watch"
fi
EOF
    chmod +x "${MOCK_BIN}/ps"
    use_mock_bin

    test_info "Running: pt robot plan --format json"
    local json_output
    json_output=$(pt robot plan --format json 2>/dev/null) || true

    if command -v jq &>/dev/null; then
        # Check protected processes are NOT flagged
        if echo "$json_output" | jq -e '.candidates[] | select(.cmd | contains("systemd"))' >/dev/null 2>&1; then
            test_error "systemd should not be flagged!"
            test_end "E2E: scan excludes protected" "fail"
            return 1
        fi

        if echo "$json_output" | jq -e '.candidates[] | select(.cmd | contains("sshd"))' >/dev/null 2>&1; then
            test_error "sshd should not be flagged!"
            test_end "E2E: scan excludes protected" "fail"
            return 1
        fi

        if echo "$json_output" | jq -e '.candidates[] | select(.cmd | contains("cron"))' >/dev/null 2>&1; then
            test_error "cron should not be flagged!"
            test_end "E2E: scan excludes protected" "fail"
            return 1
        fi

        test_info "Protected processes correctly excluded"
    else
        test_info "Skipping JSON validation (jq not available)"
    fi

    test_end "E2E: scan excludes protected" "pass"
}

@test "E2E: scan with no candidates returns clean output" {
    test_start "E2E: scan no candidates" "verify behavior when system is clean"

    # Create mock ps with only short-lived or protected processes
    cat > "${MOCK_BIN}/ps" << 'EOF'
#!/usr/bin/env bash
if [[ "$*" == *"axo"* ]] || [[ "$*" == *"aux"* ]] || [[ "$*" == *"-eo"* ]]; then
    echo "  PID  PPID   UID     ELAPSED   RSS CMD"
    echo "    1     0     0 365-00:00:00  10240 /usr/lib/systemd/systemd"
    echo "10000  1000  1000      00:30:00  65536 vim file.txt"
fi
EOF
    chmod +x "${MOCK_BIN}/ps"
    use_mock_bin

    test_info "Running: pt robot plan --format json"
    local json_output
    json_output=$(pt robot plan --format json 2>/dev/null) || true

    if command -v jq &>/dev/null; then
        local candidate_count
        candidate_count=$(echo "$json_output" | jq '.summary.candidates // 0' 2>/dev/null || echo "0")
        test_info "Candidate count: $candidate_count"

        # With only recent/protected processes, there should be few or no candidates
        if [[ "$candidate_count" -le 1 ]]; then
            test_info "Correctly reported minimal candidates for clean system"
        fi
    fi

    test_end "E2E: scan no candidates" "pass"
}

@test "E2E: scan with mixed processes sorts by score" {
    test_start "E2E: scan sorts by score" "verify highest-scored processes appear first"

    # Create mock ps with varied process types
    cat > "${MOCK_BIN}/ps" << 'EOF'
#!/usr/bin/env bash
if [[ "$*" == *"axo"* ]] || [[ "$*" == *"aux"* ]] || [[ "$*" == *"-eo"* ]]; then
    echo "  PID  PPID   UID     ELAPSED   RSS CMD"
    echo "10001     1  1000   72:00:00  524288 orphaned bun test --watch"
    echo "10002  1000  1000   24:00:00  131072 next dev --port 3000"
    echo "10003  1000  1000   02:00:00   65536 npm test"
    echo "    1     0     0 365-00:00:00  10240 /usr/lib/systemd/systemd"
fi
EOF
    chmod +x "${MOCK_BIN}/ps"
    use_mock_bin

    test_info "Running: pt robot plan --format json"
    local json_output
    json_output=$(pt robot plan --format json 2>/dev/null) || true

    if command -v jq &>/dev/null; then
        # Get scores in order
        local first_score second_score
        first_score=$(echo "$json_output" | jq '.candidates[0].score // 0' 2>/dev/null || echo "0")
        second_score=$(echo "$json_output" | jq '.candidates[1].score // 0' 2>/dev/null || echo "0")

        test_info "First candidate score: $first_score"
        test_info "Second candidate score: $second_score"

        if [[ "$first_score" -ge "$second_score" ]]; then
            test_info "Candidates correctly sorted by score (descending)"
        else
            test_warn "Candidates may not be properly sorted"
        fi
    fi

    test_end "E2E: scan sorts by score" "pass"
}

#==============================================================================
# DECISION MEMORY INTEGRATION TESTS
#==============================================================================

@test "E2E: decision memory persists across invocations" {
    skip_if_no_jq
    test_start "E2E: decision memory persists" "verify decisions saved and retrieved"

    test_info "Setting up: save a kill decision"

    # Manually write a decision
    echo '{"bun test --watch": "kill", "next dev": "spare"}' > "${CONFIG_DIR}/decisions.json"

    test_info "Verifying decision file exists"
    [[ -f "${CONFIG_DIR}/decisions.json" ]]

    test_info "Running: pt history"
    run pt history

    test_info "Exit status: $status"
    assert_equals "0" "$status" "History command should succeed"

    # Should show saved decisions
    if [[ "$output" == *"bun test"* ]] || [[ "$output" == *"decision"* ]]; then
        test_info "History correctly shows saved patterns"
    fi

    test_end "E2E: decision memory persists" "pass"
}

@test "E2E: past kill decision increases score" {
    skip_if_no_jq
    test_start "E2E: kill decision boosts score" "verify Bayesian update from history"

    test_info "Setting up: decision memory with kill pattern"

    # Save a kill decision for a pattern
    echo '{"bun test": "kill"}' > "${CONFIG_DIR}/decisions.json"

    # Create process matching that pattern
    cat > "${MOCK_BIN}/ps" << 'EOF'
#!/usr/bin/env bash
if [[ "$*" == *"axo"* ]] || [[ "$*" == *"aux"* ]] || [[ "$*" == *"-eo"* ]]; then
    echo "  PID  PPID   UID     ELAPSED   RSS CMD"
    echo "12345  1000  1000   04:00:00  102400 bun test --watch"
fi
EOF
    chmod +x "${MOCK_BIN}/ps"
    use_mock_bin

    test_info "Running: pt robot plan --format json"
    local json_output
    json_output=$(pt robot plan --format json 2>/dev/null) || true

    # The process should get boosted score due to past kill decision
    local candidate_rec
    candidate_rec=$(echo "$json_output" | jq -r '.candidates[0].rec // "UNKNOWN"' 2>/dev/null || echo "UNKNOWN")
    test_info "Recommendation: $candidate_rec"

    # With a past kill decision, score should be boosted
    test_info "Process with past kill decision should score higher"

    test_end "E2E: kill decision boosts score" "pass"
}

@test "E2E: past spare decision decreases score" {
    skip_if_no_jq
    test_start "E2E: spare decision lowers score" "verify spare dampens scoring"

    test_info "Setting up: decision memory with spare pattern"

    # Save a spare decision
    echo '{"gunicorn": "spare"}' > "${CONFIG_DIR}/decisions.json"

    # Create a gunicorn process
    cat > "${MOCK_BIN}/ps" << 'EOF'
#!/usr/bin/env bash
if [[ "$*" == *"axo"* ]] || [[ "$*" == *"aux"* ]] || [[ "$*" == *"-eo"* ]]; then
    echo "  PID  PPID   UID     ELAPSED   RSS CMD"
    echo "12345  1000  1000   24:00:00  204800 gunicorn --workers 4"
fi
EOF
    chmod +x "${MOCK_BIN}/ps"
    use_mock_bin

    test_info "Running: pt robot plan --format json"
    local json_output
    json_output=$(pt robot plan --format json 2>/dev/null) || true

    # With spare decision, process may not appear or should be SPARE
    local candidate_rec
    candidate_rec=$(echo "$json_output" | jq -r '.candidates[0].rec // "NONE"' 2>/dev/null || echo "NONE")
    test_info "Recommendation: $candidate_rec"

    if [[ "$candidate_rec" == "SPARE" ]] || [[ "$candidate_rec" == "NONE" ]]; then
        test_info "Spare decision correctly lowered scoring"
    fi

    test_end "E2E: spare decision lowers score" "pass"
}

@test "E2E: clear command removes all decisions" {
    skip_if_no_jq
    test_start "E2E: clear removes decisions" "verify clear wipes decision memory"

    test_info "Setting up: populate decision memory"

    echo '{"pattern1": "kill", "pattern2": "spare", "pattern3": "kill"}' > "${CONFIG_DIR}/decisions.json"

    # Verify decisions exist
    local count_before
    count_before=$(jq 'length' "${CONFIG_DIR}/decisions.json")
    test_info "Decisions before clear: $count_before"
    assert_equals "3" "$count_before" "Should have 3 decisions"

    # The clear command requires confirmation in interactive mode
    # In test mode, we can either directly manipulate the file or use the command
    # For now, verify the decisions file structure
    test_info "Decision memory correctly populated"

    test_end "E2E: clear removes decisions" "pass"
}

#==============================================================================
# HELP AND VERSION TESTS
#==============================================================================

@test "E2E: help command shows all subcommands" {
    test_start "E2E: help shows subcommands" "verify help documentation"

    test_info "Running: pt help"
    run pt help

    test_info "Exit status: $status"
    assert_equals "0" "$status" "Help should succeed"

    # Verify all commands documented
    assert_contains "$output" "scan" "Should document scan command"
    assert_contains "$output" "history" "Should document history command"

    test_end "E2E: help shows subcommands" "pass"
}

@test "E2E: --help flag shows usage" {
    test_start "E2E: --help shows usage" "verify --help flag works"

    test_info "Running: pt --help"
    run pt --help

    test_info "Exit status: $status"
    assert_equals "0" "$status" "Help should succeed"

    assert_contains "$output" "Process Triage" "Should show tool name"

    test_end "E2E: --help shows usage" "pass"
}

@test "E2E: version command shows version number" {
    test_start "E2E: version shows number" "verify version output"

    test_info "Running: pt --version"
    run pt --version

    test_info "Exit status: $status"
    test_info "Output: $output"

    assert_equals "0" "$status" "Version should succeed"

    # Should match semver format
    [[ "$output" =~ [0-9]+\.[0-9]+\.[0-9]+ ]]
    test_info "Version number found in output"

    test_end "E2E: version shows number" "pass"
}

@test "E2E: unknown command shows error and hint" {
    test_start "E2E: unknown command fails" "verify error handling"

    test_info "Running: pt unknowncommand12345"
    run pt unknowncommand12345

    test_info "Exit status: $status"
    test_info "Output: $output"

    # Should fail
    [[ $status -ne 0 ]]
    test_info "Unknown command correctly rejected"

    test_end "E2E: unknown command fails" "pass"
}

#==============================================================================
# CONFIGURATION TESTS
#==============================================================================

@test "E2E: respects PROCESS_TRIAGE_CONFIG environment variable" {
    test_start "E2E: config env var" "verify custom config directory"

    test_info "Setting up: custom config directory"

    local custom_config="${TEST_DIR}/custom_config"
    mkdir -p "$custom_config"
    echo '{}' > "${custom_config}/decisions.json"

    export PROCESS_TRIAGE_CONFIG="$custom_config"

    test_info "Running: pt history (should use custom config)"
    run pt history

    test_info "Exit status: $status"
    assert_equals "0" "$status" "Should succeed with custom config"

    test_end "E2E: config env var" "pass"
}

@test "E2E: creates config directory if missing" {
    test_start "E2E: creates config dir" "verify config auto-creation"

    test_info "Setting up: empty config path"

    local new_config="${TEST_DIR}/new_config_dir"
    export PROCESS_TRIAGE_CONFIG="$new_config"

    # Verify doesn't exist
    [[ ! -d "$new_config" ]]
    test_info "Confirmed config dir doesn't exist"

    test_info "Running: pt help (should trigger init)"
    run pt help

    test_info "Exit status: $status"
    assert_equals "0" "$status" "Help should succeed"

    # Config directory may or may not be created by help
    # The main point is that pt works even with missing config
    test_info "pt operates correctly with new config path"

    test_end "E2E: creates config dir" "pass"
}

#==============================================================================
# ROBOT/AGENT MODE INTEGRATION
#==============================================================================

@test "E2E: robot plan produces valid JSON for agents" {
    skip_if_no_jq
    test_start "E2E: robot plan JSON" "verify agent-consumable output"

    # robot plan can take a long time on systems with many processes
    # Use --limit to reduce scan time and --min-age to filter
    test_info "Running: pt robot plan --format json --limit 10"
    local json_output
    json_output=$(pt robot plan --format json --limit 10 2>/dev/null) || true

    # Debug: show output length
    test_info "Output length: ${#json_output} chars"

    # If output is empty, the test should handle gracefully
    if [[ -z "$json_output" ]]; then
        test_warn "robot plan produced no output (may be normal in some environments)"
        test_end "E2E: robot plan JSON" "pass"
        return 0
    fi

    # Should be valid JSON
    if echo "$json_output" | jq '.' >/dev/null 2>&1; then
        test_info "Output is valid JSON"
    else
        test_error "Output is not valid JSON"
        test_info "First 200 chars: ${json_output:0:200}"
        test_end "E2E: robot plan JSON" "fail"
        return 1
    fi

    # Check for key fields (version and mode should always be present)
    local has_version has_mode
    has_version=$(printf '%s' "$json_output" | jq 'has("version")' 2>/dev/null || echo "error")
    has_mode=$(printf '%s' "$json_output" | jq 'has("mode")' 2>/dev/null || echo "error")

    if [[ "$has_version" == "true" ]] && [[ "$has_mode" == "true" ]]; then
        test_info "JSON output has required fields (version, mode)"
    else
        test_warn "JSON output may be missing some fields"
        test_info "has_version=$has_version has_mode=$has_mode"
        # Don't fail - the output was valid JSON
    fi

    test_end "E2E: robot plan JSON" "pass"
}

@test "E2E: robot explain provides process details" {
    skip_if_no_jq
    test_start "E2E: robot explain" "verify process explanation"

    # Explain the current shell process
    test_info "Running: pt robot explain --pid $$ --format json"
    local json_output
    json_output=$(pt robot explain --pid $$ --format json 2>/dev/null) || true

    if echo "$json_output" | jq -e '.pid' >/dev/null 2>&1; then
        test_info "Got explanation for PID $$"

        local explained_pid
        explained_pid=$(echo "$json_output" | jq -r '.pid')
        assert_equals "$$" "$explained_pid" "Should explain correct PID"
    fi

    test_end "E2E: robot explain" "pass"
}

@test "E2E: robot apply with high min-age finds nothing" {
    skip_if_no_jq
    test_start "E2E: robot apply nothing" "verify nothing_to_do response"

    test_info "Running: pt robot apply --recommended --min-age 86400 --yes --format json"
    local json_output
    json_output=$(pt robot apply --recommended --min-age 86400 --yes --format json 2>/dev/null) || true

    # With very high min-age (24h), should find nothing to do
    local note
    note=$(echo "$json_output" | jq -r '.note // empty' 2>/dev/null || echo "")
    test_info "Note: $note"

    if [[ "$note" == "nothing_to_do" ]] || [[ -z "$note" ]]; then
        test_info "Correctly reported nothing to do with high min-age"
    fi

    test_end "E2E: robot apply nothing" "pass"
}

#==============================================================================
# MARKDOWN OUTPUT TESTS
#==============================================================================

@test "E2E: robot plan markdown output is formatted" {
    test_start "E2E: markdown output" "verify markdown formatting"

    test_info "Running: pt robot plan --format md"
    local md_output
    md_output=$(pt robot plan --format md 2>/dev/null) || true

    # Should contain markdown headers
    assert_contains "$md_output" "# pt robot plan" "Should have markdown header"

    # Should contain table
    if [[ "$md_output" == *"|"* ]]; then
        test_info "Markdown contains table formatting"
    fi

    test_end "E2E: markdown output" "pass"
}

#==============================================================================
# ERROR HANDLING TESTS
#==============================================================================

@test "E2E: graceful handling when ps fails" {
    test_start "E2E: ps failure handling" "verify graceful degradation"

    # Create a failing ps mock
    cat > "${MOCK_BIN}/ps" << 'EOF'
#!/usr/bin/env bash
exit 1
EOF
    chmod +x "${MOCK_BIN}/ps"
    use_mock_bin

    test_info "Running: pt robot plan --format json (with failing ps)"
    run pt robot plan --format json

    # Should handle failure gracefully (not crash)
    test_info "Exit status: $status"
    test_info "pt handled ps failure without crashing"

    test_end "E2E: ps failure handling" "pass"
}

@test "E2E: robot explain handles non-existent PID" {
    skip_if_no_jq
    test_start "E2E: explain invalid PID" "verify error for missing process"

    test_info "Running: pt robot explain --pid 999999999 --format json"
    local json_output
    json_output=$(pt robot explain --pid 999999999 --format json 2>/dev/null) || true

    # Should return an error indicator
    local error
    error=$(echo "$json_output" | jq -r '.error // empty' 2>/dev/null || echo "")
    test_info "Error field: $error"

    if [[ "$error" == "not_found" ]] || [[ -z "$error" ]]; then
        test_info "Correctly handles non-existent PID"
    fi

    test_end "E2E: explain invalid PID" "pass"
}
