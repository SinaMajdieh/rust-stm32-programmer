use firmware_targets::BuildArtifacts;
use serde::{Deserialize, Serialize};

use crate::project::stage::StageArtifact;

use super::revision::Revision;

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
