use super::ProjectTemplate;

use crate::ArmGccConfig;

use include_dir::{Dir, include_dir};

/// Embedded project template based on ARM CMSIS and STM32 device definitions.
///
/// The generated project includes the CMSIS core and STM32F1 device headers,
/// the STM32 system initialization source, and an ARM GCC configuration for
/// the Cortex-M3 core.
pub(super) static TEMPLATE: Dir<'static> =
    include_dir!("$CARGO_MANIFEST_DIR/assets/STM32F103C8/CMSIS");

/// Source files supplied by the CMSIS template and built as part of the
/// generated project.
pub(super) const BUILT_IN_SOURCES: &[&str] =
    &["src/startup_stm32f103xb.s", "src/system_stm32f1xx.c"];

/// An STM32F103C8 project template based on CMSIS.
///
/// Use [`ProjectTemplate::generate`] to generate a project from the template.
#[derive(Debug, Default, Clone, Copy)]
pub struct Cmsis;

impl ProjectTemplate for Cmsis {
    fn template() -> &'static Dir<'static> {
        &TEMPLATE
    }

    fn built_in_sources() -> &'static [&'static str] {
        BUILT_IN_SOURCES
    }

    fn build_config() -> ArmGccConfig {
        ArmGccConfig::new(
            "arm-none-eabi-gcc",
            "arm-none-eabi-objcopy",
            "cortex-m3",
            ["STM32F103xB"],
            ["CMSIS/Include", "CMSIS/Device/ST/STM32F1xx"],
            "STM32F103C8TX_FLASH.ld",
        )
    }
}
