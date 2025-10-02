#[derive(Debug, PartialEq)]
pub enum CurrentScreen {
    Main,
    FileSelection,
    MergeConfig,
    DeleteConfig,
    // Processing,
    Result,
    Help,
    Exiting,
}

#[derive(PartialEq, Debug)]
pub enum OperationMode {
    None,
    Merge,
    Delete,
}

pub struct App {
    pub current_screen: CurrentScreen,
    pub operation_mode: OperationMode,

    pub selected_files: Vec<String>,
    pub output_filename: String,
    pub pages_to_delete: String,

    pub error_message: Option<String>,
    pub success_message: Option<String>,

    pub current_input: Option<String>, // Input text
    pub selected_file_index: usize,
    pub editing_output: bool,
    pub editing_pages: bool,
    pub merge_file_index: usize,

    pub menu_mode_index: usize,
    pub editing_input: bool,
}

impl App {
    pub fn new() -> App {
        App {
            current_screen: CurrentScreen::Main,
            operation_mode: OperationMode::None,
            selected_files: Vec::new(),
            output_filename: String::new(),
            pages_to_delete: String::new(),
            error_message: None,
            success_message: None,
            selected_file_index: 0,
            current_input: None,
            editing_output: false,
            merge_file_index: 0,
            editing_pages: false,
            menu_mode_index: 0,
            editing_input: false,
        }
    }

    pub fn reset(&mut self) {
        self.selected_files.clear();
        self.output_filename.clear();
        self.pages_to_delete.clear();
        self.error_message = None;
        self.success_message = None;
        self.operation_mode = OperationMode::None;
        self.current_screen = CurrentScreen::Main;
        self.selected_file_index = 0;
        self.current_input = None;
        self.editing_output = false;
        self.merge_file_index = 0;
        self.editing_pages = false;
        self.menu_mode_index = 0;
        self.editing_input = false;
    }

    pub fn set_error(&mut self, message: String) {
        self.error_message = Some(message);
        self.success_message = None;
    }

    pub fn set_success(&mut self, message: String) {
        self.success_message = Some(message);
        self.error_message = None;
    }
}
