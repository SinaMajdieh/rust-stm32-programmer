use std::{
    fs,
    path::{Path, PathBuf},
};

use ::generation::{GenerationError, GenerationOutput, LlmGenerator};
use firmware_targets::{TargetKind, TemplateKind};
use serde::{Deserialize, Serialize};

mod build;
mod error;
mod generation;
mod program;
mod revision;
mod stage;

pub use error::{ProjectBuildError, ProjectError, ProjectIoError, ProjectProgrammingError};
pub use generation::GenerationRequest;
pub use stage::Stage;

use crate::project::{
    build::{Build, BuildInput, BuildOutput},
    generation::Generation,
    program::{Program, ProgramInput, ProgramOutput},
    stage::StageArtifact,
};

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

    /// Generate firmware code and store the result in this project.
    ///
    /// This is the high-level API intended for non-GPUI callers.
    pub async fn generate(
        &mut self,
        request: GenerationRequest,
        generator: &LlmGenerator,
    ) -> Result<(), GenerationError> {
        let output = Self::generate_output(&request, generator).await?;

        self.apply_generation(request, output);

        Ok(())
    }

    /// Performs the asynchronous generation operation without
    /// borrowing project state.
    ///
    /// This is useful for UI layers where `Project` is stored inside
    /// an asynchronous/reactive state container such as `Entity<Project>`.
    pub async fn generate_output(
        request: &GenerationRequest,
        generator: &LlmGenerator,
    ) -> Result<GenerationOutput, GenerationError> {
        generator
            .generate(::generation::GenerationRequest::new(
                &request.model,
                &request.prompt,
                request.system_prompt.as_deref(),
            ))
            .await
    }

    /// Applies a successfully generated result to the project.
    ///
    /// This operation is synchronous so the project does not need to
    /// remain mutably borrowed across an `.await`.
    pub fn apply_generation(&mut self, request: GenerationRequest, output: GenerationOutput) {
        self.generation = Some(Generation::new(request, output));
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

    /// Prepares an owned snapshot for background compilation.
    pub fn prepare_build(&self) -> Result<BuildInput, ProjectBuildError> {
        let generation = self
            .generation
            .as_ref()
            .ok_or(ProjectBuildError::GenerationMissing)?;

        Ok(BuildInput {
            target: self.target.clone(),
            template: self.template,
            root: self.root.clone(),
            source: generation.code().to_owned(),
            revision: generation.revision(),
        })
    }

    /// Applies a successfully compiled build to the project.
    pub fn apply_build(&mut self, output: BuildOutput) {
        self.build = Some(output.build);
        self.upload = None;
    }

    /// Synchronous convenience API for CLI and other non-UI callers.
    pub fn build(&mut self) -> Result<(), ProjectBuildError> {
        let input = self.prepare_build()?;
        let output = input.compile()?;
        self.apply_build(output);

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

    /// Prepares an owned snapshot for background programming.
    pub fn prepare_program(&self) -> Result<ProgramInput, ProjectProgrammingError> {
        let build = self
            .build
            .as_ref()
            .ok_or(ProjectProgrammingError::BuildMissing)?;

        if !self.has_valid_build() {
            return Err(ProjectProgrammingError::BuildOutdated);
        }

        Ok(ProgramInput {
            target: self.target.clone(),
            elf: build.artifacts().elf().to_path_buf(),
            revision: build.revision(),
        })
    }

    /// Applies a successfully programmed result to the project.
    pub fn apply_program(&mut self, output: ProgramOutput) {
        self.upload = Some(Program::new(output.revision));
    }

    /// Synchronous convenience API for CLI and other non-GPUI callers.
    pub fn program(&mut self) -> Result<(), ProjectProgrammingError> {
        let input = self.prepare_program()?;
        let output = input.execute()?;
        self.apply_program(output);

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

    pub fn latest_stage(&self) -> Stage {
        if self.has_valid_build() {
            return Stage::Deploy;
        } else if self.has_generation() {
            return Stage::Build;
        } else {
            return Stage::Generation;
        }
    }
}
