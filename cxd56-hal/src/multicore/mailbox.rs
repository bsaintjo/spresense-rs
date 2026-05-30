//! Inter-core mailbox over the CPU FIFO.
//!
//! The CXD5602 CPU FIFO ([`pac::CpuFifo`] at `0x4600_c400`) is a hardware
//! mailbox that carries two-word (`[u32; 2]`) messages between cores. Word 0
//! carries a 4-bit routing/identity nibble in bits `[31:28]`: on send it is the
//! destination core's raw id, and on receive the hardware replaces it with the
//! sender's raw id. Mirrors `cxd56_cpufifo.c` (`cxd56_cfpush` / `cxd56_cfpull`).
//!
//! This driver is blocking/polling only. An interrupt-driven (embassy-async)
//! receive path needs the `FIFO_FROM` IRQ, which is currently blocked on the
//! PAC interrupt-numbering fix — see [`crate::multicore`].
//!
//! The FIFO is global hardware shared by every core, so this is a zero-sized
//! handle that reaches the peripheral through [`pac::CpuFifo::PTR`].

use super::cpu::Core;
use crate::pac;

/// Returned by [`Mailbox::try_send`] when the transmit FIFO is full.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Full;

/// Zero-sized handle to the shared CPU FIFO mailbox.
#[derive(Copy, Clone, Debug)]
pub struct Mailbox;

impl Mailbox {
    #[inline]
    fn regs() -> &'static pac::cpu_fifo::RegisterBlock {
        // SAFETY: the CPU FIFO is a memory-mapped peripheral shared by all
        // cores; we only issue single-register reads/writes.
        unsafe { &*pac::CpuFifo::PTR }
    }

    /// Pack a destination core id into word 0's routing nibble (`[31:28]`)
    /// together with a 28-bit payload (`[27:0]`).
    #[inline]
    pub const fn pack_word0(dest: Core, payload28: u32) -> u32 {
        ((dest.raw_pid() as u32) << 28) | (payload28 & 0x0fff_ffff)
    }

    /// Extract the sender's raw ADSP id from a received word 0.
    #[inline]
    pub const fn sender_raw_pid(word0: u32) -> u8 {
        ((word0 >> 28) & 0xf) as u8
    }

    /// `true` if the transmit FIFO is full (a [`Mailbox::try_send`] would fail).
    #[inline]
    pub fn is_tx_full() -> bool {
        Self::regs().fif_push_full().read().bits() != 0
    }

    /// `true` if a message is waiting in the receive FIFO.
    #[inline]
    pub fn is_rx_ready() -> bool {
        Self::regs().fif_pull_emp().read().bits() == 0
    }

    /// Try to enqueue a two-word message without blocking.
    ///
    /// The destination core must be encoded in `words[0]` bits `[31:28]` — see
    /// [`Mailbox::pack_word0`]. Returns [`Full`] if the transmit FIFO is full.
    #[inline]
    pub fn try_send(words: [u32; 2]) -> Result<(), Full> {
        let f = Self::regs();
        if f.fif_push_full().read().bits() != 0 {
            return Err(Full);
        }
        f.fif_push_wrd0().write(|w| unsafe { w.bits(words[0]) });
        f.fif_push_wrd1().write(|w| unsafe { w.bits(words[1]) });
        f.fif_push_cmp().write(|w| w.push_cmp().complete());
        Ok(())
    }

    /// Try to dequeue a two-word message without blocking.
    ///
    /// On success, `words[0]` bits `[31:28]` hold the sender's raw id — see
    /// [`Mailbox::sender_raw_pid`].
    #[inline]
    pub fn try_recv() -> Option<[u32; 2]> {
        let f = Self::regs();
        if f.fif_pull_emp().read().bits() != 0 {
            return None;
        }
        let w0 = f.fif_pull_wrd0().read().bits();
        let w1 = f.fif_pull_wrd1().read().bits();
        f.fif_pull_cmp().write(|w| w.pull_cmp().complete());
        Some([w0, w1])
    }

    /// Spin until the message can be enqueued.
    #[inline]
    pub fn send_blocking(words: [u32; 2]) {
        while Self::try_send(words).is_err() {
            core::hint::spin_loop();
        }
    }

    /// Spin until a message is received.
    #[inline]
    pub fn recv_blocking() -> [u32; 2] {
        loop {
            if let Some(m) = Self::try_recv() {
                return m;
            }
            core::hint::spin_loop();
        }
    }
}
