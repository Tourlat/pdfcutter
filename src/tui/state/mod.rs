pub mod config_state;
pub mod file_state;
pub mod ui_state;

pub use config_state::{DeleteConfig, MergeConfig, SplitConfig};
pub use file_state::FileState;
pub use ui_state::UiState;

#[derive(Debug, PartialEq)]
pub enum CurrentScreen {
    Main,
    FileSelection,
    MergeConfig,
    DeleteConfig,
    SplitConfig,
    Result,
    Help,
    Exiting,
}

#[derive(PartialEq, Debug)]
pub enum OperationMode {
    None,
    Merge,
    Delete,
    Split,
}

#[derive(Debug)]
pub enum MessageType {
    Error(String),
    Success(String),
}
