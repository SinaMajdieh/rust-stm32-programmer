use std::{
    path::{Path, PathBuf},
    process::Command,
};

use super::{ArmGccConfig, BuildError, BuildStage, run};

/// Output format supported by the firmware image conversion stage.
#[derive(Clone, Copy)]
enum OutputFormat {
    /// Intel HEX firmware image.
    IntelHex,

    /// Raw binary firmware image.
    Binary,
}

impl OutputFormat {
    /// Returns the `objcopy` argument corresponding to this output format.
    fn argument(self) -> &'static str {
        match self {
            Self::IntelHex => "ihex",
            Self::Binary => "binary",
        }
    }

    /// Returns the file extension used for this output format.
    fn extension(self) -> &'static str {
        match self {
            Self::IntelHex => "hex",
            Self::Binary => "bin",
        }
    }

    /// Returns the build stage associated with this output format.
    ///
    /// Keeping the stage information with the format ensures that conversion
    /// failures are reported with the same stage that produced the output.
    fn stage(self) -> BuildStage {
        match self {
            Self::IntelHex => BuildStage::ConvertToHex,
            Self::Binary => BuildStage::ConvertToBinary,
        }
    }
}

/// Converts an ELF firmware image into an Intel HEX image.
///
/// The output is written next to the ELF file using the `.hex` extension.
pub(super) fn to_hex(config: &ArmGccConfig, elf: &Path) -> Result<PathBuf, BuildError> {
    convert(config, elf, OutputFormat::IntelHex)
}

/// Converts an ELF firmware image into a raw binary image.
///
/// The output is written next to the ELF file using the `.bin` extension.
pub(super) fn to_binary(config: &ArmGccConfig, elf: &Path) -> Result<PathBuf, BuildError> {
    convert(config, elf, OutputFormat::Binary)
}

/// Runs `objcopy` to convert an ELF image into the requested firmware format.
fn convert(config: &ArmGccConfig, elf: &Path, format: OutputFormat) -> Result<PathBuf, BuildError> {
    let output = elf.with_extension(format.extension());

    let mut command = Command::new(config.objcopy());

    command
        .arg("-O")
        .arg(format.argument())
        .arg(elf)
        .arg(&output);

    run(&mut command, format.stage(), &output)?;

    Ok(output)
}

#[cfg(test)]
#[path = "objcopy_tests.rs"]
mod tests;
