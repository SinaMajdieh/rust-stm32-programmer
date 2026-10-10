use std::path::PathBuf;

use firmware_targets::{
    BuildArtifacts, BuildError, Target, TargetKind, TemplateKind, create_target,
};
use serde::{Deserialize, Serialize};

use crate::project::{ProjectBuildError, stage::StageArtifact};

use super::revision::Revision;

/// Owned input required to compile generated firmware.
pub struct BuildInput {
    pub target: TargetKind,
    pub template: TemplateKind,
    pub root: PathBuf,
    pub source: String,
    pub revision: Revision,
}

/// Successfully compiled firmware, ready to apply to a project.
pub struct BuildOutput {
    pub build: Build,
}

impl BuildInput {
    /// Compiles firmware without borrowing the project state.
    pub fn compile(self) -> Result<BuildOutput, ProjectBuildError> {
        let target_name = format!("{:?}", self.target);

        let target =
            create_target(&self.target).ok_or_else(|| ProjectBuildError::TargetUnsupported {
                target: target_name.clone(),
            })?;

        let mut generated_project = target
            .generate_project(self.template, &self.root)
            .map_err(|source| ProjectBuildError::firmware(&target_name, BuildError::Io(source)))?;

        generated_project
            .write_source("src/main.c", &self.source)
            .map_err(|source| ProjectBuildError::firmware(&target_name, BuildError::Io(source)))?;

        let artifacts = generated_project.compile().map_err(|source| {
            ProjectBuildError::firmware(&target_name, BuildError::Compile(source))
        })?;

        Ok(BuildOutput {
            build: Build::new(self.revision, artifacts),
        })
    }
}

/// Firmware build output.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Build {
    source: Revision,
    revision: Revision,
    artifacts: BuildArtifacts,
}

impl Build {
    pub(crate) fn new(source: Revision, artifacts: BuildArtifacts) -> Self {
        Self {
            source,
            revision: Revision::new(),
            artifacts,
        }
    }

    pub(crate) fn artifacts(&self) -> &BuildArtifacts {
        &self.artifacts
    }
}

impl StageArtifact for Build {
    fn revision(&self) -> Revision {
        self.revision
    }

    fn source_revision(&self) -> Option<Revision> {
        Some(self.source)
    }
}
