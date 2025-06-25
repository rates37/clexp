use std::path::PathBuf;
use anyhow::Result;

#[derive(Debug, Clone, PartialEq)]
pub enum AppMode {
    Normal,  // default mode
    MultiSelect, // when selecting multiple files
    Input,  // For when giving input to rename, create, search, etc
    Confirm,  // For when prompting the user to confirm a decision (for dangerous actions like deletion)
    Command,  // When user entering a command
    Help,  // When app is showing help modal
    Clipboard,  // When app is showing the contents of the clipboard
}

pub struct App {
    // Core state:
    pub should_exit: bool,
    pub mode: AppMode,

    // Backend State:
    pub current_path: PathBuf,

    // UI State:
    pub error_message: Option<String>,
    pub status_message: Option<String>

    // Misc:
}

impl App {
    pub fn new() -> Result<Self> {
        let current_path = std::env::current_dir()?;
        let mut app = Self {
            // Core state:
            should_exit: false,
            mode: AppMode::Normal,

            // Backend State:
            current_path: current_path,

            // UI State:
            error_message: Some("Fuck".to_string()),
            status_message: None,

            // Misc:

        };

        Ok(app)
    }

    pub fn set_error(&mut self, message: String) {
        self.error_message = Some(message);
    }
}