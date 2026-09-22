use std::{
    fs,
    path::{Path, PathBuf},
};

use ::generation::LlmGenerator;
use firmware_targets::{BuildError, Target, TargetKind, TemplateKind, create_target};
use serde::{Deserialize, Serialize};

mod build;
mod error;
mod generation;
mod program;
mod revision;
mod stage;

pub use error::{ProjectBuildError, ProjectError, ProjectIoError, ProjectProgrammingError};
pub use generation::GenerationRequest;

use crate::project::{build::Build, generation::Generation, program::Program, stage::Stage};

/// A firmware project and the state produced by its pipeline stages.
///
/// The project pipeline is:
///
///
/// generation -> build -> programming
///
#[derive(Debug, Deserialize, Serialize, Default)]

pub struct Project {
    /// Directory containing the project configuration and generated files.
    #[serde(skip)]
    pub root: PathBuf,

    /// Human-readable project name.
    pub name: String,

    /// Firmware target used to generate, build, and program the project.
    pub target: TargetKind,

    /// Template used to generate the firmware project.
    pub template: TemplateKind,

    generation: Option<Generation>,
    build: Option<Build>,
    upload: Option<Program>,
}

impl Project {
    /// Name of the project configuration file.
    pub const PROJECT_FILE: &str = "Project.toml";

    /// Creates an empty project.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the project root directory.
    pub fn with_root(mut self, root: impl AsRef<Path>) -> Self {
        self.root = root.as_ref().to_path_buf();
        self
    }

    /// Sets the project name.
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    /// Sets the firmware target.
    pub fn with_target(mut self, target: TargetKind) -> Self {
        self.target = target;
        self
    }

    /// Sets the project template.
    pub fn with_template(mut self, template: TemplateKind) -> Self {
        self.template = template;
        self
    }

    /// Opens a project from a `Project.toml` file.
    pub fn open(path: impl AsRef<Path>) -> Result<Self, ProjectIoError> {
        let path = path.as_ref();

        let contents = fs::read_to_string(path).map_err(|source| ProjectIoError::Read {
            path: path.to_path_buf(),
            source,
        })?;

        let root = path
            .parent()
            .filter(|parent| !parent.as_os_str().is_empty())
            .ok_or_else(|| ProjectIoError::InvalidPath {
                path: path.to_path_buf(),
            })?
            .to_path_buf();

        let mut project: Self = toml::from_str(&contents)?;
        project.root = root;

        Ok(project)
    }

    /// Opens a project from a folder.
    pub fn open_from_dir(path: impl AsRef<Path>) -> Result<Self, ProjectIoError> {
        let path = path.as_ref();
        Self::open(path.join(Self::PROJECT_FILE))
    }

    /// Saves the project to [`Self::PROJECT_FILE`].
    pub fn save(&self) -> Result<(), ProjectIoError> {
        self.ensure_root()?;

        let path = self.project_file();
        let contents = toml::to_string_pretty(self)?;

        fs::write(&path, contents).map_err(|source| ProjectIoError::Write { path, source })?;

        Ok(())
    }

    /// Creates the project directory and saves the project configuration.
    pub fn create(&self) -> Result<(), ProjectIoError> {
        self.ensure_root()?;

        fs::create_dir_all(&self.root).map_err(|source| ProjectIoError::CreateDirectory {
            path: self.root.clone(),
            source,
        })?;

        self.save()
    }

    /// Generates firmware source code using the supplied language model.
    ///
    /// Generating new source invalidates the existing build and programming
    /// result because both were produced from older source code.
    pub async fn generate(
        &mut self,
        request: GenerationRequest,
        generator: &LlmGenerator,
    ) -> Result<(), ::generation::GenerationError> {
        let output = generator
            .generate(::generation::GenerationRequest::new(
                &request.model,
                &request.prompt,
                request.system_prompt.as_deref(),
            ))
            .await?;

        self.generation = Some(Generation::new(request, output));

        Ok(())
    }

    /// Returns the generated source code, if available.
    pub fn generated_code(&self) -> Option<&str> {
        self.generation.as_ref().map(|generation| generation.code())
    }

    pub fn generation(&self) -> Option<&Generation> {
        self.generation.as_ref()
    }

    /// Returns whether generated source code is available.
    pub fn has_generation(&self) -> bool {
        self.generation.is_some()
    }

    /// Builds the currently generated firmware source.
    ///
    /// A successful build invalidates the previous programming result.
    pub fn build(&mut self) -> Result<(), ProjectBuildError> {
        let generation = self
            .generation
            .as_ref()
            .ok_or(ProjectBuildError::GenerationMissing)?;

        let target_name = format!("{:?}", self.target);

        let target =
            create_target(&self.target).ok_or_else(|| ProjectBuildError::TargetUnsupported {
                target: target_name.clone(),
            })?;

        let mut generated_project = target
            .generate_project(self.template, &self.root)
            .map_err(|source| ProjectBuildError::firmware(&target_name, BuildError::Io(source)))?;

        generated_project
            .write_source("src/main.c", generation.code())
            .map_err(|source| ProjectBuildError::firmware(&target_name, BuildError::Io(source)))?;

        let artifacts = generated_project.compile().map_err(|source| {
            ProjectBuildError::firmware(&target_name, BuildError::Compile(source))
        })?;

        self.build = Some(Build::new(generation.revision(), artifacts));

        Ok(())
    }

    /// Returns whether the current build belongs to the current generation.
    pub fn has_valid_build(&self) -> bool {
        let Some(generation) = self.generation.as_ref() else {
            return false;
        };

        self.build
            .as_ref()
            .is_some_and(|build| build.is_valid_for(generation.revision()))
    }

    /// Programs the current firmware build onto the target device.
    pub fn program(&mut self) -> Result<(), ProjectProgrammingError> {
        let build = self
            .build
            .as_ref()
            .ok_or(ProjectProgrammingError::BuildMissing)?;

        if !self.has_valid_build() {
            return Err(ProjectProgrammingError::BuildOutdated);
        }

        let target_name = format!("{:?}", self.target);

        let target = create_target(&self.target).ok_or_else(|| {
            ProjectProgrammingError::TargetUnsupported {
                target: target_name.clone(),
            }
        })?;

        target.program(build.artifacts().elf()).map_err(|source| {
            ProjectProgrammingError::Firmware {
                target: target_name,
                source,
            }
        })?;

        self.upload = Some(Program::new(build.revision()));

        Ok(())
    }

    /// Returns whether the current programming result belongs to the current
    /// build.
    pub fn has_valid_program(&self) -> bool {
        let Some(build) = self.build.as_ref() else {
            return false;
        };

        self.upload
            .as_ref()
            .is_some_and(|upload| self.has_valid_build() && upload.is_valid_for(build.revision()))
    }

    /// Returns the path to the project configuration file.
    pub fn project_file(&self) -> PathBuf {
        self.root.join(Self::PROJECT_FILE)
    }

    fn ensure_root(&self) -> Result<(), ProjectIoError> {
        if self.root.as_os_str().is_empty() {
            return Err(ProjectIoError::MissingRoot);
        }

        Ok(())
    }
}
