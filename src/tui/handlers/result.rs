use crossterm::event::KeyCode;
use crate::tui::app::{App, CurrentScreen};

/**
 * Handle input in the result screen.
 * Shows success/error messages and allows returning to main menu.
 * @param key The key event.
 * @param app The application state.
 */
pub fn handle_result_input(key: KeyCode, app: &mut App) {
    match key {
        KeyCode::Esc | KeyCode::Enter | KeyCode::Char(' ') => {
            app.current_screen = CurrentScreen::Main;
        }
        _ => {}
    }
}