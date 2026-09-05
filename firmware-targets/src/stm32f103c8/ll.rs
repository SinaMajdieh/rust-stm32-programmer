use super::ProjectTemplate;

use crate::ArmGccConfig;

use include_dir::{Dir, include_dir};

/// Embedded project template based on the STM32 Low-Layer approach.
///
/// The generated project provides the LL-oriented project structure and a
/// minimal ARM GCC configuration for the STM32F103C8's Cortex-M3 core.
pub(super) static TEMPLATE: Dir<'static> =
    include_dir!("$CARGO_MANIFEST_DIR/assets/STM32F103C8/LL");

/// Source files supplied by the LL template and built as part of the
/// generated project.
pub(super) const BUILT_IN_SOURCES: &[&str] = &["src/startup.s"];

/// An STM32F103C8 project template based on the STM32 Low-Layer approach.
///
/// Use [`ProjectTemplate::generate`] to generate a project from the template.
#[derive(Debug, Default, Clone, Copy)]
pub struct Ll;

impl ProjectTemplate for Ll {
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
            Vec::<String>::new(),
            "linker.ld",
        )
    }
}
