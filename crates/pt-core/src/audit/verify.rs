//! Audit log integrity verification.
//!
//! This module provides functions to verify the hash chain integrity
//! of audit logs, detecting tampering or corruption.

use super::entry::{AuditEntry, AUDIT_SCHEMA_VERSION};
use super::writer::GENESIS_HASH;
use super::AuditError;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

/// Result of hash chain verification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationResult {
    /// Whether the log passed integrity verification.
    pub is_valid: bool,

    /// Total number of entries verified.
    pub entries_verified: u64,

    /// State hash (hash of all entry hashes).
    pub state_hash: String,

    /// First broken link in the chain, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub broken_link: Option<BrokenLink>,

    /// Entries with invalid self-hashes (tampered).
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tampered_entries: Vec<TamperedEntry>,

    /// Any schema version mismatches found.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub schema_warnings: Vec<SchemaWarning>,
}

/// Information about a broken link in the hash chain.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrokenLink {
    /// Line number where the break was detected (1-indexed).
    pub line: usize,

    /// Expected hash (from previous entry).
    pub expected: String,

    /// Actual hash found in the entry's prev_hash field.
    pub actual: String,

    /// Type of break detected.
    pub break_type: BreakType,
}

/// Type of hash chain break.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BreakType {
    /// The prev_hash doesn't match the previous entry's hash.
    ChainMismatch,
    /// An entry is missing from the chain.
    MissingEntry,
    /// The log file appears to be truncated.
    Truncated,
    /// The first entry doesn't have genesis hash.
    InvalidGenesis,
}

/// Information about a tampered (self-hash mismatch) entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TamperedEntry {
    /// Line number of the tampered entry (1-indexed).
    pub line: usize,

    /// Stored hash in the entry.
    pub stored_hash: String,

    /// Computed hash of the entry.
    pub computed_hash: String,

    /// Event type of the entry.
    pub event_type: String,
}

/// Warning about schema version differences.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaWarning {
    /// Line number where the warning applies (1-indexed).
    pub line: usize,

    /// Schema version found.
    pub version: String,

    /// Expected schema version.
    pub expected: String,
}

/// Verify the integrity of an audit log file.
///
/// This function:
/// 1. Reads each entry in sequence
/// 2. Verifies each entry's self-hash
/// 3. Verifies the hash chain (prev_hash matches previous entry)
/// 4. Reports the first broken link and all tampered entries
pub fn verify_log(path: &Path) -> Result<VerificationResult, AuditError> {
    if !path.exists() {
        return Ok(VerificationResult {
            is_valid: true,
            entries_verified: 0,
            state_hash: "empty".to_string(),
            broken_link: None,
            tampered_entries: Vec::new(),
            schema_warnings: Vec::new(),
        });
    }

    let file = File::open(path).map_err(|e| AuditError::Io {
        path: path.to_path_buf(),
        source: e,
    })?;

    let reader = BufReader::new(file);
    let mut entries_verified = 0u64;
    let mut prev_hash = GENESIS_HASH.to_string();
    let mut broken_link: Option<BrokenLink> = None;
    let mut tampered_entries = Vec::new();
    let mut schema_warnings = Vec::new();
    let mut combined_hashes = String::new();

    for (line_idx, line_result) in reader.lines().enumerate() {
        let line_num = line_idx + 1;

        let line = line_result.map_err(|e| AuditError::Io {
            path: path.to_path_buf(),
            source: e,
        })?;

        if line.trim().is_empty() {
            continue;
        }

        let entry: AuditEntry =
            serde_json::from_str(&line).map_err(|e| AuditError::Parse {
                line: line_num,
                source: e,
            })?;

        // Check schema version
        if entry.schema_version != AUDIT_SCHEMA_VERSION {
            schema_warnings.push(SchemaWarning {
                line: line_num,
                version: entry.schema_version.clone(),
                expected: AUDIT_SCHEMA_VERSION.to_string(),
            });
        }

        // Verify self-hash
        if !entry.verify_hash() {
            tampered_entries.push(TamperedEntry {
                line: line_num,
                stored_hash: entry.entry_hash.clone().unwrap_or_default(),
                computed_hash: compute_entry_hash(&entry),
                event_type: entry.event_type.to_string(),
            });
        }

        // Verify chain (only record first break)
        if broken_link.is_none() && entry.prev_hash != prev_hash {
            let break_type = if line_num == 1 && entry.prev_hash != GENESIS_HASH {
                BreakType::InvalidGenesis
            } else {
                BreakType::ChainMismatch
            };

            broken_link = Some(BrokenLink {
                line: line_num,
                expected: prev_hash.clone(),
                actual: entry.prev_hash.clone(),
                break_type,
            });
        }

        // Update chain state
        if let Some(ref hash) = entry.entry_hash {
            prev_hash = hash.clone();
            combined_hashes.push_str(hash);
        }

        entries_verified += 1;
    }

    // Compute final state hash
    let state_hash = if combined_hashes.is_empty() {
        "empty".to_string()
    } else {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(combined_hashes.as_bytes());
        hex::encode(hasher.finalize())
    };

    let is_valid = broken_link.is_none() && tampered_entries.is_empty();

    Ok(VerificationResult {
        is_valid,
        entries_verified,
        state_hash,
        broken_link,
        tampered_entries,
        schema_warnings,
    })
}

/// Compute the hash of an entry (for reporting in tamper detection).
fn compute_entry_hash(entry: &AuditEntry) -> String {
    let mut verify_entry = entry.clone();
    verify_entry.entry_hash = None;

    let json = serde_json::to_string(&verify_entry).unwrap_or_default();

    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(json.as_bytes());
    hex::encode(hasher.finalize())
}

/// Verify multiple log files (current + rotated) in sequence.
///
/// This is useful for verifying the entire audit history including rotated files.
pub fn verify_log_chain(paths: &[&Path]) -> Result<VerificationResult, AuditError> {
    if paths.is_empty() {
        return Ok(VerificationResult {
            is_valid: true,
            entries_verified: 0,
            state_hash: "empty".to_string(),
            broken_link: None,
            tampered_entries: Vec::new(),
            schema_warnings: Vec::new(),
        });
    }

    let mut total_entries = 0u64;
    let mut all_tampered = Vec::new();
    let mut all_warnings = Vec::new();
    let mut combined_state = String::new();
    let mut first_broken: Option<BrokenLink> = None;

    for path in paths {
        let result = verify_log(path)?;

        total_entries += result.entries_verified;
        all_tampered.extend(result.tampered_entries);
        all_warnings.extend(result.schema_warnings);
        combined_state.push_str(&result.state_hash);

        if first_broken.is_none() && result.broken_link.is_some() {
            first_broken = result.broken_link;
        }
    }

    // Compute combined state hash
    let final_state_hash = if combined_state.is_empty() {
        "empty".to_string()
    } else {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(combined_state.as_bytes());
        hex::encode(hasher.finalize())
    };

    let is_valid = first_broken.is_none() && all_tampered.is_empty();

    Ok(VerificationResult {
        is_valid,
        entries_verified: total_entries,
        state_hash: final_state_hash,
        broken_link: first_broken,
        tampered_entries: all_tampered,
        schema_warnings: all_warnings,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::audit::{AuditContext, AuditLog, AuditLogConfig};
    use tempfile::TempDir;

    fn test_config(dir: &Path) -> AuditLogConfig {
        AuditLogConfig {
            max_size_bytes: 1024 * 1024,
            auto_rotate: false,
            audit_dir: Some(dir.to_path_buf()),
        }
    }

    #[test]
    fn test_verify_empty_file() {
        let tmp = TempDir::new().unwrap();
        let path = tmp.path().join("empty.jsonl");

        let result = verify_log(&path).unwrap();

        assert!(result.is_valid);
        assert_eq!(result.entries_verified, 0);
        assert_eq!(result.state_hash, "empty");
    }

    #[test]
    fn test_verify_valid_log() {
        let tmp = TempDir::new().unwrap();
        let config = test_config(tmp.path());

        // Create a valid log
        {
            let mut log = AuditLog::open_or_create_with_config(config.clone()).unwrap();
            let ctx = AuditContext::new("run-test", "host-test");

            log.log_scan(&ctx, "started", None, None, None, None).unwrap();
            log.log_scan(&ctx, "completed", Some(100), Some(5), None, Some(500))
                .unwrap();
            log.log_action(&ctx, 1234, None, "kill", true, None, Some("SIGTERM"), false)
                .unwrap();
        }

        // Verify it
        let path = tmp.path().join("audit.jsonl");
        let result = verify_log(&path).unwrap();

        assert!(result.is_valid);
        assert_eq!(result.entries_verified, 3);
        assert!(result.broken_link.is_none());
        assert!(result.tampered_entries.is_empty());
    }

    #[test]
    fn test_verify_tampered_entry() {
        let tmp = TempDir::new().unwrap();
        let config = test_config(tmp.path());
        let path = tmp.path().join("audit.jsonl");

        // Create a valid log
        {
            let mut log = AuditLog::open_or_create_with_config(config).unwrap();
            let ctx = AuditContext::new("run-test", "host-test");

            log.log_scan(&ctx, "started", None, None, None, None).unwrap();
            log.log_scan(&ctx, "completed", Some(100), Some(5), None, None)
                .unwrap();
        }

        // Tamper with the file
        let content = std::fs::read_to_string(&path).unwrap();
        let tampered = content.replace("100", "999");
        std::fs::write(&path, tampered).unwrap();

        // Verify it
        let result = verify_log(&path).unwrap();

        assert!(!result.is_valid);
        assert_eq!(result.entries_verified, 2);
        assert!(!result.tampered_entries.is_empty());
    }

    #[test]
    fn test_verify_broken_chain() {
        let tmp = TempDir::new().unwrap();
        let path = tmp.path().join("broken.jsonl");

        // Manually create entries with broken chain
        let entry1 = r#"{"schema_version":"1.0.0","ts":"2026-01-15T12:00:00Z","event_type":"scan","run_id":"run-1","host_id":"host-1","message":"Test","prev_hash":"genesis","entry_hash":"abc123"}"#;
        let entry2 = r#"{"schema_version":"1.0.0","ts":"2026-01-15T12:00:01Z","event_type":"scan","run_id":"run-1","host_id":"host-1","message":"Test 2","prev_hash":"wrong_hash","entry_hash":"def456"}"#;

        std::fs::write(&path, format!("{}\n{}\n", entry1, entry2)).unwrap();

        let result = verify_log(&path).unwrap();

        assert!(!result.is_valid);
        assert!(result.broken_link.is_some());

        let broken = result.broken_link.unwrap();
        assert_eq!(broken.line, 2);
        assert_eq!(broken.expected, "abc123");
        assert_eq!(broken.actual, "wrong_hash");
        assert_eq!(broken.break_type, BreakType::ChainMismatch);
    }

    #[test]
    fn test_verify_invalid_genesis() {
        let tmp = TempDir::new().unwrap();
        let path = tmp.path().join("bad_genesis.jsonl");

        // Entry without genesis hash as first entry
        let entry = r#"{"schema_version":"1.0.0","ts":"2026-01-15T12:00:00Z","event_type":"scan","run_id":"run-1","host_id":"host-1","message":"Test","prev_hash":"not_genesis","entry_hash":"abc123"}"#;

        std::fs::write(&path, format!("{}\n", entry)).unwrap();

        let result = verify_log(&path).unwrap();

        assert!(!result.is_valid);
        assert!(result.broken_link.is_some());

        let broken = result.broken_link.unwrap();
        assert_eq!(broken.line, 1);
        assert_eq!(broken.break_type, BreakType::InvalidGenesis);
    }

    #[test]
    fn test_verification_result_serialization() {
        let result = VerificationResult {
            is_valid: true,
            entries_verified: 10,
            state_hash: "abc123".to_string(),
            broken_link: None,
            tampered_entries: Vec::new(),
            schema_warnings: Vec::new(),
        };

        let json = serde_json::to_string(&result).unwrap();
        assert!(json.contains(r#""is_valid":true"#));
        assert!(json.contains(r#""entries_verified":10"#));

        // Should not contain empty arrays
        assert!(!json.contains("tampered_entries"));
        assert!(!json.contains("schema_warnings"));
    }
}
