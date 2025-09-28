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
