use crate::tui::app::App;
use crate::tui::state::{CurrentScreen, OperationMode};
use crate::tui::utils::{
    validate_delete_requirements, validate_file_input, validate_merge_requirements,
    validate_split_requirements,
};
use crossterm::event::{KeyCode, KeyModifiers};

/**
 * Handle input in the file selection screen.
 * Allows adding/removing files, navigating the list, and proceeding to the next configuration screen.
 * @param key The key event.
 * @param app The application state.
 */
pub fn handle_file_selection_input(key: KeyCode, key_event_modifier: KeyModifiers, app: &mut App) {
    if app.ui_state.get_error_message().is_some() && key != KeyCode::Esc {
        app.ui_state.clear_message();
        return;
    }

    if app.ui_state.editing_input {
        match key {
            KeyCode::Char(c) => {
                app.ui_state.input_char(c);
            }
            KeyCode::Backspace => {
                app.ui_state.input_backspace();
            }
            KeyCode::Enter => {
                let input_text = app.ui_state.get_input_text();
                if !input_text.is_empty() {
                    match validate_file_input(input_text) {
                        Ok(()) => {
                            app.file_state.add_file(input_text.to_string());
                            app.ui_state.stop_input();
                            app.ui_state.clear_message();
                        }
                        Err(e) => {
                            app.set_error(e.to_string());
                            app.ui_state.editing_input = false;
                        }
                    }
                } else {
                    app.ui_state.editing_input = false;
                }
            }
            KeyCode::Esc => {
                app.ui_state.stop_input();
            }
            _ => {}
        }
        return;
    }

    match (key, key_event_modifier) {
        (KeyCode::Up, KeyModifiers::ALT) => {
            if app.file_state.selected_file_index > 0 {
                app.file_state.selected_files.swap(
                    app.file_state.selected_file_index,
                    app.file_state.selected_file_index - 1,
                );
                app.file_state.selected_file_index -= 1;
            }
        }
        (KeyCode::Down, KeyModifiers::ALT) => {
            if app.file_state.selected_file_index
                < app.file_state.selected_files.len().saturating_sub(1)
            {
                app.file_state.selected_files.swap(
                    app.file_state.selected_file_index,
                    app.file_state.selected_file_index + 1,
                );
                app.file_state.selected_file_index += 1;
            }
        }
        (key, KeyModifiers::NONE) | (key, KeyModifiers::SHIFT) => match key {
            KeyCode::Up => {
                if app.file_state.selected_file_index > 0 {
                    app.file_state.selected_file_index -= 1;
                }
            }
            KeyCode::Down => {
                if app.file_state.selected_file_index
                    < app.file_state.selected_files.len().saturating_sub(1)
                {
                    app.file_state.selected_file_index += 1;
                }
            }

            KeyCode::Tab => {
                if (app.operation_mode == OperationMode::Delete
                    || app.operation_mode == OperationMode::Split)
                    && !app.file_state.selected_files.is_empty()
                {
                    return;
                }
                app.ui_state.editing_input = true;
                app.ui_state.current_input = Some(String::new());
            }

            KeyCode::Enter | KeyCode::Right => {
                let validation_result = match app.operation_mode {
                    OperationMode::Merge => {
                        validate_merge_requirements(&app.file_state.selected_files)
                    }
                    OperationMode::Delete => {
                        validate_delete_requirements(&app.file_state.selected_files)
                    }
                    OperationMode::Split => {
                        validate_split_requirements(&app.file_state.selected_files)
                    }
                    _ => Ok(()),
                };

                match validation_result {
                    Ok(()) => {
                        app.current_screen = match app.operation_mode {
                            OperationMode::Merge => CurrentScreen::MergeConfig,
                            OperationMode::Delete => CurrentScreen::DeleteConfig,
                            OperationMode::Split => CurrentScreen::SplitConfig,
                            _ => CurrentScreen::Main,
                        };
                        app.ui_state.clear_message();
                    }
                    Err(e) => {
                        app.set_error(e.to_string());
                    }
                }
            }

            KeyCode::Backspace => {
                if !app.file_state.selected_files.is_empty()
                    && app.file_state.selected_file_index < app.file_state.selected_files.len()
                {
                    app.file_state
                        .selected_files
                        .remove(app.file_state.selected_file_index);
                    if app.file_state.selected_file_index > 0
                        && app.file_state.selected_file_index >= app.file_state.selected_files.len()
                    {
                        app.file_state.selected_file_index =
                            app.file_state.selected_files.len().saturating_sub(1);
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
