//! Integration tests for the public generic-project API.

use std::{error::Error, fs, io, path::Path};

use firmware_targets::{ArmGccConfig, BuildError, BuildStage, Project};
use tempfile::tempdir;

fn config() -> ArmGccConfig {
    ArmGccConfig::new(
        "test-gcc",
        "test-objcopy",
        "cortex-m3",
        ["TEST_MCU"],
        ["include", "vendor/include"],
        "memory.ld",
    )
}

fn project_directory() -> tempfile::TempDir {
    let directory = tempdir().unwrap();
    fs::create_dir_all(directory.path().join("src")).unwrap();
    fs::create_dir_all(directory.path().join("include")).unwrap();
    fs::create_dir_all(directory.path().join("vendor/include")).unwrap();
    fs::write(directory.path().join("memory.ld"), "MEMORY {}\n").unwrap();
    fs::write(directory.path().join("src/startup.s"), "").unwrap();
    directory
}

#[test]
fn public_configuration_api_preserves_all_values() {
    let config = config();

    assert_eq!(config.compiler(), Path::new("test-gcc"));
    assert_eq!(config.objcopy(), Path::new("test-objcopy"));
    assert_eq!(config.cpu(), "cortex-m3");
    assert_eq!(config.defines(), ["TEST_MCU"]);
    assert_eq!(config.include_dirs().len(), 2);
    assert_eq!(config.linker_script(), Path::new("memory.ld"));
}

#[test]
fn external_target_contributors_can_construct_and_extend_a_project() -> io::Result<()> {
    let directory = project_directory();
    let startup = directory.path().join("src/startup.s");
    let mut project = Project::from_generated(directory.path(), vec![startup.clone()], config())?;

    project
        .add_source("main.c", "int main(void) { return 0; }\n")?
        .add_source("vectors.S", "")?;

    assert_eq!(project.root(), directory.path());
    assert_eq!(project.sources().len(), 3);
    assert_eq!(project.sources()[0], startup);
    assert_eq!(
        fs::read_to_string(project.root().join("src/main.c"))?,
        "int main(void) { return 0; }\n"
    );

    Ok(())
}

#[test]
fn public_project_api_rejects_invalid_source_names_and_preserves_the_directory() -> io::Result<()> {
    let directory = project_directory();
    let startup = directory.path().join("src/startup.s");
    let mut project = Project::from_generated(directory.path(), vec![startup], config())?;

    for file_name in ["main.rs", "nested/main.c", "../main.c", "main"] {
        let error = project
            .add_source(file_name, "")
            .expect_err("{file_name:?} must be rejected");
        assert_eq!(error.kind(), io::ErrorKind::InvalidInput);
    }

    assert!(!project.root().join("src/main.rs").exists());
    assert!(!project.root().join("src/main.c").exists());

    Ok(())
}

#[test]
fn public_project_api_rejects_incomplete_layouts() {
    let directory = tempdir().unwrap();
    let missing_source = directory.path().join("src/startup.s");

    let error = Project::from_generated(directory.path(), vec![missing_source], config())
        .expect_err("an incomplete generated project must be rejected");

    assert_eq!(error.kind(), io::ErrorKind::NotFound);
}

#[test]
fn public_build_errors_and_stages_have_clear_contracts() {
    assert_eq!(BuildStage::Compile.to_string(), "compile");
    assert_eq!(BuildStage::Link.to_string(), "link");
    assert_eq!(BuildStage::ConvertToHex.to_string(), "convert to Intel HEX");
    assert_eq!(BuildStage::ConvertToBinary.to_string(), "convert to binary");

    let error = BuildError::from(io::Error::other("missing compiler"));
    assert_eq!(error.to_string(), "missing compiler");
    assert!(Error::source(&error).is_some());
}
