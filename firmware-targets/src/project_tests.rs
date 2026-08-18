use super::*;

use std::fs;

use tempfile::tempdir;

fn config() -> ArmGccConfig {
    ArmGccConfig::new(
        "test-gcc",
        "test-objcopy",
        "cortex-m3",
        ["TEST_TARGET"],
        ["include"],
        "memory.ld",
    )
}

fn generated_layout() -> tempfile::TempDir {
    let directory = tempdir().unwrap();
    fs::create_dir_all(directory.path().join("src")).unwrap();
    fs::create_dir_all(directory.path().join("include")).unwrap();
    fs::write(directory.path().join("memory.ld"), "MEMORY {}\n").unwrap();
    fs::write(directory.path().join("src/startup.s"), "").unwrap();
    directory
}

#[test]
fn source_name_validation_accepts_only_simple_c_and_assembly_filenames() {
    for name in ["main.c", "startup.s", "startup.S"] {
        assert!(
            validate_source_name(Path::new(name), Project::SUPPORTED_SOURCE_EXTENSIONS).is_ok()
        );
    }
}

#[test]
fn source_name_validation_rejects_paths_unsupported_extensions_and_missing_extensions() {
    for name in [
        "",
        "main",
        "main.rs",
        "nested/main.c",
        "../main.c",
        "/tmp/main.c",
        "main.C",
    ] {
        let error = validate_source_name(Path::new(name), Project::SUPPORTED_SOURCE_EXTENSIONS)
            .expect_err("{name:?} must not be accepted");

        assert_eq!(error.kind(), io::ErrorKind::InvalidInput, "{name}");
        assert_eq!(
            error.to_string(),
            "source must be a simple filename ending in .c, .s, .S"
        );
    }
}

#[test]
fn from_generated_rejects_a_missing_project_directory() {
    let directory = tempdir().unwrap();
    let path = directory.path().join("missing");
    let error = Project::from_generated(path.clone(), Vec::new(), config()).unwrap_err();

    assert_eq!(error.kind(), io::ErrorKind::NotFound);
    assert!(error.to_string().contains(&path.display().to_string()));
}

#[test]
fn from_generated_rejects_a_missing_source_file() {
    let layout = generated_layout();
    let missing = layout.path().join("src/missing.c");
    let error =
        Project::from_generated(layout.path(), vec![missing.clone()], config()).unwrap_err();

    assert_eq!(error.kind(), io::ErrorKind::NotFound);
    assert!(error.to_string().contains(&missing.display().to_string()));
}

#[test]
fn from_generated_validates_the_compiler_layout_before_constructing_the_project() {
    let layout = generated_layout();
    fs::remove_file(layout.path().join("memory.ld")).unwrap();
    let source = layout.path().join("src/startup.s");

    let error = Project::from_generated(layout.path(), vec![source], config()).unwrap_err();

    assert_eq!(error.kind(), io::ErrorKind::NotFound);
    assert!(error.to_string().contains("linker script does not exist"));
}

#[test]
fn from_generated_preserves_root_and_preexisting_sources() -> io::Result<()> {
    let layout = generated_layout();
    let source = layout.path().join("src/startup.s");
    let project = Project::from_generated(layout.path(), vec![source.clone()], config())?;

    assert_eq!(project.root(), layout.path());
    assert_eq!(project.sources(), [source]);

    Ok(())
}

#[test]
fn add_source_writes_the_contents_and_tracks_the_new_source() -> io::Result<()> {
    let layout = generated_layout();
    let startup = layout.path().join("src/startup.s");
    let mut project = Project::from_generated(layout.path(), vec![startup], config())?;

    let returned = project.add_source("main.c", "int main(void) { return 0; }\n")?;
    assert_eq!(returned.root(), layout.path());

    let main = layout.path().join("src/main.c");
    assert_eq!(fs::read_to_string(&main)?, "int main(void) { return 0; }\n");
    assert!(project.sources().contains(&main));

    Ok(())
}

#[test]
fn add_source_does_not_overwrite_an_existing_file_or_duplicate_the_source_list() -> io::Result<()> {
    let layout = generated_layout();
    let startup = layout.path().join("src/startup.s");
    let mut project = Project::from_generated(layout.path(), vec![startup], config())?;

    project.add_source("main.c", "first")?;
    let sources_before = project.sources().len();
    let error = project
        .add_source("main.c", "second")
        .expect_err("existing sources must never be overwritten");

    assert_eq!(error.kind(), io::ErrorKind::AlreadyExists);
    assert_eq!(
        fs::read_to_string(layout.path().join("src/main.c"))?,
        "first"
    );
    assert_eq!(project.sources().len(), sources_before);

    Ok(())
}
