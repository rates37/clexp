use crate::app::App;
use crate::commands::Command;
use std::path::PathBuf;

#[derive(Debug)]
pub struct MoveCommand {
    sources: Vec<PathBuf>,
    destination: PathBuf,
    moved_items: Vec<(PathBuf, PathBuf)>, // tuples of original location, new location (for undo function)
}

impl MoveCommand {
    pub fn new(sources: Vec<PathBuf>, destination: PathBuf) -> Self {
        Self {
            sources,
            destination,
            moved_items: Vec::new(),
        }
    }
}

impl Command for MoveCommand {
    fn execute(&mut self, app: &mut App) -> anyhow::Result<()> {
        let mut moved_count = 0;
        let mut errors = Vec::new();

        for source in &self.sources {
            let dest_path = if self.destination.is_dir() {
                self.destination
                    .join(source.file_name().unwrap_or_default())
            } else {
                self.destination.clone()
            };

            match std::fs::rename(source, &dest_path) {
                Ok(()) => {
                    self.moved_items.push((source.clone(), dest_path));
                    moved_count += 1;
                }
                Err(e) => errors.push(format!("{}: {}", source.display(), e)),
            }
        }

        // check if any errors occurred:
        if errors.is_empty() {
            app.set_status(format!("Moved {} item(s)", moved_count));
        } else {
            app.set_error(format!(
                "Moved {} item(s), {} error(s): {}",
                moved_count,
                errors.len(),
                errors.join(", ")
            ));
        }

        // update display:
        app.refresh_file_list()?;

        Ok(())
    }

    fn description(&self) -> String {
        if self.sources.len() == 1 {
            format!(
                "Move '{}' to '{}'",
                self.sources[0].display(),
                self.destination.display()
            )
        } else {
            format!(
                "Move {} items to '{}'",
                self.sources.len(),
                self.destination.display()
            )
        }
    }

    fn undo(&mut self, app: &mut App) -> anyhow::Result<()> {
        let mut restored_count = 0;
        let mut errors = Vec::new();

        // move all files back
        for (original, moved) in &self.moved_items {
            match std::fs::rename(moved, original) {
                Ok(()) => restored_count += 1,
                Err(e) => errors.push(format!("{}: {}", moved.display(), e)),
            }
        }

        // check for errors:
        if errors.is_empty() {
            app.set_status(format!("Undid move: restored {} item(s)", restored_count));
        } else {
            app.set_error(format!(
                "Restored {} item(s), {} error(s): {}",
                restored_count,
                errors.len(),
                errors.join(", ")
            ));
        }

        // update display:
        app.refresh_file_list()?;

        Ok(())
    }
}
