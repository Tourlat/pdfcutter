use super::MessageType;

#[derive(Debug)]
pub struct UiState {
    pub current_input: Option<String>,
    pub editing_input: bool,
    pub menu_mode_index: usize,
    pub message: Option<MessageType>,
}

impl UiState {
    pub fn new() -> Self {
        Self {
            current_input: None,
            editing_input: false,
            menu_mode_index: 0,
            message: None,
        }
    }

    pub fn reset(&mut self) {
        self.current_input = None;
        self.editing_input = false;
        self.menu_mode_index = 0;
        self.message = None;
    }

    pub fn set_error(&mut self, message: String) {
        self.message = Some(MessageType::Error(message));
    }

    pub fn set_success(&mut self, message: String) {
        self.message = Some(MessageType::Success(message));
    }

    pub fn clear_message(&mut self) {
        self.message = None;
    }

    pub fn get_error_message(&self) -> Option<&str> {
        match &self.message {
            Some(MessageType::Error(msg)) => Some(msg),
            _ => None,
        }
    }

    pub fn get_success_message(&self) -> Option<&str> {
        match &self.message {
            Some(MessageType::Success(msg)) => Some(msg),
            _ => None,
        }
    }



    pub fn stop_input(&mut self) {
        self.editing_input = false;
        self.current_input = Some(String::new());
    }

    pub fn get_input_text(&self) -> &str {
        self.current_input.as_deref().unwrap_or("")
    }

    pub fn input_char(&mut self, c: char) {
        if let Some(ref mut input) = self.current_input {
            input.push(c);
        } else {
            self.current_input = Some(c.to_string());
        }
    }

    pub fn input_backspace(&mut self) {
        if let Some(ref mut input) = self.current_input {
            input.pop();
        }
    }
}
