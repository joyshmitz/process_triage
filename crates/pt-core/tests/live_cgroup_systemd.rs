//! Live-system integration tests for cgroup + systemd detection.
//!
//! Linux-only, no mocks. Tests are gated on availability of /proc and systemctl.

#![cfg(target_os = "linux")]

use pt_core::collect::{collect_cgroup_details, collect_systemd_unit, is_systemd_available};

fn skip_if_proc_unavailable() -> bool {
    if !std::path::Path::new("/proc").exists() {
        eprintln!("skipping: /proc not available on this system");
        return true;
    }
    false
}

#[test]
fn live_collect_cgroup_details() {
    if skip_if_proc_unavailable() {
        return;
    }

    let pid = std::process::id();
    let details = collect_cgroup_details(pid).expect("collect_cgroup_details");

    assert!(
        details.unified_path.is_some() || !details.v1_paths.is_empty(),
        "expected v1 or v2 cgroup data"
    );
}

#[test]
fn live_collect_systemd_unit() {
    if skip_if_proc_unavailable() {
        return;
    }

    if !is_systemd_available() {
        eprintln!("skipping: systemctl not available");
        return;
    }

    let pid = std::process::id();
    let unit = collect_systemd_unit(pid, None);

    // Some systems may not attach a unit for the current process; allow None.
    if let Some(unit) = unit {
        assert!(!unit.name.is_empty(), "systemd unit should have a name");
    }
}
