//! Error types for the redaction engine.

use thiserror::Error;

/// Result type for redaction operations.
pub type Result<T> = std::result::Result<T, RedactionError>;

/// Errors that can occur during redaction.
#[derive(Error, Debug)]
pub enum RedactionError {
    /// Failed to load or parse the redaction policy.
    #[error("policy error: {0}")]
    PolicyError(String),

    /// Failed to load or generate the redaction key.
    #[error("key error: {0}")]
    KeyError(String),

    /// Failed to compile a regex pattern.
    #[error("pattern error: {0}")]
    PatternError(String),

    /// I/O error during key or policy file operations.
    #[error("io error: {0}")]
    IoError(#[from] std::io::Error),

    /// JSON parsing error.
    #[error("json error: {0}")]
    JsonError(#[from] serde_json::Error),

    /// Internal error that should never expose raw data.
    /// The message is sanitized to prevent secret leakage.
    #[error("internal error (details redacted for safety)")]
    InternalError,
}

impl RedactionError {
    /// Create an internal error, ensuring no sensitive data is exposed.
    /// This is the fail-closed guarantee.
    pub fn internal() -> Self {
        RedactionError::InternalError
    }
}
