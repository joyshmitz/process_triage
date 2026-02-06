#![cfg(all(feature = "daemon", unix))]

//! E2E tests for dormant daemon behavior (feature-gated behind `daemon`).
//!
//! These tests exercise the CLI daemon loop end-to-end (spawn process, write
//! config, observe state/inbox artifacts), without requiring any UI.

use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::time::{Duration, Instant};
use tempfile::TempDir;

fn write_daemon_json_config(config_dir: &Path, content: &str) {
    fs::create_dir_all(config_dir).expect("create config dir");
    fs::write(config_dir.join("daemon.json"), content).expect("write daemon.json");
}

fn daemon_pid_path(data_dir: &Path) -> PathBuf {
    data_dir.join("daemon").join("daemon.pid")
}

fn daemon_state_path(data_dir: &Path) -> PathBuf {
    data_dir.join("daemon").join("state.json")
}

fn inbox_items_path(data_dir: &Path) -> PathBuf {
    data_dir.join("inbox").join("items.jsonl")
}

fn acquire_global_lock(data_dir: &Path) -> std::fs::File {
    use std::os::unix::io::AsRawFd;

    let path = data_dir.join(".pt-lock");
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("create lock dir");
    }
    let file = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(&path)
        .expect("open lock file");

    let fd = file.as_raw_fd();
    let rc = unsafe { libc::flock(fd, libc::LOCK_EX | libc::LOCK_NB) };
    assert_eq!(rc, 0, "expected to acquire global lock at {:?}", path);
    file
}

fn start_daemon_foreground(config_dir: &Path, data_dir: &Path) -> Child {
    let exe = assert_cmd::cargo::cargo_bin!("pt-core");
    let mut cmd = Command::new(exe);
    cmd.args([
        "--format",
        "json",
        "--config",
        config_dir.to_string_lossy().as_ref(),
        "daemon",
        "start",
        "--foreground",
    ]);

    cmd.env("PROCESS_TRIAGE_DATA", data_dir);
    cmd.env("PROCESS_TRIAGE_CONFIG", config_dir);
    // The daemon loop uses the global lock directly; make sure we do not skip it.
    cmd.env_remove("PT_SKIP_GLOBAL_LOCK");

    cmd.stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());

    cmd.spawn().expect("spawn daemon")
}

fn send_sigterm(child: &Child) {
    let pid = child.id() as i32;
    unsafe {
        libc::kill(pid, libc::SIGTERM);
    }
}

fn wait_for<F: FnMut() -> bool>(timeout: Duration, mut f: F) {
    let start = Instant::now();
    while start.elapsed() < timeout {
        if f() {
            return;
        }
        std::thread::sleep(Duration::from_millis(50));
    }
    panic!("timed out after {:?}", timeout);
}

fn read_jsonl_items(path: &Path) -> Vec<Value> {
    let content = fs::read_to_string(path).expect("read jsonl");
    content
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|line| serde_json::from_str::<Value>(line).expect("valid JSONL line"))
        .collect()
}

#[test]
fn daemon_lock_contention_writes_inbox_item_and_cleans_pid() {
    let data_dir = TempDir::new().expect("temp data dir");
    let config_dir = TempDir::new().expect("temp config dir");

    write_daemon_json_config(
        config_dir.path(),
        r#"{
  "tick_interval_secs": 1,
  "max_cpu_percent": 1000.0,
  "max_rss_mb": 4096,
  "triggers": {
    "ewma_alpha": 0.3,
    "load_threshold": -1.0,
    "memory_threshold": 2.0,
    "orphan_threshold": 999999,
    "sustained_ticks": 1,
    "cooldown_ticks": 10
  },
  "escalation": {
    "min_interval_secs": 0,
    "allow_auto_mitigation": false,
    "max_deep_scan_targets": 1
  },
  "notifications": {
    "enabled": false,
    "desktop": false,
    "notify_cmd": null,
    "notify_arg": []
  }
}"#,
    );

    let _lock = acquire_global_lock(data_dir.path());
    let mut child = start_daemon_foreground(config_dir.path(), data_dir.path());

    let inbox_path = inbox_items_path(data_dir.path());
    wait_for(Duration::from_secs(10), || {
        inbox_path.exists()
            && read_jsonl_items(&inbox_path).iter().any(|v| {
                v.get("type")
                    .and_then(|t| t.as_str())
                    .map(|t| t == "lock_contention")
                    .unwrap_or(false)
            })
    });

    send_sigterm(&child);
    wait_for(Duration::from_secs(10), || {
        child.try_wait().unwrap().is_some()
    });

    // Ensure the daemon cleaned up its own pid file.
    assert!(
        !daemon_pid_path(data_dir.path()).exists(),
        "daemon pid file should be removed on clean shutdown"
    );
}

#[test]
fn daemon_trigger_cooldown_prevents_repeated_lock_contention_spam() {
    let data_dir = TempDir::new().expect("temp data dir");
    let config_dir = TempDir::new().expect("temp config dir");

    write_daemon_json_config(
        config_dir.path(),
        r#"{
  "tick_interval_secs": 1,
  "max_cpu_percent": 1000.0,
  "max_rss_mb": 4096,
  "triggers": {
    "ewma_alpha": 0.3,
    "load_threshold": -1.0,
    "memory_threshold": 2.0,
    "orphan_threshold": 999999,
    "sustained_ticks": 1,
    "cooldown_ticks": 100
  },
  "escalation": {
    "min_interval_secs": 0,
    "allow_auto_mitigation": false,
    "max_deep_scan_targets": 1
  },
  "notifications": {
    "enabled": false,
    "desktop": false,
    "notify_cmd": null,
    "notify_arg": []
  }
}"#,
    );

    let _lock = acquire_global_lock(data_dir.path());
    let mut child = start_daemon_foreground(config_dir.path(), data_dir.path());

    let inbox_path = inbox_items_path(data_dir.path());
    wait_for(Duration::from_secs(10), || inbox_path.exists());
    std::thread::sleep(Duration::from_secs(3)); // allow multiple ticks

    send_sigterm(&child);
    wait_for(Duration::from_secs(10), || {
        child.try_wait().unwrap().is_some()
    });

    let items = read_jsonl_items(&inbox_path);
    let lock_items = items
        .iter()
        .filter(|v| v.get("type").and_then(|t| t.as_str()) == Some("lock_contention"))
        .count();
    assert_eq!(
        lock_items, 1,
        "cooldown should prevent repeated lock contention entries across ticks"
    );
}

#[test]
fn daemon_overhead_budget_exceeded_is_persisted_and_skips_inbox_writes() {
    let data_dir = TempDir::new().expect("temp data dir");
    let config_dir = TempDir::new().expect("temp config dir");

    write_daemon_json_config(
        config_dir.path(),
        r#"{
  "tick_interval_secs": 1,
  "max_cpu_percent": 1000.0,
  "max_rss_mb": 0,
  "triggers": {
    "ewma_alpha": 0.3,
    "load_threshold": -1.0,
    "memory_threshold": 2.0,
    "orphan_threshold": 999999,
    "sustained_ticks": 1,
    "cooldown_ticks": 10
  },
  "escalation": {
    "min_interval_secs": 0,
    "allow_auto_mitigation": false,
    "max_deep_scan_targets": 1
  },
  "notifications": {
    "enabled": false,
    "desktop": false,
    "notify_cmd": null,
    "notify_arg": []
  }
}"#,
    );

    let mut child = start_daemon_foreground(config_dir.path(), data_dir.path());

    let state_path = daemon_state_path(data_dir.path());
    wait_for(Duration::from_secs(10), || state_path.exists());

    wait_for(Duration::from_secs(10), || {
        let content = fs::read_to_string(&state_path).expect("read state.json");
        let json: Value = serde_json::from_str(&content).expect("valid state json");
        let events = json
            .get("daemon")
            .and_then(|d| d.get("recent_events"))
            .and_then(|e| e.as_array())
            .cloned()
            .unwrap_or_default();
        events.iter().any(|ev| {
            ev.get("event_type")
                .and_then(|t| t.as_str())
                .map(|t| t == "overhead_budget_exceeded")
                .unwrap_or(false)
        })
    });

    send_sigterm(&child);
    wait_for(Duration::from_secs(10), || {
        child.try_wait().unwrap().is_some()
    });

    // When budget is exceeded we skip escalation; inbox should remain absent.
    assert!(
        !inbox_items_path(data_dir.path()).exists(),
        "no inbox items should be written when overhead budget is exceeded"
    );
}
