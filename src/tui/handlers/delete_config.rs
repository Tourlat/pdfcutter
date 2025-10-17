use crate::tui::app::{App, CurrentScreen};
use crate::tui::utils::validate_page_ranges;
use crossterm::event::KeyCode;

/**
 * Handle input in the delete configuration screen.
 * Allows editing output filename, specifying pages to delete, and starting the deletion.
 * @param key The key event.
 * @param app The application state.
 *
 */
pub fn handle_delete_config_input(key: KeyCode, app: &mut App) {
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
pub fn perform_delete(app: &mut App) {
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
