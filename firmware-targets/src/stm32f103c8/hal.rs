use super::ProjectTemplate;

use crate::ArmGccConfig;

use include_dir::{Dir, include_dir};

/// Embedded project template based on ARM CMSIS and STM32 device definitions.
///
/// The generated project includes the CMSIS core and STM32F1 device headers,
/// the STM32 system initialization source, and an ARM GCC configuration for
/// the Cortex-M3 core.
pub(super) static TEMPLATE: Dir<'static> =
    include_dir!("$CARGO_MANIFEST_DIR/assets/STM32F103C8/HAL");

/// Source files supplied by the CMSIS template and built as part of the
/// generated project.
pub(super) const BUILT_IN_SOURCES: &[&str] = &[
    "src/stm32f1xx_it.c",
    "src/stm32f1xx_hal_msp.c",
    "Drivers/STM32F1xx_HAL_Driver/Src/stm32f1xx_hal_gpio_ex.c",
    "Drivers/STM32F1xx_HAL_Driver/Src/stm32f1xx_hal_tim.c",
    "Drivers/STM32F1xx_HAL_Driver/Src/stm32f1xx_hal_tim_ex.c",
    "Drivers/STM32F1xx_HAL_Driver/Src/stm32f1xx_hal.c",
    "Drivers/STM32F1xx_HAL_Driver/Src/stm32f1xx_hal_rcc.c",
    "Drivers/STM32F1xx_HAL_Driver/Src/stm32f1xx_hal_rcc_ex.c",
    "Drivers/STM32F1xx_HAL_Driver/Src/stm32f1xx_hal_gpio.c",
    "Drivers/STM32F1xx_HAL_Driver/Src/stm32f1xx_hal_dma.c",
    "Drivers/STM32F1xx_HAL_Driver/Src/stm32f1xx_hal_cortex.c",
    "Drivers/STM32F1xx_HAL_Driver/Src/stm32f1xx_hal_pwr.c",
    "Drivers/STM32F1xx_HAL_Driver/Src/stm32f1xx_hal_flash.c",
    "Drivers/STM32F1xx_HAL_Driver/Src/stm32f1xx_hal_flash_ex.c",
    "Drivers/STM32F1xx_HAL_Driver/Src/stm32f1xx_hal_exti.c",
    "src/system_stm32f1xx.c",
    "startup_stm32f103xb.s",
];

/// An STM32F103C8 project template based on CMSIS.
///
/// Use [`ProjectTemplate::generate`] to generate a project from the template.
#[derive(Debug, Default, Clone, Copy)]
pub struct Hal;

impl ProjectTemplate for Hal {
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
            [
                "Inc",
                "Drivers/STM32F1xx_HAL_Driver/Inc",
                "Drivers/STM32F1xx_HAL_Driver/Inc/Legacy",
                "Drivers/CMSIS/Device/ST/STM32F1xx/Include",
                "Drivers/CMSIS/Include",
            ],
            "STM32F103C8Tx_FLASH.ld",
        )
    }
}
