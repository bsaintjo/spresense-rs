#[repr(C)]
///Register block
pub struct RegisterBlock {
    wrregpostcnt: Wrregpostcnt,
    wrregprecnt: Wrregprecnt,
    wrregreq: Wrregreq,
    _reserved3: [u8; 0x04],
    wrintpostcnt: Wrintpostcnt,
    wrintprecnt: Wrintprecnt,
    wrintctrl: Wrintctrl,
    wrintclr: Wrintclr,
    offsetval: Offsetval,
    offsetreq: Offsetreq,
    _reserved9: [u8; 0x08],
    rdreq: Rdreq,
    rdpostcnt: Rdpostcnt,
    rdprecnt: Rdprecnt,
    _reserved12: [u8; 0x04],
    rtpostcnt: Rtpostcnt,
    rtprecnt: Rtprecnt,
    _reserved14: [u8; 0x08],
    setalmpostcnt0: Setalmpostcnt0,
    setalmprecnt0: Setalmprecnt0,
    setalmpostcnt1: Setalmpostcnt1,
    setalmprecnt1: Setalmprecnt1,
    setalmpostcnt2: Setalmpostcnt2,
    setalmprecnt2: Setalmprecnt2,
    _reserved20: [u8; 0x08],
    almclr: Almclr,
    almouten0: Almouten0,
    almouten1: Almouten1,
    almouten2: Almouten2,
    almflg: Almflg,
    _reserved25: [u8; 0x0c],
    dbgsetalmpostcnt0: Dbgsetalmpostcnt0,
    dbgsetalmprecnt0: Dbgsetalmprecnt0,
    dbgsetalmpostcnt1: Dbgsetalmpostcnt1,
    dbgsetalmprecnt1: Dbgsetalmprecnt1,
    dbgsetalmpostcnt2: Dbgsetalmpostcnt2,
    dbgsetalmprecnt2: Dbgsetalmprecnt2,
}
impl RegisterBlock {
    ///0x00 - Write post-counter (32-bit integer seconds to set)
    #[inline(always)]
    pub const fn wrregpostcnt(&self) -> &Wrregpostcnt {
        &self.wrregpostcnt
    }
    ///0x04 - Write pre-counter (15-bit fractional 32 kHz ticks to set)
    #[inline(always)]
    pub const fn wrregprecnt(&self) -> &Wrregprecnt {
        &self.wrregprecnt
    }
    ///0x08 - Counter write request and busy status
    #[inline(always)]
    pub const fn wrregreq(&self) -> &Wrregreq {
        &self.wrregreq
    }
    ///0x10 - Write interrupt post-counter compare value
    #[inline(always)]
    pub const fn wrintpostcnt(&self) -> &Wrintpostcnt {
        &self.wrintpostcnt
    }
    ///0x14 - Write interrupt pre-counter compare value
    #[inline(always)]
    pub const fn wrintprecnt(&self) -> &Wrintprecnt {
        &self.wrintprecnt
    }
    ///0x18 - Write interrupt control
    #[inline(always)]
    pub const fn wrintctrl(&self) -> &Wrintctrl {
        &self.wrintctrl
    }
    ///0x1c - Write interrupt clear
    #[inline(always)]
    pub const fn wrintclr(&self) -> &Wrintclr {
        &self.wrintclr
    }
    ///0x20 - Frequency offset correction value
    #[inline(always)]
    pub const fn offsetval(&self) -> &Offsetval {
        &self.offsetval
    }
    ///0x24 - Frequency offset correction request and busy
    #[inline(always)]
    pub const fn offsetreq(&self) -> &Offsetreq {
        &self.offsetreq
    }
    ///0x30 - Read request (triggers a counter capture)
    #[inline(always)]
    pub const fn rdreq(&self) -> &Rdreq {
        &self.rdreq
    }
    ///0x34 - Captured post-counter value (integer seconds)
    #[inline(always)]
    pub const fn rdpostcnt(&self) -> &Rdpostcnt {
        &self.rdpostcnt
    }
    ///0x38 - Captured pre-counter value (15-bit fractional ticks)
    #[inline(always)]
    pub const fn rdprecnt(&self) -> &Rdprecnt {
        &self.rdprecnt
    }
    ///0x40 - Real-time post-counter (free-running read)
    #[inline(always)]
    pub const fn rtpostcnt(&self) -> &Rtpostcnt {
        &self.rtpostcnt
    }
    ///0x44 - Real-time pre-counter (free-running read)
    #[inline(always)]
    pub const fn rtprecnt(&self) -> &Rtprecnt {
        &self.rtprecnt
    }
    ///0x50 - Alarm 0 post-counter compare value
    #[inline(always)]
    pub const fn setalmpostcnt0(&self) -> &Setalmpostcnt0 {
        &self.setalmpostcnt0
    }
    ///0x54 - Alarm 0 pre-counter compare value
    #[inline(always)]
    pub const fn setalmprecnt0(&self) -> &Setalmprecnt0 {
        &self.setalmprecnt0
    }
    ///0x58 - Alarm 1 post-counter compare value
    #[inline(always)]
    pub const fn setalmpostcnt1(&self) -> &Setalmpostcnt1 {
        &self.setalmpostcnt1
    }
    ///0x5c - Alarm 1 pre-counter compare value
    #[inline(always)]
    pub const fn setalmprecnt1(&self) -> &Setalmprecnt1 {
        &self.setalmprecnt1
    }
    ///0x60 - Alarm 2 post-counter compare value
    #[inline(always)]
    pub const fn setalmpostcnt2(&self) -> &Setalmpostcnt2 {
        &self.setalmpostcnt2
    }
    ///0x64 - Alarm 2 pre-counter compare value
    #[inline(always)]
    pub const fn setalmprecnt2(&self) -> &Setalmprecnt2 {
        &self.setalmprecnt2
    }
    ///0x70 - Alarm flag clear register
    #[inline(always)]
    pub const fn almclr(&self) -> &Almclr {
        &self.almclr
    }
    ///0x74 - Alarm 0 output enable and busy status
    #[inline(always)]
    pub const fn almouten0(&self) -> &Almouten0 {
        &self.almouten0
    }
    ///0x78 - Alarm 1 output enable and busy status
    #[inline(always)]
    pub const fn almouten1(&self) -> &Almouten1 {
        &self.almouten1
    }
    ///0x7c - Alarm 2 output enable and busy status
    #[inline(always)]
    pub const fn almouten2(&self) -> &Almouten2 {
        &self.almouten2
    }
    ///0x80 - Alarm flag status (read-only)
    #[inline(always)]
    pub const fn almflg(&self) -> &Almflg {
        &self.almflg
    }
    ///0x90 - Debug alarm 0 post-counter compare
    #[inline(always)]
    pub const fn dbgsetalmpostcnt0(&self) -> &Dbgsetalmpostcnt0 {
        &self.dbgsetalmpostcnt0
    }
    ///0x94 - Debug alarm 0 pre-counter compare
    #[inline(always)]
    pub const fn dbgsetalmprecnt0(&self) -> &Dbgsetalmprecnt0 {
        &self.dbgsetalmprecnt0
    }
    ///0x98 - Debug alarm 1 post-counter compare
    #[inline(always)]
    pub const fn dbgsetalmpostcnt1(&self) -> &Dbgsetalmpostcnt1 {
        &self.dbgsetalmpostcnt1
    }
    ///0x9c - Debug alarm 1 pre-counter compare
    #[inline(always)]
    pub const fn dbgsetalmprecnt1(&self) -> &Dbgsetalmprecnt1 {
        &self.dbgsetalmprecnt1
    }
    ///0xa0 - Debug alarm 2 post-counter compare
    #[inline(always)]
    pub const fn dbgsetalmpostcnt2(&self) -> &Dbgsetalmpostcnt2 {
        &self.dbgsetalmpostcnt2
    }
    ///0xa4 - Debug alarm 2 pre-counter compare
    #[inline(always)]
    pub const fn dbgsetalmprecnt2(&self) -> &Dbgsetalmprecnt2 {
        &self.dbgsetalmprecnt2
    }
}
/**WRREGPOSTCNT (rw) register accessor: Write post-counter (32-bit integer seconds to set)

You can [`read`](crate::Reg::read) this register and get [`wrregpostcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wrregpostcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@wrregpostcnt`] module*/
#[doc(alias = "WRREGPOSTCNT")]
pub type Wrregpostcnt = crate::Reg<wrregpostcnt::WrregpostcntSpec>;
///Write post-counter (32-bit integer seconds to set)
pub mod wrregpostcnt;
/**WRREGPRECNT (rw) register accessor: Write pre-counter (15-bit fractional 32 kHz ticks to set)

You can [`read`](crate::Reg::read) this register and get [`wrregprecnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wrregprecnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@wrregprecnt`] module*/
#[doc(alias = "WRREGPRECNT")]
pub type Wrregprecnt = crate::Reg<wrregprecnt::WrregprecntSpec>;
///Write pre-counter (15-bit fractional 32 kHz ticks to set)
pub mod wrregprecnt;
/**WRREGREQ (rw) register accessor: Counter write request and busy status

You can [`read`](crate::Reg::read) this register and get [`wrregreq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wrregreq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@wrregreq`] module*/
#[doc(alias = "WRREGREQ")]
pub type Wrregreq = crate::Reg<wrregreq::WrregreqSpec>;
///Counter write request and busy status
pub mod wrregreq;
/**WRINTPOSTCNT (rw) register accessor: Write interrupt post-counter compare value

You can [`read`](crate::Reg::read) this register and get [`wrintpostcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wrintpostcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@wrintpostcnt`] module*/
#[doc(alias = "WRINTPOSTCNT")]
pub type Wrintpostcnt = crate::Reg<wrintpostcnt::WrintpostcntSpec>;
///Write interrupt post-counter compare value
pub mod wrintpostcnt;
/**WRINTPRECNT (rw) register accessor: Write interrupt pre-counter compare value

You can [`read`](crate::Reg::read) this register and get [`wrintprecnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wrintprecnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@wrintprecnt`] module*/
#[doc(alias = "WRINTPRECNT")]
pub type Wrintprecnt = crate::Reg<wrintprecnt::WrintprecntSpec>;
///Write interrupt pre-counter compare value
pub mod wrintprecnt;
/**WRINTCTRL (rw) register accessor: Write interrupt control

You can [`read`](crate::Reg::read) this register and get [`wrintctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wrintctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@wrintctrl`] module*/
#[doc(alias = "WRINTCTRL")]
pub type Wrintctrl = crate::Reg<wrintctrl::WrintctrlSpec>;
///Write interrupt control
pub mod wrintctrl;
/**WRINTCLR (rw) register accessor: Write interrupt clear

You can [`read`](crate::Reg::read) this register and get [`wrintclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wrintclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@wrintclr`] module*/
#[doc(alias = "WRINTCLR")]
pub type Wrintclr = crate::Reg<wrintclr::WrintclrSpec>;
///Write interrupt clear
pub mod wrintclr;
/**OFFSETVAL (rw) register accessor: Frequency offset correction value

You can [`read`](crate::Reg::read) this register and get [`offsetval::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`offsetval::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@offsetval`] module*/
#[doc(alias = "OFFSETVAL")]
pub type Offsetval = crate::Reg<offsetval::OffsetvalSpec>;
///Frequency offset correction value
pub mod offsetval;
/**OFFSETREQ (rw) register accessor: Frequency offset correction request and busy

You can [`read`](crate::Reg::read) this register and get [`offsetreq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`offsetreq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@offsetreq`] module*/
#[doc(alias = "OFFSETREQ")]
pub type Offsetreq = crate::Reg<offsetreq::OffsetreqSpec>;
///Frequency offset correction request and busy
pub mod offsetreq;
/**RDREQ (rw) register accessor: Read request (triggers a counter capture)

You can [`read`](crate::Reg::read) this register and get [`rdreq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rdreq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rdreq`] module*/
#[doc(alias = "RDREQ")]
pub type Rdreq = crate::Reg<rdreq::RdreqSpec>;
///Read request (triggers a counter capture)
pub mod rdreq;
/**RDPOSTCNT (r) register accessor: Captured post-counter value (integer seconds)

You can [`read`](crate::Reg::read) this register and get [`rdpostcnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rdpostcnt`] module*/
#[doc(alias = "RDPOSTCNT")]
pub type Rdpostcnt = crate::Reg<rdpostcnt::RdpostcntSpec>;
///Captured post-counter value (integer seconds)
pub mod rdpostcnt;
/**RDPRECNT (r) register accessor: Captured pre-counter value (15-bit fractional ticks)

You can [`read`](crate::Reg::read) this register and get [`rdprecnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rdprecnt`] module*/
#[doc(alias = "RDPRECNT")]
pub type Rdprecnt = crate::Reg<rdprecnt::RdprecntSpec>;
///Captured pre-counter value (15-bit fractional ticks)
pub mod rdprecnt;
/**RTPOSTCNT (r) register accessor: Real-time post-counter (free-running read)

You can [`read`](crate::Reg::read) this register and get [`rtpostcnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rtpostcnt`] module*/
#[doc(alias = "RTPOSTCNT")]
pub type Rtpostcnt = crate::Reg<rtpostcnt::RtpostcntSpec>;
///Real-time post-counter (free-running read)
pub mod rtpostcnt;
/**RTPRECNT (r) register accessor: Real-time pre-counter (free-running read)

You can [`read`](crate::Reg::read) this register and get [`rtprecnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rtprecnt`] module*/
#[doc(alias = "RTPRECNT")]
pub type Rtprecnt = crate::Reg<rtprecnt::RtprecntSpec>;
///Real-time pre-counter (free-running read)
pub mod rtprecnt;
/**SETALMPOSTCNT0 (rw) register accessor: Alarm 0 post-counter compare value

You can [`read`](crate::Reg::read) this register and get [`setalmpostcnt0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`setalmpostcnt0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@setalmpostcnt0`] module*/
#[doc(alias = "SETALMPOSTCNT0")]
pub type Setalmpostcnt0 = crate::Reg<setalmpostcnt0::Setalmpostcnt0Spec>;
///Alarm 0 post-counter compare value
pub mod setalmpostcnt0;
/**SETALMPRECNT0 (rw) register accessor: Alarm 0 pre-counter compare value

You can [`read`](crate::Reg::read) this register and get [`setalmprecnt0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`setalmprecnt0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@setalmprecnt0`] module*/
#[doc(alias = "SETALMPRECNT0")]
pub type Setalmprecnt0 = crate::Reg<setalmprecnt0::Setalmprecnt0Spec>;
///Alarm 0 pre-counter compare value
pub mod setalmprecnt0;
/**SETALMPOSTCNT1 (rw) register accessor: Alarm 1 post-counter compare value

You can [`read`](crate::Reg::read) this register and get [`setalmpostcnt1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`setalmpostcnt1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@setalmpostcnt1`] module*/
#[doc(alias = "SETALMPOSTCNT1")]
pub type Setalmpostcnt1 = crate::Reg<setalmpostcnt1::Setalmpostcnt1Spec>;
///Alarm 1 post-counter compare value
pub mod setalmpostcnt1;
/**SETALMPRECNT1 (rw) register accessor: Alarm 1 pre-counter compare value

You can [`read`](crate::Reg::read) this register and get [`setalmprecnt1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`setalmprecnt1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@setalmprecnt1`] module*/
#[doc(alias = "SETALMPRECNT1")]
pub type Setalmprecnt1 = crate::Reg<setalmprecnt1::Setalmprecnt1Spec>;
///Alarm 1 pre-counter compare value
pub mod setalmprecnt1;
/**SETALMPOSTCNT2 (rw) register accessor: Alarm 2 post-counter compare value

You can [`read`](crate::Reg::read) this register and get [`setalmpostcnt2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`setalmpostcnt2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@setalmpostcnt2`] module*/
#[doc(alias = "SETALMPOSTCNT2")]
pub type Setalmpostcnt2 = crate::Reg<setalmpostcnt2::Setalmpostcnt2Spec>;
///Alarm 2 post-counter compare value
pub mod setalmpostcnt2;
/**SETALMPRECNT2 (rw) register accessor: Alarm 2 pre-counter compare value

You can [`read`](crate::Reg::read) this register and get [`setalmprecnt2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`setalmprecnt2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@setalmprecnt2`] module*/
#[doc(alias = "SETALMPRECNT2")]
pub type Setalmprecnt2 = crate::Reg<setalmprecnt2::Setalmprecnt2Spec>;
///Alarm 2 pre-counter compare value
pub mod setalmprecnt2;
/**ALMCLR (rw) register accessor: Alarm flag clear register

You can [`read`](crate::Reg::read) this register and get [`almclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`almclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@almclr`] module*/
#[doc(alias = "ALMCLR")]
pub type Almclr = crate::Reg<almclr::AlmclrSpec>;
///Alarm flag clear register
pub mod almclr;
/**ALMOUTEN0 (rw) register accessor: Alarm 0 output enable and busy status

You can [`read`](crate::Reg::read) this register and get [`almouten0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`almouten0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@almouten0`] module*/
#[doc(alias = "ALMOUTEN0")]
pub type Almouten0 = crate::Reg<almouten0::Almouten0Spec>;
///Alarm 0 output enable and busy status
pub mod almouten0;
/**ALMOUTEN1 (rw) register accessor: Alarm 1 output enable and busy status

You can [`read`](crate::Reg::read) this register and get [`almouten1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`almouten1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@almouten1`] module*/
#[doc(alias = "ALMOUTEN1")]
pub type Almouten1 = crate::Reg<almouten1::Almouten1Spec>;
///Alarm 1 output enable and busy status
pub mod almouten1;
/**ALMOUTEN2 (rw) register accessor: Alarm 2 output enable and busy status

You can [`read`](crate::Reg::read) this register and get [`almouten2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`almouten2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@almouten2`] module*/
#[doc(alias = "ALMOUTEN2")]
pub type Almouten2 = crate::Reg<almouten2::Almouten2Spec>;
///Alarm 2 output enable and busy status
pub mod almouten2;
/**ALMFLG (r) register accessor: Alarm flag status (read-only)

You can [`read`](crate::Reg::read) this register and get [`almflg::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@almflg`] module*/
#[doc(alias = "ALMFLG")]
pub type Almflg = crate::Reg<almflg::AlmflgSpec>;
///Alarm flag status (read-only)
pub mod almflg;
/**DBGSETALMPOSTCNT0 (rw) register accessor: Debug alarm 0 post-counter compare

You can [`read`](crate::Reg::read) this register and get [`dbgsetalmpostcnt0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgsetalmpostcnt0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dbgsetalmpostcnt0`] module*/
#[doc(alias = "DBGSETALMPOSTCNT0")]
pub type Dbgsetalmpostcnt0 = crate::Reg<dbgsetalmpostcnt0::Dbgsetalmpostcnt0Spec>;
///Debug alarm 0 post-counter compare
pub mod dbgsetalmpostcnt0;
/**DBGSETALMPRECNT0 (rw) register accessor: Debug alarm 0 pre-counter compare

You can [`read`](crate::Reg::read) this register and get [`dbgsetalmprecnt0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgsetalmprecnt0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dbgsetalmprecnt0`] module*/
#[doc(alias = "DBGSETALMPRECNT0")]
pub type Dbgsetalmprecnt0 = crate::Reg<dbgsetalmprecnt0::Dbgsetalmprecnt0Spec>;
///Debug alarm 0 pre-counter compare
pub mod dbgsetalmprecnt0;
/**DBGSETALMPOSTCNT1 (rw) register accessor: Debug alarm 1 post-counter compare

You can [`read`](crate::Reg::read) this register and get [`dbgsetalmpostcnt1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgsetalmpostcnt1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dbgsetalmpostcnt1`] module*/
#[doc(alias = "DBGSETALMPOSTCNT1")]
pub type Dbgsetalmpostcnt1 = crate::Reg<dbgsetalmpostcnt1::Dbgsetalmpostcnt1Spec>;
///Debug alarm 1 post-counter compare
pub mod dbgsetalmpostcnt1;
/**DBGSETALMPRECNT1 (rw) register accessor: Debug alarm 1 pre-counter compare

You can [`read`](crate::Reg::read) this register and get [`dbgsetalmprecnt1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgsetalmprecnt1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dbgsetalmprecnt1`] module*/
#[doc(alias = "DBGSETALMPRECNT1")]
pub type Dbgsetalmprecnt1 = crate::Reg<dbgsetalmprecnt1::Dbgsetalmprecnt1Spec>;
///Debug alarm 1 pre-counter compare
pub mod dbgsetalmprecnt1;
/**DBGSETALMPOSTCNT2 (rw) register accessor: Debug alarm 2 post-counter compare

You can [`read`](crate::Reg::read) this register and get [`dbgsetalmpostcnt2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgsetalmpostcnt2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dbgsetalmpostcnt2`] module*/
#[doc(alias = "DBGSETALMPOSTCNT2")]
pub type Dbgsetalmpostcnt2 = crate::Reg<dbgsetalmpostcnt2::Dbgsetalmpostcnt2Spec>;
///Debug alarm 2 post-counter compare
pub mod dbgsetalmpostcnt2;
/**DBGSETALMPRECNT2 (rw) register accessor: Debug alarm 2 pre-counter compare

You can [`read`](crate::Reg::read) this register and get [`dbgsetalmprecnt2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgsetalmprecnt2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dbgsetalmprecnt2`] module*/
#[doc(alias = "DBGSETALMPRECNT2")]
pub type Dbgsetalmprecnt2 = crate::Reg<dbgsetalmprecnt2::Dbgsetalmprecnt2Spec>;
///Debug alarm 2 pre-counter compare
pub mod dbgsetalmprecnt2;
