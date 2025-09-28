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
