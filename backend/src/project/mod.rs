use std::{
    fs,
    path::{Path, PathBuf},
};

use firmware_targets::{TargetKind, TemplateKind};
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub type Result<T> = std::result::Result<T, ProjectError>;

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

    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
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

    pub fn open(source: impl AsRef<Path>) -> Result<Self> {
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

    pub fn save(&self) -> Result<()> {
        let contents = toml::to_string_pretty(self)?;
        fs::write(self.root.join(Self::PROJECT_FILE), contents).map_err(ProjectError::Write)?;
        Ok(())
    }

    pub fn create(&self) -> Result<()> {
        fs::create_dir_all(&self.root).map_err(ProjectError::CreateDirectory)?;
        self.save()
    }
}

#[derive(Debug, Error)]
pub enum ProjectError {
    #[error("Failed to create project directory: {0}")]
    CreateDirectory(#[source] std::io::Error),

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

impl ProjectError {
    pub fn user_message(&self) -> String {
        match self {
            Self::CreateDirectory(error) => match error.kind() {
                std::io::ErrorKind::PermissionDenied => {
                    "You don't have permission to create the project directory.".into()
                }
                std::io::ErrorKind::AlreadyExists => "The project directory already exists.".into(),
                _ => "The project directory could not be created.".into(),
            },
            Self::Read(error) => match error.kind() {
                std::io::ErrorKind::NotFound => "The project file could not be found.".into(),
                std::io::ErrorKind::PermissionDenied => {
                    "You don't have permission to read the project file.".into()
                }
                _ => "The project file could not be read.".into(),
            },

            Self::Write(error) => match error.kind() {
                std::io::ErrorKind::PermissionDenied => {
                    "You don't have permission to save the project file.".into()
                }
                _ => "The project file could not be saved.".into(),
            },

            Self::InvalidProjectPath => "The selected project path is invalid.".into(),

            Self::TomlDeserialize(_) => "The project file contains invalid configuration.".into(),

            Self::TomlSerialize(_) => "The project could not be saved.".into(),
        }
    }
}
