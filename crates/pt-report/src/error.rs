//! Error types for report generation.

use thiserror::Error;

/// Result type for report operations.
pub type Result<T> = std::result::Result<T, ReportError>;

/// Errors that can occur during report generation.
#[derive(Error, Debug)]
pub enum ReportError {
    /// Bundle read error.
    #[error("failed to read bundle: {0}")]
    BundleError(#[from] pt_bundle::BundleError),

    /// JSON serialization/deserialization error.
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),

    /// Template rendering error.
    #[error("template error: {0}")]
    TemplateError(String),

    /// IO error.
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    /// Asset fetch error (embed mode).
    #[error("failed to fetch asset '{url}': {reason}")]
    AssetFetchError { url: String, reason: String },

    /// Asset size limit exceeded.
    #[error("embedded assets exceed size limit ({size_mb:.1} MB > {limit_mb} MB)")]
    AssetSizeLimitExceeded { size_mb: f64, limit_mb: u64 },

    /// Missing required data.
    #[error("missing required data: {0}")]
    MissingData(String),

    /// Invalid configuration.
    #[error("invalid configuration: {0}")]
    InvalidConfig(String),
}

impl From<askama::Error> for ReportError {
    fn from(err: askama::Error) -> Self {
        ReportError::TemplateError(err.to_string())
    }
}
