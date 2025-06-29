use anyhow::Result;
use ratatui::widgets::ListState;
use std::{
    fs::{self, DirEntry},
    path::PathBuf,
};

use crate::commands::Command;

pub struct App {
    // Core state:
    pub should_exit: bool,
    pub mode: AppMode,

    // Backend State:
    pub current_path: PathBuf,
    pub file_list: StatefulList<FileItem>,

    // UI State:
    pub error_message: Option<String>,
    pub status_message: Option<String>,
    pub selection: Vec<usize>,

    // help UI:
    pub help_scroll_offset: usize,

    // Input handling:
    pub input_buffer: String,
    pub cursor_position: usize,
    pub input_context: Option<InputContext>,

    // Operation State:
    pub active_command: Option<Box<dyn Command>>,
    pub clipboard: Clipboard,
    pub clipboard_scroll_offset: usize,
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
            file_list: StatefulList::new(),

            // UI State:
            error_message: None,
            status_message: None,
            selection: Vec::new(),

            // help UI:
            help_scroll_offset: 0,

            // Input handling:
            input_buffer: String::new(),
            cursor_position: 0,
            input_context: None,

            // Operation State:
            active_command: None,
            clipboard: Clipboard::new(),
            clipboard_scroll_offset: 0,
        };

        app.refresh_file_list()?;

        Ok(app)
    }

    pub fn set_error(&mut self, message: String) {
        self.error_message = Some(message);
    }

    pub fn refresh_file_list(&mut self) -> Result<()> {
        let mut entries = fs::read_dir(&self.current_path)?
            .filter_map(|entry| entry.ok())
            .filter_map(|entry| FileItem::from_dir_entry(entry).ok())
            .collect::<Vec<_>>();

        // sort entries:
        entries.sort_by(|a, b| {
            match (a.is_dir, b.is_dir) {
                // all folders appear before files
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
            }
        });

        // check if root, if not, add parent directory to top of list:
        if let Some(parent) = self.current_path.parent() {
            entries.insert(
                0,
                FileItem {
                    name: "..".to_string(),
                    path: parent.to_path_buf(),
                    is_dir: true,
                    size: None,
                    modified: None,
                },
            );
        }

        // update self
        self.file_list = StatefulList::new_with_items(entries);

        // todo: apply filtering

        Ok(())
    }

    pub fn clear_messages(&mut self) {
        self.error_message = None;
        self.status_message = None;
    }

    pub fn navigate_to(&mut self, path: PathBuf) -> Result<()> {
        if path.is_dir() {
            // todo: keep track of browsing history here?
            self.current_path = path;
            self.refresh_file_list()?;
            self.clear_messages();
        }
        Ok(())
    }

    pub fn navigate_up(&mut self) -> Result<()> {
        if let Some(parent) = self.current_path.parent() {
            self.navigate_to(parent.to_path_buf())?;
        }
        Ok(())
    }

    pub fn enter_selected(&mut self) -> Result<()> {
        if let Some(selected_item) = self.file_list.selected().cloned() {
            if selected_item.is_dir {
                self.navigate_to(selected_item.path)?;
            } else {
                // todo: open file with relevant application
                // use system default app?
            }
        }
        Ok(())
    }

    pub fn scroll_help_down(&mut self, content_length: usize, viewport_height: usize) {
        let max_scroll = if content_length > viewport_height {
            content_length - viewport_height
        } else {
            0
        };
        if self.help_scroll_offset < max_scroll {
            self.help_scroll_offset += 1;
        }
    }

    pub fn scroll_help_up(&mut self) {
        if self.help_scroll_offset > 0 {
            self.help_scroll_offset -= 1;
        }
    }

    pub fn set_status(&mut self, message: String) {
        self.status_message = Some(message);
    }

    pub fn clear_input_buffer(&mut self) {
        self.input_buffer.clear();
        self.cursor_position = 0;
    }

    pub fn move_cursor_left(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
        }
    }

    pub fn move_cursor_right(&mut self) {
        if self.cursor_position < self.input_buffer.len() {
            self.cursor_position += 1;
        }
    }

    pub fn move_cursor_home(&mut self) {
        self.cursor_position = 0;
    }

    pub fn move_cursor_end(&mut self) {
        self.cursor_position = self.input_buffer.len();
    }

    pub fn delete_char_before_cursor(&mut self) -> Option<char> {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
            Some(self.input_buffer.remove(self.cursor_position))
        } else {
            None
        }
    }

    pub fn delete_char_at_cursor(&mut self) -> Option<char> {
        if self.cursor_position < self.input_buffer.len() {
            Some(self.input_buffer.remove(self.cursor_position))
        } else {
            None
        }
    }

    pub fn insert_char_at_cursor(&mut self, c: char) {
        self.input_buffer.insert(self.cursor_position, c);
        self.cursor_position += 1;
    }

    pub fn clear_multi_selection(&mut self) {
        self.selection.clear();
    }

    pub fn toggle_selection(&mut self) {
        if let Some(selected) = self.file_list.state.selected() {
            if let Some(pos) = self.selection.iter().position(|&i| i == selected) {
                self.selection.remove(pos);
            } else {
                self.selection.push(selected);
            }
        }
    }

    pub fn selected_items(&self) -> Vec<&FileItem> {
        self.selection
            .iter()
            .filter_map(|&i| self.file_list.items.get(i))
            .collect()
    }

    pub fn scroll_clipboard_down(&mut self, content_length: usize, viewport_height: usize) {
        let max_scroll = if content_length > viewport_height {
            content_length - viewport_height
        } else {
            0
        };
        if self.clipboard_scroll_offset < max_scroll {
            self.clipboard_scroll_offset += 1;
        }
    }

    pub fn scroll_clipboard_up(&mut self) {
        if self.clipboard_scroll_offset > 0 {
            self.clipboard_scroll_offset -= 1;
        }
    }

    pub fn execute_command(&mut self, command: &str) -> Result<()> {
        let parts: Vec<&str> = command.trim_end().split_whitespace().collect();
        if parts.is_empty() {
            return Ok(());
        }

        let command = parts[0].trim().to_lowercase();
        match command.as_str() {
            "q" | "quit" | "exit" => {
                self.should_exit = true;
            }

            "h" | "help" => {
                self.mode = AppMode::Help;
                self.clear_input_buffer();
            }

            _ => {
                self.error_message = Some(format!("Unknown command: {}", parts[0]));
                self.mode = AppMode::Normal;
            }
        }
        Ok(())
    }
}

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

// Stateful List:
#[derive(Debug, Clone)]
pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
    pub filtered_items: Vec<usize>, // indices of selected items
}

impl<T> StatefulList<T> {
    pub fn new() -> Self {
        Self {
            state: ListState::default(),
            items: Vec::new(),
            filtered_items: Vec::new(),
        }
    }

    pub fn new_with_items(items: Vec<T>) -> Self {
        let selected = (0..items.len()).collect();
        Self {
            state: ListState::default(),
            items: items,
            filtered_items: selected,
        }
    }

    pub fn filtered_items(&self) -> Vec<&T> {
        self.filtered_items
            .iter()
            .filter_map(|&i| self.items.get(i))
            .collect()
    }

    pub fn selected(&self) -> Option<&T> {
        self.state.selected().and_then(|i| self.items.get(i))
    }

    pub fn next(&mut self) {
        if self.filtered_items.is_empty() {
            return;
        }
        let i = match self.state.selected() {
            Some(i) => {
                let current_idx = self
                    .filtered_items
                    .iter()
                    .position(|&x| x == i)
                    .unwrap_or(0);
                self.filtered_items[(current_idx + 1) % self.filtered_items.len()]
            }
            None => self.filtered_items[0],
        };
        self.state.select(Some(i));
    }

    pub fn prev(&mut self) {
        if self.filtered_items.is_empty() {
            return;
        }
        let i = match self.state.selected() {
            Some(i) => {
                let current_idx = self
                    .filtered_items
                    .iter()
                    .position(|&x| x == i)
                    .unwrap_or(0);
                if current_idx == 0 {
                    self.filtered_items[self.filtered_items.len() - 1]
                } else {
                    self.filtered_items[current_idx - 1]
                }
            }
            None => self.filtered_items[0],
        };
        self.state.select(Some(i));
    }
}

// Input context:
#[derive(Debug, Clone, PartialEq)]
pub enum InputContext {
    Rename,
    CreateFile,
    CreateDir,
    Filter,
    Command,
}

// Clipboard:
#[derive(Debug, Clone, PartialEq)]
pub enum ClipboardOperation {
    None,
    Copy,
    Cut,
}

#[derive(Debug, Clone)]
pub struct Clipboard {
    pub items: Vec<PathBuf>,
    pub operation: ClipboardOperation,
}

impl Clipboard {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            operation: ClipboardOperation::None,
        }
    }
}
