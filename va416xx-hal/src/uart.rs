//! # API for the UART peripheral
//!
//! ## Examples
//!
//! - [UART simple example](https://egit.irs.uni-stuttgart.de/rust/va416xx-rs/src/branch/main/examples/simple/examples/uart.rs)
use core::marker::PhantomData;
use core::ops::Deref;

use embedded_hal_nb::serial::Read;
use fugit::RateExtU32;

use crate::clock::{Clocks, PeripheralSelect, SyscfgExt};
use crate::gpio::{AltFunc1, Pin, PD11, PD12, PE2, PE3, PF11, PF12, PF8, PF9, PG0, PG1};
use crate::time::Hertz;
use crate::{disable_interrupt, enable_interrupt};
use crate::{
    gpio::{AltFunc2, AltFunc3, PA2, PA3, PB14, PB15, PC14, PC15, PC4, PC5},
    pac::{self, uart0 as uart_base, Uart0, Uart1, Uart2},
};

//==================================================================================================
// Type-Level support
//==================================================================================================

pub trait TxRxPins<Uart> {}

impl TxRxPins<Uart0> for (Pin<PA2, AltFunc3>, Pin<PA3, AltFunc3>) {}
impl TxRxPins<Uart0> for (Pin<PC4, AltFunc2>, Pin<PC5, AltFunc2>) {}
impl TxRxPins<Uart0> for (Pin<PE2, AltFunc3>, Pin<PE3, AltFunc3>) {}
impl TxRxPins<Uart0> for (Pin<PG0, AltFunc1>, Pin<PG1, AltFunc1>) {}

impl TxRxPins<Uart1> for (Pin<PB14, AltFunc3>, Pin<PB15, AltFunc3>) {}
impl TxRxPins<Uart1> for (Pin<PD11, AltFunc3>, Pin<PD12, AltFunc3>) {}
impl TxRxPins<Uart1> for (Pin<PF11, AltFunc1>, Pin<PF12, AltFunc1>) {}

impl TxRxPins<Uart2> for (Pin<PC14, AltFunc2>, Pin<PC15, AltFunc2>) {}
impl TxRxPins<Uart2> for (Pin<PF8, AltFunc1>, Pin<PF9, AltFunc1>) {}

//==================================================================================================
// Regular Definitions
//==================================================================================================

#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    Overrun,
    FramingError,
    ParityError,
    BreakCondition,
    TransferPending,
    BufferTooShort,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Event {
    // Receiver FIFO interrupt enable. Generates interrupt
    // when FIFO is at least half full. Half full is defined as FIFO
    // count >= RXFIFOIRQTRG
    RxFifoHalfFull,
    // Framing error, Overrun error, Parity Error and Break error
    RxError,
    // Event for timeout condition: Data in the FIFO and no receiver
    // FIFO activity for 4 character times
    RxTimeout,

    // Transmitter FIFO interrupt enable. Generates interrupt
    // when FIFO is at least half full. Half full is defined as FIFO
    // count >= TXFIFOIRQTRG
    TxFifoHalfFull,
    // FIFO overflow error
    TxError,
    // Generate interrupt when transmit FIFO is empty and TXBUSY is 0
    TxEmpty,
    // Interrupt when CTSn changes value
    TxCts,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Parity {
    None,
    Odd,
    Even,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StopBits {
    One = 0,
    Two = 1,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WordSize {
    Five = 0,
    Six = 1,
    Seven = 2,
    Eight = 3,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Config {
    pub baudrate: Hertz,
    pub parity: Parity,
    pub stopbits: StopBits,
    // When false, use standard 16x baud clock, other 8x baud clock
    pub baud8: bool,
    pub wordsize: WordSize,
    pub enable_tx: bool,
    pub enable_rx: bool,
}

impl Config {
    pub fn baudrate(mut self, baudrate: Hertz) -> Self {
        self.baudrate = baudrate;
        self
    }

    pub fn parity_none(mut self) -> Self {
        self.parity = Parity::None;
        self
    }

    pub fn parity_even(mut self) -> Self {
        self.parity = Parity::Even;
        self
    }

    pub fn parity_odd(mut self) -> Self {
        self.parity = Parity::Odd;
        self
    }

    pub fn stopbits(mut self, stopbits: StopBits) -> Self {
        self.stopbits = stopbits;
        self
    }

    pub fn wordsize(mut self, wordsize: WordSize) -> Self {
        self.wordsize = wordsize;
        self
    }

    pub fn baud8(mut self, baud: bool) -> Self {
        self.baud8 = baud;
        self
    }
}

impl Default for Config {
    fn default() -> Config {
        Config {
            baudrate: 115200_u32.Hz(),
            parity: Parity::None,
            stopbits: StopBits::One,
            baud8: false,
            wordsize: WordSize::Eight,
            enable_tx: true,
            enable_rx: true,
        }
    }
}

impl From<Hertz> for Config {
    fn from(value: Hertz) -> Self {
        Config::default().baudrate(value)
    }
}

//==================================================================================================
// IRQ Definitions
//==================================================================================================

struct IrqInfo {
    rx_len: usize,
    rx_idx: usize,
    mode: IrqReceptionMode,
}

pub enum IrqResultMask {
    Complete = 0,
    Overflow = 1,
    FramingError = 2,
    ParityError = 3,
    Break = 4,
    Timeout = 5,
    Addr9 = 6,
    /// Should not happen
    Unknown = 7,
}

/// This struct is used to return the default IRQ handler result to the user
#[derive(Debug, Default)]
pub struct IrqResult {
    raw_res: u32,
    pub bytes_read: usize,
}

impl IrqResult {
    pub const fn new() -> Self {
        IrqResult {
            raw_res: 0,
            bytes_read: 0,
        }
    }
}

impl IrqResult {
    #[inline]
    pub fn raw_result(&self) -> u32 {
        self.raw_res
    }

    #[inline]
    pub(crate) fn clear_result(&mut self) {
        self.raw_res = 0;
    }
    #[inline]
    pub(crate) fn set_result(&mut self, flag: IrqResultMask) {
        self.raw_res |= 1 << flag as u32;
    }

    #[inline]
    pub fn complete(&self) -> bool {
        if ((self.raw_res >> IrqResultMask::Complete as u32) & 0x01) == 0x01 {
            return true;
        }
        false
    }

    #[inline]
    pub fn error(&self) -> bool {
        if self.overflow_error() || self.framing_error() || self.parity_error() {
            return true;
        }
        false
    }

    #[inline]
    pub fn overflow_error(&self) -> bool {
        if ((self.raw_res >> IrqResultMask::Overflow as u32) & 0x01) == 0x01 {
            return true;
        }
        false
    }

    #[inline]
    pub fn framing_error(&self) -> bool {
        if ((self.raw_res >> IrqResultMask::FramingError as u32) & 0x01) == 0x01 {
            return true;
        }
        false
    }

    #[inline]
    pub fn parity_error(&self) -> bool {
        if ((self.raw_res >> IrqResultMask::ParityError as u32) & 0x01) == 0x01 {
            return true;
        }
        false
    }

    #[inline]
    pub fn timeout(&self) -> bool {
        if ((self.raw_res >> IrqResultMask::Timeout as u32) & 0x01) == 0x01 {
            return true;
        }
        false
    }
}

#[derive(Debug, PartialEq)]
enum IrqReceptionMode {
    Idle,
    Pending,
}

//==================================================================================================
// UART implementation
//==================================================================================================

/// Type erased variant of a UART. Can be created with the [Uart::downgrade] function.
pub struct UartBase<Uart> {
    uart: Uart,
    tx: Tx<Uart>,
    rx: Rx<Uart>,
}
/// Serial abstraction. Entry point to create a new UART
pub struct Uart<UartInstance, Pins> {
    inner: UartBase<UartInstance>,
    pins: Pins,
}

/// UART using the IRQ capabilities of the peripheral. Can be created with the
/// [`Uart::into_uart_with_irq`] function. Currently, only the RX side for IRQ based reception
/// is implemented.
pub struct UartWithIrq<Uart, Pins> {
    base: UartWithIrqBase<Uart>,
    pins: Pins,
}

/// Type-erased UART using the IRQ capabilities of the peripheral. Can be created with the
/// [`UartWithIrq::downgrade`] function. Currently, only the RX side for IRQ based reception
/// is implemented.
pub struct UartWithIrqBase<UART> {
    pub inner: UartBase<UART>,
    irq_info: IrqInfo,
}

/// Serial receiver
pub struct Rx<Uart> {
    _usart: PhantomData<Uart>,
}

/// Serial transmitter
pub struct Tx<Uart> {
    _usart: PhantomData<Uart>,
}

impl<Uart> Rx<Uart> {
    fn new() -> Self {
        Self {
            _usart: PhantomData,
        }
    }
}

impl<Uart> Tx<Uart> {
    fn new() -> Self {
        Self {
            _usart: PhantomData,
        }
    }
}

pub trait Instance: Deref<Target = uart_base::RegisterBlock> {
    const IDX: u8;
    const PERIPH_SEL: PeripheralSelect;
    const IRQ_RX: pac::Interrupt;
    const IRQ_TX: pac::Interrupt;

    fn ptr() -> *const uart_base::RegisterBlock;
}

impl Instance for Uart0 {
    const IDX: u8 = 0;
    const PERIPH_SEL: PeripheralSelect = PeripheralSelect::Uart0;
    const IRQ_RX: pac::Interrupt = pac::Interrupt::UART0_RX;
    const IRQ_TX: pac::Interrupt = pac::Interrupt::UART0_TX;

    fn ptr() -> *const uart_base::RegisterBlock {
        Uart0::ptr() as *const _
    }
}

impl Instance for Uart1 {
    const IDX: u8 = 1;
    const PERIPH_SEL: PeripheralSelect = PeripheralSelect::Uart1;
    const IRQ_RX: pac::Interrupt = pac::Interrupt::UART1_RX;
    const IRQ_TX: pac::Interrupt = pac::Interrupt::UART1_TX;

    fn ptr() -> *const uart_base::RegisterBlock {
        Uart1::ptr() as *const _
    }
}

impl Instance for Uart2 {
    const IDX: u8 = 2;
    const PERIPH_SEL: PeripheralSelect = PeripheralSelect::Uart2;
    const IRQ_RX: pac::Interrupt = pac::Interrupt::UART2_RX;
    const IRQ_TX: pac::Interrupt = pac::Interrupt::UART2_TX;

    fn ptr() -> *const uart_base::RegisterBlock {
        Uart2::ptr() as *const _
    }
}

impl<Uart: Instance> UartBase<Uart> {
    fn init(self, config: Config, clocks: &Clocks) -> Self {
        if Uart::IDX == 2 {
            self.init_with_clock_freq(config, clocks.apb1())
        } else {
            self.init_with_clock_freq(config, clocks.apb2())
        }
    }

    /// This function assumes that the peripheral clock was alredy enabled
    /// in the SYSCONFIG register
    fn init_with_clock_freq(self, config: Config, apb_clk: Hertz) -> Self {
        let baud_multiplier = match config.baud8 {
            false => 16,
            true => 8,
        };
        // This is the calculation: (64.0 * (x - integer_part as f32) + 0.5) as u32 without floating
        // point calculations.
        let frac = ((apb_clk.raw() % (config.baudrate.raw() * 16)) * 64
            + (config.baudrate.raw() * 8))
            / (config.baudrate.raw() * 16);
        // Calculations here are derived from chapter 10.4.4 (p.74) of the datasheet.
        let x = apb_clk.raw() as f32 / (config.baudrate.raw() * baud_multiplier) as f32;
        let integer_part = x as u32;
        self.uart.clkscale().write(|w| unsafe {
            w.frac().bits(frac as u8);
            w.int().bits(integer_part)
        });

        let (paren, pareven) = match config.parity {
            Parity::None => (false, false),
            Parity::Odd => (true, false),
            Parity::Even => (true, true),
        };
        let stopbits = match config.stopbits {
            StopBits::One => false,
            StopBits::Two => true,
        };
        let wordsize = config.wordsize as u8;
        let baud8 = config.baud8;
        self.uart.ctrl().write(|w| {
            w.paren().bit(paren);
            w.pareven().bit(pareven);
            w.stopbits().bit(stopbits);
            w.baud8().bit(baud8);
            unsafe { w.wordsize().bits(wordsize) }
        });
        let (txenb, rxenb) = (config.enable_tx, config.enable_rx);
        // Clear the FIFO
        self.uart.fifo_clr().write(|w| {
            w.rxfifo().set_bit();
            w.txfifo().set_bit()
        });
        self.uart.enable().write(|w| {
            w.rxenable().bit(rxenb);
            w.txenable().bit(txenb)
        });
        self
    }

    #[inline]
    pub fn enable_rx(&mut self) {
        self.uart.enable().modify(|_, w| w.rxenable().set_bit());
    }

    #[inline]
    pub fn disable_rx(&mut self) {
        self.uart.enable().modify(|_, w| w.rxenable().clear_bit());
    }

    #[inline]
    pub fn enable_tx(&mut self) {
        self.uart.enable().modify(|_, w| w.txenable().set_bit());
    }

    #[inline]
    pub fn disable_tx(&mut self) {
        self.uart.enable().modify(|_, w| w.txenable().clear_bit());
    }

    #[inline]
    pub fn clear_rx_fifo(&mut self) {
        self.uart.fifo_clr().write(|w| w.rxfifo().set_bit());
    }

    #[inline]
    pub fn clear_tx_fifo(&mut self) {
        self.uart.fifo_clr().write(|w| w.txfifo().set_bit());
    }

    pub fn listen(&self, event: Event) {
        self.uart.irq_enb().modify(|_, w| match event {
            Event::RxError => w.irq_rx_status().set_bit(),
            Event::RxFifoHalfFull => w.irq_rx().set_bit(),
            Event::RxTimeout => w.irq_rx_to().set_bit(),
            Event::TxEmpty => w.irq_tx_empty().set_bit(),
            Event::TxError => w.irq_tx_status().set_bit(),
            Event::TxFifoHalfFull => w.irq_tx().set_bit(),
            Event::TxCts => w.irq_tx_cts().set_bit(),
        });
    }

    pub fn unlisten(&self, event: Event) {
        self.uart.irq_enb().modify(|_, w| match event {
            Event::RxError => w.irq_rx_status().clear_bit(),
            Event::RxFifoHalfFull => w.irq_rx().clear_bit(),
            Event::RxTimeout => w.irq_rx_to().clear_bit(),
            Event::TxEmpty => w.irq_tx_empty().clear_bit(),
            Event::TxError => w.irq_tx_status().clear_bit(),
            Event::TxFifoHalfFull => w.irq_tx().clear_bit(),
            Event::TxCts => w.irq_tx_cts().clear_bit(),
        });
    }

    pub fn release(self) -> Uart {
        // Clear the FIFO
        self.uart.fifo_clr().write(|w| {
            w.rxfifo().set_bit();
            w.txfifo().set_bit()
        });
        self.uart.enable().write(|w| {
            w.rxenable().clear_bit();
            w.txenable().clear_bit()
        });
        disable_interrupt(Uart::IRQ_RX);
        self.uart
    }

    pub fn split(self) -> (Tx<Uart>, Rx<Uart>) {
        (self.tx, self.rx)
    }
}

impl<UartInstance: Instance, Pins> Uart<UartInstance, Pins> {
    pub fn new(
        uart: UartInstance,
        pins: Pins,
        config: impl Into<Config>,
        syscfg: &mut va416xx::Sysconfig,
        clocks: &Clocks,
    ) -> Self {
        crate::clock::enable_peripheral_clock(syscfg, UartInstance::PERIPH_SEL);
        // This is done in the C HAL.
        syscfg.assert_periph_reset_for_two_cycles(UartInstance::PERIPH_SEL);
        Uart {
            inner: UartBase {
                uart,
                tx: Tx::new(),
                rx: Rx::new(),
            },
            pins,
        }
        .init(config.into(), clocks)
    }

    pub fn new_with_clock_freq(
        uart: UartInstance,
        pins: Pins,
        config: impl Into<Config>,
        syscfg: &mut va416xx::Sysconfig,
        clock: impl Into<Hertz>,
    ) -> Self {
        crate::clock::enable_peripheral_clock(syscfg, UartInstance::PERIPH_SEL);
        Uart {
            inner: UartBase {
                uart,
                tx: Tx::new(),
                rx: Rx::new(),
            },
            pins,
        }
        .init_with_clock_freq(config.into(), clock.into())
    }

    fn init(mut self, config: Config, clocks: &Clocks) -> Self {
        self.inner = self.inner.init(config, clocks);
        self
    }

    /// This function assumes that the peripheral clock was already enabled
    /// in the SYSCONFIG register
    #[allow(dead_code)]
    fn init_with_clock_freq(mut self, config: Config, sys_clk: Hertz) -> Self {
        self.inner = self.inner.init_with_clock_freq(config, sys_clk);
        self
    }

    /// If the IRQ capabilities of the peripheral are used, the UART needs to be converted
    /// with this function
    pub fn into_uart_with_irq(self) -> UartWithIrq<UartInstance, Pins> {
        let (inner, pins) = self.downgrade_internal();
        UartWithIrq {
            pins,
            base: UartWithIrqBase {
                inner,
                irq_info: IrqInfo {
                    rx_len: 0,
                    rx_idx: 0,
                    mode: IrqReceptionMode::Idle,
                },
            },
        }
    }

    delegate::delegate! {
        to self.inner {
            #[inline]
            pub fn enable_rx(&mut self);
            #[inline]
            pub fn disable_rx(&mut self);

            #[inline]
            pub fn enable_tx(&mut self);
            #[inline]
            pub fn disable_tx(&mut self);

            #[inline]
            pub fn clear_rx_fifo(&mut self);
            #[inline]
            pub fn clear_tx_fifo(&mut self);

            #[inline]
            pub fn listen(&self, event: Event);
            #[inline]
            pub fn unlisten(&self, event: Event);
            #[inline]
            pub fn split(self) -> (Tx<UartInstance>, Rx<UartInstance>);
        }
    }

    fn downgrade_internal(self) -> (UartBase<UartInstance>, Pins) {
        let base = UartBase {
            uart: self.inner.uart,
            tx: self.inner.tx,
            rx: self.inner.rx,
        };
        (base, self.pins)
    }

    pub fn downgrade(self) -> UartBase<UartInstance> {
        UartBase {
            uart: self.inner.uart,
            tx: self.inner.tx,
            rx: self.inner.rx,
        }
    }

    pub fn release(self) -> (UartInstance, Pins) {
        (self.inner.release(), self.pins)
    }
}

#[derive(Default, Debug)]
pub struct IrqUartError {
    overflow: bool,
    framing: bool,
    parity: bool,
}

impl IrqUartError {
    pub fn error(&self) -> bool {
        self.overflow || self.framing || self.parity
    }
}

#[derive(Debug)]
pub enum IrqError {
    BufferTooShort { found: usize, expected: usize },
    Uart(IrqUartError),
}

impl<Uart: Instance> UartWithIrqBase<Uart> {
    /// This initializes a non-blocking read transfer using the IRQ capabilities of the UART
    /// peripheral.
    ///
    /// The only required information is the maximum length for variable sized reception
    /// or the expected length for fixed length reception. If variable sized packets are expected,
    /// the timeout functionality of the IRQ should be enabled as well. After calling this function,
    /// the [`irq_handler`](Self::irq_handler) function should be called in the user interrupt
    /// handler to read the received packets and reinitiate another transfer if desired.
    pub fn read_fixed_len_using_irq(
        &mut self,
        max_len: usize,
        enb_timeout_irq: bool,
    ) -> Result<(), Error> {
        if self.irq_info.mode != IrqReceptionMode::Idle {
            return Err(Error::TransferPending);
        }
        self.irq_info.mode = IrqReceptionMode::Pending;
        self.irq_info.rx_idx = 0;
        self.irq_info.rx_len = max_len;
        self.inner.enable_rx();
        self.inner.enable_tx();
        self.enable_rx_irq_sources(enb_timeout_irq);
        unsafe { enable_interrupt(Uart::IRQ_RX) };
        Ok(())
    }

    #[inline]
    fn enable_rx_irq_sources(&mut self, timeout: bool) {
        self.inner.uart.irq_enb().modify(|_, w| {
            if timeout {
                w.irq_rx_to().set_bit();
            }
            w.irq_rx_status().set_bit();
            w.irq_rx().set_bit()
        });
    }

    #[inline]
    fn disable_rx_irq_sources(&mut self) {
        self.inner.uart.irq_enb().modify(|_, w| {
            w.irq_rx_to().clear_bit();
            w.irq_rx_status().clear_bit();
            w.irq_rx().clear_bit()
        });
    }

    #[inline]
    pub fn enable_tx(&mut self) {
        self.inner.enable_tx()
    }

    #[inline]
    pub fn disable_tx(&mut self) {
        self.inner.disable_tx()
    }

    pub fn cancel_transfer(&mut self) {
        self.disable_rx_irq_sources();
        self.inner.clear_tx_fifo();
        self.irq_info.rx_idx = 0;
        self.irq_info.rx_len = 0;
    }

    /// Default IRQ handler which can be used to read the packets arriving on the UART peripheral.
    ///
    /// If passed buffer is equal to or larger than the specified maximum length, an
    /// [`Error::BufferTooShort`] will be returned
    pub fn irq_handler(&mut self, buf: &mut [u8]) -> Result<IrqResult, IrqError> {
        if buf.len() < self.irq_info.rx_len {
            return Err(IrqError::BufferTooShort {
                found: buf.len(),
                expected: self.irq_info.rx_len,
            });
        }
        let mut res = IrqResult::default();
        let mut possible_error = IrqUartError::default();

        let rx_status = self.inner.uart.rxstatus().read();
        res.raw_res = rx_status.bits();
        let irq_end = self.inner.uart.irq_end().read();
        let enb_status = self.inner.uart.enable().read();
        let rx_enabled = enb_status.rxenable().bit_is_set();
        let _tx_enabled = enb_status.txenable().bit_is_set();
        let read_handler = |res: &mut IrqResult,
                            possible_error: &mut IrqUartError,
                            read_res: nb::Result<u8, Error>|
         -> Option<u8> {
            match read_res {
                Ok(byte) => Some(byte),
                Err(nb::Error::WouldBlock) => None,
                Err(nb::Error::Other(e)) => {
                    match e {
                        Error::Overrun => {
                            possible_error.overflow = true;
                        }
                        Error::FramingError => {
                            possible_error.framing = true;
                        }
                        Error::ParityError => {
                            possible_error.parity = true;
                        }
                        _ => {
                            res.set_result(IrqResultMask::Unknown);
                        }
                    }
                    None
                }
            }
        };
        if irq_end.irq_rx().bit_is_set() {
            // If this interrupt bit is set, the trigger level is available at the very least.
            // Read everything as fast as possible
            for _ in 0..core::cmp::min(
                self.inner.uart.rxfifoirqtrg().read().bits() as usize,
                self.irq_info.rx_len,
            ) {
                buf[self.irq_info.rx_idx] = (self.inner.uart.data().read().bits() & 0xff) as u8;
                self.irq_info.rx_idx += 1;
            }

            // While there is data in the FIFO, write it into the reception buffer
            loop {
                if self.irq_info.rx_idx == self.irq_info.rx_len {
                    self.irq_completion_handler(&mut res);
                    return Ok(res);
                }
                if let Some(byte) = read_handler(&mut res, &mut possible_error, self.inner.read()) {
                    buf[self.irq_info.rx_idx] = byte;
                    self.irq_info.rx_idx += 1;
                } else {
                    break;
                }
            }
        }

        // RX transfer not complete, check for RX errors
        if (self.irq_info.rx_idx < self.irq_info.rx_len) && rx_enabled {
            // Read status register again, might have changed since reading received data
            let rx_status = self.inner.uart.rxstatus().read();
            res.raw_res = rx_status.bits();
            if rx_status.rxovr().bit_is_set() {
                possible_error.overflow = true;
            }
            if rx_status.rxfrm().bit_is_set() {
                possible_error.framing = true;
            }
            if rx_status.rxpar().bit_is_set() {
                possible_error.parity = true;
            }
            if rx_status.rxto().bit_is_set() {
                // A timeout has occured but there might be some leftover data in the FIFO,
                // so read that data as well
                while let Some(byte) =
                    read_handler(&mut res, &mut possible_error, self.inner.read())
                {
                    buf[self.irq_info.rx_idx] = byte;
                    self.irq_info.rx_idx += 1;
                }
                self.irq_completion_handler(&mut res);
                res.set_result(IrqResultMask::Timeout);
                return Ok(res);
            }

            // If it is not a timeout, it's an error
            if possible_error.error() {
                self.disable_rx_irq_sources();
                return Err(IrqError::Uart(possible_error));
            }
        }

        // Clear the interrupt status bits
        self.inner
            .uart
            .irq_clr()
            .write(|w| unsafe { w.bits(irq_end.bits()) });
        Ok(res)
    }

    fn irq_completion_handler(&mut self, res: &mut IrqResult) {
        self.disable_rx_irq_sources();
        self.inner.disable_rx();
        res.bytes_read = self.irq_info.rx_idx;
        res.clear_result();
        res.set_result(IrqResultMask::Complete);
        self.irq_info.mode = IrqReceptionMode::Idle;
        self.irq_info.rx_idx = 0;
        self.irq_info.rx_len = 0;
    }

    pub fn release(self) -> Uart {
        self.inner.release()
    }
}

impl<Uart: Instance, Pins> UartWithIrq<Uart, Pins> {
    /// See [`UartWithIrqBase::read_fixed_len_using_irq`] doc
    pub fn read_fixed_len_using_irq(
        &mut self,
        max_len: usize,
        enb_timeout_irq: bool,
    ) -> Result<(), Error> {
        self.base.read_fixed_len_using_irq(max_len, enb_timeout_irq)
    }

    pub fn cancel_transfer(&mut self) {
        self.base.cancel_transfer()
    }

    /// See [`UartWithIrqBase::irq_handler`] doc
    pub fn irq_handler(&mut self, buf: &mut [u8]) -> Result<IrqResult, IrqError> {
        self.base.irq_handler(buf)
    }

    pub fn release(self) -> (Uart, Pins) {
        (self.base.release(), self.pins)
    }

    pub fn downgrade(self) -> (UartWithIrqBase<Uart>, Pins) {
        (self.base, self.pins)
    }
}

impl embedded_io::Error for Error {
    fn kind(&self) -> embedded_io::ErrorKind {
        embedded_io::ErrorKind::Other
    }
}

impl embedded_hal_nb::serial::Error for Error {
    fn kind(&self) -> embedded_hal_nb::serial::ErrorKind {
        embedded_hal_nb::serial::ErrorKind::Other
    }
}

impl<Uart> embedded_io::ErrorType for Rx<Uart> {
    type Error = Error;
}

impl<Uart> embedded_hal_nb::serial::ErrorType for Rx<Uart> {
    type Error = Error;
}

impl<Uart: Instance> embedded_hal_nb::serial::Read<u8> for Rx<Uart> {
    fn read(&mut self) -> nb::Result<u8, Self::Error> {
        let uart = unsafe { &(*Uart::ptr()) };
        let status_reader = uart.rxstatus().read();
        let err = if status_reader.rxovr().bit_is_set() {
            Some(Error::Overrun)
        } else if status_reader.rxfrm().bit_is_set() {
            Some(Error::FramingError)
        } else if status_reader.rxpar().bit_is_set() {
            Some(Error::ParityError)
        } else {
            None
        };
        if let Some(err) = err {
            // The status code is always related to the next bit for the framing
            // and parity status bits. We have to read the DATA register
            // so that the next status reflects the next DATA word
            // For overrun error, we read as well to clear the peripheral
            uart.data().read().bits();
            Err(err.into())
        } else if status_reader.rdavl().bit_is_set() {
            let data = uart.data().read().bits();
            Ok((data & 0xff) as u8)
        } else {
            Err(nb::Error::WouldBlock)
        }
    }
}

impl<Uart: Instance> embedded_io::Read for Rx<Uart> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        if buf.is_empty() {
            return Ok(0);
        }

        for byte in buf.iter_mut() {
            let w = nb::block!(<Self as embedded_hal_nb::serial::Read<u8>>::read(self))?;
            *byte = w;
        }

        Ok(buf.len())
    }
}

impl<Uart> embedded_io::ErrorType for Tx<Uart> {
    type Error = Error;
}

impl<Uart> embedded_hal_nb::serial::ErrorType for Tx<Uart> {
    type Error = Error;
}

impl<Uart: Instance> embedded_hal_nb::serial::Write<u8> for Tx<Uart> {
    fn write(&mut self, word: u8) -> nb::Result<(), Self::Error> {
        let reader = unsafe { &(*Uart::ptr()) }.txstatus().read();
        if reader.wrrdy().bit_is_clear() {
            return Err(nb::Error::WouldBlock);
        } else {
            // DPARITY bit not supported yet
            unsafe {
                // NOTE(unsafe) atomic write to data register
                // NOTE(write_volatile) 8-bit write that's not
                // possible through the svd2rust API
                (*Uart::ptr()).data().write(|w| w.bits(word as u32));
            }
        }
        Ok(())
    }

    fn flush(&mut self) -> nb::Result<(), Self::Error> {
        // SAFETY: Only TX related registers are used.
        let reader = unsafe { &(*Uart::ptr()) }.txstatus().read();
        if reader.wrbusy().bit_is_set() {
            return Err(nb::Error::WouldBlock);
        }
        Ok(())
    }
}

impl<Uart: Instance> embedded_io::Write for Tx<Uart> {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        if buf.is_empty() {
            return Ok(0);
        }

        for byte in buf.iter() {
            nb::block!(<Self as embedded_hal_nb::serial::Write<u8>>::write(
                self, *byte
            ))?;
        }

        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        nb::block!(<Self as embedded_hal_nb::serial::Write<u8>>::flush(self))
    }
}

impl<UartInstance> embedded_io::ErrorType for UartBase<UartInstance> {
    type Error = Error;
}

impl<UartInstance> embedded_hal_nb::serial::ErrorType for UartBase<UartInstance> {
    type Error = Error;
}

impl<Uart: Instance> embedded_hal_nb::serial::Read<u8> for UartBase<Uart> {
    fn read(&mut self) -> nb::Result<u8, Self::Error> {
        self.rx.read()
    }
}

impl<Uart: Instance> embedded_hal_nb::serial::Write<u8> for UartBase<Uart> {
    fn write(&mut self, word: u8) -> nb::Result<(), Self::Error> {
        self.tx.write(word)
    }

    fn flush(&mut self) -> nb::Result<(), Self::Error> {
        self.tx.flush()
    }
}
