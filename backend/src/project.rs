use std::{
    fs,
    path::{Path, PathBuf},
};

use firmware_targets::{TargetKind, TemplateKind};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Project {
    #[serde(skip)]
    pub root: PathBuf,

    pub name: String,
    pub target: TargetKind,
    pub template: TemplateKind,
}

impl Project {
    pub const PROJECT_FILE: &str = "Project.toml";

    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_root(mut self, root: impl AsRef<Path>) -> Self {
        self.root = root.as_ref().to_path_buf();
        self
    }

    pub fn with_name(mut self, name: String) -> Self {
        self.name = name;
        self
    }

    pub fn with_target(mut self, target: TargetKind) -> Self {
        self.target = target;
        self
    }

    pub fn with_template(mut self, template: TemplateKind) -> Self {
        self.template = template;
        self
    }

    pub fn open(source: impl AsRef<Path>) -> Result<Self, ProjectError> {
        let source = source.as_ref();
        let contents = fs::read_to_string(source).map_err(ProjectError::Read)?;
        let root = source
            .parent()
            .ok_or(ProjectError::InvalidProjectPath)?
            .to_path_buf();

        Ok(Project {
            root,
            ..toml::from_str(&contents)?
        })
    }
    pub fn save(&self) -> Result<(), ProjectError> {
        let contents = toml::to_string_pretty(self)?;
        fs::write(&self.root.join(Self::PROJECT_FILE), contents).map_err(ProjectError::Write)?;
        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum ProjectError {
    #[error("Failed to read project file: {0}")]
    Read(#[source] std::io::Error),

    #[error("Failed to write project file: {0}")]
    Write(#[source] std::io::Error),

    #[error("Project file has no parent directory")]
    InvalidProjectPath,

    #[error("Invalid TOML: {0}")]
    TomlDeserialize(#[from] toml::de::Error),

    #[error("Failed to serialize: {0}")]
    TomlSerialize(#[from] toml::ser::Error),
}
