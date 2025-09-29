pub mod app;
pub mod ui;

use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::io;

use app::{App, CurrentScreen};

use crate::tui::app::OperationMode;

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
                CurrentScreen::FileSelection => handle_file_selection_input(key.code, app),
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
fn handle_file_selection_input(key: KeyCode, app: &mut App) {
    if app.error_message.is_some() && key != KeyCode::Esc {
        // Clear error on any key except Esc
        app.error_message = None;
        return;
    }

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
                if !input.is_empty() && std::path::Path::new(input).exists() {
                    app.selected_files.push(input.clone());
                    app.current_input = Some(String::new());
                } else {
                    app.set_error("File not found or invalid path".to_string());
                }
            }
        }
        // Delete selected path
        KeyCode::Left | KeyCode::Delete => {
            if app.selected_file_index < app.selected_files.len() {
                app.selected_files.remove(app.selected_file_index);
                if app.selected_file_index > 0 {
                    app.selected_file_index -= 1;
                }
            }
        }
        // Go to next screen
        KeyCode::Right => {
            if !app.selected_files.is_empty() {
                if matches!(app.operation_mode, OperationMode::Merge)
                    && app.selected_files.len() < 2
                {
                    app.set_error("Please select at least 2 files to merge".to_string());
                    return;
                }
                if matches!(app.operation_mode, OperationMode::Delete)
                    && app.selected_files.len() > 1
                {
                    app.set_error("Please select only 1 file to delete pages from".to_string());
                    return;
                }
                app.current_screen = match app.operation_mode {
                    OperationMode::Merge => CurrentScreen::MergeConfig,
                    OperationMode::Delete => CurrentScreen::DeleteConfig,
                    _ => CurrentScreen::Main,
                };
            } else {
                app.set_error("Please select at least one file".to_string());
            }
        }
        // Nav in paths list
        KeyCode::Up => {
            if app.selected_file_index > 0 {
                app.selected_file_index -= 1;
            }
        }
        // Nav in paths list
        KeyCode::Down => {
            if app.selected_file_index < app.selected_files.len().saturating_sub(1) {
                app.selected_file_index += 1;
            }
        }

        KeyCode::Esc => {
            app.current_screen = CurrentScreen::Main;
        }
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
                    if let Err(e) = validate_page_ranges(&app.pages_to_delete) {
                        app.set_error(format!("Invalid page format: {}", e));
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

                // Ajouter .pdf si pas déjà présent
                if !app.output_filename.ends_with(".pdf") && !app.output_filename.is_empty() {
                    app.output_filename.push_str(".pdf");
                }

                // Nom par défaut si vide
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
                        app.set_error(format!("Invalid page format: {}", e));
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
 * Validate and parse a string of page ranges into a vector of page numbers.
 * Supports formats like "1,3,5-7".
 * @param pages_str The input string specifying pages/ranges.
 * @returns A Result containing a vector of unique page numbers or an error message.
 */
fn validate_page_ranges(pages_str: &str) -> Result<Vec<u32>, String> {
    let mut pages = Vec::new();

    for part in pages_str.split(',') {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }

        if part.contains('-') {
            let range_parts: Vec<&str> = part.split('-').collect();
            if range_parts.len() != 2 {
                return Err(format!("Invalid range format: '{}'", part));
            }

            let start: u32 = range_parts[0]
                .trim()
                .parse()
                .map_err(|_| format!("Invalid page number: '{}'", range_parts[0]))?;
            let end: u32 = range_parts[1]
                .trim()
                .parse()
                .map_err(|_| format!("Invalid page number: '{}'", range_parts[1]))?;

            if start > end {
                return Err(format!("Invalid range: {} > {}", start, end));
            }

            if start == 0 || end == 0 {
                return Err("Page numbers must be greater than 0".to_string());
            }

            for page in start..=end {
                pages.push(page);
            }
        } else {
            let page: u32 = part
                .parse()
                .map_err(|_| format!("Invalid page number: '{}'", part))?;

            if page == 0 {
                return Err("Page numbers must be greater than 0".to_string());
            }

            pages.push(page);
        }
    }

    if pages.is_empty() {
        return Err("No valid pages specified".to_string());
    }

    pages.sort();
    pages.dedup();
    Ok(pages)
}

/**
 * Perform the PDF page deletion operation using the selected file, pages to delete, and output filename.
 * Updates the app state with success or error messages.
 * @param app The application state.
 * @returns Nothing. Updates app state directly.
 */
fn perform_delete(app: &mut App) {
    use crate::pdf;

    if let Ok(pages_to_delete) = validate_page_ranges(&app.pages_to_delete) {
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
        // Clear error on any key except Esc
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
            KeyCode::Enter => {
                if !app.output_filename.is_empty() {
                    app.editing_output = false;
                    if !app.output_filename.ends_with(".pdf") && !app.output_filename.is_empty() {
                        app.output_filename.push_str(".pdf");
                    }
                } else {
                    app.set_error("Output filename cannot be empty".to_string());
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
        KeyCode::Enter => {
            // Lancer le merge
            if app.selected_files.len() < 2 {
                app.set_error("Need at least 2 files to merge".to_string());
            } else if app.output_filename.is_empty() {
                app.set_error("Output filename cannot be empty".to_string());
            } else {
                perform_merge(app);
            }
        }
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
    use std::path::Path;

    #[test]
    fn test_validate_page_ranges() {
        // Valid cases
        assert_eq!(validate_page_ranges("1,3,5-7").unwrap(), vec![1, 3, 5, 6, 7]);
        assert_eq!(validate_page_ranges("2-4,6").unwrap(), vec![2, 3, 4, 6]);
        assert_eq!(validate_page_ranges("10").unwrap(), vec![10]);
        assert_eq!(validate_page_ranges("1-3,5,7-9").unwrap(), vec![1, 2, 3, 5, 7, 8, 9]);
        assert_eq!(validate_page_ranges(" 1 , 2 - 3 , 5 ").unwrap(), vec![1, 2, 3, 5]);

        // Invalid cases
        assert!(validate_page_ranges("3-1").is_err()); // Invalid range
        assert!(validate_page_ranges("a,b,c").is_err()); // Non-numeric
        assert!(validate_page_ranges("-3").is_err()); // Invalid range format
        assert!(validate_page_ranges("0,2-4").is_err()); // Page number zero
        assert!(validate_page_ranges("1-").is_err()); // Incomplete range
        assert!(validate_page_ranges("").is_err()); // Empty input
        
        // Test with empty parts (should be handled gracefully)
        assert_eq!(validate_page_ranges("1,,3").unwrap(), vec![1, 3]); // Empty parts ignored
    }

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

        // Test typing characters
        handle_file_selection_input(KeyCode::Char('t'), &mut app);
        handle_file_selection_input(KeyCode::Char('e'), &mut app);
        handle_file_selection_input(KeyCode::Char('s'), &mut app);
        handle_file_selection_input(KeyCode::Char('t'), &mut app);
        assert_eq!(app.current_input.as_deref(), Some("test"));

        // Test backspace
        handle_file_selection_input(KeyCode::Backspace, &mut app);
        assert_eq!(app.current_input.as_deref(), Some("tes"));
        
        // Test with invalid file
        handle_file_selection_input(KeyCode::Enter, &mut app);
        assert!(app.error_message.is_some());

        // Test with valid file (README.md should exist)
        app.error_message = None;
        app.current_input = Some("README.md".to_string());
        handle_file_selection_input(KeyCode::Enter, &mut app);
        
        if Path::new("README.md").exists() {
            assert_eq!(app.selected_files.len(), 1);
            assert_eq!(app.selected_files[0], "README.md");
            assert!(app.error_message.is_none());
        } else {
            assert!(app.error_message.is_some());
        }

        // Test file removal
        if !app.selected_files.is_empty() {
            app.selected_file_index = 0;
            handle_file_selection_input(KeyCode::Left, &mut app);
            assert_eq!(app.selected_files.len(), 0);
        }

        // Test navigation to next screen with insufficient files for merge
        app.selected_files.push("file1.pdf".to_string());
        handle_file_selection_input(KeyCode::Right, &mut app);
        assert!(app.error_message.is_some()); // Not enough files for merge

        // Test with enough files for merge
        app.error_message = None;
        app.selected_files.push("file2.pdf".to_string());
        handle_file_selection_input(KeyCode::Right, &mut app);
        assert_eq!(app.current_screen, CurrentScreen::MergeConfig);
        
        // Test delete mode validation
        app.reset();
        app.operation_mode = OperationMode::Delete;
        app.selected_files.push("file1.pdf".to_string());
        app.selected_files.push("file2.pdf".to_string());
        handle_file_selection_input(KeyCode::Right, &mut app);
        assert!(app.error_message.is_some()); // Too many files for delete
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

        // Test exiting edit mode
        handle_merge_config_input(KeyCode::Enter, &mut app);
        assert!(!app.editing_output);
        assert_eq!(app.output_filename, "out.pdf"); // Should auto-add .pdf

        // Test file reordering
        app.merge_file_index = 0;
        handle_merge_config_input(KeyCode::Down, &mut app);
        assert_eq!(app.merge_file_index, 1);
        assert_eq!(app.selected_files[0], "file2.pdf");
        assert_eq!(app.selected_files[1], "file1.pdf");

        // Test merge execution with valid config
        handle_merge_config_input(KeyCode::Enter, &mut app);
        // Should attempt merge and set error or success message
        assert!(app.error_message.is_some() || app.success_message.is_some());
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
        handle_file_selection_input(KeyCode::Char('a'), &mut app);
        assert!(app.error_message.is_none());

        // Set error again and test Esc doesn't clear it in this context
        app.set_error("Test error".to_string());
        handle_file_selection_input(KeyCode::Esc, &mut app);
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
        handle_file_selection_input(KeyCode::Up, &mut app);
        assert_eq!(app.selected_file_index, 1);

        handle_file_selection_input(KeyCode::Up, &mut app);
        assert_eq!(app.selected_file_index, 0);

        // Test down navigation
        handle_file_selection_input(KeyCode::Down, &mut app);
        assert_eq!(app.selected_file_index, 1);

        handle_file_selection_input(KeyCode::Down, &mut app);
        assert_eq!(app.selected_file_index, 2);

        // Test boundary conditions
        handle_file_selection_input(KeyCode::Down, &mut app);
        assert_eq!(app.selected_file_index, 2); // Should stay at max

        app.selected_file_index = 0;
        handle_file_selection_input(KeyCode::Up, &mut app);
        assert_eq!(app.selected_file_index, 0); // Should stay at min
    }
}

