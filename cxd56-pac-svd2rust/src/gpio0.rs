#[repr(C)]
///Register block
pub struct RegisterBlock {
    gp_i2c4_bck: GpI2c4Bck,
    _reserved1: [u8; 0x0164],
    pin97: Pin97,
    pin98: Pin98,
    pin99: Pin99,
    pin100: Pin100,
}
impl RegisterBlock {
    ///0x00 - GPIO SYS pin 1 — I2C4 clock / Arduino D14
    #[inline(always)]
    pub const fn gp_i2c4_bck(&self) -> &GpI2c4Bck {
        &self.gp_i2c4_bck
    }
    ///0x168 - GPIO APP pin 97 — I2S1_BCK / LED0 on Spresense main board
    #[inline(always)]
    pub const fn pin97(&self) -> &Pin97 {
        &self.pin97
    }
    ///0x16c - GPIO APP pin 98 — I2S1_LRCK / LED1 on Spresense main board
    #[inline(always)]
    pub const fn pin98(&self) -> &Pin98 {
        &self.pin98
    }
    ///0x170 - GPIO APP pin 99 — I2S1_DATA_IN / LED2 on Spresense main board
    #[inline(always)]
    pub const fn pin99(&self) -> &Pin99 {
        &self.pin99
    }
    ///0x174 - GPIO APP pin 100 — I2S1_DATA_OUT / LED3 on Spresense main board
    #[inline(always)]
    pub const fn pin100(&self) -> &Pin100 {
        &self.pin100
    }
}
/**GP_I2C4_BCK (rw) register accessor: GPIO SYS pin 1 — I2C4 clock / Arduino D14

You can [`read`](crate::Reg::read) this register and get [`gp_i2c4_bck::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gp_i2c4_bck::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gp_i2c4_bck`] module*/
#[doc(alias = "GP_I2C4_BCK")]
pub type GpI2c4Bck = crate::Reg<gp_i2c4_bck::GpI2c4BckSpec>;
///GPIO SYS pin 1 — I2C4 clock / Arduino D14
pub mod gp_i2c4_bck;
/**PIN97 (rw) register accessor: GPIO APP pin 97 — I2S1_BCK / LED0 on Spresense main board

You can [`read`](crate::Reg::read) this register and get [`pin97::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pin97::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pin97`] module*/
#[doc(alias = "PIN97")]
pub type Pin97 = crate::Reg<pin97::Pin97Spec>;
///GPIO APP pin 97 — I2S1_BCK / LED0 on Spresense main board
pub mod pin97;
/**PIN98 (rw) register accessor: GPIO APP pin 98 — I2S1_LRCK / LED1 on Spresense main board

You can [`read`](crate::Reg::read) this register and get [`pin98::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pin98::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pin98`] module*/
#[doc(alias = "PIN98")]
pub type Pin98 = crate::Reg<pin98::Pin98Spec>;
///GPIO APP pin 98 — I2S1_LRCK / LED1 on Spresense main board
pub mod pin98;
/**PIN99 (rw) register accessor: GPIO APP pin 99 — I2S1_DATA_IN / LED2 on Spresense main board

You can [`read`](crate::Reg::read) this register and get [`pin99::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pin99::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pin99`] module*/
#[doc(alias = "PIN99")]
pub type Pin99 = crate::Reg<pin99::Pin99Spec>;
///GPIO APP pin 99 — I2S1_DATA_IN / LED2 on Spresense main board
pub mod pin99;
/**PIN100 (rw) register accessor: GPIO APP pin 100 — I2S1_DATA_OUT / LED3 on Spresense main board

You can [`read`](crate::Reg::read) this register and get [`pin100::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pin100::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pin100`] module*/
#[doc(alias = "PIN100")]
pub type Pin100 = crate::Reg<pin100::Pin100Spec>;
///GPIO APP pin 100 — I2S1_DATA_OUT / LED3 on Spresense main board
pub mod pin100;
