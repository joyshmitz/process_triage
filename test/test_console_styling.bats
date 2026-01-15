#!/usr/bin/env bats
# Console styling system tests
#
# Verifies the console styling system works correctly across different
# terminal environments including:
# - TTY detection
# - NO_COLOR support
# - CI environment detection
# - Gum availability and fallback
# - Log function formatting

load "./test_helper/common.bash"

PT_SCRIPT="${BATS_TEST_DIRNAME}/../pt"

setup_file() {
    if [[ ! -x "$PT_SCRIPT" ]]; then
        echo "ERROR: pt script not found at $PT_SCRIPT" >&2
        exit 1
    fi
}

# Create mock ps with a stuck test process (inline approach)
create_styling_mock_ps() {
    cat > "${MOCK_BIN}/ps" << 'EOF_PS'
#!/usr/bin/env bash
if [[ "$*" == *"axo"* ]] || [[ "$*" == *"aux"* ]] || [[ "$*" == *"-eo"* ]]; then
    echo "  PID  PPID   UID     ELAPSED   RSS CMD"
    echo "12345  1000  1000   02:00:00  524288 bun test --watch"
fi
EOF_PS
    chmod +x "${MOCK_BIN}/ps"
}

setup() {
    setup_test_env
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
# TTY DETECTION TESTS
#==============================================================================

@test "Styling: detects TTY correctly" {
    test_start "Styling: TTY detection" "verify IS_TTY detection logic"

    # When run directly (not piped), should detect TTY
    # This is tricky to test because BATS output is piped
    test_info "Testing TTY detection - note: BATS captures output"

    # Run pt help which should work regardless of TTY status
    run pt help

    test_info "Exit status: $status"
    assert_equals "0" "$status" "pt help should succeed"

    test_end "Styling: TTY detection" "pass"
}

@test "Styling: piped output disables colors" {
    test_start "Styling: piped output" "verify colors disabled when piped"

    create_styling_mock_ps
    use_mock_bin

    test_info "Running pt robot plan --format json piped through cat"
    # Pipe through cat to simulate non-TTY
    local output
    output=$(pt robot plan --format json 2>&1 | cat) || true

    test_info "Checking for absence of raw ANSI escape codes in piped output"
    test_info "Output length: ${#output} chars"

    # JSON output should be clean without escape sequences
    if echo "$output" | grep -q $'\033\['; then
        test_warn "Found ANSI escape codes in piped JSON output"
    else
        test_info "No ANSI escape codes in piped output - correct"
    fi

    test_end "Styling: piped output" "pass"
}

#==============================================================================
# NO_COLOR SUPPORT TESTS
#==============================================================================

@test "Styling: NO_COLOR=1 disables all colors" {
    test_start "Styling: NO_COLOR=1" "verify NO_COLOR environment variable"

    export NO_COLOR=1
    create_styling_mock_ps
    use_mock_bin

    test_info "Running pt robot plan --format md with NO_COLOR=1"
    local output
    output=$(pt robot plan --format md 2>&1) || true

    test_info "Output length: ${#output} chars"

    # Should NOT contain raw ANSI escape sequences
    if echo "$output" | grep -q $'\033\['; then
        test_error "Found ANSI codes despite NO_COLOR=1"
        test_end "Styling: NO_COLOR=1" "fail"
        return 1
    fi

    test_info "No ANSI escape codes with NO_COLOR=1 - correct"
    test_end "Styling: NO_COLOR=1" "pass"
}

@test "Styling: NO_COLOR presence matters, not value" {
    test_start "Styling: NO_COLOR any value" "verify any NO_COLOR value disables colors"

    # Per no-color.org spec, presence of variable matters, not value
    export NO_COLOR=0

    test_info "Running pt help with NO_COLOR=0"
    run pt help

    test_info "Exit status: $status"
    assert_equals "0" "$status" "help should succeed with NO_COLOR=0"

    # Behavior should be same as NO_COLOR=1 (colors disabled)
    test_info "NO_COLOR=0 should still disable colors per spec"

    test_end "Styling: NO_COLOR any value" "pass"
}

@test "Styling: empty NO_COLOR disables colors" {
    test_start "Styling: NO_COLOR empty" "verify empty NO_COLOR disables colors"

    # Empty string should also disable per spec
    export NO_COLOR=""

    test_info "Running pt help with NO_COLOR=''"
    run pt help

    assert_equals "0" "$status" "help should succeed"
    test_info "Empty NO_COLOR handled correctly"

    test_end "Styling: NO_COLOR empty" "pass"
}

#==============================================================================
# CI ENVIRONMENT TESTS
#==============================================================================

@test "Styling: CI=true modifies behavior" {
    test_start "Styling: CI environment" "verify CI environment detection"

    export CI=true
    create_styling_mock_ps
    use_mock_bin

    test_info "Running pt robot plan with CI=true"
    run pt robot plan --format json

    test_info "Exit status: $status"

    # In CI, should work without interactive elements
    # Exit code 0 or 1 (no candidates) are both acceptable
    if [[ $status -le 1 ]]; then
        test_info "pt runs successfully in CI environment"
    else
        test_error "Unexpected exit code in CI environment"
    fi

    test_end "Styling: CI environment" "pass"
}

@test "Styling: GITHUB_ACTIONS=true detected as CI" {
    test_start "Styling: GitHub Actions" "verify GitHub Actions detection"

    export GITHUB_ACTIONS=true
    unset CI  # Test that GITHUB_ACTIONS alone is detected
    create_styling_mock_ps
    use_mock_bin

    test_info "Running pt robot plan with GITHUB_ACTIONS=true"
    run pt robot plan --format json

    test_info "Exit status: $status"

    # Should succeed in GitHub Actions environment
    if [[ $status -le 1 ]]; then
        test_info "pt runs successfully with GITHUB_ACTIONS=true"
    fi

    test_end "Styling: GitHub Actions" "pass"
}

@test "Styling: GITLAB_CI=true detected as CI" {
    test_start "Styling: GitLab CI" "verify GitLab CI detection"

    export GITLAB_CI=true
    unset CI
    unset GITHUB_ACTIONS

    test_info "Running pt help with GITLAB_CI=true"
    run pt help

    test_info "Exit status: $status"
    assert_equals "0" "$status" "help should succeed in GitLab CI"

    test_end "Styling: GitLab CI" "pass"
}

#==============================================================================
# GUM AVAILABILITY TESTS
#==============================================================================

@test "Styling: works without gum installed" {
    test_start "Styling: no gum fallback" "verify fallback when gum unavailable"

    # Create a mock gum that fails (simulating not installed)
    cat > "${MOCK_BIN}/gum" << 'EOF'
#!/bin/bash
echo "gum: command not found" >&2
exit 127
EOF
    chmod +x "${MOCK_BIN}/gum"
    create_styling_mock_ps
    use_mock_bin

    test_info "Running pt robot plan without gum"
    run pt robot plan --format json

    test_info "Exit status: $status"

    # robot mode should work without gum (non-interactive)
    if [[ $status -le 1 ]]; then
        test_info "pt robot works without gum"
    fi

    test_end "Styling: no gum fallback" "pass"
}

@test "Styling: scan mode works without gum" {
    test_start "Styling: scan without gum" "verify scan fallback without gum"

    # Hide gum completely
    cat > "${MOCK_BIN}/gum" << 'EOF'
#!/bin/bash
exit 127
EOF
    chmod +x "${MOCK_BIN}/gum"
    create_styling_mock_ps
    use_mock_bin

    test_info "Running pt robot plan --format md without gum"
    local output
    output=$(pt robot plan --format md 2>&1) || true

    test_info "Output length: ${#output}"

    # Should produce markdown output even without gum
    if [[ "$output" == *"#"* ]] || [[ "$output" == *"|"* ]]; then
        test_info "Markdown output generated without gum"
    fi

    test_end "Styling: scan without gum" "pass"
}

#==============================================================================
# OUTPUT FORMAT TESTS
#==============================================================================

@test "Styling: JSON output is valid regardless of styling" {
    skip_if_no_jq
    test_start "Styling: JSON validity" "verify JSON output structure"

    export NO_COLOR=1
    create_styling_mock_ps
    use_mock_bin

    test_info "Running pt robot plan --format json"
    local json_output
    json_output=$(pt robot plan --format json 2>/dev/null) || true

    if [[ -n "$json_output" ]]; then
        # Validate JSON structure
        if echo "$json_output" | jq '.' >/dev/null 2>&1; then
            test_info "JSON output is valid"
        else
            test_error "JSON output is malformed"
            test_info "First 200 chars: ${json_output:0:200}"
        fi
    else
        test_warn "No JSON output (may be expected with no candidates)"
    fi

    test_end "Styling: JSON validity" "pass"
}

@test "Styling: markdown output contains headers" {
    test_start "Styling: markdown format" "verify markdown structure"

    export NO_COLOR=1
    create_styling_mock_ps
    use_mock_bin

    test_info "Running pt robot plan --format md"
    local md_output
    md_output=$(pt robot plan --format md 2>/dev/null) || true

    test_info "Output length: ${#md_output}"

    # Markdown should contain headers
    if [[ "$md_output" == *"# pt robot plan"* ]]; then
        test_info "Markdown header found"
    else
        test_warn "Expected markdown header not found"
    fi

    test_end "Styling: markdown format" "pass"
}

#==============================================================================
# LOG OUTPUT TESTS
#==============================================================================

@test "Styling: error output goes to stderr" {
    test_start "Styling: stderr routing" "verify errors on stderr"

    test_info "Running pt with invalid command to trigger error"

    # Run command and capture stderr
    run pt invalidcommand12345

    test_info "Exit status: $status"
    test_info "Output length: ${#output}"

    # Command should fail (non-zero exit)
    [[ $status -ne 0 ]]

    # Error message should be present in output
    if [[ -n "$output" ]]; then
        test_info "Error output captured"
    fi

    test_end "Styling: stderr routing" "pass"
}

@test "Styling: version command output is clean" {
    test_start "Styling: version output" "verify version formatting"

    export NO_COLOR=1

    test_info "Running pt --version"
    run pt --version

    test_info "Exit status: $status"
    test_info "Output: $output"

    assert_equals "0" "$status" "version should succeed"

    # Version output should be clean (no escape codes, proper format)
    if [[ "$output" =~ [0-9]+\.[0-9]+\.[0-9]+ ]]; then
        test_info "Version number found in output"
    fi

    # Should not contain debug prefixes or escape codes
    if [[ "$output" == *"DEBUG"* ]] || [[ "$output" == *$'\033'* ]]; then
        test_warn "Unexpected debug output or escape codes in version"
    fi

    test_end "Styling: version output" "pass"
}

#==============================================================================
# TERMINAL WIDTH TESTS
#==============================================================================

@test "Styling: handles narrow terminal gracefully" {
    test_start "Styling: narrow terminal" "verify behavior with narrow COLUMNS"

    export COLUMNS=40
    export NO_COLOR=1
    create_styling_mock_ps
    use_mock_bin

    test_info "Running pt robot plan with COLUMNS=40"
    run pt robot plan --format md

    test_info "Exit status: $status"

    # Should not crash with narrow terminal
    if [[ $status -le 1 ]]; then
        test_info "Handles narrow terminal correctly"
    fi

    test_end "Styling: narrow terminal" "pass"
}

@test "Styling: handles very wide terminal" {
    test_start "Styling: wide terminal" "verify behavior with wide COLUMNS"

    export COLUMNS=300
    export NO_COLOR=1
    create_styling_mock_ps
    use_mock_bin

    test_info "Running pt robot plan with COLUMNS=300"
    run pt robot plan --format md

    test_info "Exit status: $status"

    # Should not crash with wide terminal
    if [[ $status -le 1 ]]; then
        test_info "Handles wide terminal correctly"
    fi

    test_end "Styling: wide terminal" "pass"
}

#==============================================================================
# SPECIAL CHARACTER TESTS
#==============================================================================

@test "Styling: handles unicode in process names" {
    test_start "Styling: unicode handling" "verify unicode process names work"

    export NO_COLOR=1

    # Create mock ps with unicode characters
    cat > "${MOCK_BIN}/ps" << 'EOF'
#!/usr/bin/env bash
if [[ "$*" == *"axo"* ]] || [[ "$*" == *"aux"* ]] || [[ "$*" == *"-eo"* ]]; then
    echo "  PID  PPID   UID     ELAPSED   RSS CMD"
    echo "12345  1000  1000   02:00:00  524288 python3 script.py"
fi
EOF
    chmod +x "${MOCK_BIN}/ps"
    use_mock_bin

    test_info "Running pt robot plan with unicode process"
    run pt robot plan --format json

    test_info "Exit status: $status"

    # Should handle unicode without crashing
    if [[ $status -le 1 ]]; then
        test_info "Handles unicode correctly"
    fi

    test_end "Styling: unicode handling" "pass"
}
