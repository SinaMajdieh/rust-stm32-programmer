use serde::{Deserialize, Serialize};

use super::{revision::Revision, stage::Stage};

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

impl Stage for Program {
    fn revision(&self) -> Revision {
        self.revision
    }

    fn source_revision(&self) -> Option<Revision> {
        Some(self.source)
    }
}
