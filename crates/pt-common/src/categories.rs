//! Command and CWD category taxonomies for process classification.
//!
//! This module defines the taxonomies for categorizing processes by:
//! - Command type (test runner, dev server, agent, daemon, etc.)
//! - Working directory (project, system, temp, home)
//!
//! Categories feed into the Bayesian inference as Dirichlet-Categorical evidence.
//! The categorization affects prior probabilities for each process class.
//!
//! # Categorization Output
//!
//! The [`CategorizationOutput`] struct provides:
//! - `cmd_category`: The detected command category (g)
//! - `cwd_category`: The detected CWD category
//! - `cmd_signature`: A stable, hashed representation for grouping similar commands
//! - `cmd_short`: Optional display-safe short form of the command
//!
//! # Versioning
//!
//! Category mappings are versioned (see [`CATEGORIES_SCHEMA_VERSION`]) to ensure
//! deterministic, reproducible categorization across sessions.

use regex::Regex;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

/// Schema version for category taxonomy.
pub const CATEGORIES_SCHEMA_VERSION: &str = "1.0.0";

/// Command categories for process classification.
///
/// These categories affect the prior probability of a process being
/// useful, abandoned, or zombie based on command type patterns.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CommandCategory {
    /// Test runners: bun test, jest, pytest, mocha, cargo test, go test
    Test,
    /// Development servers: next dev, vite, webpack-dev-server, nodemon
    DevServer,
    /// AI/coding agents: claude, codex, copilot, gemini, cursor
    Agent,
    /// Production servers: gunicorn, nginx, apache, uvicorn, node server
    Server,
    /// System daemons: systemd services, cron, docker, supervisord
    Daemon,
    /// Build tools: webpack, esbuild, tsc, cargo build, make, gradle
    Build,
    /// Editors/IDEs: code, vim, nvim, emacs, nano, cursor
    Editor,
    /// Interactive shells: bash, zsh, fish, sh, pwsh
    Shell,
    /// Database clients: psql, mysql, mongo, redis-cli
    Database,
    /// Version control: git operations (not just git command)
    Vcs,
    /// Package managers: npm, yarn, pip, cargo, brew
    PackageManager,
    /// Container tools: docker, podman, kubectl, docker-compose
    Container,
    /// Unknown/other - default category
    Unknown,
}

impl CommandCategory {
    /// Get all category variants in order (matches Dirichlet parameter order).
    pub fn all() -> &'static [CommandCategory] {
        &[
            CommandCategory::Test,
            CommandCategory::DevServer,
            CommandCategory::Agent,
            CommandCategory::Server,
            CommandCategory::Daemon,
            CommandCategory::Build,
            CommandCategory::Editor,
            CommandCategory::Shell,
            CommandCategory::Database,
            CommandCategory::Vcs,
            CommandCategory::PackageManager,
            CommandCategory::Container,
            CommandCategory::Unknown,
        ]
    }

    /// Get the index of this category (for Dirichlet parameters).
    pub fn index(&self) -> usize {
        Self::all().iter().position(|c| c == self).unwrap_or(12)
    }

    /// Get category from index.
    pub fn from_index(idx: usize) -> Self {
        Self::all().get(idx).copied().unwrap_or(CommandCategory::Unknown)
    }

    /// Get human-readable name.
    pub fn name(&self) -> &'static str {
        match self {
            CommandCategory::Test => "test",
            CommandCategory::DevServer => "devserver",
            CommandCategory::Agent => "agent",
            CommandCategory::Server => "server",
            CommandCategory::Daemon => "daemon",
            CommandCategory::Build => "build",
            CommandCategory::Editor => "editor",
            CommandCategory::Shell => "shell",
            CommandCategory::Database => "database",
            CommandCategory::Vcs => "vcs",
            CommandCategory::PackageManager => "package_manager",
            CommandCategory::Container => "container",
            CommandCategory::Unknown => "unknown",
        }
    }
}

impl Default for CommandCategory {
    fn default() -> Self {
        CommandCategory::Unknown
    }
}

/// CWD (Current Working Directory) categories.
///
/// Working directory context affects interpretation of process behavior.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CwdCategory {
    /// User project directories: ~/projects/*, ~/code/*, ~/repos/*
    Project,
    /// System directories: /usr, /var, /etc, /opt
    System,
    /// Temporary directories: /tmp, /var/tmp, /private/tmp
    Temp,
    /// Home directory root: ~, /home/user
    Home,
    /// Application data: ~/.config/*, ~/.local/*
    AppData,
    /// Runtime directories: /run, /var/run
    Runtime,
    /// Root filesystem: /
    Root,
    /// Unknown/other
    Unknown,
}

impl CwdCategory {
    /// Get all category variants in order.
    pub fn all() -> &'static [CwdCategory] {
        &[
            CwdCategory::Project,
            CwdCategory::System,
            CwdCategory::Temp,
            CwdCategory::Home,
            CwdCategory::AppData,
            CwdCategory::Runtime,
            CwdCategory::Root,
            CwdCategory::Unknown,
        ]
    }

    /// Get the index of this category.
    pub fn index(&self) -> usize {
        Self::all().iter().position(|c| c == self).unwrap_or(7)
    }

    /// Get category from index.
    pub fn from_index(idx: usize) -> Self {
        Self::all().get(idx).copied().unwrap_or(CwdCategory::Unknown)
    }

    /// Get human-readable name.
    pub fn name(&self) -> &'static str {
        match self {
            CwdCategory::Project => "project",
            CwdCategory::System => "system",
            CwdCategory::Temp => "temp",
            CwdCategory::Home => "home",
            CwdCategory::AppData => "appdata",
            CwdCategory::Runtime => "runtime",
            CwdCategory::Root => "root",
            CwdCategory::Unknown => "unknown",
        }
    }
}

impl Default for CwdCategory {
    fn default() -> Self {
        CwdCategory::Unknown
    }
}

/// Output of command and CWD categorization.
///
/// This struct provides the stable, versioned output of categorizing a process
/// by its command line and working directory. All fields are safe to persist
/// in telemetry without leaking sensitive information.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CategorizationOutput {
    /// The detected command category (g in the inference model).
    pub cmd_category: CommandCategory,

    /// The detected working directory category.
    pub cwd_category: CwdCategory,

    /// Stable hash signature for grouping similar commands.
    ///
    /// This is a SHA-256 hash of normalized command tokens, truncated to 16 hex chars.
    /// Format: `cmd:<hash>` (e.g., `cmd:a1b2c3d4e5f6g7h8`)
    ///
    /// The normalization process:
    /// 1. Extract the base command (first token or executable name)
    /// 2. Extract significant flags/subcommands (sorted, deduplicated)
    /// 3. Hash the normalized representation
    pub cmd_signature: String,

    /// Display-safe short form of the command.
    ///
    /// This is the base command plus category indicator, suitable for display
    /// in TUI or agent outputs. Example: `jest (test)` or `next dev (devserver)`
    ///
    /// This field respects redaction by only showing the detected program name,
    /// never arguments or paths.
    pub cmd_short: String,

    /// Categorization schema version for reproducibility tracking.
    pub schema_version: String,
}

impl CategorizationOutput {
    /// Get the command category index for Dirichlet parameters.
    pub fn cmd_index(&self) -> usize {
        self.cmd_category.index()
    }

    /// Get the CWD category index.
    pub fn cwd_index(&self) -> usize {
        self.cwd_category.index()
    }
}

/// Pattern rule for matching commands to categories.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandPattern {
    /// The category this pattern matches.
    pub category: CommandCategory,
    /// Regex pattern to match against command.
    pub pattern: String,
    /// Human-readable description of what this matches.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Example commands that match this pattern.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub examples: Vec<String>,
}

/// Pattern rule for matching CWD paths to categories.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CwdPattern {
    /// The category this pattern matches.
    pub category: CwdCategory,
    /// Glob or regex pattern to match against path.
    pub pattern: String,
    /// Whether this is a glob pattern (else regex).
    #[serde(default)]
    pub is_glob: bool,
    /// Human-readable description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Compiled category matcher for efficient categorization.
pub struct CategoryMatcher {
    command_patterns: Vec<(CommandCategory, Regex)>,
    cwd_patterns: Vec<(CwdCategory, Regex)>,
    /// Retained for debugging/future use; patterns are already compiled.
    #[allow(dead_code)]
    home_dir: Option<String>,
}

impl CategoryMatcher {
    /// Create a new category matcher with default patterns.
    pub fn new() -> Self {
        Self::with_home_dir(std::env::var("HOME").ok())
    }

    /// Create a matcher with a specific home directory.
    pub fn with_home_dir(home_dir: Option<String>) -> Self {
        let command_patterns = Self::default_command_patterns();
        let cwd_patterns = Self::default_cwd_patterns(&home_dir);

        Self {
            command_patterns,
            cwd_patterns,
            home_dir,
        }
    }

    /// Categorize a command string.
    pub fn categorize_command(&self, command: &str) -> CommandCategory {
        let command_lower = command.to_lowercase();

        for (category, regex) in &self.command_patterns {
            if regex.is_match(&command_lower) {
                return *category;
            }
        }

        CommandCategory::Unknown
    }

    /// Categorize a working directory path.
    pub fn categorize_cwd(&self, path: &str) -> CwdCategory {
        // Normalize path separators
        let path_normalized = path.replace('\\', "/");

        for (category, regex) in &self.cwd_patterns {
            if regex.is_match(&path_normalized) {
                return *category;
            }
        }

        CwdCategory::Unknown
    }

    /// Get command category index for Dirichlet parameters.
    pub fn command_index(&self, command: &str) -> usize {
        self.categorize_command(command).index()
    }

    /// Get CWD category index.
    pub fn cwd_index(&self, path: &str) -> usize {
        self.categorize_cwd(path).index()
    }

    /// Fully categorize a command and working directory.
    ///
    /// Returns a [`CategorizationOutput`] with:
    /// - Command and CWD categories
    /// - A stable hash signature for grouping
    /// - A display-safe short form
    ///
    /// # Example
    ///
    /// ```
    /// use pt_common::categories::CategoryMatcher;
    ///
    /// let matcher = CategoryMatcher::new();
    /// let output = matcher.categorize("jest --watch", "/home/user/projects/app");
    /// assert_eq!(output.cmd_category.name(), "test");
    /// assert!(output.cmd_signature.starts_with("cmd:"));
    /// ```
    pub fn categorize(&self, command: &str, cwd: &str) -> CategorizationOutput {
        let cmd_category = self.categorize_command(command);
        let cwd_category = self.categorize_cwd(cwd);
        let cmd_signature = self.compute_signature(command, &cmd_category);
        let cmd_short = self.compute_short_form(command, &cmd_category);

        CategorizationOutput {
            cmd_category,
            cwd_category,
            cmd_signature,
            cmd_short,
            schema_version: CATEGORIES_SCHEMA_VERSION.to_string(),
        }
    }

    /// Compute a stable hash signature for the command.
    ///
    /// The signature is computed from normalized command tokens:
    /// 1. Extract base command (executable name without path)
    /// 2. Include category for disambiguation
    /// 3. Extract significant flags (prefixed with - or --)
    /// 4. Sort and deduplicate tokens
    /// 5. Hash with SHA-256, truncate to 16 hex chars
    fn compute_signature(&self, command: &str, category: &CommandCategory) -> String {
        let normalized = Self::normalize_command_tokens(command, category);
        let mut hasher = Sha256::new();
        hasher.update(normalized.as_bytes());
        let hash = hasher.finalize();
        // Truncate to 8 bytes (16 hex chars) for compact storage
        let hex = hex::encode(&hash[..8]);
        format!("cmd:{}", hex)
    }

    /// Normalize command tokens for hashing.
    ///
    /// This produces a stable, sorted representation of the command that:
    /// - Extracts the base executable name
    /// - Includes the category for disambiguation
    /// - Preserves significant flags (sorted)
    /// - Strips arguments/values that might contain sensitive data
    fn normalize_command_tokens(command: &str, category: &CommandCategory) -> String {
        let tokens: Vec<&str> = command.split_whitespace().collect();
        if tokens.is_empty() {
            return format!("{}:unknown", category.name());
        }

        // Extract base command (last component of path)
        let base_cmd = tokens[0]
            .rsplit('/')
            .next()
            .unwrap_or(tokens[0])
            .rsplit('\\')
            .next()
            .unwrap_or(tokens[0]);

        // Extract flags (tokens starting with - or --)
        // Only keep the flag names, not their values
        let mut flags: Vec<&str> = tokens
            .iter()
            .skip(1)
            .filter(|t| t.starts_with('-'))
            .map(|t| {
                // Strip value from --flag=value patterns
                if let Some(idx) = t.find('=') {
                    &t[..idx]
                } else {
                    *t
                }
            })
            .collect();
        flags.sort();
        flags.dedup();

        // Build normalized representation: category:base_cmd:flags
        let flags_str = flags.join(",");
        format!("{}:{}:{}", category.name(), base_cmd, flags_str)
    }

    /// Compute a display-safe short form of the command.
    ///
    /// Format: `<base_command> (<category>)`
    /// Example: `jest (test)` or `next dev (devserver)`
    fn compute_short_form(&self, command: &str, category: &CommandCategory) -> String {
        let tokens: Vec<&str> = command.split_whitespace().collect();
        if tokens.is_empty() {
            return format!("unknown ({})", category.name());
        }

        // Extract base command
        let base_cmd = tokens[0]
            .rsplit('/')
            .next()
            .unwrap_or(tokens[0])
            .rsplit('\\')
            .next()
            .unwrap_or(tokens[0]);

        // For certain categories, include the subcommand if present
        let display = match category {
            CommandCategory::DevServer | CommandCategory::Test | CommandCategory::Build => {
                // Include second token if it looks like a subcommand (not a flag)
                if tokens.len() > 1 && !tokens[1].starts_with('-') {
                    format!("{} {}", base_cmd, tokens[1])
                } else {
                    base_cmd.to_string()
                }
            }
            CommandCategory::Vcs | CommandCategory::Container | CommandCategory::PackageManager => {
                // Always include subcommand for these (git status, docker run, etc.)
                if tokens.len() > 1 && !tokens[1].starts_with('-') {
                    format!("{} {}", base_cmd, tokens[1])
                } else {
                    base_cmd.to_string()
                }
            }
            _ => base_cmd.to_string(),
        };

        format!("{} ({})", display, category.name())
    }

    /// Build default command patterns.
    fn default_command_patterns() -> Vec<(CommandCategory, Regex)> {
        let patterns = vec![
            // Test runners (highest priority - specific patterns)
            (CommandCategory::Test, r"(^|[/\s])(bun\s+test|jest|pytest|mocha|vitest|cargo\s+test|go\s+test|npm\s+test|yarn\s+test|phpunit|rspec|minitest)(\s|$)"),
            (CommandCategory::Test, r"--test|--spec|\.test\.|\.spec\."),

            // AI/coding agents
            (CommandCategory::Agent, r"(^|[/\s])(claude|codex|copilot|gemini|cursor|aider|continue|cody)(\s|$|-|_)"),
            (CommandCategory::Agent, r"anthropic|openai.*agent|ai.*assistant"),

            // Development servers
            (CommandCategory::DevServer, r"(^|[/\s])(next\s+dev|vite|webpack.*dev|nodemon|ts-node-dev|vue-cli-service\s+serve)(\s|$)"),
            (CommandCategory::DevServer, r"--hot|--watch|--hmr|dev.*server|serve.*dev"),
            (CommandCategory::DevServer, r"(^|[/\s])npm\s+run\s+dev"),

            // Build tools
            (CommandCategory::Build, r"(^|[/\s])(webpack|esbuild|rollup|parcel|vite\s+build|tsc|cargo\s+build|make|gradle|maven|cmake)(\s|$)"),
            (CommandCategory::Build, r"npm\s+run\s+build|yarn\s+build"),

            // Editors/IDEs
            (CommandCategory::Editor, r"(^|[/\s])(code|code-server|cursor|vim|nvim|neovim|emacs|nano|subl|sublime|atom|idea|pycharm|webstorm)(\s|$)"),

            // Database clients
            (CommandCategory::Database, r"(^|[/\s])(psql|mysql|mongo|mongosh|redis-cli|sqlite3|pgcli|mycli)(\s|$)"),

            // Container tools
            (CommandCategory::Container, r"(^|[/\s])(docker|podman|kubectl|docker-compose|k9s|helm|minikube|kind)(\s|$)"),

            // Version control
            (CommandCategory::Vcs, r"(^|[/\s])(git|gh|hub|svn|hg|mercurial)(\s|$)"),

            // Package managers
            (CommandCategory::PackageManager, r"(^|[/\s])(npm|yarn|pnpm|pip|pip3|cargo|brew|apt|apt-get|dnf|yum|pacman)(\s|$)"),

            // Production servers
            (CommandCategory::Server, r"(^|[/\s])(gunicorn|uvicorn|nginx|apache|httpd|node\s+server|deno\s+serve|fastify|express)(\s|$)"),
            (CommandCategory::Server, r"--production|node_env=production"),

            // Daemons (broad patterns)
            (CommandCategory::Daemon, r"(^|[/\s])(systemd|cron|crond|sshd|dockerd|containerd|supervisord|pm2|launchd)(\s|$)"),
            (CommandCategory::Daemon, r"/usr/lib/systemd|/lib/systemd"),

            // Shells (last, as many things run in shells)
            // Match shell with optional path prefix: /bin/bash, /usr/bin/zsh, etc.
            (CommandCategory::Shell, r"(^|/)(bash|zsh|fish|sh|dash|tcsh|csh|ksh|pwsh|powershell)(\s|$)"),
            // Match login shells: -bash, -zsh, etc.
            (CommandCategory::Shell, r"^-(bash|zsh|fish|sh|dash|tcsh|csh|ksh)$"),
        ];

        patterns
            .into_iter()
            .filter_map(|(cat, pat)| {
                Regex::new(pat).ok().map(|r| (cat, r))
            })
            .collect()
    }

    /// Build default CWD patterns.
    fn default_cwd_patterns(home_dir: &Option<String>) -> Vec<(CwdCategory, Regex)> {
        let mut result = Vec::new();

        // Temp directories (highest priority for specific paths)
        if let Ok(r) = Regex::new(r"^(/tmp|/var/tmp|/private/tmp|C:\\Temp)(/|$)") {
            result.push((CwdCategory::Temp, r));
        }

        // Runtime directories
        if let Ok(r) = Regex::new(r"^(/run|/var/run)(/|$)") {
            result.push((CwdCategory::Runtime, r));
        }

        // System directories (/var/tmp is handled by Temp which is checked first)
        if let Ok(r) = Regex::new(r"^(/usr|/var|/etc|/opt|/lib|/sbin|/bin)(/|$)") {
            result.push((CwdCategory::System, r));
        }

        // Root
        if let Ok(r) = Regex::new(r"^/$") {
            result.push((CwdCategory::Root, r));
        }

        // Home-relative patterns
        if let Some(home) = home_dir {
            let home_escaped = regex::escape(home);

            // Project directories (must be before general home)
            let project_pattern = format!(
                r"^{}/(projects?|code|repos?|src|workspace|dev|github|gitlab|work)(/|$)",
                home_escaped
            );
            if let Ok(r) = Regex::new(&project_pattern) {
                result.push((CwdCategory::Project, r));
            }

            // App data directories
            let appdata_pattern = format!(
                r"^{}/(\.(config|local|cache)|Library|AppData)(/|$)",
                home_escaped
            );
            if let Ok(r) = Regex::new(&appdata_pattern) {
                result.push((CwdCategory::AppData, r));
            }

            // Home directory (general, lower priority)
            let home_pattern = format!(r"^{}(/|$)", home_escaped);
            if let Ok(r) = Regex::new(&home_pattern) {
                result.push((CwdCategory::Home, r));
            }
        }

        // Fallback project detection (git repos)
        if let Ok(r) = Regex::new(r"/.git$|/.git/") {
            result.push((CwdCategory::Project, r));
        }

        result
    }
}

impl Default for CategoryMatcher {
    fn default() -> Self {
        Self::new()
    }
}

/// Category taxonomy configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryTaxonomy {
    /// Schema version.
    pub schema_version: String,

    /// Command category definitions.
    pub command_categories: Vec<CommandCategoryDef>,

    /// CWD category definitions.
    pub cwd_categories: Vec<CwdCategoryDef>,

    /// Custom command patterns (in addition to defaults).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub custom_command_patterns: Vec<CommandPattern>,

    /// Custom CWD patterns (in addition to defaults).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub custom_cwd_patterns: Vec<CwdPattern>,
}

/// Command category definition with prior adjustment hints.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandCategoryDef {
    /// Category identifier.
    pub id: CommandCategory,

    /// Human-readable name.
    pub name: String,

    /// Description.
    pub description: String,

    /// Example commands.
    pub examples: Vec<String>,

    /// Prior probability adjustment hints.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prior_hints: Option<PriorHints>,
}

/// CWD category definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CwdCategoryDef {
    /// Category identifier.
    pub id: CwdCategory,

    /// Human-readable name.
    pub name: String,

    /// Description.
    pub description: String,

    /// Example paths.
    pub examples: Vec<String>,
}

/// Hints for prior probability adjustments.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriorHints {
    /// Higher values mean more likely to be abandoned (0.0 to 1.0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abandonment_tendency: Option<f64>,

    /// Expected typical runtime in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_runtime_secs: Option<u64>,

    /// Whether this category typically runs as daemon.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daemon_like: Option<bool>,
}

impl CategoryTaxonomy {
    /// Create default taxonomy.
    pub fn default_taxonomy() -> Self {
        Self {
            schema_version: CATEGORIES_SCHEMA_VERSION.to_string(),
            command_categories: vec![
                CommandCategoryDef {
                    id: CommandCategory::Test,
                    name: "Test Runner".to_string(),
                    description: "Test execution frameworks and runners".to_string(),
                    examples: vec![
                        "bun test".to_string(),
                        "jest".to_string(),
                        "pytest".to_string(),
                        "cargo test".to_string(),
                    ],
                    prior_hints: Some(PriorHints {
                        abandonment_tendency: Some(0.7),
                        expected_runtime_secs: Some(3600),
                        daemon_like: Some(false),
                    }),
                },
                CommandCategoryDef {
                    id: CommandCategory::DevServer,
                    name: "Development Server".to_string(),
                    description: "Hot-reloading development servers".to_string(),
                    examples: vec![
                        "next dev".to_string(),
                        "vite".to_string(),
                        "webpack-dev-server".to_string(),
                    ],
                    prior_hints: Some(PriorHints {
                        abandonment_tendency: Some(0.5),
                        expected_runtime_secs: Some(86400),
                        daemon_like: Some(true),
                    }),
                },
                CommandCategoryDef {
                    id: CommandCategory::Agent,
                    name: "AI/Coding Agent".to_string(),
                    description: "AI-powered coding assistants and agents".to_string(),
                    examples: vec![
                        "claude".to_string(),
                        "codex".to_string(),
                        "cursor".to_string(),
                    ],
                    prior_hints: Some(PriorHints {
                        abandonment_tendency: Some(0.6),
                        expected_runtime_secs: Some(7200),
                        daemon_like: Some(false),
                    }),
                },
                CommandCategoryDef {
                    id: CommandCategory::Server,
                    name: "Production Server".to_string(),
                    description: "Production application servers".to_string(),
                    examples: vec![
                        "gunicorn".to_string(),
                        "uvicorn".to_string(),
                        "nginx".to_string(),
                    ],
                    prior_hints: Some(PriorHints {
                        abandonment_tendency: Some(0.1),
                        expected_runtime_secs: None,
                        daemon_like: Some(true),
                    }),
                },
                CommandCategoryDef {
                    id: CommandCategory::Daemon,
                    name: "System Daemon".to_string(),
                    description: "System services and background daemons".to_string(),
                    examples: vec![
                        "systemd".to_string(),
                        "cron".to_string(),
                        "dockerd".to_string(),
                    ],
                    prior_hints: Some(PriorHints {
                        abandonment_tendency: Some(0.05),
                        expected_runtime_secs: None,
                        daemon_like: Some(true),
                    }),
                },
                CommandCategoryDef {
                    id: CommandCategory::Build,
                    name: "Build Tool".to_string(),
                    description: "Compilation and bundling tools".to_string(),
                    examples: vec![
                        "webpack".to_string(),
                        "cargo build".to_string(),
                        "tsc".to_string(),
                    ],
                    prior_hints: Some(PriorHints {
                        abandonment_tendency: Some(0.4),
                        expected_runtime_secs: Some(600),
                        daemon_like: Some(false),
                    }),
                },
                CommandCategoryDef {
                    id: CommandCategory::Editor,
                    name: "Editor/IDE".to_string(),
                    description: "Text editors and integrated development environments".to_string(),
                    examples: vec![
                        "code".to_string(),
                        "vim".to_string(),
                        "cursor".to_string(),
                    ],
                    prior_hints: Some(PriorHints {
                        abandonment_tendency: Some(0.2),
                        expected_runtime_secs: Some(28800),
                        daemon_like: Some(false),
                    }),
                },
                CommandCategoryDef {
                    id: CommandCategory::Shell,
                    name: "Interactive Shell".to_string(),
                    description: "Command-line shells".to_string(),
                    examples: vec![
                        "bash".to_string(),
                        "zsh".to_string(),
                        "fish".to_string(),
                    ],
                    prior_hints: Some(PriorHints {
                        abandonment_tendency: Some(0.3),
                        expected_runtime_secs: Some(3600),
                        daemon_like: Some(false),
                    }),
                },
                CommandCategoryDef {
                    id: CommandCategory::Database,
                    name: "Database Client".to_string(),
                    description: "Database CLI clients and shells".to_string(),
                    examples: vec![
                        "psql".to_string(),
                        "mysql".to_string(),
                        "mongosh".to_string(),
                    ],
                    prior_hints: Some(PriorHints {
                        abandonment_tendency: Some(0.4),
                        expected_runtime_secs: Some(1800),
                        daemon_like: Some(false),
                    }),
                },
                CommandCategoryDef {
                    id: CommandCategory::Vcs,
                    name: "Version Control".to_string(),
                    description: "Version control system commands".to_string(),
                    examples: vec![
                        "git".to_string(),
                        "gh".to_string(),
                        "svn".to_string(),
                    ],
                    prior_hints: Some(PriorHints {
                        abandonment_tendency: Some(0.2),
                        expected_runtime_secs: Some(60),
                        daemon_like: Some(false),
                    }),
                },
                CommandCategoryDef {
                    id: CommandCategory::PackageManager,
                    name: "Package Manager".to_string(),
                    description: "Package and dependency managers".to_string(),
                    examples: vec![
                        "npm".to_string(),
                        "cargo".to_string(),
                        "pip".to_string(),
                    ],
                    prior_hints: Some(PriorHints {
                        abandonment_tendency: Some(0.3),
                        expected_runtime_secs: Some(300),
                        daemon_like: Some(false),
                    }),
                },
                CommandCategoryDef {
                    id: CommandCategory::Container,
                    name: "Container Tool".to_string(),
                    description: "Container and orchestration tools".to_string(),
                    examples: vec![
                        "docker".to_string(),
                        "kubectl".to_string(),
                        "podman".to_string(),
                    ],
                    prior_hints: Some(PriorHints {
                        abandonment_tendency: Some(0.2),
                        expected_runtime_secs: Some(120),
                        daemon_like: Some(false),
                    }),
                },
                CommandCategoryDef {
                    id: CommandCategory::Unknown,
                    name: "Unknown".to_string(),
                    description: "Unrecognized command type".to_string(),
                    examples: vec![],
                    prior_hints: None,
                },
            ],
            cwd_categories: vec![
                CwdCategoryDef {
                    id: CwdCategory::Project,
                    name: "Project Directory".to_string(),
                    description: "User project and code directories".to_string(),
                    examples: vec![
                        "~/projects/myapp".to_string(),
                        "~/code/backend".to_string(),
                    ],
                },
                CwdCategoryDef {
                    id: CwdCategory::System,
                    name: "System Directory".to_string(),
                    description: "System paths (/usr, /var, /etc)".to_string(),
                    examples: vec![
                        "/usr/local/bin".to_string(),
                        "/var/log".to_string(),
                    ],
                },
                CwdCategoryDef {
                    id: CwdCategory::Temp,
                    name: "Temporary Directory".to_string(),
                    description: "Temporary file storage".to_string(),
                    examples: vec![
                        "/tmp".to_string(),
                        "/var/tmp".to_string(),
                    ],
                },
                CwdCategoryDef {
                    id: CwdCategory::Home,
                    name: "Home Directory".to_string(),
                    description: "User home directory root".to_string(),
                    examples: vec![
                        "~".to_string(),
                        "/home/user".to_string(),
                    ],
                },
                CwdCategoryDef {
                    id: CwdCategory::AppData,
                    name: "Application Data".to_string(),
                    description: "User application config and data".to_string(),
                    examples: vec![
                        "~/.config".to_string(),
                        "~/.local/share".to_string(),
                    ],
                },
                CwdCategoryDef {
                    id: CwdCategory::Runtime,
                    name: "Runtime Directory".to_string(),
                    description: "Runtime state directories".to_string(),
                    examples: vec![
                        "/run".to_string(),
                        "/var/run".to_string(),
                    ],
                },
                CwdCategoryDef {
                    id: CwdCategory::Root,
                    name: "Root".to_string(),
                    description: "Root filesystem".to_string(),
                    examples: vec!["/".to_string()],
                },
                CwdCategoryDef {
                    id: CwdCategory::Unknown,
                    name: "Unknown".to_string(),
                    description: "Unrecognized path".to_string(),
                    examples: vec![],
                },
            ],
            custom_command_patterns: vec![],
            custom_cwd_patterns: vec![],
        }
    }
}

impl Default for CategoryTaxonomy {
    fn default() -> Self {
        Self::default_taxonomy()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_category_indexing() {
        assert_eq!(CommandCategory::Test.index(), 0);
        assert_eq!(CommandCategory::Unknown.index(), 12);
        assert_eq!(CommandCategory::from_index(0), CommandCategory::Test);
        assert_eq!(CommandCategory::from_index(100), CommandCategory::Unknown);
    }

    #[test]
    fn test_cwd_category_indexing() {
        assert_eq!(CwdCategory::Project.index(), 0);
        assert_eq!(CwdCategory::Unknown.index(), 7);
        assert_eq!(CwdCategory::from_index(0), CwdCategory::Project);
    }

    #[test]
    fn test_categorize_test_commands() {
        let matcher = CategoryMatcher::new();

        assert_eq!(matcher.categorize_command("bun test"), CommandCategory::Test);
        assert_eq!(matcher.categorize_command("/usr/bin/jest"), CommandCategory::Test);
        assert_eq!(matcher.categorize_command("pytest -v tests/"), CommandCategory::Test);
        assert_eq!(matcher.categorize_command("cargo test --lib"), CommandCategory::Test);
        assert_eq!(matcher.categorize_command("npm test"), CommandCategory::Test);
    }

    #[test]
    fn test_categorize_devserver_commands() {
        let matcher = CategoryMatcher::new();

        assert_eq!(matcher.categorize_command("next dev"), CommandCategory::DevServer);
        assert_eq!(matcher.categorize_command("vite --host"), CommandCategory::DevServer);
        assert_eq!(matcher.categorize_command("nodemon app.js"), CommandCategory::DevServer);
        assert_eq!(matcher.categorize_command("npm run dev"), CommandCategory::DevServer);
    }

    #[test]
    fn test_categorize_agent_commands() {
        let matcher = CategoryMatcher::new();

        assert_eq!(matcher.categorize_command("claude --version"), CommandCategory::Agent);
        assert_eq!(matcher.categorize_command("codex start"), CommandCategory::Agent);
        assert_eq!(matcher.categorize_command("cursor ."), CommandCategory::Agent);
    }

    #[test]
    fn test_categorize_shell_commands() {
        let matcher = CategoryMatcher::new();

        assert_eq!(matcher.categorize_command("bash"), CommandCategory::Shell);
        assert_eq!(matcher.categorize_command("-zsh"), CommandCategory::Shell);
        assert_eq!(matcher.categorize_command("/bin/fish"), CommandCategory::Shell);
    }

    #[test]
    fn test_categorize_unknown() {
        let matcher = CategoryMatcher::new();

        assert_eq!(matcher.categorize_command("some_random_command"), CommandCategory::Unknown);
        assert_eq!(matcher.categorize_command(""), CommandCategory::Unknown);
    }

    #[test]
    fn test_categorize_cwd() {
        let matcher = CategoryMatcher::with_home_dir(Some("/home/user".to_string()));

        assert_eq!(matcher.categorize_cwd("/tmp/test"), CwdCategory::Temp);
        assert_eq!(matcher.categorize_cwd("/var/tmp/session"), CwdCategory::Temp);
        assert_eq!(matcher.categorize_cwd("/usr/local/bin"), CwdCategory::System);
        assert_eq!(matcher.categorize_cwd("/home/user/projects/myapp"), CwdCategory::Project);
        assert_eq!(matcher.categorize_cwd("/home/user/.config/app"), CwdCategory::AppData);
        assert_eq!(matcher.categorize_cwd("/home/user"), CwdCategory::Home);
        assert_eq!(matcher.categorize_cwd("/run/user/1000"), CwdCategory::Runtime);
        assert_eq!(matcher.categorize_cwd("/"), CwdCategory::Root);
    }

    #[test]
    fn test_default_taxonomy() {
        let taxonomy = CategoryTaxonomy::default_taxonomy();

        assert_eq!(taxonomy.schema_version, CATEGORIES_SCHEMA_VERSION);
        assert_eq!(taxonomy.command_categories.len(), 13);
        assert_eq!(taxonomy.cwd_categories.len(), 8);

        // Check that test category has high abandonment tendency
        let test_cat = taxonomy.command_categories
            .iter()
            .find(|c| c.id == CommandCategory::Test)
            .unwrap();
        assert!(test_cat.prior_hints.as_ref().unwrap().abandonment_tendency.unwrap() > 0.5);
    }

    #[test]
    fn test_serialization_roundtrip() {
        let taxonomy = CategoryTaxonomy::default_taxonomy();
        let json = serde_json::to_string_pretty(&taxonomy).unwrap();
        let parsed: CategoryTaxonomy = serde_json::from_str(&json).unwrap();

        assert_eq!(taxonomy.schema_version, parsed.schema_version);
        assert_eq!(taxonomy.command_categories.len(), parsed.command_categories.len());
    }

    #[test]
    fn test_category_names() {
        assert_eq!(CommandCategory::Test.name(), "test");
        assert_eq!(CommandCategory::DevServer.name(), "devserver");
        assert_eq!(CwdCategory::Project.name(), "project");
        assert_eq!(CwdCategory::Temp.name(), "temp");
    }

    #[test]
    fn test_all_categories_covered() {
        // Ensure all() returns all variants
        let all_cmd = CommandCategory::all();
        assert!(all_cmd.contains(&CommandCategory::Test));
        assert!(all_cmd.contains(&CommandCategory::Unknown));
        assert_eq!(all_cmd.len(), 13);

        let all_cwd = CwdCategory::all();
        assert!(all_cwd.contains(&CwdCategory::Project));
        assert!(all_cwd.contains(&CwdCategory::Unknown));
        assert_eq!(all_cwd.len(), 8);
    }

    #[test]
    fn test_categorize_full_output() {
        let matcher = CategoryMatcher::with_home_dir(Some("/home/user".to_string()));
        let output = matcher.categorize("jest --watch", "/home/user/projects/app");

        assert_eq!(output.cmd_category, CommandCategory::Test);
        assert_eq!(output.cwd_category, CwdCategory::Project);
        assert!(output.cmd_signature.starts_with("cmd:"));
        assert_eq!(output.cmd_signature.len(), 4 + 16); // "cmd:" + 16 hex chars
        assert!(output.cmd_short.contains("test"));
        assert_eq!(output.schema_version, CATEGORIES_SCHEMA_VERSION);
    }

    #[test]
    fn test_signature_stability() {
        let matcher = CategoryMatcher::new();

        // Same command should produce same signature
        let sig1 = matcher.categorize("jest --watch", "/tmp").cmd_signature;
        let sig2 = matcher.categorize("jest --watch", "/tmp").cmd_signature;
        assert_eq!(sig1, sig2);

        // Different flag order should produce same signature (sorted)
        let sig3 = matcher.categorize("jest --coverage --watch", "/tmp").cmd_signature;
        let sig4 = matcher.categorize("jest --watch --coverage", "/tmp").cmd_signature;
        assert_eq!(sig3, sig4);
    }

    #[test]
    fn test_signature_differs_for_different_commands() {
        let matcher = CategoryMatcher::new();

        let sig1 = matcher.categorize("jest", "/tmp").cmd_signature;
        let sig2 = matcher.categorize("pytest", "/tmp").cmd_signature;
        assert_ne!(sig1, sig2);
    }

    #[test]
    fn test_signature_strips_arguments() {
        let matcher = CategoryMatcher::new();

        // Commands with different argument values should have same signature
        // (only flag names are preserved, not values)
        let sig1 = matcher.categorize("npm test --timeout=1000", "/tmp").cmd_signature;
        let sig2 = matcher.categorize("npm test --timeout=5000", "/tmp").cmd_signature;
        assert_eq!(sig1, sig2);
    }

    #[test]
    fn test_short_form_display() {
        let matcher = CategoryMatcher::new();

        // Test with subcommand
        let output = matcher.categorize("next dev --port 3000", "/tmp");
        assert_eq!(output.cmd_short, "next dev (devserver)");

        // Test simple command
        let output = matcher.categorize("vim", "/tmp");
        assert_eq!(output.cmd_short, "vim (editor)");

        // Test git with subcommand
        let output = matcher.categorize("git status", "/tmp");
        assert_eq!(output.cmd_short, "git status (vcs)");

        // Test docker with subcommand
        let output = matcher.categorize("docker run hello-world", "/tmp");
        assert_eq!(output.cmd_short, "docker run (container)");
    }

    #[test]
    fn test_short_form_strips_path() {
        let matcher = CategoryMatcher::new();

        let output = matcher.categorize("/usr/bin/jest --watch", "/tmp");
        assert_eq!(output.cmd_short, "jest (test)");
    }

    #[test]
    fn test_normalize_command_tokens() {
        // Test basic normalization
        let norm = CategoryMatcher::normalize_command_tokens("jest --watch", &CommandCategory::Test);
        assert!(norm.starts_with("test:jest:"));
        assert!(norm.contains("--watch"));

        // Test with path
        let norm = CategoryMatcher::normalize_command_tokens("/usr/bin/git status", &CommandCategory::Vcs);
        assert!(norm.starts_with("vcs:git:"));

        // Test flag value stripping
        let norm = CategoryMatcher::normalize_command_tokens("npm --registry=https://example.com", &CommandCategory::PackageManager);
        assert!(norm.contains("--registry"));
        assert!(!norm.contains("https://"));
    }

    #[test]
    fn test_categorization_output_indexes() {
        let matcher = CategoryMatcher::new();
        let output = matcher.categorize("jest", "/tmp");

        assert_eq!(output.cmd_index(), CommandCategory::Test.index());
        assert_eq!(output.cwd_index(), CwdCategory::Temp.index());
    }

    #[test]
    fn test_categorize_empty_command() {
        let matcher = CategoryMatcher::new();
        let output = matcher.categorize("", "/tmp");

        assert_eq!(output.cmd_category, CommandCategory::Unknown);
        assert_eq!(output.cmd_short, "unknown (unknown)");
        assert!(output.cmd_signature.starts_with("cmd:"));
    }

    #[test]
    fn test_categorization_output_serialization() {
        let matcher = CategoryMatcher::new();
        let output = matcher.categorize("jest --watch", "/tmp");

        let json = serde_json::to_string(&output).unwrap();
        let parsed: CategorizationOutput = serde_json::from_str(&json).unwrap();

        assert_eq!(output.cmd_category, parsed.cmd_category);
        assert_eq!(output.cwd_category, parsed.cwd_category);
        assert_eq!(output.cmd_signature, parsed.cmd_signature);
        assert_eq!(output.cmd_short, parsed.cmd_short);
        assert_eq!(output.schema_version, parsed.schema_version);
    }
}
