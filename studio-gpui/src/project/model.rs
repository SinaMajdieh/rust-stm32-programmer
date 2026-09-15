use std::path::PathBuf;

use firmware_targets::{TargetKind, TemplateKind};

#[derive(Debug, Clone)]
pub struct Project {
    pub name: String,
    pub location: PathBuf,
    pub target: TargetKind,
    pub template: TemplateKind,
}

impl Project {
    pub fn validate(&self) -> Result<(), String> {
        self.validate_name()?;
        self.validate_location()?;

        Ok(())
    }

    fn validate_name(&self) -> Result<(), String> {
        let name = self.name.trim();

        if name.is_empty() {
            return Err("Project name cannot be empty.".into());
        }

        if name.len() > 64 {
            return Err("Project name cannot exceed 64 characters.".into());
        }

        if !is_valid_project_name(name) {
            return Err("Project name may only contain letters, numbers, \
                 spaces, '-' and '_'."
                .into());
        }

        Ok(())
    }

    fn validate_location(&self) -> Result<(), String> {
        if self.location.as_os_str().is_empty() {
            return Err("Project location cannot be empty.".into());
        }

        if !self.location.exists() {
            return Err("The selected project location does not exist.".into());
        }

        if !self.location.is_dir() {
            return Err("The selected project location is not a directory.".into());
        }

        Ok(())
    }
}

fn is_valid_project_name(name: &str) -> bool {
    name.chars()
        .all(|character| character.is_ascii_alphanumeric() || matches!(character, ' ' | '-' | '_'))
}
