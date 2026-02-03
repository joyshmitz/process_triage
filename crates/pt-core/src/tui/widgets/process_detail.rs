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

/// Detail pane widget for a selected process.
pub struct ProcessDetail<'a> {
    block: Option<Block<'a>>,
    theme: Option<&'a Theme>,
    row: Option<&'a ProcessRow>,
    selected: bool,
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
        let block = self.block.unwrap_or_else(|| {
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
                Span::styled(row.classification.clone(), self.classification_style(&row.classification)),
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

        let evidence = vec![
            Line::from(vec![Span::styled("Evidence", self.label_style())]),
            Line::from(vec![Span::styled("• ledger not wired yet", self.value_style())]),
            Line::from(vec![Span::styled("• impact + tree pending", self.value_style())]),
        ];

        let action = vec![
            Line::from(vec![Span::styled("Action", self.label_style())]),
            Line::from(vec![Span::styled("• plan details pending", self.value_style())]),
        ];

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
