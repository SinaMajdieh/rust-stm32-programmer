use std::{
    ffi::OsString,
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use super::{ArmGccConfig, BuildError, BuildStage, run};

pub(super) struct LinkOutput {
    pub(super) elf: PathBuf,
    pub(super) map: PathBuf,
}

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
