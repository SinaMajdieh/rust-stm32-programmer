use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Identifies one successful stage execution.
///
/// Revisions have no semantic meaning. They only establish whether a
/// downstream stage was produced from the current upstream stage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Revision(Uuid);

impl Revision {
    pub(crate) fn new() -> Self {
        Self(Uuid::new_v4())
    }
}
