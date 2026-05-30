///Semaphore request command (write-only).
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct REQ(pub u32);
impl REQ {
    ///Request command.
    #[must_use]
    #[inline(always)]
    pub const fn REQ(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    ///Request command.
    #[inline(always)]
    pub const fn set_REQ(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
}
impl Default for REQ {
    #[inline(always)]
    fn default() -> REQ {
        REQ(0)
    }
}
impl core::fmt::Debug for REQ {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REQ").field("REQ", &self.REQ()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for REQ {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "REQ {{ REQ: {=u8:?} }}", self.REQ())
    }
}
///Semaphore status (read-only).
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct STS(pub u32);
impl STS {
    ///Lock state.
    #[must_use]
    #[inline(always)]
    pub const fn STATE(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    ///Lock state.
    #[inline(always)]
    pub const fn set_STATE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    ///CPU id currently holding the lock.
    #[must_use]
    #[inline(always)]
    pub const fn LOCK_OWNER(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    ///CPU id currently holding the lock.
    #[inline(always)]
    pub const fn set_LOCK_OWNER(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    ///CPU id holding the reservation.
    #[must_use]
    #[inline(always)]
    pub const fn RESV_OWNER(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x1f;
        val as u8
    }
    ///CPU id holding the reservation.
    #[inline(always)]
    pub const fn set_RESV_OWNER(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
    }
}
impl Default for STS {
    #[inline(always)]
    fn default() -> STS {
        STS(0)
    }
}
impl core::fmt::Debug for STS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STS")
            .field("STATE", &self.STATE())
            .field("LOCK_OWNER", &self.LOCK_OWNER())
            .field("RESV_OWNER", &self.RESV_OWNER())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for STS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "STS {{ STATE: {=u8:?}, LOCK_OWNER: {=u8:?}, RESV_OWNER: {=u8:?} }}",
            self.STATE(),
            self.LOCK_OWNER(),
            self.RESV_OWNER()
        )
    }
}
