use super::*;

use include_dir::Dir;
use std::{io, path::Path};
use tempfile::tempdir;

#[test]
fn generates_project_from_the_complete_embedded_template() -> io::Result<()> {
    let test_dir = tempdir()?;
    let output = test_dir.path().join("generated/blink");

    let project = Stm32F103C8::generate(output.clone())?;

    assert_eq!(project.root(), output);
    assert!(project.root().is_dir());
    assert_template_was_extracted(&PROJECT_TEMPLATE, project.root());
    assert_eq!(project.sources().len(), Stm32F103C8::BUILT_IN_SOURCES.len());

    Ok(())
}

#[test]
fn generation_creates_missing_parent_directories() -> io::Result<()> {
    let test_dir = tempdir()?;
    let output = test_dir.path().join("new/parents/blink");

    let project = Stm32F103C8::generate(&output)?;

    assert_eq!(project.root(), output);
    assert!(project.root().is_dir());

    Ok(())
}

#[test]
fn generation_never_overwrites_an_existing_project() -> io::Result<()> {
    let test_dir = tempdir()?;
    let output = test_dir.path().join("overwrite");

    Stm32F103C8::generate(&output)?;
    let error = Stm32F103C8::generate(&output)
        .expect_err("generation must not overwrite an existing project");

    assert_eq!(error.kind(), io::ErrorKind::AlreadyExists);

    Ok(())
}

#[test]
fn generated_project_accepts_application_c_and_assembly_sources() -> io::Result<()> {
    let test_dir = tempdir()?;
    let output = test_dir.path().join("project");
    let mut project = Stm32F103C8::generate(output)?;

    project
        .add_source("main.c", "int main(void) { return 0; }")?
        .add_source("interrupts.s", "")?
        .add_source("preprocessed.S", "")?;

    assert_eq!(
        project.sources().len(),
        Stm32F103C8::BUILT_IN_SOURCES.len() + 3
    );
    assert!(project.root().join("src/main.c").is_file());
    assert!(project.root().join("src/interrupts.s").is_file());
    assert!(project.root().join("src/preprocessed.S").is_file());

    Ok(())
}

#[test]
#[ignore = "requires the Arm GNU toolchain installed on the host"]
fn generated_blink_project_compiles_to_every_expected_artifact()
-> Result<(), Box<dyn std::error::Error>> {
    let test_dir = tempdir()?;
    let output = test_dir.path().join("blink");
    let mut project = Stm32F103C8::generate(output)?;

    project.add_source(
        "main.c",
        r#"
            #include "stm32f1xx.h"

            int main(void)
            {
                while (1) {
                }
            }
        "#,
    )?;

    let artifacts = project.compile()?;
    assert!(artifacts.elf().is_file());
    assert!(artifacts.hex().is_file());
    assert!(artifacts.binary().is_file());
    assert!(artifacts.map().is_file());

    Ok(())
}

fn assert_template_was_extracted(template: &Dir<'_>, output: &Path) {
    for file in template.files() {
        let generated_file = output.join(file.path());
        assert!(
            generated_file.is_file(),
            "missing generated file: {}",
            generated_file.display(),
        );
    }

    for directory in template.dirs() {
        let generated_directory = output.join(directory.path());
        assert!(
            generated_directory.is_dir(),
            "missing generated directory: {}",
            generated_directory.display(),
        );
        assert_template_was_extracted(directory, output);
    }
}
