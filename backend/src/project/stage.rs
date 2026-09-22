use super::revision::Revision;

/// Common behavior for project pipeline stages.
pub trait Stage {
    /// Revision produced by this stage.
    fn revision(&self) -> Revision;

    /// Revision of the stage's input.
    fn source_revision(&self) -> Option<Revision>;

    /// Returns whether this stage was produced from the supplied input.
    fn is_valid_for(&self, source: Revision) -> bool {
        self.source_revision() == Some(source)
    }
}
