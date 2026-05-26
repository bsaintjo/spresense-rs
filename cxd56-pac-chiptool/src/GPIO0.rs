///GPIO Port 0 — GP_* output-enable/data registers.
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPIO0 {
    ptr: *mut u8,
}
unsafe impl Send for GPIO0 {}
unsafe impl Sync for GPIO0 {}
impl GPIO0 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    ///GPIO SYS pin 1 — I2C4 clock / Arduino D14.
    #[inline(always)]
    pub const fn GP_I2C4_BCK(self) -> crate::common::Reg<regs::GP_I2C4_BCK, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    ///GPIO APP pin 97 — I2S1_BCK / LED0 on Spresense main board.
    #[inline(always)]
    pub const fn PIN97(self) -> crate::common::Reg<regs::PIN97, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0168usize) as _) }
    }
    ///GPIO APP pin 98 — I2S1_LRCK / LED1 on Spresense main board.
    #[inline(always)]
    pub const fn PIN98(self) -> crate::common::Reg<regs::PIN98, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x016cusize) as _) }
    }
    ///GPIO APP pin 99 — I2S1_DATA_IN / LED2 on Spresense main board.
    #[inline(always)]
    pub const fn PIN99(self) -> crate::common::Reg<regs::PIN99, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0170usize) as _) }
    }
    ///GPIO APP pin 100 — I2S1_DATA_OUT / LED3 on Spresense main board.
    #[inline(always)]
    pub const fn PIN100(self) -> crate::common::Reg<regs::PIN100, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0174usize) as _) }
    }
}
pub mod regs;
