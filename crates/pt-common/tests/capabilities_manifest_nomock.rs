//! No-mock capability manifest tests using real fixtures.

use pt_common::{Capabilities, OsFamily};
use std::env;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};

static ENV_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

fn fixtures_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("test")
        .join("fixtures")
        .join("capabilities")
}

fn load_fixture() -> Capabilities {
    let path = fixtures_dir().join("capabilities.json");
    let contents = std::fs::read_to_string(&path).expect("read capabilities fixture");
    serde_json::from_str(&contents).expect("parse capabilities fixture")
}

struct EnvGuard {
    key: String,
    value: Option<String>,
}

impl EnvGuard {
    fn set(key: &str, value: &str) -> Self {
        let saved = env::var(key).ok();
        env::set_var(key, value);
        Self {
            key: key.to_string(),
            value: saved,
        }
    }
}

impl Drop for EnvGuard {
    fn drop(&mut self) {
        match &self.value {
            Some(value) => env::set_var(&self.key, value),
            None => env::remove_var(&self.key),
        }
    }
}

#[test]
fn test_capabilities_fixture_parses() {
    let caps = load_fixture();

    assert_eq!(caps.schema_version, "1.0.0");
    assert!(matches!(caps.os.family, OsFamily::Linux));
    assert_eq!(caps.os.name.as_deref(), Some("Ubuntu"));

    let ps = caps.tools.get("ps").expect("ps tool present");
    assert!(ps.available);
    assert_eq!(ps.path.as_deref(), Some("/usr/bin/ps"));

    let lsof = caps.tools.get("lsof").expect("lsof tool present");
    assert!(!lsof.available);

    assert_eq!(caps.user.username, "user");
    assert!(caps.paths.config_dir.contains("process_triage"));

    let privileges = caps.privileges.expect("privileges present");
    assert_eq!(privileges.can_sudo, Some(true));
}

#[test]
fn test_capabilities_cache_roundtrip() {
    let _guard = ENV_LOCK
        .get_or_init(|| Mutex::new(()))
        .lock()
        .expect("env lock poisoned");

    let temp = tempfile::tempdir().expect("tempdir");
    let _env_guard = EnvGuard::set("XDG_CACHE_HOME", temp.path().to_string_lossy().as_ref());

    let caps = load_fixture();
    caps.save_to_cache().expect("save cache");

    let loaded = Capabilities::load_from_cache().expect("load cache");
    assert_eq!(loaded.schema_version, caps.schema_version);
    assert_eq!(loaded.user.username, caps.user.username);
    assert_eq!(loaded.paths.config_dir, caps.paths.config_dir);
}
