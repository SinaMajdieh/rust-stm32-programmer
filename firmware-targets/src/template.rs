use crate::{ArmGccConfig, Project};
use std::{fs, io, path::PathBuf};

/// Defines the files and build configuration used to generate a target project.
///
/// A project template provides three pieces of information:
///
/// - the embedded directory containing the template files;
/// - the source files that are built into the generated project; and
/// - the ARM GCC configuration used to build the project.
///
/// Implementors can then use the provided [`ProjectTemplate::generate`] method
/// to materialize the template as a [`Project`].
pub trait ProjectTemplate {
    /// Returns the embedded directory containing the template files.
    ///
    /// The returned directory is extracted into the generated project's root
    /// directory by [`ProjectTemplate::generate`].
    fn template() -> &'static include_dir::Dir<'static>;

    /// Returns the project-relative paths of source files supplied by the
    /// template.
    ///
    /// These paths are resolved relative to the generated project root and
    /// passed to [`Project::from_generated`].
    fn built_in_sources() -> &'static [&'static str];

    /// Creates the ARM GCC configuration used to build the generated project.
    fn build_config() -> ArmGccConfig;

    /// Generates a project from this template at `output`.
    ///
    /// The template files are embedded into the crate and extracted into the
    /// newly created output directory. Parent directories are created
    /// automatically when necessary.
    ///
    /// The output directory itself must not already exist.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    ///
    /// - a required parent directory cannot be created;
    /// - the output directory cannot be created; or
    /// - an embedded template file cannot be extracted.
    fn generate(output: impl Into<PathBuf>) -> io::Result<Project> {
        let directory = output.into();

        if let Some(parent) = directory
            .parent()
            .filter(|parent| !parent.as_os_str().is_empty())
        {
            fs::create_dir_all(parent)?;
        }

        fs::create_dir(&directory)?;
        Self::template().extract(&directory)?;

        let sources = Self::built_in_sources()
            .iter()
            .map(|source| directory.join(source))
            .collect();

        Project::from_generated(directory, sources, Self::build_config())
    }
}
