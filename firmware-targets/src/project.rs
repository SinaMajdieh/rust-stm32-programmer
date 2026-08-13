use crate::{ArmGccConfig, BuildArtifacts, BuildError, arm_gcc::ArmGcc};

use std::{
    fs::OpenOptions,
    io::{self, Write},
    path::{Path, PathBuf},
};

/// A generated embedded project stored on disk.
#[derive(Debug)]
pub struct Project {
    directory: PathBuf,
    sources: Vec<PathBuf>,
    build_config: ArmGccConfig,
}

impl Project {
    const SUPPORTED_SOURCE_EXTENSIONS: &[&str] = &["c", "s", "S"];

    /// Creates a project from files already generated on disk.
    ///
    /// This is the extension point for contributors adding target templates.
    /// Every source path, include directory, and linker script must exist.
    ///
    /// # Errors
    ///
    /// Returns an error if the project layout is incomplete.
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
    /// Supported extensions are `.c`, `.s`, and `.S`.
    ///
    /// # Errors
    ///
    /// Returns an error if the file name is invalid, the file already exists,
    /// or the source cannot be written.
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

    /// Compiles the project and produces ELF, HEX, BIN, and MAP files.
    ///
    /// # Errors
    ///
    /// Returns an error if the Arm GNU toolchain cannot run or any build stage
    /// fails.
    pub fn compile(&self) -> Result<BuildArtifacts, BuildError> {
        ArmGcc::new(&self.build_config, &self.directory, &self.sources).build()
    }

    /// Returns the project's root directory.
    pub fn root(&self) -> &Path {
        &self.directory
    }

    /// Returns every source file included in the build.
    pub fn sources(&self) -> &[PathBuf] {
        &self.sources
    }
}

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
