use crate::tui::app::App;
use crate::tui::state::CurrentScreen;
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
    if app.ui_state.get_error_message().is_some() && key != KeyCode::Esc {
        app.ui_state.clear_message();
        return;
    }

    if app.delete_config.editing_pages {
        match key {
            KeyCode::Char(c) => {
                app.delete_config.pages_to_delete.push(c);
            }
            KeyCode::Backspace => {
                app.delete_config.pages_to_delete.pop();
            }
            KeyCode::Enter | KeyCode::Tab => {
                app.delete_config.editing_pages = false;

                if !app.delete_config.pages_to_delete.is_empty() {
                    match validate_page_ranges(&app.delete_config.pages_to_delete) {
                        Ok(_) => {
                            app.ui_state.clear_message();
                        }
                        Err(e) => {
                            app.set_error(e.to_string());
                        }
                    }
                }
            }
            KeyCode::Esc => {
                app.delete_config.editing_pages = false;
            }
            _ => {}
        }
        return;
    }

    if app.delete_config.editing_output {
        match key {
            KeyCode::Char(c) => {
                app.delete_config.output_filename.push(c);
            }
            KeyCode::Backspace => {
                app.delete_config.output_filename.pop();
            }
            KeyCode::Enter | KeyCode::Tab => {
                app.delete_config.editing_output = false;

                if !app.delete_config.output_filename.ends_with(".pdf")
                    && !app.delete_config.output_filename.is_empty()
                {
                    app.delete_config.output_filename.push_str(".pdf");
                }

                if app.delete_config.output_filename.is_empty() {
                    app.delete_config.output_filename = "output_deleted_pages.pdf".to_string();
                }
            }
            KeyCode::Esc => {
                app.delete_config.editing_output = false;
            }
            _ => {}
        }
        return;
    }

    match key {
        KeyCode::Char('p') | KeyCode::Char('P') => {
            app.delete_config.editing_pages = true;
        }

        KeyCode::Tab => {
            app.delete_config.editing_output = true;
        }

        KeyCode::Enter => {
            if app.selected_files().is_empty() {
                app.set_error("No file selected".to_string());
            } else if app.delete_config.pages_to_delete.is_empty() {
                app.set_error("Please specify pages to delete".to_string());
            } else if app.delete_config.output_filename.is_empty() {
                app.set_error("Output filename cannot be empty".to_string());
            } else {
                match validate_page_ranges(&app.delete_config.pages_to_delete) {
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

    match validate_page_ranges(&app.delete_config.pages_to_delete) {
        Ok(pages_to_delete) => {
            match pdf::delete_pages(
                &app.selected_files()[0],
                &app.delete_config.output_filename,
                &pages_to_delete,
            ) {
                Ok(()) => {
                    app.set_success(format!(
                        "✅ Successfully deleted pages {} from '{}' and saved to '{}'",
                        app.delete_config.pages_to_delete,
                        app.selected_files()[0],
                        app.delete_config.output_filename
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
