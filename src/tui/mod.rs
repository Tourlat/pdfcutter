pub mod app;
pub mod errors;
pub mod handlers;
pub mod state;
pub mod ui;
pub mod utils;

use anyhow::Result;
use app::App;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use state::CurrentScreen;
use std::io;

use handlers::*;

pub fn run() -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app state
    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui::ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            match app.current_screen {
                CurrentScreen::Main => handle_main_input(key.code, app),
                CurrentScreen::Exiting => match key.code {
                    KeyCode::Char('y') | KeyCode::Char('Y') => return Ok(()),
                    KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
                        app.current_screen = CurrentScreen::Main;
                    }
                    _ => {}
                },
                CurrentScreen::FileSelection => {
                    handle_file_selection_input(key.code, key.modifiers, app)
                }
                CurrentScreen::MergeConfig => handle_merge_config_input(key.code, app),
                CurrentScreen::DeleteConfig => handle_delete_config_input(key.code, app),
                CurrentScreen::SplitConfig => handle_split_config_input(key.code, app),
                CurrentScreen::Result => handle_result_input(key.code, app),

                _ => {
                    if key.code == KeyCode::Esc {
                        app.current_screen = CurrentScreen::Main;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::KeyModifiers;
    use state::OperationMode;

    #[test]
    fn test_handle_main_input() {
        let mut app = App::new();

        // Test navigation
        assert_eq!(app.menu_mode_index(), 0);
        handle_main_input(KeyCode::Down, &mut app);
        assert_eq!(app.menu_mode_index(), 1);

        handle_main_input(KeyCode::Up, &mut app);
        assert_eq!(app.menu_mode_index(), 0);

        // Test wrapping
        handle_main_input(KeyCode::Up, &mut app);
        assert_eq!(app.menu_mode_index(), 4);

        // Test entering merge mode
        app.set_menu_mode_index(0);
        handle_main_input(KeyCode::Enter, &mut app);
        assert_eq!(app.operation_mode, OperationMode::Merge);
        assert_eq!(app.current_screen, CurrentScreen::FileSelection);

        // Test entering delete mode
        app.reset();
        app.set_menu_mode_index(1);
        handle_main_input(KeyCode::Enter, &mut app);
        assert_eq!(app.operation_mode, OperationMode::Delete);
        assert_eq!(app.current_screen, CurrentScreen::FileSelection);

        // Test help screen
        app.reset();
        app.set_menu_mode_index(3);
        handle_main_input(KeyCode::Enter, &mut app);
        assert_eq!(app.current_screen, CurrentScreen::Help);

        // Test exit
        app.reset();
        handle_main_input(KeyCode::Char('q'), &mut app);
        assert_eq!(app.current_screen, CurrentScreen::Exiting);
    }

    #[test]
    fn test_handle_file_selection_input() {
        let mut app = App::new();
        app.operation_mode = OperationMode::Merge;

        // Start editing input
        handle_file_selection_input(KeyCode::Tab, KeyModifiers::NONE, &mut app);
        assert!(app.editing_input());

        // Test typing characters
        handle_file_selection_input(KeyCode::Char('t'), KeyModifiers::NONE, &mut app);
        handle_file_selection_input(KeyCode::Char('e'), KeyModifiers::NONE, &mut app);
        handle_file_selection_input(KeyCode::Char('s'), KeyModifiers::NONE, &mut app);
        handle_file_selection_input(KeyCode::Char('t'), KeyModifiers::NONE, &mut app);
        assert_eq!(app.current_input(), Some("test"));

        // Test backspace
        handle_file_selection_input(KeyCode::Backspace, KeyModifiers::NONE, &mut app);
        assert_eq!(app.current_input(), Some("tes"));

        // Test with invalid file (should set error and exit edit mode)
        handle_file_selection_input(KeyCode::Enter, KeyModifiers::NONE, &mut app);
        assert!(app.error_message().is_some());
        assert!(!app.editing_input()); // Should exit edit mode even with error

        // Test with valid PDF file (if it exists)
        app.ui_state.clear_message();
        app.set_editing_input(true); // Re-enter edit mode
        if std::path::Path::new("tests/tests_pdf/a.pdf").exists() {
            app.set_current_input(Some("tests/tests_pdf/a.pdf".to_string()));
            handle_file_selection_input(KeyCode::Enter, KeyModifiers::NONE, &mut app);
            assert_eq!(app.selected_files().len(), 1);
            assert_eq!(app.selected_files()[0], "tests/tests_pdf/a.pdf");
            assert!(app.error_message().is_none());
            assert!(!app.editing_input()); // Should exit edit mode after successful add
        } else {
            // If no test PDF exists, just add a mock PDF to test file removal
            app.selected_files_mut().push("mock_file.pdf".to_string());
            app.set_editing_input(false); // Make sure we're not in edit mode
        }

        // Test file removal with Backspace (not Left)
        if !app.selected_files().is_empty() {
            app.set_selected_file_index(0);
            handle_file_selection_input(KeyCode::Backspace, KeyModifiers::NONE, &mut app);
            assert_eq!(app.selected_files().len(), 0);
        }

        // Test navigation to next screen with insufficient files for merge
        app.selected_files_mut().push("file1.pdf".to_string());
        handle_file_selection_input(KeyCode::Right, KeyModifiers::NONE, &mut app);
        assert!(app.error_message().is_some()); // Not enough files for merge

        // Test with enough files for merge
        app.ui_state.clear_message();
        app.selected_files_mut().push("file2.pdf".to_string());
        handle_file_selection_input(KeyCode::Right, KeyModifiers::NONE, &mut app);
        assert_eq!(app.current_screen, CurrentScreen::MergeConfig);

        app.reset();
        app.operation_mode = OperationMode::Delete;
        handle_file_selection_input(KeyCode::Right, KeyModifiers::NONE, &mut app);
        assert!(app.error_message().is_some()); // Not enough files for delete

        // Test delete mode validation
        app.reset();
        app.operation_mode = OperationMode::Delete;
        app.selected_files_mut().push("file1.pdf".to_string());
        app.selected_files_mut().push("file2.pdf".to_string());
        handle_file_selection_input(KeyCode::Right, KeyModifiers::NONE, &mut app);
        assert!(app.error_message().is_some()); // Too many files for delete
    }

    #[test]
    fn test_handle_merge_config_input() {
        let mut app = App::new();
        app.operation_mode = OperationMode::Merge;
        app.selected_files_mut().push("file1.pdf".to_string());
        app.selected_files_mut().push("file2.pdf".to_string());

        // Test entering edit mode
        handle_merge_config_input(KeyCode::Tab, &mut app);
        assert!(app.merge_config.editing_output);

        // Test typing in edit mode
        handle_merge_config_input(KeyCode::Char('o'), &mut app);
        handle_merge_config_input(KeyCode::Char('u'), &mut app);
        handle_merge_config_input(KeyCode::Char('t'), &mut app);
        assert_eq!(app.merge_config.output_filename, "out");

        // Test exiting edit mode with Enter (should trigger validation)
        handle_merge_config_input(KeyCode::Enter, &mut app);
        assert!(!app.merge_config.editing_output);

        // Test file reordering
        app.merge_config.editing_output = false; // Make sure we're not in edit mode
        app.set_merge_file_index(0);
        handle_merge_config_input(KeyCode::Down, &mut app);
        assert_eq!(app.merge_file_index(), 1);
        assert_eq!(app.selected_files()[0], "file2.pdf");
        assert_eq!(app.selected_files()[1], "file1.pdf");

        // Test merge execution with valid config
        app.merge_config.output_filename = "valid_output.pdf".to_string();
        handle_merge_config_input(KeyCode::Enter, &mut app);
        // Should attempt merge and set error message (files don't exist)
        assert!(app.error_message().is_some());
    }

    #[test]
    fn test_handle_delete_config_input() {
        let mut app = App::new();
        app.operation_mode = OperationMode::Delete;
        app.selected_files_mut().push("sample.pdf".to_string());

        // Test entering pages edit mode
        handle_delete_config_input(KeyCode::Char('p'), &mut app);
        assert!(app.delete_config.editing_pages);

        // Test typing pages
        handle_delete_config_input(KeyCode::Char('1'), &mut app);
        handle_delete_config_input(KeyCode::Char(','), &mut app);
        handle_delete_config_input(KeyCode::Char('3'), &mut app);
        assert_eq!(app.delete_config.pages_to_delete, "1,3");

        // Test exiting pages edit mode
        handle_delete_config_input(KeyCode::Enter, &mut app);
        assert!(!app.delete_config.editing_pages);

        // Test entering output edit mode
        handle_delete_config_input(KeyCode::Tab, &mut app);
        assert!(app.delete_config.editing_output);

        // Test typing output filename
        handle_delete_config_input(KeyCode::Char('o'), &mut app);
        handle_delete_config_input(KeyCode::Char('u'), &mut app);
        handle_delete_config_input(KeyCode::Char('t'), &mut app);
        assert_eq!(app.delete_config.output_filename, "out");

        // Test exiting output edit mode
        handle_delete_config_input(KeyCode::Enter, &mut app);
        assert!(!app.delete_config.editing_output);
        assert_eq!(app.delete_config.output_filename, "out.pdf");

        // Test delete execution
        handle_delete_config_input(KeyCode::Enter, &mut app);
        // Should attempt delete and set error message (file doesn't exist)
        assert!(app.error_message().is_some());
    }

    #[test]
    fn test_handle_result_input() {
        let mut app = App::new();
        app.current_screen = CurrentScreen::Result;
        app.set_success("Success!".to_string());

        // Test returning to main menu
        handle_result_input(KeyCode::Enter, &mut app);
        assert_eq!(app.current_screen, CurrentScreen::Main);

        // Test with Esc
        app.current_screen = CurrentScreen::Result;
        handle_result_input(KeyCode::Esc, &mut app);
        assert_eq!(app.current_screen, CurrentScreen::Main);

        // Test with Space
        app.current_screen = CurrentScreen::Result;
        handle_result_input(KeyCode::Char(' '), &mut app);
        assert_eq!(app.current_screen, CurrentScreen::Main);
    }
}
