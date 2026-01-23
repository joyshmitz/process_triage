//! Report generator implementation.

use crate::config::ReportConfig;
use crate::error::Result;
use crate::sections::*;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::io::{Read, Seek};
use tracing::{debug, info};

/// Complete report data structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportData {
    /// Report configuration.
    pub config: ReportConfig,
    /// Generation timestamp.
    pub generated_at: DateTime<Utc>,
    /// Generator version.
    pub generator_version: String,
    /// Overview section.
    pub overview: Option<OverviewSection>,
    /// Candidates section.
    pub candidates: Option<CandidatesSection>,
    /// Evidence section.
    pub evidence: Option<EvidenceSection>,
    /// Actions section.
    pub actions: Option<ActionsSection>,
    /// Galaxy-brain section.
    pub galaxy_brain: Option<GalaxyBrainSection>,
}

impl ReportData {
    /// Get the report title.
    pub fn title(&self) -> String {
        self.config
            .title
            .clone()
            .or_else(|| {
                self.overview
                    .as_ref()
                    .map(|o| format!("Session {}", o.session_id))
            })
            .unwrap_or_else(|| "Process Triage Report".to_string())
    }
}

/// Report generator.
pub struct ReportGenerator {
    config: ReportConfig,
}

impl ReportGenerator {
    /// Create a new report generator with configuration.
    pub fn new(config: ReportConfig) -> Self {
        Self { config }
    }

    /// Create a generator with default configuration.
    pub fn default_config() -> Self {
        Self::new(ReportConfig::default())
    }

    /// Get the current configuration.
    pub fn config(&self) -> &ReportConfig {
        &self.config
    }

    /// Generate report from a bundle reader.
    pub fn generate_from_bundle<R: Read + Seek>(
        &self,
        reader: &mut pt_bundle::BundleReader<R>,
    ) -> Result<String> {
        debug!("Generating report from bundle");

        // Read manifest for metadata
        let manifest = reader.manifest();

        // Build overview from manifest
        let overview = self.build_overview_from_manifest(manifest);

        // Try to read summary for additional data
        let _summary: Option<serde_json::Value> = reader.read_summary().ok();

        // Build report data
        let data = ReportData {
            config: self.config.clone(),
            generated_at: Utc::now(),
            generator_version: env!("CARGO_PKG_VERSION").to_string(),
            overview: Some(overview),
            candidates: None, // Would be populated from telemetry
            evidence: None,
            actions: None,
            galaxy_brain: if self.config.galaxy_brain {
                Some(GalaxyBrainSection::default())
            } else {
                None
            },
        };

        self.render_html(&data)
    }

    /// Generate report from structured data.
    pub fn generate(&self, data: ReportData) -> Result<String> {
        self.render_html(&data)
    }

    /// Generate report from JSON data.
    pub fn generate_from_json(&self, json: &str) -> Result<String> {
        let data: ReportData = serde_json::from_str(json)?;
        self.render_html(&data)
    }

    fn build_overview_from_manifest(
        &self,
        manifest: &pt_bundle::BundleManifest,
    ) -> OverviewSection {
        OverviewSection {
            session_id: manifest.session_id.clone(),
            host_id: manifest.host_id.clone(),
            hostname: None,
            started_at: manifest.created_at,
            ended_at: None,
            duration_ms: None,
            state: "completed".to_string(),
            mode: "unknown".to_string(),
            deep_scan: false,
            processes_scanned: 0,
            candidates_found: 0,
            kills_attempted: 0,
            kills_successful: 0,
            spares: 0,
            os_family: None,
            os_version: None,
            kernel_version: None,
            arch: None,
            cores: None,
            memory_bytes: None,
            pt_version: manifest.pt_version.clone(),
            export_profile: manifest.export_profile.to_string(),
        }
    }

    fn render_html(&self, data: &ReportData) -> Result<String> {
        let html = self.generate_html(data);

        // Optionally minify
        let output = if cfg!(debug_assertions) {
            html
        } else {
            let cfg = minify_html::Cfg {
                minify_js: true,
                minify_css: true,
                ..Default::default()
            };
            String::from_utf8(minify_html::minify(html.as_bytes(), &cfg)).unwrap_or(html)
        };

        info!(
            bytes = output.len(),
            title = %data.title(),
            "Report generated"
        );

        Ok(output)
    }

    fn generate_html(&self, data: &ReportData) -> String {
        let title = data.title();
        let theme_class = self.config.theme.css_class();
        let cdn_base = &self.config.cdn_config.base_url;
        let libs = &self.config.cdn_config.libraries;

        // Build CDN script/style tags
        let mut cdn_styles = String::new();
        let mut cdn_scripts = String::new();

        if let Some(lib) = libs.get("tailwindcss") {
            cdn_styles.push_str(&format!(
                r#"<script src="{}/tailwindcss@{}/dist/tailwind.min.js"></script>"#,
                cdn_base, lib.version
            ));
        }

        if let Some(lib) = libs.get("tabulator-tables") {
            cdn_styles.push_str(&format!(
                r#"<link rel="stylesheet" href="{}/tabulator-tables@{}/dist/css/tabulator.min.css" integrity="{}" crossorigin="anonymous">"#,
                cdn_base, lib.version, lib.sri
            ));
            cdn_scripts.push_str(&format!(
                r#"<script src="{}/tabulator-tables@{}/dist/js/tabulator.min.js" integrity="{}" crossorigin="anonymous"></script>"#,
                cdn_base, lib.version, lib.sri
            ));
        }

        if let Some(lib) = libs.get("echarts") {
            cdn_scripts.push_str(&format!(
                r#"<script src="{}/echarts@{}/dist/echarts.min.js" integrity="{}" crossorigin="anonymous"></script>"#,
                cdn_base, lib.version, lib.sri
            ));
        }

        if self.config.galaxy_brain {
            if let Some(lib) = libs.get("katex") {
                cdn_styles.push_str(&format!(
                    r#"<link rel="stylesheet" href="{}/katex@{}/dist/katex.min.css" integrity="{}" crossorigin="anonymous">"#,
                    cdn_base, lib.version, lib.sri
                ));
                cdn_scripts.push_str(&format!(
                    r#"<script src="{}/katex@{}/dist/katex.min.js" integrity="{}" crossorigin="anonymous"></script>"#,
                    cdn_base, lib.version, lib.sri
                ));
            }
        }

        // Serialize data for JavaScript
        let data_json = serde_json::to_string(data).unwrap_or_else(|_| "{}".to_string());

        format!(
            r##"<!DOCTYPE html>
<html lang="en" class="{theme_class}">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{title}</title>
    <meta name="generator" content="pt-report {version}">
    <meta name="robots" content="noindex, nofollow">
    {cdn_styles}
    <style>
        /* Base styles */
        :root {{
            --bg-primary: #ffffff;
            --bg-secondary: #f9fafb;
            --text-primary: #111827;
            --text-secondary: #6b7280;
            --border-color: #e5e7eb;
            --accent-color: #3b82f6;
        }}
        .dark {{
            --bg-primary: #111827;
            --bg-secondary: #1f2937;
            --text-primary: #f9fafb;
            --text-secondary: #9ca3af;
            --border-color: #374151;
            --accent-color: #60a5fa;
        }}
        @media (prefers-color-scheme: dark) {{
            :root:not(.light) {{
                --bg-primary: #111827;
                --bg-secondary: #1f2937;
                --text-primary: #f9fafb;
                --text-secondary: #9ca3af;
                --border-color: #374151;
                --accent-color: #60a5fa;
            }}
        }}
        body {{
            background-color: var(--bg-primary);
            color: var(--text-primary);
            font-family: ui-sans-serif, system-ui, sans-serif;
            line-height: 1.5;
        }}
        .card {{
            background-color: var(--bg-secondary);
            border: 1px solid var(--border-color);
            border-radius: 0.5rem;
            padding: 1.5rem;
            margin-bottom: 1rem;
        }}
        .stat-card {{
            text-align: center;
            padding: 1rem;
        }}
        .stat-value {{
            font-size: 2rem;
            font-weight: 700;
            color: var(--accent-color);
        }}
        .stat-label {{
            font-size: 0.875rem;
            color: var(--text-secondary);
        }}
        .tab-btn {{
            padding: 0.75rem 1.5rem;
            border-bottom: 2px solid transparent;
            cursor: pointer;
            transition: all 0.2s;
        }}
        .tab-btn:hover {{
            background-color: var(--bg-secondary);
        }}
        .tab-btn.active {{
            border-bottom-color: var(--accent-color);
            color: var(--accent-color);
        }}
        .tab-content {{
            display: none;
        }}
        .tab-content.active {{
            display: block;
        }}
        .badge {{
            display: inline-flex;
            align-items: center;
            padding: 0.25rem 0.75rem;
            border-radius: 9999px;
            font-size: 0.75rem;
            font-weight: 500;
        }}
        .evidence-bar {{
            height: 0.5rem;
            background-color: var(--border-color);
            border-radius: 0.25rem;
            overflow: hidden;
        }}
        .evidence-bar-fill {{
            height: 100%;
            transition: width 0.3s;
        }}
        .evidence-bar-fill.positive {{
            background-color: #ef4444;
        }}
        .evidence-bar-fill.negative {{
            background-color: #22c55e;
        }}
        /* Print styles */
        @media print {{
            .no-print {{ display: none !important; }}
            body {{ font-size: 10pt; }}
            .card {{ page-break-inside: avoid; }}
        }}
    </style>
</head>
<body>
    <div class="max-w-7xl mx-auto px-4 py-8">
        <!-- Header -->
        <header class="mb-8">
            <h1 class="text-3xl font-bold mb-2">{title}</h1>
            <p class="text-sm" style="color: var(--text-secondary)">
                Generated: {generated_at} | Profile: {profile}
            </p>
        </header>

        <!-- Navigation Tabs -->
        <nav class="flex border-b mb-6 no-print" style="border-color: var(--border-color)">
            {tab_buttons}
        </nav>

        <!-- Tab Contents -->
        <main>
            {tab_contents}
        </main>

        <!-- Footer -->
        <footer class="mt-8 pt-4 border-t text-sm text-center" style="border-color: var(--border-color); color: var(--text-secondary)">
            <p>Process Triage Report v{version}</p>
            <p class="mt-1">
                <a href="https://github.com/Dicklesworthstone/process_triage"
                   target="_blank" rel="noopener"
                   style="color: var(--accent-color)">Documentation</a>
            </p>
        </footer>
    </div>

    {cdn_scripts}
    <script>
        // Report data
        const REPORT_DATA = {data_json};

        // Tab switching
        function switchTab(tabId) {{
            document.querySelectorAll('.tab-btn').forEach(btn => {{
                btn.classList.toggle('active', btn.dataset.tab === tabId);
            }});
            document.querySelectorAll('.tab-content').forEach(content => {{
                content.classList.toggle('active', content.id === 'tab-' + tabId);
            }});
        }}

        // Initialize tabs
        document.querySelectorAll('.tab-btn').forEach(btn => {{
            btn.addEventListener('click', () => switchTab(btn.dataset.tab));
        }});

        // Initialize first tab
        const firstTab = document.querySelector('.tab-btn');
        if (firstTab) switchTab(firstTab.dataset.tab);

        // Initialize Tabulator if available
        if (typeof Tabulator !== 'undefined' && REPORT_DATA.candidates) {{
            new Tabulator('#candidates-table', {{
                data: REPORT_DATA.candidates.candidates,
                layout: 'fitColumns',
                pagination: true,
                paginationSize: 25,
                columns: [
                    {{ title: 'PID', field: 'pid', sorter: 'number', width: 80 }},
                    {{ title: 'Command', field: 'cmd', sorter: 'string' }},
                    {{ title: 'Score', field: 'score', sorter: 'number',
                       formatter: cell => (cell.getValue() * 100).toFixed(1) + '%' }},
                    {{ title: 'Recommendation', field: 'recommendation', sorter: 'string' }},
                    {{ title: 'Age', field: 'age_s', sorter: 'number',
                       formatter: cell => formatAge(cell.getValue()) }},
                    {{ title: 'CPU %', field: 'cpu_pct', sorter: 'number',
                       formatter: cell => cell.getValue().toFixed(1) + '%' }},
                    {{ title: 'Memory', field: 'mem_mb', sorter: 'number',
                       formatter: cell => formatMem(cell.getValue()) }},
                ],
            }});
        }}

        // Initialize ECharts if available
        if (typeof echarts !== 'undefined' && REPORT_DATA.candidates) {{
            const scoreChart = echarts.init(document.getElementById('score-chart'));
            const scores = REPORT_DATA.candidates.candidates.map(c => c.score);
            scoreChart.setOption({{
                title: {{ text: 'Score Distribution', left: 'center' }},
                xAxis: {{ type: 'category', data: ['0-20%', '20-40%', '40-60%', '60-80%', '80-100%'] }},
                yAxis: {{ type: 'value' }},
                series: [{{
                    type: 'bar',
                    data: [
                        scores.filter(s => s < 0.2).length,
                        scores.filter(s => s >= 0.2 && s < 0.4).length,
                        scores.filter(s => s >= 0.4 && s < 0.6).length,
                        scores.filter(s => s >= 0.6 && s < 0.8).length,
                        scores.filter(s => s >= 0.8).length,
                    ],
                    itemStyle: {{ color: '#3b82f6' }}
                }}]
            }});
            window.addEventListener('resize', () => scoreChart.resize());
        }}

        // Initialize KaTeX if available
        if (typeof katex !== 'undefined') {{
            document.querySelectorAll('.math').forEach(el => {{
                katex.render(el.textContent, el, {{ throwOnError: false }});
            }});
        }}

        // Utility functions
        function formatAge(seconds) {{
            if (seconds >= 86400) return Math.floor(seconds / 86400) + 'd';
            if (seconds >= 3600) return Math.floor(seconds / 3600) + 'h';
            if (seconds >= 60) return Math.floor(seconds / 60) + 'm';
            return seconds + 's';
        }}

        function formatMem(mb) {{
            if (mb >= 1024) return (mb / 1024).toFixed(1) + ' GB';
            return mb.toFixed(0) + ' MB';
        }}
    </script>
</body>
</html>"##,
            theme_class = theme_class,
            title = html_escape(&title),
            version = env!("CARGO_PKG_VERSION"),
            cdn_styles = cdn_styles,
            generated_at = data.generated_at.format("%Y-%m-%d %H:%M UTC"),
            profile = html_escape(&self.config.redaction_profile),
            tab_buttons = self.generate_tab_buttons(data),
            tab_contents = self.generate_tab_contents(data),
            cdn_scripts = cdn_scripts,
            data_json = html_escape(&data_json),
        )
    }

    fn generate_tab_buttons(&self, data: &ReportData) -> String {
        let mut buttons = Vec::new();
        let sections = &self.config.sections;

        if sections.overview && data.overview.is_some() {
            buttons.push(r#"<button class="tab-btn" data-tab="overview">Overview</button>"#);
        }
        if sections.candidates && data.candidates.is_some() {
            buttons.push(r#"<button class="tab-btn" data-tab="candidates">Candidates</button>"#);
        }
        if sections.evidence && data.evidence.is_some() {
            buttons.push(r#"<button class="tab-btn" data-tab="evidence">Evidence</button>"#);
        }
        if sections.actions && data.actions.is_some() {
            buttons.push(r#"<button class="tab-btn" data-tab="actions">Actions</button>"#);
        }
        if sections.galaxy_brain && data.galaxy_brain.is_some() {
            buttons
                .push(r#"<button class="tab-btn" data-tab="galaxy-brain">Galaxy Brain</button>"#);
        }

        buttons.join("\n            ")
    }

    fn generate_tab_contents(&self, data: &ReportData) -> String {
        let mut contents = Vec::new();
        let sections = &self.config.sections;

        if sections.overview {
            if let Some(ref overview) = data.overview {
                contents.push(self.generate_overview_tab(overview));
            }
        }
        if sections.candidates {
            if let Some(ref candidates) = data.candidates {
                contents.push(self.generate_candidates_tab(candidates));
            }
        }
        if sections.evidence {
            if let Some(ref evidence) = data.evidence {
                contents.push(self.generate_evidence_tab(evidence));
            }
        }
        if sections.actions {
            if let Some(ref actions) = data.actions {
                contents.push(self.generate_actions_tab(actions));
            }
        }
        if sections.galaxy_brain {
            if let Some(ref gb) = data.galaxy_brain {
                contents.push(self.generate_galaxy_brain_tab(gb));
            }
        }

        contents.join("\n")
    }

    fn generate_overview_tab(&self, overview: &OverviewSection) -> String {
        format!(
            r##"<section id="tab-overview" class="tab-content">
    <div class="grid grid-cols-1 md:grid-cols-4 gap-4 mb-6">
        <div class="card stat-card">
            <div class="stat-value">{processes}</div>
            <div class="stat-label">Processes Scanned</div>
        </div>
        <div class="card stat-card">
            <div class="stat-value">{candidates}</div>
            <div class="stat-label">Candidates Found</div>
        </div>
        <div class="card stat-card">
            <div class="stat-value">{kills}</div>
            <div class="stat-label">Kills Successful</div>
        </div>
        <div class="card stat-card">
            <div class="stat-value">{spares}</div>
            <div class="stat-label">Spared</div>
        </div>
    </div>

    <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
        <div class="card">
            <h3 class="text-lg font-semibold mb-4">Session Details</h3>
            <dl class="grid grid-cols-2 gap-2 text-sm">
                <dt style="color: var(--text-secondary)">Session ID</dt>
                <dd class="font-mono">{session_id}</dd>
                <dt style="color: var(--text-secondary)">Host ID</dt>
                <dd class="font-mono">{host_id}</dd>
                <dt style="color: var(--text-secondary)">Started</dt>
                <dd>{started_at}</dd>
                <dt style="color: var(--text-secondary)">Duration</dt>
                <dd>{duration}</dd>
                <dt style="color: var(--text-secondary)">Mode</dt>
                <dd>{mode}</dd>
                <dt style="color: var(--text-secondary)">State</dt>
                <dd><span class="badge bg-green-100 text-green-800">{state}</span></dd>
            </dl>
        </div>

        <div class="card">
            <h3 class="text-lg font-semibold mb-4">System Information</h3>
            <dl class="grid grid-cols-2 gap-2 text-sm">
                <dt style="color: var(--text-secondary)">OS</dt>
                <dd>{os}</dd>
                <dt style="color: var(--text-secondary)">Architecture</dt>
                <dd>{arch}</dd>
                <dt style="color: var(--text-secondary)">Cores</dt>
                <dd>{cores}</dd>
                <dt style="color: var(--text-secondary)">Memory</dt>
                <dd>{memory}</dd>
                <dt style="color: var(--text-secondary)">PT Version</dt>
                <dd>{pt_version}</dd>
                <dt style="color: var(--text-secondary)">Export Profile</dt>
                <dd><span class="badge bg-blue-100 text-blue-800">{profile}</span></dd>
            </dl>
        </div>
    </div>
</section>"##,
            processes = overview.processes_scanned,
            candidates = overview.candidates_found,
            kills = overview.kills_successful,
            spares = overview.spares,
            session_id = html_escape(&overview.session_id),
            host_id = html_escape(&overview.host_id),
            started_at = overview.started_at.format("%Y-%m-%d %H:%M:%S UTC"),
            duration = overview.duration_formatted(),
            mode = html_escape(&overview.mode),
            state = html_escape(&overview.state),
            os = html_escape(overview.os_family.as_deref().unwrap_or("Unknown")),
            arch = html_escape(overview.arch.as_deref().unwrap_or("Unknown")),
            cores = overview
                .cores
                .map(|c| c.to_string())
                .unwrap_or_else(|| "N/A".to_string()),
            memory = overview.memory_formatted(),
            pt_version = html_escape(overview.pt_version.as_deref().unwrap_or("Unknown")),
            profile = html_escape(&overview.export_profile),
        )
    }

    fn generate_candidates_tab(&self, candidates: &CandidatesSection) -> String {
        format!(
            r##"<section id="tab-candidates" class="tab-content">
    <div class="grid grid-cols-1 md:grid-cols-3 gap-4 mb-6">
        <div class="card stat-card">
            <div class="stat-value text-red-500">{kill_count}</div>
            <div class="stat-label">Kill Recommendations</div>
        </div>
        <div class="card stat-card">
            <div class="stat-value text-green-500">{spare_count}</div>
            <div class="stat-label">Spare Recommendations</div>
        </div>
        <div class="card stat-card">
            <div class="stat-value text-yellow-500">{review_count}</div>
            <div class="stat-label">Review Needed</div>
        </div>
    </div>

    <div class="card">
        <div class="flex justify-between items-center mb-4">
            <h3 class="text-lg font-semibold">Candidate Processes</h3>
            {truncation_notice}
        </div>
        <div id="candidates-table"></div>
    </div>

    <div class="card mt-4">
        <h3 class="text-lg font-semibold mb-4">Score Distribution</h3>
        <div id="score-chart" style="height: 300px;"></div>
    </div>
</section>"##,
            kill_count = candidates.kill_count(),
            spare_count = candidates.spare_count(),
            review_count = candidates.review_count(),
            truncation_notice = if candidates.truncated {
                format!(
                    r#"<span class="text-sm" style="color: var(--text-secondary)">Showing {} of {} candidates</span>"#,
                    candidates.candidates.len(),
                    candidates.total_count
                )
            } else {
                String::new()
            },
        )
    }

    fn generate_evidence_tab(&self, evidence: &EvidenceSection) -> String {
        let mut ledger_html = String::new();
        for ledger in &evidence.ledgers {
            ledger_html.push_str(&self.generate_evidence_ledger(ledger));
        }

        format!(
            r##"<section id="tab-evidence" class="tab-content">
    <div class="card mb-4">
        <h3 class="text-lg font-semibold mb-4">Evidence Factor Legend</h3>
        <div class="grid grid-cols-2 md:grid-cols-5 gap-2 text-sm">
            {factor_legend}
        </div>
    </div>

    <div class="space-y-4">
        {ledger_html}
    </div>
</section>"##,
            factor_legend = evidence
                .factor_definitions
                .iter()
                .map(|f| format!(
                    r#"<div class="p-2 rounded" style="background: var(--bg-secondary)">
                        <div class="font-medium">{}</div>
                        <div style="color: var(--text-secondary)">{}</div>
                    </div>"#,
                    html_escape(&f.name),
                    html_escape(&f.description)
                ))
                .collect::<Vec<_>>()
                .join("\n            "),
            ledger_html = ledger_html,
        )
    }

    fn generate_evidence_ledger(&self, ledger: &EvidenceLedger) -> String {
        let factors_html: String = ledger
            .factors
            .iter()
            .map(|f| {
                let bar_class = if f.favors_abandoned {
                    "positive"
                } else {
                    "negative"
                };
                format!(
                    r#"<div class="flex items-center gap-2 py-1">
                        <span class="w-20 text-sm">{}</span>
                        <div class="flex-1 evidence-bar">
                            <div class="evidence-bar-fill {}" style="width: {}%"></div>
                        </div>
                        <span class="w-16 text-right text-sm {}">{:+.2}</span>
                    </div>"#,
                    html_escape(&f.label),
                    bar_class,
                    f.bar_width(),
                    f.direction_class(),
                    f.log_odds
                )
            })
            .collect();

        format!(
            r##"<details class="card">
    <summary class="cursor-pointer flex justify-between items-center">
        <div>
            <span class="font-mono font-medium">PID {pid}</span>
            <span class="ml-2" style="color: var(--text-secondary)">{cmd}</span>
        </div>
        <div class="flex items-center gap-2">
            <span class="badge bg-blue-100 text-blue-800">{bf_interp}</span>
            <span class="font-medium">{posterior:.1}%</span>
        </div>
    </summary>
    <div class="mt-4 pt-4 border-t" style="border-color: var(--border-color)">
        <div class="grid grid-cols-3 gap-4 mb-4 text-sm">
            <div>
                <span style="color: var(--text-secondary)">Prior:</span>
                <span class="ml-1">{prior:.1}%</span>
            </div>
            <div>
                <span style="color: var(--text-secondary)">Log BF:</span>
                <span class="ml-1">{log_bf:+.2}</span>
            </div>
            <div>
                <span style="color: var(--text-secondary)">Tags:</span>
                <span class="ml-1">{tags}</span>
            </div>
        </div>
        <h4 class="text-sm font-semibold mb-2">Evidence Factors</h4>
        {factors_html}
    </div>
</details>"##,
            pid = ledger.pid,
            cmd = html_escape(&ledger.cmd),
            bf_interp = html_escape(&ledger.bf_interpretation),
            posterior = ledger.posterior_p * 100.0,
            prior = ledger.prior_p * 100.0,
            log_bf = ledger.log_bf,
            tags = ledger.tags.join(", "),
            factors_html = factors_html,
        )
    }

    fn generate_actions_tab(&self, actions: &ActionsSection) -> String {
        let rows_html: String = actions
            .actions
            .iter()
            .map(|a| {
                format!(
                    r#"<tr>
                        <td class="px-4 py-2 text-sm">{}</td>
                        <td class="px-4 py-2 font-mono">{}</td>
                        <td class="px-4 py-2">{}</td>
                        <td class="px-4 py-2"><span class="badge {}">{}</span></td>
                        <td class="px-4 py-2"><span class="badge {}">{}</span></td>
                        <td class="px-4 py-2">{}</td>
                        <td class="px-4 py-2">{}</td>
                    </tr>"#,
                    a.timestamp.format("%H:%M:%S"),
                    a.pid,
                    html_escape(&a.cmd),
                    a.recommendation_class(),
                    html_escape(&a.recommendation),
                    a.status_class(),
                    a.status_text(),
                    a.memory_freed_formatted().unwrap_or_default(),
                    a.user_feedback.as_deref().unwrap_or("-"),
                )
            })
            .collect();

        format!(
            r##"<section id="tab-actions" class="tab-content">
    <div class="grid grid-cols-2 md:grid-cols-4 gap-4 mb-6">
        <div class="card stat-card">
            <div class="stat-value">{total}</div>
            <div class="stat-label">Total Actions</div>
        </div>
        <div class="card stat-card">
            <div class="stat-value text-green-500">{successful}</div>
            <div class="stat-label">Successful</div>
        </div>
        <div class="card stat-card">
            <div class="stat-value text-red-500">{failed}</div>
            <div class="stat-label">Failed</div>
        </div>
        <div class="card stat-card">
            <div class="stat-value">{memory_freed}</div>
            <div class="stat-label">Memory Freed</div>
        </div>
    </div>

    <div class="card overflow-x-auto">
        <table class="w-full text-sm">
            <thead>
                <tr style="border-bottom: 1px solid var(--border-color)">
                    <th class="px-4 py-2 text-left">Time</th>
                    <th class="px-4 py-2 text-left">PID</th>
                    <th class="px-4 py-2 text-left">Command</th>
                    <th class="px-4 py-2 text-left">Recommendation</th>
                    <th class="px-4 py-2 text-left">Status</th>
                    <th class="px-4 py-2 text-left">Memory Freed</th>
                    <th class="px-4 py-2 text-left">Feedback</th>
                </tr>
            </thead>
            <tbody>
                {rows_html}
            </tbody>
        </table>
    </div>
</section>"##,
            total = actions.summary.total,
            successful = actions.summary.successful,
            failed = actions.summary.failed,
            memory_freed = actions.summary.memory_freed_formatted(),
            rows_html = rows_html,
        )
    }

    fn generate_galaxy_brain_tab(&self, gb: &GalaxyBrainSection) -> String {
        let factors_html: String = gb
            .factors
            .iter()
            .map(|f| {
                let examples_html: String = f
                    .examples
                    .iter()
                    .map(|ex| {
                        format!(
                            r#"<tr>
                                <td class="px-2 py-1">{}</td>
                                <td class="px-2 py-1 text-right">{:+.2}</td>
                                <td class="px-2 py-1">{}</td>
                            </tr>"#,
                            html_escape(&ex.input),
                            ex.log_odds,
                            html_escape(&ex.interpretation)
                        )
                    })
                    .collect();

                format!(
                    r##"<div class="card">
                        <h4 class="font-semibold mb-2">{name} <span class="text-sm font-normal" style="color: var(--text-secondary)">({category})</span></h4>
                        <div class="math mb-2">{formula}</div>
                        <p class="text-sm mb-3" style="color: var(--text-secondary)">{intuition}</p>
                        <table class="w-full text-sm">
                            <thead>
                                <tr style="border-bottom: 1px solid var(--border-color)">
                                    <th class="px-2 py-1 text-left">Input</th>
                                    <th class="px-2 py-1 text-right">Log-Odds</th>
                                    <th class="px-2 py-1 text-left">Interpretation</th>
                                </tr>
                            </thead>
                            <tbody>{examples_html}</tbody>
                        </table>
                    </div>"##,
                    name = html_escape(&f.name),
                    category = html_escape(&f.category),
                    formula = html_escape(&f.formula),
                    intuition = html_escape(&f.intuition),
                    examples_html = examples_html,
                )
            })
            .collect();

        let thresholds_html: String = gb
            .bf_guide
            .thresholds
            .iter()
            .map(|t| {
                format!(
                    r#"<tr>
                        <td class="px-2 py-1">{}</td>
                        <td class="px-2 py-1">{}</td>
                    </tr>"#,
                    html_escape(&t.label),
                    html_escape(&t.description)
                )
            })
            .collect();

        format!(
            r##"<section id="tab-galaxy-brain" class="tab-content">
    <div class="card mb-6">
        <h3 class="text-xl font-bold mb-4">Bayesian Process Classification</h3>
        <p class="mb-4">
            Process Triage uses Bayesian inference to estimate the probability that a process
            has been abandoned. Each piece of evidence (age, CPU usage, memory, etc.) contributes
            a likelihood ratio that updates the prior probability.
        </p>

        <h4 class="font-semibold mb-2">Prior Probabilities</h4>
        <div class="math mb-2">{prior_formula}</div>
        <p class="text-sm mb-4" style="color: var(--text-secondary)">{prior_explanation}</p>

        <h4 class="font-semibold mb-2">Bayes Factor</h4>
        <div class="math mb-2">{bf_formula}</div>
        <p class="text-sm mb-4" style="color: var(--text-secondary)">{log_odds_explanation}</p>

        <h4 class="font-semibold mb-2">Interpretation Scale</h4>
        <table class="w-full text-sm mb-4">
            <thead>
                <tr style="border-bottom: 1px solid var(--border-color)">
                    <th class="px-2 py-1 text-left">Strength</th>
                    <th class="px-2 py-1 text-left">Meaning</th>
                </tr>
            </thead>
            <tbody>{thresholds_html}</tbody>
        </table>
    </div>

    <h3 class="text-lg font-semibold mb-4">Evidence Factors</h3>
    <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
        {factors_html}
    </div>
</section>"##,
            prior_formula = html_escape(&gb.priors.formula),
            prior_explanation = html_escape(&gb.priors.explanation),
            bf_formula = html_escape(&gb.bf_guide.formula),
            log_odds_explanation = html_escape(&gb.bf_guide.log_odds_explanation),
            thresholds_html = thresholds_html,
            factors_html = factors_html,
        )
    }
}

impl ActionRow {
    fn recommendation_class(&self) -> &'static str {
        match self.recommendation.as_str() {
            "kill" => "bg-red-100 text-red-800",
            "spare" => "bg-green-100 text-green-800",
            _ => "bg-yellow-100 text-yellow-800",
        }
    }
}

/// Escape HTML special characters.
fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_report_generator_default() {
        let generator = ReportGenerator::default_config();
        assert!(!generator.config.embed_assets);
    }

    #[test]
    fn test_html_escape() {
        assert_eq!(html_escape("<script>"), "&lt;script&gt;");
        assert_eq!(html_escape("a & b"), "a &amp; b");
        assert_eq!(html_escape(r#""quoted""#), "&quot;quoted&quot;");
    }

    #[test]
    fn test_empty_report() {
        let config = ReportConfig::default();
        let generator = ReportGenerator::new(config);
        let data = ReportData {
            config: ReportConfig::default(),
            generated_at: Utc::now(),
            generator_version: "test".to_string(),
            overview: None,
            candidates: None,
            evidence: None,
            actions: None,
            galaxy_brain: None,
        };
        let html = generator.generate(data).unwrap();
        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains("Process Triage Report"));
    }

    #[test]
    fn test_report_with_overview() {
        let generator = ReportGenerator::default_config();
        let data = ReportData {
            config: ReportConfig::default(),
            generated_at: Utc::now(),
            generator_version: "test".to_string(),
            overview: Some(OverviewSection {
                session_id: "test-123".to_string(),
                host_id: "host-abc".to_string(),
                hostname: Some("testhost".to_string()),
                started_at: Utc::now(),
                ended_at: None,
                duration_ms: Some(60000),
                state: "completed".to_string(),
                mode: "interactive".to_string(),
                deep_scan: false,
                processes_scanned: 100,
                candidates_found: 10,
                kills_attempted: 5,
                kills_successful: 4,
                spares: 5,
                os_family: Some("linux".to_string()),
                os_version: None,
                kernel_version: None,
                arch: Some("x86_64".to_string()),
                cores: Some(8),
                memory_bytes: Some(16_000_000_000),
                pt_version: Some("0.1.0".to_string()),
                export_profile: "safe".to_string(),
            }),
            candidates: None,
            evidence: None,
            actions: None,
            galaxy_brain: None,
        };
        let html = generator.generate(data).unwrap();
        assert!(html.contains("test-123"));
        assert!(html.contains("100")); // processes scanned
    }

    #[test]
    fn test_galaxy_brain_section() {
        let config = ReportConfig::default().with_galaxy_brain(true);
        let generator = ReportGenerator::new(config);
        let data = ReportData {
            config: ReportConfig::default().with_galaxy_brain(true),
            generated_at: Utc::now(),
            generator_version: "test".to_string(),
            overview: None,
            candidates: None,
            evidence: None,
            actions: None,
            galaxy_brain: Some(GalaxyBrainSection::default()),
        };
        let html = generator.generate(data).unwrap();
        assert!(html.contains("Galaxy Brain"));
        assert!(html.contains("Bayesian"));
    }
}
