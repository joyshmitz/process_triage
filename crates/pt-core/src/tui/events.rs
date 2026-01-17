//! Event handling for the Process Triage TUI.
//!
//! Provides keyboard event handling with customizable key bindings.

use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};

/// Configurable key bindings for TUI navigation.
#[derive(Debug, Clone)]
pub struct KeyBindings {
    /// Key to quit the application.
    pub quit: Vec<KeyEvent>,
    /// Key to confirm selection.
    pub confirm: Vec<KeyEvent>,
    /// Key to cancel/go back.
    pub cancel: Vec<KeyEvent>,
    /// Key to show help.
    pub help: Vec<KeyEvent>,
    /// Key to focus search input.
    pub search: Vec<KeyEvent>,
    /// Key to select next item.
    pub next: Vec<KeyEvent>,
    /// Key to select previous item.
    pub prev: Vec<KeyEvent>,
    /// Key to toggle selection.
    pub toggle: Vec<KeyEvent>,
    /// Key to select all.
    pub select_all: Vec<KeyEvent>,
    /// Key to deselect all.
    pub deselect_all: Vec<KeyEvent>,
    /// Key to execute selected actions.
    pub execute: Vec<KeyEvent>,
    /// Key to switch to next tab/pane.
    pub next_tab: Vec<KeyEvent>,
    /// Key to switch to previous tab/pane.
    pub prev_tab: Vec<KeyEvent>,
}

impl Default for KeyBindings {
    fn default() -> Self {
        Self {
            quit: vec![
                KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE),
                KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL),
            ],
            confirm: vec![KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE)],
            cancel: vec![KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE)],
            help: vec![
                KeyEvent::new(KeyCode::Char('?'), KeyModifiers::NONE),
                KeyEvent::new(KeyCode::F(1), KeyModifiers::NONE),
            ],
            search: vec![KeyEvent::new(KeyCode::Char('/'), KeyModifiers::NONE)],
            next: vec![
                KeyEvent::new(KeyCode::Down, KeyModifiers::NONE),
                KeyEvent::new(KeyCode::Char('j'), KeyModifiers::NONE),
            ],
            prev: vec![
                KeyEvent::new(KeyCode::Up, KeyModifiers::NONE),
                KeyEvent::new(KeyCode::Char('k'), KeyModifiers::NONE),
            ],
            toggle: vec![KeyEvent::new(KeyCode::Char(' '), KeyModifiers::NONE)],
            select_all: vec![KeyEvent::new(KeyCode::Char('a'), KeyModifiers::CONTROL)],
            deselect_all: vec![KeyEvent::new(KeyCode::Char('u'), KeyModifiers::CONTROL)],
            execute: vec![KeyEvent::new(KeyCode::Char('x'), KeyModifiers::NONE)],
            next_tab: vec![KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE)],
            prev_tab: vec![KeyEvent::new(KeyCode::BackTab, KeyModifiers::SHIFT)],
        }
    }
}

impl KeyBindings {
    /// Check if a key event matches any quit binding.
    pub fn is_quit(&self, key: &KeyEvent) -> bool {
        self.quit.iter().any(|k| k == key)
    }

    /// Check if a key event matches any confirm binding.
    pub fn is_confirm(&self, key: &KeyEvent) -> bool {
        self.confirm.iter().any(|k| k == key)
    }

    /// Check if a key event matches any cancel binding.
    pub fn is_cancel(&self, key: &KeyEvent) -> bool {
        self.cancel.iter().any(|k| k == key)
    }

    /// Check if a key event matches any help binding.
    pub fn is_help(&self, key: &KeyEvent) -> bool {
        self.help.iter().any(|k| k == key)
    }

    /// Check if a key event matches any search binding.
    pub fn is_search(&self, key: &KeyEvent) -> bool {
        self.search.iter().any(|k| k == key)
    }

    /// Check if a key event matches any next binding.
    pub fn is_next(&self, key: &KeyEvent) -> bool {
        self.next.iter().any(|k| k == key)
    }

    /// Check if a key event matches any prev binding.
    pub fn is_prev(&self, key: &KeyEvent) -> bool {
        self.prev.iter().any(|k| k == key)
    }

    /// Check if a key event matches any toggle binding.
    pub fn is_toggle(&self, key: &KeyEvent) -> bool {
        self.toggle.iter().any(|k| k == key)
    }

    /// Check if a key event matches any execute binding.
    pub fn is_execute(&self, key: &KeyEvent) -> bool {
        self.execute.iter().any(|k| k == key)
    }
}

/// Application-level action resulting from event handling.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppAction {
    /// No action needed.
    None,
    /// Quit the application.
    Quit,
    /// Show help dialog.
    ShowHelp,
    /// Execute selected actions.
    Execute,
    /// Focus changed to another widget.
    FocusChanged,
    /// Data needs to be refreshed.
    Refresh,
    /// State changed, redraw needed.
    Redraw,
}

/// Handle a crossterm event and return the resulting action.
pub fn handle_event(event: Event, bindings: &KeyBindings) -> AppAction {
    match event {
        Event::Key(key) => handle_key_event(&key, bindings),
        Event::Mouse(_mouse) => {
            // Mouse events are handled by individual widgets
            AppAction::None
        }
        Event::Resize(_, _) => AppAction::Redraw,
        _ => AppAction::None,
    }
}

fn handle_key_event(key: &KeyEvent, bindings: &KeyBindings) -> AppAction {
    if bindings.is_quit(key) {
        return AppAction::Quit;
    }
    if bindings.is_help(key) {
        return AppAction::ShowHelp;
    }
    if bindings.is_execute(key) {
        return AppAction::Execute;
    }

    AppAction::None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_bindings() {
        let bindings = KeyBindings::default();

        // Test quit bindings
        let q_key = KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE);
        assert!(bindings.is_quit(&q_key));

        let ctrl_c = KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL);
        assert!(bindings.is_quit(&ctrl_c));

        // Test navigation
        let down = KeyEvent::new(KeyCode::Down, KeyModifiers::NONE);
        assert!(bindings.is_next(&down));

        let j = KeyEvent::new(KeyCode::Char('j'), KeyModifiers::NONE);
        assert!(bindings.is_next(&j));
    }

    #[test]
    fn test_handle_quit_event() {
        let bindings = KeyBindings::default();
        let quit_event = Event::Key(KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE));

        let action = handle_event(quit_event, &bindings);
        assert_eq!(action, AppAction::Quit);
    }

    #[test]
    fn test_resize_triggers_redraw() {
        let bindings = KeyBindings::default();
        let resize_event = Event::Resize(80, 24);

        let action = handle_event(resize_event, &bindings);
        assert_eq!(action, AppAction::Redraw);
    }
}
