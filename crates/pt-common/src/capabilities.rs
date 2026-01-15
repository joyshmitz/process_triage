//! Capabilities cache schema and types.
//!
//! This module defines the schema for caching detected system capabilities
//! and tool availability. The cache enables:
//! - pt-core to make decisions based on available tools
//! - Graceful degradation when tools are missing
//! - User awareness of what's available vs missing
//! - Conditional probe selection
//!
//! Cache location: `~/.cache/pt/capabilities.json`

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// Current capabilities schema version.
pub const CAPABILITIES_SCHEMA_VERSION: &str = "1.0.0";

/// Default cache staleness threshold in seconds (1 hour).
pub const DEFAULT_CACHE_TTL_SECS: u64 = 3600;

/// Complete capabilities manifest for the system.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Capabilities {
    /// Schema version for forward compatibility.
    pub schema_version: String,

    /// Operating system information.
    pub os: OsInfo,

    /// Available diagnostic tools and their capabilities.
    pub tools: HashMap<String, ToolInfo>,

    /// Linux /proc filesystem availability (None on non-Linux).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proc_fs: Option<ProcFsInfo>,

    /// Control groups availability (Linux).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cgroups: Option<CgroupInfo>,

    /// Systemd availability (Linux).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub systemd: Option<SystemdInfo>,

    /// Launchd availability (macOS).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launchd: Option<LaunchdInfo>,

    /// Pressure Stall Information availability (Linux).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub psi: Option<PsiInfo>,

    /// Container runtime detection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub containers: Option<ContainerInfo>,

    /// Sudo availability for privileged operations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sudo: Option<SudoInfo>,

    /// Current user information.
    pub user: UserInfo,

    /// Standard paths used by process_triage.
    pub paths: PathsInfo,

    /// System resource information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<SystemInfo>,

    /// Privilege and capability information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privileges: Option<PrivilegesInfo>,

    /// ISO 8601 timestamp of when capabilities were discovered.
    pub discovered_at: String,

    /// How long capability discovery took in milliseconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discovery_duration_ms: Option<u64>,

    /// Version of pt (bash) that generated this manifest.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wrapper_version: Option<String>,
}

/// Operating system information.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OsInfo {
    /// OS family (affects collection strategy).
    pub family: OsFamily,

    /// Distribution or OS name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// OS release name (e.g., "Ubuntu 24.04", "macOS 14.0").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release: Option<String>,

    /// OS version string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    /// Kernel version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kernel: Option<String>,

    /// CPU architecture.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arch: Option<CpuArch>,
}

/// Operating system family.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum OsFamily {
    Linux,
    #[serde(alias = "darwin")]
    Macos,
    Freebsd,
    Unknown,
}

/// CPU architecture.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum CpuArch {
    X86_64,
    Aarch64,
    Arm64,
    I686,
    Armv7l,
}

/// Information about a single diagnostic tool.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ToolInfo {
    /// Whether the tool is installed and executable.
    pub available: bool,

    /// Absolute path to the tool binary.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,

    /// Tool version string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    /// Permission-related capabilities for this tool.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<ToolPermissions>,

    /// Tool passed a basic functionality check (not just installed).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub functional: Option<bool>,

    /// Why the tool has limited functionality.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restricted_reason: Option<String>,

    /// Additional notes about tool availability or quirks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,

    /// Reason tool is unavailable (if not available).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

/// Permission-related capabilities for a tool.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ToolPermissions {
    /// Tool requires sudo for full functionality.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sudo: Option<bool>,

    /// CAP_SYS_PTRACE capability available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cap_sys_ptrace: Option<bool>,

    /// CAP_PERFMON capability available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cap_perfmon: Option<bool>,

    /// CAP_BPF capability available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cap_bpf: Option<bool>,

    /// CAP_NET_ADMIN capability available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cap_net_admin: Option<bool>,

    /// Tool has setuid bit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suid: Option<bool>,
}

/// Linux /proc filesystem availability.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProcFsInfo {
    /// Whether /proc is mounted and readable.
    pub available: bool,

    /// Which /proc/PID/* files are readable by current user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readable_fields: Option<Vec<ProcField>>,
}

/// Readable /proc/PID/* fields.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ProcField {
    Stat,
    Status,
    Io,
    Cgroup,
    Wchan,
    Fd,
    Maps,
    Smaps,
    Schedstat,
    Sched,
    Ns,
    Environ,
    Cmdline,
    Cwd,
    Exe,
}

/// Control groups availability.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CgroupInfo {
    /// Cgroups version in use.
    pub version: CgroupVersion,

    /// Whether cgroup paths are readable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readable: Option<bool>,

    /// Available cgroup controllers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controllers: Option<Vec<String>>,
}

/// Cgroups version.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum CgroupVersion {
    V1,
    V2,
    Hybrid,
    None,
}

/// Systemd availability (Linux).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SystemdInfo {
    /// Whether systemd is the init system.
    pub available: bool,

    /// Systemd version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    /// Whether user units are accessible.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_units: Option<bool>,
}

/// Launchd availability (macOS).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LaunchdInfo {
    /// Whether launchd is available.
    pub available: bool,

    /// Whether user agents are accessible.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agents: Option<bool>,
}

/// Pressure Stall Information availability (Linux).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PsiInfo {
    /// Whether PSI is available.
    pub available: bool,

    /// CPU pressure available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu: Option<bool>,

    /// Memory pressure available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<bool>,

    /// IO pressure available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub io: Option<bool>,
}

/// Container runtime detection.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ContainerInfo {
    /// Docker availability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub docker: Option<DockerInfo>,

    /// Podman availability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub podman: Option<PodmanInfo>,

    /// Whether running inside a container.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inside_container: Option<bool>,

    /// Container type if running inside one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_type: Option<String>,
}

/// Docker availability information.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DockerInfo {
    pub available: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub socket: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// Podman availability information.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PodmanInfo {
    pub available: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rootless: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// Sudo availability for privileged operations.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SudoInfo {
    /// Whether sudo is available.
    pub available: bool,

    /// Whether sudo can run without password (NOPASSWD).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passwordless: Option<bool>,

    /// Whether a cached sudo session exists.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_active: Option<bool>,
}

/// Current user information.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserInfo {
    /// User ID.
    pub uid: u32,

    /// Username.
    pub username: String,

    /// Home directory path.
    pub home: String,

    /// User's login shell path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shell: Option<String>,
}

/// Standard paths used by process_triage.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PathsInfo {
    /// Configuration directory (e.g., ~/.config/process_triage).
    pub config_dir: String,

    /// Data directory (e.g., ~/.local/share/process_triage).
    pub data_dir: String,

    /// Cache directory (e.g., ~/.cache/pt).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_dir: Option<String>,

    /// Telemetry storage directory.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telemetry_dir: Option<String>,

    /// Path to the coordination lock file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock_file: Option<String>,
}

/// System resource information.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SystemInfo {
    /// Number of logical CPUs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_count: Option<u32>,

    /// Total system memory in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_total_bytes: Option<u64>,

    /// Clock ticks per second (sysconf(_SC_CLK_TCK)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clk_tck: Option<u32>,

    /// Unique boot identifier from /proc/sys/kernel/random/boot_id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boot_id: Option<String>,
}

/// Privilege and capability information.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PrivilegesInfo {
    /// Whether non-interactive sudo is available (sudo -n).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_sudo: Option<bool>,

    /// Value of /proc/sys/kernel/perf_event_paranoid (-1 to 4).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub perf_paranoid: Option<i8>,

    /// Value of /proc/sys/kernel/yama/ptrace_scope (0-3).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ptrace_scope: Option<u8>,
}

impl Capabilities {
    /// Check if the capabilities cache is stale.
    pub fn is_stale(&self, ttl_secs: u64) -> bool {
        use chrono::{DateTime, Utc};

        let discovered_at = match DateTime::parse_from_rfc3339(&self.discovered_at) {
            Ok(dt) => dt.with_timezone(&Utc),
            Err(_) => return true, // Can't parse, assume stale
        };

        let now = Utc::now();
        let age = now.signed_duration_since(discovered_at);

        age.num_seconds() > ttl_secs as i64
    }

    /// Check if a tool is available.
    pub fn has_tool(&self, name: &str) -> bool {
        self.tools
            .get(name)
            .map(|t| t.available)
            .unwrap_or(false)
    }

    /// Check if a tool is functional (available and passed functionality check).
    pub fn tool_is_functional(&self, name: &str) -> bool {
        self.tools
            .get(name)
            .map(|t| t.available && t.functional.unwrap_or(true))
            .unwrap_or(false)
    }

    /// Get the path to a tool if available.
    pub fn tool_path(&self, name: &str) -> Option<&str> {
        self.tools
            .get(name)
            .filter(|t| t.available)
            .and_then(|t| t.path.as_deref())
    }

    /// Check if running on Linux.
    pub fn is_linux(&self) -> bool {
        matches!(self.os.family, OsFamily::Linux)
    }

    /// Check if running on macOS.
    pub fn is_macos(&self) -> bool {
        matches!(self.os.family, OsFamily::Macos)
    }

    /// Check if /proc is available.
    pub fn has_procfs(&self) -> bool {
        self.proc_fs
            .as_ref()
            .map(|p| p.available)
            .unwrap_or(false)
    }

    /// Check if a specific /proc field is readable.
    pub fn can_read_proc_field(&self, field: ProcField) -> bool {
        self.proc_fs
            .as_ref()
            .and_then(|p| p.readable_fields.as_ref())
            .map(|fields| fields.contains(&field))
            .unwrap_or(false)
    }

    /// Check if cgroups v2 is available.
    pub fn has_cgroups_v2(&self) -> bool {
        self.cgroups
            .as_ref()
            .map(|c| matches!(c.version, CgroupVersion::V2))
            .unwrap_or(false)
    }

    /// Check if systemd is available.
    pub fn has_systemd(&self) -> bool {
        self.systemd
            .as_ref()
            .map(|s| s.available)
            .unwrap_or(false)
    }

    /// Check if PSI (Pressure Stall Information) is available.
    pub fn has_psi(&self) -> bool {
        self.psi
            .as_ref()
            .map(|p| p.available)
            .unwrap_or(false)
    }

    /// Check if non-interactive sudo is available.
    pub fn can_sudo(&self) -> bool {
        self.sudo
            .as_ref()
            .map(|s| s.available && s.passwordless.unwrap_or(false))
            .unwrap_or(false)
            || self.privileges
                .as_ref()
                .and_then(|p| p.can_sudo)
                .unwrap_or(false)
    }

    /// Get cache file path.
    pub fn cache_path() -> PathBuf {
        let cache_dir = dirs::cache_dir()
            .unwrap_or_else(|| PathBuf::from("/tmp"))
            .join("pt");
        cache_dir.join("capabilities.json")
    }

    /// Load capabilities from cache file.
    pub fn load_from_cache() -> Result<Self, CapabilitiesError> {
        let path = Self::cache_path();
        let contents = std::fs::read_to_string(&path)
            .map_err(|e| CapabilitiesError::IoError {
                path: path.clone(),
                reason: e.to_string(),
            })?;

        serde_json::from_str(&contents).map_err(|e| CapabilitiesError::ParseError {
            path,
            reason: e.to_string(),
        })
    }

    /// Save capabilities to cache file.
    pub fn save_to_cache(&self) -> Result<(), CapabilitiesError> {
        let path = Self::cache_path();

        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| CapabilitiesError::IoError {
                path: parent.to_path_buf(),
                reason: e.to_string(),
            })?;
        }

        let contents = serde_json::to_string_pretty(self).map_err(|e| {
            CapabilitiesError::SerializeError {
                reason: e.to_string(),
            }
        })?;

        std::fs::write(&path, contents).map_err(|e| CapabilitiesError::IoError {
            path,
            reason: e.to_string(),
        })
    }
}

impl Default for Capabilities {
    fn default() -> Self {
        Self {
            schema_version: CAPABILITIES_SCHEMA_VERSION.to_string(),
            os: OsInfo {
                family: OsFamily::Unknown,
                name: None,
                release: None,
                version: None,
                kernel: None,
                arch: None,
            },
            tools: HashMap::new(),
            proc_fs: None,
            cgroups: None,
            systemd: None,
            launchd: None,
            psi: None,
            containers: None,
            sudo: None,
            user: UserInfo {
                uid: 0,
                username: String::new(),
                home: String::new(),
                shell: None,
            },
            paths: PathsInfo {
                config_dir: String::new(),
                data_dir: String::new(),
                cache_dir: None,
                telemetry_dir: None,
                lock_file: None,
            },
            system: None,
            privileges: None,
            discovered_at: chrono::Utc::now().to_rfc3339(),
            discovery_duration_ms: None,
            wrapper_version: None,
        }
    }
}

/// Errors that can occur when working with capabilities.
#[derive(Debug, thiserror::Error)]
pub enum CapabilitiesError {
    #[error("Failed to read capabilities from {path}: {reason}")]
    IoError { path: PathBuf, reason: String },

    #[error("Failed to parse capabilities from {path}: {reason}")]
    ParseError { path: PathBuf, reason: String },

    #[error("Failed to serialize capabilities: {reason}")]
    SerializeError { reason: String },

    #[error("Capabilities cache is stale (discovered at {discovered_at})")]
    CacheStale { discovered_at: String },

    #[error("Schema version mismatch: expected {expected}, got {actual}")]
    VersionMismatch { expected: String, actual: String },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_capabilities() {
        let caps = Capabilities::default();
        assert_eq!(caps.schema_version, CAPABILITIES_SCHEMA_VERSION);
        assert!(matches!(caps.os.family, OsFamily::Unknown));
        assert!(caps.tools.is_empty());
    }

    #[test]
    fn test_has_tool() {
        let mut caps = Capabilities::default();
        caps.tools.insert(
            "ps".to_string(),
            ToolInfo {
                available: true,
                path: Some("/usr/bin/ps".to_string()),
                version: None,
                permissions: None,
                functional: Some(true),
                restricted_reason: None,
                notes: None,
                reason: None,
            },
        );

        assert!(caps.has_tool("ps"));
        assert!(!caps.has_tool("perf"));
    }

    #[test]
    fn test_tool_is_functional() {
        let mut caps = Capabilities::default();

        // Available and functional
        caps.tools.insert(
            "ps".to_string(),
            ToolInfo {
                available: true,
                path: Some("/usr/bin/ps".to_string()),
                version: None,
                permissions: None,
                functional: Some(true),
                restricted_reason: None,
                notes: None,
                reason: None,
            },
        );

        // Available but not functional
        caps.tools.insert(
            "perf".to_string(),
            ToolInfo {
                available: true,
                path: Some("/usr/bin/perf".to_string()),
                version: None,
                permissions: None,
                functional: Some(false),
                restricted_reason: Some("kernel.perf_event_paranoid=4".to_string()),
                notes: None,
                reason: None,
            },
        );

        assert!(caps.tool_is_functional("ps"));
        assert!(!caps.tool_is_functional("perf"));
        assert!(!caps.tool_is_functional("nonexistent"));
    }

    #[test]
    fn test_os_family_detection() {
        let mut caps = Capabilities::default();

        caps.os.family = OsFamily::Linux;
        assert!(caps.is_linux());
        assert!(!caps.is_macos());

        caps.os.family = OsFamily::Macos;
        assert!(!caps.is_linux());
        assert!(caps.is_macos());
    }

    #[test]
    fn test_procfs_availability() {
        let mut caps = Capabilities::default();
        assert!(!caps.has_procfs());

        caps.proc_fs = Some(ProcFsInfo {
            available: true,
            readable_fields: Some(vec![ProcField::Stat, ProcField::Status, ProcField::Cmdline]),
        });

        assert!(caps.has_procfs());
        assert!(caps.can_read_proc_field(ProcField::Stat));
        assert!(!caps.can_read_proc_field(ProcField::Environ));
    }

    #[test]
    fn test_serialization_roundtrip() {
        let mut caps = Capabilities::default();
        caps.os = OsInfo {
            family: OsFamily::Linux,
            name: Some("Ubuntu".to_string()),
            release: Some("24.04".to_string()),
            version: None,
            kernel: Some("6.8.0-40-generic".to_string()),
            arch: Some(CpuArch::X86_64),
        };
        caps.tools.insert(
            "ps".to_string(),
            ToolInfo {
                available: true,
                path: Some("/usr/bin/ps".to_string()),
                version: Some("procps-ng 4.0.4".to_string()),
                permissions: None,
                functional: Some(true),
                restricted_reason: None,
                notes: None,
                reason: None,
            },
        );

        let json = serde_json::to_string_pretty(&caps).unwrap();
        let parsed: Capabilities = serde_json::from_str(&json).unwrap();

        assert_eq!(caps.os.family, parsed.os.family);
        assert_eq!(caps.os.name, parsed.os.name);
        assert!(parsed.has_tool("ps"));
    }

    #[test]
    fn test_cache_path() {
        let path = Capabilities::cache_path();
        assert!(path.ends_with("capabilities.json"));
        assert!(path.to_string_lossy().contains("pt"));
    }

    #[test]
    fn test_is_stale() {
        let mut caps = Capabilities::default();

        // Fresh cache
        caps.discovered_at = chrono::Utc::now().to_rfc3339();
        assert!(!caps.is_stale(3600));

        // Stale cache (2 hours ago)
        let old_time = chrono::Utc::now() - chrono::Duration::hours(2);
        caps.discovered_at = old_time.to_rfc3339();
        assert!(caps.is_stale(3600)); // 1 hour TTL

        // Invalid timestamp
        caps.discovered_at = "invalid".to_string();
        assert!(caps.is_stale(3600)); // Treat as stale
    }
}
