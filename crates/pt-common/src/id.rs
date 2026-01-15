//! Process and session identity types.
//!
//! These types ensure safe process identification across the codebase.
//! A process is uniquely identified by (pid, start_id, uid) tuple.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Process ID wrapper with display formatting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ProcessId(pub u32);

impl fmt::Display for ProcessId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<u32> for ProcessId {
    fn from(pid: u32) -> Self {
        ProcessId(pid)
    }
}

/// Start ID - unique identifier for a specific process incarnation.
///
/// Format: `<boot_id>:<start_time_ticks>:<pid>` (Linux)
/// or `<boot_id>:<start_time>:<pid>` (macOS)
///
/// This disambiguates PID reuse across reboots and within a boot.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct StartId(pub String);

impl StartId {
    /// Create a new StartId from components (Linux).
    pub fn from_linux(boot_id: &str, start_time_ticks: u64, pid: u32) -> Self {
        StartId(format!("{}:{}:{}", boot_id, start_time_ticks, pid))
    }

    /// Create a new StartId from components (macOS).
    pub fn from_macos(boot_id: &str, start_time: u64, pid: u32) -> Self {
        StartId(format!("{}:{}:{}", boot_id, start_time, pid))
    }

    /// Parse and validate a StartId string.
    pub fn parse(s: &str) -> Option<Self> {
        let mut parts = s.split(':');
        let boot_id = parts.next()?;
        let start_time = parts.next()?;
        let pid = parts.next()?;
        if parts.next().is_some() {
            return None;
        }
        if uuid::Uuid::parse_str(boot_id).is_err() {
            return None;
        }
        if start_time.parse::<u64>().is_err() {
            return None;
        }
        if pid.parse::<u32>().is_err() {
            return None;
        }
        Some(StartId(s.to_string()))
    }
}

impl fmt::Display for StartId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Session ID for tracking triage sessions.
///
/// Format: `pt-YYYYMMDD-HHMMSS-XXXX`
/// Example: `pt-20260115-143022-a7xq`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SessionId(pub String);

impl SessionId {
    /// Generate a new session ID.
    pub fn new() -> Self {
        let now = chrono::Utc::now();
        let suffix = generate_base32_suffix();
        SessionId(format!("pt-{}-{}-{}", now.format("%Y%m%d"), now.format("%H%M%S"), suffix))
    }

    /// Parse an existing session ID string.
    pub fn parse(s: &str) -> Option<Self> {
        if s.len() != 23 {
            return None;
        }
        let bytes = s.as_bytes();
        if bytes.get(0) != Some(&b'p')
            || bytes.get(1) != Some(&b't')
            || bytes.get(2) != Some(&b'-')
            || bytes.get(11) != Some(&b'-')
            || bytes.get(18) != Some(&b'-')
        {
            return None;
        }
        let date = &s[3..11];
        let time = &s[12..18];
        let suffix = &s[19..23];
        if !date.chars().all(|c| c.is_ascii_digit()) {
            return None;
        }
        if !time.chars().all(|c| c.is_ascii_digit()) {
            return None;
        }
        if !suffix.chars().all(|c| matches!(c, 'a'..='z' | '2'..='7')) {
            return None;
        }
        Some(SessionId(s.to_string()))
    }
}

impl Default for SessionId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for SessionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Complete process identity tuple for safe revalidation.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ProcessIdentity {
    pub pid: ProcessId,
    pub start_id: StartId,
    pub uid: u32,
}

impl ProcessIdentity {
    pub fn new(pid: u32, start_id: StartId, uid: u32) -> Self {
        ProcessIdentity {
            pid: ProcessId(pid),
            start_id,
            uid,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_id_format() {
        let sid = SessionId::new();
        assert!(sid.0.starts_with("pt-"));
        assert_eq!(sid.0.len(), 23);
    }

    #[test]
    fn test_start_id_linux() {
        let sid = StartId::from_linux("9d2d4e20-8c2b-4a3a-a8a2-90bcb7a1d86f", 123456789, 4242);
        assert_eq!(
            sid.0,
            "9d2d4e20-8c2b-4a3a-a8a2-90bcb7a1d86f:123456789:4242"
        );
    }

    #[test]
    fn test_start_id_macos() {
        let sid = StartId::from_macos("9d2d4e20-8c2b-4a3a-a8a2-90bcb7a1d86f", 987654321, 1234);
        assert_eq!(
            sid.0,
            "9d2d4e20-8c2b-4a3a-a8a2-90bcb7a1d86f:987654321:1234"
        );
    }
}

fn generate_base32_suffix() -> String {
    let uuid = uuid::Uuid::new_v4();
    let bytes = uuid.as_bytes();
    let mut value = ((bytes[0] as u32) << 16) | ((bytes[1] as u32) << 8) | (bytes[2] as u32);
    value &= 0x000F_FFFF;
    let alphabet = b"abcdefghijklmnopqrstuvwxyz234567";
    let mut out = String::with_capacity(4);
    for shift in [15_u32, 10, 5, 0] {
        let idx = ((value >> shift) & 0x1F) as usize;
        out.push(alphabet[idx] as char);
    }
    out
}
