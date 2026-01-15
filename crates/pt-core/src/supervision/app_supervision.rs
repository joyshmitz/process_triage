//! App-level supervisor detection.
//!
//! This module detects processes managed by application-level supervisors like
//! pm2, supervisord, nodemon, and forever. These supervisors typically:
//! - Auto-restart processes that crash
//! - Maintain process lifecycle metadata
//! - Provide management commands (stop/restart/delete)
//!
//! # Why This Matters
//!
//! Killing a process managed by pm2/supervisord directly often triggers:
//! - Immediate respawn by the supervisor
//! - Incorrect process state in supervisor's tracking
//! - Potential respawn loops if the process keeps getting killed
//!
//! The correct action is to use supervisor-specific commands (e.g., `pm2 stop`).
//!
//! # Supported Supervisors
//!
//! - **pm2**: Popular Node.js production process manager
//! - **supervisord**: Python-based process control system
//! - **nodemon**: Node.js development auto-restarter
//! - **forever**: Simple Node.js process manager

use super::ancestry::AncestryAnalyzer;
use super::environ::read_environ;
use super::signature::SignatureDatabase;
use super::types::{EvidenceType, SupervisionEvidence};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

/// Errors from app supervisor detection.
#[derive(Debug, Error)]
pub enum AppSupervisionError {
    #[error("Process {0} not found")]
    ProcessNotFound(u32),

    #[error("Failed to read process environment: {0}")]
    EnvironmentError(String),

    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
}

/// Type of app-level supervisor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AppSupervisorType {
    /// PM2 process manager
    Pm2,
    /// Python supervisord
    Supervisord,
    /// nodemon file watcher
    Nodemon,
    /// forever process manager
    Forever,
    /// Unknown supervisor type
    Unknown,
}

impl std::fmt::Display for AppSupervisorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppSupervisorType::Pm2 => write!(f, "pm2"),
            AppSupervisorType::Supervisord => write!(f, "supervisord"),
            AppSupervisorType::Nodemon => write!(f, "nodemon"),
            AppSupervisorType::Forever => write!(f, "forever"),
            AppSupervisorType::Unknown => write!(f, "unknown"),
        }
    }
}

/// Result of app supervisor detection.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSupervisionResult {
    /// The process ID analyzed.
    pub pid: u32,

    /// Whether the process is supervised by an app supervisor.
    pub is_supervised: bool,

    /// Type of app supervisor detected.
    pub supervisor_type: AppSupervisorType,

    /// Name of the supervisor (for display).
    pub supervisor_name: Option<String>,

    /// PM2 process name (if PM2-managed).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pm2_name: Option<String>,

    /// PM2 process ID (internal to pm2).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pm2_id: Option<String>,

    /// Supervisord program name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supervisord_program: Option<String>,

    /// Supervisord group name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supervisord_group: Option<String>,

    /// Confidence score (0.0-1.0).
    pub confidence: f64,

    /// Evidence supporting the detection.
    pub evidence: Vec<SupervisionEvidence>,

    /// Recommended supervisor action.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommended_action: Option<AppSupervisorAction>,

    /// Human-readable explanation.
    pub explanation: String,
}

impl AppSupervisionResult {
    /// Create a result indicating no app supervisor detected.
    pub fn not_supervised(pid: u32) -> Self {
        Self {
            pid,
            is_supervised: false,
            supervisor_type: AppSupervisorType::Unknown,
            supervisor_name: None,
            pm2_name: None,
            pm2_id: None,
            supervisord_program: None,
            supervisord_group: None,
            confidence: 1.0,
            evidence: vec![],
            recommended_action: None,
            explanation: "Process is not managed by a known app supervisor".to_string(),
        }
    }
}

/// Recommended supervisor action.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSupervisorAction {
    /// Action type.
    pub action_type: AppActionType,

    /// Command to execute (for reference, NOT auto-executed).
    pub command: String,

    /// Alternative commands for different scenarios.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub alternatives: Vec<AlternativeAction>,

    /// Whether this action is considered safe.
    pub is_safe: bool,

    /// Warning message if relevant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warning: Option<String>,

    /// Hint about respawn behavior.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub respawn_hint: Option<String>,
}

/// Alternative action for supervisor control.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlternativeAction {
    /// Action description.
    pub description: String,
    /// Command to execute.
    pub command: String,
}

/// Types of app supervisor actions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AppActionType {
    /// Stop the process via supervisor.
    Stop,
    /// Restart the process via supervisor.
    Restart,
    /// Delete/remove from supervisor.
    Delete,
    /// Show status/info.
    Status,
    /// Show logs.
    Logs,
}

/// Analyzer for app-level supervision.
pub struct AppSupervisionAnalyzer {
    /// Whether to include action recommendations.
    include_action_recommendations: bool,

    /// Signature database for detection (reserved for future use).
    #[allow(dead_code)]
    signature_db: SignatureDatabase,
}

impl AppSupervisionAnalyzer {
    /// Create a new analyzer with defaults.
    pub fn new() -> Self {
        Self {
            include_action_recommendations: true,
            signature_db: SignatureDatabase::with_defaults(),
        }
    }

    /// Set whether to include action recommendations.
    pub fn with_action_recommendations(mut self, include: bool) -> Self {
        self.include_action_recommendations = include;
        self
    }

    /// Analyze a process for app supervisor management.
    pub fn analyze(&self, pid: u32) -> Result<AppSupervisionResult, AppSupervisionError> {
        // Read process environment
        let env = match read_environ(pid) {
            Ok(env) => env,
            Err(super::environ::EnvironError::ProcessNotFound(p)) => {
                return Err(AppSupervisionError::ProcessNotFound(p));
            }
            Err(e) => {
                return Err(AppSupervisionError::EnvironmentError(e.to_string()));
            }
        };

        // Check for PM2
        if let Some(result) = self.detect_pm2(pid, &env) {
            return Ok(result);
        }

        // Check for supervisord
        if let Some(result) = self.detect_supervisord(pid, &env) {
            return Ok(result);
        }

        // Check for nodemon
        if let Some(result) = self.detect_nodemon(pid, &env) {
            return Ok(result);
        }

        // Check for forever
        if let Some(result) = self.detect_forever(pid, &env) {
            return Ok(result);
        }

        // Check via ancestry
        if let Some(result) = self.detect_via_ancestry(pid) {
            return Ok(result);
        }

        Ok(AppSupervisionResult::not_supervised(pid))
    }

    /// Detect PM2-managed process.
    fn detect_pm2(&self, pid: u32, env: &HashMap<String, String>) -> Option<AppSupervisionResult> {
        // PM2 sets PM2_HOME, PM2_PROGRAMMATIC, pm_id, etc.
        let pm2_home = env.get("PM2_HOME");
        let pm2_id = env.get("pm_id").or_else(|| env.get("PM2_ID"));
        let pm2_name = env.get("name").or_else(|| env.get("PM2_PROCESS_NAME"));

        if pm2_home.is_none() && pm2_id.is_none() {
            return None;
        }

        let mut evidence = vec![];
        let mut confidence: f64 = 0.0;

        if pm2_home.is_some() {
            evidence.push(SupervisionEvidence {
                evidence_type: EvidenceType::Environment,
                description: "PM2_HOME environment variable present".to_string(),
                weight: 0.7,
            });
            confidence = confidence.max(0.7);
        }

        if pm2_id.is_some() {
            evidence.push(SupervisionEvidence {
                evidence_type: EvidenceType::Environment,
                description: "pm_id environment variable present".to_string(),
                weight: 0.9,
            });
            confidence = confidence.max(0.9);
        }

        let pm2_name_str = pm2_name.cloned();
        let pm2_id_str = pm2_id.cloned();

        let explanation = match (&pm2_name_str, &pm2_id_str) {
            (Some(name), Some(id)) => format!("Process is managed by PM2 as '{}' (id: {})", name, id),
            (Some(name), None) => format!("Process is managed by PM2 as '{}'", name),
            (None, Some(id)) => format!("Process is managed by PM2 (id: {})", id),
            (None, None) => "Process is managed by PM2".to_string(),
        };

        let recommended_action = if self.include_action_recommendations {
            Some(self.generate_pm2_action(&pm2_name_str, &pm2_id_str))
        } else {
            None
        };

        Some(AppSupervisionResult {
            pid,
            is_supervised: true,
            supervisor_type: AppSupervisorType::Pm2,
            supervisor_name: Some("PM2".to_string()),
            pm2_name: pm2_name_str,
            pm2_id: pm2_id_str,
            supervisord_program: None,
            supervisord_group: None,
            confidence,
            evidence,
            recommended_action,
            explanation,
        })
    }

    /// Detect supervisord-managed process.
    fn detect_supervisord(&self, pid: u32, env: &HashMap<String, String>) -> Option<AppSupervisionResult> {
        // supervisord sets SUPERVISOR_* variables
        let supervisor_enabled = env.get("SUPERVISOR_ENABLED");
        let supervisor_process_name = env.get("SUPERVISOR_PROCESS_NAME");
        let supervisor_group_name = env.get("SUPERVISOR_GROUP_NAME");

        if supervisor_enabled.is_none() && supervisor_process_name.is_none() {
            return None;
        }

        let mut evidence = vec![];
        let mut confidence: f64 = 0.0;

        if supervisor_enabled.is_some() {
            evidence.push(SupervisionEvidence {
                evidence_type: EvidenceType::Environment,
                description: "SUPERVISOR_ENABLED environment variable present".to_string(),
                weight: 0.8,
            });
            confidence = confidence.max(0.8);
        }

        if supervisor_process_name.is_some() {
            evidence.push(SupervisionEvidence {
                evidence_type: EvidenceType::Environment,
                description: format!(
                    "SUPERVISOR_PROCESS_NAME={}",
                    supervisor_process_name.as_ref().unwrap()
                ),
                weight: 0.9,
            });
            confidence = confidence.max(0.9);
        }

        let program = supervisor_process_name.cloned();
        let group = supervisor_group_name.cloned();

        let explanation = match (&program, &group) {
            (Some(prog), Some(grp)) => {
                format!("Process is managed by supervisord as '{}' in group '{}'", prog, grp)
            }
            (Some(prog), None) => format!("Process is managed by supervisord as '{}'", prog),
            _ => "Process is managed by supervisord".to_string(),
        };

        let recommended_action = if self.include_action_recommendations {
            Some(self.generate_supervisord_action(&program, &group))
        } else {
            None
        };

        Some(AppSupervisionResult {
            pid,
            is_supervised: true,
            supervisor_type: AppSupervisorType::Supervisord,
            supervisor_name: Some("supervisord".to_string()),
            pm2_name: None,
            pm2_id: None,
            supervisord_program: program,
            supervisord_group: group,
            confidence,
            evidence,
            recommended_action,
            explanation,
        })
    }

    /// Detect nodemon-managed process.
    fn detect_nodemon(&self, pid: u32, env: &HashMap<String, String>) -> Option<AppSupervisionResult> {
        // nodemon sets NODEMON_CONFIG or can be detected via parent
        let nodemon_config = env.get("NODEMON_CONFIG");

        if nodemon_config.is_none() {
            return None;
        }

        let evidence = vec![SupervisionEvidence {
            evidence_type: EvidenceType::Environment,
            description: "NODEMON_CONFIG environment variable present".to_string(),
            weight: 0.85,
        }];

        let recommended_action = if self.include_action_recommendations {
            Some(self.generate_nodemon_action())
        } else {
            None
        };

        Some(AppSupervisionResult {
            pid,
            is_supervised: true,
            supervisor_type: AppSupervisorType::Nodemon,
            supervisor_name: Some("nodemon".to_string()),
            pm2_name: None,
            pm2_id: None,
            supervisord_program: None,
            supervisord_group: None,
            confidence: 0.85,
            evidence,
            recommended_action,
            explanation: "Process is managed by nodemon (development auto-restarter)".to_string(),
        })
    }

    /// Detect forever-managed process.
    fn detect_forever(&self, pid: u32, env: &HashMap<String, String>) -> Option<AppSupervisionResult> {
        // forever sets FOREVER_ROOT, FOREVER_UID
        let forever_root = env.get("FOREVER_ROOT");
        let forever_uid = env.get("FOREVER_UID");

        if forever_root.is_none() && forever_uid.is_none() {
            return None;
        }

        let mut evidence = vec![];
        let mut confidence: f64 = 0.0;

        if forever_root.is_some() {
            evidence.push(SupervisionEvidence {
                evidence_type: EvidenceType::Environment,
                description: "FOREVER_ROOT environment variable present".to_string(),
                weight: 0.8,
            });
            confidence = confidence.max(0.8);
        }

        if forever_uid.is_some() {
            evidence.push(SupervisionEvidence {
                evidence_type: EvidenceType::Environment,
                description: format!("FOREVER_UID={}", forever_uid.as_ref().unwrap()),
                weight: 0.85,
            });
            confidence = confidence.max(0.85);
        }

        let uid = forever_uid.cloned();

        let recommended_action = if self.include_action_recommendations {
            Some(self.generate_forever_action(&uid))
        } else {
            None
        };

        let explanation = match &uid {
            Some(id) => format!("Process is managed by forever (uid: {})", id),
            None => "Process is managed by forever".to_string(),
        };

        Some(AppSupervisionResult {
            pid,
            is_supervised: true,
            supervisor_type: AppSupervisorType::Forever,
            supervisor_name: Some("forever".to_string()),
            pm2_name: None,
            pm2_id: None,
            supervisord_program: None,
            supervisord_group: None,
            confidence,
            evidence,
            recommended_action,
            explanation,
        })
    }

    /// Detect supervision via parent process ancestry.
    fn detect_via_ancestry(&self, pid: u32) -> Option<AppSupervisionResult> {
        let mut analyzer = AncestryAnalyzer::new();

        let result = analyzer.analyze(pid).ok()?;

        if !result.is_supervised {
            return None;
        }

        // Check if the detected supervisor is one of our app supervisors
        let supervisor_name = result.supervisor_name.as_deref()?;

        let (supervisor_type, explanation) = match supervisor_name.to_lowercase().as_str() {
            "pm2" => (AppSupervisorType::Pm2, "Process has PM2 in ancestry chain"),
            "supervisord" => (AppSupervisorType::Supervisord, "Process has supervisord in ancestry chain"),
            "nodemon" => (AppSupervisorType::Nodemon, "Process has nodemon in ancestry chain"),
            "forever" => (AppSupervisorType::Forever, "Process has forever in ancestry chain"),
            _ => return None, // Not an app supervisor we care about
        };

        let recommended_action = if self.include_action_recommendations {
            match supervisor_type {
                AppSupervisorType::Pm2 => Some(self.generate_pm2_action(&None, &None)),
                AppSupervisorType::Supervisord => Some(self.generate_supervisord_action(&None, &None)),
                AppSupervisorType::Nodemon => Some(self.generate_nodemon_action()),
                AppSupervisorType::Forever => Some(self.generate_forever_action(&None)),
                AppSupervisorType::Unknown => None,
            }
        } else {
            None
        };

        Some(AppSupervisionResult {
            pid,
            is_supervised: true,
            supervisor_type,
            supervisor_name: Some(supervisor_name.to_string()),
            pm2_name: None,
            pm2_id: None,
            supervisord_program: None,
            supervisord_group: None,
            confidence: result.confidence * 0.8, // Slightly lower confidence via ancestry
            evidence: result.evidence.into_iter().map(|e| SupervisionEvidence {
                evidence_type: EvidenceType::Ancestry,
                description: e.description,
                weight: e.weight,
            }).collect(),
            recommended_action,
            explanation: explanation.to_string(),
        })
    }

    /// Generate PM2 action recommendation.
    fn generate_pm2_action(
        &self,
        name: &Option<String>,
        id: &Option<String>,
    ) -> AppSupervisorAction {
        let target = match (name, id) {
            (Some(n), _) => n.clone(),
            (_, Some(i)) => i.clone(),
            _ => "all".to_string(),
        };

        AppSupervisorAction {
            action_type: AppActionType::Stop,
            command: format!("pm2 stop {}", target),
            alternatives: vec![
                AlternativeAction {
                    description: "Restart process".to_string(),
                    command: format!("pm2 restart {}", target),
                },
                AlternativeAction {
                    description: "Delete from PM2".to_string(),
                    command: format!("pm2 delete {}", target),
                },
                AlternativeAction {
                    description: "Show status".to_string(),
                    command: format!("pm2 show {}", target),
                },
                AlternativeAction {
                    description: "View logs".to_string(),
                    command: format!("pm2 logs {}", target),
                },
            ],
            is_safe: false,
            warning: Some("PM2 will respawn the process if restart policy is set".to_string()),
            respawn_hint: Some("Use 'pm2 delete' to permanently remove, or 'pm2 stop' to pause".to_string()),
        }
    }

    /// Generate supervisord action recommendation.
    fn generate_supervisord_action(
        &self,
        program: &Option<String>,
        group: &Option<String>,
    ) -> AppSupervisorAction {
        let target = match (program, group) {
            (Some(prog), Some(grp)) => format!("{}:{}", grp, prog),
            (Some(prog), None) => prog.clone(),
            _ => "all".to_string(),
        };

        AppSupervisorAction {
            action_type: AppActionType::Stop,
            command: format!("supervisorctl stop {}", target),
            alternatives: vec![
                AlternativeAction {
                    description: "Restart process".to_string(),
                    command: format!("supervisorctl restart {}", target),
                },
                AlternativeAction {
                    description: "Show status".to_string(),
                    command: format!("supervisorctl status {}", target),
                },
                AlternativeAction {
                    description: "View recent log".to_string(),
                    command: format!("supervisorctl tail {}", target),
                },
            ],
            is_safe: false,
            warning: Some("supervisord may respawn the process depending on autorestart setting".to_string()),
            respawn_hint: Some("Check /etc/supervisor/conf.d/ for autorestart configuration".to_string()),
        }
    }

    /// Generate nodemon action recommendation.
    fn generate_nodemon_action(&self) -> AppSupervisorAction {
        AppSupervisorAction {
            action_type: AppActionType::Stop,
            command: "# Send SIGINT to nodemon parent process".to_string(),
            alternatives: vec![
                AlternativeAction {
                    description: "Find nodemon parent".to_string(),
                    command: "pgrep -f nodemon".to_string(),
                },
            ],
            is_safe: true,
            warning: Some("nodemon is a development tool - killing the main process stops watching".to_string()),
            respawn_hint: Some("nodemon only respawns on file changes, not crashes".to_string()),
        }
    }

    /// Generate forever action recommendation.
    fn generate_forever_action(&self, uid: &Option<String>) -> AppSupervisorAction {
        let target = uid.clone().unwrap_or_else(|| "0".to_string());

        AppSupervisorAction {
            action_type: AppActionType::Stop,
            command: format!("forever stop {}", target),
            alternatives: vec![
                AlternativeAction {
                    description: "Restart process".to_string(),
                    command: format!("forever restart {}", target),
                },
                AlternativeAction {
                    description: "List all processes".to_string(),
                    command: "forever list".to_string(),
                },
                AlternativeAction {
                    description: "Stop all processes".to_string(),
                    command: "forever stopall".to_string(),
                },
            ],
            is_safe: false,
            warning: Some("forever will respawn crashed processes by default".to_string()),
            respawn_hint: Some("Use 'forever stop' to prevent respawning".to_string()),
        }
    }
}

impl Default for AppSupervisionAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

/// Convenience function to detect app supervision.
pub fn detect_app_supervision(pid: u32) -> Result<AppSupervisionResult, AppSupervisionError> {
    let analyzer = AppSupervisionAnalyzer::new();
    analyzer.analyze(pid)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_supervision_analyzer_new() {
        let analyzer = AppSupervisionAnalyzer::new();
        assert!(analyzer.include_action_recommendations);
    }

    #[test]
    fn test_app_supervision_result_not_supervised() {
        let result = AppSupervisionResult::not_supervised(1234);
        assert!(!result.is_supervised);
        assert_eq!(result.supervisor_type, AppSupervisorType::Unknown);
    }

    #[test]
    fn test_app_supervisor_type_display() {
        assert_eq!(AppSupervisorType::Pm2.to_string(), "pm2");
        assert_eq!(AppSupervisorType::Supervisord.to_string(), "supervisord");
        assert_eq!(AppSupervisorType::Nodemon.to_string(), "nodemon");
        assert_eq!(AppSupervisorType::Forever.to_string(), "forever");
    }

    #[test]
    fn test_pm2_action_generation() {
        let analyzer = AppSupervisionAnalyzer::new();
        let action = analyzer.generate_pm2_action(&Some("myapp".to_string()), &None);

        assert_eq!(action.action_type, AppActionType::Stop);
        assert_eq!(action.command, "pm2 stop myapp");
        assert!(!action.alternatives.is_empty());
        assert!(action.warning.is_some());
        assert!(action.respawn_hint.is_some());
    }

    #[test]
    fn test_supervisord_action_generation() {
        let analyzer = AppSupervisionAnalyzer::new();
        let action = analyzer.generate_supervisord_action(
            &Some("worker".to_string()),
            &Some("celery".to_string()),
        );

        assert_eq!(action.action_type, AppActionType::Stop);
        assert_eq!(action.command, "supervisorctl stop celery:worker");
    }

    #[test]
    fn test_action_type_serialization() {
        let action = AppSupervisorAction {
            action_type: AppActionType::Stop,
            command: "pm2 stop app".to_string(),
            alternatives: vec![],
            is_safe: false,
            warning: None,
            respawn_hint: None,
        };

        let json = serde_json::to_string(&action).unwrap();
        assert!(json.contains("\"action_type\":\"stop\""));
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn test_detect_app_supervision_current_process() {
        let pid = std::process::id();
        let result = detect_app_supervision(pid);

        // Should succeed for current process
        assert!(result.is_ok());

        let result = result.unwrap();
        // Current process is likely not PM2/supervisord managed
        assert!(result.confidence >= 0.0 && result.confidence <= 1.0);

        crate::test_log!(
            INFO,
            "app supervision test",
            pid = pid,
            is_supervised = result.is_supervised,
            supervisor_type = result.supervisor_type.to_string().as_str()
        );
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn test_nomock_app_supervision_spawned_process() {
        use crate::test_utils::ProcessHarness;

        if !ProcessHarness::is_available() {
            crate::test_log!(INFO, "Skipping no-mock test: ProcessHarness not available");
            return;
        }

        let harness = ProcessHarness::default();
        let proc = harness
            .spawn_shell("sleep 30")
            .expect("spawn sleep process");

        crate::test_log!(
            INFO,
            "app supervision spawned process test",
            pid = proc.pid()
        );

        let result = detect_app_supervision(proc.pid());
        assert!(result.is_ok());

        let result = result.unwrap();
        // Spawned process should not be supervised by app supervisors
        assert!(!result.is_supervised);
        assert_eq!(result.supervisor_type, AppSupervisorType::Unknown);
    }
}
