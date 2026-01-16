//! Live-system integration tests for ToolRunner.
//!
//! These tests use real commands and avoid mocks/fakes. They are gated by
//! command availability so they can skip safely on minimal environments.

use pt_core::collect::{ToolError, ToolRunnerBuilder};
use std::path::Path;
use std::time::Duration;

fn command_exists(cmd: &str) -> bool {
    if cmd.contains('/') {
        return Path::new(cmd).exists();
    }

    let Ok(path) = std::env::var("PATH") else {
        return false;
    };

    for dir in path.split(':') {
        if dir.is_empty() {
            continue;
        }
        let candidate = Path::new(dir).join(cmd);
        if candidate.exists() {
            return true;
        }
    }

    false
}

#[test]
fn live_tool_runner_true() {
    if !command_exists("true") {
        eprintln!("skipping: true not found in PATH");
        return;
    }

    let runner = ToolRunnerBuilder::new().use_nice(false).build();
    let output = runner.run_tool("true", &[], None).expect("run true");
    assert!(output.success());
    assert!(!output.timed_out);
}

#[test]
fn live_tool_runner_false() {
    if !command_exists("false") {
        eprintln!("skipping: false not found in PATH");
        return;
    }

    let runner = ToolRunnerBuilder::new().use_nice(false).build();
    let output = runner.run_tool("false", &[], None).expect("run false");
    assert!(!output.success());
    assert_eq!(output.exit_code, Some(1));
}

#[test]
fn live_tool_runner_timeout() {
    if !command_exists("sleep") {
        eprintln!("skipping: sleep not found in PATH");
        return;
    }

    let runner = ToolRunnerBuilder::new()
        .use_nice(false)
        .timeout(Duration::from_millis(100))
        .build();

    let output = runner.run_tool("sleep", &["5"], None).expect("run sleep");
    assert!(output.timed_out);
}

#[test]
#[cfg(unix)]
fn live_tool_runner_output_truncation() {
    if !command_exists("head") {
        eprintln!("skipping: head not found in PATH");
        return;
    }
    if !Path::new("/dev/zero").exists() {
        eprintln!("skipping: /dev/zero not available");
        return;
    }

    let runner = ToolRunnerBuilder::new()
        .use_nice(false)
        .max_output(128)
        .build();

    let result = runner.run_tool("head", &["-c", "1024", "/dev/zero"], None);
    match result {
        Ok(output) => {
            assert!(output.truncated, "expected truncation");
            assert!(output.stdout.len() <= 128);
        }
        Err(ToolError::CommandNotFound(_)) => {
            eprintln!("skipping: head not executable");
        }
        Err(err) => panic!("unexpected error: {err:?}"),
    }
}
