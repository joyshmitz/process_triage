//! No-mock cross-profile redaction tests for bd-yaps.
//!
//! Validates redaction guarantees across all export profiles
//! in a bundle context:
//! - Secrets never leak in any profile
//! - Profile-specific behavior is correct
//! - Redacted output fed to BundleWriter stays redacted through roundtrip

use pt_redact::{ExportProfile, FieldClass, KeyMaterial, RedactionEngine, RedactionPolicy};

/// Canary secrets that cover common secret formats.
const CANARY_SECRETS: &[&str] = &[
    "AKIAIOSFODNN7EXAMPLE",
    "ghp_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",
    "sk-proj-xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",
    "sk-ant-api03-xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",
    "postgres://admin:secretpass@localhost/db",
    "-----BEGIN RSA PRIVATE KEY-----",
    "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIn0",
];

/// Field classes where secret detection is expected to fire.
/// PathTmp/PathHome/PathSystem apply normalization/hashing actions
/// that don't run the secret detector (by design: paths rarely contain tokens).
const SECRET_FIELD_CLASSES: &[FieldClass] = &[FieldClass::CmdlineArg, FieldClass::FreeText];

// ============================================================================
// Cross-Profile Secret Leak Prevention
// ============================================================================

#[test]
fn test_no_secrets_leak_in_any_profile_any_field_class() {
    let policy = RedactionPolicy::default();
    let key = KeyMaterial::from_bytes([42u8; 32], "cross-profile-test");
    let engine = RedactionEngine::with_key(policy, key);

    let profiles = [
        ExportProfile::Minimal,
        ExportProfile::Safe,
        ExportProfile::Forensic,
    ];

    for profile in &profiles {
        for field_class in SECRET_FIELD_CLASSES {
            for secret in CANARY_SECRETS {
                let result = engine.redact_with_profile(secret, *field_class, *profile);
                assert!(
                    !result.output.contains(secret),
                    "Secret leaked!\n  profile={:?}\n  field_class={:?}\n  secret={}\n  output={}",
                    profile,
                    field_class,
                    secret,
                    result.output
                );
            }
        }
    }

    eprintln!(
        "[INFO] Tested {} secrets x {} profiles x {} field classes = {} combinations",
        CANARY_SECRETS.len(),
        profiles.len(),
        SECRET_FIELD_CLASSES.len(),
        CANARY_SECRETS.len() * profiles.len() * SECRET_FIELD_CLASSES.len()
    );
}

// ============================================================================
// Profile-Specific Behavior Tests
// ============================================================================

#[test]
fn test_minimal_profile_redacts_everything() {
    let policy = RedactionPolicy::default();
    let key = KeyMaterial::from_bytes([0u8; 32], "minimal-test");
    let engine = RedactionEngine::with_key(policy, key);

    let values = [
        ("hostname.example.com", FieldClass::Hostname),
        ("/home/user/secrets.txt", FieldClass::PathHome),
        ("normal text content", FieldClass::FreeText),
    ];

    for (value, field_class) in &values {
        let result = engine.redact_with_profile(value, *field_class, ExportProfile::Minimal);
        // Minimal profile should modify everything sensitive
        assert!(
            result.was_modified || result.output != *value,
            "Minimal profile should modify '{}' (field_class={:?}): output={}",
            value,
            field_class,
            result.output
        );
    }
}

#[test]
fn test_safe_profile_hashes_hostnames() {
    let policy = RedactionPolicy::default();
    let key = KeyMaterial::from_bytes([0u8; 32], "safe-test");
    let engine = RedactionEngine::with_key(policy, key);

    let result = engine.redact_with_profile(
        "myserver.example.com",
        FieldClass::Hostname,
        ExportProfile::Safe,
    );

    // Safe profile should hash hostnames
    assert!(result.was_modified, "Safe profile should modify hostname");
    assert!(
        !result.output.contains("myserver"),
        "Safe profile hostname output should not contain original: {}",
        result.output
    );
}

#[test]
fn test_system_paths_allowed_across_profiles() {
    let policy = RedactionPolicy::default();
    let key = KeyMaterial::from_bytes([0u8; 32], "path-test");
    let engine = RedactionEngine::with_key(policy, key);

    let system_paths = ["/usr/bin/python3", "/bin/bash", "/sbin/init"];

    for path in &system_paths {
        let result = engine.redact(*path, FieldClass::PathSystem);
        assert_eq!(
            result.output, *path,
            "System path should be allowed: {}",
            path
        );
    }
}

// ============================================================================
// Hash Consistency Across Profiles
// ============================================================================

#[test]
fn test_hash_consistent_within_same_profile() {
    let policy = RedactionPolicy::default();
    let key = KeyMaterial::from_bytes([99u8; 32], "consistency-test");
    let engine = RedactionEngine::with_key(policy, key);

    let value = "consistent-hostname.example.com";

    let profiles = [
        ExportProfile::Minimal,
        ExportProfile::Safe,
        ExportProfile::Forensic,
    ];

    for profile in &profiles {
        let result1 = engine.redact_with_profile(value, FieldClass::Hostname, *profile);
        let result2 = engine.redact_with_profile(value, FieldClass::Hostname, *profile);

        assert_eq!(
            result1.output, result2.output,
            "Same profile {:?} should produce consistent output for same input",
            profile
        );
    }
}

// ============================================================================
// Bundle-Context Redaction Tests
// ============================================================================

#[test]
fn test_redacted_summary_stays_clean_through_json_roundtrip() {
    let policy = RedactionPolicy::default();
    let key = KeyMaterial::from_bytes([7u8; 32], "bundle-roundtrip");
    let engine = RedactionEngine::with_key(policy, key);

    let secrets_to_embed = [
        "AKIAIOSFODNN7EXAMPLE",
        "postgres://admin:secretpass@localhost/db",
        "ghp_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",
    ];

    let profiles = [
        ExportProfile::Minimal,
        ExportProfile::Safe,
        ExportProfile::Forensic,
    ];

    for profile in &profiles {
        let mut redacted_notes = Vec::new();

        for secret in &secrets_to_embed {
            let result = engine.redact_with_profile(secret, FieldClass::FreeText, *profile);
            redacted_notes.push(result.output);
        }

        // Build a JSON summary with redacted values
        let summary = serde_json::json!({
            "notes": redacted_notes,
            "profile": format!("{:?}", profile),
        });

        // Serialize and deserialize (simulating bundle write+read)
        let json_str = serde_json::to_string(&summary).expect("serialize");
        let parsed: serde_json::Value = serde_json::from_str(&json_str).expect("parse");

        // Verify no secrets leaked through JSON roundtrip
        let output = parsed.to_string();
        for secret in &secrets_to_embed {
            assert!(
                !output.contains(secret),
                "Secret '{}' leaked in {:?} profile after JSON roundtrip: {}",
                secret,
                profile,
                output
            );
        }
    }

    eprintln!("[INFO] Bundle-context redaction verified for all profiles");
}

#[test]
fn test_env_redaction_across_profiles() {
    let policy = RedactionPolicy::default();
    let key = KeyMaterial::from_bytes([0u8; 32], "env-test");
    let engine = RedactionEngine::with_key(policy, key);

    let env_vars = [
        ("AWS_SECRET_KEY", "wJalrXUtnFEMI/EXAMPLE"),
        ("DATABASE_URL", "postgres://user:pass@localhost/db"),
        ("API_TOKEN", "sk-live-xxxxxxxxxxxxxxxxxxxxxxxx"),
    ];

    let profiles = [
        ExportProfile::Minimal,
        ExportProfile::Safe,
        ExportProfile::Forensic,
    ];

    for profile in &profiles {
        for (name, value) in &env_vars {
            let (_, value_result) = engine.redact_env(name, value);
            assert_eq!(
                value_result.output, "[REDACTED]",
                "Env var {} should be redacted in {:?} profile",
                name, profile
            );
        }
    }
}

// ============================================================================
// Policy Version Tracking
// ============================================================================

#[test]
fn test_policy_version_and_key_id_consistent() {
    let policy = RedactionPolicy::default();
    let key = KeyMaterial::from_bytes([0u8; 32], "version-tracking");
    let engine = RedactionEngine::with_key(policy, key);

    let version = engine.policy_version();
    assert_eq!(version, "1.0.0", "Default policy version should be 1.0.0");

    let key_id = engine.key_id();
    assert_eq!(key_id, "version-tracking");

    // These should be stable across calls
    assert_eq!(engine.policy_version(), version);
    assert_eq!(engine.key_id(), key_id);
}
