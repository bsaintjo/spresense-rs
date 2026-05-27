#![no_std]

pub use cxd56_pac_chiptool as pac;

pub mod gpio;

embassy_hal_internal::peripherals! {
    GP_I2S1_BCK,
}

pub fn init() -> Peripherals {
    Peripherals::take()
}
