use crate::{ArmGccConfig, BuildArtifacts, BuildError, arm_gcc::ArmGcc};

use std::{
    fs::OpenOptions,
    io::{self, Write},
    path::{Path, PathBuf},
};

/// A generated embedded project stored on disk.
///
/// A `Project` represents a project directory together with the source files
/// that belong to the build and the toolchain configuration used to compile
/// them.
///
/// Projects are normally created from a [`crate::ProjectTemplate`] using its
/// [`crate::ProjectTemplate::generate`] method. Additional application source
/// files can then be added with [`Project::add_source`] before compiling the
/// project with [`Project::compile`].
#[derive(Debug)]
pub struct Project {
    directory: PathBuf,
    sources: Vec<PathBuf>,
    build_config: ArmGccConfig,
}

impl Project {
    /// Source file extensions accepted by [`Project::add_source`].
    const SUPPORTED_SOURCE_EXTENSIONS: &[&str] = &["c", "s", "S"];

    /// Creates a project from files already generated on disk.
    ///
    /// This is the lower-level entry point used by target templates after
    /// extracting their embedded files. It can also be used by additional
    /// target implementations to construct a [`Project`] from an existing
    /// project layout.
    ///
    /// The project root, every source file, all configured include directories,
    /// and the configured linker script must already exist.
    ///
    /// # Errors
    ///
    /// Returns an [`io::Error`] if:
    ///
    /// - `directory` does not exist or is not a directory;
    /// - a source file does not exist or is not a regular file;
    /// - a configured include directory does not exist; or
    /// - the configured linker script does not exist.
    pub fn from_generated(
        directory: impl Into<PathBuf>,
        sources: Vec<PathBuf>,
        build_config: ArmGccConfig,
    ) -> io::Result<Self> {
        let directory = directory.into();

        if !directory.is_dir() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("project directory does not exist: {}", directory.display()),
            ));
        }

        for source in &sources {
            if !source.is_file() {
                return Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    format!("project source does not exist: {}", source.display()),
                ));
            }
        }

        build_config.validate_project_layout(&directory)?;

        Ok(Self {
            directory,
            sources,
            build_config,
        })
    }

    /// Adds a C or assembly source file to the project's `src` directory.
    ///
    /// `file_name` must be a simple filename rather than a path. The supported
    /// extensions are `.c`, `.s`, and `.S`.
    ///
    /// The source is written to disk immediately and becomes part of
    /// subsequent builds.
    ///
    /// # Errors
    ///
    /// Returns an [`io::Error`] if:
    ///
    /// - `file_name` is not a simple filename;
    /// - the file extension is unsupported;
    /// - a file with the same name already exists; or
    /// - the source cannot be written to disk.
    pub fn add_source(
        &mut self,
        file_name: impl AsRef<Path>,
        source: &str,
    ) -> io::Result<&mut Self> {
        let file_name = file_name.as_ref();

        validate_source_name(file_name, Self::SUPPORTED_SOURCE_EXTENSIONS)?;

        let path = self.directory.join("src").join(file_name);

        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path)?;

        file.write_all(source.as_bytes())?;
        self.sources.push(path);

        Ok(self)
    }

    /// Compiles the project using its configured ARM GNU toolchain.
    ///
    /// A successful build produces ELF, Intel HEX, raw binary, and linker map
    /// files in the project's `build` directory.
    ///
    /// # Errors
    ///
    /// Returns a [`BuildError`] if:
    ///
    /// - the project layout is invalid;
    /// - a source file cannot be compiled;
    /// - the linker fails; or
    /// - an output image cannot be generated.
    pub fn compile(&self) -> Result<BuildArtifacts, BuildError> {
        ArmGcc::new(&self.build_config, &self.directory, &self.sources).build()
    }

    /// Returns the root directory of the project.
    pub fn root(&self) -> &Path {
        &self.directory
    }

    /// Returns the source files included in the build.
    ///
    /// The returned paths are the paths of the source files on disk.
    pub fn sources(&self) -> &[PathBuf] {
        &self.sources
    }
}

/// Validates the filename supplied to [`Project::add_source`].
///
/// Source names must consist of a single path component and use one of the
/// extensions supported by the ARM GCC compilation stage.
fn validate_source_name(file_name: &Path, supported_extensions: &[&str]) -> io::Result<()> {
    let is_single_component = file_name
        .parent()
        .is_some_and(|parent| parent.as_os_str().is_empty());

    let supported_extension = file_name
        .extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| supported_extensions.contains(&extension));

    if !is_single_component || !supported_extension {
        let extensions = supported_extensions
            .iter()
            .map(|extension| format!(".{extension}"))
            .collect::<Vec<_>>()
            .join(", ");

        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("source must be a simple filename ending in {extensions}"),
        ));
    }

    Ok(())
}

#[cfg(test)]
#[path = "project_tests.rs"]
mod tests;
