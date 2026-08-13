//! Integration tests for the STM32F103C8 Blue Pill target.

use std::io;

use firmware_targets::stm32f103c8::Stm32F103C8;
use tempfile::tempdir;

#[test]
fn target_generation_produces_a_reusable_project_with_builtin_sources() -> io::Result<()> {
    let directory = tempdir()?;
    let output = directory.path().join("generated/blue-pill");

    let mut project = Stm32F103C8::generate(&output)?;

    assert_eq!(project.root(), output);
    assert_eq!(project.sources().len(), 2);
    assert!(project.root().join("src/startup_stm32f103xb.s").is_file());
    assert!(project.root().join("src/system_stm32f1xx.c").is_file());

    project.add_source("main.c", "int main(void) { for (;;) {} }\n")?;
    assert!(project.root().join("src/main.c").is_file());
    assert_eq!(project.sources().len(), 3);

    Ok(())
}

#[test]
fn target_generation_does_not_replace_an_existing_project() -> io::Result<()> {
    let directory = tempdir()?;
    let output = directory.path().join("blue-pill");

    Stm32F103C8::generate(&output)?;
    let error = Stm32F103C8::generate(&output)
        .expect_err("the target generator must never overwrite user files");

    assert_eq!(error.kind(), io::ErrorKind::AlreadyExists);

    Ok(())
}

#[test]
#[ignore = "requires the Arm GNU toolchain installed on the host"]
fn public_api_builds_a_minimal_blue_pill_program() -> Result<(), Box<dyn std::error::Error>> {
    let directory = tempdir()?;
    let output = directory.path().join("blue-pill");
    let mut project = Stm32F103C8::generate(&output)?;

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
