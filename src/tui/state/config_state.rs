#[derive(Debug, Clone)]
pub struct MergeConfig {
    pub output_filename: String,
    pub editing_output: bool,
}

impl MergeConfig {
    pub fn new() -> Self {
        Self {
            output_filename: String::new(),
            editing_output: false,
        }
    }

    pub fn reset(&mut self) {
        self.output_filename.clear();
        self.editing_output = false;
    }
}

#[derive(Debug, Clone)]
pub struct DeleteConfig {
    pub pages_to_delete: String,
    pub output_filename: String,
    pub editing_pages: bool,
    pub editing_output: bool,
}

impl DeleteConfig {
    pub fn new() -> Self {
        Self {
            pages_to_delete: String::new(),
            output_filename: String::new(),
            editing_pages: false,
            editing_output: false,
        }
    }

    pub fn reset(&mut self) {
        self.pages_to_delete.clear();
        self.output_filename.clear();
        self.editing_pages = false;
        self.editing_output = false;
    }
}

#[derive(Debug, Clone)]
pub struct SplitConfig {
    pub segments: String,
    pub output_prefix: String,
    pub use_named_segments: bool,
    pub editing_segments: bool,
    pub editing_prefix: bool,
}

impl SplitConfig {
    pub fn new() -> Self {
        Self {
            segments: String::new(),
            output_prefix: String::new(),
            use_named_segments: false,
            editing_segments: false,
            editing_prefix: false,
        }
    }

    pub fn reset(&mut self) {
        self.segments.clear();
        self.output_prefix.clear();
        self.use_named_segments = false;
        self.editing_segments = false;
        self.editing_prefix = false;
    }
}
