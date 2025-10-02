pub mod app;
pub mod errors;
pub mod ui;
pub mod utils;

use anyhow::Result;
use app::{App, CurrentScreen};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::io;

use crate::tui::app::OperationMode;
use utils::*;

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

fn handle_main_input(key: KeyCode, app: &mut App) {
    match key {
        KeyCode::Char('q') | KeyCode::Char('Q') => {
            app.current_screen = CurrentScreen::Exiting;
        }
        KeyCode::Char('1') => {
            app.reset();
            app.menu_mode_index = 0;
            app.operation_mode = app::OperationMode::Merge;
            app.current_screen = CurrentScreen::FileSelection;
        }
        KeyCode::Char('2') => {
            app.reset();
            app.menu_mode_index = 1;
            app.operation_mode = app::OperationMode::Delete;
            app.current_screen = CurrentScreen::FileSelection;
        }
        KeyCode::Char('3') => {
            app.menu_mode_index = 2;
            app.current_screen = CurrentScreen::Help;
        }
        KeyCode::Up => {
            if app.menu_mode_index > 0 {
                app.menu_mode_index -= 1;
            } else {
                app.menu_mode_index = 3;
            }
        }
        KeyCode::Down => {
            if app.menu_mode_index < 3 {
                app.menu_mode_index += 1;
            } else {
                app.menu_mode_index = 0;
            }
        }
        KeyCode::Enter => match app.menu_mode_index {
            0 => {
                app.reset();
                app.operation_mode = OperationMode::Merge;
                app.current_screen = CurrentScreen::FileSelection;
            }
            1 => {
                app.reset();
                app.operation_mode = OperationMode::Delete;
                app.current_screen = CurrentScreen::FileSelection;
            }
            2 => {
                app.current_screen = CurrentScreen::Help;
            }
            3 => {
                app.current_screen = CurrentScreen::Exiting;
            }
            _ => {}
        },
        KeyCode::Esc => {
            app.current_screen = CurrentScreen::Exiting;
        }
        _ => {}
    }
}

/**
 * Handle input in the file selection screen.
 * Allows adding/removing files, navigating the list, and proceeding to the next configuration screen.
 * @param key The key event.
 * @param app The application state.
 */
fn handle_file_selection_input(key: KeyCode, key_event_modifier: KeyModifiers, app: &mut App) {
    if app.error_message.is_some() && key != KeyCode::Esc {
        app.error_message = None;
        return;
    }

    if app.editing_input {
        match key {
            KeyCode::Char(c) => {
                if let Some(ref mut input) = app.current_input {
                    input.push(c);
                } else {
                    app.current_input = Some(c.to_string());
                }
            }
            KeyCode::Backspace => {
                if let Some(ref mut input) = app.current_input {
                    input.pop();
                }
            }
            KeyCode::Enter => {
                if let Some(ref input) = app.current_input {
                    if !input.is_empty() {
                        match validate_file_input(input) {
                            Ok(()) => {
                                app.selected_files.push(input.clone());
                                app.current_input = Some(String::new());
                                app.error_message = None;
                                app.editing_input = false;
                            }
                            Err(e) => {
                                app.set_error(e.to_string());
                                app.editing_input = false;
                            }
                        }
                    } else {
                        app.editing_input = false;
                    }
                }
            }
            KeyCode::Esc => {
                app.editing_input = false;
                app.current_input = Some(String::new());
            }
            _ => {}
        }
        return;
    }

    match (key, key_event_modifier) {
        (KeyCode::Up, KeyModifiers::ALT) => {
            if app.selected_file_index > 0 {
                app.selected_files
                    .swap(app.selected_file_index, app.selected_file_index - 1);
                app.selected_file_index -= 1;
            }
        }
        (KeyCode::Down, KeyModifiers::ALT) => {
            if app.selected_file_index < app.selected_files.len().saturating_sub(1) {
                app.selected_files
                    .swap(app.selected_file_index, app.selected_file_index + 1);
                app.selected_file_index += 1;
            }
        }
        (key, KeyModifiers::NONE) | (key, KeyModifiers::SHIFT) => match key {
            KeyCode::Up => {
                if app.selected_file_index > 0 {
                    app.selected_file_index -= 1;
                }
            }
            KeyCode::Down => {
                if app.selected_file_index < app.selected_files.len().saturating_sub(1) {
                    app.selected_file_index += 1;
                }
            }

            KeyCode::Tab => {
                app.editing_input = true;
                app.current_input = Some(String::new());
            }

            KeyCode::Enter | KeyCode::Right => {
                let validation_result = match app.operation_mode {
                    OperationMode::Merge => validate_merge_requirements(&app.selected_files),
                    OperationMode::Delete => validate_delete_requirements(&app.selected_files),
                    _ => Ok(()),
                };

                match validation_result {
                    Ok(()) => {
                        app.current_screen = match app.operation_mode {
                            OperationMode::Merge => CurrentScreen::MergeConfig,
                            OperationMode::Delete => CurrentScreen::DeleteConfig,
                            _ => CurrentScreen::Main,
                        };
                        app.error_message = None;
                    }
                    Err(e) => {
                        app.set_error(e.to_string());
                    }
                }
            }

            KeyCode::Backspace => {
                if !app.selected_files.is_empty()
                    && app.selected_file_index < app.selected_files.len()
                {
                    app.selected_files.remove(app.selected_file_index);
                    if app.selected_file_index > 0
                        && app.selected_file_index >= app.selected_files.len()
                    {
                        app.selected_file_index = app.selected_files.len().saturating_sub(1);
                    }
                }
            }

            KeyCode::Esc => {
                app.current_screen = CurrentScreen::Main;
            }

            _ => {}
        },
        _ => {}
    }
}
/**
 * Handle input in the delete configuration screen.
 * Allows editing output filename, specifying pages to delete, and starting the deletion.
 * @param key The key event.
 * @param app The application state.
 *
 */
fn handle_delete_config_input(key: KeyCode, app: &mut App) {
    if app.error_message.is_some() && key != KeyCode::Esc {
        app.error_message = None;
        return;
    }

    if app.editing_pages {
        match key {
            KeyCode::Char(c) => {
                app.pages_to_delete.push(c);
            }
            KeyCode::Backspace => {
                app.pages_to_delete.pop();
            }
            KeyCode::Enter | KeyCode::Tab => {
                app.editing_pages = false;

                if !app.pages_to_delete.is_empty() {
                    match validate_page_ranges(&app.pages_to_delete) {
                        Ok(_) => {
                            app.error_message = None;
                        }
                        Err(e) => {
                            app.set_error(e.to_string());
                        }
                    }
                }
            }
            KeyCode::Esc => {
                app.editing_pages = false;
            }
            _ => {}
        }
        return;
    }

    if app.editing_output {
        match key {
            KeyCode::Char(c) => {
                app.output_filename.push(c);
            }
            KeyCode::Backspace => {
                app.output_filename.pop();
            }
            KeyCode::Enter | KeyCode::Tab => {
                app.editing_output = false;

                if !app.output_filename.ends_with(".pdf") && !app.output_filename.is_empty() {
                    app.output_filename.push_str(".pdf");
                }

                if app.output_filename.is_empty() {
                    app.output_filename = "output_deleted_pages.pdf".to_string();
                }
            }
            KeyCode::Esc => {
                app.editing_output = false;
            }
            _ => {}
        }
        return;
    }

    match key {
        KeyCode::Char('p') | KeyCode::Char('P') => {
            app.editing_pages = true;
        }

        KeyCode::Tab => {
            app.editing_output = true;
        }

        KeyCode::Enter => {
            if app.selected_files.is_empty() {
                app.set_error("No file selected".to_string());
            } else if app.pages_to_delete.is_empty() {
                app.set_error("Please specify pages to delete".to_string());
            } else if app.output_filename.is_empty() {
                app.set_error("Output filename cannot be empty".to_string());
            } else {
                match validate_page_ranges(&app.pages_to_delete) {
                    Ok(_) => {
                        perform_delete(app);
                    }
                    Err(e) => {
                        app.set_error(e.to_string());
                    }
                }
            }
        }

        KeyCode::Esc => {
            app.current_screen = CurrentScreen::FileSelection;
        }

        _ => {}
    }
}

/**
 * Perform the PDF page deletion operation using the selected file, pages to delete, and output filename.
 * Updates the app state with success or error messages.
 * @param app The application state.
 * @returns Nothing. Updates app state directly.
 */
fn perform_delete(app: &mut App) {
    use crate::pdf;

    match validate_page_ranges(&app.pages_to_delete) {
        Ok(pages_to_delete) => {
            match pdf::delete_pages(
                &app.selected_files[0],
                &app.output_filename,
                &pages_to_delete,
            ) {
                Ok(()) => {
                    app.set_success(format!(
                        "✅ Successfully deleted pages {} from '{}' and saved to '{}'",
                        app.pages_to_delete, app.selected_files[0], app.output_filename
                    ));
                    app.current_screen = CurrentScreen::Result;
                }
                Err(e) => {
                    app.set_error(format!("Failed to delete pages: {}", e));
                    app.current_screen = CurrentScreen::Result;
                }
            }
        }
        Err(e) => {
            app.set_error(e.to_string());
            app.current_screen = CurrentScreen::Result;
        }
    }
}

/**
 * Handle input in the merge configuration screen.
 * Allows editing output filename, reordering files, and starting the merge.
 * @param key The key event.
 * @param app The application state.
 *
 */
fn handle_merge_config_input(key: KeyCode, app: &mut App) {
    if app.error_message.is_some() && key != KeyCode::Esc {
        app.error_message = None;
        return;
    }

    if app.editing_output {
        match key {
            KeyCode::Char(c) => {
                app.output_filename.push(c);
            }
            KeyCode::Backspace => {
                app.output_filename.pop();
            }
            KeyCode::Enter | KeyCode::Tab => {
                app.editing_output = false;

                if !app.output_filename.ends_with(".pdf") && !app.output_filename.is_empty() {
                    app.output_filename.push_str(".pdf");
                }

                if app.output_filename.is_empty() {
                    app.output_filename = "output_merged.pdf".to_string();
                }
            }
            KeyCode::Esc => {
                app.editing_output = false;
            }
            _ => {}
        }
        return;
    }

    match key {
        KeyCode::Tab => {
            app.editing_output = true;
        }
        KeyCode::Up => {
            if app.merge_file_index > 0 {
                app.merge_file_index -= 1;
                app.selected_files
                    .swap(app.merge_file_index, app.merge_file_index + 1);
            }
        }
        KeyCode::Down => {
            if app.merge_file_index < app.selected_files.len().saturating_sub(1) {
                app.selected_files
                    .swap(app.merge_file_index, app.merge_file_index + 1);
                app.merge_file_index += 1;
            }
        }
        KeyCode::Enter => match validate_merge_requirements(&app.selected_files) {
            Ok(()) => {
                if app.output_filename.is_empty() {
                    app.set_error("Output filename cannot be empty".to_string());
                } else {
                    perform_merge(app);
                }
            }
            Err(e) => {
                app.set_error(e.to_string());
            }
        },
        KeyCode::Esc => {
            app.current_screen = CurrentScreen::FileSelection;
        }
        _ => {}
    }
}

/**
 * Perform the PDF merge operation using the selected files and output filename.
 * Updates the app state with success or error messages.
 * @param app The application state.
 * @retiurns Nothing. Updates app state directly.
 */
fn perform_merge(app: &mut App) {
    use crate::pdf;

    match pdf::merge_pdfs(&app.selected_files, &app.output_filename) {
        Ok(()) => {
            app.set_success(format!(
                "✅ Successfully merged {} files into '{}'",
                app.selected_files.len(),
                app.output_filename
            ));
            app.current_screen = CurrentScreen::Result;
        }
        Err(e) => {
            app.set_error(format!("Failed to merge PDFs: {}", e));
            app.current_screen = CurrentScreen::Result;
        }
    }
}

/**
 * Handle input in the result screen.
 * Shows success/error messages and allows returning to main menu.
 * @param key The key event.
 * @param app The application state.
 */
fn handle_result_input(key: KeyCode, app: &mut App) {
    match key {
        KeyCode::Esc | KeyCode::Enter | KeyCode::Char(' ') => {
            app.current_screen = CurrentScreen::Main;
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_main_input() {
        let mut app = App::new();

        // Test navigation
        assert_eq!(app.menu_mode_index, 0);
        handle_main_input(KeyCode::Down, &mut app);
        assert_eq!(app.menu_mode_index, 1);

        handle_main_input(KeyCode::Up, &mut app);
        assert_eq!(app.menu_mode_index, 0);

        // Test wrapping
        handle_main_input(KeyCode::Up, &mut app);
        assert_eq!(app.menu_mode_index, 3);

        // Test entering merge mode
        app.menu_mode_index = 0;
        handle_main_input(KeyCode::Enter, &mut app);
        assert_eq!(app.operation_mode, OperationMode::Merge);
        assert_eq!(app.current_screen, CurrentScreen::FileSelection);

        // Test entering delete mode
        app.reset();
        app.menu_mode_index = 1;
        handle_main_input(KeyCode::Enter, &mut app);
        assert_eq!(app.operation_mode, OperationMode::Delete);
        assert_eq!(app.current_screen, CurrentScreen::FileSelection);

        // Test help screen
        app.reset();
        app.menu_mode_index = 2;
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
        assert!(app.editing_input);

        // Test typing characters
        handle_file_selection_input(KeyCode::Char('t'), KeyModifiers::NONE, &mut app);
        handle_file_selection_input(KeyCode::Char('e'), KeyModifiers::NONE, &mut app);
        handle_file_selection_input(KeyCode::Char('s'), KeyModifiers::NONE, &mut app);
        handle_file_selection_input(KeyCode::Char('t'), KeyModifiers::NONE, &mut app);
        assert_eq!(app.current_input.as_deref(), Some("test"));

        // Test backspace
        handle_file_selection_input(KeyCode::Backspace, KeyModifiers::NONE, &mut app);
        assert_eq!(app.current_input.as_deref(), Some("tes"));

        // Test with invalid file (should set error and exit edit mode)
        handle_file_selection_input(KeyCode::Enter, KeyModifiers::NONE, &mut app);
        assert!(app.error_message.is_some());
        assert!(!app.editing_input); // Should exit edit mode even with error

        // Test with valid PDF file (if it exists)
        app.error_message = None;
        app.editing_input = true; // Re-enter edit mode
        if std::path::Path::new("tests/tests_pdf/a.pdf").exists() {
            app.current_input = Some("tests/tests_pdf/a.pdf".to_string());
            handle_file_selection_input(KeyCode::Enter, KeyModifiers::NONE, &mut app);
            assert_eq!(app.selected_files.len(), 1);
            assert_eq!(app.selected_files[0], "tests/tests_pdf/a.pdf");
            assert!(app.error_message.is_none());
            assert!(!app.editing_input); // Should exit edit mode after successful add
        } else {
            // If no test PDF exists, just add a mock PDF to test file removal
            app.selected_files.push("mock_file.pdf".to_string());
            app.editing_input = false; // Make sure we're not in edit mode
        }

        // Test file removal with Backspace (not Left)
        if !app.selected_files.is_empty() {
            app.selected_file_index = 0;
            handle_file_selection_input(KeyCode::Backspace, KeyModifiers::NONE, &mut app);
            assert_eq!(app.selected_files.len(), 0);
        }

        // Test navigation to next screen with insufficient files for merge
        app.selected_files.push("file1.pdf".to_string());
        handle_file_selection_input(KeyCode::Right, KeyModifiers::NONE, &mut app);
        assert!(app.error_message.is_some()); // Not enough files for merge

        // Test with enough files for merge
        app.error_message = None;
        app.selected_files.push("file2.pdf".to_string());
        handle_file_selection_input(KeyCode::Right, KeyModifiers::NONE, &mut app);
        assert_eq!(app.current_screen, CurrentScreen::MergeConfig);

        // Test delete mode validation
        app.reset();
        app.operation_mode = OperationMode::Delete;
        app.selected_files.push("file1.pdf".to_string());
        app.selected_files.push("file2.pdf".to_string());
        handle_file_selection_input(KeyCode::Right, KeyModifiers::NONE, &mut app);
        assert!(app.error_message.is_some()); // Too many files for delete
    }

    #[test]
    fn test_file_selection_editing_modes() {
        let mut app = App::new();
        app.operation_mode = OperationMode::Merge;

        // Test that editing_input starts as false
        assert!(!app.editing_input);

        // Test entering edit mode with Tab
        handle_file_selection_input(KeyCode::Tab, KeyModifiers::NONE, &mut app);
        assert!(app.editing_input);
        assert_eq!(app.current_input.as_deref(), Some(""));

        // Test canceling edit mode with Esc
        handle_file_selection_input(KeyCode::Esc, KeyModifiers::NONE, &mut app);
        assert!(!app.editing_input);
        assert_eq!(app.current_input.as_deref(), Some(""));

        // Test entering edit mode again and adding empty string (should exit edit mode)
        handle_file_selection_input(KeyCode::Tab, KeyModifiers::NONE, &mut app);
        assert!(app.editing_input);
        handle_file_selection_input(KeyCode::Enter, KeyModifiers::NONE, &mut app);
        assert!(!app.editing_input); // Should exit edit mode with empty input

        // Test navigation mode (when not editing)
        app.selected_files.push("file1.pdf".to_string());
        app.selected_files.push("file2.pdf".to_string());
        app.selected_file_index = 0;

        // Test up/down navigation
        handle_file_selection_input(KeyCode::Down, KeyModifiers::NONE, &mut app);
        assert_eq!(app.selected_file_index, 1);
        assert!(!app.editing_input); // Should still be in navigation mode

        handle_file_selection_input(KeyCode::Up, KeyModifiers::NONE, &mut app);
        assert_eq!(app.selected_file_index, 0);
        assert!(!app.editing_input); // Should still be in navigation mode

        // Test that Enter in navigation mode goes to next screen (with enough files)
        handle_file_selection_input(KeyCode::Enter, KeyModifiers::NONE, &mut app);
        assert_eq!(app.current_screen, CurrentScreen::MergeConfig);
    }

    #[test]
    fn test_file_reordering_with_alt() {
        let mut app = App::new();
        app.operation_mode = OperationMode::Merge;
        app.selected_files.push("file1.pdf".to_string());
        app.selected_files.push("file2.pdf".to_string());
        app.selected_files.push("file3.pdf".to_string());

        // Test moving file down with Alt+Down
        app.selected_file_index = 0;
        handle_file_selection_input(KeyCode::Down, KeyModifiers::ALT, &mut app);
        assert_eq!(app.selected_file_index, 1);
        assert_eq!(app.selected_files[0], "file2.pdf");
        assert_eq!(app.selected_files[1], "file1.pdf");
        assert_eq!(app.selected_files[2], "file3.pdf");

        // Test moving file up with Alt+Up
        handle_file_selection_input(KeyCode::Up, KeyModifiers::ALT, &mut app);
        assert_eq!(app.selected_file_index, 0);
        assert_eq!(app.selected_files[0], "file1.pdf");
        assert_eq!(app.selected_files[1], "file2.pdf");
        assert_eq!(app.selected_files[2], "file3.pdf");

        // Test boundary conditions - can't move up from index 0
        handle_file_selection_input(KeyCode::Up, KeyModifiers::ALT, &mut app);
        assert_eq!(app.selected_file_index, 0);
        assert_eq!(app.selected_files[0], "file1.pdf");

        // Test boundary conditions - can't move down from last index
        app.selected_file_index = 2;
        handle_file_selection_input(KeyCode::Down, KeyModifiers::ALT, &mut app);
        assert_eq!(app.selected_file_index, 2);
        assert_eq!(app.selected_files[2], "file3.pdf");
    }
    #[test]
    fn test_handle_merge_config_input() {
        let mut app = App::new();
        app.operation_mode = OperationMode::Merge;
        app.selected_files.push("file1.pdf".to_string());
        app.selected_files.push("file2.pdf".to_string());

        // Test entering edit mode
        handle_merge_config_input(KeyCode::Tab, &mut app);
        assert!(app.editing_output);

        // Test typing in edit mode
        handle_merge_config_input(KeyCode::Char('o'), &mut app);
        handle_merge_config_input(KeyCode::Char('u'), &mut app);
        handle_merge_config_input(KeyCode::Char('t'), &mut app);
        assert_eq!(app.output_filename, "out");

        // Test exiting edit mode with Enter (should trigger validation)
        handle_merge_config_input(KeyCode::Enter, &mut app);
        assert!(!app.editing_output);

        // Test file reordering
        app.editing_output = false; // Make sure we're not in edit mode
        app.merge_file_index = 0;
        handle_merge_config_input(KeyCode::Down, &mut app);
        assert_eq!(app.merge_file_index, 1);
        assert_eq!(app.selected_files[0], "file2.pdf");
        assert_eq!(app.selected_files[1], "file1.pdf");

        // Test merge execution with valid config
        app.output_filename = "valid_output.pdf".to_string();
        handle_merge_config_input(KeyCode::Enter, &mut app);
        // Should attempt merge and set error message (files don't exist)
        assert!(app.error_message.is_some());
    }

    #[test]
    fn test_handle_delete_config_input() {
        let mut app = App::new();
        app.operation_mode = OperationMode::Delete;
        app.selected_files.push("sample.pdf".to_string());

        // Test entering pages edit mode
        handle_delete_config_input(KeyCode::Char('p'), &mut app);
        assert!(app.editing_pages);

        // Test typing pages
        handle_delete_config_input(KeyCode::Char('1'), &mut app);
        handle_delete_config_input(KeyCode::Char(','), &mut app);
        handle_delete_config_input(KeyCode::Char('3'), &mut app);
        assert_eq!(app.pages_to_delete, "1,3");

        // Test exiting pages edit mode
        handle_delete_config_input(KeyCode::Enter, &mut app);
        assert!(!app.editing_pages);

        // Test entering output edit mode
        handle_delete_config_input(KeyCode::Tab, &mut app);
        assert!(app.editing_output);

        // Test typing output filename
        handle_delete_config_input(KeyCode::Char('o'), &mut app);
        handle_delete_config_input(KeyCode::Char('u'), &mut app);
        handle_delete_config_input(KeyCode::Char('t'), &mut app);
        assert_eq!(app.output_filename, "out");

        // Test exiting output edit mode
        handle_delete_config_input(KeyCode::Enter, &mut app);
        assert!(!app.editing_output);
        assert_eq!(app.output_filename, "out.pdf");

        // Test delete execution
        handle_delete_config_input(KeyCode::Enter, &mut app);
        // Should attempt delete and set error message (file doesn't exist)
        assert!(app.error_message.is_some());
    }

    #[test]
    fn test_handle_result_input() {
        let mut app = App::new();
        app.current_screen = CurrentScreen::Result;
        app.success_message = Some("Success!".to_string());

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

    #[test]
    fn test_perform_delete_with_mock_data() {
        let mut app = App::new();
        app.selected_files.push("nonexistent.pdf".to_string());
        app.pages_to_delete = "1,3-4".to_string();
        app.output_filename = "output.pdf".to_string();

        perform_delete(&mut app);

        // Should fail because file doesn't exist
        assert!(app.error_message.is_some());
        assert!(app.success_message.is_none());
    }

    #[test]
    fn test_perform_merge_with_mock_data() {
        let mut app = App::new();
        app.selected_files.push("nonexistent1.pdf".to_string());
        app.selected_files.push("nonexistent2.pdf".to_string());
        app.output_filename = "output.pdf".to_string();

        perform_merge(&mut app);

        // Should fail because files don't exist
        assert!(app.error_message.is_some());
        assert!(app.success_message.is_none());
    }

    #[test]
    fn test_app_reset() {
        let mut app = App::new();

        // Populate with some data
        app.selected_files.push("test.pdf".to_string());
        app.output_filename = "output.pdf".to_string();
        app.pages_to_delete = "1,2,3".to_string();
        app.error_message = Some("Error".to_string());
        app.success_message = Some("Success".to_string());
        app.operation_mode = OperationMode::Merge;
        app.current_screen = CurrentScreen::FileSelection;
        app.editing_output = true;
        app.editing_pages = true;
        app.menu_mode_index = 2;

        // Reset
        app.reset();

        // Verify everything is reset
        assert!(app.selected_files.is_empty());
        assert!(app.output_filename.is_empty());
        assert!(app.pages_to_delete.is_empty());
        assert!(app.error_message.is_none());
        assert!(app.success_message.is_none());
        assert_eq!(app.operation_mode, OperationMode::None);
        assert_eq!(app.current_screen, CurrentScreen::Main);
        assert!(!app.editing_output);
        assert!(!app.editing_pages);
        assert_eq!(app.menu_mode_index, 0);
    }

    #[test]
    fn test_error_handling_in_file_selection() {
        let mut app = App::new();
        app.set_error("Test error".to_string());

        // Any key except Esc should clear the error
        handle_file_selection_input(KeyCode::Char('a'), KeyModifiers::NONE, &mut app);
        assert!(app.error_message.is_none());

        // Set error again and test Esc doesn't clear it in this context
        app.set_error("Test error".to_string());
        handle_file_selection_input(KeyCode::Esc, KeyModifiers::NONE, &mut app);
        assert_eq!(app.current_screen, CurrentScreen::Main);
    }

    #[test]
    fn test_file_navigation() {
        let mut app = App::new();
        app.selected_files.push("file1.pdf".to_string());
        app.selected_files.push("file2.pdf".to_string());
        app.selected_files.push("file3.pdf".to_string());

        // Test up navigation
        app.selected_file_index = 2;
        handle_file_selection_input(KeyCode::Up, KeyModifiers::NONE, &mut app);
        assert_eq!(app.selected_file_index, 1);

        handle_file_selection_input(KeyCode::Up, KeyModifiers::NONE, &mut app);
        assert_eq!(app.selected_file_index, 0);

        // Test down navigation
        handle_file_selection_input(KeyCode::Down, KeyModifiers::NONE, &mut app);
        assert_eq!(app.selected_file_index, 1);

        handle_file_selection_input(KeyCode::Down, KeyModifiers::NONE, &mut app);
        assert_eq!(app.selected_file_index, 2);

        // Test boundary conditions
        handle_file_selection_input(KeyCode::Down, KeyModifiers::NONE, &mut app);
        assert_eq!(app.selected_file_index, 2); // Should stay at max

        app.selected_file_index = 0;
        handle_file_selection_input(KeyCode::Up, KeyModifiers::NONE, &mut app);
        assert_eq!(app.selected_file_index, 0); // Should stay at min
    }

    #[test]
    fn test_file_reordering_with_ctrl() {
        let mut app = App::new();
        app.selected_files.push("file1.pdf".to_string());
        app.selected_files.push("file2.pdf".to_string());
        app.selected_files.push("file3.pdf".to_string());

        // Test moving file down with Ctrl+Down
        app.selected_file_index = 0;
        handle_file_selection_input(KeyCode::Down, KeyModifiers::ALT, &mut app);
        assert_eq!(app.selected_file_index, 1);
        assert_eq!(app.selected_files[0], "file2.pdf");
        assert_eq!(app.selected_files[1], "file1.pdf");
        assert_eq!(app.selected_files[2], "file3.pdf");

        // Test moving file up with Ctrl+Up
        handle_file_selection_input(KeyCode::Up, KeyModifiers::ALT, &mut app);
        assert_eq!(app.selected_file_index, 0);
        assert_eq!(app.selected_files[0], "file1.pdf");
        assert_eq!(app.selected_files[1], "file2.pdf");
        assert_eq!(app.selected_files[2], "file3.pdf");

        // Test boundary conditions - can't move up from index 0
        handle_file_selection_input(KeyCode::Up, KeyModifiers::ALT, &mut app);
        assert_eq!(app.selected_file_index, 0);
        assert_eq!(app.selected_files[0], "file1.pdf");

        // Test boundary conditions - can't move down from last index
        app.selected_file_index = 2;
        handle_file_selection_input(KeyCode::Down, KeyModifiers::ALT, &mut app);
        assert_eq!(app.selected_file_index, 2);
        assert_eq!(app.selected_files[2], "file3.pdf");
    }

    #[test]
    fn test_file_validation_in_input() {
        let mut app = App::new();
        app.operation_mode = OperationMode::Merge;

        // Test with non-PDF file (README.md)
        if std::path::Path::new("README.md").exists() {
            app.editing_input = true;
            app.current_input = Some("README.md".to_string());
            handle_file_selection_input(KeyCode::Enter, KeyModifiers::NONE, &mut app);
            assert!(app.error_message.is_some());
            assert!(
                app.error_message
                    .as_ref()
                    .unwrap()
                    .contains("Invalid PDF file")
            );
            assert_eq!(app.selected_files.len(), 0);
            assert!(!app.editing_input);
        }

        // Test with non-existent file
        app.error_message = None;
        app.editing_input = true;
        app.current_input = Some("nonexistent.pdf".to_string());
        handle_file_selection_input(KeyCode::Enter, KeyModifiers::NONE, &mut app);
        assert!(app.error_message.is_some());
        assert!(
            app.error_message
                .as_ref()
                .unwrap()
                .contains("File not found")
        );
        assert!(!app.editing_input);

        // Test with empty input
        app.error_message = None;
        app.editing_input = true;
        app.current_input = Some("".to_string());
        handle_file_selection_input(KeyCode::Enter, KeyModifiers::NONE, &mut app);
        assert!(!app.editing_input);
    }

    #[test]
    fn test_validation_functions() {
        // Test validate_merge_requirements
        assert!(validate_merge_requirements(&vec!["file1.pdf".to_string()]).is_err());
        assert!(
            validate_merge_requirements(&vec!["file1.pdf".to_string(), "file2.pdf".to_string()])
                .is_ok()
        );

        // Test validate_delete_requirements
        assert!(validate_delete_requirements(&vec![]).is_err());
        assert!(validate_delete_requirements(&vec!["file1.pdf".to_string()]).is_ok());
        assert!(
            validate_delete_requirements(&vec!["file1.pdf".to_string(), "file2.pdf".to_string()])
                .is_err()
        );

        // Test validate_page_ranges (moved from utils tests)
        assert!(validate_page_ranges("1,3,5-7").is_ok());
        assert!(validate_page_ranges("3-1").is_err());
        assert!(validate_page_ranges("a,b,c").is_err());
        assert!(validate_page_ranges("0,2-4").is_err());
    }

    #[test]
    fn test_error_message_format() {
        let mut app = App::new();
        app.operation_mode = OperationMode::Merge;

        // Test specific error message format for insufficient files
        app.selected_files.push("file1.pdf".to_string());
        handle_file_selection_input(KeyCode::Right, KeyModifiers::NONE, &mut app);
        assert!(app.error_message.is_some());
        assert!(
            app.error_message
                .as_ref()
                .unwrap()
                .contains("Not enough files for merge")
        );

        // Test error message for too many files in delete mode
        app.reset();
        app.operation_mode = OperationMode::Delete;
        app.selected_files.push("file1.pdf".to_string());
        app.selected_files.push("file2.pdf".to_string());
        handle_file_selection_input(KeyCode::Right, KeyModifiers::NONE, &mut app);
        assert!(app.error_message.is_some());
        assert!(
            app.error_message
                .as_ref()
                .unwrap()
                .contains("Too many files for delete operation")
        );
    }

    #[test]
    fn test_page_validation_in_delete_config() {
        let mut app = App::new();
        app.operation_mode = OperationMode::Delete;
        app.selected_files.push("sample.pdf".to_string());

        // Test valid page ranges
        app.editing_pages = true;
        app.pages_to_delete = "1,3-5".to_string();
        handle_delete_config_input(KeyCode::Enter, &mut app);
        assert!(!app.editing_pages);
        assert!(app.error_message.is_none());

        // Test invalid page ranges
        app.editing_pages = true;
        app.pages_to_delete = "3-1".to_string(); // Invalid range
        handle_delete_config_input(KeyCode::Enter, &mut app);
        assert!(!app.editing_pages);
        assert!(app.error_message.is_some());
        assert!(
            app.error_message
                .as_ref()
                .unwrap()
                .contains("Invalid page range")
        );

        // Test invalid page numbers
        app.error_message = None;
        app.editing_pages = true;
        app.pages_to_delete = "0,1,2".to_string(); // Zero is invalid
        handle_delete_config_input(KeyCode::Enter, &mut app);
        assert!(app.error_message.is_some());
        assert!(
            app.error_message
                .as_ref()
                .unwrap()
                .contains("Page numbers must be greater than 0")
        );
    }
}
