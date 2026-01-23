//! Search input widget for filtering processes.
//!
//! Simple text input widget for Process Triage-specific styling and behavior.

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, StatefulWidget, Widget},
};

use crate::tui::theme::Theme;

/// Search input widget for filtering the process list.
#[derive(Debug, Default)]
pub struct SearchInput<'a> {
    /// Block wrapper for the input.
    block: Option<Block<'a>>,
    /// Placeholder text when empty.
    placeholder: &'a str,
    /// Theme for styling.
    theme: Option<&'a Theme>,
}

impl<'a> SearchInput<'a> {
    /// Create a new search input.
    pub fn new() -> Self {
        Self {
            block: None,
            placeholder: "Search processes...",
            theme: None,
        }
    }

    /// Set the block wrapper.
    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }

    /// Set the placeholder text.
    pub fn placeholder(mut self, placeholder: &'a str) -> Self {
        self.placeholder = placeholder;
        self
    }

    /// Set the theme.
    pub fn theme(mut self, theme: &'a Theme) -> Self {
        self.theme = Some(theme);
        self
    }

    /// Build the styled block based on focus state.
    fn styled_block(&self, focused: bool) -> Block<'a> {
        let title = if focused {
            " Search [Enter to filter] "
        } else {
            " Search "
        };

        let border_style = if let Some(theme) = self.theme {
            if focused {
                theme.style_border_focused()
            } else {
                theme.style_border()
            }
        } else {
            Style::default().fg(if focused {
                Color::Cyan
            } else {
                Color::DarkGray
            })
        };

        Block::default()
            .borders(Borders::ALL)
            .title(title)
            .border_style(border_style)
    }
}

impl<'a> StatefulWidget for SearchInput<'a> {
    type State = SearchInputState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let focused = state.focused;
        let block = self
            .block
            .clone()
            .unwrap_or_else(|| self.styled_block(focused));

        let inner = block.inner(area);
        block.render(area, buf);

        let input_style = if let Some(theme) = self.theme {
            if focused {
                theme.style_highlight()
            } else {
                theme.style_normal()
            }
        } else {
            Style::default()
        };

        // Render the current value
        if state.value.is_empty() && !focused {
            // Render placeholder
            let placeholder_style = if let Some(theme) = self.theme {
                theme.style_muted()
            } else {
                Style::default().fg(Color::DarkGray)
            };

            for (i, ch) in self.placeholder.chars().enumerate() {
                if inner.x + i as u16 >= inner.right() {
                    break;
                }
                buf[(inner.x + i as u16, inner.y)]
                    .set_char(ch)
                    .set_style(placeholder_style);
            }
        } else {
            // Render value with cursor
            let display = if focused {
                format!("{}_", state.value)
            } else {
                state.value.clone()
            };

            for (i, ch) in display.chars().enumerate() {
                if inner.x + i as u16 >= inner.right() {
                    break;
                }
                buf[(inner.x + i as u16, inner.y)]
                    .set_char(ch)
                    .set_style(input_style);
            }
        }
    }
}

/// State for the search input widget.
#[derive(Debug, Clone)]
pub struct SearchInputState {
    /// Current input value.
    pub value: String,
    /// Whether the input is focused.
    pub focused: bool,
    /// Search history (last N searches).
    history: Vec<String>,
    /// Current position in history (for up/down navigation).
    history_pos: Option<usize>,
}

impl Default for SearchInputState {
    fn default() -> Self {
        Self::new()
    }
}

impl SearchInputState {
    /// Create a new search input state.
    pub fn new() -> Self {
        Self {
            value: String::new(),
            focused: false,
            history: Vec::new(),
            history_pos: None,
        }
    }

    /// Get the current search value.
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Set the search value.
    pub fn set_value(&mut self, value: &str) {
        self.value = value.to_string();
    }

    /// Clear the search input.
    pub fn clear(&mut self) {
        self.value.clear();
        self.history_pos = None;
    }

    /// Type a character into the input.
    pub fn type_char(&mut self, ch: char) {
        self.value.push(ch);
    }

    /// Delete the last character.
    pub fn backspace(&mut self) {
        self.value.pop();
    }

    /// Commit current search to history.
    pub fn commit(&mut self) {
        if !self.value.is_empty() {
            // Remove if already in history
            let value = self.value.clone();
            self.history.retain(|h| h != &value);
            // Add to front
            self.history.insert(0, value);
            // Limit history size
            if self.history.len() > 10 {
                self.history.pop();
            }
        }
        self.history_pos = None;
    }

    /// Navigate to previous search in history.
    pub fn history_prev(&mut self) {
        if self.history.is_empty() {
            return;
        }

        let new_pos = match self.history_pos {
            None => 0,
            Some(pos) if pos + 1 < self.history.len() => pos + 1,
            Some(pos) => pos,
        };

        self.history_pos = Some(new_pos);
        self.value = self.history[new_pos].clone();
    }

    /// Navigate to next search in history.
    pub fn history_next(&mut self) {
        match self.history_pos {
            None => {}
            Some(0) => {
                self.history_pos = None;
                self.value.clear();
            }
            Some(pos) => {
                self.history_pos = Some(pos - 1);
                self.value = self.history[pos - 1].clone();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_input_state_new() {
        let state = SearchInputState::new();
        assert_eq!(state.value(), "");
        assert!(state.history.is_empty());
    }

    #[test]
    fn test_set_and_get_value() {
        let mut state = SearchInputState::new();
        state.set_value("test query");
        assert_eq!(state.value(), "test query");
    }

    #[test]
    fn test_clear() {
        let mut state = SearchInputState::new();
        state.set_value("test");
        state.clear();
        assert_eq!(state.value(), "");
    }

    #[test]
    fn test_type_and_backspace() {
        let mut state = SearchInputState::new();
        state.type_char('a');
        state.type_char('b');
        state.type_char('c');
        assert_eq!(state.value(), "abc");

        state.backspace();
        assert_eq!(state.value(), "ab");
    }

    #[test]
    fn test_history() {
        let mut state = SearchInputState::new();

        state.set_value("first");
        state.commit();

        state.set_value("second");
        state.commit();

        state.set_value("third");
        state.commit();

        assert_eq!(state.history.len(), 3);
        assert_eq!(state.history[0], "third");
        assert_eq!(state.history[1], "second");
        assert_eq!(state.history[2], "first");
    }

    #[test]
    fn test_history_navigation() {
        let mut state = SearchInputState::new();

        state.set_value("first");
        state.commit();
        state.set_value("second");
        state.commit();

        state.clear();

        state.history_prev();
        assert_eq!(state.value(), "second");

        state.history_prev();
        assert_eq!(state.value(), "first");

        state.history_next();
        assert_eq!(state.value(), "second");

        state.history_next();
        assert_eq!(state.value(), "");
    }

    #[test]
    fn test_history_deduplication() {
        let mut state = SearchInputState::new();

        state.set_value("query");
        state.commit();

        state.set_value("other");
        state.commit();

        state.set_value("query");
        state.commit();

        // Should have only 2 entries, with "query" at front
        assert_eq!(state.history.len(), 2);
        assert_eq!(state.history[0], "query");
        assert_eq!(state.history[1], "other");
    }
}
