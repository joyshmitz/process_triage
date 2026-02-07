//! Detail pane widget for a selected process.
//!
//! Renders a compact drill-down summary in the right-hand pane.

use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::tui::theme::Theme;
use crate::tui::widgets::ProcessRow;

/// Detail pane modes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DetailView {
    Summary,
    GalaxyBrain,
    Genealogy,
}

/// Detail pane widget for a selected process.
pub struct ProcessDetail<'a> {
    block: Option<Block<'a>>,
    theme: Option<&'a Theme>,
    row: Option<&'a ProcessRow>,
    selected: bool,
    view: DetailView,
}

impl<'a> Default for ProcessDetail<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> ProcessDetail<'a> {
    /// Create a new detail pane widget.
    pub fn new() -> Self {
        Self {
            block: None,
            theme: None,
            row: None,
            selected: false,
            view: DetailView::Summary,
        }
    }

    /// Set the block wrapper.
    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }

    /// Set the theme.
    pub fn theme(mut self, theme: &'a Theme) -> Self {
        self.theme = Some(theme);
        self
    }

    /// Set the selected row and selection state.
    pub fn row(mut self, row: Option<&'a ProcessRow>, selected: bool) -> Self {
        self.row = row;
        self.selected = selected;
        self
    }

    /// Set the detail view mode.
    pub fn view(mut self, view: DetailView) -> Self {
        self.view = view;
        self
    }

    fn classification_style(&self, classification: &str) -> Style {
        if let Some(theme) = self.theme {
            match classification.to_uppercase().as_str() {
                "KILL" => theme.style_kill(),
                "REVIEW" => theme.style_review(),
                "SPARE" => theme.style_spare(),
                _ => theme.style_normal(),
            }
        } else {
            match classification.to_uppercase().as_str() {
                "KILL" => Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                "REVIEW" => Style::default().fg(Color::Yellow),
                "SPARE" => Style::default().fg(Color::Green),
                _ => Style::default(),
            }
        }
    }

    fn label_style(&self) -> Style {
        if let Some(theme) = self.theme {
            theme.style_muted()
        } else {
            Style::default().fg(Color::DarkGray)
        }
    }

    fn value_style(&self) -> Style {
        if let Some(theme) = self.theme {
            theme.style_normal()
        } else {
            Style::default()
        }
    }
}

impl<'a> Widget for ProcessDetail<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = self.block.clone().unwrap_or_else(|| {
            let border_style = if let Some(theme) = self.theme {
                theme.style_border()
            } else {
                Style::default().fg(Color::DarkGray)
            };
            Block::default()
                .borders(Borders::ALL)
                .title(" Detail ")
                .border_style(border_style)
        });

        let inner = block.inner(area);
        block.render(area, buf);

        let Some(row) = self.row else {
            let message = Paragraph::new("No process selected")
                .style(self.value_style())
                .alignment(Alignment::Center);
            message.render(inner, buf);
            return;
        };

        let sections = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(4), // Header
                Constraint::Length(4), // Stats
                Constraint::Min(4),    // Evidence placeholder
                Constraint::Length(3), // Action placeholder
            ])
            .split(inner);

        let selected_label = if self.selected { "yes" } else { "no" };

        let header = vec![
            Line::from(vec![
                Span::styled("PID: ", self.label_style()),
                Span::styled(row.pid.to_string(), self.value_style()),
                Span::styled("  ", self.value_style()),
                Span::styled("Class: ", self.label_style()),
                Span::styled(
                    row.classification.clone(),
                    self.classification_style(&row.classification),
                ),
            ]),
            Line::from(vec![
                Span::styled("Command: ", self.label_style()),
                Span::styled(row.command.clone(), self.value_style()),
            ]),
            Line::from(vec![
                Span::styled("Selected: ", self.label_style()),
                Span::styled(selected_label, self.value_style()),
            ]),
        ];

        let stats = vec![
            Line::from(vec![
                Span::styled("Score: ", self.label_style()),
                Span::styled(row.score.to_string(), self.value_style()),
                Span::styled("  ", self.value_style()),
                Span::styled("Runtime: ", self.label_style()),
                Span::styled(row.runtime.clone(), self.value_style()),
            ]),
            Line::from(vec![
                Span::styled("Memory: ", self.label_style()),
                Span::styled(row.memory.clone(), self.value_style()),
            ]),
        ];

        let evidence_height = sections[2].height.max(1) as usize;
        let (evidence, action) = match self.view {
            DetailView::Summary => (
                {
                    let mut lines = Vec::new();
                    lines.push(Line::from(vec![Span::styled(
                        "Evidence",
                        self.label_style(),
                    )]));
                    if let Some(summary) = row.why_summary.as_ref() {
                        lines.push(Line::from(vec![Span::styled(
                            summary.clone(),
                            self.value_style(),
                        )]));
                    } else {
                        lines.push(Line::from(vec![Span::styled(
                            "No evidence summary available",
                            self.value_style(),
                        )]));
                    }
                    for item in &row.top_evidence {
                        lines.push(Line::from(vec![Span::styled(
                            format!("• {}", item),
                            self.value_style(),
                        )]));
                    }
                    if lines.len() > evidence_height {
                        lines.truncate(evidence_height);
                    }
                    lines
                },
                {
                    let mut lines = Vec::new();
                    lines.push(Line::from(vec![Span::styled("Action", self.label_style())]));
                    if !row.plan_preview.is_empty() {
                        let first = row.plan_preview.get(0).cloned().unwrap_or_default();
                        lines.push(Line::from(vec![
                            Span::styled("Plan: ", self.label_style()),
                            Span::styled(first, self.value_style()),
                        ]));
                        if let Some(second) = row.plan_preview.get(1) {
                            let mut line = second.clone();
                            if row.plan_preview.len() > 2 {
                                line.push_str(" …");
                            }
                            lines.push(Line::from(vec![Span::styled(line, self.value_style())]));
                        }
                    } else {
                        lines.push(Line::from(vec![
                            Span::styled("Recommended: ", self.label_style()),
                            Span::styled(row.classification.clone(), self.value_style()),
                        ]));
                        if let Some(confidence) = row.confidence.as_ref() {
                            lines.push(Line::from(vec![
                                Span::styled("Confidence: ", self.label_style()),
                                Span::styled(confidence.clone(), self.value_style()),
                            ]));
                        }
                    }
                    lines
                },
            ),
            DetailView::GalaxyBrain => (
                {
                    let mut lines = Vec::new();
                    lines.push(Line::from(vec![Span::styled(
                        "Galaxy Brain",
                        self.label_style(),
                    )]));

                    if let Some(trace) = row.galaxy_brain.as_deref() {
                        let trace_lines: Vec<&str> = trace.lines().collect();
                        let max_lines = evidence_height.saturating_sub(1).max(1);
                        for line in trace_lines.iter().take(max_lines) {
                            lines.push(Line::from(vec![Span::styled(*line, self.value_style())]));
                        }
                        if trace_lines.len() > max_lines {
                            lines.push(Line::from(vec![Span::styled(
                                format!("… {} more lines", trace_lines.len() - max_lines),
                                self.label_style(),
                            )]));
                        }
                    } else {
                        lines.push(Line::from(vec![Span::styled(
                            "• math ledger pending",
                            self.value_style(),
                        )]));
                        lines.push(Line::from(vec![Span::styled(
                            "• posterior odds pending",
                            self.value_style(),
                        )]));
                    }

                    lines
                },
                vec![
                    Line::from(vec![Span::styled("Notes", self.label_style())]),
                    Line::from(vec![Span::styled(
                        "• press g to toggle",
                        self.value_style(),
                    )]),
                ],
            ),
            DetailView::Genealogy => (
                vec![
                    Line::from(vec![Span::styled("Genealogy", self.label_style())]),
                    Line::from(vec![Span::styled(
                        "• process tree pending",
                        self.value_style(),
                    )]),
                    Line::from(vec![Span::styled(
                        "• supervisor chain pending",
                        self.value_style(),
                    )]),
                ],
                vec![
                    Line::from(vec![Span::styled("Notes", self.label_style())]),
                    Line::from(vec![Span::styled(
                        "• press s to return",
                        self.value_style(),
                    )]),
                ],
            ),
        };

        Paragraph::new(header)
            .style(self.value_style())
            .render(sections[0], buf);
        Paragraph::new(stats)
            .style(self.value_style())
            .render(sections[1], buf);
        Paragraph::new(evidence)
            .style(self.value_style())
            .render(sections[2], buf);
        Paragraph::new(action)
            .style(self.value_style())
            .render(sections[3], buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::buffer::Buffer;
    use ratatui::layout::Rect;

    fn line_string(buf: &Buffer, area: Rect, y: u16) -> String {
        let mut line = String::new();
        for x in area.x..area.x.saturating_add(area.width) {
            line.push_str(buf[(x, y)].symbol());
        }
        line
    }

    fn buffer_contains(buf: &Buffer, area: Rect, needle: &str) -> bool {
        for y in area.y..area.y.saturating_add(area.height) {
            if line_string(buf, area, y).contains(needle) {
                return true;
            }
        }
        false
    }

    fn sample_row() -> ProcessRow {
        ProcessRow {
            pid: 4242,
            score: 91,
            classification: "KILL".to_string(),
            runtime: "3h 12m".to_string(),
            memory: "1.2 GB".to_string(),
            command: "node dev server".to_string(),
            selected: false,
            galaxy_brain: None,
            why_summary: Some("Classified as abandoned with high confidence.".to_string()),
            top_evidence: vec![
                "runtime (2.8 bits toward abandoned)".to_string(),
                "cpu_idle (1.6 bits toward abandoned)".to_string(),
            ],
            confidence: Some("high".to_string()),
            plan_preview: Vec::new(),
        }
    }

    #[test]
    fn test_detail_renders_empty_state() {
        let area = Rect::new(0, 0, 40, 12);
        let mut buf = Buffer::empty(area);
        ProcessDetail::new().render(area, &mut buf);

        assert!(buffer_contains(&buf, area, "No process selected"));
    }

    #[test]
    fn test_detail_renders_row_fields() {
        let area = Rect::new(0, 0, 60, 16);
        let mut buf = Buffer::empty(area);
        let row = sample_row();

        ProcessDetail::new()
            .row(Some(&row), false)
            .render(area, &mut buf);

        assert!(buffer_contains(&buf, area, "PID:"));
        assert!(buffer_contains(&buf, area, "Command:"));
        assert!(buffer_contains(&buf, area, "node dev server"));
    }

    // ── DetailView enum ─────────────────────────────────────────────

    #[test]
    fn detail_view_eq() {
        assert_eq!(DetailView::Summary, DetailView::Summary);
        assert_eq!(DetailView::GalaxyBrain, DetailView::GalaxyBrain);
        assert_eq!(DetailView::Genealogy, DetailView::Genealogy);
        assert_ne!(DetailView::Summary, DetailView::GalaxyBrain);
    }

    // ── ProcessDetail builder ───────────────────────────────────────

    #[test]
    fn detail_default() {
        let d = ProcessDetail::default();
        assert!(d.block.is_none());
        assert!(d.theme.is_none());
        assert!(d.row.is_none());
        assert!(!d.selected);
        assert_eq!(d.view, DetailView::Summary);
    }

    #[test]
    fn detail_view_builder() {
        let d = ProcessDetail::new().view(DetailView::GalaxyBrain);
        assert_eq!(d.view, DetailView::GalaxyBrain);
    }

    #[test]
    fn detail_row_builder_sets_selected() {
        let row = sample_row();
        let d = ProcessDetail::new().row(Some(&row), true);
        assert!(d.selected);
        assert!(d.row.is_some());
    }

    // ── Render: selected indicator ──────────────────────────────────

    #[test]
    fn detail_renders_selected_yes() {
        let area = Rect::new(0, 0, 60, 16);
        let mut buf = Buffer::empty(area);
        let row = sample_row();
        ProcessDetail::new()
            .row(Some(&row), true)
            .render(area, &mut buf);
        assert!(buffer_contains(&buf, area, "Selected:"));
        assert!(buffer_contains(&buf, area, "yes"));
    }

    #[test]
    fn detail_renders_selected_no() {
        let area = Rect::new(0, 0, 60, 16);
        let mut buf = Buffer::empty(area);
        let row = sample_row();
        ProcessDetail::new()
            .row(Some(&row), false)
            .render(area, &mut buf);
        // "no" appears after "Selected:"
        assert!(buffer_contains(&buf, area, "no"));
    }

    // ── Render: stats section ───────────────────────────────────────

    #[test]
    fn detail_renders_score() {
        let area = Rect::new(0, 0, 60, 16);
        let mut buf = Buffer::empty(area);
        let row = sample_row();
        ProcessDetail::new()
            .row(Some(&row), false)
            .render(area, &mut buf);
        assert!(buffer_contains(&buf, area, "Score:"));
        assert!(buffer_contains(&buf, area, "91"));
    }

    #[test]
    fn detail_renders_runtime() {
        let area = Rect::new(0, 0, 60, 16);
        let mut buf = Buffer::empty(area);
        let row = sample_row();
        ProcessDetail::new()
            .row(Some(&row), false)
            .render(area, &mut buf);
        assert!(buffer_contains(&buf, area, "Runtime:"));
        assert!(buffer_contains(&buf, area, "3h 12m"));
    }

    #[test]
    fn detail_renders_memory() {
        let area = Rect::new(0, 0, 60, 16);
        let mut buf = Buffer::empty(area);
        let row = sample_row();
        ProcessDetail::new()
            .row(Some(&row), false)
            .render(area, &mut buf);
        assert!(buffer_contains(&buf, area, "Memory:"));
        assert!(buffer_contains(&buf, area, "1.2 GB"));
    }

    // ── Render: Summary view evidence ───────────────────────────────

    #[test]
    fn detail_renders_evidence_section() {
        let area = Rect::new(0, 0, 60, 16);
        let mut buf = Buffer::empty(area);
        let row = sample_row();
        ProcessDetail::new()
            .row(Some(&row), false)
            .view(DetailView::Summary)
            .render(area, &mut buf);
        assert!(buffer_contains(&buf, area, "Evidence"));
    }

    #[test]
    fn detail_renders_why_summary() {
        let area = Rect::new(0, 0, 80, 20);
        let mut buf = Buffer::empty(area);
        let row = sample_row();
        ProcessDetail::new()
            .row(Some(&row), false)
            .render(area, &mut buf);
        assert!(buffer_contains(&buf, area, "abandoned"));
    }

    #[test]
    fn detail_renders_no_evidence_when_summary_none() {
        let area = Rect::new(0, 0, 80, 20);
        let mut buf = Buffer::empty(area);
        let mut row = sample_row();
        row.why_summary = None;
        row.top_evidence = vec![];
        ProcessDetail::new()
            .row(Some(&row), false)
            .render(area, &mut buf);
        assert!(buffer_contains(&buf, area, "No evidence summary"));
    }

    // ── Render: Summary view action section ─────────────────────────

    #[test]
    fn detail_renders_recommended_action() {
        let area = Rect::new(0, 0, 60, 16);
        let mut buf = Buffer::empty(area);
        let row = sample_row();
        ProcessDetail::new()
            .row(Some(&row), false)
            .render(area, &mut buf);
        assert!(buffer_contains(&buf, area, "Action"));
        assert!(buffer_contains(&buf, area, "Recommended:"));
    }

    #[test]
    fn detail_renders_plan_preview_when_present() {
        let area = Rect::new(0, 0, 60, 16);
        let mut buf = Buffer::empty(area);
        let mut row = sample_row();
        row.plan_preview = vec!["kill -9 4242".to_string(), "verify PID gone".to_string()];
        ProcessDetail::new()
            .row(Some(&row), false)
            .render(area, &mut buf);
        assert!(buffer_contains(&buf, area, "Plan:"));
        assert!(buffer_contains(&buf, area, "kill -9 4242"));
    }

    #[test]
    fn detail_renders_confidence() {
        let area = Rect::new(0, 0, 60, 16);
        let mut buf = Buffer::empty(area);
        let row = sample_row();
        ProcessDetail::new()
            .row(Some(&row), false)
            .render(area, &mut buf);
        assert!(buffer_contains(&buf, area, "Confidence:"));
        assert!(buffer_contains(&buf, area, "high"));
    }

    // ── Render: GalaxyBrain view ────────────────────────────────────

    #[test]
    fn detail_galaxy_brain_pending() {
        let area = Rect::new(0, 0, 60, 16);
        let mut buf = Buffer::empty(area);
        let row = sample_row(); // galaxy_brain is None
        ProcessDetail::new()
            .row(Some(&row), false)
            .view(DetailView::GalaxyBrain)
            .render(area, &mut buf);
        assert!(buffer_contains(&buf, area, "Galaxy Brain"));
        assert!(buffer_contains(&buf, area, "math ledger pending"));
    }

    #[test]
    fn detail_galaxy_brain_with_trace() {
        let area = Rect::new(0, 0, 80, 20);
        let mut buf = Buffer::empty(area);
        let mut row = sample_row();
        row.galaxy_brain = Some("P(abandoned|evidence) = 0.85\nBF = 5.67".to_string());
        ProcessDetail::new()
            .row(Some(&row), false)
            .view(DetailView::GalaxyBrain)
            .render(area, &mut buf);
        assert!(buffer_contains(&buf, area, "Galaxy Brain"));
        assert!(buffer_contains(&buf, area, "P(abandoned|evidence)"));
    }

    #[test]
    fn detail_galaxy_brain_notes() {
        let area = Rect::new(0, 0, 60, 16);
        let mut buf = Buffer::empty(area);
        let row = sample_row();
        ProcessDetail::new()
            .row(Some(&row), false)
            .view(DetailView::GalaxyBrain)
            .render(area, &mut buf);
        assert!(buffer_contains(&buf, area, "Notes"));
        assert!(buffer_contains(&buf, area, "press g to toggle"));
    }

    // ── Render: Genealogy view ──────────────────────────────────────

    #[test]
    fn detail_genealogy_view() {
        let area = Rect::new(0, 0, 60, 16);
        let mut buf = Buffer::empty(area);
        let row = sample_row();
        ProcessDetail::new()
            .row(Some(&row), false)
            .view(DetailView::Genealogy)
            .render(area, &mut buf);
        assert!(buffer_contains(&buf, area, "Genealogy"));
        assert!(buffer_contains(&buf, area, "process tree pending"));
    }

    #[test]
    fn detail_genealogy_notes() {
        let area = Rect::new(0, 0, 60, 16);
        let mut buf = Buffer::empty(area);
        let row = sample_row();
        ProcessDetail::new()
            .row(Some(&row), false)
            .view(DetailView::Genealogy)
            .render(area, &mut buf);
        assert!(buffer_contains(&buf, area, "press s to return"));
    }

    // ── classification_style (without theme) ────────────────────────

    #[test]
    fn classification_style_kill_is_red() {
        let d = ProcessDetail::new();
        let style = d.classification_style("KILL");
        assert_eq!(style.fg, Some(Color::Red));
    }

    #[test]
    fn classification_style_review_is_yellow() {
        let d = ProcessDetail::new();
        let style = d.classification_style("REVIEW");
        assert_eq!(style.fg, Some(Color::Yellow));
    }

    #[test]
    fn classification_style_spare_is_green() {
        let d = ProcessDetail::new();
        let style = d.classification_style("SPARE");
        assert_eq!(style.fg, Some(Color::Green));
    }

    #[test]
    fn classification_style_unknown_is_default() {
        let d = ProcessDetail::new();
        let style = d.classification_style("OTHER");
        assert_eq!(style, Style::default());
    }

    #[test]
    fn classification_style_case_insensitive() {
        let d = ProcessDetail::new();
        let style = d.classification_style("kill");
        assert_eq!(style.fg, Some(Color::Red));
    }

    // ── label_style / value_style (without theme) ───────────────────

    #[test]
    fn label_style_without_theme() {
        let d = ProcessDetail::new();
        let style = d.label_style();
        assert_eq!(style.fg, Some(Color::DarkGray));
    }

    #[test]
    fn value_style_without_theme() {
        let d = ProcessDetail::new();
        let style = d.value_style();
        assert_eq!(style, Style::default());
    }

    // ── Render: REVIEW and SPARE rows ───────────────────────────────

    #[test]
    fn detail_renders_review_row() {
        let area = Rect::new(0, 0, 60, 16);
        let mut buf = Buffer::empty(area);
        let mut row = sample_row();
        row.classification = "REVIEW".to_string();
        row.score = 65;
        ProcessDetail::new()
            .row(Some(&row), false)
            .render(area, &mut buf);
        assert!(buffer_contains(&buf, area, "REVIEW"));
    }

    #[test]
    fn detail_renders_spare_row() {
        let area = Rect::new(0, 0, 60, 16);
        let mut buf = Buffer::empty(area);
        let mut row = sample_row();
        row.classification = "SPARE".to_string();
        row.score = 20;
        ProcessDetail::new()
            .row(Some(&row), false)
            .render(area, &mut buf);
        assert!(buffer_contains(&buf, area, "SPARE"));
    }
}
