use crate::app::App;
use crate::commands::Command;
use std::path::PathBuf;

#[derive(Debug)]
pub struct RenameCommand {
    source: PathBuf,
    new_name: String,
    old_name: Option<String>, // For potential undo functionality later
}

impl RenameCommand {
    pub fn new(source: PathBuf, new_name: String) -> Self {
        Self {
            source,
            new_name,
            old_name: None,
        }
    }
}

impl Command for RenameCommand {
    fn execute(&mut self, app: &mut App) -> anyhow::Result<()> {
        let old_name = self
            .source
            .file_name()
            .ok_or_else(|| anyhow::anyhow!("Invalid file path"))?
            .to_string_lossy()
            .to_string();

        let new_path = self.source.with_file_name(&self.new_name);

        std::fs::rename(&self.source, &new_path)?;

        self.old_name = Some(old_name);
        app.refresh_file_list()?;
        app.set_status(format!(
            "Renamed '{}' to '{}'",
            self.old_name.as_ref().unwrap(),
            self.new_name
        ));
        Ok(())
    }

    fn description(&self) -> String {
        format!("Rename '{}' to '{}'", self.source.display(), self.new_name)
    }

    fn undo(&mut self, app: &mut App) -> anyhow::Result<()> {
        if let Some(old_name) = &self.old_name {
            let current_path = self.source.with_file_name(&self.new_name);
            let original_path = self.source.with_file_name(old_name);

            std::fs::rename(current_path, original_path)?;
            app.refresh_file_list()?;
            app.set_status(format!("Undid rename: restored '{}'", old_name));
        }
        Ok(())
    }
}
