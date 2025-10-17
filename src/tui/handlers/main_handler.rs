use crate::tui::app::{self, App, CurrentScreen, OperationMode};
use crossterm::event::KeyCode;

pub fn handle_main_input(key: KeyCode, app: &mut App) {
    let number_of_menu_items = 4;

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
            app.menu_mode_index = 3;
            app.operation_mode = app::OperationMode::Split;
            app.current_screen = CurrentScreen::FileSelection;
        }
        KeyCode::Char('4') => {
            app.menu_mode_index = 2;
            app.current_screen = CurrentScreen::Help;
        }
        KeyCode::Up => {
            if app.menu_mode_index > 0 {
                app.menu_mode_index -= 1;
            } else {
                app.menu_mode_index = number_of_menu_items;
            }
        }
        KeyCode::Down => {
            if app.menu_mode_index < number_of_menu_items {
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
                app.reset();
                app.operation_mode = OperationMode::Split;
                app.current_screen = CurrentScreen::FileSelection;
            }
            3 => {
                app.current_screen = CurrentScreen::Help;
            }
            4 => {
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
