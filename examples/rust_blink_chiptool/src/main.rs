#![no_std]
#![no_main]

use cortex_m::asm;
use cortex_m_rt::entry;
use panic_halt as _;

use cxd56_pac_chiptool::GPIO0;

// ~0.5 s at 153.6 MHz
const DELAY_CYCLES: u32 = 76_800_000;

#[entry]
fn main() -> ! {
    // Configure PIN97 (LED0) as output, initially low.
    // DIR is active-low: false = drive output.
    GPIO0.PIN97().write(|w| {
        w.set_DIR(false);
        w.set_OUT(false);
    });

    loop {
        // Read GP_I2C4_BCK input level.
        let pin1_high = GPIO0.GP_I2C4_BCK().read().IN();

        // Mirror input level on LED0.
        GPIO0.PIN97().modify(|w| w.set_OUT(pin1_high));
        asm::delay(DELAY_CYCLES);

        // Force LED off so there is always a visible blink.
        GPIO0.PIN97().modify(|w| w.set_OUT(false));
        asm::delay(DELAY_CYCLES);
    }
}
