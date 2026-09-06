use std::path::Path;

use crate::{
    ProjectTemplate, Target, TemplateKind,
    programmer::{OpenOcd, OpenOcdConfig, ProgramError, ProgramRequest, ProgramResult, Programmer},
    stm32f103c8::templates::{Cmsis, Hal, Ll},
};

/// A programming target backed by a configurable programmer.
///
/// `Target` provides a convenient high-level interface for programming
/// firmware while allowing the underlying [`Programmer`] implementation to be
/// replaced when necessary.
///
/// The default programmer is [`OpenOcd`].
#[derive(Debug, Clone)]
pub struct Stm32f103c8<P: Programmer = OpenOcd> {
    programmer: P,
}

impl OpenOcd {
    /// Creates an OpenOCD programmer configured for the STM32F103C8.
    ///
    /// The default configuration uses an ST-Link interface and the STM32F1
    /// target configuration supplied by OpenOCD.
    pub fn stm32f103c8() -> Self {
        Self::new(OpenOcdConfig::new(
            "interface/stlink.cfg",
            "target/stm32f1x.cfg",
        ))
    }
}

impl Target for Stm32f103c8 {
    fn generate_project(
        &self,
        template: TemplateKind,
        path: impl AsRef<Path>,
    ) -> std::io::Result<crate::Project> {
        match template {
            TemplateKind::Ll => Ll::generate(path.as_ref()),
            TemplateKind::Cmsis => Cmsis::generate(path.as_ref()),
            TemplateKind::Hal => Hal::generate(path.as_ref()),
        }
    }

    /// Programs a firmware image using the default programming options.
    ///
    /// This is a convenience wrapper around [`Target::program_with`].
    fn program(&self, firmware: impl AsRef<Path>) -> Result<ProgramResult, ProgramError> {
        self.program_with(ProgramRequest::new(firmware.as_ref()))
    }
}

impl<P: Programmer> Stm32f103c8<P> {
    /// Creates a programming target using the specified programmer.
    ///
    /// This constructor allows callers to provide custom programmer
    /// implementations, which is useful for alternative programming tools and
    /// testing.
    pub fn with_programmer(programmer: P) -> Self {
        Self { programmer }
    }

    // /// Programs a firmware image using the default programming options.
    // ///
    // /// This is a convenience wrapper around [`Target::program_with`].
    // pub fn program(&self, firmware: impl AsRef<Path>) -> Result<ProgramResult, ProgramError> {
    //     self.program_with(ProgramRequest::new(firmware.as_ref()))
    // }

    /// Programs a firmware image using the specified request.
    ///
    /// This method delegates the operation to the configured
    /// [`Programmer`] implementation.
    pub fn program_with(&self, request: ProgramRequest) -> Result<ProgramResult, ProgramError> {
        self.programmer.program(&request)
    }
}

impl Stm32f103c8<OpenOcd> {
    /// Creates an STM32F103C8 programming target using OpenOCD.
    ///
    /// The target is configured to use an ST-Link debug interface and the
    /// STM32F1 OpenOCD target configuration.
    pub fn new() -> Self {
        Self {
            programmer: OpenOcd::stm32f103c8(),
        }
    }
}

impl Default for Stm32f103c8<OpenOcd> {
    fn default() -> Self {
        Self::new()
    }
}
