//! End-to-end tests for agent plan workflow.
//!
//! Tests the `agent plan` command which generates action plans
//! without execution for review and validation.

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
// Basic Plan Tests
// ============================================================================

mod plan_basic {
    use super::*;

    #[test]
    fn plan_runs_without_error() {
        pt_core().args(["agent", "plan"]).assert().success();
    }

    #[test]
    fn plan_with_json_format() {
        pt_core()
            .args(["--format", "json", "agent", "plan"])
            .assert()
            .success()
            .stdout(predicate::str::contains("schema_version"))
            .stdout(predicate::str::contains("session_id"));
    }

    #[test]
    fn plan_produces_valid_json() {
        let output = pt_core()
            .args(["--format", "json", "agent", "plan"])
            .assert()
            .success()
            .get_output()
            .stdout
            .clone();

        let json: Value = serde_json::from_slice(&output).expect("Output should be valid JSON");

        // Verify required fields exist
        assert!(
            json.get("schema_version").is_some(),
            "Missing schema_version"
        );
        assert!(json.get("session_id").is_some(), "Missing session_id");
        assert!(json.get("generated_at").is_some(), "Missing generated_at");
    }
}

// ============================================================================
// Plan Options Tests
// ============================================================================

mod plan_options {
    use super::*;

    #[test]
    fn plan_with_max_candidates() {
        pt_core()
            .args(["agent", "plan", "--max-candidates", "10"])
            .assert()
            .success();
    }

    #[test]
    fn plan_with_threshold() {
        pt_core()
            .args(["agent", "plan", "--threshold", "0.8"])
            .assert()
            .success();
    }

    #[test]
    fn plan_with_only_filter_kill() {
        pt_core()
            .args(["agent", "plan", "--only", "kill"])
            .assert()
            .success();
    }

    #[test]
    fn plan_with_only_filter_review() {
        pt_core()
            .args(["agent", "plan", "--only", "review"])
            .assert()
            .success();
    }

    #[test]
    fn plan_with_only_filter_all() {
        pt_core()
            .args(["agent", "plan", "--only", "all"])
            .assert()
            .success();
    }

    #[test]
    fn plan_with_yes_flag() {
        pt_core()
            .args(["agent", "plan", "--yes"])
            .assert()
            .success();
    }

    #[test]
    fn plan_with_combined_options() {
        pt_core()
            .args([
                "agent",
                "plan",
                "--max-candidates",
                "5",
                "--threshold",
                "0.9",
                "--only",
                "kill",
            ])
            .assert()
            .success();
    }
}

// ============================================================================
// Output Format Tests
// ============================================================================

mod plan_formats {
    use super::*;

    #[test]
    fn plan_json_format() {
        pt_core()
            .args(["--format", "json", "agent", "plan"])
            .assert()
            .success()
            .stdout(predicate::str::starts_with("{"));
    }

    #[test]
    fn plan_summary_format() {
        pt_core()
            .args(["--format", "summary", "agent", "plan"])
            .assert()
            .success()
            .stdout(predicate::str::contains("agent plan"));
    }

    #[test]
    fn plan_prose_format() {
        pt_core()
            .args(["--format", "prose", "agent", "plan"])
            .assert()
            .success()
            .stdout(predicate::str::contains("pt-core"));
    }

    #[test]
    fn plan_exitcode_format() {
        // Exitcode format produces no output on success
        pt_core()
            .args(["--format", "exitcode", "agent", "plan"])
            .assert()
            .success()
            .stdout(predicate::str::is_empty());
    }
}

// ============================================================================
// Schema Validation Tests
// ============================================================================

mod plan_schema {
    use super::*;

    #[test]
    fn plan_has_schema_version() {
        let output = pt_core()
            .args(["--format", "json", "agent", "plan"])
            .assert()
            .success()
            .get_output()
            .stdout
            .clone();

        let json: Value = serde_json::from_slice(&output).unwrap();
        let version = json
            .get("schema_version")
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
    fn plan_session_id_is_valid() {
        let output = pt_core()
            .args(["--format", "json", "agent", "plan"])
            .assert()
            .success()
            .get_output()
            .stdout
            .clone();

        let json: Value = serde_json::from_slice(&output).unwrap();
        let session_id = json
            .get("session_id")
            .expect("Missing session_id")
            .as_str()
            .expect("session_id should be string");

        // Session ID should be non-empty
        assert!(!session_id.is_empty(), "Session ID should not be empty");
        assert!(
            session_id.len() >= 8,
            "Session ID seems too short: {}",
            session_id
        );
    }

    #[test]
    fn plan_generated_at_is_iso8601() {
        let output = pt_core()
            .args(["--format", "json", "agent", "plan"])
            .assert()
            .success()
            .get_output()
            .stdout
            .clone();

        let json: Value = serde_json::from_slice(&output).unwrap();
        let generated_at = json
            .get("generated_at")
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

mod plan_integration {
    use super::*;

    #[test]
    fn plan_with_dry_run() {
        pt_core()
            .args(["--dry-run", "--format", "json", "agent", "plan"])
            .assert()
            .success();
    }

    #[test]
    fn plan_with_robot_mode() {
        pt_core()
            .args(["--robot", "--format", "json", "agent", "plan"])
            .assert()
            .success();
    }

    #[test]
    fn plan_with_shadow_mode() {
        pt_core()
            .args(["--shadow", "--format", "json", "agent", "plan"])
            .assert()
            .success();
    }

    #[test]
    fn plan_with_verbose_flag() {
        pt_core()
            .args(["-v", "--format", "json", "agent", "plan"])
            .assert()
            .success();
    }

    #[test]
    fn plan_with_quiet_flag() {
        pt_core()
            .args(["-q", "--format", "json", "agent", "plan"])
            .assert()
            .success();
    }

    #[test]
    fn plan_with_standalone_flag() {
        pt_core()
            .args(["--standalone", "--format", "json", "agent", "plan"])
            .assert()
            .success();
    }

    #[test]
    fn consecutive_plans_have_different_session_ids() {
        let output1 = pt_core()
            .args(["--format", "json", "agent", "plan"])
            .assert()
            .success()
            .get_output()
            .stdout
            .clone();

        let output2 = pt_core()
            .args(["--format", "json", "agent", "plan"])
            .assert()
            .success()
            .get_output()
            .stdout
            .clone();

        let json1: Value = serde_json::from_slice(&output1).unwrap();
        let json2: Value = serde_json::from_slice(&output2).unwrap();

        let id1 = json1.get("session_id").unwrap().as_str().unwrap();
        let id2 = json2.get("session_id").unwrap().as_str().unwrap();

        assert_ne!(id1, id2, "Each plan should have unique session ID");
    }
}

// ============================================================================
// Error Handling Tests
// ============================================================================

mod plan_errors {
    use super::*;

    #[test]
    fn plan_with_invalid_format_fails() {
        pt_core()
            .args(["--format", "invalid_format", "agent", "plan"])
            .assert()
            .failure();
    }

    // NOTE: Threshold validation not yet implemented in stub
    // When implemented, add test for invalid threshold values

    #[test]
    fn plan_help_works() {
        pt_core()
            .args(["agent", "plan", "--help"])
            .assert()
            .success()
            .stdout(predicate::str::contains("plan"))
            .stdout(predicate::str::contains("threshold"));
    }
}
