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

    pub fn get_default_filename(&self) -> &str {
        if self.output_filename.is_empty() {
            "merged_output.pdf"
        } else {
            &self.output_filename
        }
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

    pub fn get_default_filename(&self) -> &str {
        if self.output_filename.is_empty() {
            "modified_output.pdf"
        } else {
            &self.output_filename
        }
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

    pub fn get_default_segments(&self) -> &str {
        if self.segments.is_empty() {
            if self.use_named_segments {
                "intro:1-3,chapter1:4-10"
            } else {
                "1-3,5,7-9"
            }
        } else {
            &self.segments
        }
    }

    pub fn get_default_prefix(&self) -> &str {
        if self.output_prefix.is_empty() {
            "split_output"
        } else {
            &self.output_prefix
        }
    }
}
