use std::{
    path::{Path, PathBuf},
    process::Command,
};

use super::{ArmGccConfig, BuildError, BuildStage, run};

#[derive(Clone, Copy)]
enum OutputFormat {
    IntelHex,
    Binary,
}

impl OutputFormat {
    fn argument(self) -> &'static str {
        match self {
            Self::IntelHex => "ihex",
            Self::Binary => "binary",
        }
    }

    fn extension(self) -> &'static str {
        match self {
            Self::IntelHex => "hex",
            Self::Binary => "bin",
        }
    }

    fn stage(self) -> BuildStage {
        match self {
            Self::IntelHex => BuildStage::ConvertToHex,
            Self::Binary => BuildStage::ConvertToBinary,
        }
    }
}

pub(super) fn to_hex(config: &ArmGccConfig, elf: &Path) -> Result<PathBuf, BuildError> {
    convert(config, elf, OutputFormat::IntelHex)
}

pub(super) fn to_binary(config: &ArmGccConfig, elf: &Path) -> Result<PathBuf, BuildError> {
    convert(config, elf, OutputFormat::Binary)
}

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
