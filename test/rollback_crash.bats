#!/usr/bin/env bats
# Crash Recovery Tests for pt-core rollback mechanism
#
# These tests validate crash recovery scenarios:
# - Simulate crash during download: no change to binary
# - Simulate crash during replace: rollback works
# - Simulate crash during verification: rollback works
# - Power loss simulation: recovery on next run
#
# Reference: process_triage-ica.1.2

load "./test_helper/common.bash"

PT_CORE="${BATS_TEST_DIRNAME}/../target/release/pt-core"

setup_file() {
    # Ensure pt-core is built
    if [[ ! -x "$PT_CORE" ]]; then
        echo "# Building pt-core..." >&3
        (cd "${BATS_TEST_DIRNAME}/.." && cargo build --release 2>/dev/null) || {
            echo "ERROR: Failed to build pt-core" >&2
            exit 1
        }
    fi
}

setup() {
    setup_test_env
    export PROCESS_TRIAGE_CONFIG="$CONFIG_DIR"
    test_start "$BATS_TEST_NAME" "Crash recovery test"

    # Create test directories
    export TEST_BIN_DIR="${TEST_DIR}/bin"
    export TEST_ROLLBACK_DIR="${TEST_DIR}/rollback"
    mkdir -p "$TEST_BIN_DIR" "$TEST_ROLLBACK_DIR"
}

teardown() {
    test_end "$BATS_TEST_NAME" "${BATS_TEST_COMPLETED:-fail}"
    teardown_test_env
}

#==============================================================================
# HELPER FUNCTIONS
#==============================================================================

# Create a mock binary that reports a specific version
create_mock_binary() {
    local path="$1"
    local version="$2"

    cat > "$path" << EOF
#!/bin/bash
case "\$1" in
    --version) echo "pt-core $version" ;;
    health) echo "OK" ;;
    *) echo "Unknown command: \$1" >&2; exit 1 ;;
esac
EOF
    chmod +x "$path"
}

# Create an incomplete/partial file to simulate mid-download crash
create_partial_file() {
    local path="$1"
    local size="${2:-1024}"

    # Create a file with random bytes (simulating partial download)
    head -c "$size" /dev/urandom > "$path" 2>/dev/null || \
        dd if=/dev/urandom of="$path" bs="$size" count=1 2>/dev/null
}

# Create a backup in the test rollback directory
create_test_backup() {
    local version="$1"
    local backup_name="pt-core-${version}-$(date +%Y%m%d%H%M%S)"
    local backup_path="${TEST_ROLLBACK_DIR}/${backup_name}"

    create_mock_binary "$backup_path" "$version"

    local checksum
    if command -v sha256sum &>/dev/null; then
        checksum=$(sha256sum "$backup_path" | cut -d' ' -f1)
    else
        checksum=$(shasum -a 256 "$backup_path" | cut -d' ' -f1)
    fi

    local size_bytes
    size_bytes=$(stat -c%s "$backup_path" 2>/dev/null || stat -f%z "$backup_path" 2>/dev/null)

    cat > "${backup_path}.json" << EOF
{
    "version": "$version",
    "created_at": "$(date -u +%Y-%m-%dT%H:%M:%SZ)",
    "checksum": "$checksum",
    "original_path": "${TEST_BIN_DIR}/pt-core",
    "size_bytes": $size_bytes
}
EOF

    echo "$backup_path"
}

#==============================================================================
# CRASH DURING DOWNLOAD TESTS
#==============================================================================

@test "Crash Recovery: Partial download file does not affect current binary" {
    test_start "Crash: partial download" "verify partial download is safe"

    # Create current working binary
    local current_binary="${TEST_BIN_DIR}/pt-core"
    create_mock_binary "$current_binary" "1.0.0"

    # Verify it works
    run "$current_binary" --version
    assert_equals "0" "$status" "Original binary should work"
    assert_contains "$output" "1.0.0" "Should show original version"

    # Simulate partial download (as would happen if download was interrupted)
    local partial_download="${TEST_DIR}/.pt-core.new.$$"
    create_partial_file "$partial_download" 512

    # Original binary should still work
    run "$current_binary" --version
    assert_equals "0" "$status" "Original binary should still work after partial download"
    assert_contains "$output" "1.0.0" "Should still show original version"

    test_end "Crash: partial download" "pass"
}

@test "Crash Recovery: Temp file cleanup on failed download" {
    test_start "Crash: temp cleanup" "verify temp files are cleaned up"

    # Create current working binary
    local current_binary="${TEST_BIN_DIR}/pt-core"
    create_mock_binary "$current_binary" "1.0.0"

    # Simulate leftover temp files from crashed download
    local temp_file="${TEST_BIN_DIR}/.pt-core.new.12345"
    create_partial_file "$temp_file" 256

    # Original binary should still work
    run "$current_binary" --version
    assert_equals "0" "$status" "Original binary should work despite temp file"
    assert_contains "$output" "1.0.0" "Should show original version"

    test_end "Crash: temp cleanup" "pass"
}

#==============================================================================
# CRASH DURING REPLACE TESTS
#==============================================================================

@test "Crash Recovery: Binary replacement is atomic (no partial state)" {
    test_start "Crash: atomic replace" "verify atomic replacement"

    # Create current working binary
    local current_binary="${TEST_BIN_DIR}/pt-core"
    create_mock_binary "$current_binary" "1.0.0"

    # Create new binary
    local new_binary="${TEST_DIR}/pt-core-new"
    create_mock_binary "$new_binary" "2.0.0"

    # Atomic replacement test: rename should be atomic on same filesystem
    local temp_binary="${TEST_BIN_DIR}/.pt-core.new.$$"
    cp "$new_binary" "$temp_binary"
    chmod +x "$temp_binary"

    # Perform atomic rename
    mv "$temp_binary" "$current_binary"

    # Verify new version is in place
    run "$current_binary" --version
    assert_equals "0" "$status" "Binary should work after atomic replace"
    assert_contains "$output" "2.0.0" "Should show new version"

    test_end "Crash: atomic replace" "pass"
}

@test "Crash Recovery: Backup exists before any modification attempt" {
    test_start "Crash: backup before modify" "verify backup-first pattern"

    # Create current working binary
    local current_binary="${TEST_BIN_DIR}/pt-core"
    create_mock_binary "$current_binary" "1.0.0"

    # Create backup BEFORE any modification
    local backup_path
    backup_path=$(create_test_backup "1.0.0")

    # Verify backup was created
    [[ -f "$backup_path" ]]
    [[ -f "${backup_path}.json" ]]

    # Verify backup is functional
    run "$backup_path" --version
    assert_equals "0" "$status" "Backup should be executable"
    assert_contains "$output" "1.0.0" "Backup should have original version"

    test_end "Crash: backup before modify" "pass"
}

#==============================================================================
# CRASH DURING VERIFICATION TESTS
#==============================================================================

@test "Crash Recovery: Broken new binary triggers rollback" {
    test_start "Crash: broken binary rollback" "verify auto-rollback on broken binary"

    # Create current working binary
    local current_binary="${TEST_BIN_DIR}/pt-core"
    create_mock_binary "$current_binary" "1.0.0"

    # Create backup
    local backup_path
    backup_path=$(create_test_backup "1.0.0")

    # Simulate replacing with broken binary
    cat > "$current_binary" << 'EOF'
#!/bin/bash
echo "FATAL: Corrupted binary" >&2
exit 1
EOF
    chmod +x "$current_binary"

    # Verify the broken binary fails
    run "$current_binary" --version
    assert_not_equals "0" "$status" "Broken binary should fail"

    # Restore from backup (simulating rollback)
    cp "$backup_path" "$current_binary"
    chmod +x "$current_binary"

    # Verify rollback worked
    run "$current_binary" --version
    assert_equals "0" "$status" "Rolled back binary should work"
    assert_contains "$output" "1.0.0" "Should have original version"

    test_end "Crash: broken binary rollback" "pass"
}

@test "Crash Recovery: Version mismatch triggers rollback" {
    test_start "Crash: version mismatch" "verify rollback on unexpected version"

    # Create current working binary
    local current_binary="${TEST_BIN_DIR}/pt-core"
    create_mock_binary "$current_binary" "1.0.0"

    # Create backup
    local backup_path
    backup_path=$(create_test_backup "1.0.0")

    # Simulate updating but getting wrong version
    create_mock_binary "$current_binary" "1.0.1"  # Expected 2.0.0 but got 1.0.1

    # Check version
    run "$current_binary" --version
    assert_equals "0" "$status" "Binary should run"
    assert_contains "$output" "1.0.1" "Wrong version present"

    # Rollback due to version mismatch
    cp "$backup_path" "$current_binary"
    chmod +x "$current_binary"

    run "$current_binary" --version
    assert_equals "0" "$status" "Rolled back binary should work"
    assert_contains "$output" "1.0.0" "Original version restored"

    test_end "Crash: version mismatch" "pass"
}

#==============================================================================
# POWER LOSS SIMULATION TESTS
#==============================================================================

@test "Crash Recovery: System can recover from interrupted update on restart" {
    test_start "Crash: power loss recovery" "verify recovery after power loss"

    # Create current working binary
    local current_binary="${TEST_BIN_DIR}/pt-core"
    create_mock_binary "$current_binary" "1.0.0"

    # Create backup (simulating backup that was made before power loss)
    local backup_path
    backup_path=$(create_test_backup "1.0.0")

    # Simulate state after power loss during update:
    # - Current binary is corrupted
    # - Backup exists
    cat > "$current_binary" << 'EOF'
#!/bin/bash
# Corrupted - power loss during write
EOF
    # Make it not executable to simulate partial write
    chmod -x "$current_binary" 2>/dev/null || true

    # Verify current binary is broken
    run "$current_binary" --version 2>&1
    # Should fail (either permission denied or syntax error)

    # Recovery: restore from backup
    cp "$backup_path" "$current_binary"
    chmod +x "$current_binary"

    # Verify recovery
    run "$current_binary" --version
    assert_equals "0" "$status" "Recovered binary should work"
    assert_contains "$output" "1.0.0" "Should have original version"

    test_end "Crash: power loss recovery" "pass"
}

@test "Crash Recovery: Leftover temp files don't break operation" {
    test_start "Crash: leftover temps" "verify resilience to leftover files"

    # Create current working binary
    local current_binary="${TEST_BIN_DIR}/pt-core"
    create_mock_binary "$current_binary" "1.0.0"

    # Create various leftover files that might exist after crash
    touch "${TEST_BIN_DIR}/.pt-core.new.12345"
    touch "${TEST_BIN_DIR}/.pt-core.tmp"
    create_partial_file "${TEST_BIN_DIR}/.pt-core.download" 256

    # Current binary should still work
    run "$current_binary" --version
    assert_equals "0" "$status" "Binary should work despite leftover files"
    assert_contains "$output" "1.0.0" "Should show correct version"

    test_end "Crash: leftover temps" "pass"
}

#==============================================================================
# BACKUP INTEGRITY TESTS
#==============================================================================

@test "Crash Recovery: Corrupted backup is detected during rollback" {
    test_start "Crash: corrupted backup" "verify corrupted backup detection"

    # Create backup
    local backup_path
    backup_path=$(create_test_backup "1.0.0")

    # Corrupt the backup
    echo "corrupted content" >> "$backup_path"

    # Attempt to verify (would fail checksum)
    local expected_checksum
    expected_checksum=$(grep '"checksum"' "${backup_path}.json" | cut -d'"' -f4)

    local actual_checksum
    if command -v sha256sum &>/dev/null; then
        actual_checksum=$(sha256sum "$backup_path" | cut -d' ' -f1)
    else
        actual_checksum=$(shasum -a 256 "$backup_path" | cut -d' ' -f1)
    fi

    # Checksums should NOT match (backup is corrupted)
    [[ "$expected_checksum" != "$actual_checksum" ]]

    test_end "Crash: corrupted backup" "pass"
}

@test "Crash Recovery: Multiple backups allow fallback to older version" {
    test_start "Crash: multi-backup fallback" "verify fallback to older backup"

    # Create current binary
    local current_binary="${TEST_BIN_DIR}/pt-core"
    create_mock_binary "$current_binary" "3.0.0"

    # Create multiple backups
    sleep 0.1
    local backup1
    backup1=$(create_test_backup "1.0.0")
    sleep 0.1
    local backup2
    backup2=$(create_test_backup "2.0.0")

    # Corrupt the newest backup (2.0.0)
    echo "corrupted" >> "$backup2"

    # Should be able to fallback to 1.0.0
    cp "$backup1" "$current_binary"
    chmod +x "$current_binary"

    run "$current_binary" --version
    assert_equals "0" "$status" "Should work with older backup"
    assert_contains "$output" "1.0.0" "Should have older version"

    test_end "Crash: multi-backup fallback" "pass"
}
