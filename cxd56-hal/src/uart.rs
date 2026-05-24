use crate::clocks::{ClockError, Clocks, PeripheralId};
use crate::pac;
use arm_pl011_uart::{DataBits, LineConfig, PL011Registers, Uart as Pl011Uart};
use core::{fmt, ptr::NonNull};
use embedded_hal_nb::nb;
use safe_mmio::UniqueMmioPointer;

pub use arm_pl011_uart::Error as Pl011Error;

#[derive(Debug)]
pub enum UartError {
    Clock(ClockError),
    Init(Pl011Error),
}

impl From<ClockError> for UartError {
    fn from(e: ClockError) -> Self {
        Self::Clock(e)
    }
}

pub enum WordLength {
    Five,
    Six,
    Seven,
    Eight,
}

pub enum StopBits {
    One,
    Two,
}

pub enum Parity {
    None,
    Even,
    Odd,
}

pub struct UartConfig {
    pub baud_rate: u32,
    pub word_length: WordLength,
    pub stop_bits: StopBits,
    pub parity: Parity,
}

impl Default for UartConfig {
    fn default() -> Self {
        Self {
            baud_rate: 115_200,
            word_length: WordLength::Eight,
            stop_bits: StopBits::One,
            parity: Parity::None,
        }
    }
}

/// Blocking driver for UART2 (the IMG-domain UART connected to the on-board
/// CP2102N USB-to-serial bridge on the Spresense main board).
pub struct Uart2 {
    inner: Pl011Uart<'static>,
    // Kept to enforce singleton ownership — pac::Uart2 cannot be obtained twice.
    _pac: pac::Uart2,
}

impl Uart2 {
    /// Initialise UART2 with the given baud rate and frame format.
    ///
    /// Enables the IMG-UART clock (≈ 39 MHz) and programs the PL011
    /// control registers.  Must be called after `Clocks::freeze`.
    pub fn new(
        uart: pac::Uart2,
        clocks: &Clocks,
        config: UartConfig,
    ) -> Result<Self, UartError> {
        PeripheralId::ImgUart.enable()?;

        let f_uart = clocks.img_uart.to_Hz();

        let line_config = LineConfig {
            data_bits: match config.word_length {
                WordLength::Eight => DataBits::Bits8,
                WordLength::Seven => DataBits::Bits7,
                WordLength::Six => DataBits::Bits6,
                WordLength::Five => DataBits::Bits5,
            },
            parity: match config.parity {
                Parity::None => arm_pl011_uart::Parity::None,
                Parity::Even => arm_pl011_uart::Parity::Even,
                Parity::Odd => arm_pl011_uart::Parity::Odd,
            },
            stop_bits: match config.stop_bits {
                StopBits::One => arm_pl011_uart::StopBits::One,
                StopBits::Two => arm_pl011_uart::StopBits::Two,
            },
        };

        // SAFETY: pac::Uart2 is the singleton ownership token for this register
        // block; we consume it (_pac field) to prevent duplicate construction.
        // pac::Uart2::ptr() is a const fn that returns the chip's fixed UART2
        // base address (0x0210_3000). The CXD5602 UART register layout is
        // PL011-compatible (confirmed by matching offsets and bitfield names in
        // the svd2rust PAC).
        let base: *mut PL011Registers = pac::Uart2::ptr().cast_mut().cast();
        let mmio = unsafe { UniqueMmioPointer::new(NonNull::new_unchecked(base)) };

        let mut inner = Pl011Uart::new(mmio);
        inner
            .enable(line_config, config.baud_rate, f_uart)
            .map_err(UartError::Init)?;

        Ok(Self { inner, _pac: uart })
    }

    /// Transmit one byte, blocking until the TX FIFO has room.
    #[inline]
    pub fn write_byte(&mut self, byte: u8) {
        while self.inner.is_tx_fifo_full() {}
        self.inner.write_word(byte);
    }

    /// Read one byte if the RX FIFO is non-empty, otherwise return `None`.
    /// Silently discards receive errors; use the `embedded-hal-nb` or
    /// `embedded-io` traits if error reporting is needed.
    #[inline]
    pub fn read_byte(&mut self) -> Option<u8> {
        self.inner.read_word().ok().flatten()
    }

    /// Block until the TX FIFO and shift register are empty.
    #[inline]
    pub fn flush(&mut self) {
        while self.inner.is_busy() {}
    }
}

impl fmt::Write for Uart2 {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.inner.write_str(s)
    }
}

// embedded-hal-nb trait impls — forwarded to arm_pl011_uart::Uart.

impl embedded_hal_nb::serial::ErrorType for Uart2 {
    type Error = Pl011Error;
}

impl embedded_hal_nb::serial::Read<u8> for Uart2 {
    fn read(&mut self) -> nb::Result<u8, Self::Error> {
        embedded_hal_nb::serial::Read::read(&mut self.inner)
    }
}

impl embedded_hal_nb::serial::Write<u8> for Uart2 {
    fn write(&mut self, word: u8) -> nb::Result<(), Self::Error> {
        embedded_hal_nb::serial::Write::write(&mut self.inner, word)
    }

    fn flush(&mut self) -> nb::Result<(), Self::Error> {
        embedded_hal_nb::serial::Write::flush(&mut self.inner)
    }
}

// embedded-io trait impls — forwarded to arm_pl011_uart::Uart.

impl embedded_io::ErrorType for Uart2 {
    type Error = Pl011Error;
}

impl embedded_io::Read for Uart2 {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        embedded_io::Read::read(&mut self.inner, buf)
    }
}

impl embedded_io::Write for Uart2 {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        embedded_io::Write::write(&mut self.inner, buf)
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        embedded_io::Write::flush(&mut self.inner)
    }
}

impl embedded_io::ReadReady for Uart2 {
    fn read_ready(&mut self) -> Result<bool, Self::Error> {
        embedded_io::ReadReady::read_ready(&mut self.inner)
    }
}

impl embedded_io::WriteReady for Uart2 {
    fn write_ready(&mut self) -> Result<bool, Self::Error> {
        embedded_io::WriteReady::write_ready(&mut self.inner)
    }
}
