use crate::tui::app::{App, CurrentScreen};
use crate::tui::utils::validate_merge_requirements;
use crossterm::event::KeyCode;

/**
 * Handle input in the merge configuration screen.
 * Allows editing output filename, reordering files, and starting the merge.
 * @param key The key event.
 * @param app The application state.
 *
 */
pub fn handle_merge_config_input(key: KeyCode, app: &mut App) {
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
pub fn perform_merge(app: &mut App) {
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
