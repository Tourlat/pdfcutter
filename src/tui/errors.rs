// src/tui/errors.rs
#[derive(Debug, thiserror::Error)]
pub enum TuiError {
    #[error("File not found: {path}")]
    FileNotFound { path: String },
    
    #[error("Invalid PDF file: {path}")]
    InvalidPdf { path: String },
    
    #[error("Invalid page range: {input}")]
    InvalidPageRange { input: String },
    
    #[error("Not enough files for merge (need at least 2, got {count})")]
    InsufficientFiles { count: usize },
    
    #[error("Too many files for delete operation (need exactly 1, got {count})")]
    TooManyFiles { count: usize },
    
    #[error("PDF operation failed: {source}")]
    PdfOperation { #[from] source: anyhow::Error },
}

pub type TuiResult<T> = Result<T, TuiError>;