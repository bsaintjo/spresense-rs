///APP / GNSS sub-domain clock and reset controller.
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TOPREG_SUB {
    ptr: *mut u8,
}
unsafe impl Send for TOPREG_SUB {}
unsafe impl Sync for TOPREG_SUB {}
impl TOPREG_SUB {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    ///GNSS DSP software reset.
    #[inline(always)]
    pub const fn SWRESET_GNSDSP(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0404usize) as _) }
    }
    ///Application domain software reset.
    #[inline(always)]
    pub const fn SWRESET_APP(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0408usize) as _) }
    }
    ///System CPU clock enable.
    #[inline(always)]
    pub const fn SYSCPU_CKEN(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0410usize) as _) }
    }
    ///Application domain clock enables.
    #[inline(always)]
    pub const fn APP_CKEN(self) -> crate::common::Reg<regs::APP_CKEN, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0414usize) as _) }
    }
    ///Application domain clock source select.
    #[inline(always)]
    pub const fn APP_CKSEL(self) -> crate::common::Reg<regs::APP_CKSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0418usize) as _) }
    }
    ///Application domain clock divider.
    #[inline(always)]
    pub const fn APP_DIV(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x041cusize) as _) }
    }
    ///SYSIOP sub-domain peripheral clock enables.
    #[inline(always)]
    pub const fn SYSIOP_SUB_CKEN(
        self,
    ) -> crate::common::Reg<regs::SYSIOP_SUB_CKEN, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0420usize) as _) }
    }
    ///GPIO interrupt clear register, slots 0–11 (write 1<<(16+slot) to clear).
    #[inline(always)]
    pub const fn PMU_WAKE_TRIG0_CLR(
        self,
    ) -> crate::common::Reg<regs::PMU_WAKE_TRIG0_CLR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0430usize) as _) }
    }
    ///GPIO interrupt clear register, second bank.
    #[inline(always)]
    pub const fn PMU_WAKE_TRIG1_CLR(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0434usize) as _) }
    }
    ///GPIO interrupt raw (pre-mask) status, slots 0–11.
    #[inline(always)]
    pub const fn PMU_WAKE_TRIG0_RAW(
        self,
    ) -> crate::common::Reg<regs::PMU_WAKE_TRIG0_RAW, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0438usize) as _) }
    }
    ///GPIO interrupt raw status, second bank.
    #[inline(always)]
    pub const fn PMU_WAKE_TRIG1_RAW(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x043cusize) as _) }
    }
    ///GPIO interrupt masked status, slots 0–11 (ISR reads this).
    #[inline(always)]
    pub const fn PMU_WAKE_TRIG0(
        self,
    ) -> crate::common::Reg<regs::PMU_WAKE_TRIG0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0440usize) as _) }
    }
    ///GPIO interrupt masked status, second bank.
    #[inline(always)]
    pub const fn PMU_WAKE_TRIG1(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0444usize) as _) }
    }
    ///GNSS DSP clock enables.
    #[inline(always)]
    pub const fn GNSDSP_CKEN(self) -> crate::common::Reg<regs::GNSDSP_CKEN, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0c20usize) as _) }
    }
    ///GNSS clock divider.
    #[inline(always)]
    pub const fn GNSS_DIV(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0c28usize) as _) }
    }
    ///Chip identification register (read-only).
    #[inline(always)]
    pub const fn CHIP_ID(self) -> crate::common::Reg<regs::CHIP_ID, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1490usize) as _) }
    }
}
pub mod regs;
