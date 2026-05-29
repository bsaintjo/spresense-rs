///Application domain clock enables.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct APP_CKEN(pub u32);
impl APP_CKEN {
    ///APP CPU clock enable.
    #[must_use]
    #[inline(always)]
    pub const fn APP_CPU(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    ///APP CPU clock enable.
    #[inline(always)]
    pub const fn set_APP_CPU(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    ///APP MCLK enable.
    #[must_use]
    #[inline(always)]
    pub const fn APP_MCLK(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    ///APP MCLK enable.
    #[inline(always)]
    pub const fn set_APP_MCLK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    ///APP AHB clock enable.
    #[must_use]
    #[inline(always)]
    pub const fn APP_AHB(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    ///APP AHB clock enable.
    #[inline(always)]
    pub const fn set_APP_AHB(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for APP_CKEN {
    #[inline(always)]
    fn default() -> APP_CKEN {
        APP_CKEN(0)
    }
}
impl core::fmt::Debug for APP_CKEN {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APP_CKEN")
            .field("APP_CPU", &self.APP_CPU())
            .field("APP_MCLK", &self.APP_MCLK())
            .field("APP_AHB", &self.APP_AHB())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for APP_CKEN {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "APP_CKEN {{ APP_CPU: {=bool:?}, APP_MCLK: {=bool:?}, APP_AHB: {=bool:?} }}",
            self.APP_CPU(),
            self.APP_MCLK(),
            self.APP_AHB()
        )
    }
}
///Application domain clock source select.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct APP_CKSEL(pub u32);
impl APP_CKSEL {
    ///Audio MCLK source select.
    #[must_use]
    #[inline(always)]
    pub const fn AUD_MCLK(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    ///Audio MCLK source select.
    #[inline(always)]
    pub const fn set_AUD_MCLK(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
}
impl Default for APP_CKSEL {
    #[inline(always)]
    fn default() -> APP_CKSEL {
        APP_CKSEL(0)
    }
}
impl core::fmt::Debug for APP_CKSEL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APP_CKSEL")
            .field("AUD_MCLK", &self.AUD_MCLK())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for APP_CKSEL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "APP_CKSEL {{ AUD_MCLK: {=u8:?} }}", self.AUD_MCLK())
    }
}
///Chip identification register (read-only).
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CHIP_ID(pub u32);
impl CHIP_ID {
    ///Chip ID value.
    #[must_use]
    #[inline(always)]
    pub const fn ID(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    ///Chip ID value.
    #[inline(always)]
    pub const fn set_ID(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for CHIP_ID {
    #[inline(always)]
    fn default() -> CHIP_ID {
        CHIP_ID(0)
    }
}
impl core::fmt::Debug for CHIP_ID {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CHIP_ID").field("ID", &self.ID()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CHIP_ID {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CHIP_ID {{ ID: {=u32:?} }}", self.ID())
    }
}
///GNSS DSP clock enables.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GNSDSP_CKEN(pub u32);
impl GNSDSP_CKEN {
    ///GNSS DSP P1 clock enable.
    #[must_use]
    #[inline(always)]
    pub const fn GNSDSP_P1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    ///GNSS DSP P1 clock enable.
    #[inline(always)]
    pub const fn set_GNSDSP_P1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    ///GNSS DSP coprocessor clock enable.
    #[must_use]
    #[inline(always)]
    pub const fn GNSDSP_COP(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    ///GNSS DSP coprocessor clock enable.
    #[inline(always)]
    pub const fn set_GNSDSP_COP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for GNSDSP_CKEN {
    #[inline(always)]
    fn default() -> GNSDSP_CKEN {
        GNSDSP_CKEN(0)
    }
}
impl core::fmt::Debug for GNSDSP_CKEN {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GNSDSP_CKEN")
            .field("GNSDSP_P1", &self.GNSDSP_P1())
            .field("GNSDSP_COP", &self.GNSDSP_COP())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GNSDSP_CKEN {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GNSDSP_CKEN {{ GNSDSP_P1: {=bool:?}, GNSDSP_COP: {=bool:?} }}",
            self.GNSDSP_P1(),
            self.GNSDSP_COP()
        )
    }
}
///GPIO interrupt masked status, slots 0–11 (ISR reads this).
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PMU_WAKE_TRIG0(pub u32);
impl PMU_WAKE_TRIG0 {
    ///Slot 0 masked trigger status (EXDEVICE_0).
    #[must_use]
    #[inline(always)]
    pub const fn SLOT0(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    ///Slot 0 masked trigger status (EXDEVICE_0).
    #[inline(always)]
    pub const fn set_SLOT0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    ///Slot 1 masked trigger status (EXDEVICE_1).
    #[must_use]
    #[inline(always)]
    pub const fn SLOT1(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    ///Slot 1 masked trigger status (EXDEVICE_1).
    #[inline(always)]
    pub const fn set_SLOT1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    ///Slot 2 masked trigger status (EXDEVICE_2).
    #[must_use]
    #[inline(always)]
    pub const fn SLOT2(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    ///Slot 2 masked trigger status (EXDEVICE_2).
    #[inline(always)]
    pub const fn set_SLOT2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    ///Slot 3 masked trigger status (EXDEVICE_3).
    #[must_use]
    #[inline(always)]
    pub const fn SLOT3(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    ///Slot 3 masked trigger status (EXDEVICE_3).
    #[inline(always)]
    pub const fn set_SLOT3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    ///Slot 4 masked trigger status (EXDEVICE_4).
    #[must_use]
    #[inline(always)]
    pub const fn SLOT4(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    ///Slot 4 masked trigger status (EXDEVICE_4).
    #[inline(always)]
    pub const fn set_SLOT4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    ///Slot 5 masked trigger status (EXDEVICE_5).
    #[must_use]
    #[inline(always)]
    pub const fn SLOT5(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    ///Slot 5 masked trigger status (EXDEVICE_5).
    #[inline(always)]
    pub const fn set_SLOT5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    ///Slot 6 masked trigger status (EXDEVICE_6).
    #[must_use]
    #[inline(always)]
    pub const fn SLOT6(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    ///Slot 6 masked trigger status (EXDEVICE_6).
    #[inline(always)]
    pub const fn set_SLOT6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    ///Slot 7 masked trigger status (EXDEVICE_7).
    #[must_use]
    #[inline(always)]
    pub const fn SLOT7(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    ///Slot 7 masked trigger status (EXDEVICE_7).
    #[inline(always)]
    pub const fn set_SLOT7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    ///Slot 8 masked trigger status (EXDEVICE_8).
    #[must_use]
    #[inline(always)]
    pub const fn SLOT8(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    ///Slot 8 masked trigger status (EXDEVICE_8).
    #[inline(always)]
    pub const fn set_SLOT8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    ///Slot 9 masked trigger status (EXDEVICE_9).
    #[must_use]
    #[inline(always)]
    pub const fn SLOT9(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    ///Slot 9 masked trigger status (EXDEVICE_9).
    #[inline(always)]
    pub const fn set_SLOT9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    ///Slot 10 masked trigger status (EXDEVICE_10).
    #[must_use]
    #[inline(always)]
    pub const fn SLOT10(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    ///Slot 10 masked trigger status (EXDEVICE_10).
    #[inline(always)]
    pub const fn set_SLOT10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    ///Slot 11 masked trigger status (EXDEVICE_11).
    #[must_use]
    #[inline(always)]
    pub const fn SLOT11(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    ///Slot 11 masked trigger status (EXDEVICE_11).
    #[inline(always)]
    pub const fn set_SLOT11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
}
impl Default for PMU_WAKE_TRIG0 {
    #[inline(always)]
    fn default() -> PMU_WAKE_TRIG0 {
        PMU_WAKE_TRIG0(0)
    }
}
impl core::fmt::Debug for PMU_WAKE_TRIG0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PMU_WAKE_TRIG0")
            .field("SLOT0", &self.SLOT0())
            .field("SLOT1", &self.SLOT1())
            .field("SLOT2", &self.SLOT2())
            .field("SLOT3", &self.SLOT3())
            .field("SLOT4", &self.SLOT4())
            .field("SLOT5", &self.SLOT5())
            .field("SLOT6", &self.SLOT6())
            .field("SLOT7", &self.SLOT7())
            .field("SLOT8", &self.SLOT8())
            .field("SLOT9", &self.SLOT9())
            .field("SLOT10", &self.SLOT10())
            .field("SLOT11", &self.SLOT11())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PMU_WAKE_TRIG0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PMU_WAKE_TRIG0 {{ SLOT0: {=bool:?}, SLOT1: {=bool:?}, SLOT2: {=bool:?}, SLOT3: {=bool:?}, SLOT4: {=bool:?}, SLOT5: {=bool:?}, SLOT6: {=bool:?}, SLOT7: {=bool:?}, SLOT8: {=bool:?}, SLOT9: {=bool:?}, SLOT10: {=bool:?}, SLOT11: {=bool:?} }}",
            self.SLOT0(),
            self.SLOT1(),
            self.SLOT2(),
            self.SLOT3(),
            self.SLOT4(),
            self.SLOT5(),
            self.SLOT6(),
            self.SLOT7(),
            self.SLOT8(),
            self.SLOT9(),
            self.SLOT10(),
            self.SLOT11()
        )
    }
}
///GPIO interrupt clear register, slots 0–11 (write 1<<(16+slot) to clear).
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PMU_WAKE_TRIG0_CLR(pub u32);
impl PMU_WAKE_TRIG0_CLR {
    ///Clear slot 0 latched interrupt (EXDEVICE_0).
    #[must_use]
    #[inline(always)]
    pub const fn SLOT0(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    ///Clear slot 0 latched interrupt (EXDEVICE_0).
    #[inline(always)]
    pub const fn set_SLOT0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    ///Clear slot 1 latched interrupt (EXDEVICE_1).
    #[must_use]
    #[inline(always)]
    pub const fn SLOT1(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    ///Clear slot 1 latched interrupt (EXDEVICE_1).
    #[inline(always)]
    pub const fn set_SLOT1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    ///Clear slot 2 latched interrupt (EXDEVICE_2).
    #[must_use]
    #[inline(always)]
    pub const fn SLOT2(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    ///Clear slot 2 latched interrupt (EXDEVICE_2).
    #[inline(always)]
    pub const fn set_SLOT2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    ///Clear slot 3 latched interrupt (EXDEVICE_3).
    #[must_use]
    #[inline(always)]
    pub const fn SLOT3(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    ///Clear slot 3 latched interrupt (EXDEVICE_3).
    #[inline(always)]
    pub const fn set_SLOT3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    ///Clear slot 4 latched interrupt (EXDEVICE_4).
    #[must_use]
    #[inline(always)]
    pub const fn SLOT4(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    ///Clear slot 4 latched interrupt (EXDEVICE_4).
    #[inline(always)]
    pub const fn set_SLOT4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    ///Clear slot 5 latched interrupt (EXDEVICE_5).
    #[must_use]
    #[inline(always)]
    pub const fn SLOT5(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    ///Clear slot 5 latched interrupt (EXDEVICE_5).
    #[inline(always)]
    pub const fn set_SLOT5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    ///Clear slot 6 latched interrupt (EXDEVICE_6).
    #[must_use]
    #[inline(always)]
    pub const fn SLOT6(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    ///Clear slot 6 latched interrupt (EXDEVICE_6).
    #[inline(always)]
    pub const fn set_SLOT6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    ///Clear slot 7 latched interrupt (EXDEVICE_7).
    #[must_use]
    #[inline(always)]
    pub const fn SLOT7(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    ///Clear slot 7 latched interrupt (EXDEVICE_7).
    #[inline(always)]
    pub const fn set_SLOT7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    ///Clear slot 8 latched interrupt (EXDEVICE_8).
    #[must_use]
    #[inline(always)]
    pub const fn SLOT8(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    ///Clear slot 8 latched interrupt (EXDEVICE_8).
    #[inline(always)]
    pub const fn set_SLOT8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    ///Clear slot 9 latched interrupt (EXDEVICE_9).
    #[must_use]
    #[inline(always)]
    pub const fn SLOT9(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    ///Clear slot 9 latched interrupt (EXDEVICE_9).
    #[inline(always)]
    pub const fn set_SLOT9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    ///Clear slot 10 latched interrupt (EXDEVICE_10).
    #[must_use]
    #[inline(always)]
    pub const fn SLOT10(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    ///Clear slot 10 latched interrupt (EXDEVICE_10).
    #[inline(always)]
    pub const fn set_SLOT10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    ///Clear slot 11 latched interrupt (EXDEVICE_11).
    #[must_use]
    #[inline(always)]
    pub const fn SLOT11(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    ///Clear slot 11 latched interrupt (EXDEVICE_11).
    #[inline(always)]
    pub const fn set_SLOT11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
}
impl Default for PMU_WAKE_TRIG0_CLR {
    #[inline(always)]
    fn default() -> PMU_WAKE_TRIG0_CLR {
        PMU_WAKE_TRIG0_CLR(0)
    }
}
impl core::fmt::Debug for PMU_WAKE_TRIG0_CLR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PMU_WAKE_TRIG0_CLR")
            .field("SLOT0", &self.SLOT0())
            .field("SLOT1", &self.SLOT1())
            .field("SLOT2", &self.SLOT2())
            .field("SLOT3", &self.SLOT3())
            .field("SLOT4", &self.SLOT4())
            .field("SLOT5", &self.SLOT5())
            .field("SLOT6", &self.SLOT6())
            .field("SLOT7", &self.SLOT7())
            .field("SLOT8", &self.SLOT8())
            .field("SLOT9", &self.SLOT9())
            .field("SLOT10", &self.SLOT10())
            .field("SLOT11", &self.SLOT11())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PMU_WAKE_TRIG0_CLR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PMU_WAKE_TRIG0_CLR {{ SLOT0: {=bool:?}, SLOT1: {=bool:?}, SLOT2: {=bool:?}, SLOT3: {=bool:?}, SLOT4: {=bool:?}, SLOT5: {=bool:?}, SLOT6: {=bool:?}, SLOT7: {=bool:?}, SLOT8: {=bool:?}, SLOT9: {=bool:?}, SLOT10: {=bool:?}, SLOT11: {=bool:?} }}",
            self.SLOT0(),
            self.SLOT1(),
            self.SLOT2(),
            self.SLOT3(),
            self.SLOT4(),
            self.SLOT5(),
            self.SLOT6(),
            self.SLOT7(),
            self.SLOT8(),
            self.SLOT9(),
            self.SLOT10(),
            self.SLOT11()
        )
    }
}
///GPIO interrupt raw (pre-mask) status, slots 0–11.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PMU_WAKE_TRIG0_RAW(pub u32);
impl PMU_WAKE_TRIG0_RAW {
    ///Slot 0 raw trigger status (EXDEVICE_0).
    #[must_use]
    #[inline(always)]
    pub const fn SLOT0(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    ///Slot 0 raw trigger status (EXDEVICE_0).
    #[inline(always)]
    pub const fn set_SLOT0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    ///Slot 1 raw trigger status (EXDEVICE_1).
    #[must_use]
    #[inline(always)]
    pub const fn SLOT1(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    ///Slot 1 raw trigger status (EXDEVICE_1).
    #[inline(always)]
    pub const fn set_SLOT1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    ///Slot 2 raw trigger status (EXDEVICE_2).
    #[must_use]
    #[inline(always)]
    pub const fn SLOT2(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    ///Slot 2 raw trigger status (EXDEVICE_2).
    #[inline(always)]
    pub const fn set_SLOT2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    ///Slot 3 raw trigger status (EXDEVICE_3).
    #[must_use]
    #[inline(always)]
    pub const fn SLOT3(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    ///Slot 3 raw trigger status (EXDEVICE_3).
    #[inline(always)]
    pub const fn set_SLOT3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    ///Slot 4 raw trigger status (EXDEVICE_4).
    #[must_use]
    #[inline(always)]
    pub const fn SLOT4(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    ///Slot 4 raw trigger status (EXDEVICE_4).
    #[inline(always)]
    pub const fn set_SLOT4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    ///Slot 5 raw trigger status (EXDEVICE_5).
    #[must_use]
    #[inline(always)]
    pub const fn SLOT5(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    ///Slot 5 raw trigger status (EXDEVICE_5).
    #[inline(always)]
    pub const fn set_SLOT5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    ///Slot 6 raw trigger status (EXDEVICE_6).
    #[must_use]
    #[inline(always)]
    pub const fn SLOT6(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    ///Slot 6 raw trigger status (EXDEVICE_6).
    #[inline(always)]
    pub const fn set_SLOT6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    ///Slot 7 raw trigger status (EXDEVICE_7).
    #[must_use]
    #[inline(always)]
    pub const fn SLOT7(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    ///Slot 7 raw trigger status (EXDEVICE_7).
    #[inline(always)]
    pub const fn set_SLOT7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    ///Slot 8 raw trigger status (EXDEVICE_8).
    #[must_use]
    #[inline(always)]
    pub const fn SLOT8(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    ///Slot 8 raw trigger status (EXDEVICE_8).
    #[inline(always)]
    pub const fn set_SLOT8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    ///Slot 9 raw trigger status (EXDEVICE_9).
    #[must_use]
    #[inline(always)]
    pub const fn SLOT9(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    ///Slot 9 raw trigger status (EXDEVICE_9).
    #[inline(always)]
    pub const fn set_SLOT9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    ///Slot 10 raw trigger status (EXDEVICE_10).
    #[must_use]
    #[inline(always)]
    pub const fn SLOT10(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    ///Slot 10 raw trigger status (EXDEVICE_10).
    #[inline(always)]
    pub const fn set_SLOT10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    ///Slot 11 raw trigger status (EXDEVICE_11).
    #[must_use]
    #[inline(always)]
    pub const fn SLOT11(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    ///Slot 11 raw trigger status (EXDEVICE_11).
    #[inline(always)]
    pub const fn set_SLOT11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
}
impl Default for PMU_WAKE_TRIG0_RAW {
    #[inline(always)]
    fn default() -> PMU_WAKE_TRIG0_RAW {
        PMU_WAKE_TRIG0_RAW(0)
    }
}
impl core::fmt::Debug for PMU_WAKE_TRIG0_RAW {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PMU_WAKE_TRIG0_RAW")
            .field("SLOT0", &self.SLOT0())
            .field("SLOT1", &self.SLOT1())
            .field("SLOT2", &self.SLOT2())
            .field("SLOT3", &self.SLOT3())
            .field("SLOT4", &self.SLOT4())
            .field("SLOT5", &self.SLOT5())
            .field("SLOT6", &self.SLOT6())
            .field("SLOT7", &self.SLOT7())
            .field("SLOT8", &self.SLOT8())
            .field("SLOT9", &self.SLOT9())
            .field("SLOT10", &self.SLOT10())
            .field("SLOT11", &self.SLOT11())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PMU_WAKE_TRIG0_RAW {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PMU_WAKE_TRIG0_RAW {{ SLOT0: {=bool:?}, SLOT1: {=bool:?}, SLOT2: {=bool:?}, SLOT3: {=bool:?}, SLOT4: {=bool:?}, SLOT5: {=bool:?}, SLOT6: {=bool:?}, SLOT7: {=bool:?}, SLOT8: {=bool:?}, SLOT9: {=bool:?}, SLOT10: {=bool:?}, SLOT11: {=bool:?} }}",
            self.SLOT0(),
            self.SLOT1(),
            self.SLOT2(),
            self.SLOT3(),
            self.SLOT4(),
            self.SLOT5(),
            self.SLOT6(),
            self.SLOT7(),
            self.SLOT8(),
            self.SLOT9(),
            self.SLOT10(),
            self.SLOT11()
        )
    }
}
///SYSIOP sub-domain peripheral clock enables.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SYSIOP_SUB_CKEN(pub u32);
impl SYSIOP_SUB_CKEN {
    ///AHB bridge COMIF clock enable.
    #[must_use]
    #[inline(always)]
    pub const fn CK_AHB_BRG_COMIF(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    ///AHB bridge COMIF clock enable.
    #[inline(always)]
    pub const fn set_CK_AHB_BRG_COMIF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    ///COM bridge clock enable.
    #[must_use]
    #[inline(always)]
    pub const fn CK_COM_BRG(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    ///COM bridge clock enable.
    #[inline(always)]
    pub const fn set_CK_COM_BRG(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    ///AHB DMAC3 clock enable.
    #[must_use]
    #[inline(always)]
    pub const fn CK_AHB_DMAC3(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    ///AHB DMAC3 clock enable.
    #[inline(always)]
    pub const fn set_CK_AHB_DMAC3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    ///UART1 clock enable.
    #[must_use]
    #[inline(always)]
    pub const fn CK_UART1(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    ///UART1 clock enable.
    #[inline(always)]
    pub const fn set_CK_UART1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    ///SPI master clock enable.
    #[must_use]
    #[inline(always)]
    pub const fn CK_SPIM(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    ///SPI master clock enable.
    #[inline(always)]
    pub const fn set_CK_SPIM(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    ///I2C master clock enable.
    #[must_use]
    #[inline(always)]
    pub const fn CK_I2CM(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    ///I2C master clock enable.
    #[inline(always)]
    pub const fn set_CK_I2CM(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    ///SAKE HCLK enable.
    #[must_use]
    #[inline(always)]
    pub const fn CK_HCLK_SAKE(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    ///SAKE HCLK enable.
    #[inline(always)]
    pub const fn set_CK_HCLK_SAKE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    ///Serial flash controller HCLK enable.
    #[must_use]
    #[inline(always)]
    pub const fn CK_SFC_HCLK(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    ///Serial flash controller HCLK enable.
    #[inline(always)]
    pub const fn set_CK_SFC_HCLK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    ///Serial flash controller SFCLK enable.
    #[must_use]
    #[inline(always)]
    pub const fn CK_SFC_SFCLK(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    ///Serial flash controller SFCLK enable.
    #[inline(always)]
    pub const fn set_CK_SFC_SFCLK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    ///Serial flash controller HCLK low-speed enable.
    #[must_use]
    #[inline(always)]
    pub const fn CK_SFC_HCLK_LOW(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    ///Serial flash controller HCLK low-speed enable.
    #[inline(always)]
    pub const fn set_CK_SFC_HCLK_LOW(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    ///COM UART PCLK enable.
    #[must_use]
    #[inline(always)]
    pub const fn CK_COM_UART_PCLK(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    ///COM UART PCLK enable.
    #[inline(always)]
    pub const fn set_CK_COM_UART_PCLK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for SYSIOP_SUB_CKEN {
    #[inline(always)]
    fn default() -> SYSIOP_SUB_CKEN {
        SYSIOP_SUB_CKEN(0)
    }
}
impl core::fmt::Debug for SYSIOP_SUB_CKEN {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYSIOP_SUB_CKEN")
            .field("CK_AHB_BRG_COMIF", &self.CK_AHB_BRG_COMIF())
            .field("CK_COM_BRG", &self.CK_COM_BRG())
            .field("CK_AHB_DMAC3", &self.CK_AHB_DMAC3())
            .field("CK_UART1", &self.CK_UART1())
            .field("CK_SPIM", &self.CK_SPIM())
            .field("CK_I2CM", &self.CK_I2CM())
            .field("CK_HCLK_SAKE", &self.CK_HCLK_SAKE())
            .field("CK_SFC_HCLK", &self.CK_SFC_HCLK())
            .field("CK_SFC_SFCLK", &self.CK_SFC_SFCLK())
            .field("CK_SFC_HCLK_LOW", &self.CK_SFC_HCLK_LOW())
            .field("CK_COM_UART_PCLK", &self.CK_COM_UART_PCLK())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SYSIOP_SUB_CKEN {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SYSIOP_SUB_CKEN {{ CK_AHB_BRG_COMIF: {=bool:?}, CK_COM_BRG: {=bool:?}, CK_AHB_DMAC3: {=bool:?}, CK_UART1: {=bool:?}, CK_SPIM: {=bool:?}, CK_I2CM: {=bool:?}, CK_HCLK_SAKE: {=bool:?}, CK_SFC_HCLK: {=bool:?}, CK_SFC_SFCLK: {=bool:?}, CK_SFC_HCLK_LOW: {=bool:?}, CK_COM_UART_PCLK: {=bool:?} }}",
            self.CK_AHB_BRG_COMIF(),
            self.CK_COM_BRG(),
            self.CK_AHB_DMAC3(),
            self.CK_UART1(),
            self.CK_SPIM(),
            self.CK_I2CM(),
            self.CK_HCLK_SAKE(),
            self.CK_SFC_HCLK(),
            self.CK_SFC_SFCLK(),
            self.CK_SFC_HCLK_LOW(),
            self.CK_COM_UART_PCLK()
        )
    }
}
