use super::state::{
    CurrentScreen, DeleteConfig, FileState, MergeConfig, OperationMode, SplitConfig, UiState,
};

pub struct App {
    pub current_screen: CurrentScreen,
    pub operation_mode: OperationMode,

    pub file_state: FileState,
    pub merge_config: MergeConfig,
    pub delete_config: DeleteConfig,
    pub split_config: SplitConfig,
    pub ui_state: UiState,
}

impl App {
    pub fn new() -> Self {
        Self {
            current_screen: CurrentScreen::Main,
            operation_mode: OperationMode::None,
            file_state: FileState::new(),
            merge_config: MergeConfig::new(),
            delete_config: DeleteConfig::new(),
            split_config: SplitConfig::new(),
            ui_state: UiState::new(),
        }
    }

    pub fn reset(&mut self) {
        self.operation_mode = OperationMode::None;
        self.current_screen = CurrentScreen::Main;
        self.file_state.reset();
        self.merge_config.reset();
        self.delete_config.reset();
        self.split_config.reset();
        self.ui_state.reset();
    }

    pub fn set_error(&mut self, message: String) {
        self.ui_state.set_error(message);
    }

    pub fn set_success(&mut self, message: String) {
        self.ui_state.set_success(message);
    }

    pub fn selected_files(&self) -> &Vec<String> {
        &self.file_state.selected_files
    }

    pub fn selected_files_mut(&mut self) -> &mut Vec<String> {
        &mut self.file_state.selected_files
    }

    pub fn selected_file_index(&self) -> usize {
        self.file_state.selected_file_index
    }

    pub fn merge_file_index(&self) -> usize {
        self.file_state.merge_file_index
    }

    pub fn error_message(&self) -> Option<&str> {
        self.ui_state.get_error_message()
    }

    pub fn success_message(&self) -> Option<&str> {
        self.ui_state.get_success_message()
    }

    pub fn current_input(&self) -> Option<&str> {
        self.ui_state.current_input.as_deref()
    }

    pub fn editing_input(&self) -> bool {
        self.ui_state.editing_input
    }

    pub fn menu_mode_index(&self) -> usize {
        self.ui_state.menu_mode_index
    }

    pub fn set_selected_file_index(&mut self, index: usize) {
        self.file_state.selected_file_index = index;
    }

    pub fn set_merge_file_index(&mut self, index: usize) {
        self.file_state.merge_file_index = index;
    }

    pub fn set_menu_mode_index(&mut self, index: usize) {
        self.ui_state.menu_mode_index = index;
    }

    pub fn set_editing_input(&mut self, editing: bool) {
        self.ui_state.editing_input = editing;
    }

    pub fn set_current_input(&mut self, input: Option<String>) {
        self.ui_state.current_input = input;
    }

    pub fn add_file(&mut self, file_path: String) {
        self.file_state.add_file(file_path);
    }

    pub fn remove_current_file(&mut self) {
        let index = self.file_state.selected_file_index;
        self.file_state.remove_file(index);
    }

    pub fn swap_files(&mut self, index1: usize, index2: usize) {
        self.file_state.swap_files(index1, index2);
    }
}
