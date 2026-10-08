use super::revision::Revision;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
#[repr(usize)]
pub enum Stage {
    #[default]
    Generation = 0,
    Build = 1,
    Deploy = 2,
}

impl Stage {
    pub const ALL: [Self; 3] = [Self::Generation, Self::Build, Self::Deploy];

    pub const fn index(self) -> usize {
        self as usize
    }
}

impl TryFrom<usize> for Stage {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Generation),
            1 => Ok(Self::Build),
            2 => Ok(Self::Deploy),
            _ => Err(()),
        }
    }
}

/// An artifact produced by a project pipeline stage.
pub trait StageArtifact {
    /// Revision produced by this stage.
    fn revision(&self) -> Revision;

    /// Revision consumed by this stage, if any.
    fn source_revision(&self) -> Option<Revision>;

    /// Returns whether this artifact was produced from the supplied revision.
    fn is_valid_for(&self, source: Revision) -> bool {
        self.source_revision() == Some(source)
    }
}
