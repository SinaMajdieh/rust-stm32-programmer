mod stm32f103c8;

pub trait Compile {
    type Firmware;
    type Error;

    fn compile(&self) -> Result<Self::Firmware, Self::Error>;
}
