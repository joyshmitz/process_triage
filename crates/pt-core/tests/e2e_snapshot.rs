//! End-to-end tests for agent snapshot workflow.
//!
//! Tests the `agent snapshot` command which creates session snapshots
//! for later comparison and verification.

use assert_cmd::cargo::cargo_bin_cmd;
use assert_cmd::Command;
use predicates::prelude::*;
use serde_json::Value;
use std::time::Duration;

/// Get a Command for pt-core binary.
fn pt_core() -> Command {
    let mut cmd = cargo_bin_cmd!("pt-core");
    cmd.timeout(Duration::from_secs(30));
    cmd
}

// ============================================================================
// Basic Snapshot Tests
// ============================================================================

mod snapshot_basic {
    use super::*;

    #[test]
    fn snapshot_runs_without_error() {
        pt_core()
            .args(["agent", "snapshot"])
            .assert()
            .success();
    }

    #[test]
    fn snapshot_with_json_format() {
        pt_core()
            .args(["--format", "json", "agent", "snapshot"])
            .assert()
            .success()
            .stdout(predicate::str::contains("schema_version"))
            .stdout(predicate::str::contains("session_id"));
    }

    #[test]
    fn snapshot_produces_valid_json() {
        let output = pt_core()
            .args(["--format", "json", "agent", "snapshot"])
            .assert()
            .success()
            .get_output()
            .stdout
            .clone();

        let json: Value = serde_json::from_slice(&output)
            .expect("Output should be valid JSON");

        // Verify required fields exist
        assert!(json.get("schema_version").is_some(), "Missing schema_version");
        assert!(json.get("session_id").is_some(), "Missing session_id");
        assert!(json.get("generated_at").is_some(), "Missing generated_at");
    }

    #[test]
    fn snapshot_with_label() {
        pt_core()
            .args(["agent", "snapshot", "--label", "test-snapshot"])
            .assert()
            .success();
    }
}

// ============================================================================
// Output Format Tests
// ============================================================================

mod snapshot_formats {
    use super::*;

    #[test]
    fn snapshot_json_format() {
        pt_core()
            .args(["--format", "json", "agent", "snapshot"])
            .assert()
            .success()
            .stdout(predicate::str::starts_with("{"));
    }

    #[test]
    fn snapshot_summary_format() {
        pt_core()
            .args(["--format", "summary", "agent", "snapshot"])
            .assert()
            .success()
            .stdout(predicate::str::contains("agent snapshot"));
    }

    #[test]
    fn snapshot_prose_format() {
        pt_core()
            .args(["--format", "prose", "agent", "snapshot"])
            .assert()
            .success()
            .stdout(predicate::str::contains("pt-core"));
    }

    #[test]
    fn snapshot_exitcode_format() {
        // Exitcode format produces no output on success
        pt_core()
            .args(["--format", "exitcode", "agent", "snapshot"])
            .assert()
            .success()
            .stdout(predicate::str::is_empty());
    }
}

// ============================================================================
// Schema Validation Tests
// ============================================================================

mod snapshot_schema {
    use super::*;

    #[test]
    fn snapshot_has_schema_version() {
        let output = pt_core()
            .args(["--format", "json", "agent", "snapshot"])
            .assert()
            .success()
            .get_output()
            .stdout
            .clone();

        let json: Value = serde_json::from_slice(&output).unwrap();
        let version = json.get("schema_version")
            .expect("Missing schema_version")
            .as_str()
            .expect("schema_version should be string");

        // Schema version should be semver-like
        assert!(
            version.contains('.'),
            "Schema version should be semver format: {}",
            version
        );
    }

    #[test]
    fn snapshot_session_id_is_valid() {
        let output = pt_core()
            .args(["--format", "json", "agent", "snapshot"])
            .assert()
            .success()
            .get_output()
            .stdout
            .clone();

        let json: Value = serde_json::from_slice(&output).unwrap();
        let session_id = json.get("session_id")
            .expect("Missing session_id")
            .as_str()
            .expect("session_id should be string");

        // Session ID should be non-empty and have reasonable format
        assert!(!session_id.is_empty(), "Session ID should not be empty");
        assert!(session_id.len() >= 8, "Session ID seems too short: {}", session_id);
    }

    #[test]
    fn snapshot_generated_at_is_iso8601() {
        let output = pt_core()
            .args(["--format", "json", "agent", "snapshot"])
            .assert()
            .success()
            .get_output()
            .stdout
            .clone();

        let json: Value = serde_json::from_slice(&output).unwrap();
        let generated_at = json.get("generated_at")
            .expect("Missing generated_at")
            .as_str()
            .expect("generated_at should be string");

        // ISO 8601 timestamps contain 'T' separator
        assert!(
            generated_at.contains('T'),
            "Timestamp should be ISO 8601: {}",
            generated_at
        );
    }
}

// ============================================================================
// Integration Tests
// ============================================================================

mod snapshot_integration {
    use super::*;

    #[test]
    fn consecutive_snapshots_have_different_session_ids() {
        let output1 = pt_core()
            .args(["--format", "json", "agent", "snapshot"])
            .assert()
            .success()
            .get_output()
            .stdout
            .clone();

        let output2 = pt_core()
            .args(["--format", "json", "agent", "snapshot"])
            .assert()
            .success()
            .get_output()
            .stdout
            .clone();

        let json1: Value = serde_json::from_slice(&output1).unwrap();
        let json2: Value = serde_json::from_slice(&output2).unwrap();

        let id1 = json1.get("session_id").unwrap().as_str().unwrap();
        let id2 = json2.get("session_id").unwrap().as_str().unwrap();

        assert_ne!(id1, id2, "Each snapshot should have unique session ID");
    }

    #[test]
    fn snapshot_with_verbose_flag() {
        pt_core()
            .args(["-v", "--format", "json", "agent", "snapshot"])
            .assert()
            .success();
    }

    #[test]
    fn snapshot_with_quiet_flag() {
        pt_core()
            .args(["-q", "--format", "json", "agent", "snapshot"])
            .assert()
            .success();
    }

    #[test]
    fn snapshot_with_standalone_flag() {
        pt_core()
            .args(["--standalone", "--format", "json", "agent", "snapshot"])
            .assert()
            .success();
    }
}

// ============================================================================
// Error Handling Tests
// ============================================================================

mod snapshot_errors {
    use super::*;

    #[test]
    fn snapshot_with_invalid_format_fails() {
        pt_core()
            .args(["--format", "invalid_format", "agent", "snapshot"])
            .assert()
            .failure();
    }

    #[test]
    fn snapshot_help_works() {
        pt_core()
            .args(["agent", "snapshot", "--help"])
            .assert()
            .success()
            .stdout(predicate::str::contains("snapshot"));
    }
}
