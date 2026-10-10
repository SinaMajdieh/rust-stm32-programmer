use std::path::PathBuf;

use firmware_targets::{Target, TargetKind, create_target};
use serde::{Deserialize, Serialize};

use crate::project::{ProjectProgrammingError, stage::StageArtifact};

use super::revision::Revision;

/// Owned input required to program a firmware build.
pub struct ProgramInput {
    pub target: TargetKind,
    pub elf: PathBuf,
    pub revision: Revision,
}

/// A successfully programmed firmware build.
pub struct ProgramOutput {
    pub revision: Revision,
}

impl ProgramInput {
    /// Programs the firmware without borrowing the project state.
    pub fn execute(self) -> Result<ProgramOutput, ProjectProgrammingError> {
        let target_name = format!("{:?}", self.target);

        let target = create_target(&self.target).ok_or_else(|| {
            ProjectProgrammingError::TargetUnsupported {
                target: target_name.clone(),
            }
        })?;

        target
            .program(&self.elf)
            .map_err(|source| ProjectProgrammingError::Firmware {
                target: target_name,
                source,
            })?;

        Ok(ProgramOutput {
            revision: self.revision,
        })
    }
}

/// Successful firmware programming result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Program {
    source: Revision,
    revision: Revision,
}

impl Program {
    pub(crate) fn new(source: Revision) -> Self {
        Self {
            source,
            revision: Revision::new(),
        }
    }
}

impl StageArtifact for Program {
    fn revision(&self) -> Revision {
        self.revision
    }

    fn source_revision(&self) -> Option<Revision> {
        Some(self.source)
    }
}
