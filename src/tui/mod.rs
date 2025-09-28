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
                CurrentScreen::Result => match key.code {
                    _ => app.current_screen = CurrentScreen::Main,
                },

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
    app.reset();
    match key {
        KeyCode::Char('q') | KeyCode::Char('Q') => {
            app.current_screen = CurrentScreen::Exiting;
        }
        KeyCode::Char('1') => {
            app.operation_mode = app::OperationMode::Merge;
            app.current_screen = CurrentScreen::FileSelection;
        }
        KeyCode::Char('2') => {
            app.operation_mode = app::OperationMode::Delete;
            app.current_screen = CurrentScreen::FileSelection;
        }
        KeyCode::Char('3') => {
            app.current_screen = CurrentScreen::Help;
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
