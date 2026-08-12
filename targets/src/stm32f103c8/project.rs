use std::{
    fs::OpenOptions,
    io::{self, Write},
    path::{Path, PathBuf},
};

/// A generated STM32 project stored on disk.
#[derive(Debug)]
pub struct Project {
    directory: PathBuf,
    sources: Vec<PathBuf>,
}

impl Project {
    pub(super) fn from_generated(directory: PathBuf, sources: Vec<PathBuf>) -> Self {
        Self { directory, sources }
    }
    /// Adds a source file to the generated project's `src` directory.
    ///
    /// Supported extensions are `.c`, `.s`, and `.S`.
    ///
    /// # Errors
    ///
    /// Returns an error if `file_name` is not a simple supported filename,
    /// already exists, or cannot be written.
    pub fn add_source(&mut self, file_name: impl AsRef<Path>, source: &str) -> io::Result<()> {
        let file_name = file_name.as_ref();

        validate_source_name(file_name)?;

        let path = self.directory.join("src").join(file_name);

        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path)?;

        file.write_all(source.as_bytes())?;

        self.sources.push(path);

        Ok(())
    }
    /// Returns the root directory of the generated project.
    pub fn root(&self) -> &Path {
        self.directory.as_path()
    }
    /// Returns the source files belonging to the project.
    pub fn sources(&self) -> &[PathBuf] {
        &self.sources
    }
}

fn validate_source_name(file_name: &Path) -> io::Result<()> {
    const SUPPORTED_EXTENSIONS: &[&str] = &["c", "s", "S"];
    let is_single_component = file_name
        .parent()
        .is_some_and(|parent| parent.as_os_str().is_empty());

    let supported = file_name
        .extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| SUPPORTED_EXTENSIONS.contains(&extension));

    if !is_single_component || !supported {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!(
                "source must be a simple filename with one of these extensions: {SUPPORTED_EXTENSIONS:?}"
            ),
        ));
    }

    Ok(())
}
