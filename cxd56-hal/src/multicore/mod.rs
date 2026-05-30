//! Multicore support for the CXD5602 APP-domain Cortex-M4F cluster.
//!
//! Targets the **single combined-image** model: all cores run from one binary
//! with multiple entry points (embassy-rp style — no ELF loader / ASMP runtime
//! loader). The pieces:
//!
//! - [`cpu`] — each core's identity ([`Core`], [`current`]).
//! - [`spawn`] — bring up worker cores ([`spawn`], [`ack_boot`]).
//! - [`sph`] — hardware-semaphore cross-core locks ([`Sph`], [`HwMutex`]). The
//!   Cortex-M4 `LDREX`/`STREX` monitors do not work across cores, so the SPH
//!   block is the only sound cross-core mutual-exclusion primitive.
//! - [`mailbox`] — two-word inter-core messages over the CPU FIFO ([`Mailbox`]).
//!
//! # Scope: polling/spinning only
//!
//! Every primitive here is blocking/polling. Interrupt-driven paths (an
//! embassy-async [`Mailbox`] receive waker on `FIFO_FROM`, or SPH release
//! wakeups) are intentionally **not** wired up yet: the generated PAC interrupt
//! discriminants are the NuttX full-vector indices (e.g. `I2C0 = 33`), but
//! cortex-m-rt places `__INTERRUPTS[n]` at vector slot `16 + n`, so the
//! discriminant must be the 0-based NVIC number (`value - 16`). Until that
//! numbering is corrected in the PAC, `#[interrupt]` handlers land in the wrong
//! vector slot, so no interrupt-driven multicore path can be relied upon.

pub mod cpu;
pub mod mailbox;
pub mod spawn;
pub mod sph;

pub use cpu::{current, Core};
pub use mailbox::{Full, Mailbox};
pub use spawn::{ack_boot, spawn, SpawnError};
pub use sph::{HwMutex, HwMutexGuard, Sph};
