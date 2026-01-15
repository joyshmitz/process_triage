//! Overview section data.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Overview section containing session summary.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverviewSection {
    /// Session identifier.
    pub session_id: String,
    /// Host identifier.
    pub host_id: String,
    /// Hostname.
    pub hostname: Option<String>,
    /// Session start time.
    pub started_at: DateTime<Utc>,
    /// Session end time.
    pub ended_at: Option<DateTime<Utc>>,
    /// Duration in milliseconds.
    pub duration_ms: Option<u64>,
    /// Session state.
    pub state: String,
    /// Scan mode.
    pub mode: String,
    /// Whether deep scan was enabled.
    pub deep_scan: bool,

    // Counts
    /// Total processes scanned.
    pub processes_scanned: usize,
    /// Candidates identified.
    pub candidates_found: usize,
    /// Kill actions attempted.
    pub kills_attempted: usize,
    /// Successful kills.
    pub kills_successful: usize,
    /// Spared processes.
    pub spares: usize,

    // System info
    /// Operating system family.
    pub os_family: Option<String>,
    /// OS version.
    pub os_version: Option<String>,
    /// Kernel version.
    pub kernel_version: Option<String>,
    /// Architecture.
    pub arch: Option<String>,
    /// CPU core count.
    pub cores: Option<u32>,
    /// Total memory in bytes.
    pub memory_bytes: Option<u64>,

    // Version info
    /// Process triage version.
    pub pt_version: Option<String>,
    /// Export profile used.
    pub export_profile: String,
}

impl OverviewSection {
    /// Get formatted duration.
    pub fn duration_formatted(&self) -> String {
        match self.duration_ms {
            Some(ms) if ms >= 60_000 => format!("{:.1} min", ms as f64 / 60_000.0),
            Some(ms) if ms >= 1_000 => format!("{:.1} s", ms as f64 / 1000.0),
            Some(ms) => format!("{} ms", ms),
            None => "N/A".to_string(),
        }
    }

    /// Get formatted memory.
    pub fn memory_formatted(&self) -> String {
        match self.memory_bytes {
            Some(bytes) if bytes >= 1_073_741_824 => {
                format!("{:.1} GB", bytes as f64 / 1_073_741_824.0)
            }
            Some(bytes) if bytes >= 1_048_576 => format!("{:.0} MB", bytes as f64 / 1_048_576.0),
            Some(bytes) => format!("{:.0} KB", bytes as f64 / 1024.0),
            None => "N/A".to_string(),
        }
    }

    /// Calculate candidate rate as percentage.
    pub fn candidate_rate_pct(&self) -> f64 {
        if self.processes_scanned > 0 {
            100.0 * self.candidates_found as f64 / self.processes_scanned as f64
        } else {
            0.0
        }
    }

    /// Calculate kill success rate as percentage.
    pub fn kill_success_rate_pct(&self) -> f64 {
        if self.kills_attempted > 0 {
            100.0 * self.kills_successful as f64 / self.kills_attempted as f64
        } else {
            0.0
        }
    }
}
