//! Field classification for redaction decisions.

use serde::{Deserialize, Serialize};

/// Classification of data fields for redaction decisions.
///
/// Each field class has a default risk level and action, which can be
/// overridden by the redaction policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FieldClass {
    /// Full command line (highest risk)
    Cmdline,
    /// Command name only (argv[0])
    Cmd,
    /// Individual command line argument
    CmdlineArg,
    /// Environment variable name
    EnvName,
    /// Environment variable value (highest risk)
    EnvValue,
    /// Path under $HOME
    PathHome,
    /// Path under /tmp or temp directories
    PathTmp,
    /// System path (/usr, /etc)
    PathSystem,
    /// Project/work directory path
    PathProject,
    /// Machine hostname
    Hostname,
    /// IPv4 or IPv6 address
    IpAddress,
    /// Full URL
    Url,
    /// URL hostname component
    UrlHost,
    /// URL path component
    UrlPath,
    /// URL credentials (user:pass)
    UrlCredentials,
    /// System username
    Username,
    /// Numeric user ID
    Uid,
    /// Process ID
    Pid,
    /// Network port
    Port,
    /// Docker/container ID
    ContainerId,
    /// Systemd unit name
    SystemdUnit,
    /// Free-form text (logs, messages)
    FreeText,
}

impl FieldClass {
    /// Returns the risk level for this field class.
    pub fn risk_level(&self) -> RiskLevel {
        match self {
            FieldClass::Cmdline => RiskLevel::Critical,
            FieldClass::Cmd => RiskLevel::Low,
            FieldClass::CmdlineArg => RiskLevel::Variable,
            FieldClass::EnvName => RiskLevel::Low,
            FieldClass::EnvValue => RiskLevel::Critical,
            FieldClass::PathHome => RiskLevel::High,
            FieldClass::PathTmp => RiskLevel::Medium,
            FieldClass::PathSystem => RiskLevel::Low,
            FieldClass::PathProject => RiskLevel::High,
            FieldClass::Hostname => RiskLevel::Medium,
            FieldClass::IpAddress => RiskLevel::High,
            FieldClass::Url => RiskLevel::High,
            FieldClass::UrlHost => RiskLevel::Medium,
            FieldClass::UrlPath => RiskLevel::Medium,
            FieldClass::UrlCredentials => RiskLevel::Critical,
            FieldClass::Username => RiskLevel::High,
            FieldClass::Uid => RiskLevel::Low,
            FieldClass::Pid => RiskLevel::None,
            FieldClass::Port => RiskLevel::Low,
            FieldClass::ContainerId => RiskLevel::Low,
            FieldClass::SystemdUnit => RiskLevel::Low,
            FieldClass::FreeText => RiskLevel::Variable,
        }
    }

    /// Returns the default action for this field class.
    pub fn default_action(&self) -> crate::Action {
        use crate::Action;
        match self {
            FieldClass::Cmdline => Action::NormalizeHash,
            FieldClass::Cmd => Action::Allow,
            FieldClass::CmdlineArg => Action::DetectAction,
            FieldClass::EnvName => Action::Allow,
            FieldClass::EnvValue => Action::Redact,
            FieldClass::PathHome => Action::NormalizeHash,
            FieldClass::PathTmp => Action::Normalize,
            FieldClass::PathSystem => Action::Allow,
            FieldClass::PathProject => Action::Hash,
            FieldClass::Hostname => Action::Hash,
            FieldClass::IpAddress => Action::Hash,
            FieldClass::Url => Action::NormalizeHash,
            FieldClass::UrlHost => Action::Hash,
            FieldClass::UrlPath => Action::Normalize,
            FieldClass::UrlCredentials => Action::Redact,
            FieldClass::Username => Action::Hash,
            FieldClass::Uid => Action::Allow,
            FieldClass::Pid => Action::Allow,
            FieldClass::Port => Action::Allow,
            FieldClass::ContainerId => Action::Truncate,
            FieldClass::SystemdUnit => Action::Allow,
            FieldClass::FreeText => Action::DetectAction,
        }
    }

    /// Parse a field class from a string.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "cmdline" => Some(FieldClass::Cmdline),
            "cmd" => Some(FieldClass::Cmd),
            "cmdline_arg" => Some(FieldClass::CmdlineArg),
            "env_name" => Some(FieldClass::EnvName),
            "env_value" => Some(FieldClass::EnvValue),
            "path_home" => Some(FieldClass::PathHome),
            "path_tmp" => Some(FieldClass::PathTmp),
            "path_system" => Some(FieldClass::PathSystem),
            "path_project" => Some(FieldClass::PathProject),
            "hostname" => Some(FieldClass::Hostname),
            "ip_address" => Some(FieldClass::IpAddress),
            "url" => Some(FieldClass::Url),
            "url_host" => Some(FieldClass::UrlHost),
            "url_path" => Some(FieldClass::UrlPath),
            "url_credentials" => Some(FieldClass::UrlCredentials),
            "username" => Some(FieldClass::Username),
            "uid" => Some(FieldClass::Uid),
            "pid" => Some(FieldClass::Pid),
            "port" => Some(FieldClass::Port),
            "container_id" => Some(FieldClass::ContainerId),
            "systemd_unit" => Some(FieldClass::SystemdUnit),
            "free_text" => Some(FieldClass::FreeText),
            _ => None,
        }
    }
}

impl std::fmt::Display for FieldClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            FieldClass::Cmdline => "cmdline",
            FieldClass::Cmd => "cmd",
            FieldClass::CmdlineArg => "cmdline_arg",
            FieldClass::EnvName => "env_name",
            FieldClass::EnvValue => "env_value",
            FieldClass::PathHome => "path_home",
            FieldClass::PathTmp => "path_tmp",
            FieldClass::PathSystem => "path_system",
            FieldClass::PathProject => "path_project",
            FieldClass::Hostname => "hostname",
            FieldClass::IpAddress => "ip_address",
            FieldClass::Url => "url",
            FieldClass::UrlHost => "url_host",
            FieldClass::UrlPath => "url_path",
            FieldClass::UrlCredentials => "url_credentials",
            FieldClass::Username => "username",
            FieldClass::Uid => "uid",
            FieldClass::Pid => "pid",
            FieldClass::Port => "port",
            FieldClass::ContainerId => "container_id",
            FieldClass::SystemdUnit => "systemd_unit",
            FieldClass::FreeText => "free_text",
        };
        write!(f, "{}", s)
    }
}

/// Risk level for field classes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum RiskLevel {
    /// Never sensitive
    None,
    /// Generally safe
    Low,
    /// Contextual info
    Medium,
    /// PII, identifying info
    High,
    /// Secrets, credentials, tokens
    Critical,
    /// Depends on content
    Variable,
}
