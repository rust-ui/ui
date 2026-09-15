// Library interface for ui-cli
// This allows integration tests and external consumers to access shared functionality

pub mod shared {
    pub mod cli_error;
    pub mod framework;
    pub mod markdown_utils;
    pub mod rust_ui_client;
    pub mod task_spinner;
}
