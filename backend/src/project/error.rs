use std::{io, path::PathBuf};

use firmware_targets::{BuildError as FirmwareBuildError, programmer::ProgrammingError};
use thiserror::Error;

/// Unified error returned by the project module.
#[derive(Debug, Error)]
pub enum ProjectError {
    /// A project file operation failed.
    #[error(transparent)]
    Io(#[from] ProjectIoError),

    /// Source generation failed.
    #[error(transparent)]
    Generation(#[from] ::generation::GenerationError),

    /// Firmware building failed.
    #[error(transparent)]
    Build(#[from] ProjectBuildError),

    /// Firmware programming failed.
    #[error(transparent)]
    Programming(#[from] ProjectProgrammingError),
}

/// Errors produced while opening, creating, or saving a project.
#[derive(Debug, Error)]
pub enum ProjectIoError {
    /// The project directory could not be created.
    #[error("failed to create project directory `{path}`")]
    CreateDirectory {
        /// Directory that could not be created.
        path: PathBuf,

        /// Underlying operating-system error.
        #[source]
        source: io::Error,
    },

    /// The project file could not be read.
    #[error("failed to read project file `{path}`")]
    Read {
        /// File that could not be read.
        path: PathBuf,

        /// Underlying operating-system error.
        #[source]
        source: io::Error,
    },

    /// The project file could not be written.
    #[error("failed to write project file `{path}`")]
    Write {
        /// File that could not be written.
        path: PathBuf,

        /// Underlying operating-system error.
        #[source]
        source: io::Error,
    },

    /// The project root has not been configured.
    #[error("project root directory has not been configured")]
    MissingRoot,

    /// The supplied project path has no parent directory.
    #[error("project path `{path}` has no parent directory")]
    InvalidPath {
        /// Invalid project path.
        path: PathBuf,
    },

    /// The project file contains invalid TOML.
    #[error("project file contains invalid TOML: {0}")]
    Deserialize(#[from] toml::de::Error),

    /// The project could not be serialized as TOML.
    #[error("failed to serialize project as TOML: {0}")]
    Serialize(#[from] toml::ser::Error),
}

/// Errors produced while building firmware.
#[derive(Debug, Error)]
pub enum ProjectBuildError {
    /// Source code must be generated before the project can be built.
    #[error("cannot build project: generate source code first")]
    GenerationMissing,

    /// The selected target is not supported.
    #[error("cannot build project: target `{target}` is not supported")]
    TargetUnsupported {
        /// Unsupported target.
        target: String,
    },

    /// The firmware target failed while creating or compiling the project.
    #[error("failed to build firmware project for target `{target}`: {source}")]
    Firmware {
        /// Target used for the build.
        target: String,

        /// Underlying firmware build error.
        #[source]
        source: FirmwareBuildError,
    },
}

impl ProjectBuildError {
    pub(crate) fn firmware(target: impl Into<String>, source: FirmwareBuildError) -> Self {
        Self::Firmware {
            target: target.into(),
            source,
        }
    }
}

/// Errors produced while programming firmware.
#[derive(Debug, Error)]
pub enum ProjectProgrammingError {
    /// A build must exist before firmware can be programmed.
    #[error("cannot program firmware: build the project first")]
    BuildMissing,

    /// The existing build was produced from older generated source.
    #[error("cannot program firmware: the build is outdated; build the project again")]
    BuildOutdated,

    /// The selected target is not supported.
    #[error("cannot program firmware: target `{target}` is not supported")]
    TargetUnsupported {
        /// Unsupported target.
        target: String,
    },

    /// The target programmer failed.
    #[error("failed to program firmware for target `{target}`: {source}")]
    Firmware {
        /// Target used for programming.
        target: String,

        /// Underlying programming error.
        #[source]
        source: ProgrammingError,
    },
}
