//! Hardware semaphores (SPH) — the cross-core lock primitive.
//!
//! The Cortex-M4 `LDREX`/`STREX` exclusive monitors are local to each core and
//! do **not** provide mutual exclusion *across* cores, so `core::sync::atomic`
//! compare-exchange cannot build a cross-core lock. The CXD5602 instead exposes
//! 16 hardware test-and-set semaphores at `0x4600_c800` (the [`pac::Sph`]
//! peripheral). Each is a 16-byte slot with a write-only `REQ` command register
//! and a read-only `STS` status register. Mirrors `cxd56_sph.c`.
//!
//! These are global hardware shared by every core, so the driver accesses them
//! through [`pac::Sph::PTR`] rather than owning a singleton handle.

use super::cpu;
use crate::pac;
use core::cell::UnsafeCell;
use core::ops::{Deref, DerefMut};

/// Number of hardware semaphores.
pub const COUNT: usize = 16;

// REQ command field (`REQ[1:0]`).
const REQ_UNLOCK: u32 = 0;
const REQ_LOCK: u32 = 1;
#[allow(dead_code)]
const REQ_RESERVE: u32 = 2;
#[allow(dead_code)]
const REQ_INTRCLR: u32 = 3;

/// A handle to one of the 16 hardware semaphores.
///
/// `Sph` is a lightweight `Copy` index — multiple cores may hold a handle to the
/// same semaphore; mutual exclusion is enforced by the hardware, not by Rust
/// ownership.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Sph(usize);

impl Sph {
    /// Bind to hardware semaphore `id` (`0..16`).
    ///
    /// # Panics
    /// Panics if `id >= 16`.
    pub const fn new(id: usize) -> Self {
        assert!(id < COUNT, "SPH id out of range");
        Sph(id)
    }

    #[inline]
    fn regs() -> &'static pac::sph::RegisterBlock {
        // SAFETY: SPH is a memory-mapped peripheral shared by all cores; we only
        // issue single-register reads/writes with no aliasing requirements.
        unsafe { &*pac::Sph::PTR }
    }

    /// Attempt to acquire the semaphore without spinning.
    ///
    /// Issues a `LOCK` request and checks whether this core won arbitration by
    /// comparing the recorded owner against this core's raw ADSP id. Returns
    /// `true` iff this core now holds the lock.
    #[inline]
    pub fn try_lock(self) -> bool {
        let sph = Self::regs();
        sph.req(self.0).write(|w| unsafe { w.bits(REQ_LOCK) });
        // The owner field records the ADSP master id (= core index + 2). If the
        // semaphore was free, our LOCK request set the owner to us; if it was
        // already held, the request is ignored and the owner is unchanged.
        sph.sts(self.0).read().lock_owner().bits() == cpu::raw_pid()
    }

    /// Spin until the semaphore is acquired.
    #[inline]
    pub fn lock(self) {
        while !self.try_lock() {
            core::hint::spin_loop();
        }
    }

    /// Release the semaphore. Only meaningful if this core currently holds it.
    #[inline]
    pub fn unlock(self) {
        Self::regs()
            .req(self.0)
            .write(|w| unsafe { w.bits(REQ_UNLOCK) });
    }

    /// The raw ADSP id of the core currently holding the lock, or `None` if the
    /// semaphore is idle.
    #[inline]
    pub fn owner(self) -> Option<u8> {
        let s = Self::regs().sts(self.0).read();
        if s.state().bits() == 0 {
            None
        } else {
            Some(s.lock_owner().bits())
        }
    }
}

/// A mutex guarding `T` with a hardware semaphore, safe to share across cores.
///
/// Place the `HwMutex<T>` (and the data it guards) in memory visible to every
/// participating core — i.e. a `static` in the shared combined image. The `T`
/// must be `Send` because the guarded value moves between cores.
pub struct HwMutex<T> {
    sph: Sph,
    data: UnsafeCell<T>,
}

// SAFETY: access to `data` is serialised by the hardware semaphore, which works
// across cores (unlike LDREX/STREX). `T: Send` because the value is handed
// between cores under the lock.
unsafe impl<T: Send> Sync for HwMutex<T> {}
unsafe impl<T: Send> Send for HwMutex<T> {}

impl<T> HwMutex<T> {
    /// Create a cross-core mutex guarded by hardware semaphore `sph`.
    ///
    /// All cores sharing the data must agree on the same `sph` id.
    pub const fn new(sph: Sph, data: T) -> Self {
        Self {
            sph,
            data: UnsafeCell::new(data),
        }
    }

    /// Spin until the lock is acquired, then return a guard.
    #[inline]
    pub fn lock(&self) -> HwMutexGuard<'_, T> {
        self.sph.lock();
        HwMutexGuard { mutex: self }
    }

    /// Try to acquire the lock without spinning.
    #[inline]
    pub fn try_lock(&self) -> Option<HwMutexGuard<'_, T>> {
        if self.sph.try_lock() {
            Some(HwMutexGuard { mutex: self })
        } else {
            None
        }
    }
}

/// RAII guard returned by [`HwMutex::lock`]. Releases the semaphore on drop.
pub struct HwMutexGuard<'a, T> {
    mutex: &'a HwMutex<T>,
}

impl<T> Deref for HwMutexGuard<'_, T> {
    type Target = T;
    #[inline]
    fn deref(&self) -> &T {
        // SAFETY: we hold the hardware lock for the lifetime of the guard.
        unsafe { &*self.mutex.data.get() }
    }
}

impl<T> DerefMut for HwMutexGuard<'_, T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut T {
        // SAFETY: we hold the hardware lock for the lifetime of the guard.
        unsafe { &mut *self.mutex.data.get() }
    }
}

impl<T> Drop for HwMutexGuard<'_, T> {
    #[inline]
    fn drop(&mut self) {
        self.mutex.sph.unlock();
    }
}
