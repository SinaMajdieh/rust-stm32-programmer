use super::*;

#[test]
fn build_artifacts_return_the_paths_passed_to_the_constructor() {
    let artifacts = BuildArtifacts::new(
        PathBuf::from("build/firmware.elf"),
        PathBuf::from("build/firmware.hex"),
        PathBuf::from("build/firmware.bin"),
        PathBuf::from("build/firmware.map"),
    );

    assert_eq!(artifacts.elf(), Path::new("build/firmware.elf"));
    assert_eq!(artifacts.hex(), Path::new("build/firmware.hex"));
    assert_eq!(artifacts.binary(), Path::new("build/firmware.bin"));
    assert_eq!(artifacts.map(), Path::new("build/firmware.map"));
}

#[test]
fn build_stages_have_stable_user_facing_names() {
    assert_eq!(BuildStage::Compile.to_string(), "compile");
    assert_eq!(BuildStage::Link.to_string(), "link");
    assert_eq!(BuildStage::ConvertToHex.to_string(), "convert to Intel HEX");
    assert_eq!(BuildStage::ConvertToBinary.to_string(), "convert to binary");
}

#[test]
fn unsupported_source_errors_name_the_file_and_supported_extensions() {
    let error = BuildError::UnsupportedSource {
        path: PathBuf::from("src/main.rs"),
    };

    assert_eq!(
        error.to_string(),
        "unsupported source file src/main.rs; supported extensions are .c, .s, and .S"
    );
    assert!(std::error::Error::source(&error).is_none());
}

#[test]
fn io_errors_are_displayed_and_exposed_as_the_error_source() {
    let error = BuildError::from(std::io::Error::other("toolchain unavailable"));

    assert_eq!(error.to_string(), "toolchain unavailable");
    assert!(std::error::Error::source(&error).is_some());
}
