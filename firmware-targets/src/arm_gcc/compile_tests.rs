use super::*;

fn config() -> ArmGccConfig {
    ArmGccConfig::new(
        "arm-none-eabi-gcc",
        "arm-none-eabi-objcopy",
        "cortex-m3",
        ["STM32F103xB", "USE_HAL_DRIVER=1"],
        ["CMSIS/Include", "CMSIS/Device/ST/STM32F1xx"],
        "memory.ld",
    )
}

fn arguments(command: &Command) -> Vec<String> {
    command
        .get_args()
        .map(|argument| argument.to_string_lossy().into_owned())
        .collect()
}

#[test]
fn recognizes_c_and_both_assembly_file_extensions() {
    for source in ["main.c", "startup.s", "startup.S"] {
        assert!(is_supported_source(Path::new(source)), "{source}");
    }

    for source in ["main.C", "main.cpp", "main.rs", "main", ".c"] {
        assert!(!is_supported_source(Path::new(source)), "{source}");
    }
}

#[test]
fn object_path_uses_the_source_filename_and_appends_o() {
    let output = object_path(Path::new("src/main.c"), Path::new("build")).unwrap();
    assert_eq!(output, Path::new("build/main.c.o"));

    let output = object_path(Path::new("startup.S"), Path::new("build")).unwrap();
    assert_eq!(output, Path::new("build/startup.S.o"));
}

#[test]
fn object_path_rejects_a_path_without_a_filename() {
    let error = object_path(Path::new(""), Path::new("build")).unwrap_err();

    assert!(matches!(
        error,
        BuildError::UnsupportedSource { source } if source.as_path() == Path::new("")
    ));
}

#[test]
fn c_compile_command_has_target_flags_defines_includes_and_output() {
    let root = Path::new("generated/blink");
    let source = root.join("src/main.c");
    let object = root.join("build/main.c.o");
    let command = command(&config(), root, &source, &object);

    assert_eq!(command.get_program(), "arm-none-eabi-gcc");
    assert_eq!(
        arguments(&command),
        [
            "-mcpu=cortex-m3",
            "-mthumb",
            "-ffunction-sections",
            "-fdata-sections",
            "-Og",
            "-g3",
            "-std=c11",
            "-DSTM32F103xB",
            "-DUSE_HAL_DRIVER=1",
            "-I",
            "generated/blink/CMSIS/Include",
            "-I",
            "generated/blink/CMSIS/Device/ST/STM32F1xx",
            "-c",
            "generated/blink/src/main.c",
            "-o",
            "generated/blink/build/main.c.o",
        ]
    );
}

#[test]
fn assembly_compile_command_omits_the_c_language_standard() {
    let root = Path::new("generated/blink");
    let source = root.join("src/startup_stm32f103xb.s");
    let object = root.join("build/startup_stm32f103xb.s.o");
    let command = command(&config(), root, &source, &object);
    let arguments = arguments(&command);

    assert!(!arguments.iter().any(|argument| argument == "-std=c11"));
    assert_eq!(
        arguments.last(),
        Some(&"generated/blink/build/startup_stm32f103xb.s.o".into())
    );
}
