//! Structured event definitions for logging.
//!
//! Events follow a consistent schema for machine-parseable JSONL output.
//! All events include correlation IDs (run_id, session_id) and stage.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Log levels for events.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Level {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl From<tracing::Level> for Level {
    fn from(level: tracing::Level) -> Self {
        match level {
            tracing::Level::TRACE => Level::Trace,
            tracing::Level::DEBUG => Level::Debug,
            tracing::Level::INFO => Level::Info,
            tracing::Level::WARN => Level::Warn,
            tracing::Level::ERROR => Level::Error,
        }
    }
}

/// Processing stages in the pt-core pipeline.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Stage {
    /// Initial startup and configuration.
    Init,
    /// Process scanning and sampling.
    Scan,
    /// Bayesian inference over process states.
    Infer,
    /// Decision-theoretic action selection.
    Decide,
    /// Interactive UI confirmation.
    Ui,
    /// Action execution (kill, pause, etc.).
    Apply,
    /// Post-action verification.
    Verify,
    /// Report generation.
    Report,
    /// Session bundling and export.
    Bundle,
    /// Background daemon monitoring.
    Daemon,
}

impl std::fmt::Display for Stage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Stage::Init => "init",
            Stage::Scan => "scan",
            Stage::Infer => "infer",
            Stage::Decide => "decide",
            Stage::Ui => "ui",
            Stage::Apply => "apply",
            Stage::Verify => "verify",
            Stage::Report => "report",
            Stage::Bundle => "bundle",
            Stage::Daemon => "daemon",
        };
        write!(f, "{}", s)
    }
}

/// Standard event names used in logging.
pub mod event_names {
    // Run lifecycle
    pub const RUN_STARTED: &str = "run.started";
    pub const RUN_FINISHED: &str = "run.finished";

    // Scan stage
    pub const SCAN_STARTED: &str = "scan.started";
    pub const SCAN_SAMPLED: &str = "scan.sampled";
    pub const SCAN_FINISHED: &str = "scan.finished";

    // Infer stage
    pub const INFER_STARTED: &str = "infer.started";
    pub const INFER_PROC_DONE: &str = "infer.proc_done";
    pub const INFER_FINISHED: &str = "infer.finished";

    // Decide stage
    pub const DECIDE_STARTED: &str = "decide.started";
    pub const DECIDE_BLOCKED_BY_POLICY: &str = "decide.blocked_by_policy";
    pub const DECIDE_RECOMMENDED_ACTION: &str = "decide.recommended_action";
    pub const DECIDE_FINISHED: &str = "decide.finished";

    // UI stage
    pub const UI_STARTED: &str = "ui.started";
    pub const UI_USER_INPUT: &str = "ui.user_input";
    pub const UI_FINISHED: &str = "ui.finished";

    // Apply stage
    pub const APPLY_STARTED: &str = "apply.started";
    pub const APPLY_INTENT_LOGGED: &str = "apply.intent_logged";
    pub const APPLY_ACTION_ATTEMPTED: &str = "apply.action_attempted";
    pub const APPLY_ACTION_RESULT: &str = "apply.action_result";
    pub const APPLY_FINISHED: &str = "apply.finished";

    // Verify stage
    pub const VERIFY_STARTED: &str = "verify.started";
    pub const VERIFY_RESULT: &str = "verify.result";
    pub const VERIFY_FINISHED: &str = "verify.finished";

    // Config/init events
    pub const CONFIG_LOADED: &str = "config.loaded";
    pub const CONFIG_DEFAULT_USED: &str = "config.default_used";
    pub const CONFIG_ERROR: &str = "config.error";

    // Error events
    pub const INTERNAL_ERROR: &str = "internal_error";
}

/// A structured log event for JSONL output.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEvent {
    /// Timestamp when the event occurred.
    pub ts: DateTime<Utc>,

    /// Log level.
    pub level: Level,

    /// Event name (e.g., "run.started", "scan.finished").
    pub event: String,

    /// Unique ID for this invocation of pt-core.
    pub run_id: String,

    /// Session ID when a session exists (nullable).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,

    /// Current processing stage.
    pub stage: Stage,

    /// Host identifier.
    pub host_id: String,

    /// Human-readable message.
    pub message: String,

    /// Additional structured fields (stable keys).
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub fields: HashMap<String, serde_json::Value>,

    /// Process ID when event concerns a specific process.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pid: Option<u32>,

    /// Process start_id when event concerns a specific process.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_id: Option<String>,
}

impl LogEvent {
    /// Create a new log event with required fields.
    pub fn new(
        level: Level,
        event: impl Into<String>,
        run_id: impl Into<String>,
        host_id: impl Into<String>,
        stage: Stage,
        message: impl Into<String>,
    ) -> Self {
        LogEvent {
            ts: Utc::now(),
            level,
            event: event.into(),
            run_id: run_id.into(),
            session_id: None,
            stage,
            host_id: host_id.into(),
            message: message.into(),
            fields: HashMap::new(),
            pid: None,
            start_id: None,
        }
    }

    /// Set the session ID.
    pub fn with_session_id(mut self, session_id: impl Into<String>) -> Self {
        self.session_id = Some(session_id.into());
        self
    }

    /// Add a field to the event.
    pub fn with_field(mut self, key: impl Into<String>, value: impl Serialize) -> Self {
        if let Ok(v) = serde_json::to_value(value) {
            self.fields.insert(key.into(), v);
        }
        self
    }

    /// Set process context.
    pub fn with_process(mut self, pid: u32, start_id: impl Into<String>) -> Self {
        self.pid = Some(pid);
        self.start_id = Some(start_id.into());
        self
    }

    /// Serialize to a single JSON line.
    pub fn to_jsonl(&self) -> String {
        serde_json::to_string(self).unwrap_or_else(|_| {
            format!(
                r#"{{"error":"serialization_failed","event":"{}"}}"#,
                self.event
            )
        })
    }
}

/// Context for generating log events with consistent run/session IDs.
#[derive(Debug, Clone)]
pub struct LogContext {
    /// Unique ID for this invocation.
    pub run_id: String,
    /// Session ID (if a session has been created).
    pub session_id: Option<String>,
    /// Host identifier.
    pub host_id: String,
}

impl LogContext {
    /// Create a new log context.
    pub fn new(run_id: impl Into<String>, host_id: impl Into<String>) -> Self {
        LogContext {
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

    /// Create an event with this context.
    pub fn event(
        &self,
        level: Level,
        event: impl Into<String>,
        stage: Stage,
        message: impl Into<String>,
    ) -> LogEvent {
        let mut e = LogEvent::new(level, event, &self.run_id, &self.host_id, stage, message);
        if let Some(ref sid) = self.session_id {
            e.session_id = Some(sid.clone());
        }
        e
    }

    /// Shortcut for info-level event.
    pub fn info(
        &self,
        event: impl Into<String>,
        stage: Stage,
        message: impl Into<String>,
    ) -> LogEvent {
        self.event(Level::Info, event, stage, message)
    }

    /// Shortcut for debug-level event.
    pub fn debug(
        &self,
        event: impl Into<String>,
        stage: Stage,
        message: impl Into<String>,
    ) -> LogEvent {
        self.event(Level::Debug, event, stage, message)
    }

    /// Shortcut for warn-level event.
    pub fn warn(
        &self,
        event: impl Into<String>,
        stage: Stage,
        message: impl Into<String>,
    ) -> LogEvent {
        self.event(Level::Warn, event, stage, message)
    }

    /// Shortcut for error-level event.
    pub fn error(
        &self,
        event: impl Into<String>,
        stage: Stage,
        message: impl Into<String>,
    ) -> LogEvent {
        self.event(Level::Error, event, stage, message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_event_serialization() {
        let event = LogEvent::new(
            Level::Info,
            "run.started",
            "run-12345",
            "host-abc",
            Stage::Init,
            "Starting triage run",
        )
        .with_session_id("pt-20260115-143022-a7xq")
        .with_field("config_version", "1.0.0");

        let json = event.to_jsonl();
        assert!(json.contains(r#""event":"run.started""#));
        assert!(json.contains(r#""level":"info""#));
        assert!(json.contains(r#""stage":"init""#));
        assert!(json.contains(r#""run_id":"run-12345""#));
        assert!(json.contains(r#""session_id":"pt-20260115-143022-a7xq""#));
    }

    #[test]
    fn test_log_event_with_process() {
        let event = LogEvent::new(
            Level::Debug,
            "infer.proc_done",
            "run-12345",
            "host-abc",
            Stage::Infer,
            "Inference complete for process",
        )
        .with_process(1234, "boot-id:12345:1234");

        let json = event.to_jsonl();
        assert!(json.contains(r#""pid":1234"#));
        assert!(json.contains(r#""start_id":"boot-id:12345:1234""#));
    }

    #[test]
    fn test_log_context() {
        let ctx = LogContext::new("run-abc", "host-xyz").with_session_id("pt-20260115-143022-b2c3");

        let event = ctx.info("scan.started", Stage::Scan, "Beginning scan");
        assert_eq!(event.run_id, "run-abc");
        assert_eq!(event.host_id, "host-xyz");
        assert_eq!(
            event.session_id,
            Some("pt-20260115-143022-b2c3".to_string())
        );
        assert_eq!(event.stage, Stage::Scan);
    }

    #[test]
    fn test_stage_display() {
        assert_eq!(Stage::Scan.to_string(), "scan");
        assert_eq!(Stage::Infer.to_string(), "infer");
        assert_eq!(Stage::Decide.to_string(), "decide");
    }

    #[test]
    fn test_event_names() {
        assert_eq!(event_names::RUN_STARTED, "run.started");
        assert_eq!(event_names::SCAN_FINISHED, "scan.finished");
        assert_eq!(event_names::APPLY_ACTION_RESULT, "apply.action_result");
    }
}
