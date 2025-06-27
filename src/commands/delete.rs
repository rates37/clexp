use crate::app::App;
use crate::commands::Command;
use std::path::PathBuf;

#[derive(Debug)]
pub struct DeleteCommand {
    targets: Vec<PathBuf>, // support deleting many at once
}

impl DeleteCommand {
    pub fn new(targets: Vec<PathBuf>) -> Self {
        Self { targets }
    }

    pub fn new_single(target: PathBuf) -> Self {
        Self {
            targets: vec![target],
        }
    }
}

impl Command for DeleteCommand {
    fn execute(&mut self, app: &mut App) -> anyhow::Result<()> {
        let mut delete_count = 0;
        let mut errors = Vec::new();

        // try delete each file/directory:
        for target in &self.targets {
            match if target.is_dir() {
                std::fs::remove_dir_all(target)
            } else {
                std::fs::remove_file(target)
            } {
                Ok(()) => delete_count += 1,
                Err(e) => errors.push(format!("{} {}", target.display(), e)),
            }
        }

        // handle errors:
        if errors.is_empty() {
            app.set_status(format!("Deleted {} item(s)", delete_count));
        } else {
            app.set_error(format!(
                "Deleted {} item(s), with {} error(s): {}",
                delete_count,
                errors.len(),
                errors.join(", ")
            ));
        }

        // update display:
        app.refresh_file_list()?;

        Ok(())
    }

    fn description(&self) -> String {
        if self.targets.len() == 1 {
            format!("Delete '{}'", self.targets[0].display())
        } else {
            format!("Delete {} items", self.targets.len())
        }
    }
}
