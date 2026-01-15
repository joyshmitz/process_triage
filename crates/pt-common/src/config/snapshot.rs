//! Configuration snapshots for telemetry and audit.
//!
//! Captures a complete snapshot of the active configuration including:
//! - File paths and hashes
//! - Schema versions
//! - Resolution method
//! - Effective values

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use super::{ConfigResolution, ConfigSource, Policy, Priors};
use crate::error::{Error, Result};

/// Complete configuration snapshot for telemetry and audit.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigSnapshot {
    /// Timestamp when snapshot was created
    pub snapshot_at: DateTime<Utc>,

    /// Combined hash of all config content
    pub combined_hash: String,

    /// Priors source information
    pub priors_source: SourceInfo,

    /// Policy source information
    pub policy_source: SourceInfo,

    /// Active schema versions
    pub schema_versions: SchemaVersions,
}

impl ConfigSnapshot {
    /// Create a new snapshot from loaded configs.
    pub fn new(
        priors: &Priors,
        policy: &Policy,
        priors_source: ConfigSource,
        policy_source: ConfigSource,
    ) -> Result<Self> {
        let now = Utc::now();

        // Compute combined hash
        let priors_json = serde_json::to_string(priors)
            .map_err(|e| Error::Config(format!("failed to serialize priors: {}", e)))?;
        let policy_json = serde_json::to_string(policy)
            .map_err(|e| Error::Config(format!("failed to serialize policy: {}", e)))?;

        let combined_hash = compute_combined_hash(&priors_json, &policy_json);

        Ok(ConfigSnapshot {
            snapshot_at: now,
            combined_hash,
            priors_source: SourceInfo::from_config_source(priors_source),
            policy_source: SourceInfo::from_config_source(policy_source),
            schema_versions: SchemaVersions {
                priors: priors.schema_version.clone(),
                policy: policy.schema_version.clone(),
            },
        })
    }

    /// Create a snapshot for built-in defaults.
    pub fn from_defaults(priors: &Priors, policy: &Policy) -> Result<Self> {
        ConfigSnapshot::new(
            priors,
            policy,
            ConfigSource {
                path: None,
                hash: None,
                resolution: ConfigResolution::Default,
            },
            ConfigSource {
                path: None,
                hash: None,
                resolution: ConfigResolution::Default,
            },
        )
    }

    /// Return true if both configs are from defaults.
    pub fn is_default(&self) -> bool {
        self.priors_source.resolution == "default" && self.policy_source.resolution == "default"
    }

    /// Return the snapshot as a JSON value for telemetry inclusion.
    pub fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "snapshot_at": self.snapshot_at.to_rfc3339(),
            "combined_hash": self.combined_hash,
            "priors": {
                "path": self.priors_source.path,
                "hash": self.priors_source.hash,
                "resolution": self.priors_source.resolution,
            },
            "policy": {
                "path": self.policy_source.path,
                "hash": self.policy_source.hash,
                "resolution": self.policy_source.resolution,
            },
            "schema_versions": {
                "priors": self.schema_versions.priors,
                "policy": self.schema_versions.policy,
            }
        })
    }
}

/// Source information for a config file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceInfo {
    /// Path to the file (None if defaults)
    pub path: Option<String>,

    /// SHA-256 hash of file content (None if defaults)
    pub hash: Option<String>,

    /// How the config was resolved
    pub resolution: String,
}

impl SourceInfo {
    fn from_config_source(source: ConfigSource) -> Self {
        SourceInfo {
            path: source.path,
            hash: source.hash,
            resolution: source.resolution.to_string(),
        }
    }
}

/// Schema versions for loaded configs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaVersions {
    pub priors: String,
    pub policy: String,
}

/// Compute a combined hash from multiple config strings.
fn compute_combined_hash(priors: &str, policy: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(b"priors:");
    hasher.update(priors.as_bytes());
    hasher.update(b":policy:");
    hasher.update(policy.as_bytes());
    let result = hasher.finalize();
    hex::encode(result)
}

#[cfg(test)]
mod tests {
    use super::super::policy::POLICY_SCHEMA_VERSION;
    use super::super::priors::PRIORS_SCHEMA_VERSION;
    use super::*;

    #[test]
    fn test_snapshot_from_defaults() {
        let priors = Priors::default();
        let policy = Policy::default();
        let snapshot = ConfigSnapshot::from_defaults(&priors, &policy).unwrap();

        assert!(snapshot.is_default());
        assert!(!snapshot.combined_hash.is_empty());
        assert_eq!(snapshot.schema_versions.priors, PRIORS_SCHEMA_VERSION);
        assert_eq!(snapshot.schema_versions.policy, POLICY_SCHEMA_VERSION);
    }

    #[test]
    fn test_snapshot_json() {
        let priors = Priors::default();
        let policy = Policy::default();
        let snapshot = ConfigSnapshot::from_defaults(&priors, &policy).unwrap();
        let json = snapshot.to_json();

        assert!(json.get("snapshot_at").is_some());
        assert!(json.get("combined_hash").is_some());
        assert!(json.get("priors").is_some());
        assert!(json.get("policy").is_some());
    }
}
