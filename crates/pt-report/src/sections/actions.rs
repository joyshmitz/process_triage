//! Actions section data.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Single action row.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionRow {
    /// Timestamp of the action.
    pub timestamp: DateTime<Utc>,
    /// Process ID.
    pub pid: u32,
    /// Process start ID.
    pub start_id: String,
    /// Command name.
    pub cmd: String,
    /// Recommendation that led to this action.
    pub recommendation: String,
    /// Actual decision made.
    pub decision: String,
    /// Decision source (user, auto, policy).
    pub decision_source: String,
    /// Action type (SIGTERM, SIGKILL, etc.).
    pub action_type: Option<String>,
    /// Whether action was attempted.
    pub action_attempted: bool,
    /// Whether action succeeded.
    pub action_successful: bool,
    /// Signal sent.
    pub signal_sent: Option<String>,
    /// Signal response.
    pub signal_response: Option<String>,
    /// Process state after action.
    pub process_state_after: Option<String>,
    /// Memory freed in bytes.
    pub memory_freed_bytes: Option<u64>,
    /// Error message if failed.
    pub error_message: Option<String>,
    /// User feedback label.
    pub user_feedback: Option<String>,
    /// Feedback timestamp.
    pub feedback_ts: Option<DateTime<Utc>>,
    /// Score at time of decision.
    pub score: f64,
}

impl ActionRow {
    /// Get status badge text.
    pub fn status_text(&self) -> &'static str {
        if !self.action_attempted {
            "Skipped"
        } else if self.action_successful {
            "Success"
        } else {
            "Failed"
        }
    }

    /// Get CSS class for status badge.
    pub fn status_class(&self) -> &'static str {
        if !self.action_attempted {
            "bg-gray-100 text-gray-800 dark:bg-gray-700 dark:text-gray-200"
        } else if self.action_successful {
            "bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-200"
        } else {
            "bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-200"
        }
    }

    /// Get feedback badge class if present.
    pub fn feedback_class(&self) -> Option<&'static str> {
        self.user_feedback.as_ref().map(|fb| match fb.as_str() {
            "correct_kill" | "correct_spare" => {
                "bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-200"
            }
            "false_positive" | "false_negative" => {
                "bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-200"
            }
            _ => "bg-gray-100 text-gray-800 dark:bg-gray-700 dark:text-gray-200",
        })
    }

    /// Format memory freed.
    pub fn memory_freed_formatted(&self) -> Option<String> {
        self.memory_freed_bytes.map(|bytes| {
            if bytes >= 1_073_741_824 {
                format!("{:.1} GB", bytes as f64 / 1_073_741_824.0)
            } else if bytes >= 1_048_576 {
                format!("{:.0} MB", bytes as f64 / 1_048_576.0)
            } else if bytes >= 1024 {
                format!("{:.0} KB", bytes as f64 / 1024.0)
            } else {
                format!("{} B", bytes)
            }
        })
    }
}

/// Actions section containing all action data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionsSection {
    /// All action rows.
    pub actions: Vec<ActionRow>,
    /// Summary statistics.
    pub summary: ActionsSummary,
}

/// Summary statistics for actions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionsSummary {
    /// Total actions.
    pub total: usize,
    /// Successful actions.
    pub successful: usize,
    /// Failed actions.
    pub failed: usize,
    /// Skipped actions.
    pub skipped: usize,
    /// Total memory freed in bytes.
    pub total_memory_freed: u64,
    /// Actions with user feedback.
    pub with_feedback: usize,
    /// Correct feedback count.
    pub correct_feedback: usize,
    /// Incorrect feedback count.
    pub incorrect_feedback: usize,
}

impl ActionsSection {
    /// Create a new actions section.
    pub fn new(actions: Vec<ActionRow>) -> Self {
        let summary = ActionsSummary::from_actions(&actions);
        Self { actions, summary }
    }

    /// Get actions filtered by status.
    pub fn successful_actions(&self) -> Vec<&ActionRow> {
        self.actions
            .iter()
            .filter(|a| a.action_attempted && a.action_successful)
            .collect()
    }

    /// Get failed actions.
    pub fn failed_actions(&self) -> Vec<&ActionRow> {
        self.actions
            .iter()
            .filter(|a| a.action_attempted && !a.action_successful)
            .collect()
    }
}

impl ActionsSummary {
    /// Calculate summary from actions.
    pub fn from_actions(actions: &[ActionRow]) -> Self {
        let mut summary = Self {
            total: actions.len(),
            successful: 0,
            failed: 0,
            skipped: 0,
            total_memory_freed: 0,
            with_feedback: 0,
            correct_feedback: 0,
            incorrect_feedback: 0,
        };

        for action in actions {
            if !action.action_attempted {
                summary.skipped += 1;
            } else if action.action_successful {
                summary.successful += 1;
                if let Some(freed) = action.memory_freed_bytes {
                    summary.total_memory_freed += freed;
                }
            } else {
                summary.failed += 1;
            }

            if let Some(ref feedback) = action.user_feedback {
                summary.with_feedback += 1;
                if feedback.starts_with("correct") {
                    summary.correct_feedback += 1;
                } else if feedback.contains("false") {
                    summary.incorrect_feedback += 1;
                }
            }
        }

        summary
    }

    /// Get success rate as percentage.
    pub fn success_rate(&self) -> f64 {
        let attempted = self.successful + self.failed;
        if attempted > 0 {
            100.0 * self.successful as f64 / attempted as f64
        } else {
            0.0
        }
    }

    /// Get accuracy rate from feedback.
    pub fn accuracy_rate(&self) -> Option<f64> {
        if self.with_feedback > 0 {
            Some(100.0 * self.correct_feedback as f64 / self.with_feedback as f64)
        } else {
            None
        }
    }

    /// Format total memory freed.
    pub fn memory_freed_formatted(&self) -> String {
        let bytes = self.total_memory_freed;
        if bytes >= 1_073_741_824 {
            format!("{:.1} GB", bytes as f64 / 1_073_741_824.0)
        } else if bytes >= 1_048_576 {
            format!("{:.0} MB", bytes as f64 / 1_048_576.0)
        } else if bytes >= 1024 {
            format!("{:.0} KB", bytes as f64 / 1024.0)
        } else {
            format!("{} B", bytes)
        }
    }
}
