use firmware_targets::stm32f103c8::Stm32F103C8;

fn main() {
    let code = r#"
    #include "stm32f1xx.h"

    void SystemClock_Config(void) {
        // Already configured to use 8 MHz HSI as default clock
    }

    int main(void) {
        // Enable GPIOC clock
        RCC->APB2ENR |= RCC_APB2ENR_IOPCEN;

        // Configure PC13 as output
        GPIOC->CRH &= ~(0xF << (4 * 1)); // Clear mode bits for PC13
        GPIOC->CRH |= (0x1 << (4 * 1));  // Set mode to push-pull output, max speed 2 MHz

        while (1) {
            // Toggle PC13
            GPIOC->ODR ^= (1 << 13);

            // Blocking delay of 500 ms using HSI clock
            for (volatile uint32_t i = 0; i < (8 * 500 * 1000) / 2; ++i);
        }
    }
    "#;
    let mut project = Stm32F103C8::generate("build").unwrap();
    project.add_source("main.c", code).unwrap();

    match project.compile() {
        Ok(_) => (),
        Err(error) => println!("{error}"),
    };

    println!("done")
}
