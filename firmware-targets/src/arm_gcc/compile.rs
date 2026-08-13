use std::{
    ffi::{OsStr, OsString},
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use super::{ArmGccConfig, BuildError, BuildStage, run};

pub(super) fn sources(
    config: &ArmGccConfig,
    project_root: &Path,
    sources: &[PathBuf],
) -> Result<Vec<PathBuf>, BuildError> {
    let build_directory = project_root.join("build");
    fs::create_dir_all(&build_directory)?;

    sources
        .iter()
        .map(|source| compile_source(config, project_root, source, &build_directory))
        .collect()
}

fn compile_source(
    config: &ArmGccConfig,
    project_root: &Path,
    source: &Path,
    build_directory: &Path,
) -> Result<PathBuf, BuildError> {
    if !is_supported_source(source) {
        return Err(BuildError::UnsupportedSource {
            source: source.to_path_buf(),
        });
    }

    let object = object_path(source, build_directory)?;

    let mut command = command(config, project_root, source, &object);
    run(&mut command, BuildStage::Compile, source)?;

    Ok(object)
}

fn command(config: &ArmGccConfig, project_root: &Path, source: &Path, object: &Path) -> Command {
    let mut command = Command::new(config.compiler());

    command
        .arg(format!("-mcpu={}", config.cpu()))
        .arg("-mthumb")
        .arg("-ffunction-sections")
        .arg("-fdata-sections")
        .arg("-Og")
        .arg("-g3");

    if source.extension() == Some(OsStr::new("c")) {
        command.arg("-std=c11");
    }

    for define in config.defines() {
        command.arg(format!("-D{define}"));
    }

    for include_dir in config.include_dirs() {
        command.arg("-I").arg(project_root.join(include_dir));
    }

    command.arg("-c").arg(source).arg("-o").arg(object);

    command
}

fn object_path(source: &Path, build_directory: &Path) -> Result<PathBuf, BuildError> {
    let file_name = source
        .file_name()
        .ok_or_else(|| BuildError::UnsupportedSource {
            source: source.to_path_buf(),
        })?;

    let mut object_name = OsString::from(file_name);
    object_name.push(".o");

    Ok(build_directory.join(object_name))
}

fn is_supported_source(source: &Path) -> bool {
    matches!(
        source.extension(),
        Some(extension)
            if extension == OsStr::new("c")
                || extension == OsStr::new("s")
                || extension == OsStr::new("S")
    )
}
