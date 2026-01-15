//! Systemd unit detection and collection.
//!
//! This module provides systemd unit information for process triage:
//! - Unit name and type detection via `systemctl status <pid>`
//! - Active state and sub-state
//! - Main PID tracking for service restart detection
//!
//! # Data Sources
//! - `systemctl show --property=... <pid>` - structured unit info
//! - Cgroup path parsing (fallback when systemctl unavailable)

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::process::Command;

/// Systemd unit information for a process.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SystemdUnit {
    /// Full unit name (e.g., "nginx.service", "session-1.scope").
    pub name: String,

    /// Unit type (service, scope, slice, socket, etc.).
    pub unit_type: SystemdUnitType,

    /// Current active state.
    pub active_state: SystemdActiveState,

    /// Sub-state (running, exited, dead, etc.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_state: Option<String>,

    /// Main PID of the unit (for services).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub main_pid: Option<u32>,

    /// Control PID (for units with control processes).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_pid: Option<u32>,

    /// Fragment path (unit file location).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fragment_path: Option<String>,

    /// Unit description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Whether this process is the main process of the unit.
    pub is_main_process: bool,

    /// Provenance tracking.
    pub provenance: SystemdProvenance,
}

/// Systemd unit type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SystemdUnitType {
    /// .service units (daemons, services).
    Service,
    /// .scope units (externally started processes).
    Scope,
    /// .slice units (resource management).
    Slice,
    /// .socket units (socket activation).
    Socket,
    /// .target units (synchronization points).
    Target,
    /// .mount units (mount points).
    Mount,
    /// .timer units (timer activation).
    Timer,
    /// .path units (path-based activation).
    Path,
    /// Unknown or not a systemd unit.
    #[default]
    Unknown,
}

impl SystemdUnitType {
    /// Parse unit type from unit name suffix.
    pub fn from_unit_name(name: &str) -> Self {
        if name.ends_with(".service") {
            SystemdUnitType::Service
        } else if name.ends_with(".scope") {
            SystemdUnitType::Scope
        } else if name.ends_with(".slice") {
            SystemdUnitType::Slice
        } else if name.ends_with(".socket") {
            SystemdUnitType::Socket
        } else if name.ends_with(".target") {
            SystemdUnitType::Target
        } else if name.ends_with(".mount") {
            SystemdUnitType::Mount
        } else if name.ends_with(".timer") {
            SystemdUnitType::Timer
        } else if name.ends_with(".path") {
            SystemdUnitType::Path
        } else {
            SystemdUnitType::Unknown
        }
    }
}

/// Systemd active state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SystemdActiveState {
    /// Unit is active and running.
    Active,
    /// Unit is being activated.
    Activating,
    /// Unit is being deactivated.
    Deactivating,
    /// Unit is inactive (stopped).
    Inactive,
    /// Unit has failed.
    Failed,
    /// Unit is being reloaded.
    Reloading,
    /// Unknown state.
    #[default]
    Unknown,
}

impl SystemdActiveState {
    /// Parse active state from string.
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "active" => SystemdActiveState::Active,
            "activating" => SystemdActiveState::Activating,
            "deactivating" => SystemdActiveState::Deactivating,
            "inactive" => SystemdActiveState::Inactive,
            "failed" => SystemdActiveState::Failed,
            "reloading" => SystemdActiveState::Reloading,
            _ => SystemdActiveState::Unknown,
        }
    }

    /// Whether the unit is in a running/active state.
    pub fn is_running(&self) -> bool {
        matches!(self, SystemdActiveState::Active | SystemdActiveState::Reloading)
    }
}

/// Provenance tracking for systemd data.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SystemdProvenance {
    /// Source of the unit info (systemctl, cgroup_path, etc.).
    pub source: SystemdDataSource,

    /// Any warnings during collection.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub warnings: Vec<String>,
}

/// Source of systemd unit information.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SystemdDataSource {
    /// From `systemctl show` command.
    SystemctlShow,
    /// From cgroup path parsing.
    CgroupPath,
    /// Not available.
    #[default]
    None,
}

/// Collect systemd unit information for a process.
///
/// First tries `systemctl show`, falls back to cgroup path parsing.
///
/// # Arguments
/// * `pid` - Process ID
/// * `cgroup_unit` - Unit name from cgroup path (fallback)
///
/// # Returns
/// * `Option<SystemdUnit>` - Unit info or None if not managed by systemd
pub fn collect_systemd_unit(pid: u32, cgroup_unit: Option<&str>) -> Option<SystemdUnit> {
    // Try systemctl show first
    if let Some(unit) = query_systemctl(pid) {
        return Some(unit);
    }

    // Fall back to cgroup path info
    if let Some(unit_name) = cgroup_unit {
        return Some(unit_from_cgroup_path(unit_name, pid));
    }

    None
}

/// Query systemctl for unit information.
fn query_systemctl(pid: u32) -> Option<SystemdUnit> {
    // Properties to query
    let properties = [
        "Id",
        "ActiveState",
        "SubState",
        "MainPID",
        "ControlPID",
        "FragmentPath",
        "Description",
    ];

    let output = Command::new("systemctl")
        .args([
            "show",
            "--property",
            &properties.join(","),
            &pid.to_string(),
        ])
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    parse_systemctl_output(&stdout, pid)
}

/// Parse systemctl show output.
pub fn parse_systemctl_output(output: &str, pid: u32) -> Option<SystemdUnit> {
    let props = parse_properties(output);

    // Get unit name (Id)
    let name = props.get("Id")?.clone();

    // Skip if it's just "-" or empty (not managed by systemd)
    if name.is_empty() || name == "-" {
        return None;
    }

    let unit_type = SystemdUnitType::from_unit_name(&name);

    let active_state = props
        .get("ActiveState")
        .map(|s| SystemdActiveState::from_str(s))
        .unwrap_or_default();

    let sub_state = props.get("SubState").cloned().filter(|s| !s.is_empty() && s != "-");

    let main_pid = props
        .get("MainPID")
        .and_then(|s| s.parse::<u32>().ok())
        .filter(|&p| p > 0);

    let control_pid = props
        .get("ControlPID")
        .and_then(|s| s.parse::<u32>().ok())
        .filter(|&p| p > 0);

    let fragment_path = props
        .get("FragmentPath")
        .cloned()
        .filter(|s| !s.is_empty() && s != "-");

    let description = props
        .get("Description")
        .cloned()
        .filter(|s| !s.is_empty() && s != "-");

    // Check if this PID is the main process
    let is_main_process = main_pid.map(|mp| mp == pid).unwrap_or(false);

    Some(SystemdUnit {
        name,
        unit_type,
        active_state,
        sub_state,
        main_pid,
        control_pid,
        fragment_path,
        description,
        is_main_process,
        provenance: SystemdProvenance {
            source: SystemdDataSource::SystemctlShow,
            warnings: Vec::new(),
        },
    })
}

/// Parse key=value properties from systemctl output.
fn parse_properties(output: &str) -> HashMap<String, String> {
    let mut props = HashMap::new();

    for line in output.lines() {
        if let Some((key, value)) = line.split_once('=') {
            props.insert(key.to_string(), value.to_string());
        }
    }

    props
}

/// Create unit info from cgroup path (fallback).
fn unit_from_cgroup_path(unit_name: &str, _pid: u32) -> SystemdUnit {
    let unit_type = SystemdUnitType::from_unit_name(unit_name);

    SystemdUnit {
        name: unit_name.to_string(),
        unit_type,
        active_state: SystemdActiveState::Unknown,
        sub_state: None,
        main_pid: None,
        control_pid: None,
        fragment_path: None,
        description: None,
        is_main_process: false,
        provenance: SystemdProvenance {
            source: SystemdDataSource::CgroupPath,
            warnings: vec!["Unit info from cgroup path only; systemctl unavailable".to_string()],
        },
    }
}

/// Check if systemd is available on the system.
pub fn is_systemd_available() -> bool {
    // Check if systemctl exists and is functional
    Command::new("systemctl")
        .args(["--version"])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// Check if a process is managed by systemd.
pub fn is_systemd_managed(pid: u32) -> bool {
    Command::new("systemctl")
        .args(["status", &pid.to_string()])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unit_type_from_name() {
        assert_eq!(
            SystemdUnitType::from_unit_name("nginx.service"),
            SystemdUnitType::Service
        );
        assert_eq!(
            SystemdUnitType::from_unit_name("session-1.scope"),
            SystemdUnitType::Scope
        );
        assert_eq!(
            SystemdUnitType::from_unit_name("user-1000.slice"),
            SystemdUnitType::Slice
        );
        assert_eq!(
            SystemdUnitType::from_unit_name("ssh.socket"),
            SystemdUnitType::Socket
        );
        assert_eq!(
            SystemdUnitType::from_unit_name("graphical.target"),
            SystemdUnitType::Target
        );
        assert_eq!(
            SystemdUnitType::from_unit_name("random-name"),
            SystemdUnitType::Unknown
        );
    }

    #[test]
    fn test_active_state_from_str() {
        assert_eq!(
            SystemdActiveState::from_str("active"),
            SystemdActiveState::Active
        );
        assert_eq!(
            SystemdActiveState::from_str("Active"),
            SystemdActiveState::Active
        );
        assert_eq!(
            SystemdActiveState::from_str("inactive"),
            SystemdActiveState::Inactive
        );
        assert_eq!(
            SystemdActiveState::from_str("failed"),
            SystemdActiveState::Failed
        );
        assert_eq!(
            SystemdActiveState::from_str("unknown-state"),
            SystemdActiveState::Unknown
        );
    }

    #[test]
    fn test_active_state_is_running() {
        assert!(SystemdActiveState::Active.is_running());
        assert!(SystemdActiveState::Reloading.is_running());
        assert!(!SystemdActiveState::Inactive.is_running());
        assert!(!SystemdActiveState::Failed.is_running());
    }

    #[test]
    fn test_parse_systemctl_output() {
        let output = r#"Id=nginx.service
ActiveState=active
SubState=running
MainPID=1234
ControlPID=0
FragmentPath=/usr/lib/systemd/system/nginx.service
Description=The nginx HTTP and reverse proxy server
"#;

        let unit = parse_systemctl_output(output, 1234).unwrap();

        assert_eq!(unit.name, "nginx.service");
        assert_eq!(unit.unit_type, SystemdUnitType::Service);
        assert_eq!(unit.active_state, SystemdActiveState::Active);
        assert_eq!(unit.sub_state, Some("running".to_string()));
        assert_eq!(unit.main_pid, Some(1234));
        assert_eq!(unit.control_pid, None); // 0 should be filtered
        assert!(unit.fragment_path.is_some());
        assert!(unit.description.is_some());
        assert!(unit.is_main_process);
    }

    #[test]
    fn test_parse_systemctl_output_not_main_process() {
        let output = r#"Id=nginx.service
ActiveState=active
SubState=running
MainPID=1234
ControlPID=0
FragmentPath=/usr/lib/systemd/system/nginx.service
Description=The nginx HTTP and reverse proxy server
"#;

        // PID 5678 is not the main process
        let unit = parse_systemctl_output(output, 5678).unwrap();
        assert!(!unit.is_main_process);
    }

    #[test]
    fn test_parse_systemctl_output_scope() {
        let output = r#"Id=session-1.scope
ActiveState=active
SubState=running
MainPID=0
ControlPID=0
FragmentPath=
Description=Session 1 of user ubuntu
"#;

        let unit = parse_systemctl_output(output, 1234).unwrap();

        assert_eq!(unit.name, "session-1.scope");
        assert_eq!(unit.unit_type, SystemdUnitType::Scope);
        assert_eq!(unit.main_pid, None); // 0 filtered
        assert_eq!(unit.fragment_path, None); // Empty filtered
    }

    #[test]
    fn test_parse_systemctl_output_empty_id() {
        let output = r#"Id=-
ActiveState=inactive
"#;

        let unit = parse_systemctl_output(output, 1234);
        assert!(unit.is_none());
    }

    #[test]
    fn test_unit_from_cgroup_path() {
        let unit = unit_from_cgroup_path("nginx.service", 1234);

        assert_eq!(unit.name, "nginx.service");
        assert_eq!(unit.unit_type, SystemdUnitType::Service);
        assert_eq!(unit.active_state, SystemdActiveState::Unknown);
        assert_eq!(unit.provenance.source, SystemdDataSource::CgroupPath);
        assert!(!unit.provenance.warnings.is_empty());
    }

    #[test]
    fn test_parse_properties() {
        let output = "Key1=value1\nKey2=value2\nKey3=\n";
        let props = parse_properties(output);

        assert_eq!(props.get("Key1"), Some(&"value1".to_string()));
        assert_eq!(props.get("Key2"), Some(&"value2".to_string()));
        assert_eq!(props.get("Key3"), Some(&"".to_string()));
    }
}
