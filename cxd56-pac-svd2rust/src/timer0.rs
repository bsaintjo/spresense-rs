#[repr(C)]
///Register block
pub struct RegisterBlock {
    load: Load,
    value: Value,
    control: Control,
    intclr: Intclr,
    ris: Ris,
    mis: Mis,
    bgload: Bgload,
}
impl RegisterBlock {
    ///0x00 - Counter reload value
    #[inline(always)]
    pub const fn load(&self) -> &Load {
        &self.load
    }
    ///0x04 - Current counter value (read-only)
    #[inline(always)]
    pub const fn value(&self) -> &Value {
        &self.value
    }
    ///0x08 - Timer mode, prescaler, and interrupt control
    #[inline(always)]
    pub const fn control(&self) -> &Control {
        &self.control
    }
    ///0x0c - Interrupt clear (write any value to clear)
    #[inline(always)]
    pub const fn intclr(&self) -> &Intclr {
        &self.intclr
    }
    ///0x10 - Raw interrupt status
    #[inline(always)]
    pub const fn ris(&self) -> &Ris {
        &self.ris
    }
    ///0x14 - Masked interrupt status
    #[inline(always)]
    pub const fn mis(&self) -> &Mis {
        &self.mis
    }
    ///0x18 - Background load register (no immediate counter restart)
    #[inline(always)]
    pub const fn bgload(&self) -> &Bgload {
        &self.bgload
    }
}
/**LOAD (rw) register accessor: Counter reload value

You can [`read`](crate::Reg::read) this register and get [`load::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`load::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@load`] module*/
#[doc(alias = "LOAD")]
pub type Load = crate::Reg<load::LoadSpec>;
///Counter reload value
pub mod load;
/**VALUE (r) register accessor: Current counter value (read-only)

You can [`read`](crate::Reg::read) this register and get [`value::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@value`] module*/
#[doc(alias = "VALUE")]
pub type Value = crate::Reg<value::ValueSpec>;
///Current counter value (read-only)
pub mod value;
/**CONTROL (rw) register accessor: Timer mode, prescaler, and interrupt control

You can [`read`](crate::Reg::read) this register and get [`control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@control`] module*/
#[doc(alias = "CONTROL")]
pub type Control = crate::Reg<control::ControlSpec>;
///Timer mode, prescaler, and interrupt control
pub mod control;
/**INTCLR (w) register accessor: Interrupt clear (write any value to clear)

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@intclr`] module*/
#[doc(alias = "INTCLR")]
pub type Intclr = crate::Reg<intclr::IntclrSpec>;
///Interrupt clear (write any value to clear)
pub mod intclr;
/**RIS (r) register accessor: Raw interrupt status

You can [`read`](crate::Reg::read) this register and get [`ris::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ris`] module*/
#[doc(alias = "RIS")]
pub type Ris = crate::Reg<ris::RisSpec>;
///Raw interrupt status
pub mod ris;
/**MIS (r) register accessor: Masked interrupt status

You can [`read`](crate::Reg::read) this register and get [`mis::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mis`] module*/
#[doc(alias = "MIS")]
pub type Mis = crate::Reg<mis::MisSpec>;
///Masked interrupt status
pub mod mis;
/**BGLOAD (rw) register accessor: Background load register (no immediate counter restart)

You can [`read`](crate::Reg::read) this register and get [`bgload::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgload::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@bgload`] module*/
#[doc(alias = "BGLOAD")]
pub type Bgload = crate::Reg<bgload::BgloadSpec>;
///Background load register (no immediate counter restart)
pub mod bgload;
