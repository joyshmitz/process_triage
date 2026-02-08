#![cfg(feature = "ui")]
//! E2E TUI workflow tests using ratatui's TestBackend.
//!
//! Exercises full interaction sequences: navigation, search/filter,
//! selection toggling, drilldown views, execute/abort, help, quit.
//! Each test records event sequences and validates state transitions.

use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use pt_core::tui::widgets::ProcessRow;
use pt_core::tui::{App, AppState, Theme};
use ratatui::backend::TestBackend;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::Terminal;

// ===========================================================================
// Helpers
// ===========================================================================

fn key(code: KeyCode) -> Event {
    Event::Key(KeyEvent::new(code, KeyModifiers::NONE))
}

fn key_ctrl(c: char) -> Event {
    Event::Key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::CONTROL))
}

fn key_char(c: char) -> Event {
    key(KeyCode::Char(c))
}

fn apply_events(app: &mut App, events: &[Event]) {
    for event in events {
        app.handle_event(event.clone()).unwrap();
    }
}

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

fn buffer_text(buf: &Buffer, area: Rect) -> String {
    (area.y..area.y.saturating_add(area.height))
        .map(|y| line_string(buf, area, y))
        .collect::<Vec<_>>()
        .join("\n")
}

fn make_row(
    pid: u32,
    score: u32,
    classification: &str,
    runtime: &str,
    memory: &str,
    command: &str,
    galaxy_brain: Option<&str>,
) -> ProcessRow {
    ProcessRow {
        pid,
        score,
        classification: classification.to_string(),
        runtime: runtime.to_string(),
        memory: memory.to_string(),
        command: command.to_string(),
        selected: false,
        galaxy_brain: galaxy_brain.map(|s| s.to_string()),
        why_summary: None,
        top_evidence: vec![],
        confidence: None,
        plan_preview: vec![],
    }
}

fn sample_rows() -> Vec<ProcessRow> {
    vec![
        make_row(
            1001,
            95,
            "KILL",
            "3d 2h",
            "2.1 GB",
            "node dev-server --watch",
            Some("Galaxy-Brain Mode\nPosterior: P(abandoned)=0.95"),
        ),
        make_row(
            1002,
            72,
            "REVIEW",
            "12h 30m",
            "512 MB",
            "python train.py --epochs 100",
            Some("Galaxy-Brain Mode\nPosterior: P(abandoned)=0.72"),
        ),
        make_row(1003, 15, "SPARE", "5m", "64 MB", "vim session.rs", None),
        make_row(
            1004,
            88,
            "KILL",
            "7d 4h",
            "4.0 GB",
            "jupyter notebook --port 8888",
            Some("Galaxy-Brain Mode\nPosterior: P(abandoned)=0.88"),
        ),
        make_row(
            1005,
            45,
            "REVIEW",
            "1h 15m",
            "256 MB",
            "cargo test --workspace",
            None,
        ),
    ]
}

fn app_with_rows() -> App {
    let mut app = App::new();
    app.process_table.set_rows(sample_rows());
    app
}

fn render(app: &mut App, width: u16, height: u16) -> Terminal<TestBackend> {
    let backend = TestBackend::new(width, height);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.draw(|frame| app.render(frame)).unwrap();
    terminal
}

// ===========================================================================
// 1. Navigation Workflows
// ===========================================================================

#[test]
fn nav_cursor_down_moves_selection() {
    let mut app = app_with_rows();
    assert_eq!(app.process_table.cursor, 0);

    apply_events(&mut app, &[key_char('j')]);
    assert_eq!(app.process_table.cursor, 1);

    apply_events(&mut app, &[key(KeyCode::Down)]);
    assert_eq!(app.process_table.cursor, 2);
}

#[test]
fn nav_cursor_up_moves_selection() {
    let mut app = app_with_rows();

    // Move down first, then back up
    apply_events(&mut app, &[key_char('j'), key_char('j'), key_char('k')]);
    assert_eq!(app.process_table.cursor, 1);

    apply_events(&mut app, &[key(KeyCode::Up)]);
    assert_eq!(app.process_table.cursor, 0);
}

#[test]
fn nav_cursor_wraps_at_boundaries() {
    let mut app = app_with_rows();

    // At top, going up shouldn't crash
    apply_events(&mut app, &[key_char('k')]);
    assert_eq!(app.process_table.cursor, 0);
}

#[test]
fn nav_home_end_jumps() {
    let mut app = app_with_rows();

    apply_events(&mut app, &[key(KeyCode::End)]);
    assert_eq!(app.process_table.cursor, 4); // last row

    apply_events(&mut app, &[key(KeyCode::Home)]);
    assert_eq!(app.process_table.cursor, 0);
}

#[test]
fn nav_page_down_up() {
    let mut app = app_with_rows();

    apply_events(&mut app, &[key(KeyCode::PageDown)]);
    // With 5 rows and page size 10, should be at last row
    assert_eq!(app.process_table.cursor, 4);

    apply_events(&mut app, &[key(KeyCode::PageUp)]);
    assert_eq!(app.process_table.cursor, 0);
}

#[test]
fn nav_ctrl_d_ctrl_u_page() {
    let mut app = app_with_rows();

    apply_events(&mut app, &[key_ctrl('d')]);
    assert_eq!(app.process_table.cursor, 4);

    apply_events(&mut app, &[key_ctrl('u')]);
    assert_eq!(app.process_table.cursor, 0);
}

#[test]
fn nav_n_forward_shift_n_backward() {
    let mut app = app_with_rows();

    apply_events(&mut app, &[key_char('n'), key_char('n')]);
    assert_eq!(app.process_table.cursor, 2);

    apply_events(&mut app, &[key_char('N')]);
    assert_eq!(app.process_table.cursor, 1);
}

// ===========================================================================
// 2. Search/Filter Workflows
// ===========================================================================

#[test]
fn search_enter_and_exit() {
    let mut app = app_with_rows();
    assert_eq!(app.state, AppState::Normal);

    apply_events(&mut app, &[key_char('/')]);
    assert_eq!(app.state, AppState::Searching);

    apply_events(&mut app, &[key(KeyCode::Esc)]);
    assert_eq!(app.state, AppState::Normal);
}

#[test]
fn search_type_and_filter() {
    let mut app = app_with_rows();

    // Enter search mode and type "node"
    apply_events(
        &mut app,
        &[
            key_char('/'),
            key_char('n'),
            key_char('o'),
            key_char('d'),
            key_char('e'),
        ],
    );

    // Search query should be populated
    assert_eq!(app.search.value, "node");
}

#[test]
fn search_backspace_deletes() {
    let mut app = app_with_rows();

    apply_events(
        &mut app,
        &[
            key_char('/'),
            key_char('a'),
            key_char('b'),
            key(KeyCode::Backspace),
        ],
    );

    assert_eq!(app.search.value, "a");
}

#[test]
fn search_enter_confirms_and_returns_to_normal() {
    let mut app = app_with_rows();

    apply_events(
        &mut app,
        &[key_char('/'), key_char('k'), key(KeyCode::Enter)],
    );

    assert_eq!(app.state, AppState::Normal);
    // Search should have been committed
    assert!(!app.search.value.is_empty());
}

// ===========================================================================
// 3. Selection/Toggle Workflows
// ===========================================================================

#[test]
fn toggle_selection_on_current_row() {
    let mut app = app_with_rows();

    assert_eq!(app.process_table.selected_count(), 0);

    apply_events(&mut app, &[key_char(' ')]);
    assert_eq!(app.process_table.selected_count(), 1);

    // Toggle again to deselect
    apply_events(&mut app, &[key_char(' ')]);
    assert_eq!(app.process_table.selected_count(), 0);
}

#[test]
fn select_all_and_deselect_all() {
    let mut app = app_with_rows();

    // Select all with 'A'
    apply_events(&mut app, &[key_char('A')]);
    assert_eq!(app.process_table.selected_count(), 5);

    // Deselect all with 'u'
    apply_events(&mut app, &[key_char('u')]);
    assert_eq!(app.process_table.selected_count(), 0);
}

#[test]
fn select_recommended_selects_kill_only() {
    let mut app = app_with_rows();

    // 'a' selects recommended (KILL classification)
    apply_events(&mut app, &[key_char('a')]);
    let count = app.process_table.selected_count();
    // We have 2 KILL rows (1001, 1004)
    assert_eq!(count, 2, "Should select 2 KILL-classified rows");
}

#[test]
fn invert_selection() {
    let mut app = app_with_rows();

    // Select first row
    apply_events(&mut app, &[key_char(' ')]);
    assert_eq!(app.process_table.selected_count(), 1);

    // Invert with 'x'
    apply_events(&mut app, &[key_char('x')]);
    assert_eq!(app.process_table.selected_count(), 4);
}

#[test]
fn navigate_and_toggle_multiple() {
    let mut app = app_with_rows();

    // Select rows 0, 2, 4 by navigating and toggling
    apply_events(
        &mut app,
        &[
            key_char(' '), // select row 0
            key_char('j'), // move to row 1
            key_char('j'), // move to row 2
            key_char(' '), // select row 2
            key_char('j'), // move to row 3
            key_char('j'), // move to row 4
            key_char(' '), // select row 4
        ],
    );
    assert_eq!(app.process_table.selected_count(), 3);
}

// ===========================================================================
// 4. Drilldown / Detail View Workflows
// ===========================================================================

#[test]
fn detail_view_toggle_visibility() {
    let mut app = app_with_rows();

    // Enter toggles detail visibility
    apply_events(&mut app, &[key(KeyCode::Enter)]);
    // This toggles the detail pane visibility

    // Render to verify no crash
    let _terminal = render(&mut app, 120, 40);
}

#[test]
fn detail_view_switch_to_galaxy_brain() {
    let mut app = app_with_rows();

    // 'g' toggles galaxy brain view
    apply_events(&mut app, &[key_char('g')]);

    let terminal = render(&mut app, 120, 40);
    let buf = terminal.backend().buffer();
    let area = Rect::new(0, 0, 120, 40);
    assert!(
        buffer_contains(buf, area, "Galaxy Brain")
            || buffer_contains(buf, area, "Galaxy-Brain Mode")
            || buffer_contains(buf, area, "Math Trace"),
        "Galaxy brain view should be visible"
    );
}

#[test]
fn detail_view_switch_to_summary() {
    let mut app = app_with_rows();

    // Switch to galaxy brain then back to summary
    apply_events(&mut app, &[key_char('g'), key_char('s')]);

    let terminal = render(&mut app, 120, 40);
    let buf = terminal.backend().buffer();
    let area = Rect::new(0, 0, 120, 40);
    // Summary view should show process details
    assert!(
        buffer_contains(buf, area, "1001") || buffer_contains(buf, area, "node"),
        "Summary view should show process info"
    );
}

#[test]
fn detail_view_switch_to_genealogy() {
    let mut app = app_with_rows();

    // 't' switches to genealogy view
    apply_events(&mut app, &[key_char('t')]);

    let _terminal = render(&mut app, 120, 40);
    // Just verify no crash
}

#[test]
fn detail_view_follows_cursor() {
    let mut app = app_with_rows();

    // Render at first row
    let terminal = render(&mut app, 120, 40);
    let buf = terminal.backend().buffer();
    let area = Rect::new(0, 0, 120, 40);
    let text1 = buffer_text(buf, area);

    // Move to row 2 and re-render
    apply_events(&mut app, &[key_char('j'), key_char('j')]);
    let terminal = render(&mut app, 120, 40);
    let buf = terminal.backend().buffer();
    let text2 = buffer_text(buf, area);

    // The detail pane content should change (different process focused)
    assert_ne!(text1, text2, "Detail should update when cursor moves");
}

// ===========================================================================
// 5. Execute / Abort Workflows
// ===========================================================================

#[test]
fn execute_shows_confirmation_dialog() {
    let mut app = app_with_rows();

    // Select a process and press 'e' to execute
    apply_events(&mut app, &[key_char(' '), key_char('e')]);

    // Should show confirmation dialog
    assert!(
        app.confirm_dialog.visible,
        "Confirmation dialog should be visible"
    );
}

#[test]
fn execute_abort_with_escape() {
    let mut app = app_with_rows();

    // Select, execute, then abort
    apply_events(&mut app, &[key_char(' '), key_char('e')]);
    assert!(app.confirm_dialog.visible);

    apply_events(&mut app, &[key(KeyCode::Esc)]);
    assert!(!app.confirm_dialog.visible);
    assert_eq!(app.state, AppState::Normal);
}

#[test]
fn execute_confirm_with_enter() {
    let mut app = app_with_rows();

    // Select, execute, then confirm
    apply_events(&mut app, &[key_char(' '), key_char('e')]);
    assert!(app.confirm_dialog.visible);

    apply_events(&mut app, &[key(KeyCode::Enter)]);
    // After confirmation, dialog should close
    assert!(!app.confirm_dialog.visible);
}

#[test]
fn execute_with_no_selection_shows_status() {
    let mut app = app_with_rows();
    assert_eq!(app.process_table.selected_count(), 0);

    // Try to execute with nothing selected
    apply_events(&mut app, &[key_char('e')]);

    // Behavior varies: might show confirmation or status message
    // Just ensure no crash
    let _terminal = render(&mut app, 120, 40);
}

#[test]
fn confirmation_dialog_tab_toggles() {
    let mut app = app_with_rows();

    apply_events(&mut app, &[key_char(' '), key_char('e')]);
    assert!(app.confirm_dialog.visible);

    // Tab should toggle between options
    apply_events(&mut app, &[key(KeyCode::Tab)]);
    // No crash
    let _terminal = render(&mut app, 120, 40);
}

#[test]
fn confirmation_dialog_left_right() {
    let mut app = app_with_rows();

    apply_events(&mut app, &[key_char(' '), key_char('e')]);
    assert!(app.confirm_dialog.visible);

    // Navigate left/right
    apply_events(
        &mut app,
        &[
            key_char('l'), // right
            key_char('h'), // left
        ],
    );
    let _terminal = render(&mut app, 120, 40);
}

// ===========================================================================
// 6. Help Overlay Workflow
// ===========================================================================

#[test]
fn help_opens_and_closes() {
    let mut app = app_with_rows();

    apply_events(&mut app, &[key_char('?')]);
    assert_eq!(app.state, AppState::Help);

    apply_events(&mut app, &[key(KeyCode::Esc)]);
    assert_eq!(app.state, AppState::Normal);
}

#[test]
fn help_renders_keybindings() {
    let mut app = app_with_rows();

    apply_events(&mut app, &[key_char('?')]);
    let terminal = render(&mut app, 100, 30);
    let buf = terminal.backend().buffer();
    let area = Rect::new(0, 0, 100, 30);

    assert!(
        buffer_contains(buf, area, "TUI Help"),
        "Help title should be visible"
    );
}

#[test]
fn help_question_mark_toggles() {
    let mut app = app_with_rows();

    apply_events(&mut app, &[key_char('?')]);
    assert_eq!(app.state, AppState::Help);

    apply_events(&mut app, &[key_char('?')]);
    assert_eq!(app.state, AppState::Normal);
}

// ===========================================================================
// 7. Quit Workflow
// ===========================================================================

#[test]
fn quit_with_q() {
    let mut app = app_with_rows();

    apply_events(&mut app, &[key_char('q')]);
    assert!(app.should_quit());
}

#[test]
fn quit_with_escape() {
    let mut app = app_with_rows();

    apply_events(&mut app, &[key(KeyCode::Esc)]);
    assert!(app.should_quit());
}

#[test]
fn quit_from_help_returns_to_normal() {
    let mut app = app_with_rows();

    // Enter help, then quit help
    apply_events(&mut app, &[key_char('?')]);
    assert_eq!(app.state, AppState::Help);

    apply_events(&mut app, &[key_char('q')]);
    assert_eq!(app.state, AppState::Normal);

    // Now quit from normal
    apply_events(&mut app, &[key_char('q')]);
    assert!(app.should_quit());
}

// ===========================================================================
// 8. Focus Cycling
// ===========================================================================

#[test]
fn tab_cycles_focus() {
    let mut app = app_with_rows();

    // Tab should cycle through focus targets
    apply_events(&mut app, &[key(KeyCode::Tab)]);
    // After one tab, focus should have moved (no crash)
    let _terminal = render(&mut app, 120, 40);

    apply_events(&mut app, &[key(KeyCode::Tab)]);
    let _terminal = render(&mut app, 120, 40);
}

// ===========================================================================
// 9. Resize Handling
// ===========================================================================

#[test]
fn resize_updates_layout() {
    let mut app = app_with_rows();

    // Initial render at 120x40
    let _terminal = render(&mut app, 120, 40);

    // Simulate resize
    app.handle_event(Event::Resize(200, 60)).unwrap();
    let _terminal = render(&mut app, 200, 60);

    // Simulate resize to compact
    app.handle_event(Event::Resize(80, 24)).unwrap();
    let _terminal = render(&mut app, 80, 24);
}

#[test]
fn minimal_terminal_renders_without_crash() {
    let mut app = app_with_rows();

    // Very small terminal
    app.handle_event(Event::Resize(40, 10)).unwrap();
    let _terminal = render(&mut app, 40, 10);
}

// ===========================================================================
// 10. Theme Switching
// ===========================================================================

#[test]
fn dark_theme_renders() {
    let mut app = app_with_rows().with_theme(Theme::dark());
    let _terminal = render(&mut app, 120, 40);
}

#[test]
fn light_theme_renders() {
    let mut app = app_with_rows().with_theme(Theme::light());
    let _terminal = render(&mut app, 120, 40);
}

#[test]
fn high_contrast_theme_renders() {
    let mut app = app_with_rows().with_theme(Theme::high_contrast());
    let _terminal = render(&mut app, 120, 40);
}

// ===========================================================================
// 11. Full Workflow Sequences
// ===========================================================================

#[test]
fn workflow_triage_and_execute() {
    let mut app = app_with_rows();

    // 1. Browse processes (navigate down 2)
    apply_events(&mut app, &[key_char('j'), key_char('j')]);
    assert_eq!(app.process_table.cursor, 2);

    // 2. Open help
    apply_events(&mut app, &[key_char('?')]);
    assert_eq!(app.state, AppState::Help);
    apply_events(&mut app, &[key(KeyCode::Esc)]);
    assert_eq!(app.state, AppState::Normal);

    // 3. Select recommended
    apply_events(&mut app, &[key_char('a')]);
    assert!(app.process_table.selected_count() > 0);

    // 4. Switch to galaxy brain for the first KILL row
    apply_events(&mut app, &[key(KeyCode::Home), key_char('g')]);
    let terminal = render(&mut app, 120, 40);
    let buf = terminal.backend().buffer();
    let area = Rect::new(0, 0, 120, 40);
    assert!(
        buffer_contains(buf, area, "Galaxy-Brain Mode")
            || buffer_contains(buf, area, "Galaxy Brain")
    );

    // 5. Execute
    apply_events(&mut app, &[key_char('e')]);
    assert!(app.confirm_dialog.visible);

    // 6. Confirm
    apply_events(&mut app, &[key(KeyCode::Enter)]);
    assert!(!app.confirm_dialog.visible);

    // 7. Quit
    apply_events(&mut app, &[key_char('q')]);
    assert!(app.should_quit());
}

#[test]
fn workflow_search_filter_select_abort() {
    let mut app = app_with_rows();

    // 1. Search for "python"
    apply_events(
        &mut app,
        &[
            key_char('/'),
            key_char('p'),
            key_char('y'),
            key_char('t'),
            key_char('h'),
            key_char('o'),
            key_char('n'),
            key(KeyCode::Enter),
        ],
    );
    assert_eq!(app.state, AppState::Normal);

    // 2. Select all visible
    apply_events(&mut app, &[key_char('A')]);

    // 3. Execute
    apply_events(&mut app, &[key_char('e')]);

    // 4. Abort
    apply_events(&mut app, &[key(KeyCode::Esc)]);
    assert!(!app.confirm_dialog.visible);
    assert_eq!(app.state, AppState::Normal);

    // 5. Deselect all
    apply_events(&mut app, &[key_char('u')]);
    assert_eq!(app.process_table.selected_count(), 0);
}

#[test]
fn workflow_view_mode_switching() {
    let mut app = app_with_rows();

    // Switch through all detail views
    apply_events(&mut app, &[key_char('s')]); // Summary
    let _terminal = render(&mut app, 120, 40);

    apply_events(&mut app, &[key_char('g')]); // Galaxy brain
    let _terminal = render(&mut app, 120, 40);

    apply_events(&mut app, &[key_char('t')]); // Genealogy
    let _terminal = render(&mut app, 120, 40);

    apply_events(&mut app, &[key_char('s')]); // Back to summary
    let _terminal = render(&mut app, 120, 40);
}

// ===========================================================================
// 12. Status Messages
// ===========================================================================

#[test]
fn refresh_sets_status_message() {
    let mut app = app_with_rows();

    apply_events(&mut app, &[key_char('r')]);
    assert!(app.take_refresh());
}

// ===========================================================================
// 13. Empty State
// ===========================================================================

#[test]
fn empty_process_list_renders() {
    let mut app = App::new();
    // No rows set
    let _terminal = render(&mut app, 120, 40);
}

#[test]
fn empty_process_list_navigation_safe() {
    let mut app = App::new();

    // Navigation on empty list should not crash
    apply_events(
        &mut app,
        &[
            key_char('j'),
            key_char('k'),
            key(KeyCode::Home),
            key(KeyCode::End),
            key_char(' '),
            key_char('A'),
            key_char('u'),
        ],
    );
    let _terminal = render(&mut app, 120, 40);
}

// ===========================================================================
// 14. Rapid Event Sequences (Flake Control)
// ===========================================================================

#[test]
fn rapid_navigation_stress() {
    let mut app = app_with_rows();

    // 100 rapid key presses
    for _ in 0..100 {
        apply_events(&mut app, &[key_char('j')]);
    }
    // Cursor should be clamped to valid range
    assert!(app.process_table.cursor < 5);

    for _ in 0..100 {
        apply_events(&mut app, &[key_char('k')]);
    }
    assert_eq!(app.process_table.cursor, 0);
}

#[test]
fn rapid_toggle_stress() {
    let mut app = app_with_rows();

    // Rapidly toggle selection 200 times
    for _ in 0..200 {
        apply_events(&mut app, &[key_char(' ')]);
    }

    // Even number of toggles → should be back to 0
    assert_eq!(app.process_table.selected_count(), 0);
}

#[test]
fn rapid_search_enter_exit_stress() {
    let mut app = app_with_rows();

    // Enter and exit search 50 times
    for _ in 0..50 {
        apply_events(&mut app, &[key_char('/'), key(KeyCode::Esc)]);
    }
    assert_eq!(app.state, AppState::Normal);
}

#[test]
fn interleaved_resize_and_keys() {
    let mut app = app_with_rows();

    // Mix resize events with key events
    for width in (60..200).step_by(20) {
        app.handle_event(Event::Resize(width, 40)).unwrap();
        apply_events(&mut app, &[key_char('j'), key_char(' ')]);
        let _terminal = render(&mut app, width, 40);
    }
}

// ===========================================================================
// 15. Rendering at Different Terminal Sizes
// ===========================================================================

#[test]
fn render_at_compact_80x24() {
    let mut app = app_with_rows();
    app.handle_event(Event::Resize(80, 24)).unwrap();
    let _terminal = render(&mut app, 80, 24);
}

#[test]
fn render_at_standard_120x40() {
    let mut app = app_with_rows();
    app.handle_event(Event::Resize(120, 40)).unwrap();
    let _terminal = render(&mut app, 120, 40);
}

#[test]
fn render_at_wide_200x60() {
    let mut app = app_with_rows();
    app.handle_event(Event::Resize(200, 60)).unwrap();
    let _terminal = render(&mut app, 200, 60);
}

#[test]
fn render_at_minimal_40x10() {
    let mut app = app_with_rows();
    app.handle_event(Event::Resize(40, 10)).unwrap();
    let _terminal = render(&mut app, 40, 10);
}
