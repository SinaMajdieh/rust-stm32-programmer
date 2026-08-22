use std::{
    ffi::OsString,
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use super::{ArmGccConfig, BuildError, BuildStage, run};

/// Output produced by the linker stage.
pub(super) struct LinkOutput {
    /// The linked ELF firmware image.
    pub(super) elf: PathBuf,

    /// The linker map containing symbol and memory-layout information.
    pub(super) map: PathBuf,
}

/// Links compiled object files into the firmware ELF image.
///
/// The linker uses the project's configured linker script and enables section
/// garbage collection. The resulting ELF image and linker map are written to
/// the project's `build` directory.
pub(super) fn objects(
    config: &ArmGccConfig,
    project_root: &Path,
    objects: &[PathBuf],
) -> Result<LinkOutput, BuildError> {
    let build_directory = project_root.join("build");
    fs::create_dir_all(&build_directory)?;

    let elf = build_directory.join("firmware.elf");
    let map = build_directory.join("firmware.map");

    let mut command = command(config, project_root, objects, &elf, &map);

    run(&mut command, BuildStage::Link, &elf)?;

    Ok(LinkOutput { elf, map })
}

/// Constructs the linker invocation.
///
/// Linking is performed through the configured ARM GCC compiler so that GCC
/// supplies the appropriate linker and runtime configuration. The project's
/// linker script determines the target memory layout, while section garbage
/// collection removes unused sections from the final image.
fn command(
    config: &ArmGccConfig,
    project_root: &Path,
    objects: &[PathBuf],
    elf: &Path,
    map: &Path,
) -> Command {
    let linker_script = project_root.join(config.linker_script());

    let mut map_argument = OsString::from("-Wl,-Map=");
    map_argument.push(map);

    let mut command = Command::new(config.compiler());

    command
        .arg(format!("-mcpu={}", config.cpu()))
        .arg("-mthumb")
        .arg("--specs=nano.specs")
        .arg("--specs=nosys.specs")
        .arg("-T")
        .arg(linker_script)
        .arg("-Wl,--gc-sections")
        .arg(map_argument)
        .args(objects)
        .arg("-o")
        .arg(elf);

    command
}

#[cfg(test)]
#[path = "link_tests.rs"]
mod tests;
