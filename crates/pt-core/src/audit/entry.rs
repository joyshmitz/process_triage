//! Audit log entry types and schema.
//!
//! Each audit entry follows a consistent schema with:
//! - Timestamp (ISO-8601 with microseconds)
//! - Event type (scan, recommend, action, policy_check, error)
//! - Session/run context for correlation
//! - Event-specific details
//! - Hash chain fields for integrity

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Schema version for audit log entries.
pub const AUDIT_SCHEMA_VERSION: &str = "1.0.0";

/// Types of events recorded in the audit log.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuditEventType {
    /// Process scan started or completed.
    Scan,
    /// Action recommended for a process.
    Recommend,
    /// Action executed (or attempted) on a process.
    Action,
    /// Policy check performed (protected patterns, rate limits, etc.).
    PolicyCheck,
    /// Error encountered during operation.
    Error,
    /// Session lifecycle event (created, completed, etc.).
    Session,
    /// Log rotation checkpoint.
    Checkpoint,
}

impl std::fmt::Display for AuditEventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            AuditEventType::Scan => "scan",
            AuditEventType::Recommend => "recommend",
            AuditEventType::Action => "action",
            AuditEventType::PolicyCheck => "policy_check",
            AuditEventType::Error => "error",
            AuditEventType::Session => "session",
            AuditEventType::Checkpoint => "checkpoint",
        };
        write!(f, "{}", s)
    }
}

/// A single audit log entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntry {
    /// Schema version for forward compatibility.
    pub schema_version: String,

    /// Timestamp when the event occurred (ISO-8601 with microseconds).
    pub ts: DateTime<Utc>,

    /// Type of event being logged.
    pub event_type: AuditEventType,

    /// Unique ID for this invocation of pt-core.
    pub run_id: String,

    /// Session ID when a session exists (nullable).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,

    /// Host identifier for multi-host correlation.
    pub host_id: String,

    /// Human-readable description of the event.
    pub message: String,

    /// Event-specific structured details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,

    /// SHA-256 hash of the previous entry (hex string).
    /// First entry in a log file uses "genesis" or references the checkpoint hash.
    pub prev_hash: String,

    /// SHA-256 hash of this entry (excluding this field).
    /// Computed after all other fields are set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_hash: Option<String>,
}

impl AuditEntry {
    /// Create a new audit entry with the given context.
    pub fn new(
        ctx: &AuditContext,
        event_type: AuditEventType,
        message: impl Into<String>,
        prev_hash: impl Into<String>,
    ) -> Self {
        AuditEntry {
            schema_version: AUDIT_SCHEMA_VERSION.to_string(),
            ts: Utc::now(),
            event_type,
            run_id: ctx.run_id.clone(),
            session_id: ctx.session_id.clone(),
            host_id: ctx.host_id.clone(),
            message: message.into(),
            details: None,
            prev_hash: prev_hash.into(),
            entry_hash: None,
        }
    }

    /// Add structured details to the entry.
    pub fn with_details<T: Serialize>(mut self, details: &T) -> Self {
        self.details = serde_json::to_value(details).ok();
        self
    }

    /// Compute and set the entry hash.
    ///
    /// The hash is computed over the JSON representation of the entry
    /// with `entry_hash` set to None.
    pub fn compute_hash(&mut self) {
        // Temporarily clear entry_hash for hashing
        self.entry_hash = None;

        // Serialize to JSON for hashing
        let json = serde_json::to_string(self).unwrap_or_default();

        // Compute SHA-256
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(json.as_bytes());
        let result = hasher.finalize();

        self.entry_hash = Some(hex::encode(result));
    }

    /// Verify that the entry hash is correct.
    pub fn verify_hash(&self) -> bool {
        let stored_hash = match &self.entry_hash {
            Some(h) => h.clone(),
            None => return false,
        };

        // Create a copy without entry_hash for verification
        let mut verify_entry = self.clone();
        verify_entry.entry_hash = None;

        let json = serde_json::to_string(&verify_entry).unwrap_or_default();

        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(json.as_bytes());
        let computed = hex::encode(hasher.finalize());

        computed == stored_hash
    }

    /// Get the entry hash (for chaining).
    pub fn hash(&self) -> &str {
        self.entry_hash.as_deref().unwrap_or("invalid")
    }

    /// Serialize to a single JSON line.
    pub fn to_jsonl(&self) -> String {
        serde_json::to_string(self).unwrap_or_else(|_| {
            format!(
                r#"{{"error":"serialization_failed","event_type":"{}"}}"#,
                self.event_type
            )
        })
    }
}

/// Context for generating audit entries with consistent IDs.
#[derive(Debug, Clone)]
pub struct AuditContext {
    /// Unique ID for this invocation of pt-core.
    pub run_id: String,
    /// Session ID (if a session has been created).
    pub session_id: Option<String>,
    /// Host identifier.
    pub host_id: String,
}

impl AuditContext {
    /// Create a new audit context.
    pub fn new(run_id: impl Into<String>, host_id: impl Into<String>) -> Self {
        AuditContext {
            run_id: run_id.into(),
            session_id: None,
            host_id: host_id.into(),
        }
    }

    /// Set the session ID.
    pub fn with_session_id(mut self, session_id: impl Into<String>) -> Self {
        self.session_id = Some(session_id.into());
        self
    }
}

/// Details for scan events.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanDetails {
    /// Whether scan started or completed.
    pub phase: String,
    /// Number of processes scanned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_count: Option<u32>,
    /// Number of candidates identified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub candidate_count: Option<u32>,
    /// Scan mode (quick, deep).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_mode: Option<String>,
    /// Duration in milliseconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_ms: Option<u64>,
}

/// Details for recommendation events.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecommendDetails {
    /// Process ID.
    pub pid: u32,
    /// Process start ID for stable identity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_id: Option<String>,
    /// Command (may be redacted).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cmd: Option<String>,
    /// Recommended action (kill, review, spare).
    pub action: String,
    /// Posterior probability for the recommended class.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub posterior: Option<f64>,
    /// Classification (useful, useful_bad, abandoned, zombie).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classification: Option<String>,
    /// Rationale for the recommendation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rationale: Option<String>,
}

/// Details for action events.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionDetails {
    /// Process ID.
    pub pid: u32,
    /// Process start ID for stable identity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_id: Option<String>,
    /// Action type (kill, pause, renice, throttle, freeze).
    pub action: String,
    /// Whether the action succeeded.
    pub success: bool,
    /// Error message if action failed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// Signal sent (for kill actions).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signal: Option<String>,
    /// Whether this was a dry-run.
    #[serde(default)]
    pub dry_run: bool,
    /// Verification status after action.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified: Option<bool>,
    /// Additional context.
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub context: HashMap<String, serde_json::Value>,
}

/// Details for policy check events.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyCheckDetails {
    /// Policy rule that was checked.
    pub rule: String,
    /// Whether the check passed (action allowed).
    pub passed: bool,
    /// Process ID if applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pid: Option<u32>,
    /// Reason for the result.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// Which guardrail was triggered (if blocked).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guardrail: Option<String>,
}

/// Details for error events.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorDetails {
    /// Error category.
    pub category: String,
    /// Error message.
    pub message: String,
    /// Error code if applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// Context about where the error occurred.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    /// Whether the error is recoverable.
    #[serde(default)]
    pub recoverable: bool,
}

/// Details for checkpoint events (log rotation).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckpointDetails {
    /// Total entries in the log file up to this checkpoint.
    pub entry_count: u64,
    /// Full state hash (hash of all entry hashes concatenated).
    pub state_hash: String,
    /// Previous log file reference (for rotation).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev_log_file: Option<String>,
    /// Reason for checkpoint (rotation, shutdown, periodic).
    pub reason: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audit_entry_creation() {
        let ctx = AuditContext::new("run-12345", "host-abc");
        let entry = AuditEntry::new(&ctx, AuditEventType::Scan, "Scan started", "genesis");

        assert_eq!(entry.run_id, "run-12345");
        assert_eq!(entry.host_id, "host-abc");
        assert_eq!(entry.event_type, AuditEventType::Scan);
        assert_eq!(entry.prev_hash, "genesis");
    }

    #[test]
    fn test_audit_entry_hash_computation() {
        let ctx = AuditContext::new("run-12345", "host-abc");
        let mut entry = AuditEntry::new(&ctx, AuditEventType::Scan, "Scan started", "genesis");

        entry.compute_hash();

        assert!(entry.entry_hash.is_some());
        assert_eq!(entry.entry_hash.as_ref().unwrap().len(), 64); // SHA-256 = 64 hex chars
    }

    #[test]
    fn test_audit_entry_hash_verification() {
        let ctx = AuditContext::new("run-12345", "host-abc");
        let mut entry = AuditEntry::new(&ctx, AuditEventType::Scan, "Scan started", "genesis");

        entry.compute_hash();
        assert!(entry.verify_hash());

        // Tamper with the entry
        entry.message = "Tampered message".to_string();
        assert!(!entry.verify_hash());
    }

    #[test]
    fn test_audit_entry_with_details() {
        let ctx = AuditContext::new("run-12345", "host-abc");
        let details = ScanDetails {
            phase: "started".to_string(),
            process_count: Some(150),
            candidate_count: None,
            scan_mode: Some("quick".to_string()),
            duration_ms: None,
        };

        let entry = AuditEntry::new(&ctx, AuditEventType::Scan, "Scan started", "genesis")
            .with_details(&details);

        assert!(entry.details.is_some());
        let json = entry.to_jsonl();
        assert!(json.contains(r#""phase":"started""#));
        assert!(json.contains(r#""process_count":150"#));
    }

    #[test]
    fn test_audit_context_with_session() {
        let ctx = AuditContext::new("run-12345", "host-abc")
            .with_session_id("pt-20260115-143022-a7xq");

        assert_eq!(ctx.session_id, Some("pt-20260115-143022-a7xq".to_string()));

        let entry = AuditEntry::new(&ctx, AuditEventType::Action, "Kill executed", "prev");
        assert_eq!(entry.session_id, Some("pt-20260115-143022-a7xq".to_string()));
    }

    #[test]
    fn test_action_details_serialization() {
        let details = ActionDetails {
            pid: 1234,
            start_id: Some("boot-id:12345:1234".to_string()),
            action: "kill".to_string(),
            success: true,
            error: None,
            signal: Some("SIGTERM".to_string()),
            dry_run: false,
            verified: Some(true),
            context: HashMap::new(),
        };

        let json = serde_json::to_string(&details).unwrap();
        assert!(json.contains(r#""pid":1234"#));
        assert!(json.contains(r#""action":"kill""#));
        assert!(json.contains(r#""success":true"#));
    }

    #[test]
    fn test_event_type_display() {
        assert_eq!(AuditEventType::Scan.to_string(), "scan");
        assert_eq!(AuditEventType::Action.to_string(), "action");
        assert_eq!(AuditEventType::PolicyCheck.to_string(), "policy_check");
        assert_eq!(AuditEventType::Checkpoint.to_string(), "checkpoint");
    }
}
