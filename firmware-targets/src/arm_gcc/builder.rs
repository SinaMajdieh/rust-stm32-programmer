use std::{
    error::Error,
    fmt, io,
    path::{Path, PathBuf},
    process::ExitStatus,
};

use super::{ArmGccConfig, compile, link, objcopy};

/// Files produced by a successful firmware build.
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

    /// Returns the ELF firmware image.
    pub fn elf(&self) -> &Path {
        &self.elf
    }

    /// Returns the Intel HEX firmware image.
    pub fn hex(&self) -> &Path {
        &self.hex
    }

    /// Returns the raw binary firmware image.
    pub fn binary(&self) -> &Path {
        &self.binary
    }

    /// Returns the linker map file.
    pub fn map(&self) -> &Path {
        &self.map
    }
}

/// The build stage that produced an error.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuildStage {
    /// Compiling a C or assembly source file.
    Compile,
    /// Linking object files into an ELF image.
    Link,
    /// Converting an ELF image to Intel HEX.
    ConvertToHex,
    /// Converting an ELF image to raw binary.
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
    /// A filesystem or process-launch error.
    Io(io::Error),

    /// A source file has an unsupported extension.
    UnsupportedSource {
        /// The unsupported source file.
        source: PathBuf,
    },

    /// An external tool returned a non-zero exit status.
    CommandFailed {
        /// The stage that failed.
        stage: BuildStage,

        /// The file being produced or compiled.
        path: PathBuf,

        /// The tool's exit status.
        status: ExitStatus,

        /// Compiler diagnostics written to standard error.
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

/// Executes an ARM GCC firmware build.
pub(crate) struct ArmGcc<'a> {
    config: &'a ArmGccConfig,
    project_root: &'a Path,
    sources: &'a [PathBuf],
}

impl<'a> ArmGcc<'a> {
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

    pub(crate) fn build(&self) -> Result<BuildArtifacts, BuildError> {
        self.config.validate_project_layout(self.project_root)?;

        let objects = compile::sources(self.config, self.project_root, self.sources)?;
        let linked = link::objects(self.config, self.project_root, &objects)?;

        let hex = objcopy::to_hex(self.config, &linked.elf)?;
        let binary = objcopy::to_binary(self.config, &linked.elf)?;

        Ok(BuildArtifacts::new(linked.elf, hex, binary, linked.map))
    }
}
