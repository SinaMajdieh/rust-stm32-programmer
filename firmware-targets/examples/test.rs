use firmware_targets::stm32f103c8::{Ll, ProjectTemplate};

fn main() {
    let code = r#"
        #include <stdint.h>

        /*
         * STM32F103C8 memory-mapped registers
         *
         * RCC APB2 peripheral clock enable register
         */
        #define RCC_APB2ENR   (*(volatile uint32_t *)0x40021018)

        /*
         * GPIOC configuration registers
         */
        #define GPIOC_CRH     (*(volatile uint32_t *)0x40011004)
        #define GPIOC_ODR     (*(volatile uint32_t *)0x4001100C)

        /*
         * Bit used to enable the GPIOC peripheral clock.
         * IOPCEN = bit 4
         */
        #define RCC_IOPCEN    (1U << 4)

        /*
         * PC13 is the onboard LED.
         *
         * PC13 is in GPIOC_CRH because it is pin 8..15.
         *
         * Each pin gets 4 configuration bits.
         * PC13 starts at bit (13 - 8) * 4 = 20.
         */
        #define PC13_CONFIG_SHIFT 20

        static void delay(volatile uint32_t count)
        {
            while (count--)
            {
                __asm volatile ("nop");
            }
        }

        int main(void)
        {
            /*
             * 1. Enable the clock for GPIOC.
             */
            RCC_APB2ENR |= RCC_IOPCEN;

            /*
             * 2. Configure PC13 as:
             *
             * MODE = 01 → output mode, max 10 MHz
             * CNF  = 00 → general-purpose push-pull
             *
             * Binary:
             *
             * 0001
             *
             * which is 0x1.
             */
            GPIOC_CRH &= ~(0xFU << PC13_CONFIG_SHIFT);
            GPIOC_CRH |=  (0x1U << PC13_CONFIG_SHIFT);

            while (1)
            {
                /*
                 * LED ON
                 *
                 * Active-low:
                 * PC13 = 0
                 */
                GPIOC_ODR &= ~(1U << 13);

                delay(500000);

                /*
                 * LED OFF
                 *
                 * PC13 = 1
                 */
                GPIOC_ODR |= (1U << 13);

                delay(500000);
            }
        }

    "#;
    let mut project = Ll::generate("build").unwrap();
    project.add_source("main.c", code).unwrap();

    match project.compile() {
        Ok(_) => (),
        Err(error) => println!("{error}"),
    };

    println!("done")
}
