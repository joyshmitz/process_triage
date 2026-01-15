//! Session persistence and resumability primitives.
//!
//! This module implements the core pieces of the session model from
//! `specs/session-model.md`:
//! - Resolve session root dir via XDG + env overrides
//! - Create/open session directories
//! - Persist `manifest.json` + `context.json` + optional `capabilities.json`
//! - Update session state history in an append-only manner
//!
//! NOTE: Higher-level commands (agent plan/apply/verify, etc.) build on these
//! primitives. This module intentionally avoids any TUI assumptions.

use chrono::Utc;
use pt_common::{schema::SCHEMA_VERSION, SessionId};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use thiserror::Error;

const ENV_DATA_DIR: &str = "PROCESS_TRIAGE_DATA";

const DIR_NAME: &str = "process_triage";
const SESSIONS_DIR_NAME: &str = "sessions";

const MANIFEST_FILE: &str = "manifest.json";
const CONTEXT_FILE: &str = "context.json";
const CAPABILITIES_FILE: &str = "capabilities.json";

const SCAN_DIR: &str = "scan";
const INFERENCE_DIR: &str = "inference";
const DECISION_DIR: &str = "decision";
const ACTION_DIR: &str = "action";
const TELEMETRY_DIR: &str = "telemetry";
const LOGS_DIR: &str = "logs";
const EXPORTS_DIR: &str = "exports";

const SCAN_PROBES_DIR: &str = "scan/probes";

#[derive(Debug, Error)]
pub enum SessionError {
    #[error("failed to resolve XDG data dir (set {ENV_DATA_DIR} or XDG_DATA_HOME)")]
    DataDirUnavailable,

    #[error("session not found: {session_id}")]
    NotFound { session_id: String },

    #[error("I/O error at {path}: {source}")]
    Io {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("failed to parse JSON at {path}: {source}")]
    Json {
        path: PathBuf,
        #[source]
        source: serde_json::Error,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SessionState {
    Created,
    Scanning,
    Planned,
    Executing,
    Completed,
    Cancelled,
    Failed,
    Archived,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SessionMode {
    Interactive,
    RobotPlan,
    RobotApply,
    DaemonAlert,
    ScanOnly,
    Export,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateTransition {
    pub state: SessionState,
    pub ts: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionTiming {
    pub created_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionManifest {
    pub schema_version: String,
    pub session_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_session_id: Option<String>,
    pub state: SessionState,
    pub state_history: Vec<StateTransition>,
    pub mode: SessionMode,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    pub timing: SessionTiming,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl SessionManifest {
    pub fn new(
        session_id: &SessionId,
        parent_session_id: Option<&SessionId>,
        mode: SessionMode,
        label: Option<String>,
    ) -> Self {
        let now = Utc::now().to_rfc3339();
        Self {
            schema_version: SCHEMA_VERSION.to_string(),
            session_id: session_id.0.clone(),
            parent_session_id: parent_session_id.map(|id| id.0.clone()),
            state: SessionState::Created,
            state_history: vec![StateTransition {
                state: SessionState::Created,
                ts: now.clone(),
            }],
            mode,
            label,
            timing: SessionTiming {
                created_at: now,
                updated_at: None,
            },
            error: None,
        }
    }

    pub fn record_state(&mut self, state: SessionState) {
        let now = Utc::now().to_rfc3339();
        self.state = state;
        self.state_history.push(StateTransition { state, ts: now.clone() });
        self.timing.updated_at = Some(now);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionContext {
    pub schema_version: String,
    pub session_id: String,
    pub generated_at: String,
    pub host_id: String,
    pub run_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    pub os: SessionOs,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionOs {
    pub family: String,
    pub arch: String,
}

impl SessionContext {
    pub fn new(session_id: &SessionId, host_id: String, run_id: String, label: Option<String>) -> Self {
        Self {
            schema_version: SCHEMA_VERSION.to_string(),
            session_id: session_id.0.clone(),
            generated_at: Utc::now().to_rfc3339(),
            host_id,
            run_id,
            label,
            os: SessionOs {
                family: std::env::consts::OS.to_string(),
                arch: std::env::consts::ARCH.to_string(),
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct SessionStore {
    sessions_root: PathBuf,
}

impl SessionStore {
    pub fn from_env() -> Result<Self, SessionError> {
        Ok(Self {
            sessions_root: resolve_sessions_root()?,
        })
    }

    pub fn sessions_root(&self) -> &Path {
        &self.sessions_root
    }

    pub fn session_dir(&self, session_id: &SessionId) -> PathBuf {
        self.sessions_root.join(&session_id.0)
    }

    pub fn create(&self, manifest: &SessionManifest) -> Result<SessionHandle, SessionError> {
        std::fs::create_dir_all(&self.sessions_root).map_err(|e| SessionError::Io {
            path: self.sessions_root.clone(),
            source: e,
        })?;

        let session_id = SessionId(manifest.session_id.clone());
        let dir = self.session_dir(&session_id);
        std::fs::create_dir_all(&dir).map_err(|e| SessionError::Io {
            path: dir.clone(),
            source: e,
        })?;

        // Create canonical subdirs upfront (makes later phases simpler).
        for rel in [
            SCAN_DIR,
            SCAN_PROBES_DIR,
            INFERENCE_DIR,
            DECISION_DIR,
            ACTION_DIR,
            TELEMETRY_DIR,
            LOGS_DIR,
            EXPORTS_DIR,
        ] {
            let p = dir.join(rel);
            std::fs::create_dir_all(&p).map_err(|e| SessionError::Io { path: p, source: e })?;
        }

        let handle = SessionHandle { id: session_id, dir };
        handle.write_manifest(manifest)?;
        Ok(handle)
    }

    pub fn open(&self, session_id: &SessionId) -> Result<SessionHandle, SessionError> {
        let dir = self.session_dir(session_id);
        if !dir.exists() {
            return Err(SessionError::NotFound {
                session_id: session_id.0.clone(),
            });
        }
        Ok(SessionHandle {
            id: session_id.clone(),
            dir,
        })
    }
}

#[derive(Debug, Clone)]
pub struct SessionHandle {
    pub id: SessionId,
    pub dir: PathBuf,
}

impl SessionHandle {
    pub fn manifest_path(&self) -> PathBuf {
        self.dir.join(MANIFEST_FILE)
    }

    pub fn context_path(&self) -> PathBuf {
        self.dir.join(CONTEXT_FILE)
    }

    pub fn capabilities_path(&self) -> PathBuf {
        self.dir.join(CAPABILITIES_FILE)
    }

    pub fn read_manifest(&self) -> Result<SessionManifest, SessionError> {
        let path = self.manifest_path();
        let content = std::fs::read_to_string(&path).map_err(|e| SessionError::Io {
            path: path.clone(),
            source: e,
        })?;
        serde_json::from_str(&content).map_err(|e| SessionError::Json { path, source: e })
    }

    pub fn write_manifest(&self, manifest: &SessionManifest) -> Result<(), SessionError> {
        write_json_pretty(&self.manifest_path(), manifest)
    }

    pub fn write_context(&self, ctx: &SessionContext) -> Result<(), SessionError> {
        write_json_pretty(&self.context_path(), ctx)
    }

    pub fn write_capabilities_json(&self, raw_json: &str) -> Result<(), SessionError> {
        // Best-effort: if invalid JSON, still store as a JSON string wrapper to keep file parseable.
        let value: serde_json::Value = match serde_json::from_str(raw_json) {
            Ok(v) => v,
            Err(_) => serde_json::json!({ "raw": raw_json }),
        };
        write_json_pretty(&self.capabilities_path(), &value)
    }

    pub fn update_state(&self, new_state: SessionState) -> Result<SessionManifest, SessionError> {
        let mut manifest = self.read_manifest()?;
        manifest.record_state(new_state);
        self.write_manifest(&manifest)?;
        Ok(manifest)
    }
}

fn resolve_sessions_root() -> Result<PathBuf, SessionError> {
    // 1) Explicit override: PROCESS_TRIAGE_DATA
    if let Ok(dir) = std::env::var(ENV_DATA_DIR) {
        return Ok(PathBuf::from(dir).join(SESSIONS_DIR_NAME));
    }

    // 2) XDG_DATA_HOME
    if let Ok(xdg) = std::env::var("XDG_DATA_HOME") {
        return Ok(PathBuf::from(xdg).join(DIR_NAME).join(SESSIONS_DIR_NAME));
    }

    // 3) Platform default (dirs)
    if let Some(base) = dirs::data_dir() {
        return Ok(base.join(DIR_NAME).join(SESSIONS_DIR_NAME));
    }

    Err(SessionError::DataDirUnavailable)
}

fn write_json_pretty<T: Serialize>(path: &Path, value: &T) -> Result<(), SessionError> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| SessionError::Io {
            path: parent.to_path_buf(),
            source: e,
        })?;
    }
    let content = serde_json::to_string_pretty(value).map_err(|e| SessionError::Json {
        path: path.to_path_buf(),
        source: e,
    })?;
    std::fs::write(path, content).map_err(|e| SessionError::Io {
        path: path.to_path_buf(),
        source: e,
    })
}

