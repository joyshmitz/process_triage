//! E2E scenarios for specific CLI workflows.
//!
//! Covers:
//! - Basic scan with specific process verification
//! - Invalid configuration handling
//!
//! These tests rely on `ProcessHarness` to interact with real system state (no mocks).

#[cfg(feature = "test-utils")]
mod e2e_scenarios {
    use assert_cmd::cargo::cargo_bin_cmd;
    use assert_cmd::Command;
    use predicates::prelude::*;
    use pt_core::test_utils::ProcessHarness;
    use serde_json::Value;
    use std::fs::File;
    use std::io::Write;
    use std::time::Duration;
    use tempfile::tempdir;

    /// Get a Command for pt-core binary.
    fn pt_core() -> Command {
        let mut cmd = cargo_bin_cmd!("pt-core");
        cmd.timeout(Duration::from_secs(30));
        cmd
    }

    #[test]
    fn test_basic_scan_finds_specific_process() -> Result<(), Box<dyn std::error::Error>> {
        if !ProcessHarness::is_available() {
            eprintln!("Skipping test_basic_scan_finds_specific_process: ProcessHarness not available");
            return Ok(());
        }

        let harness = ProcessHarness::default();
        // Spawn a unique sleeper so we can identify it
        let sleeper = harness
            .spawn_sleep(100)?;
        let pid = sleeper.pid();

        // Run scan
        let output = pt_core()
            .args(["--format", "json", "scan"])
            .assert()
            .success()
            .get_output()
            .stdout
            .clone();

        let json: Value = serde_json::from_slice(&output)?;
        let processes = json
            .get("scan")
            .and_then(|s| s.get("processes"))
            .and_then(|p| p.as_array())
            .ok_or("Should have scan.processes array")?;

        // Find our sleeper
        let found = processes.iter().find(|p| {
            p.get("pid")
                .and_then(|v| v.as_u64())
                .map(|v| v as u32)
                == Some(pid)
        });

        if found.is_none() {
            return Err(format!("Scan output should contain our spawned sleeper process (PID {})", pid).into());
        }

        let proc = found.unwrap();
        // Verify some fields
        let comm = proc.get("comm").and_then(|s| s.as_str()).unwrap_or("");
        if !comm.contains("sleep") && !comm.contains("sh") {
             return Err(format!("Command should be sleep or sh (got: {})", comm).into());
        }
        
        Ok(())
    }

    #[test]
    fn test_invalid_config_file_returns_error() -> Result<(), Box<dyn std::error::Error>> {
        let dir = tempdir()?;
        let config_path = dir.path().join("policy.json");
        
        // Write invalid JSON
        let mut file = File::create(&config_path)?;
        writeln!(file, "{{ invalid_json_here ")?;

        // Run pt check --config <dir>
        // Note: pt expects a config directory, not file path usually, 
        // or --config <dir> which contains policy.json.
        // We pass the directory.
        
        let output = pt_core()
            .arg("--config")
            .arg(dir.path())
            .args(["check", "--policy"])
            .assert()
            .failure(); // Should fail due to parse error

        output.stdout(predicate::str::contains("Invalid JSON"));
        Ok(())
    }
    
    #[test]
    fn test_valid_config_loading() -> Result<(), Box<dyn std::error::Error>> {
        let dir = tempdir()?;
        let config_path = dir.path().join("policy.json");
        
        // Write valid minimal policy JSON
        let mut file = File::create(&config_path)?;
        // Schema version must be a string
        // guardrails needs protected_patterns, never_kill_ppid, max_kills_per_run, min_process_age_seconds
        // loss_matrix needs false_positive, false_negative, useful, useful_bad, abandoned, zombie
        // robot_mode needs enabled, max_concurrent_kills, require_human_for_supervised (guessing), min_posterior, max_blast_radius_mb, max_kills, require_known_signature
        // fdr_control needs target_fdr, enabled, method, alpha
        // data_loss_gates needs block_if_open_write_fds, block_if_locked_files, block_if_active_tty
        // loss values must be non-negative
        // guardrails.never_kill_ppid must contain at least 1
        writeln!(file, r#"{{ 
            "schema_version": "1.0.0", 
            "guardrails": {{ 
                "protected_patterns": [], 
                "never_kill_ppid": [1], 
                "max_kills_per_run": 5, 
                "min_process_age_seconds": 60 
            }}, 
            "loss_matrix": {{ 
                "false_positive": 10.0, 
                "false_negative": 1.0, 
                "useful": {{ "kill": 100.0, "pause": 10.0, "renice": 1.0, "ignore": 0.0, "keep": 0.0 }}, 
                "useful_bad": {{ "kill": 10.0, "pause": 0.0, "renice": 5.0, "ignore": 5.0, "keep": 5.0 }}, 
                "abandoned": {{ "kill": 0.0, "pause": 5.0, "renice": 0.0, "ignore": 10.0, "keep": 5.0 }}, 
                "zombie": {{ "kill": 5.0, "pause": 5.0, "renice": 0.0, "ignore": 0.0, "keep": 0.0 }} 
            }},
            "robot_mode": {{
                "enabled": true,
                "max_kills_per_run": 5,
                "require_confirmation": false,
                "min_posterior": 0.9,
                "max_blast_radius_mb": 500,
                "max_kills": 5,
                "require_known_signature": false
            }},
            "fdr_control": {{
                "enabled": true,
                "target_fdr": 0.1,
                "method": "bh",
                "alpha": 0.05
            }},
            "data_loss_gates": {{
                "block_if_open_write_fds": true,
                "block_if_locked_files": true,
                "block_if_active_tty": true
            }}
        }}"#)?;

        // Run pt check --config <dir>
        pt_core()
            .arg("--config")
            .arg(dir.path())
            .args(["check", "--policy"])
            .assert()
            .success();
            
        Ok(())
    }
}
