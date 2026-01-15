//! Comprehensive audit logging with cryptographic integrity verification.
//!
//! The audit module provides tamper-evident logging for all pt-core operations.
//! Each entry includes a SHA-256 hash of the previous entry, creating an
//! append-only log where tampering can be detected.
//!
//! # Design
//!
//! - **Format**: JSON Lines (JSONL), one entry per line
//! - **Hash chain**: Each entry includes `prev_hash` (SHA-256 of previous entry)
//! - **Rotation**: Logs rotate at configurable size/age with checkpoint preservation
//! - **Verification**: `verify_log()` validates the complete hash chain
//!
//! # Usage
//!
//! ```ignore
//! use pt_core::audit::{AuditLog, AuditEventType, AuditContext};
//!
//! // Open or create the audit log
//! let mut log = AuditLog::open_or_create()?;
//!
//! // Create context for consistent session/run IDs
//! let ctx = AuditContext::new("run-12345", "host-abc");
//!
//! // Log events
//! log.log_scan_started(&ctx, 150)?;
//! log.log_action_executed(&ctx, 1234, "kill", true, None)?;
//! log.log_policy_check(&ctx, "protected_pattern", true, Some("systemd matched"))?;
//!
//! // Verify integrity
//! let result = log.verify_integrity()?;
//! assert!(result.is_valid);
//! ```
//!
//! # File Location
//!
//! The audit log is stored at:
//! - `$PROCESS_TRIAGE_DATA/audit/audit.jsonl` (if PROCESS_TRIAGE_DATA is set)
//! - `$XDG_DATA_HOME/process_triage/audit/audit.jsonl` (otherwise)
//!
//! Rotated logs are named `audit.YYYYMMDD-HHMMSS.jsonl` with a final checkpoint entry.

mod entry;
mod verify;
mod writer;

pub use entry::{
    ActionDetails, AuditContext, AuditEntry, AuditEventType, CheckpointDetails, ErrorDetails,
    PolicyCheckDetails, RecommendDetails, ScanDetails, AUDIT_SCHEMA_VERSION,
};
pub use verify::{
    verify_log, verify_log_chain, BreakType, BrokenLink, SchemaWarning, TamperedEntry,
    VerificationResult,
};
pub use writer::{AuditLog, AuditLogConfig, RotationConfig, GENESIS_HASH};

use std::path::PathBuf;
use thiserror::Error;

/// Errors that can occur during audit logging operations.
#[derive(Debug, Error)]
pub enum AuditError {
    #[error("failed to resolve audit log directory (set PROCESS_TRIAGE_DATA or XDG_DATA_HOME)")]
    DataDirUnavailable,

    #[error("I/O error at {path}: {source}")]
    Io {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("failed to serialize audit entry: {source}")]
    Serialization {
        #[source]
        source: serde_json::Error,
    },

    #[error("failed to parse audit entry at line {line}: {source}")]
    Parse {
        line: usize,
        #[source]
        source: serde_json::Error,
    },

    #[error("hash chain verification failed: {message}")]
    IntegrityError { message: String },

    #[error("audit log is locked by another process")]
    Locked,
}

/// Default directory name for audit logs within the data directory.
pub(crate) const AUDIT_DIR_NAME: &str = "audit";

/// Default audit log filename.
pub(crate) const AUDIT_LOG_FILENAME: &str = "audit.jsonl";

/// Resolve the audit log directory using standard XDG paths.
pub fn resolve_audit_dir() -> Result<PathBuf, AuditError> {
    // 1. Explicit override: PROCESS_TRIAGE_DATA
    if let Ok(dir) = std::env::var("PROCESS_TRIAGE_DATA") {
        return Ok(PathBuf::from(dir).join(AUDIT_DIR_NAME));
    }

    // 2. XDG_DATA_HOME
    if let Ok(xdg) = std::env::var("XDG_DATA_HOME") {
        return Ok(PathBuf::from(xdg)
            .join("process_triage")
            .join(AUDIT_DIR_NAME));
    }

    // 3. Platform default (dirs crate)
    if let Some(base) = dirs::data_dir() {
        return Ok(base.join("process_triage").join(AUDIT_DIR_NAME));
    }

    Err(AuditError::DataDirUnavailable)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_audit_dir_with_env() {
        // Save original value
        let orig = std::env::var("PROCESS_TRIAGE_DATA").ok();

        std::env::set_var("PROCESS_TRIAGE_DATA", "/tmp/pt-test-data");
        let dir = resolve_audit_dir().unwrap();
        assert_eq!(dir, PathBuf::from("/tmp/pt-test-data/audit"));

        // Restore original value
        match orig {
            Some(v) => std::env::set_var("PROCESS_TRIAGE_DATA", v),
            None => std::env::remove_var("PROCESS_TRIAGE_DATA"),
        }
    }
}
