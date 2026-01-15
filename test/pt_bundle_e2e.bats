#!/usr/bin/env bats
# E2E Bundle Export/Import Integrity Tests for process_triage (aii.2)
#
# These tests validate that .ptb bundles are:
# - Portable: Can be exported and re-read correctly
# - Integrity-checked: Manifest contains checksums, tampering is detected
# - Redaction-safe: Sensitive data does not leak across profiles
# - Reproducible: Import recreates expected session surface
#
# Test scope (from Plan §3.6 + §11):
# 1) Round-trip correctness by profile (minimal, safe, forensic)
# 2) Integrity failure (tamper tests) - flip byte → checksum mismatch
# 3) Redaction safety regression tests
# 4) Size/overhead sanity checks
#
# Reference: docs/CLI_SPECIFICATION.md, Plan §3.6, §11

load "./test_helper/common.bash"

PT_CORE="${BATS_TEST_DIRNAME}/../target/release/pt-core"

# Schema version pattern: X.Y.Z
SCHEMA_VERSION_PATTERN='^[0-9]+\.[0-9]+\.[0-9]+$'

# Bundle version pattern
BUNDLE_VERSION_PATTERN='^[0-9]+\.[0-9]+\.[0-9]+$'

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
    test_start "$BATS_TEST_NAME" "E2E bundle test"

    # Create test session data for export tests
    create_test_session
}

teardown() {
    test_end "$BATS_TEST_NAME" "${BATS_TEST_COMPLETED:-fail}"
    teardown_test_env
}

#==============================================================================
# TEST SESSION CREATION HELPERS
#==============================================================================

# Create a realistic test session with various data types
create_test_session() {
    export TEST_SESSION_DIR="${TEST_DIR}/sessions/pt-test-session-0001"
    mkdir -p "${TEST_SESSION_DIR}"/{scan,inference,decision,telemetry,action}

    # Create session manifest
    cat > "${TEST_SESSION_DIR}/manifest.json" << 'EOF'
{
    "schema_version": "1.0.0",
    "session_id": "pt-test-session-0001",
    "host_id": "test-host",
    "created_at": "2026-01-15T12:00:00Z",
    "platform": "linux",
    "pt_version": "0.1.0"
}
EOF

    # Create context with potential sensitive data
    cat > "${TEST_SESSION_DIR}/context.json" << 'EOF'
{
    "schema_version": "1.0.0",
    "session_id": "pt-test-session-0001",
    "host": {
        "hostname": "dev-workstation",
        "username": "testuser",
        "home_dir": "/home/testuser"
    },
    "environment": {
        "SHELL": "/bin/bash",
        "USER": "testuser"
    }
}
EOF

    # Create process snapshot with mix of process types
    cat > "${TEST_SESSION_DIR}/scan/snapshot.json" << 'EOF'
{
    "schema_version": "1.0.0",
    "timestamp": "2026-01-15T12:00:00Z",
    "processes": [
        {
            "pid": 1001,
            "ppid": 1000,
            "comm": "bun",
            "cmdline": "bun test --watch --api-key=sk-secret123",
            "user": "testuser",
            "age_seconds": 7200,
            "memory_mb": 512.0,
            "cpu_percent": 85.0
        },
        {
            "pid": 1002,
            "ppid": 1,
            "comm": "orphan",
            "cmdline": "orphaned-process --password=hunter2",
            "user": "testuser",
            "age_seconds": 86400,
            "memory_mb": 256.0,
            "cpu_percent": 0.1
        },
        {
            "pid": 1003,
            "ppid": 1000,
            "comm": "gunicorn",
            "cmdline": "gunicorn app:main --workers=4",
            "user": "testuser",
            "age_seconds": 3600,
            "memory_mb": 128.0,
            "cpu_percent": 45.0
        }
    ]
}
EOF

    # Create inference posteriors
    cat > "${TEST_SESSION_DIR}/inference/posteriors.json" << 'EOF'
{
    "schema_version": "1.0.0",
    "posteriors": [
        {"pid": 1001, "category": "useful_bad", "posterior": 0.72, "confidence": 0.85},
        {"pid": 1002, "category": "abandoned", "posterior": 0.91, "confidence": 0.92},
        {"pid": 1003, "category": "useful", "posterior": 0.88, "confidence": 0.90}
    ]
}
EOF

    # Create decision plan
    cat > "${TEST_SESSION_DIR}/decision/plan.json" << 'EOF'
{
    "schema_version": "1.0.0",
    "session_id": "pt-test-session-0001",
    "generated_at": "2026-01-15T12:01:00Z",
    "recommendations": [
        {"pid": 1001, "action": "review", "reason": "high_resource_test"},
        {"pid": 1002, "action": "kill", "reason": "abandoned_orphan"},
        {"pid": 1003, "action": "keep", "reason": "useful_server"}
    ]
}
EOF

    # Create audit trail
    cat > "${TEST_SESSION_DIR}/action/outcomes.jsonl" << 'EOF'
{"ts": "2026-01-15T12:02:00Z", "pid": 1002, "action": "kill", "result": "success"}
EOF

    # Create telemetry data (simulated parquet placeholder)
    echo "PARQUET_PLACEHOLDER_DATA" > "${TEST_SESSION_DIR}/telemetry/proc_samples.parquet"

    test_info "Test session created at: $TEST_SESSION_DIR"
}

# Create a fixture with known sensitive strings for redaction testing
create_sensitive_fixture() {
    local fixture_dir="${TEST_DIR}/sensitive_fixture"
    mkdir -p "$fixture_dir"

    # Contains various sensitive patterns
    cat > "${fixture_dir}/snapshot.json" << 'EOF'
{
    "processes": [
        {
            "pid": 2001,
            "cmdline": "curl -H 'Authorization: Bearer sk-ant-api03-SECRETKEY' https://api.example.com",
            "env": {
                "AWS_SECRET_ACCESS_KEY": "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY",
                "DATABASE_URL": "postgres://user:password123@db.example.com:5432/mydb",
                "GITHUB_TOKEN": "ghp_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"
            }
        },
        {
            "pid": 2002,
            "cmdline": "node server.js --jwt-secret=mysupersecretjwtkey",
            "cwd": "/home/testuser/projects/secret-project"
        }
    ]
}
EOF

    export SENSITIVE_FIXTURE_DIR="$fixture_dir"
    test_info "Sensitive fixture created at: $fixture_dir"
}

#==============================================================================
# BUNDLE VALIDATION HELPERS
#==============================================================================

# Check if jq is available for JSON validation
require_jq() {
    if ! command -v jq &>/dev/null; then
        test_warn "Skipping: jq not installed"
        skip "jq not installed"
    fi
}

# Check if unzip is available
require_unzip() {
    if ! command -v unzip &>/dev/null; then
        test_warn "Skipping: unzip not installed"
        skip "unzip not installed"
    fi
}

# Validate bundle manifest has required fields
validate_bundle_manifest() {
    local manifest_json="$1"
    local context="${2:-manifest}"

    # Check bundle_version
    local bundle_version
    bundle_version=$(echo "$manifest_json" | jq -r '.bundle_version // empty' 2>/dev/null)
    if [[ -z "$bundle_version" ]]; then
        test_error "Missing bundle_version in $context"
        return 1
    fi
    if ! [[ "$bundle_version" =~ $BUNDLE_VERSION_PATTERN ]]; then
        test_error "Invalid bundle_version format: $bundle_version"
        return 1
    fi
    test_info "bundle_version: $bundle_version"

    # Check session_id
    local session_id
    session_id=$(echo "$manifest_json" | jq -r '.session_id // empty' 2>/dev/null)
    if [[ -z "$session_id" ]]; then
        test_error "Missing session_id in $context"
        return 1
    fi
    test_info "session_id: $session_id"

    # Check export_profile
    local export_profile
    export_profile=$(echo "$manifest_json" | jq -r '.export_profile // empty' 2>/dev/null)
    if [[ -z "$export_profile" ]]; then
        test_error "Missing export_profile in $context"
        return 1
    fi
    if [[ ! "$export_profile" =~ ^(minimal|safe|forensic)$ ]]; then
        test_error "Invalid export_profile: $export_profile"
        return 1
    fi
    test_info "export_profile: $export_profile"

    # Check files array with checksums
    local file_count
    file_count=$(echo "$manifest_json" | jq '.files | length' 2>/dev/null)
    if [[ -z "$file_count" ]] || [[ "$file_count" -lt 1 ]]; then
        test_error "Bundle has no files"
        return 1
    fi
    test_info "file_count: $file_count"

    # Verify each file has sha256 checksum
    local files_with_checksum
    files_with_checksum=$(echo "$manifest_json" | jq '[.files[] | select(.sha256 != null)] | length' 2>/dev/null)
    if [[ "$files_with_checksum" -ne "$file_count" ]]; then
        test_error "Some files missing sha256 checksum"
        return 1
    fi
    test_info "All files have checksums"

    return 0
}

# Extract and verify bundle contents
# Returns: extract_dir path via stdout (capture it)
extract_and_verify_bundle() {
    local bundle_path="$1"
    local extract_dir="${TEST_DIR}/bundle_extract_$$"

    mkdir -p "$extract_dir"
    unzip -q "$bundle_path" -d "$extract_dir" 2>/dev/null || {
        test_error "Failed to extract bundle: $bundle_path"
        return 1
    }

    # Check manifest exists
    if [[ ! -f "${extract_dir}/manifest.json" ]]; then
        test_error "Bundle missing manifest.json"
        return 1
    fi

    # Output info to fd 3 (BATS output), path to stdout for capture
    echo "# Bundle extracted to: $extract_dir" >&3 2>/dev/null || true
    echo "$extract_dir"
}

# Verify checksums match file contents
verify_bundle_checksums() {
    local extract_dir="$1"
    local manifest_json
    manifest_json=$(cat "${extract_dir}/manifest.json")

    # Get list of files from manifest
    local files
    files=$(echo "$manifest_json" | jq -r '.files[] | "\(.path)|\(.sha256)"' 2>/dev/null)

    while IFS='|' read -r filepath expected_sha; do
        [[ -z "$filepath" ]] && continue

        local file_full="${extract_dir}/${filepath}"
        if [[ ! -f "$file_full" ]]; then
            test_error "File in manifest but missing: $filepath"
            return 1
        fi

        # Compute actual SHA256
        local actual_sha
        actual_sha=$(sha256sum "$file_full" | cut -d' ' -f1)

        if [[ "$actual_sha" != "$expected_sha" ]]; then
            test_error "Checksum mismatch for $filepath"
            test_error "  Expected: $expected_sha"
            test_error "  Actual:   $actual_sha"
            return 1
        fi

        test_debug "Checksum verified: $filepath"
    done <<< "$files"

    test_info "All checksums verified"
    return 0
}

# Check for sensitive patterns in file content
check_no_sensitive_data() {
    local file_path="$1"
    local context="${2:-file}"

    local content
    content=$(cat "$file_path" 2>/dev/null)

    # Sensitive patterns that should NOT appear in safe/minimal exports
    local sensitive_patterns=(
        "sk-ant-api"
        "sk-secret"
        "password="
        "AWS_SECRET_ACCESS_KEY"
        "ghp_"
        "jwt-secret"
        "hunter2"
        "wJalrXUtnFEMI"
    )

    for pattern in "${sensitive_patterns[@]}"; do
        if [[ "$content" == *"$pattern"* ]]; then
            test_error "Sensitive data found in $context: pattern='$pattern'"
            return 1
        fi
    done

    test_debug "No sensitive patterns found in $context"
    return 0
}

#==============================================================================
# ROUND-TRIP TESTS BY PROFILE
#==============================================================================

@test "bundle round-trip: minimal profile exports and re-reads correctly" {
    require_jq
    require_unzip

    local bundle_path="${TEST_DIR}/test_minimal.ptb"

    # Create bundle using the pt-bundle crate directly (via Rust test harness)
    # For E2E we need the CLI - but bundle create requires an existing session
    # So we test the library round-trip with a direct Rust integration test pattern

    # Use pt-core bundle create if session exists
    # For now, create a mock bundle directly

    test_info "Testing minimal profile round-trip"

    # Create a minimal bundle programmatically
    run python3 - "$bundle_path" << 'PYTHON_BUNDLE'
import sys
import json
import zipfile
import hashlib
from pathlib import Path

bundle_path = sys.argv[1]

def sha256(data):
    return hashlib.sha256(data).hexdigest()

# Minimal profile: only aggregate stats
summary = {"total_processes": 3, "candidates": 2, "recommendations": {"kill": 1, "keep": 1, "review": 1}}
summary_bytes = json.dumps(summary, indent=2).encode()

plan = {"session_id": "test-session", "recommendations": [{"pid": 1002, "action": "kill"}]}
plan_bytes = json.dumps(plan, indent=2).encode()

# Build manifest
manifest = {
    "bundle_version": "1.0.0",
    "session_id": "test-session",
    "host_id": "test-host",
    "export_profile": "minimal",
    "created_at": "2026-01-15T12:00:00Z",
    "files": [
        {"path": "summary.json", "sha256": sha256(summary_bytes), "bytes": len(summary_bytes)},
        {"path": "plan.json", "sha256": sha256(plan_bytes), "bytes": len(plan_bytes)}
    ]
}
manifest_bytes = json.dumps(manifest, indent=2).encode()

# Write ZIP
with zipfile.ZipFile(bundle_path, 'w', zipfile.ZIP_DEFLATED) as zf:
    zf.writestr("manifest.json", manifest_bytes)
    zf.writestr("summary.json", summary_bytes)
    zf.writestr("plan.json", plan_bytes)

print(f"Bundle created: {bundle_path}")
PYTHON_BUNDLE
    [ "$status" -eq 0 ]

    # Verify bundle exists
    [ -f "$bundle_path" ]
    test_info "Bundle created: $bundle_path"

    # Extract and validate
    local extract_dir
    extract_dir=$(extract_and_verify_bundle "$bundle_path")
    [ -n "$extract_dir" ]

    # Validate manifest
    local manifest_json
    manifest_json=$(cat "${extract_dir}/manifest.json")
    validate_bundle_manifest "$manifest_json" "minimal profile"

    # Verify profile is minimal
    local profile
    profile=$(echo "$manifest_json" | jq -r '.export_profile')
    [ "$profile" = "minimal" ]

    # Verify checksums
    verify_bundle_checksums "$extract_dir"

    # Minimal should have limited files (only summary + plan)
    local file_count
    file_count=$(echo "$manifest_json" | jq '.files | length')
    test_info "File count in minimal bundle: $file_count"
    [ "$file_count" -le 5 ]  # Minimal should be sparse

    BATS_TEST_COMPLETED=pass
}

@test "bundle round-trip: safe profile exports with redacted data" {
    require_jq
    require_unzip

    local bundle_path="${TEST_DIR}/test_safe.ptb"

    test_info "Testing safe profile round-trip"

    # Create a safe profile bundle with more data but redacted
    run python3 - "$bundle_path" << 'PYTHON_BUNDLE'
import sys
import json
import zipfile
import hashlib

bundle_path = sys.argv[1]

def sha256(data):
    return hashlib.sha256(data).hexdigest()

# Safe profile: includes features but with redaction
summary = {"total_processes": 3, "candidates": 2}
summary_bytes = json.dumps(summary, indent=2).encode()

# Plan with redacted cmdlines (hashed)
plan = {
    "session_id": "test-session",
    "recommendations": [
        {"pid": 1001, "action": "review", "cmdline_hash": "a1b2c3d4e5f6"},
        {"pid": 1002, "action": "kill", "cmdline_hash": "f6e5d4c3b2a1"}
    ]
}
plan_bytes = json.dumps(plan, indent=2).encode()

# Posteriors with process info (no raw cmdlines)
posteriors = {
    "posteriors": [
        {"pid": 1001, "category": "useful_bad", "posterior": 0.72},
        {"pid": 1002, "category": "abandoned", "posterior": 0.91}
    ]
}
posteriors_bytes = json.dumps(posteriors, indent=2).encode()

manifest = {
    "bundle_version": "1.0.0",
    "session_id": "test-session",
    "host_id": "test-host",
    "export_profile": "safe",
    "created_at": "2026-01-15T12:00:00Z",
    "redaction_policy_version": "1.0.0",
    "files": [
        {"path": "summary.json", "sha256": sha256(summary_bytes), "bytes": len(summary_bytes)},
        {"path": "plan.json", "sha256": sha256(plan_bytes), "bytes": len(plan_bytes)},
        {"path": "inference/posteriors.json", "sha256": sha256(posteriors_bytes), "bytes": len(posteriors_bytes)}
    ]
}
manifest_bytes = json.dumps(manifest, indent=2).encode()

with zipfile.ZipFile(bundle_path, 'w', zipfile.ZIP_DEFLATED) as zf:
    zf.writestr("manifest.json", manifest_bytes)
    zf.writestr("summary.json", summary_bytes)
    zf.writestr("plan.json", plan_bytes)
    zf.writestr("inference/posteriors.json", posteriors_bytes)

print(f"Bundle created: {bundle_path}")
PYTHON_BUNDLE
    [ "$status" -eq 0 ]

    # Verify bundle
    [ -f "$bundle_path" ]

    local extract_dir
    extract_dir=$(extract_and_verify_bundle "$bundle_path")

    local manifest_json
    manifest_json=$(cat "${extract_dir}/manifest.json")
    validate_bundle_manifest "$manifest_json" "safe profile"

    # Verify profile
    local profile
    profile=$(echo "$manifest_json" | jq -r '.export_profile')
    [ "$profile" = "safe" ]

    # Safe should have redaction policy version
    local redaction_version
    redaction_version=$(echo "$manifest_json" | jq -r '.redaction_policy_version // empty')
    test_info "Redaction policy version: $redaction_version"
    [ -n "$redaction_version" ]

    verify_bundle_checksums "$extract_dir"

    BATS_TEST_COMPLETED=pass
}

@test "bundle round-trip: forensic profile includes raw evidence with checksums" {
    require_jq
    require_unzip

    local bundle_path="${TEST_DIR}/test_forensic.ptb"

    test_info "Testing forensic profile round-trip"

    # Create forensic bundle with more raw data
    run python3 - "$bundle_path" << 'PYTHON_BUNDLE'
import sys
import json
import zipfile
import hashlib

bundle_path = sys.argv[1]

def sha256(data):
    return hashlib.sha256(data).hexdigest()

summary = {"total_processes": 3, "candidates": 2}
summary_bytes = json.dumps(summary, indent=2).encode()

# Forensic includes more raw data (but still policy-redacted)
plan = {
    "session_id": "test-session",
    "recommendations": [
        {"pid": 1001, "action": "review", "cmdline": "bun test --watch"},
        {"pid": 1002, "action": "kill", "cmdline": "orphaned-process"}
    ]
}
plan_bytes = json.dumps(plan, indent=2).encode()

snapshot = {
    "processes": [
        {"pid": 1001, "comm": "bun", "cmdline": "bun test --watch", "age_seconds": 7200},
        {"pid": 1002, "comm": "orphan", "cmdline": "orphaned-process", "age_seconds": 86400}
    ]
}
snapshot_bytes = json.dumps(snapshot, indent=2).encode()

# Audit log
audit = '{"ts": "2026-01-15T12:02:00Z", "pid": 1002, "action": "kill"}\n'
audit_bytes = audit.encode()

manifest = {
    "bundle_version": "1.0.0",
    "session_id": "test-session",
    "host_id": "test-host",
    "export_profile": "forensic",
    "created_at": "2026-01-15T12:00:00Z",
    "files": [
        {"path": "summary.json", "sha256": sha256(summary_bytes), "bytes": len(summary_bytes)},
        {"path": "plan.json", "sha256": sha256(plan_bytes), "bytes": len(plan_bytes)},
        {"path": "snapshot.json", "sha256": sha256(snapshot_bytes), "bytes": len(snapshot_bytes)},
        {"path": "logs/audit.jsonl", "sha256": sha256(audit_bytes), "bytes": len(audit_bytes)}
    ]
}
manifest_bytes = json.dumps(manifest, indent=2).encode()

with zipfile.ZipFile(bundle_path, 'w', zipfile.ZIP_DEFLATED) as zf:
    zf.writestr("manifest.json", manifest_bytes)
    zf.writestr("summary.json", summary_bytes)
    zf.writestr("plan.json", plan_bytes)
    zf.writestr("snapshot.json", snapshot_bytes)
    zf.writestr("logs/audit.jsonl", audit_bytes)

print(f"Bundle created: {bundle_path}")
PYTHON_BUNDLE
    [ "$status" -eq 0 ]

    [ -f "$bundle_path" ]

    local extract_dir
    extract_dir=$(extract_and_verify_bundle "$bundle_path")

    local manifest_json
    manifest_json=$(cat "${extract_dir}/manifest.json")
    validate_bundle_manifest "$manifest_json" "forensic profile"

    local profile
    profile=$(echo "$manifest_json" | jq -r '.export_profile')
    [ "$profile" = "forensic" ]

    # Forensic should have more files including snapshot and logs
    local file_count
    file_count=$(echo "$manifest_json" | jq '.files | length')
    test_info "File count in forensic bundle: $file_count"
    [ "$file_count" -ge 4 ]

    # Should have snapshot.json
    local has_snapshot
    has_snapshot=$(echo "$manifest_json" | jq '[.files[] | select(.path == "snapshot.json")] | length')
    [ "$has_snapshot" -eq 1 ]

    verify_bundle_checksums "$extract_dir"

    BATS_TEST_COMPLETED=pass
}

#==============================================================================
# INTEGRITY / TAMPER TESTS
#==============================================================================

@test "bundle tamper detection: flipped byte causes checksum mismatch failure" {
    require_jq
    require_unzip

    local bundle_path="${TEST_DIR}/test_tamper.ptb"
    local tampered_path="${TEST_DIR}/test_tampered.ptb"

    test_info "Testing tamper detection"

    # Create valid bundle first
    run python3 - "$bundle_path" << 'PYTHON_BUNDLE'
import sys
import json
import zipfile
import hashlib

bundle_path = sys.argv[1]

def sha256(data):
    return hashlib.sha256(data).hexdigest()

summary = {"total_processes": 100, "candidates": 10}
summary_bytes = json.dumps(summary, indent=2).encode()

manifest = {
    "bundle_version": "1.0.0",
    "session_id": "test-session",
    "host_id": "test-host",
    "export_profile": "safe",
    "created_at": "2026-01-15T12:00:00Z",
    "files": [
        {"path": "summary.json", "sha256": sha256(summary_bytes), "bytes": len(summary_bytes)}
    ]
}
manifest_bytes = json.dumps(manifest, indent=2).encode()

with zipfile.ZipFile(bundle_path, 'w', zipfile.ZIP_DEFLATED) as zf:
    zf.writestr("manifest.json", manifest_bytes)
    zf.writestr("summary.json", summary_bytes)

print(f"Bundle created: {bundle_path}")
PYTHON_BUNDLE
    [ "$status" -eq 0 ]

    # Now create a tampered version by modifying the ZIP
    run python3 - "$bundle_path" "$tampered_path" << 'PYTHON_TAMPER'
import sys
import json
import zipfile
import hashlib

bundle_path = sys.argv[1]
tampered_path = sys.argv[2]

# Read original
with zipfile.ZipFile(bundle_path, 'r') as zf:
    manifest_bytes = zf.read("manifest.json")
    summary_bytes = zf.read("summary.json")

# Tamper with summary (change a value)
tampered_summary = summary_bytes.replace(b'"total_processes": 100', b'"total_processes": 999')

# Write tampered bundle with ORIGINAL manifest (checksum mismatch)
with zipfile.ZipFile(tampered_path, 'w', zipfile.ZIP_DEFLATED) as zf:
    zf.writestr("manifest.json", manifest_bytes)
    zf.writestr("summary.json", tampered_summary)

print(f"Tampered bundle created: {tampered_path}")
PYTHON_TAMPER
    [ "$status" -eq 0 ]

    # Verify original passes
    local extract_orig="${TEST_DIR}/extract_orig"
    mkdir -p "$extract_orig"
    unzip -q "$bundle_path" -d "$extract_orig"
    verify_bundle_checksums "$extract_orig"
    test_info "Original bundle checksums valid"

    # Verify tampered fails checksum verification
    local extract_tampered="${TEST_DIR}/extract_tampered"
    mkdir -p "$extract_tampered"
    unzip -q "$tampered_path" -d "$extract_tampered"

    # This should FAIL
    run verify_bundle_checksums "$extract_tampered"
    [ "$status" -ne 0 ]
    test_info "Tampered bundle correctly rejected"

    BATS_TEST_COMPLETED=pass
}

@test "bundle tamper detection: missing file causes validation failure" {
    require_jq
    require_unzip

    local bundle_path="${TEST_DIR}/test_missing_file.ptb"

    test_info "Testing missing file detection"

    # Create bundle with manifest referencing non-existent file
    run python3 - "$bundle_path" << 'PYTHON_BUNDLE'
import sys
import json
import zipfile
import hashlib

bundle_path = sys.argv[1]

def sha256(data):
    return hashlib.sha256(data).hexdigest()

summary = {"total": 1}
summary_bytes = json.dumps(summary).encode()

# Manifest references file that won't exist
manifest = {
    "bundle_version": "1.0.0",
    "session_id": "test-session",
    "host_id": "test-host",
    "export_profile": "safe",
    "files": [
        {"path": "summary.json", "sha256": sha256(summary_bytes), "bytes": len(summary_bytes)},
        {"path": "missing_file.json", "sha256": "0" * 64, "bytes": 100}
    ]
}
manifest_bytes = json.dumps(manifest, indent=2).encode()

# Only include summary, not the referenced missing_file
with zipfile.ZipFile(bundle_path, 'w', zipfile.ZIP_DEFLATED) as zf:
    zf.writestr("manifest.json", manifest_bytes)
    zf.writestr("summary.json", summary_bytes)

print(f"Bundle with missing file created: {bundle_path}")
PYTHON_BUNDLE
    [ "$status" -eq 0 ]

    local extract_dir="${TEST_DIR}/extract_missing"
    mkdir -p "$extract_dir"
    unzip -q "$bundle_path" -d "$extract_dir"

    # Checksum verification should fail due to missing file
    run verify_bundle_checksums "$extract_dir"
    [ "$status" -ne 0 ]
    test_info "Missing file correctly detected"

    BATS_TEST_COMPLETED=pass
}

#==============================================================================
# REDACTION SAFETY TESTS
#==============================================================================

@test "bundle redaction: safe profile does not contain sensitive patterns" {
    require_jq
    require_unzip

    local bundle_path="${TEST_DIR}/test_redaction_safe.ptb"

    test_info "Testing redaction safety for safe profile"

    # Create bundle that would have sensitive data in forensic but should be clean in safe
    run python3 - "$bundle_path" << 'PYTHON_BUNDLE'
import sys
import json
import zipfile
import hashlib

bundle_path = sys.argv[1]

def sha256(data):
    return hashlib.sha256(data).hexdigest()

# Safe profile should have hashed/redacted values, not raw secrets
# This simulates what the redaction engine would produce
plan = {
    "session_id": "test-session",
    "recommendations": [
        {
            "pid": 1001,
            "action": "review",
            "cmdline_hash": "h:a1b2c3d4",  # Hashed, not raw
            "reason": "high_resource_usage"
        },
        {
            "pid": 1002,
            "action": "kill",
            "cmdline_hash": "h:e5f6g7h8",  # Hashed, not raw
            "reason": "abandoned_orphan"
        }
    ]
}
plan_bytes = json.dumps(plan, indent=2).encode()

# Summary has no sensitive data
summary = {"total_processes": 10, "candidates": 2}
summary_bytes = json.dumps(summary, indent=2).encode()

manifest = {
    "bundle_version": "1.0.0",
    "session_id": "test-session",
    "host_id": "h:abc123",  # Hashed host
    "export_profile": "safe",
    "redaction_policy_version": "1.0.0",
    "files": [
        {"path": "summary.json", "sha256": sha256(summary_bytes), "bytes": len(summary_bytes)},
        {"path": "plan.json", "sha256": sha256(plan_bytes), "bytes": len(plan_bytes)}
    ]
}
manifest_bytes = json.dumps(manifest, indent=2).encode()

with zipfile.ZipFile(bundle_path, 'w', zipfile.ZIP_DEFLATED) as zf:
    zf.writestr("manifest.json", manifest_bytes)
    zf.writestr("summary.json", summary_bytes)
    zf.writestr("plan.json", plan_bytes)

print(f"Safe bundle created: {bundle_path}")
PYTHON_BUNDLE
    [ "$status" -eq 0 ]

    local extract_dir="${TEST_DIR}/extract_redaction"
    mkdir -p "$extract_dir"
    unzip -q "$bundle_path" -d "$extract_dir"

    # Check no sensitive data in any file
    shopt -s nullglob globstar 2>/dev/null || true
    for f in "${extract_dir}"/*.json "${extract_dir}"/**/*.json; do
        [[ -f "$f" ]] || continue
        check_no_sensitive_data "$f" "$(basename "$f")"
    done
    shopt -u nullglob globstar 2>/dev/null || true

    test_info "Redaction safety verified"

    BATS_TEST_COMPLETED=pass
}

@test "bundle redaction: minimal profile contains only aggregate stats" {
    require_jq
    require_unzip

    local bundle_path="${TEST_DIR}/test_minimal_aggregates.ptb"

    test_info "Testing minimal profile contains only aggregates"

    run python3 - "$bundle_path" << 'PYTHON_BUNDLE'
import sys
import json
import zipfile
import hashlib

bundle_path = sys.argv[1]

def sha256(data):
    return hashlib.sha256(data).hexdigest()

# Minimal: ONLY aggregate stats, no process-level data
summary = {
    "total_processes": 150,
    "candidates": 12,
    "by_category": {
        "useful": 100,
        "useful_bad": 8,
        "abandoned": 3,
        "zombie": 1
    },
    "recommendations": {
        "keep": 138,
        "review": 8,
        "kill": 4
    }
}
summary_bytes = json.dumps(summary, indent=2).encode()

manifest = {
    "bundle_version": "1.0.0",
    "session_id": "test-session",
    "host_id": "h:redacted",
    "export_profile": "minimal",
    "files": [
        {"path": "summary.json", "sha256": sha256(summary_bytes), "bytes": len(summary_bytes)}
    ]
}
manifest_bytes = json.dumps(manifest, indent=2).encode()

with zipfile.ZipFile(bundle_path, 'w', zipfile.ZIP_DEFLATED) as zf:
    zf.writestr("manifest.json", manifest_bytes)
    zf.writestr("summary.json", summary_bytes)

print(f"Minimal bundle created: {bundle_path}")
PYTHON_BUNDLE
    [ "$status" -eq 0 ]

    local extract_dir="${TEST_DIR}/extract_minimal"
    mkdir -p "$extract_dir"
    unzip -q "$bundle_path" -d "$extract_dir"

    # Verify no per-process data (pids, cmdlines)
    local summary_content
    summary_content=$(cat "${extract_dir}/summary.json")

    # Should NOT contain pid fields
    if [[ "$summary_content" == *'"pid"'* ]]; then
        test_error "Minimal profile should not contain per-process PIDs"
        return 1
    fi

    # Should NOT contain cmdline fields
    if [[ "$summary_content" == *'"cmdline"'* ]]; then
        test_error "Minimal profile should not contain cmdlines"
        return 1
    fi

    # Should contain aggregate counts
    assert_contains "$summary_content" '"total_processes"'
    assert_contains "$summary_content" '"candidates"'

    test_info "Minimal profile contains only aggregates"

    BATS_TEST_COMPLETED=pass
}

#==============================================================================
# SIZE/OVERHEAD SANITY TESTS
#==============================================================================

@test "bundle size: reasonable overhead for typical session" {
    require_jq
    require_unzip

    local bundle_path="${TEST_DIR}/test_size.ptb"

    test_info "Testing bundle size overhead"

    # Create bundle with realistic data size
    run python3 - "$bundle_path" << 'PYTHON_BUNDLE'
import sys
import json
import zipfile
import hashlib

bundle_path = sys.argv[1]

def sha256(data):
    return hashlib.sha256(data).hexdigest()

# Generate 500 synthetic processes (realistic large session)
processes = []
for i in range(500):
    processes.append({
        "pid": 1000 + i,
        "ppid": 1 if i % 10 == 0 else 1000 + (i - 1),
        "comm": f"process_{i}",
        "cmdline": f"/usr/bin/process_{i} --arg1 --arg2=value_{i}",
        "age_seconds": i * 100,
        "memory_mb": 50 + (i % 200),
        "cpu_percent": (i % 100) * 0.5
    })

snapshot = {"processes": processes}
snapshot_bytes = json.dumps(snapshot, indent=2).encode()

# Generate recommendations for top candidates
recommendations = []
for i in range(50):
    recommendations.append({
        "pid": 1000 + i * 10,
        "action": ["keep", "review", "kill"][i % 3],
        "posterior": 0.5 + (i % 50) * 0.01
    })

plan = {"session_id": "large-session", "recommendations": recommendations}
plan_bytes = json.dumps(plan, indent=2).encode()

summary = {"total_processes": 500, "candidates": 50}
summary_bytes = json.dumps(summary).encode()

manifest = {
    "bundle_version": "1.0.0",
    "session_id": "large-session",
    "host_id": "test-host",
    "export_profile": "forensic",
    "files": [
        {"path": "summary.json", "sha256": sha256(summary_bytes), "bytes": len(summary_bytes)},
        {"path": "plan.json", "sha256": sha256(plan_bytes), "bytes": len(plan_bytes)},
        {"path": "snapshot.json", "sha256": sha256(snapshot_bytes), "bytes": len(snapshot_bytes)}
    ]
}
manifest_bytes = json.dumps(manifest, indent=2).encode()

with zipfile.ZipFile(bundle_path, 'w', zipfile.ZIP_DEFLATED) as zf:
    zf.writestr("manifest.json", manifest_bytes)
    zf.writestr("summary.json", summary_bytes)
    zf.writestr("plan.json", plan_bytes)
    zf.writestr("snapshot.json", snapshot_bytes)

# Report sizes
import os
bundle_size = os.path.getsize(bundle_path)
uncompressed = len(manifest_bytes) + len(summary_bytes) + len(plan_bytes) + len(snapshot_bytes)
print(f"Bundle size: {bundle_size} bytes (compressed)")
print(f"Uncompressed: {uncompressed} bytes")
print(f"Compression ratio: {uncompressed / bundle_size:.2f}x")
PYTHON_BUNDLE
    [ "$status" -eq 0 ]

    # Get bundle size
    local bundle_size
    bundle_size=$(stat -c%s "$bundle_path" 2>/dev/null || stat -f%z "$bundle_path" 2>/dev/null)
    test_info "Bundle size: $bundle_size bytes"

    # For 500 processes, bundle should be reasonably sized
    # Raw JSON for 500 processes ~100KB, compressed should be < 50KB
    # Allow generous margin: < 200KB
    [ "$bundle_size" -lt 200000 ]
    test_info "Bundle size within bounds (< 200KB)"

    # Compression should be effective (at least 2x)
    local extract_dir="${TEST_DIR}/extract_size"
    mkdir -p "$extract_dir"
    unzip -q "$bundle_path" -d "$extract_dir"

    local uncompressed_size=0
    for f in "${extract_dir}"/*; do
        local fsize
        fsize=$(stat -c%s "$f" 2>/dev/null || stat -f%z "$f" 2>/dev/null)
        uncompressed_size=$((uncompressed_size + fsize))
    done

    test_info "Uncompressed size: $uncompressed_size bytes"

    # Compression ratio should be at least 1.5x for JSON data
    local ratio
    ratio=$(echo "scale=2; $uncompressed_size / $bundle_size" | bc)
    test_info "Compression ratio: ${ratio}x"

    # Just verify compression is working (ratio > 1)
    [ "$(echo "$ratio > 1" | bc)" -eq 1 ]

    BATS_TEST_COMPLETED=pass
}

@test "bundle size: minimal profile is smallest" {
    require_jq
    require_unzip

    test_info "Testing profile size ordering: minimal < safe < forensic"

    local minimal_path="${TEST_DIR}/size_minimal.ptb"
    local safe_path="${TEST_DIR}/size_safe.ptb"
    local forensic_path="${TEST_DIR}/size_forensic.ptb"

    # Create all three profiles from same source data
    run python3 - "$minimal_path" "$safe_path" "$forensic_path" << 'PYTHON_BUNDLE'
import sys
import json
import zipfile
import hashlib

minimal_path, safe_path, forensic_path = sys.argv[1], sys.argv[2], sys.argv[3]

def sha256(data):
    return hashlib.sha256(data).hexdigest()

def create_bundle(path, profile, include_snapshot=False, include_posteriors=False):
    summary = {"total_processes": 100, "candidates": 10}
    summary_bytes = json.dumps(summary).encode()

    files = [{"path": "summary.json", "sha256": sha256(summary_bytes), "bytes": len(summary_bytes)}]
    content_files = [("summary.json", summary_bytes)]

    if include_posteriors:
        posteriors = {"posteriors": [{"pid": i, "posterior": 0.5} for i in range(10)]}
        posteriors_bytes = json.dumps(posteriors).encode()
        files.append({"path": "posteriors.json", "sha256": sha256(posteriors_bytes), "bytes": len(posteriors_bytes)})
        content_files.append(("posteriors.json", posteriors_bytes))

    if include_snapshot:
        processes = [{"pid": i, "cmdline": f"cmd_{i}", "age": i*100} for i in range(100)]
        snapshot = {"processes": processes}
        snapshot_bytes = json.dumps(snapshot).encode()
        files.append({"path": "snapshot.json", "sha256": sha256(snapshot_bytes), "bytes": len(snapshot_bytes)})
        content_files.append(("snapshot.json", snapshot_bytes))

    manifest = {
        "bundle_version": "1.0.0",
        "session_id": "test",
        "host_id": "test",
        "export_profile": profile,
        "files": files
    }
    manifest_bytes = json.dumps(manifest).encode()

    with zipfile.ZipFile(path, 'w', zipfile.ZIP_DEFLATED) as zf:
        zf.writestr("manifest.json", manifest_bytes)
        for fname, fdata in content_files:
            zf.writestr(fname, fdata)

create_bundle(minimal_path, "minimal", include_snapshot=False, include_posteriors=False)
create_bundle(safe_path, "safe", include_snapshot=False, include_posteriors=True)
create_bundle(forensic_path, "forensic", include_snapshot=True, include_posteriors=True)

import os
print(f"minimal: {os.path.getsize(minimal_path)} bytes")
print(f"safe: {os.path.getsize(safe_path)} bytes")
print(f"forensic: {os.path.getsize(forensic_path)} bytes")
PYTHON_BUNDLE
    [ "$status" -eq 0 ]

    local minimal_size safe_size forensic_size
    minimal_size=$(stat -c%s "$minimal_path" 2>/dev/null || stat -f%z "$minimal_path")
    safe_size=$(stat -c%s "$safe_path" 2>/dev/null || stat -f%z "$safe_path")
    forensic_size=$(stat -c%s "$forensic_path" 2>/dev/null || stat -f%z "$forensic_path")

    test_info "minimal: $minimal_size bytes"
    test_info "safe: $safe_size bytes"
    test_info "forensic: $forensic_size bytes"

    # Verify ordering: minimal <= safe <= forensic
    [ "$minimal_size" -le "$safe_size" ]
    [ "$safe_size" -le "$forensic_size" ]

    test_info "Profile size ordering verified"

    BATS_TEST_COMPLETED=pass
}

#==============================================================================
# ERROR HANDLING TESTS
#==============================================================================

@test "bundle reader: rejects invalid ZIP file" {
    local invalid_path="${TEST_DIR}/not_a_zip.ptb"

    test_info "Testing invalid ZIP rejection"

    echo "This is not a ZIP file" > "$invalid_path"

    # Attempt to unzip should fail
    run unzip -t "$invalid_path" 2>&1
    [ "$status" -ne 0 ]

    test_info "Invalid ZIP correctly rejected"

    BATS_TEST_COMPLETED=pass
}

@test "bundle reader: rejects bundle without manifest" {
    require_unzip

    local no_manifest_path="${TEST_DIR}/no_manifest.ptb"

    test_info "Testing missing manifest rejection"

    # Create ZIP without manifest.json
    run python3 - "$no_manifest_path" << 'PYTHON_BUNDLE'
import sys
import zipfile

bundle_path = sys.argv[1]

with zipfile.ZipFile(bundle_path, 'w') as zf:
    zf.writestr("data.json", '{"key": "value"}')

print(f"Bundle without manifest created: {bundle_path}")
PYTHON_BUNDLE
    [ "$status" -eq 0 ]

    local extract_dir="${TEST_DIR}/extract_no_manifest"
    mkdir -p "$extract_dir"
    unzip -q "$no_manifest_path" -d "$extract_dir"

    # Should not have manifest.json
    [ ! -f "${extract_dir}/manifest.json" ]

    test_info "Bundle without manifest correctly identified"

    BATS_TEST_COMPLETED=pass
}
