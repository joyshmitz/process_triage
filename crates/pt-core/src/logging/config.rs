//! Logging configuration.
//!
//! Supports configuration via:
//! - Environment variables (PT_LOG, RUST_LOG)
//! - CLI flags (--log-level, --log-format)

use serde::{Deserialize, Serialize};

/// Log output format.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LogFormat {
    /// Human-readable console format (default).
    #[default]
    Human,
    /// Machine-parseable JSON lines.
    Jsonl,
}

impl std::str::FromStr for LogFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "human" | "console" | "pretty" => Ok(LogFormat::Human),
            "jsonl" | "json" | "structured" | "machine" => Ok(LogFormat::Jsonl),
            _ => Err(format!("unknown log format: {}", s)),
        }
    }
}

impl std::fmt::Display for LogFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogFormat::Human => write!(f, "human"),
            LogFormat::Jsonl => write!(f, "jsonl"),
        }
    }
}

/// Log level filter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    /// Most verbose.
    Trace,
    /// Debug information.
    Debug,
    /// Standard operational info (default).
    #[default]
    Info,
    /// Warnings only.
    Warn,
    /// Errors only.
    Error,
    /// Completely silent.
    Off,
}

impl std::str::FromStr for LogLevel {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "trace" => Ok(LogLevel::Trace),
            "debug" => Ok(LogLevel::Debug),
            "info" => Ok(LogLevel::Info),
            "warn" | "warning" => Ok(LogLevel::Warn),
            "error" => Ok(LogLevel::Error),
            "off" | "none" | "quiet" => Ok(LogLevel::Off),
            _ => Err(format!("unknown log level: {}", s)),
        }
    }
}

impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogLevel::Trace => write!(f, "trace"),
            LogLevel::Debug => write!(f, "debug"),
            LogLevel::Info => write!(f, "info"),
            LogLevel::Warn => write!(f, "warn"),
            LogLevel::Error => write!(f, "error"),
            LogLevel::Off => write!(f, "off"),
        }
    }
}

impl From<LogLevel> for tracing::Level {
    fn from(level: LogLevel) -> Self {
        match level {
            LogLevel::Trace => tracing::Level::TRACE,
            LogLevel::Debug => tracing::Level::DEBUG,
            LogLevel::Info => tracing::Level::INFO,
            LogLevel::Warn => tracing::Level::WARN,
            LogLevel::Error | LogLevel::Off => tracing::Level::ERROR,
        }
    }
}

impl From<LogLevel> for tracing_subscriber::filter::LevelFilter {
    fn from(level: LogLevel) -> Self {
        match level {
            LogLevel::Trace => tracing_subscriber::filter::LevelFilter::TRACE,
            LogLevel::Debug => tracing_subscriber::filter::LevelFilter::DEBUG,
            LogLevel::Info => tracing_subscriber::filter::LevelFilter::INFO,
            LogLevel::Warn => tracing_subscriber::filter::LevelFilter::WARN,
            LogLevel::Error => tracing_subscriber::filter::LevelFilter::ERROR,
            LogLevel::Off => tracing_subscriber::filter::LevelFilter::OFF,
        }
    }
}

/// Logging configuration.
#[derive(Debug, Clone)]
pub struct LogConfig {
    /// Output format.
    pub format: LogFormat,
    /// Minimum log level.
    pub level: LogLevel,
    /// Whether to include timestamps in human output.
    pub timestamps: bool,
    /// Whether to include file/line info in debug output.
    pub source_location: bool,
}

impl Default for LogConfig {
    fn default() -> Self {
        LogConfig {
            format: LogFormat::Human,
            level: LogLevel::Info,
            timestamps: true,
            source_location: false,
        }
    }
}

impl LogConfig {
    /// Create config from environment and CLI overrides.
    pub fn from_env(cli_level: Option<LogLevel>, cli_format: Option<LogFormat>) -> Self {
        let mut config = LogConfig::default();

        // Check environment variables (PT_LOG takes precedence over RUST_LOG)
        if let Ok(val) = std::env::var("PT_LOG") {
            if let Ok(level) = val.parse::<LogLevel>() {
                config.level = level;
            }
        } else if let Ok(val) = std::env::var("RUST_LOG") {
            // Simple parsing - just look for pt_core level
            if val.contains("trace") {
                config.level = LogLevel::Trace;
            } else if val.contains("debug") {
                config.level = LogLevel::Debug;
            } else if val.contains("warn") {
                config.level = LogLevel::Warn;
            } else if val.contains("error") {
                config.level = LogLevel::Error;
            }
        }

        // Check PT_LOG_FORMAT
        if let Ok(val) = std::env::var("PT_LOG_FORMAT") {
            if let Ok(format) = val.parse::<LogFormat>() {
                config.format = format;
            }
        }

        // CLI overrides take final precedence
        if let Some(level) = cli_level {
            config.level = level;
        }
        if let Some(format) = cli_format {
            config.format = format;
        }

        config
    }

    /// Set log format.
    pub fn with_format(mut self, format: LogFormat) -> Self {
        self.format = format;
        self
    }

    /// Set log level.
    pub fn with_level(mut self, level: LogLevel) -> Self {
        self.level = level;
        self
    }

    /// Enable timestamps in human output.
    pub fn with_timestamps(mut self, enabled: bool) -> Self {
        self.timestamps = enabled;
        self
    }

    /// Enable source location in debug output.
    pub fn with_source_location(mut self, enabled: bool) -> Self {
        self.source_location = enabled;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_format_parse() {
        assert_eq!("human".parse::<LogFormat>().unwrap(), LogFormat::Human);
        assert_eq!("jsonl".parse::<LogFormat>().unwrap(), LogFormat::Jsonl);
        assert_eq!("json".parse::<LogFormat>().unwrap(), LogFormat::Jsonl);
        assert_eq!("console".parse::<LogFormat>().unwrap(), LogFormat::Human);
    }

    #[test]
    fn test_log_level_parse() {
        assert_eq!("trace".parse::<LogLevel>().unwrap(), LogLevel::Trace);
        assert_eq!("debug".parse::<LogLevel>().unwrap(), LogLevel::Debug);
        assert_eq!("info".parse::<LogLevel>().unwrap(), LogLevel::Info);
        assert_eq!("warn".parse::<LogLevel>().unwrap(), LogLevel::Warn);
        assert_eq!("warning".parse::<LogLevel>().unwrap(), LogLevel::Warn);
        assert_eq!("error".parse::<LogLevel>().unwrap(), LogLevel::Error);
        assert_eq!("off".parse::<LogLevel>().unwrap(), LogLevel::Off);
        assert_eq!("quiet".parse::<LogLevel>().unwrap(), LogLevel::Off);
    }

    #[test]
    fn test_log_format_display() {
        assert_eq!(LogFormat::Human.to_string(), "human");
        assert_eq!(LogFormat::Jsonl.to_string(), "jsonl");
    }

    #[test]
    fn test_log_level_display() {
        assert_eq!(LogLevel::Info.to_string(), "info");
        assert_eq!(LogLevel::Debug.to_string(), "debug");
    }

    #[test]
    fn test_log_config_default() {
        let config = LogConfig::default();
        assert_eq!(config.format, LogFormat::Human);
        assert_eq!(config.level, LogLevel::Info);
        assert!(config.timestamps);
        assert!(!config.source_location);
    }

    #[test]
    fn test_log_config_builder() {
        let config = LogConfig::default()
            .with_format(LogFormat::Jsonl)
            .with_level(LogLevel::Debug)
            .with_timestamps(false);

        assert_eq!(config.format, LogFormat::Jsonl);
        assert_eq!(config.level, LogLevel::Debug);
        assert!(!config.timestamps);
    }
}
