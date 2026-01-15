//! Secret detection using pattern matching and entropy analysis.
//!
//! Automatically detects sensitive data like API keys, tokens, and passwords
//! using regex patterns and Shannon entropy analysis.

use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};

/// Type of detected secret.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SecretType {
    /// AWS access key (AKIA...)
    AwsAccessKey,
    /// AWS secret key
    AwsSecretKey,
    /// GitHub personal access token
    GitHubToken,
    /// GitLab personal access token
    GitLabToken,
    /// Slack token (xoxb-...)
    SlackToken,
    /// JSON Web Token
    Jwt,
    /// Private key (PEM format)
    PrivateKey,
    /// Password in argument
    PasswordArg,
    /// Token in argument
    TokenArg,
    /// API key in argument
    ApiKeyArg,
    /// Secret environment variable
    SecretEnvVar,
    /// Database connection string
    ConnectionString,
    /// High-entropy string (possible secret)
    HighEntropy,
    /// Generic sensitive argument
    SensitiveArg,
    /// OpenAI/Anthropic API key
    AiApiKey,
    /// Generic API key pattern
    GenericApiKey,
}

impl SecretType {
    /// Returns the recommended action for this secret type.
    pub fn recommended_action(&self) -> crate::Action {
        use crate::Action;
        match self {
            // Always redact these - too dangerous to hash
            SecretType::AwsAccessKey
            | SecretType::AwsSecretKey
            | SecretType::GitHubToken
            | SecretType::GitLabToken
            | SecretType::SlackToken
            | SecretType::Jwt
            | SecretType::PrivateKey
            | SecretType::PasswordArg
            | SecretType::TokenArg
            | SecretType::ApiKeyArg
            | SecretType::SecretEnvVar
            | SecretType::AiApiKey => Action::Redact,

            // Normalize connection strings (remove credentials, keep structure)
            SecretType::ConnectionString => Action::NormalizeHash,

            // Hash high-entropy strings (might be useful for pattern matching)
            SecretType::HighEntropy | SecretType::GenericApiKey => Action::Hash,

            // Redact generic sensitive args
            SecretType::SensitiveArg => Action::Redact,
        }
    }
}

/// Detection pattern definition.
struct DetectionPattern {
    secret_type: SecretType,
    pattern: Lazy<Regex>,
    description: &'static str,
}

// Pre-compiled detection patterns
static PATTERNS: &[DetectionPattern] = &[
    DetectionPattern {
        secret_type: SecretType::AwsAccessKey,
        pattern: Lazy::new(|| Regex::new(r"AKIA[0-9A-Z]{16}").unwrap()),
        description: "AWS Access Key ID",
    },
    DetectionPattern {
        secret_type: SecretType::GitHubToken,
        pattern: Lazy::new(|| Regex::new(r"gh[pousr]_[A-Za-z0-9_]{36,}").unwrap()),
        description: "GitHub Personal Access Token",
    },
    DetectionPattern {
        secret_type: SecretType::GitLabToken,
        pattern: Lazy::new(|| Regex::new(r"glpat-[A-Za-z0-9\-_]{20,}").unwrap()),
        description: "GitLab Personal Access Token",
    },
    DetectionPattern {
        secret_type: SecretType::SlackToken,
        pattern: Lazy::new(|| Regex::new(r"xox[baprs]-[A-Za-z0-9\-]+").unwrap()),
        description: "Slack Token",
    },
    DetectionPattern {
        secret_type: SecretType::Jwt,
        pattern: Lazy::new(|| {
            Regex::new(r"eyJ[A-Za-z0-9_-]+\.eyJ[A-Za-z0-9_-]+\.[A-Za-z0-9_-]+").unwrap()
        }),
        description: "JSON Web Token",
    },
    DetectionPattern {
        secret_type: SecretType::PrivateKey,
        pattern: Lazy::new(|| Regex::new(r"-----BEGIN[A-Z ]*PRIVATE KEY-----").unwrap()),
        description: "Private Key (PEM)",
    },
    DetectionPattern {
        secret_type: SecretType::AiApiKey,
        pattern: Lazy::new(|| {
            // OpenAI: sk-..., Anthropic: sk-ant-...
            Regex::new(r"sk-(?:ant-)?[A-Za-z0-9_-]{20,}").unwrap()
        }),
        description: "AI API Key (OpenAI/Anthropic)",
    },
];

// Argument patterns (for command line detection)
static ARG_PATTERNS: &[DetectionPattern] = &[
    DetectionPattern {
        secret_type: SecretType::PasswordArg,
        pattern: Lazy::new(|| Regex::new(r"--password[=\s]+[^\s]+").unwrap()),
        description: "Password argument",
    },
    DetectionPattern {
        secret_type: SecretType::TokenArg,
        pattern: Lazy::new(|| Regex::new(r"--token[=\s]+[^\s]+").unwrap()),
        description: "Token argument",
    },
    DetectionPattern {
        secret_type: SecretType::ApiKeyArg,
        pattern: Lazy::new(|| Regex::new(r"--api[-_]?key[=\s]+[^\s]+").unwrap()),
        description: "API key argument",
    },
];

// Environment variable patterns
static ENV_SECRET_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?i).*(KEY|TOKEN|SECRET|PASSWORD|CREDENTIAL|AUTH).*=.+").unwrap()
});

// Connection string patterns
static CONNECTION_STRING_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?i)(postgres|mysql|mongodb|redis|amqp)://[^@]+@").unwrap()
});

/// Secret detector for automatic sensitivity detection.
#[derive(Clone)]
pub struct SecretDetector {
    /// Entropy threshold for high-entropy detection.
    entropy_threshold: f64,
    /// Minimum length for entropy analysis.
    min_entropy_length: usize,
    /// Custom patterns to detect.
    custom_patterns: Vec<(Regex, SecretType)>,
}

impl SecretDetector {
    /// Create a new secret detector with default settings.
    pub fn new() -> Self {
        Self {
            entropy_threshold: 4.5,
            min_entropy_length: 16,
            custom_patterns: Vec::new(),
        }
    }

    /// Create a detector with a custom entropy threshold.
    pub fn with_entropy_threshold(threshold: f64) -> Self {
        Self {
            entropy_threshold: threshold,
            min_entropy_length: 16,
            custom_patterns: Vec::new(),
        }
    }

    /// Add a custom detection pattern.
    pub fn add_pattern(&mut self, pattern: Regex, secret_type: SecretType) {
        self.custom_patterns.push((pattern, secret_type));
    }

    /// Detect if a value contains a secret.
    pub fn detect(&self, value: &str) -> Option<SecretType> {
        // Check explicit patterns first (most specific)
        for pattern in PATTERNS {
            if pattern.pattern.is_match(value) {
                return Some(pattern.secret_type);
            }
        }

        // Check argument patterns
        for pattern in ARG_PATTERNS {
            if pattern.pattern.is_match(value) {
                return Some(pattern.secret_type);
            }
        }

        // Check connection strings
        if CONNECTION_STRING_PATTERN.is_match(value) {
            return Some(SecretType::ConnectionString);
        }

        // Check custom patterns
        for (pattern, secret_type) in &self.custom_patterns {
            if pattern.is_match(value) {
                return Some(*secret_type);
            }
        }

        // Check for high entropy (possible secret)
        if self.is_high_entropy(value) {
            return Some(SecretType::HighEntropy);
        }

        None
    }

    /// Detect secrets in environment variable context.
    pub fn detect_env(&self, name: &str, value: &str) -> Option<SecretType> {
        // Check if name suggests a secret
        let name_upper = name.to_uppercase();
        if name_upper.contains("KEY")
            || name_upper.contains("TOKEN")
            || name_upper.contains("SECRET")
            || name_upper.contains("PASSWORD")
            || name_upper.contains("CREDENTIAL")
            || name_upper.contains("AUTH")
        {
            return Some(SecretType::SecretEnvVar);
        }

        // Check value for patterns
        self.detect(value)
    }

    /// Detect secrets in command line argument context.
    pub fn detect_arg(&self, arg: &str, prev_arg: Option<&str>) -> Option<SecretType> {
        // Check if previous arg was a sensitive flag
        if let Some(prev) = prev_arg {
            let prev_lower = prev.to_lowercase();
            if prev_lower == "--password"
                || prev_lower == "--token"
                || prev_lower == "--api-key"
                || prev_lower == "--apikey"
                || prev_lower == "--secret"
            {
                return Some(SecretType::SensitiveArg);
            }
        }

        // Check the argument itself
        self.detect(arg)
    }

    /// Check if a string has high entropy (likely a secret).
    pub fn is_high_entropy(&self, value: &str) -> bool {
        if value.len() < self.min_entropy_length {
            return false;
        }

        // Skip if it looks like a normal word or path
        if value.chars().all(|c| c.is_ascii_alphabetic() || c == '_' || c == '-') {
            return false;
        }

        let entropy = shannon_entropy(value);
        entropy > self.entropy_threshold
    }

    /// Get the entropy of a string.
    pub fn entropy(&self, value: &str) -> f64 {
        shannon_entropy(value)
    }
}

impl Default for SecretDetector {
    fn default() -> Self {
        Self::new()
    }
}

/// Calculate Shannon entropy of a string.
///
/// Higher entropy suggests more randomness (typical of secrets).
/// Base64-encoded secrets typically have entropy > 4.5.
pub fn shannon_entropy(value: &str) -> f64 {
    if value.is_empty() {
        return 0.0;
    }

    let mut freq = [0u32; 256];
    let len = value.len() as f64;

    for byte in value.bytes() {
        freq[byte as usize] += 1;
    }

    let mut entropy = 0.0;
    for &count in &freq {
        if count > 0 {
            let p = count as f64 / len;
            entropy -= p * p.log2();
        }
    }

    entropy
}

/// Detection result with context.
#[derive(Debug, Clone)]
pub struct Detection {
    /// Type of secret detected.
    pub secret_type: SecretType,
    /// Start position in the input.
    pub start: usize,
    /// End position in the input.
    pub end: usize,
    /// The matched text (should be redacted before display).
    matched: String,
}

impl Detection {
    /// Get a redacted version of the match for logging.
    pub fn redacted_match(&self) -> String {
        if self.matched.len() <= 8 {
            "[REDACTED]".to_string()
        } else {
            format!("{}...[REDACTED]", &self.matched[..4])
        }
    }
}

/// Find all secrets in a string with their positions.
pub fn find_all_secrets(value: &str) -> Vec<Detection> {
    let mut detections = Vec::new();

    // Check each pattern
    for pattern in PATTERNS {
        for m in pattern.pattern.find_iter(value) {
            detections.push(Detection {
                secret_type: pattern.secret_type,
                start: m.start(),
                end: m.end(),
                matched: m.as_str().to_string(),
            });
        }
    }

    // Check argument patterns
    for pattern in ARG_PATTERNS {
        for m in pattern.pattern.find_iter(value) {
            detections.push(Detection {
                secret_type: pattern.secret_type,
                start: m.start(),
                end: m.end(),
                matched: m.as_str().to_string(),
            });
        }
    }

    // Check connection strings
    for m in CONNECTION_STRING_PATTERN.find_iter(value) {
        detections.push(Detection {
            secret_type: SecretType::ConnectionString,
            start: m.start(),
            end: m.end(),
            matched: m.as_str().to_string(),
        });
    }

    // Sort by position
    detections.sort_by_key(|d| d.start);

    detections
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_aws_key() {
        let detector = SecretDetector::new();
        let result = detector.detect("AKIAIOSFODNN7EXAMPLE");
        assert_eq!(result, Some(SecretType::AwsAccessKey));
    }

    #[test]
    fn test_detect_github_token() {
        let detector = SecretDetector::new();
        let result = detector.detect("ghp_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx");
        assert_eq!(result, Some(SecretType::GitHubToken));
    }

    #[test]
    fn test_detect_jwt() {
        let detector = SecretDetector::new();
        let jwt = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c";
        let result = detector.detect(jwt);
        assert_eq!(result, Some(SecretType::Jwt));
    }

    #[test]
    fn test_detect_private_key() {
        let detector = SecretDetector::new();
        let result = detector.detect("-----BEGIN RSA PRIVATE KEY-----");
        assert_eq!(result, Some(SecretType::PrivateKey));
    }

    #[test]
    fn test_detect_password_arg() {
        let detector = SecretDetector::new();
        let result = detector.detect("--password=secret123");
        assert_eq!(result, Some(SecretType::PasswordArg));
    }

    #[test]
    fn test_detect_token_arg() {
        let detector = SecretDetector::new();
        let result = detector.detect("--token abc123xyz");
        assert_eq!(result, Some(SecretType::TokenArg));
    }

    #[test]
    fn test_detect_env_secret() {
        let detector = SecretDetector::new();

        assert_eq!(
            detector.detect_env("AWS_SECRET_KEY", "anything"),
            Some(SecretType::SecretEnvVar)
        );
        assert_eq!(
            detector.detect_env("DATABASE_PASSWORD", "anything"),
            Some(SecretType::SecretEnvVar)
        );
        assert_eq!(
            detector.detect_env("AUTH_TOKEN", "anything"),
            Some(SecretType::SecretEnvVar)
        );
    }

    #[test]
    fn test_detect_connection_string() {
        let detector = SecretDetector::new();
        let result = detector.detect("postgres://user:pass@localhost/db");
        assert_eq!(result, Some(SecretType::ConnectionString));
    }

    #[test]
    fn test_detect_ai_api_key() {
        let detector = SecretDetector::new();

        // OpenAI style
        let result = detector.detect("sk-proj-abcdefghijklmnopqrstuvwxyz");
        assert_eq!(result, Some(SecretType::AiApiKey));

        // Anthropic style
        let result = detector.detect("sk-ant-api03-abcdefghijklmnopqrstuvwxyz");
        assert_eq!(result, Some(SecretType::AiApiKey));
    }

    #[test]
    fn test_entropy_calculation() {
        // Low entropy (repeated characters)
        let low = shannon_entropy("aaaaaaaaaaaaaaaa");
        assert!(low < 1.0);

        // High entropy (random-looking)
        let high = shannon_entropy("aB3$xY9@kL5#mN7!");
        assert!(high > 3.5);

        // Base64-like (typical secrets)
        let b64 = shannon_entropy("SGVsbG8gV29ybGQhIQ==");
        assert!(b64 > 3.0);
    }

    #[test]
    fn test_high_entropy_detection() {
        let detector = SecretDetector::new();

        // Should detect high entropy
        assert!(detector.is_high_entropy("aB3cD4eF5gH6iJ7kL8mN9"));

        // Should not detect normal words
        assert!(!detector.is_high_entropy("hello_world_test"));

        // Too short
        assert!(!detector.is_high_entropy("short"));
    }

    #[test]
    fn test_find_all_secrets() {
        let input = "curl --token ghp_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx https://api.github.com";
        let detections = find_all_secrets(input);

        assert!(detections.len() >= 2);
        assert!(detections.iter().any(|d| d.secret_type == SecretType::TokenArg));
        assert!(detections
            .iter()
            .any(|d| d.secret_type == SecretType::GitHubToken));
    }

    #[test]
    fn test_no_false_positives() {
        let detector = SecretDetector::new();

        // Normal paths should not trigger
        assert!(detector.detect("/usr/bin/python3").is_none());
        assert!(detector.detect("/home/user/project").is_none());

        // Normal arguments should not trigger
        assert!(detector.detect("--verbose").is_none());
        assert!(detector.detect("--output=/tmp/test.log").is_none());

        // Normal env vars should not trigger
        assert!(detector.detect_env("PATH", "/usr/bin").is_none());
        assert!(detector.detect_env("HOME", "/home/user").is_none());
    }

    #[test]
    fn test_detection_redacted_match() {
        let input = "ghp_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx";
        let detections = find_all_secrets(input);

        assert!(!detections.is_empty());
        let detection = &detections[0];
        let redacted = detection.redacted_match();

        // Should not contain the full secret
        assert!(!redacted.contains("xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"));
    }
}
