use super::*;

use std::path::PathBuf;

#[cfg(unix)]
use std::os::unix::process::ExitStatusExt;

#[cfg(unix)]
#[test]
fn failed_tool_commands_capture_stderr_and_build_context() {
    let mut command = Command::new("sh");
    command.args(["-c", "printf diagnostic >&2; exit 7"]);

    let error = run(&mut command, BuildStage::Compile, Path::new("src/main.c"))
        .expect_err("a failing command must return a build error");

    match error {
        BuildError::CommandFailed {
            stage,
            path,
            status,
            diagnostics,
        } => {
            assert_eq!(stage, BuildStage::Compile);
            assert_eq!(path, Path::new("src/main.c"));
            assert_eq!(status.code(), Some(7));
            assert_eq!(diagnostics, "diagnostic");
        }
        error => panic!("expected CommandFailed, received {error:?}"),
    }
}

#[cfg(unix)]
#[test]
fn failed_tool_commands_use_stdout_when_stderr_is_empty() {
    let mut command = Command::new("sh");
    command.args(["-c", "printf diagnostic; exit 1"]);

    let error = run(
        &mut command,
        BuildStage::Link,
        Path::new("build/firmware.elf"),
    )
    .expect_err("a failing command must return a build error");

    assert!(matches!(
        error,
        BuildError::CommandFailed {
            stage: BuildStage::Link,
            diagnostics,
            ..
        } if diagnostics == "diagnostic"
    ));
}

#[cfg(unix)]
#[test]
fn command_failed_display_includes_trimmed_diagnostics() {
    let error = BuildError::CommandFailed {
        stage: BuildStage::ConvertToHex,
        path: PathBuf::from("build/firmware.hex"),
        status: std::process::ExitStatus::from_raw(256),
        diagnostics: "conversion failed\n\n".into(),
    };

    assert_eq!(
        error.to_string(),
        "convert to Intel HEX stage failed for build/firmware.hex (exit status: 1):\nconversion failed"
    );
}