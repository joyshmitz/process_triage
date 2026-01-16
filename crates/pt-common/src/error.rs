//! Error types for Process Triage.
//!
//! This module provides structured error handling with:
//! - Stable error codes for machine parsing
//! - Category classification for error grouping
//! - Recoverability hints for automation
//! - Remediation suggestions for humans
//! - Suggested actions for agents
//!
//! # Human-Facing Output
//!
//! Errors can be formatted for human consumption with headline, reason, and fix:
//! ```text
//! ✗ Configuration Error
//!   Reason: Invalid priors file format
//!   Fix: Run 'pt check --priors' to validate, or reset with 'pt config reset'
//! ```
//!
//! # Agent-Facing Output
//!
//! Errors serialize to structured JSON:
//! ```json
//! {
//!   "code": 11,
//!   "category": "config",
//!   "message": "invalid priors file: parse error at line 5",
//!   "recoverable": true,
//!   "suggested_action": "reset_config",
//!   "context": { "file": "priors.json", "line": 5 }
//! }
//! ```

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

/// Result type alias for Process Triage operations.
pub type Result<T> = std::result::Result<T, Error>;

/// Error categories for grouping related errors.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ErrorCategory {
    /// Configuration file errors (priors, policy, schema).
    Config,
    /// Process scanning and collection errors.
    Collection,
    /// Bayesian inference and numerical errors.
    Inference,
    /// Action execution errors (kill, pause, etc.).
    Action,
    /// Session management errors.
    Session,
    /// File I/O and serialization errors.
    Io,
    /// Platform compatibility errors.
    Platform,
}

impl std::fmt::Display for ErrorCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorCategory::Config => write!(f, "config"),
            ErrorCategory::Collection => write!(f, "collection"),
            ErrorCategory::Inference => write!(f, "inference"),
            ErrorCategory::Action => write!(f, "action"),
            ErrorCategory::Session => write!(f, "session"),
            ErrorCategory::Io => write!(f, "io"),
            ErrorCategory::Platform => write!(f, "platform"),
        }
    }
}

/// Suggested actions for agents to take in response to errors.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SuggestedAction {
    /// Retry the operation (possibly with backoff).
    Retry,
    /// Reset configuration to defaults.
    ResetConfig,
    /// Run validation/check command.
    RunCheck,
    /// Refresh/rescan process list.
    Rescan,
    /// Wait for a resource to become available.
    Wait,
    /// Request elevated privileges.
    Elevate,
    /// Skip this item and continue.
    Skip,
    /// Abort the operation.
    Abort,
    /// Manual intervention required.
    ManualIntervention,
    /// No action needed (informational).
    None,
}

impl std::fmt::Display for SuggestedAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SuggestedAction::Retry => write!(f, "retry"),
            SuggestedAction::ResetConfig => write!(f, "reset_config"),
            SuggestedAction::RunCheck => write!(f, "run_check"),
            SuggestedAction::Rescan => write!(f, "rescan"),
            SuggestedAction::Wait => write!(f, "wait"),
            SuggestedAction::Elevate => write!(f, "elevate"),
            SuggestedAction::Skip => write!(f, "skip"),
            SuggestedAction::Abort => write!(f, "abort"),
            SuggestedAction::ManualIntervention => write!(f, "manual_intervention"),
            SuggestedAction::None => write!(f, "none"),
        }
    }
}

/// Unified error type for Process Triage.
#[derive(Error, Debug)]
pub enum Error {
    // Configuration errors (10-19)
    #[error("configuration error: {0}")]
    Config(String),

    #[error("invalid priors file: {0}")]
    InvalidPriors(String),

    #[error("invalid policy file: {0}")]
    InvalidPolicy(String),

    #[error("schema validation failed: {0}")]
    SchemaValidation(String),

    // Collection errors (20-29)
    #[error("process collection failed: {0}")]
    Collection(String),

    #[error("process {pid} not found")]
    ProcessNotFound { pid: u32 },

    #[error("process identity mismatch: expected start_id={expected}, got {actual}")]
    IdentityMismatch { expected: String, actual: String },

    #[error("permission denied accessing process {pid}")]
    PermissionDenied { pid: u32 },

    // Inference errors (30-39)
    #[error("inference failed: {0}")]
    Inference(String),

    #[error("numerical instability detected: {0}")]
    NumericalInstability(String),

    // Action errors (40-49)
    #[error("action execution failed: {0}")]
    ActionFailed(String),

    #[error("action blocked by policy: {0}")]
    PolicyBlocked(String),

    #[error("action timeout after {seconds}s")]
    ActionTimeout { seconds: u64 },

    // Session errors (50-59)
    #[error("session not found: {session_id}")]
    SessionNotFound { session_id: String },

    #[error("session expired: {session_id}")]
    SessionExpired { session_id: String },

    #[error("session corrupted: {0}")]
    SessionCorrupted(String),

    // I/O errors (60-69)
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON serialization error: {0}")]
    Json(#[from] serde_json::Error),

    // Platform errors (70-79)
    #[error("unsupported platform: {0}")]
    UnsupportedPlatform(String),

    #[error("capability not available: {0}")]
    CapabilityMissing(String),
}

impl Error {
    /// Returns the error code for this error type.
    ///
    /// Error codes are stable and grouped by category:
    /// - 10-19: Configuration errors
    /// - 20-29: Collection errors
    /// - 30-39: Inference errors
    /// - 40-49: Action errors
    /// - 50-59: Session errors
    /// - 60-69: I/O errors
    /// - 70-79: Platform errors
    pub fn code(&self) -> u32 {
        match self {
            Error::Config(_) => 10,
            Error::InvalidPriors(_) => 11,
            Error::InvalidPolicy(_) => 12,
            Error::SchemaValidation(_) => 13,
            Error::Collection(_) => 20,
            Error::ProcessNotFound { .. } => 21,
            Error::IdentityMismatch { .. } => 22,
            Error::PermissionDenied { .. } => 23,
            Error::Inference(_) => 30,
            Error::NumericalInstability(_) => 31,
            Error::ActionFailed(_) => 40,
            Error::PolicyBlocked(_) => 41,
            Error::ActionTimeout { .. } => 42,
            Error::SessionNotFound { .. } => 50,
            Error::SessionExpired { .. } => 51,
            Error::SessionCorrupted(_) => 52,
            Error::Io(_) => 60,
            Error::Json(_) => 61,
            Error::UnsupportedPlatform(_) => 70,
            Error::CapabilityMissing(_) => 71,
        }
    }

    /// Returns the error category for grouping and filtering.
    pub fn category(&self) -> ErrorCategory {
        match self {
            Error::Config(_)
            | Error::InvalidPriors(_)
            | Error::InvalidPolicy(_)
            | Error::SchemaValidation(_) => ErrorCategory::Config,

            Error::Collection(_)
            | Error::ProcessNotFound { .. }
            | Error::IdentityMismatch { .. }
            | Error::PermissionDenied { .. } => ErrorCategory::Collection,

            Error::Inference(_) | Error::NumericalInstability(_) => ErrorCategory::Inference,

            Error::ActionFailed(_) | Error::PolicyBlocked(_) | Error::ActionTimeout { .. } => {
                ErrorCategory::Action
            }

            Error::SessionNotFound { .. }
            | Error::SessionExpired { .. }
            | Error::SessionCorrupted(_) => ErrorCategory::Session,

            Error::Io(_) | Error::Json(_) => ErrorCategory::Io,

            Error::UnsupportedPlatform(_) | Error::CapabilityMissing(_) => ErrorCategory::Platform,
        }
    }

    /// Returns whether this error is potentially recoverable.
    ///
    /// Recoverable errors may be resolved by:
    /// - Retrying with a delay
    /// - Refreshing stale data
    /// - Resetting configuration
    /// - Requesting elevated privileges
    pub fn is_recoverable(&self) -> bool {
        match self {
            // Config errors: recoverable by fixing/resetting config
            Error::Config(_) => true,
            Error::InvalidPriors(_) => true,
            Error::InvalidPolicy(_) => true,
            Error::SchemaValidation(_) => true,

            // Collection: mostly recoverable (transient)
            Error::Collection(_) => true,
            Error::ProcessNotFound { .. } => false, // Process is gone
            Error::IdentityMismatch { .. } => false, // TOCTOU failure
            Error::PermissionDenied { .. } => true, // Can elevate

            // Inference: may be recoverable with different inputs
            Error::Inference(_) => true,
            Error::NumericalInstability(_) => true,

            // Action: depends on cause
            Error::ActionFailed(_) => true, // Retry possible
            Error::PolicyBlocked(_) => false, // Policy is intentional
            Error::ActionTimeout { .. } => true, // Retry with longer timeout

            // Session: mostly recoverable
            Error::SessionNotFound { .. } => false, // Session is gone
            Error::SessionExpired { .. } => true, // Can create new session
            Error::SessionCorrupted(_) => true, // Can recreate

            // I/O: often transient
            Error::Io(_) => true,
            Error::Json(_) => true,

            // Platform: not recoverable at runtime
            Error::UnsupportedPlatform(_) => false,
            Error::CapabilityMissing(_) => true, // Can install/configure
        }
    }

    /// Returns the suggested action for agents.
    pub fn suggested_action(&self) -> SuggestedAction {
        match self {
            Error::Config(_) => SuggestedAction::RunCheck,
            Error::InvalidPriors(_) => SuggestedAction::ResetConfig,
            Error::InvalidPolicy(_) => SuggestedAction::ResetConfig,
            Error::SchemaValidation(_) => SuggestedAction::RunCheck,

            Error::Collection(_) => SuggestedAction::Rescan,
            Error::ProcessNotFound { .. } => SuggestedAction::Skip,
            Error::IdentityMismatch { .. } => SuggestedAction::Rescan,
            Error::PermissionDenied { .. } => SuggestedAction::Elevate,

            Error::Inference(_) => SuggestedAction::Retry,
            Error::NumericalInstability(_) => SuggestedAction::Skip,

            Error::ActionFailed(_) => SuggestedAction::Retry,
            Error::PolicyBlocked(_) => SuggestedAction::Skip,
            Error::ActionTimeout { .. } => SuggestedAction::Retry,

            Error::SessionNotFound { .. } => SuggestedAction::Abort,
            Error::SessionExpired { .. } => SuggestedAction::Rescan,
            Error::SessionCorrupted(_) => SuggestedAction::Rescan,

            Error::Io(_) => SuggestedAction::Retry,
            Error::Json(_) => SuggestedAction::ManualIntervention,

            Error::UnsupportedPlatform(_) => SuggestedAction::Abort,
            Error::CapabilityMissing(_) => SuggestedAction::ManualIntervention,
        }
    }

    /// Returns a human-readable remediation hint.
    pub fn remediation(&self) -> &'static str {
        match self {
            Error::Config(_) => {
                "Run 'pt check' to validate configuration, or check syntax in config files."
            }
            Error::InvalidPriors(_) => {
                "Run 'pt check --priors' to validate, or reset with 'pt agent export-priors --default | pt agent import-priors -'."
            }
            Error::InvalidPolicy(_) => {
                "Run 'pt check --policy' to validate, or restore policy.json from backup."
            }
            Error::SchemaValidation(_) => {
                "Ensure configuration files match the expected schema version. See docs/PRIORS_SCHEMA.md."
            }

            Error::Collection(_) => {
                "Retry the scan. If persistent, check /proc permissions and system load."
            }
            Error::ProcessNotFound { .. } => {
                "The process exited before action could complete. This is normal for short-lived processes."
            }
            Error::IdentityMismatch { .. } => {
                "PID was reused between plan and execution. Run a fresh scan to get current process identities."
            }
            Error::PermissionDenied { .. } => {
                "Run with elevated privileges: 'sudo pt' or set CAP_KILL capability."
            }

            Error::Inference(_) => {
                "Retry with '--deep' for more evidence. If persistent, report as a bug with session bundle."
            }
            Error::NumericalInstability(_) => {
                "Internal numerical issue. Skip this process and report with 'pt bundle create --session <id>'."
            }

            Error::ActionFailed(_) => {
                "Retry the action. Check if the process is in D-state (uninterruptible sleep) or protected."
            }
            Error::PolicyBlocked(_) => {
                "This action is blocked by policy. Use '--force' to override (if allowed) or update policy.json."
            }
            Error::ActionTimeout { .. } => {
                "Process did not respond to signal in time. Retry with '--timeout' to increase wait time."
            }

            Error::SessionNotFound { .. } => {
                "The session has been deleted or never existed. List sessions with 'pt agent sessions list'."
            }
            Error::SessionExpired { .. } => {
                "Session data is stale. Start a new session with 'pt agent snapshot'."
            }
            Error::SessionCorrupted(_) => {
                "Session data is corrupted. Delete and recreate with 'pt agent sessions delete <id>'."
            }

            Error::Io(_) => {
                "Check disk space, permissions, and that config directories exist. Retry the operation."
            }
            Error::Json(_) => {
                "Invalid JSON in file. Check syntax with 'cat <file> | jq .' or restore from backup."
            }

            Error::UnsupportedPlatform(_) => {
                "This feature is not available on your platform. See README for supported platforms."
            }
            Error::CapabilityMissing(_) => {
                "Required capability is missing. Install the dependency or run with elevated privileges."
            }
        }
    }

    /// Returns a short headline for human-readable output.
    pub fn headline(&self) -> &'static str {
        match self {
            Error::Config(_) => "Configuration Error",
            Error::InvalidPriors(_) => "Invalid Priors Configuration",
            Error::InvalidPolicy(_) => "Invalid Policy Configuration",
            Error::SchemaValidation(_) => "Schema Validation Failed",

            Error::Collection(_) => "Process Collection Error",
            Error::ProcessNotFound { .. } => "Process Not Found",
            Error::IdentityMismatch { .. } => "Process Identity Mismatch",
            Error::PermissionDenied { .. } => "Permission Denied",

            Error::Inference(_) => "Inference Error",
            Error::NumericalInstability(_) => "Numerical Instability",

            Error::ActionFailed(_) => "Action Failed",
            Error::PolicyBlocked(_) => "Action Blocked by Policy",
            Error::ActionTimeout { .. } => "Action Timeout",

            Error::SessionNotFound { .. } => "Session Not Found",
            Error::SessionExpired { .. } => "Session Expired",
            Error::SessionCorrupted(_) => "Session Corrupted",

            Error::Io(_) => "I/O Error",
            Error::Json(_) => "JSON Parse Error",

            Error::UnsupportedPlatform(_) => "Unsupported Platform",
            Error::CapabilityMissing(_) => "Missing Capability",
        }
    }
}

/// Structured error response for JSON output.
///
/// Used by agent/robot modes for machine-parseable error reporting.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructuredError {
    /// Stable error code.
    pub code: u32,

    /// Error category for grouping.
    pub category: ErrorCategory,

    /// Human-readable error message.
    pub message: String,

    /// Whether the error is potentially recoverable.
    pub recoverable: bool,

    /// Suggested action for agents.
    pub suggested_action: SuggestedAction,

    /// Additional structured context (e.g., pid, file path).
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub context: HashMap<String, serde_json::Value>,
}

impl From<&Error> for StructuredError {
    fn from(err: &Error) -> Self {
        let mut context = HashMap::new();

        // Add error-specific context
        match err {
            Error::ProcessNotFound { pid } => {
                context.insert("pid".to_string(), serde_json::json!(pid));
            }
            Error::PermissionDenied { pid } => {
                context.insert("pid".to_string(), serde_json::json!(pid));
            }
            Error::IdentityMismatch { expected, actual } => {
                context.insert("expected_start_id".to_string(), serde_json::json!(expected));
                context.insert("actual_start_id".to_string(), serde_json::json!(actual));
            }
            Error::ActionTimeout { seconds } => {
                context.insert("timeout_seconds".to_string(), serde_json::json!(seconds));
            }
            Error::SessionNotFound { session_id } => {
                context.insert("session_id".to_string(), serde_json::json!(session_id));
            }
            Error::SessionExpired { session_id } => {
                context.insert("session_id".to_string(), serde_json::json!(session_id));
            }
            _ => {}
        }

        StructuredError {
            code: err.code(),
            category: err.category(),
            message: err.to_string(),
            recoverable: err.is_recoverable(),
            suggested_action: err.suggested_action(),
            context,
        }
    }
}

impl StructuredError {
    /// Add additional context to the error.
    pub fn with_context(mut self, key: impl Into<String>, value: impl Serialize) -> Self {
        if let Ok(v) = serde_json::to_value(value) {
            self.context.insert(key.into(), v);
        }
        self
    }

    /// Serialize to JSON string.
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap_or_else(|_| {
            format!(r#"{{"code":{},"error":"serialization_failed"}}"#, self.code)
        })
    }

    /// Serialize to pretty JSON string.
    pub fn to_json_pretty(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_else(|_| self.to_json())
    }
}

/// Result of a batch operation that may have partial success.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchResult<T> {
    /// Successfully completed items.
    pub succeeded: Vec<T>,

    /// Failed items with their errors.
    pub failed: Vec<BatchError>,

    /// Summary statistics.
    pub summary: BatchSummary,
}

/// A single error in a batch operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchError {
    /// Index or identifier of the failed item.
    pub item_id: String,

    /// The structured error.
    pub error: StructuredError,
}

/// Summary of batch operation results.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchSummary {
    /// Total items attempted.
    pub total: usize,

    /// Number of successful items.
    pub succeeded: usize,

    /// Number of failed items.
    pub failed: usize,

    /// Whether all items succeeded.
    pub all_succeeded: bool,

    /// Whether any items succeeded.
    pub any_succeeded: bool,
}

impl<T> BatchResult<T> {
    /// Create a new batch result from succeeded and failed items.
    pub fn new(succeeded: Vec<T>, failed: Vec<BatchError>) -> Self {
        let total = succeeded.len() + failed.len();
        let succeeded_count = succeeded.len();
        let failed_count = failed.len();

        BatchResult {
            succeeded,
            failed,
            summary: BatchSummary {
                total,
                succeeded: succeeded_count,
                failed: failed_count,
                all_succeeded: failed_count == 0,
                any_succeeded: succeeded_count > 0,
            },
        }
    }

    /// Create a fully successful batch result.
    pub fn all_success(items: Vec<T>) -> Self {
        Self::new(items, Vec::new())
    }

    /// Create a fully failed batch result.
    pub fn all_failed(errors: Vec<BatchError>) -> Self {
        Self::new(Vec::new(), errors)
    }

    /// Add a failure to the batch result.
    pub fn add_failure(&mut self, item_id: impl Into<String>, error: &Error) {
        self.failed.push(BatchError {
            item_id: item_id.into(),
            error: StructuredError::from(error),
        });
        self.summary.failed += 1;
        self.summary.total += 1;
        self.summary.all_succeeded = false;
    }

    /// Add a success to the batch result.
    pub fn add_success(&mut self, item: T) {
        self.succeeded.push(item);
        self.summary.succeeded += 1;
        self.summary.total += 1;
        self.summary.any_succeeded = true;
    }
}

impl<T> Default for BatchResult<T> {
    fn default() -> Self {
        Self::new(Vec::new(), Vec::new())
    }
}

/// Format an error for human-readable stderr output.
///
/// Output format:
/// ```text
/// ✗ [Headline]
///   Reason: [Error message]
///   Fix: [Remediation hint]
/// ```
pub fn format_error_human(err: &Error, use_color: bool) -> String {
    let (red, cyan, reset) = if use_color {
        ("\x1b[31m", "\x1b[36m", "\x1b[0m")
    } else {
        ("", "", "")
    };

    format!(
        "{red}✗{reset} {headline}\n  Reason: {message}\n  {cyan}Fix:{reset} {remediation}",
        red = red,
        cyan = cyan,
        reset = reset,
        headline = err.headline(),
        message = err,
        remediation = err.remediation()
    )
}

/// Format a batch result for human-readable stderr output.
pub fn format_batch_human<T: std::fmt::Display>(result: &BatchResult<T>, use_color: bool) -> String {
    let (green, red, reset) = if use_color {
        ("\x1b[32m", "\x1b[31m", "\x1b[0m")
    } else {
        ("", "", "")
    };

    let mut output = String::new();

    // Summary line
    if result.summary.all_succeeded {
        output.push_str(&format!(
            "{green}✓{reset} All {} items completed successfully\n",
            result.summary.total, green = green, reset = reset
        ));
    } else if result.summary.any_succeeded {
        output.push_str(&format!(
            "Partial success: {} of {} items completed\n",
            result.summary.succeeded, result.summary.total
        ));
    } else {
        output.push_str(&format!(
            "{red}✗{reset} All {} items failed\n",
            result.summary.total, red = red, reset = reset
        ));
    }

    // List failures
    if !result.failed.is_empty() {
        output.push_str("\nErrors:\n");
        for batch_err in &result.failed {
            output.push_str(&format!(
                "  {red}✗{reset} {}: {}\n",
                batch_err.item_id,
                batch_err.error.message,
                red = red,
                reset = reset
            ));
        }
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_code() {
        assert_eq!(Error::Config("test".into()).code(), 10);
        assert_eq!(Error::ProcessNotFound { pid: 123 }.code(), 21);
        assert_eq!(Error::ActionTimeout { seconds: 30 }.code(), 42);
    }

    #[test]
    fn test_error_category() {
        assert_eq!(Error::Config("test".into()).category(), ErrorCategory::Config);
        assert_eq!(Error::ProcessNotFound { pid: 123 }.category(), ErrorCategory::Collection);
        assert_eq!(Error::ActionFailed("test".into()).category(), ErrorCategory::Action);
    }

    #[test]
    fn test_error_recoverable() {
        assert!(Error::Config("test".into()).is_recoverable());
        assert!(!Error::ProcessNotFound { pid: 123 }.is_recoverable());
        assert!(!Error::PolicyBlocked("test".into()).is_recoverable());
        assert!(Error::ActionTimeout { seconds: 30 }.is_recoverable());
    }

    #[test]
    fn test_suggested_action() {
        assert_eq!(
            Error::PermissionDenied { pid: 123 }.suggested_action(),
            SuggestedAction::Elevate
        );
        assert_eq!(
            Error::ProcessNotFound { pid: 123 }.suggested_action(),
            SuggestedAction::Skip
        );
        assert_eq!(
            Error::InvalidPriors("test".into()).suggested_action(),
            SuggestedAction::ResetConfig
        );
    }

    #[test]
    fn test_structured_error_from_error() {
        let err = Error::ProcessNotFound { pid: 12345 };
        let structured = StructuredError::from(&err);

        assert_eq!(structured.code, 21);
        assert_eq!(structured.category, ErrorCategory::Collection);
        assert!(!structured.recoverable);
        assert_eq!(structured.suggested_action, SuggestedAction::Skip);
        assert_eq!(
            structured.context.get("pid"),
            Some(&serde_json::json!(12345))
        );
    }

    #[test]
    fn test_structured_error_json() {
        let err = Error::ActionTimeout { seconds: 30 };
        let structured = StructuredError::from(&err);
        let json = structured.to_json();

        assert!(json.contains(r#""code":42"#));
        assert!(json.contains(r#""category":"action""#));
        assert!(json.contains(r#""recoverable":true"#));
        assert!(json.contains(r#""suggested_action":"retry""#));
    }

    #[test]
    fn test_batch_result() {
        let mut batch: BatchResult<String> = BatchResult::default();

        batch.add_success("item1".to_string());
        batch.add_success("item2".to_string());
        batch.add_failure("item3", &Error::ProcessNotFound { pid: 123 });

        assert_eq!(batch.summary.total, 3);
        assert_eq!(batch.summary.succeeded, 2);
        assert_eq!(batch.summary.failed, 1);
        assert!(!batch.summary.all_succeeded);
        assert!(batch.summary.any_succeeded);
    }

    #[test]
    fn test_format_error_human() {
        let err = Error::PermissionDenied { pid: 1234 };
        let formatted = format_error_human(&err, false);

        assert!(formatted.contains("Permission Denied"));
        assert!(formatted.contains("permission denied accessing process 1234"));
        assert!(formatted.contains("sudo pt"));
    }

    #[test]
    fn test_error_category_display() {
        assert_eq!(ErrorCategory::Config.to_string(), "config");
        assert_eq!(ErrorCategory::Action.to_string(), "action");
    }

    #[test]
    fn test_suggested_action_display() {
        assert_eq!(SuggestedAction::Retry.to_string(), "retry");
        assert_eq!(SuggestedAction::ResetConfig.to_string(), "reset_config");
    }
}
