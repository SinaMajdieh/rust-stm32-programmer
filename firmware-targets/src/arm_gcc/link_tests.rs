use super::*;

fn config() -> ArmGccConfig {
    ArmGccConfig::new(
        "arm-none-eabi-gcc",
        "arm-none-eabi-objcopy",
        "cortex-m3",
        ["STM32F103xB"],
        ["CMSIS/Include"],
        "STM32F103C8TX_FLASH.ld",
    )
}

fn arguments(command: &Command) -> Vec<String> {
    command
        .get_args()
        .map(|argument| argument.to_string_lossy().into_owned())
        .collect()
}

#[test]
fn link_command_contains_the_linker_script_map_file_and_every_object() {
    let root = Path::new("generated/blink");
    let objects = vec![
        root.join("build/main.c.o"),
        root.join("build/system_stm32f1xx.c.o"),
        root.join("build/startup_stm32f103xb.s.o"),
    ];
    let elf = root.join("build/firmware.elf");
    let map = root.join("build/firmware.map");
    let command = command(&config(), root, &objects, &elf, &map);

    assert_eq!(command.get_program(), "arm-none-eabi-gcc");
    assert_eq!(
        arguments(&command),
        [
            "-mcpu=cortex-m3",
            "-mthumb",
            "--specs=nano.specs",
            "--specs=nosys.specs",
            "-T",
            "generated/blink/STM32F103C8TX_FLASH.ld",
            "-Wl,--gc-sections",
            "-Wl,-Map=generated/blink/build/firmware.map",
            "generated/blink/build/main.c.o",
            "generated/blink/build/system_stm32f1xx.c.o",
            "generated/blink/build/startup_stm32f103xb.s.o",
            "-o",
            "generated/blink/build/firmware.elf",
        ]
    );
}


