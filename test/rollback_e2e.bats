#!/usr/bin/env bats
# E2E rollback and self-update tests for pt
# Tests backup creation, rollback, and recovery functionality.

load "./test_helper/common.bash"

PT_CORE="${BATS_TEST_DIRNAME}/../target/debug/pt-core"

setup_file() {
    # Build pt-core if needed
    if [[ ! -x "$PT_CORE" ]]; then
        pushd "${BATS_TEST_DIRNAME}/.." > /dev/null
        cargo build -p pt-core 2>/dev/null || skip "pt-core build failed"
        popd > /dev/null
    fi
}

setup() {
    setup_test_env

    export PROCESS_TRIAGE_CONFIG="$CONFIG_DIR"

    # Create a custom cache directory for testing
    export TEST_CACHE_DIR="${TEST_DIR}/cache"
    mkdir -p "$TEST_CACHE_DIR/process_triage/rollback"
}

teardown() {
    restore_path
    teardown_test_env
}

#==============================================================================
# BACKUP LIST TESTS
#==============================================================================

@test "E2E Rollback: list-backups with no backups shows empty" {
    test_start "E2E Rollback: list-backups empty" "verify empty backup list handling"

    run "$PT_CORE" update list-backups --format summary 2>&1

    assert_equals "0" "$status" "list-backups should succeed with no backups"
    assert_contains "$output" "No backup" "Should indicate no backups found"

    test_end "E2E Rollback: list-backups empty" "pass"
}

@test "E2E Rollback: list-backups JSON format" {
    test_start "E2E Rollback: list-backups json" "verify JSON output format"

    run "$PT_CORE" update list-backups --format json 2>&1

    assert_equals "0" "$status" "list-backups should succeed"

    # Verify JSON structure
    if ! echo "$output" | jq . > /dev/null 2>&1; then
        test_error "Output is not valid JSON"
        test_end "E2E Rollback: list-backups json" "fail"
        return 1
    fi

    # Verify required fields
    assert_contains "$output" '"backups"' "Should have backups field"
    assert_contains "$output" '"count"' "Should have count field"

    test_end "E2E Rollback: list-backups json" "pass"
}

#==============================================================================
# ROLLBACK COMMAND TESTS
#==============================================================================

@test "E2E Rollback: rollback with no backup shows error" {
    test_start "E2E Rollback: no backup available" "verify rollback fails without backup"

    run "$PT_CORE" update rollback --format summary 2>&1

    # Should fail or report no backup
    assert_contains "$output" "No backup" "Should report no backup available"

    test_end "E2E Rollback: no backup available" "pass"
}

@test "E2E Rollback: rollback to nonexistent version" {
    test_start "E2E Rollback: nonexistent version" "verify rollback to bad version fails"

    run "$PT_CORE" update rollback "99.99.99" 2>&1

    # Should report version not found
    assert_contains "$output" "No backup" "Should report version not found"

    test_end "E2E Rollback: nonexistent version" "pass"
}

#==============================================================================
# VERIFY-BACKUP TESTS
#==============================================================================

@test "E2E Rollback: verify-backup with no backup" {
    test_start "E2E Rollback: verify no backup" "verify backup verification with no backup"

    run "$PT_CORE" update verify-backup 2>&1

    # Should report no backup to verify
    assert_contains "$output" "No backup" "Should report no backup available"

    test_end "E2E Rollback: verify no backup" "pass"
}

@test "E2E Rollback: verify-backup JSON format" {
    test_start "E2E Rollback: verify-backup json" "verify JSON output format"

    run "$PT_CORE" update verify-backup --format json 2>&1

    # Verify JSON structure
    if ! echo "$output" | jq . > /dev/null 2>&1; then
        test_error "Output is not valid JSON"
        test_end "E2E Rollback: verify-backup json" "fail"
        return 1
    fi

    test_end "E2E Rollback: verify-backup json" "pass"
}

#==============================================================================
# PRUNE-BACKUPS TESTS
#==============================================================================

@test "E2E Rollback: prune-backups default keep" {
    test_start "E2E Rollback: prune default" "verify prune with default retention"

    run "$PT_CORE" update prune-backups --format json 2>&1

    assert_equals "0" "$status" "prune-backups should succeed"
    assert_contains "$output" '"kept": 3' "Should indicate default keep count"

    test_end "E2E Rollback: prune default" "pass"
}

@test "E2E Rollback: prune-backups custom keep" {
    test_start "E2E Rollback: prune custom" "verify prune with custom retention"

    run "$PT_CORE" update prune-backups --keep 5 --format json 2>&1

    assert_equals "0" "$status" "prune-backups should succeed"
    assert_contains "$output" '"kept": 5' "Should indicate custom keep count"

    test_end "E2E Rollback: prune custom" "pass"
}

@test "E2E Rollback: prune-backups JSON format" {
    test_start "E2E Rollback: prune json" "verify prune JSON output"

    run "$PT_CORE" update prune-backups --format json 2>&1

    assert_equals "0" "$status" "prune-backups should succeed"

    # Verify JSON structure
    if ! echo "$output" | jq . > /dev/null 2>&1; then
        test_error "Output is not valid JSON"
        test_end "E2E Rollback: prune json" "fail"
        return 1
    fi

    assert_contains "$output" '"status"' "Should have status field"
    assert_contains "$output" '"kept"' "Should have kept field"

    test_end "E2E Rollback: prune json" "pass"
}

#==============================================================================
# SHOW-BACKUP TESTS
#==============================================================================

@test "E2E Rollback: show-backup nonexistent version" {
    test_start "E2E Rollback: show nonexistent" "verify show-backup with bad version"

    run "$PT_CORE" update show-backup "1.0.0" 2>&1

    # Should fail with error message
    assert_contains "$output" "No backup found" "Should report version not found"

    test_end "E2E Rollback: show nonexistent" "pass"
}

#==============================================================================
# HELP AND USAGE TESTS
#==============================================================================

@test "E2E Rollback: update help shows subcommands" {
    test_start "E2E Rollback: help" "verify update help shows all subcommands"

    run "$PT_CORE" update --help 2>&1

    assert_equals "0" "$status" "help should succeed"
    assert_contains "$output" "rollback" "Should show rollback subcommand"
    assert_contains "$output" "list-backups" "Should show list-backups subcommand"
    assert_contains "$output" "verify-backup" "Should show verify-backup subcommand"
    assert_contains "$output" "prune-backups" "Should show prune-backups subcommand"
    assert_contains "$output" "show-backup" "Should show show-backup subcommand"

    test_end "E2E Rollback: help" "pass"
}

@test "E2E Rollback: rollback help shows options" {
    test_start "E2E Rollback: rollback help" "verify rollback subcommand help"

    run "$PT_CORE" update rollback --help 2>&1

    assert_equals "0" "$status" "help should succeed"
    assert_contains "$output" "TARGET" "Should show target version argument"
    assert_contains "$output" "--force" "Should show force flag"

    test_end "E2E Rollback: rollback help" "pass"
}
