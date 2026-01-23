use pt_core::collect::tool_runner::{ToolConfig, ToolRunner};
use std::time::{Duration, Instant};

#[test]
fn test_tool_runner_grandchild_hang() {
    let runner = ToolRunner::new(ToolConfig {
        default_timeout: Duration::from_secs(2), // We want it to timeout quickly if it hangs
        budget_ms: 10000,
        ..Default::default()
    });

    // Spawn a grandchild that sleeps for 5s and holds stdout open, while parent exits immediately.
    // In sh:
    // (sleep 5; echo "done") &
    // exit 0
    //
    // If drain_to_limit blocks waiting for EOF, this run_tool will take 5s, exceeding our 2s timeout expectation.
    // Note: We don't expect ToolError::Timeout (which kills the child), because the child exits immediately.
    // We expect run_tool to return quickly with the child's exit code.

    let start = Instant::now();
    let result = runner.run_tool(
        "sh",
        &["-c", "(sleep 5; echo 'alive' >&1) & exit 0"],
        Some(Duration::from_secs(1)),
    );
    let duration = start.elapsed();

    println!("Execution took {:?}", duration);

    // If it took >= 5s, we hung.
    assert!(
        duration < Duration::from_secs(4),
        "ToolRunner hung waiting for grandchild pipe!"
    );

    // Result should be ok (child exited 0)
    assert!(result.is_ok());
}
