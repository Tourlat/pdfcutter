use crate::tui::app::App;
use crate::tui::state::CurrentScreen;
use crossterm::event::KeyCode;

pub fn handle_split_config_input(key: KeyCode, app: &mut App) {
    if app.ui_state.get_error_message().is_some() && key != KeyCode::Esc {
        app.ui_state.clear_message();
        return;
    }

    if app.split_config.editing_segments {
        match key {
            KeyCode::Char(c) => {
                app.split_config.segments.push(c);
            }
            KeyCode::Backspace => {
                app.split_config.segments.pop();
            }
            KeyCode::Enter | KeyCode::Tab => {
                app.split_config.editing_segments = false;
                if app.split_config.segments.is_empty() {
                    if app.split_config.use_named_segments {
                        app.split_config.segments =
                            "intro:1-3,chapter1:4-10,conclusion:11".to_string();
                    } else {
                        app.split_config.segments = "1-3,5,7-9".to_string();
                    }
                }
            }
            KeyCode::Esc => {
                app.split_config.editing_segments = false;
            }
            _ => {}
        }
        return;
    }

    if app.split_config.editing_prefix {
        match key {
            KeyCode::Char(c) => {
                app.split_config.output_prefix.push(c);
            }
            KeyCode::Backspace => {
                app.split_config.output_prefix.pop();
            }
            KeyCode::Enter | KeyCode::Tab => {
                app.split_config.editing_prefix = false;
                if app.split_config.output_prefix.is_empty() {
                    app.split_config.output_prefix = "split_output".to_string();
                }
            }
            KeyCode::Esc => {
                app.split_config.editing_prefix = false;
            }
            _ => {}
        }
        return;
    }

    match key {
        KeyCode::Char('s') | KeyCode::Char('S') => {
            app.split_config.editing_segments = true;
        }
        KeyCode::Char(' ') => {
            app.split_config.use_named_segments = !app.split_config.use_named_segments;
            app.split_config.segments.clear();
        }
        KeyCode::Char('p') | KeyCode::Char('P') => {
            app.split_config.editing_prefix = true;
        }
        KeyCode::Enter => {
            if app.selected_files().is_empty() {
                app.set_error("No file selected".to_string());
            } else if app.split_config.segments.is_empty() {
                app.set_error("Please specify page segments".to_string());
            } else if app.split_config.output_prefix.is_empty() {
                app.set_error("Output prefix cannot be empty".to_string());
            } else {
                perform_split(app);
            }
        }
        KeyCode::Esc => {
            app.current_screen = CurrentScreen::FileSelection;
        }
        _ => {}
    }
}

pub fn perform_split(app: &mut App) {
    use crate::pdf;

    let result = if app.split_config.use_named_segments {
        pdf::split::split_pdfs_named(
            &app.selected_files()[0],
            &app.split_config.output_prefix,
            &app.split_config.segments,
        )
    } else {
        pdf::split::split_pdfs(
            &app.selected_files()[0],
            &app.split_config.output_prefix,
            &app.split_config.segments,
        )
    };

    match result {
        Ok(output_files) => {
            app.set_success(format!(
                "✅ Successfully split PDF into {} files: {}",
                output_files.len(),
                output_files.join(", ")
            ));
            app.current_screen = CurrentScreen::Result;
        }
        Err(e) => {
            app.set_error(format!("Failed to split PDF: {}", e));
            app.current_screen = CurrentScreen::Result;
        }
    }
}
