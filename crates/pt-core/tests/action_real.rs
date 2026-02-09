#![cfg(feature = "test-utils")]

use pt_common::{IdentityQuality, ProcessId, ProcessIdentity, StartId};
use pt_core::action::executor::ActionRunner;
use pt_core::action::SignalActionRunner;
use pt_core::decision::Action as PlanActionType;
use pt_core::plan::{ActionConfidence, ActionRationale, ActionRouting, ActionTimeouts, PlanAction};
use pt_core::test_utils::ProcessHarness;
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

#[test]
fn test_signal_kill_real() {
    if !ProcessHarness::is_available() {
        return;
    }
    let harness = ProcessHarness;
    // Use a long sleep so we have time to kill it
    let proc = harness.spawn_sleep(60).expect("spawn");
    let pid = proc.pid();

    // Create a SignalActionRunner
    let runner = SignalActionRunner::with_defaults();

    // Create a kill action plan
    let action = PlanAction {
        action_id: "test-kill".to_string(),
        action: PlanActionType::Kill,
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

    // Execute kill
    let result = runner.execute(&action);
    assert!(result.is_ok(), "kill failed: {:?}", result);

    // Verify process is gone
    std::thread::sleep(Duration::from_millis(100));

    // Check if running using signal 0
    // If it's a zombie, kill(0) returns success (0).
    // So we should check if verify() succeeds.

    let verify = runner.verify(&action);
    assert!(
        verify.is_ok(),
        "Verify failed: process still alive/running? {:?}",
        verify
    );
}

#[test]
fn test_signal_pause_resume_real() {
    if !ProcessHarness::is_available() {
        return;
    }
    let harness = ProcessHarness;
    let proc = harness.spawn_sleep(60).expect("spawn");
    let pid = proc.pid();

    let runner = SignalActionRunner::with_defaults();

    let pause_action = PlanAction {
        action_id: "test-pause".to_string(),
        action: PlanActionType::Pause,
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

    // Pause
    let result = runner.execute(&pause_action);
    assert!(result.is_ok(), "pause failed: {:?}", result);

    // Check state (Linux only)
    #[cfg(target_os = "linux")]
    {
        std::thread::sleep(Duration::from_millis(100));
        let stat = std::fs::read_to_string(format!("/proc/{}/stat", pid)).unwrap();
        // State is 3rd field. T = stopped.
        assert!(
            stat.contains(") T ") || stat.contains(") t "),
            "Process should be stopped (T): {}",
            stat
        );
    }

    // Resume
    let result = runner.resume(pid, false, None);
    assert!(result.is_ok(), "resume failed: {:?}", result);

    #[cfg(target_os = "linux")]
    {
        std::thread::sleep(Duration::from_millis(50));
        let stat = std::fs::read_to_string(format!("/proc/{}/stat", pid)).unwrap();
        assert!(
            stat.contains(") S ") || stat.contains(") R "),
            "Process should be running (S/R): {}",
            stat
        );
    }
}

#[test]
#[cfg(target_os = "linux")]
fn test_process_group_pause_resume_real() {
    use pt_core::action::SignalConfig;
    use pt_core::test_utils::is_process_stopped;

    if !ProcessHarness::is_available() {
        return;
    }
    let harness = ProcessHarness;

    // Spawn a process group (parent + child)
    let proc = harness.spawn_process_group().expect("spawn group");
    let pid = proc.pid();

    // Wait for child to spawn
    std::thread::sleep(Duration::from_millis(200));

    // Get PGID and all PIDs in the group
    let pgid = proc.pgid().expect("should have pgid");
    let group_pids = proc.group_pids();
    assert!(
        group_pids.len() >= 2,
        "expected at least 2 processes in group, got {:?}",
        group_pids
    );

    // Create runner with process group targeting enabled
    let runner = SignalActionRunner::new(SignalConfig {
        use_process_groups: true,
        ..Default::default()
    });

    // Create a pause action targeting the process group
    let pause_action = PlanAction {
        action_id: "test-group-pause".to_string(),
        action: PlanActionType::Pause,
        target: ProcessIdentity {
            pid: ProcessId(pid),
            start_id: StartId("mock".to_string()),
            uid: 1000,
            pgid: Some(pgid),
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

    // Pause the entire group
    let result = runner.execute(&pause_action);
    assert!(result.is_ok(), "group pause failed: {:?}", result);

    // Verify all processes in the group are stopped
    std::thread::sleep(Duration::from_millis(100));
    for gpid in &group_pids {
        assert!(
            is_process_stopped(*gpid),
            "process {} in group should be stopped",
            gpid
        );
    }

    // Create a resume action
    let resume_action = PlanAction {
        action_id: "test-group-resume".to_string(),
        action: PlanActionType::Resume,
        target: ProcessIdentity {
            pid: ProcessId(pid),
            start_id: StartId("mock".to_string()),
            uid: 1000,
            pgid: Some(pgid),
            sid: None,
            quality: IdentityQuality::Full,
        },
        order: 1,
        stage: 1,
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

    // Resume the entire group
    let result = runner.execute(&resume_action);
    assert!(result.is_ok(), "group resume failed: {:?}", result);

    // Verify all processes in the group are running again
    std::thread::sleep(Duration::from_millis(100));
    for gpid in &group_pids {
        assert!(
            !is_process_stopped(*gpid),
            "process {} in group should be running",
            gpid
        );
    }
}

#[test]
fn test_zombie_verification_real() {
    if !ProcessHarness::is_available() {
        return;
    }
    let harness = ProcessHarness;

    // Spawn a process that exits immediately -> Zombie
    let proc = harness.spawn_shell("true").expect("spawn true");
    let pid = proc.pid();

    // Wait until it becomes zombie
    let mut is_zombie = false;
    for _ in 0..20 {
        if let Ok(stat) = std::fs::read_to_string(format!("/proc/{}/stat", pid)) {
            if stat.contains(") Z ") {
                is_zombie = true;
                break;
            }
        }
        std::thread::sleep(Duration::from_millis(50));
    }
    assert!(is_zombie, "process {} never became zombie", pid);

    let runner = SignalActionRunner::with_defaults();
    let action = PlanAction {
        action_id: "test-kill-zombie".to_string(),
        action: PlanActionType::Kill,
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

    // Execute kill on zombie should succeed (no-op or ignored signal)
    let result = runner.execute(&action);
    assert!(result.is_ok(), "Kill on zombie failed: {:?}", result);

    // Verify should succeed (Z count as dead)
    let result = runner.verify(&action);
    assert!(result.is_ok(), "Verify on zombie failed: {:?}", result);
}
