#[repr(C)]
///Register block
pub struct RegisterBlock {
    wdogload: Wdogload,
    wdogvalue: Wdogvalue,
    wdogcontrol: Wdogcontrol,
    wdogintclr: Wdogintclr,
    wdogris: Wdogris,
    wdogmis: Wdogmis,
    _reserved6: [u8; 0x0be8],
    wdoglock: Wdoglock,
}
impl RegisterBlock {
    ///0x00 - Watchdog load register
    #[inline(always)]
    pub const fn wdogload(&self) -> &Wdogload {
        &self.wdogload
    }
    ///0x04 - Watchdog current value register (read-only)
    #[inline(always)]
    pub const fn wdogvalue(&self) -> &Wdogvalue {
        &self.wdogvalue
    }
    ///0x08 - Watchdog control register
    #[inline(always)]
    pub const fn wdogcontrol(&self) -> &Wdogcontrol {
        &self.wdogcontrol
    }
    ///0x0c - Watchdog interrupt clear (write any value to clear and reload)
    #[inline(always)]
    pub const fn wdogintclr(&self) -> &Wdogintclr {
        &self.wdogintclr
    }
    ///0x10 - Watchdog raw interrupt status
    #[inline(always)]
    pub const fn wdogris(&self) -> &Wdogris {
        &self.wdogris
    }
    ///0x14 - Watchdog masked interrupt status
    #[inline(always)]
    pub const fn wdogmis(&self) -> &Wdogmis {
        &self.wdogmis
    }
    ///0xc00 - Watchdog lock register (write 0x1ACCE551 to unlock)
    #[inline(always)]
    pub const fn wdoglock(&self) -> &Wdoglock {
        &self.wdoglock
    }
}
/**WDOGLOAD (rw) register accessor: Watchdog load register

You can [`read`](crate::Reg::read) this register and get [`wdogload::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdogload::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@wdogload`] module*/
#[doc(alias = "WDOGLOAD")]
pub type Wdogload = crate::Reg<wdogload::WdogloadSpec>;
///Watchdog load register
pub mod wdogload;
/**WDOGVALUE (r) register accessor: Watchdog current value register (read-only)

You can [`read`](crate::Reg::read) this register and get [`wdogvalue::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@wdogvalue`] module*/
#[doc(alias = "WDOGVALUE")]
pub type Wdogvalue = crate::Reg<wdogvalue::WdogvalueSpec>;
///Watchdog current value register (read-only)
pub mod wdogvalue;
/**WDOGCONTROL (rw) register accessor: Watchdog control register

You can [`read`](crate::Reg::read) this register and get [`wdogcontrol::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdogcontrol::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@wdogcontrol`] module*/
#[doc(alias = "WDOGCONTROL")]
pub type Wdogcontrol = crate::Reg<wdogcontrol::WdogcontrolSpec>;
///Watchdog control register
pub mod wdogcontrol;
/**WDOGINTCLR (w) register accessor: Watchdog interrupt clear (write any value to clear and reload)

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdogintclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@wdogintclr`] module*/
#[doc(alias = "WDOGINTCLR")]
pub type Wdogintclr = crate::Reg<wdogintclr::WdogintclrSpec>;
///Watchdog interrupt clear (write any value to clear and reload)
pub mod wdogintclr;
/**WDOGRIS (r) register accessor: Watchdog raw interrupt status

You can [`read`](crate::Reg::read) this register and get [`wdogris::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@wdogris`] module*/
#[doc(alias = "WDOGRIS")]
pub type Wdogris = crate::Reg<wdogris::WdogrisSpec>;
///Watchdog raw interrupt status
pub mod wdogris;
/**WDOGMIS (r) register accessor: Watchdog masked interrupt status

You can [`read`](crate::Reg::read) this register and get [`wdogmis::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@wdogmis`] module*/
#[doc(alias = "WDOGMIS")]
pub type Wdogmis = crate::Reg<wdogmis::WdogmisSpec>;
///Watchdog masked interrupt status
pub mod wdogmis;
/**WDOGLOCK (rw) register accessor: Watchdog lock register (write 0x1ACCE551 to unlock)

You can [`read`](crate::Reg::read) this register and get [`wdoglock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdoglock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@wdoglock`] module*/
#[doc(alias = "WDOGLOCK")]
pub type Wdoglock = crate::Reg<wdoglock::WdoglockSpec>;
///Watchdog lock register (write 0x1ACCE551 to unlock)
pub mod wdoglock;
