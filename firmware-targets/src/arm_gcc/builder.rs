use std::{
    error::Error,
    fmt, io,
    path::{Path, PathBuf},
    process::ExitStatus,
};

use super::{ArmGccConfig, compile, link, objcopy};

/// Files produced by a successful firmware build.
///
/// The artifacts consist of the linked ELF image, Intel HEX and raw binary
/// firmware images, and the linker map file.
#[derive(Debug, Clone)]
pub struct BuildArtifacts {
    elf: PathBuf,
    hex: PathBuf,
    binary: PathBuf,
    map: PathBuf,
}

impl BuildArtifacts {
    pub(crate) fn new(elf: PathBuf, hex: PathBuf, binary: PathBuf, map: PathBuf) -> Self {
        Self {
            elf,
            hex,
            binary,
            map,
        }
    }

    /// Returns the path to the ELF firmware image.
    pub fn elf(&self) -> &Path {
        &self.elf
    }

    /// Returns the path to the Intel HEX firmware image.
    pub fn hex(&self) -> &Path {
        &self.hex
    }

    /// Returns the path to the raw binary firmware image.
    pub fn binary(&self) -> &Path {
        &self.binary
    }

    /// Returns the path to the linker map file.
    pub fn map(&self) -> &Path {
        &self.map
    }
}

/// Identifies the build stage at which an error occurred.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuildStage {
    /// Compiling a C or assembly source file into an object file.
    Compile,

    /// Linking object files into an ELF firmware image.
    Link,

    /// Converting the ELF image into an Intel HEX firmware image.
    ConvertToHex,

    /// Converting the ELF image into a raw binary firmware image.
    ConvertToBinary,
}

impl fmt::Display for BuildStage {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::Compile => "compile",
            Self::Link => "link",
            Self::ConvertToHex => "convert to Intel HEX",
            Self::ConvertToBinary => "convert to binary",
        };

        formatter.write_str(name)
    }
}

/// An error produced while building firmware.
#[derive(Debug)]
pub enum BuildError {
    /// A filesystem operation or external process could not be started.
    Io(io::Error),

    /// A source file has an extension that is not supported by the ARM GCC
    /// build pipeline.
    UnsupportedSource {
        /// The unsupported source file.
        source: PathBuf,
    },

    /// An external tool exited unsuccessfully during a build stage.
    CommandFailed {
        /// The stage at which the command failed.
        stage: BuildStage,

        /// The source or output file associated with the failed command.
        path: PathBuf,

        /// The exit status returned by the external tool.
        status: ExitStatus,

        /// Diagnostics written by the tool to standard error.
        diagnostics: String,
    },
}

impl fmt::Display for BuildError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(error) => error.fmt(formatter),

            Self::UnsupportedSource { source } => write!(
                formatter,
                "unsupported source file {}; supported extensions are .c, .s, and .S",
                source.display()
            ),

            Self::CommandFailed {
                stage,
                path,
                status,
                diagnostics,
            } => {
                write!(
                    formatter,
                    "{stage} stage failed for {} ({status})",
                    path.display()
                )?;

                if !diagnostics.trim().is_empty() {
                    write!(formatter, ":\n{}", diagnostics.trim_end())?;
                }

                Ok(())
            }
        }
    }
}

impl Error for BuildError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Io(error) => Some(error),
            Self::UnsupportedSource { .. } | Self::CommandFailed { .. } => None,
        }
    }
}

impl From<io::Error> for BuildError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}

/// Coordinates the ARM GCC firmware build pipeline.
///
/// `ArmGcc` owns no build state itself; it borrows the configuration, project
/// root, and source list supplied by the caller. A build proceeds through the
/// following stages:
///
/// 1. Validate the project layout.
/// 2. Compile each source file into an object file.
/// 3. Link the object files into an ELF image and linker map.
/// 4. Convert the ELF image into Intel HEX and raw binary images.
pub(crate) struct ArmGcc<'a> {
    config: &'a ArmGccConfig,
    project_root: &'a Path,
    sources: &'a [PathBuf],
}

impl<'a> ArmGcc<'a> {
    /// Creates a build coordinator for the given project.
    pub(crate) fn new(
        config: &'a ArmGccConfig,
        project_root: &'a Path,
        sources: &'a [PathBuf],
    ) -> Self {
        Self {
            config,
            project_root,
            sources,
        }
    }

    /// Runs the complete ARM GCC firmware build pipeline.
    ///
    /// The build artifacts are written to the project's `build` directory.
    /// Compilation, linking, and image conversion are performed by the
    /// corresponding stages of the build pipeline.
    pub(crate) fn build(&self) -> Result<BuildArtifacts, BuildError> {
        self.config.validate_project_layout(self.project_root)?;

        let objects = compile::sources(self.config, self.project_root, self.sources)?;

        let linked = link::objects(self.config, self.project_root, &objects)?;

        let hex = objcopy::to_hex(self.config, &linked.elf)?;
        let binary = objcopy::to_binary(self.config, &linked.elf)?;

        Ok(BuildArtifacts::new(linked.elf, hex, binary, linked.map))
    }
}

#[cfg(test)]
#[path = "builder_tests.rs"]
mod tests;
