//! CLI E2E tests for `pt-core telemetry status` and `pt-core telemetry prune`.
//!
//! Validates:
//! - Exit codes for success and failure paths
//! - JSON output schema for status and prune commands
//! - Deterministic status→prune→status workflow via CLI
//! - Error messages for invalid arguments and configs

use assert_cmd::cargo::cargo_bin_cmd;
use assert_cmd::Command;
use predicates::prelude::*;
use serde_json::Value;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::time::{Duration, SystemTime};
use tempfile::tempdir;

// ============================================================================
// Helpers
// ============================================================================

/// Get a Command for pt-core binary.
fn pt_core() -> Command {
    let mut cmd = cargo_bin_cmd!("pt-core");
    cmd.timeout(Duration::from_secs(60));
    cmd
}

/// Create a fake parquet file with specific size and simulated age.
fn create_fake_parquet(path: &Path, size_bytes: usize, age_days: u64) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let mut file = fs::File::create(path)?;
    file.write_all(&vec![0u8; size_bytes])?;
    file.sync_all()?;

    let mtime = SystemTime::now() - Duration::from_secs(age_days * 86400);
    let atime = filetime::FileTime::from_system_time(mtime);
    filetime::set_file_times(path, atime, atime)?;
    Ok(())
}

/// Create a retention config JSON file.
fn write_retention_config(dir: &Path, config_json: &str) -> std::path::PathBuf {
    let config_path = dir.join("telemetry_retention.json");
    let mut f = fs::File::create(&config_path).expect("create config file");
    f.write_all(config_json.as_bytes())
        .expect("write config file");
    config_path
}

// ============================================================================
// Status Command Tests
// ============================================================================

#[test]
fn test_telemetry_status_success_exit_code() {
    let dir = tempdir().expect("tempdir");

    pt_core()
        .args([
            "--format",
            "json",
            "telemetry",
            "--telemetry-dir",
            dir.path().to_str().unwrap(),
            "status",
        ])
        .assert()
        .success()
        .code(0);
}

#[test]
fn test_telemetry_status_json_schema() {
    let dir = tempdir().expect("tempdir");
    let root = dir.path();

    // Create some fixture data
    let file = root.join("proc_samples/year=2025/month=01/day=15/host_id=test/data.parquet");
    create_fake_parquet(&file, 1024, 10).expect("create fixture");

    let output = pt_core()
        .args([
            "--format",
            "json",
            "telemetry",
            "--telemetry-dir",
            root.to_str().unwrap(),
            "status",
        ])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let json: Value = serde_json::from_slice(&output).expect("parse JSON output");

    // Verify required fields
    assert!(
        json.get("command").is_some(),
        "JSON should have 'command' field"
    );
    assert_eq!(json["command"], "telemetry status");

    assert!(
        json.get("status").is_some(),
        "JSON should have 'status' field"
    );

    let status = &json["status"];
    assert!(
        status.get("total_bytes").is_some(),
        "status should have total_bytes"
    );
    assert!(
        status.get("total_files").is_some(),
        "status should have total_files"
    );
    assert!(
        status.get("by_table").is_some(),
        "status should have by_table"
    );
    assert!(
        status.get("disk_budget_bytes").is_some(),
        "status should have disk_budget_bytes"
    );
    assert!(
        status.get("ttl_eligible_files").is_some(),
        "status should have ttl_eligible_files"
    );

    // Verify data makes sense
    assert_eq!(status["total_files"], 1, "should see 1 file");
    assert_eq!(status["total_bytes"], 1024, "should see 1024 bytes");
}

#[test]
fn test_telemetry_status_empty_dir() {
    let dir = tempdir().expect("tempdir");

    let output = pt_core()
        .args([
            "--format",
            "json",
            "telemetry",
            "--telemetry-dir",
            dir.path().to_str().unwrap(),
            "status",
        ])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let json: Value = serde_json::from_slice(&output).expect("parse JSON output");
    assert_eq!(json["status"]["total_files"], 0);
    assert_eq!(json["status"]["total_bytes"], 0);
}

// ============================================================================
// Prune Command Tests
// ============================================================================

#[test]
fn test_telemetry_prune_dry_run_success() {
    let dir = tempdir().expect("tempdir");
    let root = dir.path();

    let file = root.join("proc_samples/year=2025/month=01/day=01/host_id=test/old.parquet");
    create_fake_parquet(&file, 1024, 45).expect("create fixture");

    let output = pt_core()
        .args([
            "--format",
            "json",
            "telemetry",
            "--telemetry-dir",
            root.to_str().unwrap(),
            "prune",
            "--keep",
            "30d",
            "--dry-run",
        ])
        .assert()
        .success()
        .code(0)
        .get_output()
        .stdout
        .clone();

    let json: Value = serde_json::from_slice(&output).expect("parse JSON output");
    assert_eq!(json["command"], "telemetry prune");
    assert_eq!(json["dry_run"], true);
    assert!(
        json["event_count"].as_u64().unwrap() >= 1,
        "should report at least 1 eligible file"
    );

    // File should still exist (dry run)
    assert!(file.exists(), "file should NOT be deleted in dry-run mode");
}

#[test]
fn test_telemetry_prune_real_deletes_files() {
    let dir = tempdir().expect("tempdir");
    let root = dir.path();

    let file = root.join("proc_samples/year=2025/month=01/day=01/host_id=test/old.parquet");
    create_fake_parquet(&file, 1024, 45).expect("create fixture");
    assert!(file.exists(), "file should exist before prune");

    let output = pt_core()
        .args([
            "--format",
            "json",
            "telemetry",
            "--telemetry-dir",
            root.to_str().unwrap(),
            "prune",
            "--keep",
            "30d",
        ])
        .assert()
        .success()
        .code(0)
        .get_output()
        .stdout
        .clone();

    let json: Value = serde_json::from_slice(&output).expect("parse JSON output");
    assert_eq!(json["dry_run"], false);
    assert!(json["event_count"].as_u64().unwrap() >= 1);
    assert!(json["freed_bytes"].as_u64().unwrap() > 0);

    // File should be deleted
    assert!(!file.exists(), "file should be deleted after prune");
}

#[test]
fn test_telemetry_prune_keep_everything() {
    let dir = tempdir().expect("tempdir");
    let root = dir.path();

    let file = root.join("proc_samples/year=2025/month=01/day=01/host_id=test/old.parquet");
    create_fake_parquet(&file, 1024, 45).expect("create fixture");

    let output = pt_core()
        .args([
            "--format",
            "json",
            "telemetry",
            "--telemetry-dir",
            root.to_str().unwrap(),
            "prune",
            "--keep-everything",
        ])
        .assert()
        .success()
        .code(0)
        .get_output()
        .stdout
        .clone();

    let json: Value = serde_json::from_slice(&output).expect("parse JSON output");
    assert_eq!(
        json["event_count"], 0,
        "keep_everything should prune nothing"
    );

    assert!(
        file.exists(),
        "file should be preserved with keep_everything"
    );
}

// ============================================================================
// Failure Path Exit Codes
// ============================================================================

#[test]
fn test_telemetry_prune_invalid_keep_value_exit_code() {
    let dir = tempdir().expect("tempdir");

    // "invalid" is not a valid duration
    pt_core()
        .args([
            "telemetry",
            "--telemetry-dir",
            dir.path().to_str().unwrap(),
            "prune",
            "--keep",
            "invalid",
        ])
        .assert()
        .failure()
        .code(10) // ArgsError
        .stderr(predicate::str::contains("invalid keep value"));
}

#[test]
fn test_telemetry_prune_zero_keep_value_exit_code() {
    let dir = tempdir().expect("tempdir");

    // "0d" should fail (must be at least 1 day)
    pt_core()
        .args([
            "telemetry",
            "--telemetry-dir",
            dir.path().to_str().unwrap(),
            "prune",
            "--keep",
            "0d",
        ])
        .assert()
        .failure()
        .code(10) // ArgsError
        .stderr(predicate::str::contains("keep must be at least 1 day"));
}

#[test]
fn test_telemetry_prune_invalid_retention_config_exit_code() {
    let dir = tempdir().expect("tempdir");
    let config_dir = dir.path().join("config");
    fs::create_dir_all(&config_dir).expect("create config dir");

    // Write invalid JSON config
    let config_path = write_retention_config(&config_dir, "{ this is not valid json }}}");

    pt_core()
        .args([
            "telemetry",
            "--telemetry-dir",
            dir.path().to_str().unwrap(),
            "--retention-config",
            config_path.to_str().unwrap(),
            "prune",
            "--keep",
            "30d",
        ])
        .assert()
        .failure()
        .code(21) // IoError (JSON parse failure)
        .stderr(predicate::str::contains("telemetry prune"));
}

#[test]
fn test_telemetry_status_invalid_retention_config_exit_code() {
    let dir = tempdir().expect("tempdir");
    let config_dir = dir.path().join("config");
    fs::create_dir_all(&config_dir).expect("create config dir");

    let config_path = write_retention_config(&config_dir, "not json at all");

    pt_core()
        .args([
            "telemetry",
            "--telemetry-dir",
            dir.path().to_str().unwrap(),
            "--retention-config",
            config_path.to_str().unwrap(),
            "status",
        ])
        .assert()
        .failure()
        .code(21) // IoError
        .stderr(predicate::str::contains("telemetry status"));
}

// ============================================================================
// Status → Prune → Status Workflow via CLI
// ============================================================================

#[test]
fn test_telemetry_status_prune_status_workflow_via_cli() {
    let dir = tempdir().expect("tempdir");
    let root = dir.path();

    // Create fixture: 2 old files (will be pruned) + 1 young (kept)
    let old1 = root.join("proc_samples/year=2025/month=01/day=01/host_id=test/old1.parquet");
    let old2 = root.join("proc_features/year=2025/month=01/day=01/host_id=test/old2.parquet");
    let young = root.join("runs/year=2025/month=01/day=15/host_id=test/young.parquet");

    create_fake_parquet(&old1, 2048, 45).expect("old1");
    create_fake_parquet(&old2, 4096, 40).expect("old2");
    create_fake_parquet(&young, 1024, 5).expect("young");

    let dir_str = root.to_str().unwrap();

    // Step 1: Status before prune
    let output_before = pt_core()
        .args([
            "--format",
            "json",
            "telemetry",
            "--telemetry-dir",
            dir_str,
            "status",
        ])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let json_before: Value = serde_json::from_slice(&output_before).expect("parse");
    let files_before = json_before["status"]["total_files"]
        .as_u64()
        .expect("total_files");
    assert_eq!(files_before, 3, "should have 3 files before prune");

    // Step 2: Prune
    let prune_output = pt_core()
        .args([
            "--format",
            "json",
            "telemetry",
            "--telemetry-dir",
            dir_str,
            "prune",
            "--keep",
            "30d",
        ])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let prune_json: Value = serde_json::from_slice(&prune_output).expect("parse");
    let pruned_count = prune_json["event_count"].as_u64().expect("event_count");
    assert_eq!(pruned_count, 2, "should prune 2 old files");

    // Step 3: Status after prune
    let output_after = pt_core()
        .args([
            "--format",
            "json",
            "telemetry",
            "--telemetry-dir",
            dir_str,
            "status",
        ])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let json_after: Value = serde_json::from_slice(&output_after).expect("parse");
    let files_after = json_after["status"]["total_files"]
        .as_u64()
        .expect("total_files");
    assert_eq!(
        files_after,
        files_before - pruned_count,
        "files after should be reduced by pruned count"
    );
    assert_eq!(files_after, 1, "should have 1 file remaining (young run)");

    eprintln!(
        "[INFO] CLI workflow: {} before → {} pruned → {} after",
        files_before, pruned_count, files_after
    );
}

// ============================================================================
// Retention Config File Integration
// ============================================================================

#[test]
fn test_telemetry_prune_with_custom_config() {
    let dir = tempdir().expect("tempdir");
    let root = dir.path();

    // Create a file that's 50 days old
    let file = root.join("proc_samples/year=2025/month=01/day=01/host_id=test/data.parquet");
    create_fake_parquet(&file, 1024, 50).expect("create fixture");

    // Config with 60-day TTL for proc_samples (file should NOT be pruned)
    let config_json = r#"{
        "telemetry_retention": {
            "proc_samples_days": 60
        }
    }"#;
    let config_path = write_retention_config(root, config_json);

    let output = pt_core()
        .args([
            "--format",
            "json",
            "telemetry",
            "--telemetry-dir",
            root.to_str().unwrap(),
            "--retention-config",
            config_path.to_str().unwrap(),
            "prune",
            "--keep",
            "60d",
        ])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let json: Value = serde_json::from_slice(&output).expect("parse JSON");
    assert_eq!(
        json["event_count"], 0,
        "50-day file should not be pruned with 60-day TTL"
    );
    assert!(file.exists(), "file should be preserved");
}

#[test]
fn test_telemetry_prune_with_aggressive_config() {
    let dir = tempdir().expect("tempdir");
    let root = dir.path();

    // Create a file that's 10 days old
    let file = root.join("proc_samples/year=2025/month=01/day=01/host_id=test/data.parquet");
    create_fake_parquet(&file, 1024, 10).expect("create fixture");

    // Config with 5-day TTL (file should be pruned)
    let config_json = r#"{
        "telemetry_retention": {
            "proc_samples_days": 5
        }
    }"#;
    let config_path = write_retention_config(root, config_json);

    let output = pt_core()
        .args([
            "--format",
            "json",
            "telemetry",
            "--telemetry-dir",
            root.to_str().unwrap(),
            "--retention-config",
            config_path.to_str().unwrap(),
            "prune",
            "--keep",
            "5d",
        ])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let json: Value = serde_json::from_slice(&output).expect("parse JSON");
    assert!(
        json["event_count"].as_u64().unwrap() >= 1,
        "10-day file should be pruned with 5-day TTL"
    );
    assert!(!file.exists(), "file should be deleted");
}
