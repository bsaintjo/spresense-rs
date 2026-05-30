//! CPU identity for the APP-domain Cortex-M4F cluster.
//!
//! The CXD5602 APP cluster has up to six Cortex-M4F cores (`Core0`..`Core5`).
//! `Core0` is the main core that the boot ROM starts; the others are spawned by
//! [`super::spawn`]. Each core can read its own identity from the memory-mapped
//! `ADSP_PID` register, which returns `index + 2`. Mirrors `up_cpu_index`
//! (`cxd56_cpuindex.c:58`: `return getreg32(CXD56_ADSP_PID) - 2;`).

use core::ptr;

/// APP_DSP processor-ID register, in each core's local address view.
/// Reads `index + 2`; the 0-based core index is therefore `value - 2`.
const ADSP_PID: *const u32 = 0x0e00_2040 as *const u32;

/// One of the six APP-domain Cortex-M4F cores.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Core {
    /// Main core — started by the boot ROM. Cannot be spawned.
    Core0 = 0,
    Core1 = 1,
    Core2 = 2,
    Core3 = 3,
    Core4 = 4,
    Core5 = 5,
}

impl Core {
    /// The main core (the one running at reset).
    pub const MAIN: Core = Core::Core0;

    /// Number of APP-domain cores.
    pub const COUNT: u8 = 6;

    /// 0-based core index (0..=5).
    #[inline]
    pub const fn index(self) -> u8 {
        self as u8
    }

    /// Raw ADSP master id (`index + 2`). This is the value held by `ADSP_PID`
    /// and recorded in the SPH `LOCK_OWNER`/`RESV_OWNER` status fields.
    #[inline]
    pub const fn raw_pid(self) -> u8 {
        self as u8 + 2
    }

    /// Convert a 0-based index into a [`Core`], or `None` if out of range.
    #[inline]
    pub fn from_index(index: u8) -> Option<Core> {
        if index < Self::COUNT {
            // SAFETY: range-checked; `Core` is `repr(u8)` with contiguous
            // discriminants 0..=5.
            Some(unsafe { core::mem::transmute::<u8, Core>(index) })
        } else {
            None
        }
    }
}

/// Read this core's raw ADSP processor id (`2..=7`).
#[inline]
pub fn raw_pid() -> u8 {
    (unsafe { ptr::read_volatile(ADSP_PID) } & 0x1f) as u8
}

/// This core's identity (`Core0`..`Core5`). `Core0` is the main core.
#[inline]
pub fn current() -> Core {
    Core::from_index(raw_pid().wrapping_sub(2)).unwrap_or(Core::Core0)
}
