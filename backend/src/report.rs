/// Report emitted while a backend operation is running.
///
/// Reports allow higher-level applications to display operation progress and
/// status without coupling the backend to a particular user interface.
#[derive(Debug, Clone)]
pub enum Report {
    /// Reports progress as a percentage from `0` to `100`.
    Progress(u8),

    /// Reports a human-readable status or log message.
    Log(String),
}
