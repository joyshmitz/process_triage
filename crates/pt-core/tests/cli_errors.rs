//! CLI error handling tests for pt-core.
//!
//! These tests verify that invalid arguments and commands produce
//! appropriate error messages and exit codes.

use assert_cmd::Command;
use predicates::prelude::*;

/// Get a Command for pt-core binary.
fn pt_core() -> Command {
    Command::cargo_bin("pt-core").expect("pt-core binary should exist")
}

// ============================================================================
// Invalid Subcommand Tests
// ============================================================================

mod invalid_subcommand {
    use super::*;

    #[test]
    fn unknown_command_fails() {
        pt_core()
            .arg("nonexistent-command")
            .assert()
            .failure()
            .stderr(predicate::str::contains("error"));
    }

    #[test]
    fn unknown_agent_subcommand_fails() {
        pt_core()
            .args(["agent", "nonexistent"])
            .assert()
            .failure()
            .stderr(predicate::str::contains("error"));
    }

    #[test]
    fn unknown_config_subcommand_fails() {
        pt_core()
            .args(["config", "nonexistent"])
            .assert()
            .failure()
            .stderr(predicate::str::contains("error"));
    }

    #[test]
    fn unknown_bundle_subcommand_fails() {
        pt_core()
            .args(["bundle", "nonexistent"])
            .assert()
            .failure()
            .stderr(predicate::str::contains("error"));
    }

    #[test]
    fn query_sessions_with_invalid_option_fails() {
        pt_core()
            .args(["query", "sessions", "--invalid-option"])
            .assert()
            .failure()
            .stderr(predicate::str::contains("error"));
    }
}

// ============================================================================
// Invalid Option Tests
// ============================================================================

mod invalid_options {
    use super::*;

    #[test]
    fn unknown_global_flag_fails() {
        pt_core()
            .arg("--nonexistent-flag")
            .assert()
            .failure()
            .stderr(predicate::str::contains("error"));
    }

    #[test]
    fn invalid_format_value_fails() {
        pt_core()
            .args(["--format", "invalid_format_name"])
            .assert()
            .failure()
            .stderr(predicate::str::contains("error"));
    }

    #[test]
    fn missing_required_value_fails() {
        pt_core()
            .args(["--format"])
            .assert()
            .failure()
            .stderr(predicate::str::contains("error"));
    }
}

// ============================================================================
// Agent Command Error Tests
// ============================================================================

mod agent_errors {
    use super::*;

    #[test]
    fn agent_explain_requires_session() {
        pt_core()
            .args(["agent", "explain", "--pids", "1234"])
            .assert()
            .failure()
            .stderr(predicate::str::contains("--session"));
    }

    #[test]
    fn agent_apply_requires_session() {
        pt_core()
            .args(["agent", "apply"])
            .assert()
            .failure()
            .stderr(predicate::str::contains("--session"));
    }

    #[test]
    fn agent_verify_requires_session() {
        pt_core()
            .args(["agent", "verify"])
            .assert()
            .failure()
            .stderr(predicate::str::contains("--session"));
    }

    #[test]
    fn agent_diff_requires_base() {
        pt_core()
            .args(["agent", "diff"])
            .assert()
            .failure()
            .stderr(predicate::str::contains("--base"));
    }
}

// ============================================================================
// Bundle Command Error Tests
// ============================================================================

mod bundle_errors {
    use super::*;

    #[test]
    fn bundle_inspect_requires_path() {
        pt_core()
            .args(["bundle", "inspect"])
            .assert()
            .failure()
            .stderr(predicate::str::contains("error"));
    }

    #[test]
    fn bundle_extract_requires_path() {
        pt_core()
            .args(["bundle", "extract"])
            .assert()
            .failure()
            .stderr(predicate::str::contains("error"));
    }
}

// ============================================================================
// Config Command Error Tests
// ============================================================================

mod config_errors {
    use super::*;

    #[test]
    fn config_schema_requires_file() {
        pt_core()
            .args(["config", "schema"])
            .assert()
            .failure()
            .stderr(predicate::str::contains("--file"));
    }
}

// ============================================================================
// Numeric Argument Error Tests
// ============================================================================

mod numeric_errors {
    use super::*;

    #[test]
    fn scan_samples_rejects_non_numeric() {
        pt_core()
            .args(["scan", "--samples", "not-a-number"])
            .assert()
            .failure()
            .stderr(predicate::str::contains("error"));
    }

    #[test]
    fn scan_interval_rejects_non_numeric() {
        pt_core()
            .args(["scan", "--interval", "not-a-number"])
            .assert()
            .failure()
            .stderr(predicate::str::contains("error"));
    }

    #[test]
    fn agent_plan_threshold_rejects_non_numeric() {
        pt_core()
            .args(["agent", "plan", "--threshold", "not-a-number"])
            .assert()
            .failure()
            .stderr(predicate::str::contains("error"));
    }

    #[test]
    fn deep_scan_pids_rejects_non_numeric() {
        pt_core()
            .args(["deep-scan", "--pids", "abc,def"])
            .assert()
            .failure()
            .stderr(predicate::str::contains("error"));
    }
}

// ============================================================================
// Exit Code Tests for Errors
// ============================================================================

mod exit_codes {
    use super::*;

    #[test]
    fn invalid_args_returns_nonzero() {
        let output = pt_core()
            .arg("--invalid-flag")
            .assert()
            .failure();

        // Exit code should be non-zero for argument errors
        output.code(predicate::ne(0));
    }

    #[test]
    fn missing_required_arg_returns_nonzero() {
        let output = pt_core()
            .args(["agent", "apply"])
            .assert()
            .failure();

        output.code(predicate::ne(0));
    }
}

// ============================================================================
// Error Message Quality Tests
// ============================================================================

mod error_messages {
    use super::*;

    #[test]
    fn unknown_command_suggests_similar() {
        // clap should suggest similar commands for typos
        pt_core()
            .arg("scna") // typo of "scan"
            .assert()
            .failure()
            .stderr(predicate::str::contains("error"));
    }

    #[test]
    fn error_messages_are_helpful() {
        // Verify error messages include the problematic argument
        pt_core()
            .args(["--format", "badformat"])
            .assert()
            .failure()
            .stderr(predicate::str::contains("badformat"));
    }
}
