use super::*;

use std::{fs, io};

use tempfile::tempdir;

fn config() -> ArmGccConfig {
    ArmGccConfig::new(
        "test-gcc",
        "test-objcopy",
        "cortex-m3",
        ["FIRST", "SECOND=2"],
        ["include", "vendor/include"],
        "memory.ld",
    )
}

#[test]
fn constructor_and_accessors_preserve_configuration() {
    let config = config();

    assert_eq!(config.compiler(), Path::new("test-gcc"));
    assert_eq!(config.objcopy(), Path::new("test-objcopy"));
    assert_eq!(config.cpu(), "cortex-m3");
    assert_eq!(config.defines(), ["FIRST", "SECOND=2"]);
    assert_eq!(
        config.include_dirs(),
        [PathBuf::from("include"), PathBuf::from("vendor/include")]
    );
    assert_eq!(config.linker_script(), Path::new("memory.ld"));
}

#[test]
fn layout_validation_accepts_existing_include_directories_and_linker_script() -> io::Result<()> {
    let directory = tempdir()?;
    fs::create_dir_all(directory.path().join("include"))?;
    fs::create_dir_all(directory.path().join("vendor/include"))?;
    fs::write(directory.path().join("memory.ld"), "MEMORY {}")?;

    config().validate_project_layout(directory.path())
}

#[test]
fn layout_validation_reports_the_first_missing_include_directory() -> io::Result<()> {
    let directory = tempdir()?;
    let error = config()
        .validate_project_layout(directory.path())
        .expect_err("a missing include directory must be rejected");

    assert_eq!(error.kind(), io::ErrorKind::NotFound);
    assert_eq!(
        error.to_string(),
        format!(
            "include directory does not exist: {}",
            directory.path().join("include").display()
        )
    );

    Ok(())
}

#[test]
fn layout_validation_rejects_a_missing_linker_script() -> io::Result<()> {
    let directory = tempdir()?;
    fs::create_dir_all(directory.path().join("include"))?;
    fs::create_dir_all(directory.path().join("vendor/include"))?;

    let error = config()
        .validate_project_layout(directory.path())
        .expect_err("a missing linker script must be rejected");

    assert_eq!(error.kind(), io::ErrorKind::NotFound);
    assert_eq!(
        error.to_string(),
        format!(
            "linker script does not exist: {}",
            directory.path().join("memory.ld").display()
        )
    );

    Ok(())
}

#[test]
fn layout_validation_rejects_a_directory_where_a_linker_script_is_required() -> io::Result<()> {
    let directory = tempdir()?;
    fs::create_dir_all(directory.path().join("include"))?;
    fs::create_dir_all(directory.path().join("vendor/include"))?;
    fs::create_dir(directory.path().join("memory.ld"))?;

    let error = config()
        .validate_project_layout(directory.path())
        .expect_err("a linker-script directory must be rejected");

    assert_eq!(error.kind(), io::ErrorKind::NotFound);

    Ok(())
}
