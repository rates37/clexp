use crate::app::App;
use crate::commands::Command;
use std::path::PathBuf;

#[derive(Debug)]
pub struct CopyCommand {
    sources: Vec<PathBuf>,
    destination: PathBuf,
}

impl CopyCommand {
    pub fn new(sources: Vec<PathBuf>, destination: PathBuf) -> Self {
        Self {
            sources,
            destination,
        }
    }
}

impl Command for CopyCommand {
    fn execute(&mut self, app: &mut App) -> anyhow::Result<()> {
        let mut copy_count = 0;
        let mut errors = Vec::new();

        // copy all sources to the destination:
        for source in &self.sources {
            let dest_path = if self.destination.is_dir() {
                self.destination
                    .join(source.file_name().unwrap_or_default())
            } else {
                self.destination.clone()
            };

            match copy_recursively(source, &dest_path) {
                Ok(()) => copy_count += 1,
                Err(e) => errors.push(format!("{}: {}", source.display(), e)),
            }
        }

        // check if any errors occurred in copying
        if errors.is_empty() {
            app.set_status(format!("Copied {} item(s)", copy_count));
        } else {
            app.set_error(format!(
                "Copied {} item(s), {} error(s): {}",
                copy_count,
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
                "Copy '{}' to '{}'",
                self.sources[0].display(),
                self.destination.display()
            )
        } else {
            format!(
                "Copy {} items to '{}'",
                self.sources.len(),
                self.destination.display()
            )
        }
    }
}

fn copy_recursively(source: &PathBuf, destination: &PathBuf) -> anyhow::Result<()> {
    if source.is_dir() {
        // create destination directory:
        std::fs::create_dir_all(destination)?;

        // copy each entry in the source dir:
        for entry in std::fs::read_dir(source)? {
            let entry = entry?;
            let source_path = entry.path();
            let dest_path = destination.join(entry.file_name());
            copy_recursively(&source_path, &dest_path)?;
        }
    } else {
        // Ensure destination's parent directory exists:
        if let Some(parent) = destination.parent() {
            std::fs::create_dir_all(parent)?;
        }
        // copy file:
        std::fs::copy(source, destination)?;
    }

    Ok(())
}
