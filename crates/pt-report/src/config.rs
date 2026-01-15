//! Report configuration types.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Report color theme.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ReportTheme {
    /// Light theme.
    Light,
    /// Dark theme.
    Dark,
    /// Auto-detect from system preference.
    #[default]
    Auto,
}

impl ReportTheme {
    /// Get the CSS class for this theme.
    pub fn css_class(&self) -> &'static str {
        match self {
            ReportTheme::Light => "light",
            ReportTheme::Dark => "dark",
            ReportTheme::Auto => "",
        }
    }
}

/// CDN library configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CdnLibrary {
    /// Pinned version number.
    pub version: String,
    /// Subresource integrity hash (SHA-384).
    pub sri: String,
    /// Path within npm package.
    #[serde(default)]
    pub path: Option<String>,
}

impl CdnLibrary {
    /// Create a new CDN library configuration.
    pub fn new(version: impl Into<String>, sri: impl Into<String>) -> Self {
        Self {
            version: version.into(),
            sri: sri.into(),
            path: None,
        }
    }

    /// Set the path within the npm package.
    pub fn with_path(mut self, path: impl Into<String>) -> Self {
        self.path = Some(path.into());
        self
    }

    /// Get the full CDN URL for this library.
    pub fn url(&self, base_url: &str, package_name: &str) -> String {
        let path = self
            .path
            .as_deref()
            .unwrap_or("dist/index.min.js");
        format!("{}/{}@{}/{}", base_url, package_name, self.version, path)
    }
}

/// Report section visibility configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportSections {
    /// Overview section (session summary).
    #[serde(default = "default_true")]
    pub overview: bool,
    /// Candidates table section.
    #[serde(default = "default_true")]
    pub candidates: bool,
    /// Evidence ledgers section.
    #[serde(default = "default_true")]
    pub evidence: bool,
    /// Actions and outcomes section.
    #[serde(default = "default_true")]
    pub actions: bool,
    /// Telemetry charts section.
    #[serde(default = "default_true")]
    pub telemetry: bool,
    /// Galaxy-brain math section.
    #[serde(default)]
    pub galaxy_brain: bool,
}

fn default_true() -> bool {
    true
}

impl Default for ReportSections {
    fn default() -> Self {
        Self {
            overview: true,
            candidates: true,
            evidence: true,
            actions: true,
            telemetry: true,
            galaxy_brain: false,
        }
    }
}

/// CDN configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CdnConfig {
    /// Base URL for CDN resources.
    #[serde(default = "default_cdn_base")]
    pub base_url: String,
    /// Enable fallback rendering if CDN fails.
    #[serde(default = "default_true")]
    pub fallback_enabled: bool,
    /// Library configurations.
    #[serde(default = "default_libraries")]
    pub libraries: HashMap<String, CdnLibrary>,
}

fn default_cdn_base() -> String {
    "https://cdn.jsdelivr.net/npm".to_string()
}

fn default_libraries() -> HashMap<String, CdnLibrary> {
    let mut libs = HashMap::new();

    // Tailwind CSS for styling
    libs.insert(
        "tailwindcss".to_string(),
        CdnLibrary::new(
            "3.4.1",
            "sha384-KyZXEAg3QhqLMpG8r+8fhAXLRk2vvoC2f3B09zVXn8CA5QIVfZOJ3BCsw2P0p/We",
        )
        .with_path("dist/tailwind.min.css"),
    );

    // Tabulator for interactive tables
    libs.insert(
        "tabulator-tables".to_string(),
        CdnLibrary::new(
            "6.2.1",
            "sha384-oD6f4eeWZpfD1IYpz1PiHqBgMWDTbJFBVB7e4L4eC2wKCJ8X9p8mYmUWJMKk4KkM",
        )
        .with_path("dist/js/tabulator.min.js"),
    );

    // ECharts for charts
    libs.insert(
        "echarts".to_string(),
        CdnLibrary::new(
            "5.5.0",
            "sha384-FGLEKkFq1MZrC7PkPPA6QPDh8S4tFZ0Dy0y+7yE7+Z9E9e3y7R7r5QlR6v1W7zE3",
        )
        .with_path("dist/echarts.min.js"),
    );

    // Mermaid for diagrams
    libs.insert(
        "mermaid".to_string(),
        CdnLibrary::new(
            "10.9.0",
            "sha384-qGKsKMh3J3Y5V3EY7v7E4B3NJ6X4Y8y3QEMYc5qPjH5r5Q3l4v7R3W7m7X4Y8E3",
        )
        .with_path("dist/mermaid.min.js"),
    );

    // KaTeX for math rendering
    libs.insert(
        "katex".to_string(),
        CdnLibrary::new(
            "0.16.9",
            "sha384-dMJ1GzWxYlxj9N4W7h3wEAqZ8F1p4KxRxPxYk3E4q7Y8y3QEMYc5qPjH5r5Q3l4v",
        )
        .with_path("dist/katex.min.js"),
    );

    // JSZip for artifact download
    libs.insert(
        "jszip".to_string(),
        CdnLibrary::new(
            "3.10.1",
            "sha384-H1Gz2r/3zy7HL3p5cqZG0q9K5y4X7E5X9q3Y8y3QEMYc5qPjH5r5Q3l4v7R3W7m7",
        )
        .with_path("dist/jszip.min.js"),
    );

    libs
}

impl Default for CdnConfig {
    fn default() -> Self {
        Self {
            base_url: default_cdn_base(),
            fallback_enabled: true,
            libraries: default_libraries(),
        }
    }
}

/// Resource limits for report generation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportLimits {
    /// Maximum candidates to include in the table.
    #[serde(default = "default_max_candidates")]
    pub max_candidates: usize,
    /// Maximum data points for timeline charts.
    #[serde(default = "default_max_timeline_points")]
    pub max_timeline_points: usize,
    /// Maximum size for embedded assets (MB).
    #[serde(default = "default_embed_size_limit")]
    pub embed_size_limit_mb: u64,
}

fn default_max_candidates() -> usize {
    1000
}

fn default_max_timeline_points() -> usize {
    10000
}

fn default_embed_size_limit() -> u64 {
    10
}

impl Default for ReportLimits {
    fn default() -> Self {
        Self {
            max_candidates: default_max_candidates(),
            max_timeline_points: default_max_timeline_points(),
            embed_size_limit_mb: default_embed_size_limit(),
        }
    }
}

/// Complete report configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportConfig {
    /// Schema version.
    #[serde(default = "default_schema_version")]
    pub schema_version: String,
    /// Custom report title.
    pub title: Option<String>,
    /// Color theme.
    #[serde(default)]
    pub theme: ReportTheme,
    /// Embed CDN assets for offline viewing.
    #[serde(default)]
    pub embed_assets: bool,
    /// Include galaxy-brain math tab.
    #[serde(default)]
    pub galaxy_brain: bool,
    /// Section visibility.
    #[serde(default)]
    pub sections: ReportSections,
    /// CDN configuration.
    #[serde(default)]
    pub cdn_config: CdnConfig,
    /// Resource limits.
    #[serde(default)]
    pub limits: ReportLimits,
    /// Redaction profile for displayed data.
    #[serde(default = "default_redaction_profile")]
    pub redaction_profile: String,
}

fn default_schema_version() -> String {
    "1.0.0".to_string()
}

fn default_redaction_profile() -> String {
    "safe".to_string()
}

impl Default for ReportConfig {
    fn default() -> Self {
        Self {
            schema_version: default_schema_version(),
            title: None,
            theme: ReportTheme::default(),
            embed_assets: false,
            galaxy_brain: false,
            sections: ReportSections::default(),
            cdn_config: CdnConfig::default(),
            limits: ReportLimits::default(),
            redaction_profile: default_redaction_profile(),
        }
    }
}

impl ReportConfig {
    /// Create a new report configuration.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the report title.
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set the theme.
    pub fn with_theme(mut self, theme: ReportTheme) -> Self {
        self.theme = theme;
        self
    }

    /// Enable asset embedding.
    pub fn with_embed_assets(mut self, embed: bool) -> Self {
        self.embed_assets = embed;
        self
    }

    /// Enable galaxy-brain math section.
    pub fn with_galaxy_brain(mut self, enabled: bool) -> Self {
        self.galaxy_brain = enabled;
        self.sections.galaxy_brain = enabled;
        self
    }

    /// Load configuration from JSON.
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    /// Serialize to JSON.
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = ReportConfig::default();
        assert_eq!(config.schema_version, "1.0.0");
        assert_eq!(config.theme, ReportTheme::Auto);
        assert!(!config.embed_assets);
        assert!(config.sections.overview);
        assert!(!config.sections.galaxy_brain);
    }

    #[test]
    fn test_config_builder() {
        let config = ReportConfig::new()
            .with_title("Test Report")
            .with_theme(ReportTheme::Dark)
            .with_galaxy_brain(true);

        assert_eq!(config.title, Some("Test Report".to_string()));
        assert_eq!(config.theme, ReportTheme::Dark);
        assert!(config.galaxy_brain);
        assert!(config.sections.galaxy_brain);
    }

    #[test]
    fn test_cdn_library_url() {
        let lib = CdnLibrary::new("5.5.0", "sha384-test").with_path("dist/echarts.min.js");
        let url = lib.url("https://cdn.jsdelivr.net/npm", "echarts");
        assert_eq!(
            url,
            "https://cdn.jsdelivr.net/npm/echarts@5.5.0/dist/echarts.min.js"
        );
    }

    #[test]
    fn test_config_serialization() {
        let config = ReportConfig::default();
        let json = config.to_json().unwrap();
        let parsed: ReportConfig = ReportConfig::from_json(&json).unwrap();
        assert_eq!(parsed.schema_version, config.schema_version);
    }
}
