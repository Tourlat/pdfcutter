use crate::tui::app::{App, CurrentScreen, OperationMode};
use crate::tui::utils::*;
use crossterm::event::{KeyCode, KeyModifiers};

/**
 * Handle input in the file selection screen.
 * Allows adding/removing files, navigating the list, and proceeding to the next configuration screen.
 * @param key The key event.
 * @param app The application state.
 */
pub fn handle_file_selection_input(key: KeyCode, key_event_modifier: KeyModifiers, app: &mut App) {
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
