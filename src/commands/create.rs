use crate::app::App;
use crate::commands::Command;
use std::path::PathBuf;

#[derive(Debug)]
pub struct CreateFileCommand {
    path: PathBuf,
    content: String,
    created: bool,
}

impl CreateFileCommand {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            content: String::new(),
            created: false,
        }
    }

    pub fn new_with_content(path: PathBuf, content: String) -> Self {
        Self {
            path,
            content,
            created: false,
        }
    }
}

impl Command for CreateFileCommand {
    fn execute(&mut self, app: &mut App) -> anyhow::Result<()> {
        // create parent directory (if doesn't exist):
        if let Some(parent) = self.path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        // create the file:
        std::fs::write(&self.path, &self.content)?;
        self.created = true;

        // update display:
        app.refresh_file_list()?;
        app.set_status(format!("Created file: {}", self.path.display()));

        Ok(())
    }

    fn description(&self) -> String {
        format!("Create file {}", self.path.display())
    }

    fn undo(&mut self, app: &mut App) -> anyhow::Result<()> {
        if self.created {
            // remove the file:
            std::fs::remove_file(&self.path)?;

            // update display:
            app.refresh_file_list()?;
            app.set_status(format!("Undid 'Create file: {}'", self.path.display()));
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct CreateDirCommand {
    path: PathBuf,
    created: bool,
}

impl CreateDirCommand {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            created: false,
        }
    }
}

impl Command for CreateDirCommand {
    fn execute(&mut self, app: &mut App) -> anyhow::Result<()> {
        // create directory:
        std::fs::create_dir_all(&self.path)?;
        self.created = true;

        // update display:
        app.refresh_file_list()?;
        app.set_status(format!("Created directory: {}", self.path.display()));

        Ok(())
    }

    fn description(&self) -> String {
        format!("Create directory: '{}'", self.path.display())
    }

    fn undo(&mut self, app: &mut App) -> anyhow::Result<()> {
        if self.created {
            // remove the directory:
            std::fs::remove_dir(&self.path)?;

            // update display:
            app.refresh_file_list()?;
            app.set_status(format!("Removed directory: '{}'", self.path.display()));
        }

        Ok(())
    }
}
