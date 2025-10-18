pub mod main_handler;
pub mod file_selection;
pub mod merge_config;
pub mod delete_config;
pub mod result;
pub mod split_config;

pub use main_handler::handle_main_input;
pub use file_selection::handle_file_selection_input;
pub use merge_config::handle_merge_config_input;
pub use delete_config::handle_delete_config_input;
pub use result::handle_result_input;
pub use split_config::handle_split_config_input;
