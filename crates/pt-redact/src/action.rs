//! Redaction actions.

use serde::{Deserialize, Serialize};

/// Action to apply when redacting a field.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Action {
    /// Persist as-is (no modification)
    Allow,
    /// Remove/replace entirely with [REDACTED]
    Redact,
    /// Replace with keyed hash [HASH:key_id:hex]
    Hash,
    /// Pattern replacement (lossy normalization)
    Normalize,
    /// Normalize then hash
    #[serde(rename = "normalize+hash")]
    NormalizeHash,
    /// Keep prefix/suffix only
    Truncate,
    /// Auto-detect and apply appropriate action
    #[serde(rename = "detect+action")]
    DetectAction,
}

impl Action {
    /// Parse an action from a string.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "allow" => Some(Action::Allow),
            "redact" => Some(Action::Redact),
            "hash" => Some(Action::Hash),
            "normalize" => Some(Action::Normalize),
            "normalize+hash" => Some(Action::NormalizeHash),
            "truncate" => Some(Action::Truncate),
            "detect+action" => Some(Action::DetectAction),
            _ => None,
        }
    }

    /// Returns whether this action modifies the value.
    pub fn is_modifying(&self) -> bool {
        !matches!(self, Action::Allow)
    }

    /// Returns whether this action is considered "safe" (redacts or hashes).
    pub fn is_safe(&self) -> bool {
        matches!(
            self,
            Action::Redact | Action::Hash | Action::NormalizeHash
        )
    }
}

impl std::fmt::Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Action::Allow => "allow",
            Action::Redact => "redact",
            Action::Hash => "hash",
            Action::Normalize => "normalize",
            Action::NormalizeHash => "normalize+hash",
            Action::Truncate => "truncate",
            Action::DetectAction => "detect+action",
        };
        write!(f, "{}", s)
    }
}

impl Default for Action {
    fn default() -> Self {
        Action::Redact // Fail-closed default
    }
}
