#[derive(Debug, Clone)]
pub struct FileState {
    pub selected_files: Vec<String>,
    pub selected_file_index: usize,
    pub merge_file_index: usize,
}

impl FileState {
    pub fn new() -> Self {
        Self {
            selected_files: Vec::new(),
            selected_file_index: 0,
            merge_file_index: 0,
        }
    }

    pub fn reset(&mut self) {
        self.selected_files.clear();
        self.selected_file_index = 0;
        self.merge_file_index = 0;
    }

    pub fn add_file(&mut self, file_path: String) {
        self.selected_files.push(file_path);
    }

    pub fn remove_file(&mut self, index: usize) {
        if index < self.selected_files.len() {
            self.selected_files.remove(index);
            // Ajuster les index si nécessaire
            if self.selected_file_index >= self.selected_files.len()
                && !self.selected_files.is_empty()
            {
                self.selected_file_index = self.selected_files.len() - 1;
            }
            if self.merge_file_index >= self.selected_files.len() && !self.selected_files.is_empty()
            {
                self.merge_file_index = self.selected_files.len() - 1;
            }
        }
    }

    pub fn swap_files(&mut self, index1: usize, index2: usize) {
        if index1 < self.selected_files.len() && index2 < self.selected_files.len() {
            self.selected_files.swap(index1, index2);
        }
    }

    pub fn is_empty(&self) -> bool {
        self.selected_files.is_empty()
    }

    pub fn len(&self) -> usize {
        self.selected_files.len()
    }
}
