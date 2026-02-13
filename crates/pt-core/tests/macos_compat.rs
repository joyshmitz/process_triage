use pt_common::IdentityQuality;
use pt_core::action::{SupervisorAction, SupervisorInfo};

#[test]
fn identity_quality_variants_exhaustive() {
    let variants = [
        IdentityQuality::Full,
        IdentityQuality::NoBootId,
        IdentityQuality::PidOnly,
    ];

    let labels: Vec<&'static str> = variants
        .iter()
        .map(|variant| match variant {
            IdentityQuality::Full => "full",
            IdentityQuality::NoBootId => "no_boot_id",
            IdentityQuality::PidOnly => "pid_only",
        })
        .collect();

    assert_eq!(labels, vec!["full", "no_boot_id", "pid_only"]);
}

#[test]
fn supervisor_info_construction_without_systemd() {
    let info = SupervisorInfo {
        supervisor: "supervisord".to_string(),
        unit_name: None,
        unit_type: None,
        is_main_process: false,
        recommended_action: SupervisorAction::KillProcess,
        systemd_unit: None,
    };

    assert_eq!(info.supervisor, "supervisord");
    assert!(info.unit_name.is_none());
    assert!(info.unit_type.is_none());
    assert!(!info.is_main_process);
    assert!(info.systemd_unit.is_none());
    assert!(matches!(
        info.recommended_action,
        SupervisorAction::KillProcess
    ));
}

#[cfg(target_os = "linux")]
#[test]
fn supervisor_info_with_systemd() {
    use pt_core::collect::systemd::{
        SystemdActiveState, SystemdDataSource, SystemdProvenance, SystemdUnit, SystemdUnitType,
    };

    let unit = SystemdUnit {
        name: "test.service".to_string(),
        unit_type: SystemdUnitType::Service,
        active_state: SystemdActiveState::Active,
        sub_state: Some("running".to_string()),
        main_pid: Some(4242),
        control_pid: None,
        fragment_path: None,
        description: Some("test unit".to_string()),
        is_main_process: true,
        provenance: SystemdProvenance {
            source: SystemdDataSource::SystemctlShow,
            warnings: vec![],
        },
    };

    let info = SupervisorInfo {
        supervisor: "systemd".to_string(),
        unit_name: Some(unit.name.clone()),
        unit_type: Some(SystemdUnitType::Service),
        is_main_process: true,
        recommended_action: SupervisorAction::RestartUnit {
            command: "systemctl restart test.service".to_string(),
        },
        systemd_unit: Some(unit),
    };

    assert_eq!(info.supervisor, "systemd");
    assert_eq!(info.unit_name.as_deref(), Some("test.service"));
    assert_eq!(info.unit_type, Some(SystemdUnitType::Service));
    assert!(info.is_main_process);
    assert!(matches!(
        info.recommended_action,
        SupervisorAction::RestartUnit { .. }
    ));
    assert!(info.systemd_unit.is_some());
}
