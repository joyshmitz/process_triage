//! Confirmation dialog widget.
//!
//! Modal dialog for confirming destructive actions like process termination.

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Clear, Paragraph, StatefulWidget, Widget, Wrap},
};

use crate::tui::theme::Theme;

/// Button choice in confirmation dialog.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ConfirmChoice {
    /// Yes/Confirm button.
    Yes,
    /// No/Cancel button (default).
    #[default]
    No,
}

/// Confirmation dialog widget.
#[derive(Debug)]
pub struct ConfirmDialog<'a> {
    /// Dialog title.
    title: &'a str,
    /// Message text.
    message: &'a str,
    /// Optional details (e.g., process list).
    details: Option<&'a str>,
    /// Theme for styling.
    theme: Option<&'a Theme>,
    /// Yes button label.
    yes_label: &'a str,
    /// No button label.
    no_label: &'a str,
}

impl<'a> Default for ConfirmDialog<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> ConfirmDialog<'a> {
    /// Create a new confirmation dialog.
    pub fn new() -> Self {
        Self {
            title: "Confirm",
            message: "Are you sure?",
            details: None,
            theme: None,
            yes_label: "Yes",
            no_label: "No",
        }
    }

    /// Set the dialog title.
    pub fn title(mut self, title: &'a str) -> Self {
        self.title = title;
        self
    }

    /// Set the message text.
    pub fn message(mut self, message: &'a str) -> Self {
        self.message = message;
        self
    }

    /// Set optional details text.
    pub fn details(mut self, details: &'a str) -> Self {
        self.details = Some(details);
        self
    }

    /// Set the theme.
    pub fn theme(mut self, theme: &'a Theme) -> Self {
        self.theme = Some(theme);
        self
    }

    /// Set button labels.
    pub fn labels(mut self, yes: &'a str, no: &'a str) -> Self {
        self.yes_label = yes;
        self.no_label = no;
        self
    }

    /// Calculate dialog area centered in parent.
    fn dialog_area(&self, parent: Rect) -> Rect {
        let width = 60.min(parent.width.saturating_sub(4));
        let height = if self.details.is_some() { 12 } else { 8 };
        let height = height.min(parent.height.saturating_sub(4));

        let x = parent.x + (parent.width.saturating_sub(width)) / 2;
        let y = parent.y + (parent.height.saturating_sub(height)) / 2;

        Rect::new(x, y, width, height)
    }
}

impl<'a> StatefulWidget for ConfirmDialog<'a> {
    type State = ConfirmDialogState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let dialog_area = self.dialog_area(area);

        // Clear background
        Clear.render(dialog_area, buf);

        // Draw border
        let border_style = if let Some(theme) = self.theme {
            Style::default().fg(theme.warning)
        } else {
            Style::default().fg(Color::Yellow)
        };

        let block = Block::default()
            .borders(Borders::ALL)
            .title(format!(" {} ", self.title))
            .border_style(border_style);

        let inner = block.inner(dialog_area);
        block.render(dialog_area, buf);

        // Message
        let message_style = if let Some(theme) = self.theme {
            theme.style_normal()
        } else {
            Style::default()
        };

        let message = Paragraph::new(self.message)
            .style(message_style)
            .wrap(Wrap { trim: true });
        message.render(Rect::new(inner.x, inner.y, inner.width, 2), buf);

        // Details (if present)
        let button_y = if let Some(details) = self.details {
            let details_style = if let Some(theme) = self.theme {
                theme.style_muted()
            } else {
                Style::default().fg(Color::DarkGray)
            };

            let details_area = Rect::new(
                inner.x,
                inner.y + 3,
                inner.width,
                inner.height.saturating_sub(6),
            );
            let details_para = Paragraph::new(details)
                .style(details_style)
                .wrap(Wrap { trim: true });
            details_para.render(details_area, buf);

            inner.bottom().saturating_sub(2)
        } else {
            inner.bottom().saturating_sub(2)
        };

        // Buttons
        let yes_style = if state.selected == ConfirmChoice::Yes {
            if let Some(theme) = self.theme {
                Style::default()
                    .fg(theme.bg)
                    .bg(theme.danger)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Red)
                    .add_modifier(Modifier::BOLD)
            }
        } else if let Some(theme) = self.theme {
            theme.style_normal()
        } else {
            Style::default()
        };

        let no_style = if state.selected == ConfirmChoice::No {
            if let Some(theme) = self.theme {
                Style::default()
                    .fg(theme.bg)
                    .bg(theme.highlight)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Cyan)
                    .add_modifier(Modifier::BOLD)
            }
        } else if let Some(theme) = self.theme {
            theme.style_normal()
        } else {
            Style::default()
        };

        let yes_text = format!(" {} ", self.yes_label);
        let no_text = format!(" {} ", self.no_label);
        let total_button_width = yes_text.len() + no_text.len() + 4;
        let button_x = inner.x + (inner.width.saturating_sub(total_button_width as u16)) / 2;

        // Render Yes button
        for (i, ch) in yes_text.chars().enumerate() {
            let x = button_x + (i as u16);
            if x < inner.right() && button_y < inner.bottom() {
                buf[(x, button_y)].set_char(ch).set_style(yes_style);
            }
        }

        // Render No button
        let no_x = button_x + yes_text.len() as u16 + 2;
        for (i, ch) in no_text.chars().enumerate() {
            let x = no_x + (i as u16);
            if x < inner.right() && button_y < inner.bottom() {
                buf[(x, button_y)].set_char(ch).set_style(no_style);
            }
        }
    }
}

/// State for the confirmation dialog.
#[derive(Debug)]
pub struct ConfirmDialogState {
    /// Whether the dialog is visible.
    pub visible: bool,
    /// Currently selected button.
    pub selected: ConfirmChoice,
    /// Result when dialog is dismissed.
    pub result: Option<ConfirmChoice>,
}

impl Default for ConfirmDialogState {
    fn default() -> Self {
        Self::new()
    }
}

impl ConfirmDialogState {
    /// Create a new dialog state.
    pub fn new() -> Self {
        Self {
            visible: false,
            selected: ConfirmChoice::No,
            result: None,
        }
    }

    /// Show the dialog.
    pub fn show(&mut self) {
        self.visible = true;
        self.selected = ConfirmChoice::No;
        self.result = None;
    }

    /// Hide the dialog.
    pub fn hide(&mut self) {
        self.visible = false;
    }

    /// Toggle selected button.
    pub fn toggle(&mut self) {
        self.selected = match self.selected {
            ConfirmChoice::Yes => ConfirmChoice::No,
            ConfirmChoice::No => ConfirmChoice::Yes,
        };
    }

    /// Select left button (Yes).
    pub fn select_left(&mut self) {
        self.selected = ConfirmChoice::Yes;
    }

    /// Select right button (No).
    pub fn select_right(&mut self) {
        self.selected = ConfirmChoice::No;
    }

    /// Confirm with current selection.
    pub fn confirm(&mut self) -> ConfirmChoice {
        self.result = Some(self.selected);
        self.visible = false;
        self.selected
    }

    /// Cancel dialog (equivalent to No).
    pub fn cancel(&mut self) {
        self.result = Some(ConfirmChoice::No);
        self.visible = false;
    }

    /// Check if dialog was confirmed with Yes.
    pub fn was_confirmed(&self) -> bool {
        matches!(self.result, Some(ConfirmChoice::Yes))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dialog_state_defaults() {
        let state = ConfirmDialogState::new();
        assert!(!state.visible);
        assert_eq!(state.selected, ConfirmChoice::No);
        assert!(state.result.is_none());
    }

    #[test]
    fn test_show_and_hide() {
        let mut state = ConfirmDialogState::new();

        state.show();
        assert!(state.visible);
        assert_eq!(state.selected, ConfirmChoice::No);

        state.hide();
        assert!(!state.visible);
    }

    #[test]
    fn test_toggle_selection() {
        let mut state = ConfirmDialogState::new();
        state.show();

        assert_eq!(state.selected, ConfirmChoice::No);

        state.toggle();
        assert_eq!(state.selected, ConfirmChoice::Yes);

        state.toggle();
        assert_eq!(state.selected, ConfirmChoice::No);
    }

    #[test]
    fn test_confirm_yes() {
        let mut state = ConfirmDialogState::new();
        state.show();
        state.select_left(); // Yes

        let choice = state.confirm();
        assert_eq!(choice, ConfirmChoice::Yes);
        assert!(state.was_confirmed());
        assert!(!state.visible);
    }

    #[test]
    fn test_cancel() {
        let mut state = ConfirmDialogState::new();
        state.show();
        state.select_left(); // Yes

        state.cancel();
        assert!(!state.was_confirmed());
        assert_eq!(state.result, Some(ConfirmChoice::No));
    }
}
