///Hardware semaphores (16) for inter-CPU mutual exclusion.
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SPH {
    ptr: *mut u8,
}
unsafe impl Send for SPH {}
unsafe impl Sync for SPH {}
impl SPH {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    ///Semaphore request command (write-only).
    #[inline(always)]
    pub const fn REQ(self, n: usize) -> crate::common::Reg<regs::REQ, crate::common::W> {
        assert!(n < 16usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize + n * 16usize) as _) }
    }
    ///Semaphore status (read-only).
    #[inline(always)]
    pub const fn STS(self, n: usize) -> crate::common::Reg<regs::STS, crate::common::R> {
        assert!(n < 16usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize + n * 16usize) as _) }
    }
}
pub mod regs;
