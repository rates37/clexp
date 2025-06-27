use crate::app::App;
use anyhow::Result;

/// Command trait defines the interface for all file operations
pub trait Command: std::fmt::Debug {
    /// Execute the command, modifying the app state as needed:
    fn execute(&mut self, app: &mut App) -> Result<()>;

    /// Get a human-readable description of the command's purpose
    fn description(&self) -> String;

    /// Optional: Undo the command. Not all commands might need to support this
    fn undo(&mut self, _app: &mut App) -> Result<()> {
        Err(anyhow::anyhow!("Undo not implemented for this command!"))
    }
}

pub mod rename;
pub use rename::RenameCommand;

pub mod create;
pub use create::{CreateDirCommand, CreateFileCommand};

pub mod delete;
pub use delete::DeleteCommand;

pub mod copy;
pub use copy::CopyCommand;
