//! Theme and styling for the Process Triage TUI.
//!
//! Provides consistent colors and styles across all widgets.

use ratatui::style::{Color, Modifier, Style};

/// Theme mode selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ThemeMode {
    /// Light theme for high ambient light.
    Light,
    /// Dark theme (default).
    #[default]
    Dark,
    /// High contrast for accessibility.
    HighContrast,
}

/// Theme configuration for the TUI.
#[derive(Debug, Clone)]
pub struct Theme {
    /// Current theme mode.
    pub mode: ThemeMode,

    /// Background color for main areas.
    pub bg: Color,
    /// Primary foreground text color.
    pub fg: Color,

    /// Highlight color for selections.
    pub highlight: Color,
    /// Muted color for secondary information.
    pub muted: Color,

    /// Color for KILL classification (red).
    pub kill: Color,
    /// Color for REVIEW classification (yellow).
    pub review: Color,
    /// Color for SPARE classification (green).
    pub spare: Color,

    /// Danger/error color.
    pub danger: Color,
    /// Warning color.
    pub warning: Color,
    /// Success color.
    pub success: Color,

    /// Border color for unfocused widgets.
    pub border: Color,
    /// Border color for focused widgets.
    pub border_focused: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Self::dark()
    }
}

impl Theme {
    /// Create a dark theme (default).
    pub fn dark() -> Self {
        Self {
            mode: ThemeMode::Dark,
            bg: Color::Reset,
            fg: Color::White,
            highlight: Color::Cyan,
            muted: Color::DarkGray,
            kill: Color::Red,
            review: Color::Yellow,
            spare: Color::Green,
            danger: Color::Red,
            warning: Color::Yellow,
            success: Color::Green,
            border: Color::DarkGray,
            border_focused: Color::Cyan,
        }
    }

    /// Create a light theme.
    pub fn light() -> Self {
        Self {
            mode: ThemeMode::Light,
            bg: Color::White,
            fg: Color::Black,
            highlight: Color::Blue,
            muted: Color::Gray,
            kill: Color::Red,
            review: Color::Rgb(200, 150, 0), // Darker yellow
            spare: Color::Rgb(0, 128, 0),    // Darker green
            danger: Color::Red,
            warning: Color::Rgb(200, 150, 0),
            success: Color::Rgb(0, 128, 0),
            border: Color::Gray,
            border_focused: Color::Blue,
        }
    }

    /// Create a high contrast theme.
    pub fn high_contrast() -> Self {
        Self {
            mode: ThemeMode::HighContrast,
            bg: Color::Black,
            fg: Color::White,
            highlight: Color::Yellow,
            muted: Color::White,
            kill: Color::LightRed,
            review: Color::LightYellow,
            spare: Color::LightGreen,
            danger: Color::LightRed,
            warning: Color::LightYellow,
            success: Color::LightGreen,
            border: Color::White,
            border_focused: Color::Yellow,
        }
    }

    /// Get style for normal text.
    pub fn style_normal(&self) -> Style {
        Style::default().fg(self.fg).bg(self.bg)
    }

    /// Get style for highlighted/selected items.
    pub fn style_highlight(&self) -> Style {
        Style::default()
            .fg(self.highlight)
            .add_modifier(Modifier::BOLD)
    }

    /// Get style for muted/secondary text.
    pub fn style_muted(&self) -> Style {
        Style::default().fg(self.muted)
    }

    /// Get style for KILL classification.
    pub fn style_kill(&self) -> Style {
        Style::default().fg(self.kill).add_modifier(Modifier::BOLD)
    }

    /// Get style for REVIEW classification.
    pub fn style_review(&self) -> Style {
        Style::default().fg(self.review)
    }

    /// Get style for SPARE classification.
    pub fn style_spare(&self) -> Style {
        Style::default().fg(self.spare)
    }

    /// Get border style for unfocused widgets.
    pub fn style_border(&self) -> Style {
        Style::default().fg(self.border)
    }

    /// Get border style for focused widgets.
    pub fn style_border_focused(&self) -> Style {
        Style::default().fg(self.border_focused)
    }

    /// Get style for error messages.
    pub fn style_error(&self) -> Style {
        Style::default()
            .fg(self.danger)
            .add_modifier(Modifier::BOLD)
    }

    /// Get style for success messages.
    pub fn style_success(&self) -> Style {
        Style::default().fg(self.success)
    }

    /// Get style for warning messages.
    pub fn style_warning(&self) -> Style {
        Style::default().fg(self.warning)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_theme_is_dark() {
        let theme = Theme::default();
        assert_eq!(theme.mode, ThemeMode::Dark);
    }

    #[test]
    fn test_theme_modes_have_distinct_colors() {
        let dark = Theme::dark();
        let light = Theme::light();
        let hc = Theme::high_contrast();

        // Background should differ between light and dark
        assert_ne!(dark.bg, light.bg);
        // High contrast should be distinct
        assert_eq!(hc.fg, Color::White);
        assert_eq!(hc.bg, Color::Black);
    }

    #[test]
    fn test_classification_colors_consistent() {
        for theme in [Theme::dark(), Theme::light(), Theme::high_contrast()] {
            // Kill should always be some shade of red
            assert!(matches!(
                theme.kill,
                Color::Red | Color::LightRed | Color::Rgb(_, _, _)
            ));
        }
    }
}
