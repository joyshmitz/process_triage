//! Integration tests for cgroup CPU throttle action.
//!
//! These tests require:
//! - Linux with cgroup v2 support
//! - Write access to cgroup hierarchy (root or appropriate permissions)
//!
//! Tests are automatically skipped if requirements are not met.

#![cfg(all(feature = "test-utils", target_os = "linux"))]

use pt_common::{IdentityQuality, ProcessId, ProcessIdentity, StartId};
use pt_core::action::executor::ActionRunner;
use pt_core::action::{
    can_throttle_process, CpuThrottleActionRunner, CpuThrottleConfig, DEFAULT_PERIOD_US,
    DEFAULT_THROTTLE_FRACTION, MIN_QUOTA_US,
};
use pt_core::collect::cgroup::{collect_cgroup_details, CgroupVersion};
use pt_core::decision::Action as PlanActionType;
use pt_core::plan::{ActionConfidence, ActionRationale, ActionRouting, ActionTimeouts, PlanAction};
use pt_core::test_utils::ProcessHarness;
use std::fs;
use std::time::Duration;

fn empty_rationale() -> ActionRationale {
    ActionRationale {
        expected_loss: None,
        expected_recovery: None,
        expected_recovery_stddev: None,
        posterior_odds_abandoned_vs_useful: None,
        sprt_boundary: None,
        posterior: None,
        memory_mb: None,
        has_known_signature: None,
        category: None,
    }
}

fn has_cgroup_v2_write_access() -> bool {
    if let Ok(cgroup) = fs::read_to_string("/proc/self/cgroup") {
        for line in cgroup.lines() {
            if let Some(path) = line.strip_prefix("0::") {
                let cpu_max_path = format!("/sys/fs/cgroup{}/cpu.max", path);
                if let Ok(metadata) = fs::metadata(&cpu_max_path) {
                    return !metadata.permissions().readonly();
                }
            }
        }
    }
    false
}

// ============================================================================
// Configuration and calculation tests (no cgroup access needed)
// ============================================================================

#[test]
fn test_config_defaults() {
    let config = CpuThrottleConfig::default();
    assert_eq!(config.target_fraction, DEFAULT_THROTTLE_FRACTION);
    assert_eq!(config.period_us, DEFAULT_PERIOD_US);
    assert!(config.fallback_to_v1);
    assert!(config.capture_reversal);
}

#[test]
fn test_config_with_fraction() {
    let config = CpuThrottleConfig::with_fraction(0.5);
    assert_eq!(config.target_fraction, 0.5);
    assert_eq!(config.period_us, DEFAULT_PERIOD_US);
}

#[test]
fn test_quota_calculation_standard() {
    let config = CpuThrottleConfig {
        target_fraction: 0.25,
        period_us: 100_000,
        ..Default::default()
    };
    // 25% of 100ms = 25ms = 25000us
    assert_eq!(config.quota_us(), 25_000);
}

#[test]
fn test_quota_calculation_multiple_cores() {
    let config = CpuThrottleConfig {
        target_fraction: 2.0, // 2 cores
        period_us: 100_000,
        ..Default::default()
    };
    // 200% of 100ms = 200ms = 200000us
    assert_eq!(config.quota_us(), 200_000);
}

#[test]
fn test_quota_minimum_enforced() {
    let config = CpuThrottleConfig {
        target_fraction: 0.000001, // Very small
        period_us: 100_000,
        ..Default::default()
    };
    // Should be clamped to MIN_QUOTA_US
    assert_eq!(config.quota_us(), MIN_QUOTA_US);
}

#[test]
fn test_runner_creation() {
    let runner = CpuThrottleActionRunner::with_defaults();
    // Just verify it can be created
    let _ = runner;
}

// ============================================================================
// Cgroup detection tests (read-only, no write access needed)
// ============================================================================

#[test]
fn test_can_throttle_self() {
    let my_pid = std::process::id();
    let result = can_throttle_process(my_pid);
    // Just verify the function works - result depends on system configuration
    pt_core::test_log!(
        INFO,
        "can_throttle_process check",
        pid = my_pid,
        can_throttle = result
    );
}

#[test]
fn test_collect_cgroup_details_self() {
    let my_pid = std::process::id();
    let details = collect_cgroup_details(my_pid);
    assert!(details.is_some(), "Should be able to read cgroup for self");

    let details = details.unwrap();
    pt_core::test_log!(
        INFO,
        "cgroup details",
        pid = my_pid,
        version = format!("{:?}", details.version).as_str(),
        unified_path = details.unified_path.as_deref().unwrap_or("none")
    );
}

#[test]
fn test_reversal_metadata_capture() {
    let runner = CpuThrottleActionRunner::with_defaults();
    let my_pid = std::process::id();

    let metadata = runner.capture_reversal_metadata(my_pid);
    pt_core::test_log!(
        INFO,
        "reversal metadata capture",
        pid = my_pid,
        has_metadata = metadata.is_some()
    );

    if let Some(meta) = metadata {
        assert_eq!(meta.pid, my_pid);
        assert!(!meta.cgroup_path.is_empty());
        pt_core::test_log!(
            INFO,
            "reversal metadata",
            cgroup_path = meta.cgroup_path.as_str(),
            source = format!("{:?}", meta.source).as_str(),
            previous_quota = format!("{:?}", meta.previous_quota_us).as_str()
        );
    }
}

// ============================================================================
// Live throttle tests (require cgroup write access)
// ============================================================================

#[test]
fn test_throttle_spawned_process() {
    if !ProcessHarness::is_available() {
        pt_core::test_log!(INFO, "Skipping: ProcessHarness not available");
        return;
    }

    // First check if we have cgroup access for our own process
    // (spawned processes inherit our cgroup)
    if !has_cgroup_v2_write_access() {
        pt_core::test_log!(INFO, "Skipping: no cgroup v2 write access");
        return;
    }

    let harness = ProcessHarness::default();
    let proc = harness.spawn_sleep(60).expect("spawn sleep process");
    let pid = proc.pid();

    pt_core::test_log!(INFO, "throttle test starting", pid = pid);

    // Check if we can throttle this process
    if !can_throttle_process(pid) {
        pt_core::test_log!(INFO, "Skipping: cannot throttle spawned process", pid = pid);
        return;
    }

    // Capture original state
    let runner = CpuThrottleActionRunner::with_defaults();
    let reversal = runner.capture_reversal_metadata(pid);
    pt_core::test_log!(
        INFO,
        "captured reversal metadata",
        pid = pid,
        has_reversal = reversal.is_some()
    );

    // Create throttle action
    let action = PlanAction {
        action_id: "test-throttle".to_string(),
        action: PlanActionType::Throttle,
        target: ProcessIdentity {
            pid: ProcessId(pid),
            start_id: StartId("mock".to_string()),
            uid: 1000,
            pgid: None,
            sid: None,
            quality: IdentityQuality::Full,
        },
        order: 0,
        stage: 0,
        timeouts: ActionTimeouts::default(),
        pre_checks: vec![],
        rationale: empty_rationale(),
        on_success: vec![],
        on_failure: vec![],
        blocked: false,
        routing: ActionRouting::Direct,
        confidence: ActionConfidence::Normal,
        original_zombie_target: None,
        d_state_diagnostics: None,
    };

    // Execute throttle
    let result = runner.execute(&action);
    if let Err(ref e) = result {
        pt_core::test_log!(
            INFO,
            "throttle execution result",
            pid = pid,
            error = format!("{:?}", e).as_str()
        );
        // Permission errors and cgroup access failures are expected in non-privileged environments
        let error_msg = format!("{:?}", e);
        if matches!(e, pt_core::action::ActionError::PermissionDenied)
            || error_msg.contains("no writable cgroup")
            || error_msg.contains("permission")
        {
            pt_core::test_log!(INFO, "Skipping verification: cgroup write access unavailable");
            return;
        }
    }
    assert!(result.is_ok(), "throttle failed: {:?}", result);
    pt_core::test_log!(INFO, "throttle executed successfully", pid = pid);

    // Verify throttle was applied
    std::thread::sleep(Duration::from_millis(50));
    let verify = runner.verify(&action);
    assert!(verify.is_ok(), "throttle verification failed: {:?}", verify);
    pt_core::test_log!(INFO, "throttle verified successfully", pid = pid);

    // Verify CPU limits were changed
    let details = collect_cgroup_details(pid).expect("collect cgroup details");
    if let Some(ref limits) = details.cpu_limits {
        pt_core::test_log!(
            INFO,
            "post-throttle CPU limits",
            pid = pid,
            quota_us = format!("{:?}", limits.quota_us).as_str(),
            period_us = format!("{:?}", limits.period_us).as_str(),
            effective_cores = format!("{:?}", limits.effective_cores).as_str()
        );
    }

    // Test reversal
    if let Some(ref metadata) = reversal {
        let restore_result = runner.restore_from_metadata(metadata);
        assert!(
            restore_result.is_ok(),
            "reversal failed: {:?}",
            restore_result
        );
        pt_core::test_log!(INFO, "reversal successful", pid = pid);
    }
}

#[test]
fn test_throttle_permission_denied() {
    if !ProcessHarness::is_available() {
        pt_core::test_log!(INFO, "Skipping: ProcessHarness not available");
        return;
    }

    // Try to throttle PID 1 (init) which should fail with permission denied
    let init_pid = 1u32;

    let runner = CpuThrottleActionRunner::with_defaults();

    let action = PlanAction {
        action_id: "test-throttle-init".to_string(),
        action: PlanActionType::Throttle,
        target: ProcessIdentity {
            pid: ProcessId(init_pid),
            start_id: StartId("mock".to_string()),
            uid: 0,
            pgid: None,
            sid: None,
            quality: IdentityQuality::Full,
        },
        order: 0,
        stage: 0,
        timeouts: ActionTimeouts::default(),
        pre_checks: vec![],
        rationale: empty_rationale(),
        on_success: vec![],
        on_failure: vec![],
        blocked: false,
        routing: ActionRouting::Direct,
        confidence: ActionConfidence::Normal,
        original_zombie_target: None,
        d_state_diagnostics: None,
    };

    // This should fail (either permission denied or protected)
    let result = runner.execute(&action);
    pt_core::test_log!(
        INFO,
        "throttle init result",
        error = format!("{:?}", result).as_str()
    );

    // We expect this to fail - either permission denied or process not found
    assert!(result.is_err(), "throttling init should fail");
}

#[test]
fn test_throttle_nonexistent_process() {
    let runner = CpuThrottleActionRunner::with_defaults();

    let action = PlanAction {
        action_id: "test-throttle-nonexistent".to_string(),
        action: PlanActionType::Throttle,
        target: ProcessIdentity {
            pid: ProcessId(999_999_999),
            start_id: StartId("mock".to_string()),
            uid: 1000,
            pgid: None,
            sid: None,
            quality: IdentityQuality::Full,
        },
        order: 0,
        stage: 0,
        timeouts: ActionTimeouts::default(),
        pre_checks: vec![],
        rationale: empty_rationale(),
        on_success: vec![],
        on_failure: vec![],
        blocked: false,
        routing: ActionRouting::Direct,
        confidence: ActionConfidence::Normal,
        original_zombie_target: None,
        d_state_diagnostics: None,
    };

    let result = runner.execute(&action);
    assert!(
        result.is_err(),
        "throttling nonexistent process should fail"
    );
    pt_core::test_log!(
        INFO,
        "throttle nonexistent result",
        error = format!("{:?}", result).as_str()
    );
}

#[test]
fn test_throttle_wrong_action_type() {
    let runner = CpuThrottleActionRunner::with_defaults();

    // Try to use throttle runner for a Kill action
    let action = PlanAction {
        action_id: "test-wrong-action".to_string(),
        action: PlanActionType::Kill, // Wrong action type
        target: ProcessIdentity {
            pid: ProcessId(std::process::id()),
            start_id: StartId("mock".to_string()),
            uid: 1000,
            pgid: None,
            sid: None,
            quality: IdentityQuality::Full,
        },
        order: 0,
        stage: 0,
        timeouts: ActionTimeouts::default(),
        pre_checks: vec![],
        rationale: empty_rationale(),
        on_success: vec![],
        on_failure: vec![],
        blocked: false,
        routing: ActionRouting::Direct,
        confidence: ActionConfidence::Normal,
        original_zombie_target: None,
        d_state_diagnostics: None,
    };

    let result = runner.execute(&action);
    assert!(
        result.is_err(),
        "kill action on throttle runner should fail"
    );
}

#[test]
fn test_throttle_keep_action_noop() {
    let runner = CpuThrottleActionRunner::with_defaults();

    // Keep action should be a no-op
    let action = PlanAction {
        action_id: "test-keep".to_string(),
        action: PlanActionType::Keep,
        target: ProcessIdentity {
            pid: ProcessId(std::process::id()),
            start_id: StartId("mock".to_string()),
            uid: 1000,
            pgid: None,
            sid: None,
            quality: IdentityQuality::Full,
        },
        order: 0,
        stage: 0,
        timeouts: ActionTimeouts::default(),
        pre_checks: vec![],
        rationale: empty_rationale(),
        on_success: vec![],
        on_failure: vec![],
        blocked: false,
        routing: ActionRouting::Direct,
        confidence: ActionConfidence::Normal,
        original_zombie_target: None,
        d_state_diagnostics: None,
    };

    let result = runner.execute(&action);
    assert!(result.is_ok(), "Keep action should succeed as no-op");
}

// ============================================================================
// Cgroup version detection tests
// ============================================================================

#[test]
fn test_cgroup_version_detection() {
    let my_pid = std::process::id();
    let details = collect_cgroup_details(my_pid);

    if let Some(details) = details {
        pt_core::test_log!(
            INFO,
            "cgroup version",
            version = format!("{:?}", details.version).as_str()
        );

        match details.version {
            CgroupVersion::V2 => {
                assert!(
                    details.unified_path.is_some(),
                    "v2 should have unified path"
                );
            }
            CgroupVersion::V1 => {
                assert!(
                    !details.v1_paths.is_empty(),
                    "v1 should have controller paths"
                );
            }
            CgroupVersion::Hybrid => {
                assert!(
                    details.unified_path.is_some() || !details.v1_paths.is_empty(),
                    "hybrid should have some paths"
                );
            }
            CgroupVersion::Unknown => {
                // This is valid on some systems
            }
        }
    }
}
