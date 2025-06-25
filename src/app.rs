use anyhow::Result;
use color_eyre::owo_colors::colors::Magenta;
use std::{fs::DirEntry, path::PathBuf, vec};

#[derive(Debug, Clone, PartialEq)]
pub enum AppMode {
    Normal,      // default mode
    MultiSelect, // when selecting multiple files
    Input,       // For when giving input to rename, create, search, etc
    Confirm, // For when prompting the user to confirm a decision (for dangerous actions like deletion)
    Command, // When user entering a command
    Help,    // When app is showing help modal
    Clipboard, // When app is showing the contents of the clipboard
}

pub struct App {
    // Core state:
    pub should_exit: bool,
    pub mode: AppMode,

    // Backend State:
    pub current_path: PathBuf,
    pub file_list: Vec<String>,

    // UI State:
    pub error_message: Option<String>,
    pub status_message: Option<String>, // Misc:
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
            file_list: vec![],

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

// File items:
#[derive(Debug, Clone)]
pub struct FileItem {
    pub name: String,
    pub path: PathBuf,     // path to this item
    pub is_dir: bool,      // whether or not is a directory
    pub size: Option<u64>, // size in bytes
    pub modified: Option<std::time::SystemTime>, // last modified date
                           // pub permissions: Option<String>,  // much later feature so removed for now
}

impl FileItem {
    pub fn display_name(&self) -> String {
        if self.is_dir {
            format!("{}/", self.name)
        } else {
            self.name.clone()
        }
    }

    pub fn from_dir_entry(entry: DirEntry) -> Result<Self> {
        let metadata = entry.metadata()?;
        let path = entry.path();
        let name = entry
            .file_name()
            .into_string()
            .unwrap_or_else(|_| "Invalid filename".to_string());
        Ok(Self {
            name: name,
            path: path,
            is_dir: metadata.is_dir(),
            size: if metadata.is_file() {
                Some(metadata.len())
            } else {
                None
            },
            modified: metadata.modified().ok(),
        })
    }
}
