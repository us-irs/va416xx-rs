//! # API for the UART peripheral
//!
//! The core of this API are the [Uart], [UartBase], [Rx] and [Tx] structures.
//! The RX structure also has a dedicated [RxWithIrq] variant which allows reading the receiver
//! using interrupts.
//!
//! ## Examples
//!
//! - [UART simple example](https://egit.irs.uni-stuttgart.de/rust/va416xx-rs/src/branch/main/examples/simple/examples/uart.rs)
//! - [UART echo with IRQ and Embassy](https://egit.irs.uni-stuttgart.de/rust/va416xx-rs/src/branch/main/examples/embassy/src/bin/uart-echo-with-irq.rs)
//! - [Flashloader app using UART with IRQs](https://egit.irs.uni-stuttgart.de/rust/va416xx-rs/src/branch/main/flashloader)
use core::convert::Infallible;
use core::ops::Deref;

use embedded_hal_nb::serial::Read;
use fugit::RateExtU32;

use crate::clock::{Clocks, PeripheralSelect, SyscfgExt};
use crate::gpio::PF13;
use crate::time::Hertz;
use crate::{disable_interrupt, enable_interrupt};
use crate::{
    gpio::{
        AltFunc1, AltFunc2, AltFunc3, Pin, PA2, PA3, PB14, PB15, PC14, PC4, PC5, PD11, PD12, PE2,
        PE3, PF12, PF9, PG0, PG1,
    },
    pac::{self, uart0 as uart_base, Uart0, Uart1, Uart2},
};

#[cfg(not(feature = "va41628"))]
use crate::gpio::{PC15, PF8};

//==================================================================================================
// Type-Level support
//==================================================================================================

pub trait RxPin<Uart> {}
pub trait TxPin<Uart> {}

impl TxPin<Uart0> for Pin<PA2, AltFunc3> {}
impl RxPin<Uart0> for Pin<PA3, AltFunc3> {}

impl TxPin<Uart0> for Pin<PC4, AltFunc2> {}
impl RxPin<Uart0> for Pin<PC5, AltFunc2> {}

impl TxPin<Uart0> for Pin<PE2, AltFunc3> {}
impl RxPin<Uart0> for Pin<PE3, AltFunc3> {}

impl TxPin<Uart0> for Pin<PG0, AltFunc1> {}
impl RxPin<Uart0> for Pin<PG1, AltFunc1> {}

impl TxPin<Uart1> for Pin<PB14, AltFunc3> {}
impl RxPin<Uart1> for Pin<PB15, AltFunc3> {}

impl TxPin<Uart1> for Pin<PD11, AltFunc3> {}
impl RxPin<Uart1> for Pin<PD12, AltFunc3> {}

impl TxPin<Uart1> for Pin<PF12, AltFunc1> {}
impl RxPin<Uart1> for Pin<PF13, AltFunc1> {}

impl TxPin<Uart2> for Pin<PC14, AltFunc2> {}
#[cfg(not(feature = "va41628"))]
impl RxPin<Uart2> for Pin<PC15, AltFunc2> {}

#[cfg(not(feature = "va41628"))]
impl TxPin<Uart2> for Pin<PF8, AltFunc1> {}
impl RxPin<Uart2> for Pin<PF9, AltFunc1> {}

//==================================================================================================
// Regular Definitions
//==================================================================================================

#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct TransferPendingError;

#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RxError {
    Overrun,
    Framing,
    Parity,
}
#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    Rx(RxError),
    BreakCondition,
}

impl From<RxError> for Error {
    fn from(value: RxError) -> Self {
        Self::Rx(value)
    }
}

impl embedded_io::Error for Error {
    fn kind(&self) -> embedded_io::ErrorKind {
        embedded_io::ErrorKind::Other
    }
}

impl embedded_io::Error for RxError {
    fn kind(&self) -> embedded_io::ErrorKind {
        embedded_io::ErrorKind::Other
    }
}
impl embedded_hal_nb::serial::Error for RxError {
    fn kind(&self) -> embedded_hal_nb::serial::ErrorKind {
        match self {
            RxError::Overrun => embedded_hal_nb::serial::ErrorKind::Overrun,
            RxError::Framing => embedded_hal_nb::serial::ErrorKind::FrameFormat,
            RxError::Parity => embedded_hal_nb::serial::ErrorKind::Parity,
        }
    }
}

impl embedded_hal_nb::serial::Error for Error {
    fn kind(&self) -> embedded_hal_nb::serial::ErrorKind {
        match self {
            Error::Rx(rx_error) => embedded_hal_nb::serial::Error::kind(rx_error),
            Error::BreakCondition => embedded_hal_nb::serial::ErrorKind::Other,
        }
    }
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

#[derive(Debug, Copy, Clone)]
pub struct IrqContextTimeoutOrMaxSize {
    rx_idx: usize,
    mode: IrqReceptionMode,
    pub max_len: usize,
}

impl IrqContextTimeoutOrMaxSize {
    pub fn new(max_len: usize) -> Self {
        IrqContextTimeoutOrMaxSize {
            rx_idx: 0,
            max_len,
            mode: IrqReceptionMode::Idle,
        }
    }
}

impl IrqContextTimeoutOrMaxSize {
    pub fn reset(&mut self) {
        self.rx_idx = 0;
        self.mode = IrqReceptionMode::Idle;
    }
}

/// This struct is used to return the default IRQ handler result to the user
#[derive(Debug, Default)]
pub struct IrqResult {
    pub bytes_read: usize,
    pub errors: Option<IrqUartError>,
}

/// This struct is used to return the default IRQ handler result to the user
#[derive(Debug, Default)]
pub struct IrqResultMaxSizeOrTimeout {
    complete: bool,
    timeout: bool,
    pub errors: Option<IrqUartError>,
    pub bytes_read: usize,
}

impl IrqResultMaxSizeOrTimeout {
    pub fn new() -> Self {
        IrqResultMaxSizeOrTimeout {
            complete: false,
            timeout: false,
            errors: None,
            bytes_read: 0,
        }
    }
}
impl IrqResultMaxSizeOrTimeout {
    #[inline]
    pub fn has_errors(&self) -> bool {
        self.errors.is_some()
    }

    #[inline]
    pub fn overflow_error(&self) -> bool {
        self.errors.map_or(false, |e| e.overflow)
    }

    #[inline]
    pub fn framing_error(&self) -> bool {
        self.errors.map_or(false, |e| e.framing)
    }

    #[inline]
    pub fn parity_error(&self) -> bool {
        self.errors.map_or(false, |e| e.parity)
    }

    #[inline]
    pub fn timeout(&self) -> bool {
        self.timeout
    }

    #[inline]
    pub fn complete(&self) -> bool {
        self.complete
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum IrqReceptionMode {
    Idle,
    Pending,
}

#[derive(Default, Debug, Copy, Clone)]
pub struct IrqUartError {
    overflow: bool,
    framing: bool,
    parity: bool,
    other: bool,
}

impl IrqUartError {
    #[inline(always)]
    pub fn overflow(&self) -> bool {
        self.overflow
    }

    #[inline(always)]
    pub fn framing(&self) -> bool {
        self.framing
    }

    #[inline(always)]
    pub fn parity(&self) -> bool {
        self.parity
    }

    #[inline(always)]
    pub fn other(&self) -> bool {
        self.other
    }
}

impl IrqUartError {
    #[inline(always)]
    pub fn error(&self) -> bool {
        self.overflow || self.framing || self.parity
    }
}

#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct BufferTooShortError {
    found: usize,
    expected: usize,
}

//==================================================================================================
// UART peripheral wrapper
//==================================================================================================

pub trait Instance: Deref<Target = uart_base::RegisterBlock> {
    const IDX: u8;
    const PERIPH_SEL: PeripheralSelect;
    const IRQ_RX: pac::Interrupt;
    const IRQ_TX: pac::Interrupt;

    /// Retrieve the peripheral structure.
    ///
    /// # Safety
    ///
    /// This circumvents the safety guarantees of the HAL.
    unsafe fn steal() -> Self;
    fn ptr() -> *const uart_base::RegisterBlock;
}

impl Instance for Uart0 {
    const IDX: u8 = 0;
    const PERIPH_SEL: PeripheralSelect = PeripheralSelect::Uart0;
    const IRQ_RX: pac::Interrupt = pac::Interrupt::UART0_RX;
    const IRQ_TX: pac::Interrupt = pac::Interrupt::UART0_TX;

    unsafe fn steal() -> Self {
        pac::Peripherals::steal().uart0
    }
    fn ptr() -> *const uart_base::RegisterBlock {
        Uart0::ptr() as *const _
    }
}

impl Instance for Uart1 {
    const IDX: u8 = 1;
    const PERIPH_SEL: PeripheralSelect = PeripheralSelect::Uart1;
    const IRQ_RX: pac::Interrupt = pac::Interrupt::UART1_RX;
    const IRQ_TX: pac::Interrupt = pac::Interrupt::UART1_TX;

    unsafe fn steal() -> Self {
        pac::Peripherals::steal().uart1
    }
    fn ptr() -> *const uart_base::RegisterBlock {
        Uart1::ptr() as *const _
    }
}

impl Instance for Uart2 {
    const IDX: u8 = 2;
    const PERIPH_SEL: PeripheralSelect = PeripheralSelect::Uart2;
    const IRQ_RX: pac::Interrupt = pac::Interrupt::UART2_RX;
    const IRQ_TX: pac::Interrupt = pac::Interrupt::UART2_TX;

    unsafe fn steal() -> Self {
        pac::Peripherals::steal().uart2
    }
    fn ptr() -> *const uart_base::RegisterBlock {
        Uart2::ptr() as *const _
    }
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

impl<UartInstance> embedded_io::ErrorType for UartBase<UartInstance> {
    type Error = Error;
}

impl<UartInstance> embedded_hal_nb::serial::ErrorType for UartBase<UartInstance> {
    type Error = Error;
}

impl<Uart: Instance> embedded_hal_nb::serial::Read<u8> for UartBase<Uart> {
    fn read(&mut self) -> nb::Result<u8, Self::Error> {
        self.rx.read().map_err(|e| e.map(Error::Rx))
    }
}

impl<Uart: Instance> embedded_hal_nb::serial::Write<u8> for UartBase<Uart> {
    fn write(&mut self, word: u8) -> nb::Result<(), Self::Error> {
        self.tx.write(word).map_err(|e| {
            if let nb::Error::Other(_) = e {
                unreachable!()
            }
            nb::Error::WouldBlock
        })
    }

    fn flush(&mut self) -> nb::Result<(), Self::Error> {
        self.tx.flush().map_err(|e| {
            if let nb::Error::Other(_) = e {
                unreachable!()
            }
            nb::Error::WouldBlock
        })
    }
}

/// Serial abstraction. Entry point to create a new UART
pub struct Uart<UartInstance, Pins> {
    inner: UartBase<UartInstance>,
    pins: Pins,
}

impl<TxPinInst: TxPin<UartInstance>, RxPinInst: RxPin<UartInstance>, UartInstance: Instance>
    Uart<UartInstance, (TxPinInst, RxPinInst)>
{
    pub fn new(
        uart: UartInstance,
        pins: (TxPinInst, RxPinInst),
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
                tx: Tx::new(unsafe { UartInstance::steal() }),
                rx: Rx::new(unsafe { UartInstance::steal() }),
            },
            pins,
        }
        .init(config.into(), clocks)
    }

    pub fn new_with_clock_freq(
        uart: UartInstance,
        pins: (TxPinInst, RxPinInst),
        config: impl Into<Config>,
        syscfg: &mut va416xx::Sysconfig,
        clock: impl Into<Hertz>,
    ) -> Self {
        crate::clock::enable_peripheral_clock(syscfg, UartInstance::PERIPH_SEL);
        Uart {
            inner: UartBase {
                uart,
                tx: Tx::new(unsafe { UartInstance::steal() }),
                rx: Rx::new(unsafe { UartInstance::steal() }),
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

    pub fn downgrade(self) -> UartBase<UartInstance> {
        UartBase {
            uart: self.inner.uart,
            tx: self.inner.tx,
            rx: self.inner.rx,
        }
    }

    pub fn release(self) -> (UartInstance, (TxPinInst, RxPinInst)) {
        (self.inner.release(), self.pins)
    }
}

/// Serial receiver.
///
/// Can be created by using the [Uart::split] or [UartBase::split] API.
pub struct Rx<Uart>(Uart);

impl<Uart: Instance> Rx<Uart> {
    fn new(uart: Uart) -> Self {
        Self(uart)
    }

    /// Direct access to the peripheral structure.
    ///
    /// # Safety
    ///
    /// You must ensure that only registers related to the operation of the RX side are used.
    pub unsafe fn uart(&self) -> &Uart {
        &self.0
    }

    #[inline]
    pub fn clear_fifo(&self) {
        self.0.fifo_clr().write(|w| w.rxfifo().set_bit());
    }

    #[inline]
    pub fn enable(&mut self) {
        self.0.enable().modify(|_, w| w.rxenable().set_bit());
    }

    #[inline]
    pub fn disable(&mut self) {
        self.0.enable().modify(|_, w| w.rxenable().clear_bit());
    }

    /// Low level function to read a word from the UART FIFO.
    ///
    /// Uses the [nb] API to allow usage in blocking and non-blocking contexts.
    ///
    /// Please note that you might have to mask the returned value with 0xff to retrieve the actual
    /// value if you use the manual parity mode. See chapter 11.4.1 for more information.
    #[inline(always)]
    pub fn read_fifo(&self) -> nb::Result<u32, Infallible> {
        if self.0.rxstatus().read().rdavl().bit_is_clear() {
            return Err(nb::Error::WouldBlock);
        }
        Ok(self.read_fifo_unchecked())
    }

    /// Low level function to read a word from from the UART FIFO.
    ///
    /// This does not necesarily mean there is a word in the FIFO available.
    /// Use the [Self::read_fifo] function to read a word from the FIFO reliably using the [nb]
    /// API.
    ///
    /// Please note that you might have to mask the returned value with 0xff to retrieve the actual
    /// value if you use the manual parity mode. See chapter 11.4.1 for more information.
    #[inline(always)]
    pub fn read_fifo_unchecked(&self) -> u32 {
        self.0.data().read().bits()
    }

    pub fn into_rx_with_irq(self) -> RxWithIrq<Uart> {
        RxWithIrq(self)
    }

    pub fn release(self) -> Uart {
        self.0
    }
}

impl<Uart> embedded_io::ErrorType for Rx<Uart> {
    type Error = RxError;
}

impl<Uart> embedded_hal_nb::serial::ErrorType for Rx<Uart> {
    type Error = RxError;
}

impl<Uart: Instance> embedded_hal_nb::serial::Read<u8> for Rx<Uart> {
    fn read(&mut self) -> nb::Result<u8, Self::Error> {
        let uart = unsafe { &(*Uart::ptr()) };
        let status_reader = uart.rxstatus().read();
        let err = if status_reader.rxovr().bit_is_set() {
            Some(RxError::Overrun)
        } else if status_reader.rxfrm().bit_is_set() {
            Some(RxError::Framing)
        } else if status_reader.rxpar().bit_is_set() {
            Some(RxError::Parity)
        } else {
            None
        };
        if let Some(err) = err {
            // The status code is always related to the next bit for the framing
            // and parity status bits. We have to read the DATA register
            // so that the next status reflects the next DATA word
            // For overrun error, we read as well to clear the peripheral
            self.read_fifo_unchecked();
            return Err(err.into());
        }
        self.read_fifo().map(|val| (val & 0xff) as u8).map_err(|e| {
            if let nb::Error::Other(_) = e {
                unreachable!()
            }
            nb::Error::WouldBlock
        })
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

/// Serial transmitter
///
/// Can be created by using the [Uart::split] or [UartBase::split] API.
pub struct Tx<Uart>(Uart);

impl<Uart: Instance> Tx<Uart> {
    fn new(uart: Uart) -> Self {
        Self(uart)
    }

    /// Direct access to the peripheral structure.
    ///
    /// # Safety
    ///
    /// You must ensure that only registers related to the operation of the TX side are used.
    pub unsafe fn uart(&self) -> &Uart {
        &self.0
    }

    #[inline]
    pub fn clear_fifo(&self) {
        self.0.fifo_clr().write(|w| w.txfifo().set_bit());
    }

    #[inline]
    pub fn enable(&mut self) {
        self.0.enable().modify(|_, w| w.txenable().set_bit());
    }

    #[inline]
    pub fn disable(&mut self) {
        self.0.enable().modify(|_, w| w.txenable().clear_bit());
    }

    /// Low level function to write a word to the UART FIFO.
    ///
    /// Uses the [nb] API to allow usage in blocking and non-blocking contexts.
    ///
    /// Please note that you might have to mask the returned value with 0xff to retrieve the actual
    /// value if you use the manual parity mode. See chapter 11.4.1 for more information.
    #[inline(always)]
    pub fn write_fifo(&self, data: u32) -> nb::Result<(), Infallible> {
        if self.0.txstatus().read().wrrdy().bit_is_clear() {
            return Err(nb::Error::WouldBlock);
        }
        self.write_fifo_unchecked(data);
        Ok(())
    }

    /// Low level function to write a word to the UART FIFO.
    ///
    /// This does not necesarily mean that the FIFO can process another word because it might be
    /// full.
    /// Use the [Self::read_fifo] function to write a word to the FIFO reliably using the [nb]
    /// API.
    #[inline(always)]
    pub fn write_fifo_unchecked(&self, data: u32) {
        self.0.data().write(|w| unsafe { w.bits(data) });
    }
}

impl<Uart> embedded_io::ErrorType for Tx<Uart> {
    type Error = Infallible;
}

impl<Uart> embedded_hal_nb::serial::ErrorType for Tx<Uart> {
    type Error = Infallible;
}

impl<Uart: Instance> embedded_hal_nb::serial::Write<u8> for Tx<Uart> {
    fn write(&mut self, word: u8) -> nb::Result<(), Self::Error> {
        self.write_fifo(word as u32)
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

/// Serial receiver, using interrupts to offload reading to the hardware.
///
/// You can use [Rx::to_rx_with_irq] to convert a normal [Rx] structure into this structure.
/// This structure provides two distinct ways to read the UART RX using interrupts. It should
/// be noted that the interrupt service routine (ISR) still has to be provided by the user. However,
/// this structure provides API calls which can be used inside the ISRs to simplify the reading
/// of the UART.
///
///  1. The first way simply empties the FIFO on an interrupt into a user provided buffer. You
///     can simply use [Self::start] to prepare the peripheral and then call the
///     [Self::irq_handler] in the interrupt service routine.
///  2. The second way reads packets bounded by a maximum size or a baudtick based timeout. You
///     can use [Self::read_fixed_len_or_timeout_based_using_irq] to prepare the peripheral and
///     then call the [Self::irq_handler_max_size_or_timeout_based] in the interrupt service
///     routine. You have to call [Self::read_fixed_len_or_timeout_based_using_irq] in the ISR to
///     start reading the next packet.
pub struct RxWithIrq<Uart>(Rx<Uart>);

impl<Uart: Instance> RxWithIrq<Uart> {
    /// This function should be called once at initialization time if the regular
    /// [Self::irq_handler] is used to read the UART receiver to enable and start the receiver.
    pub fn start(&mut self) {
        self.0.enable();
        self.enable_rx_irq_sources(true);
        unsafe { enable_interrupt(Uart::IRQ_RX) };
    }

    #[inline(always)]
    pub fn uart(&self) -> &Uart {
        &self.0 .0
    }

    /// This function is used together with the [Self::irq_handler_max_size_or_timeout_based]
    /// function to read packets with a maximum size or variable sized packets by using the
    /// receive timeout of the hardware.
    ///
    /// This function should be called once at initialization to initiate the context state
    /// and to [Self::start] the receiver. After that, it should be called after each
    /// completed [Self::irq_handler_max_size_or_timeout_based] call to restart the reception
    /// of a packet.
    pub fn read_fixed_len_or_timeout_based_using_irq(
        &mut self,
        context: &mut IrqContextTimeoutOrMaxSize,
    ) -> Result<(), TransferPendingError> {
        if context.mode != IrqReceptionMode::Idle {
            return Err(TransferPendingError);
        }
        context.mode = IrqReceptionMode::Pending;
        context.rx_idx = 0;
        self.start();
        Ok(())
    }

    #[inline]
    fn enable_rx_irq_sources(&mut self, timeout: bool) {
        self.uart().irq_enb().modify(|_, w| {
            if timeout {
                w.irq_rx_to().set_bit();
            }
            w.irq_rx_status().set_bit();
            w.irq_rx().set_bit()
        });
    }

    #[inline]
    fn disable_rx_irq_sources(&mut self) {
        self.uart().irq_enb().modify(|_, w| {
            w.irq_rx_to().clear_bit();
            w.irq_rx_status().clear_bit();
            w.irq_rx().clear_bit()
        });
    }

    pub fn cancel_transfer(&mut self) {
        self.disable_rx_irq_sources();
        self.0.clear_fifo();
    }

    /// This function should be called in the user provided UART interrupt handler.
    ///
    /// It simply empties any bytes in the FIFO into the user provided buffer and returns the
    /// result of the operation.
    ///
    /// This function will not disable the RX interrupts, so you don't need to call any other
    /// API after calling this function to continue emptying the FIFO. RX errors are handled
    /// as partial errors and are returned as part of the [IrqResult].
    pub fn irq_handler(&mut self, buf: &mut [u8; 16]) -> IrqResult {
        let mut result = IrqResult::default();

        let irq_end = self.uart().irq_end().read();
        let enb_status = self.uart().enable().read();
        let rx_enabled = enb_status.rxenable().bit_is_set();

        // Half-Full interrupt. We have a guaranteed amount of data we can read.
        if irq_end.irq_rx().bit_is_set() {
            let available_bytes = self.uart().rxfifoirqtrg().read().bits() as usize;

            // If this interrupt bit is set, the trigger level is available at the very least.
            // Read everything as fast as possible
            for _ in 0..available_bytes {
                buf[result.bytes_read] = (self.uart().data().read().bits() & 0xff) as u8;
                result.bytes_read += 1;
            }
        }

        // Timeout, empty the FIFO completely.
        if irq_end.irq_rx_to().bit_is_set() {
            loop {
                // While there is data in the FIFO, write it into the reception buffer
                let read_result = self.0.read();
                if let Some(byte) = self.read_handler(&mut result.errors, &read_result) {
                    buf[result.bytes_read] = byte;
                    result.bytes_read += 1;
                } else {
                    break;
                }
            }
        }

        // RX transfer not complete, check for RX errors
        if rx_enabled {
            self.check_for_errors(&mut result.errors);
        }

        // Clear the interrupt status bits
        self.uart()
            .irq_clr()
            .write(|w| unsafe { w.bits(irq_end.bits()) });
        result
    }

    /// This function should be called in the user provided UART interrupt handler.
    ///
    /// This function is used to read packets which either have a maximum size or variable sized
    /// packet which are bounded by sufficient delays between them, triggering a hardware timeout.
    ///
    /// If either the maximum number of packets have been read or a timeout occured, the transfer
    /// will be deemed completed. The state information of the transfer is tracked in the
    /// [IrqContextTimeoutOrMaxSize] structure.
    ///
    /// If passed buffer is equal to or larger than the specified maximum length, an
    /// [BufferTooShortError] will be returned. Other RX errors are treated as partial errors
    /// and returned inside the [IrqResultMaxSizeOrTimeout] structure.
    pub fn irq_handler_max_size_or_timeout_based(
        &mut self,
        context: &mut IrqContextTimeoutOrMaxSize,
        buf: &mut [u8],
    ) -> Result<IrqResultMaxSizeOrTimeout, BufferTooShortError> {
        if buf.len() < context.max_len {
            return Err(BufferTooShortError {
                found: buf.len(),
                expected: context.max_len,
            });
        }
        let mut result = IrqResultMaxSizeOrTimeout::default();

        let irq_end = self.uart().irq_end().read();
        let enb_status = self.uart().enable().read();
        let rx_enabled = enb_status.rxenable().bit_is_set();

        // Half-Full interrupt. We have a guaranteed amount of data we can read.
        if irq_end.irq_rx().bit_is_set() {
            // Determine the number of bytes to read, ensuring we leave 1 byte in the FIFO.
            // We use this trick/hack because the timeout feature of the peripheral relies on data
            // being in the RX FIFO. If data continues arriving, another half-full IRQ will fire.
            // If not, the last byte(s) is/are emptied by the timeout interrupt.
            let available_bytes = self.uart().rxfifoirqtrg().read().bits() as usize;

            let bytes_to_read = core::cmp::min(
                available_bytes.saturating_sub(1),
                context.max_len - context.rx_idx,
            );

            // If this interrupt bit is set, the trigger level is available at the very least.
            // Read everything as fast as possible
            for _ in 0..bytes_to_read {
                buf[context.rx_idx] = (self.uart().data().read().bits() & 0xff) as u8;
                context.rx_idx += 1;
            }

            // On high-baudrates, data might be available immediately, and we possible have to
            // read continuosly? Then again, the CPU should always be faster than that. I'd rather
            // rely on the hardware firing another IRQ. I have not tried baudrates higher than
            // 115200 so far.
        }
        // Timeout, empty the FIFO completely.
        if irq_end.irq_rx_to().bit_is_set() {
            // While there is data in the FIFO, write it into the reception buffer
            loop {
                if context.rx_idx == context.max_len {
                    break;
                }
                let read_result = self.0.read();
                if let Some(byte) = self.read_handler(&mut result.errors, &read_result) {
                    buf[context.rx_idx] = byte;
                    context.rx_idx += 1;
                } else {
                    break;
                }
            }
            self.irq_completion_handler_max_size_timeout(&mut result, context);
            return Ok(result);
        }

        // RX transfer not complete, check for RX errors
        if (context.rx_idx < context.max_len) && rx_enabled {
            self.check_for_errors(&mut result.errors);
        }

        // Clear the interrupt status bits
        self.uart()
            .irq_clr()
            .write(|w| unsafe { w.bits(irq_end.bits()) });
        Ok(result)
    }

    fn read_handler(
        &self,
        errors: &mut Option<IrqUartError>,
        read_res: &nb::Result<u8, RxError>,
    ) -> Option<u8> {
        match read_res {
            Ok(byte) => Some(*byte),
            Err(nb::Error::WouldBlock) => None,
            Err(nb::Error::Other(e)) => {
                // Ensure `errors` is Some(IrqUartError), initializing if it's None
                let err = errors.get_or_insert(IrqUartError::default());

                // Now we can safely modify fields inside `err`
                match e {
                    RxError::Overrun => err.overflow = true,
                    RxError::Framing => err.framing = true,
                    RxError::Parity => err.parity = true,
                }
                None
            }
        }
    }

    fn check_for_errors(&self, errors: &mut Option<IrqUartError>) {
        let rx_status = self.uart().rxstatus().read();

        if rx_status.rxovr().bit_is_set()
            || rx_status.rxfrm().bit_is_set()
            || rx_status.rxpar().bit_is_set()
        {
            let err = errors.get_or_insert(IrqUartError::default());

            if rx_status.rxovr().bit_is_set() {
                err.overflow = true;
            }
            if rx_status.rxfrm().bit_is_set() {
                err.framing = true;
            }
            if rx_status.rxpar().bit_is_set() {
                err.parity = true;
            }
        }
    }

    fn irq_completion_handler_max_size_timeout(
        &mut self,
        res: &mut IrqResultMaxSizeOrTimeout,
        context: &mut IrqContextTimeoutOrMaxSize,
    ) {
        self.disable_rx_irq_sources();
        self.0.disable();
        res.bytes_read = context.rx_idx;
        res.complete = true;
        context.mode = IrqReceptionMode::Idle;
        context.rx_idx = 0;
    }

    /// # Safety
    ///
    /// This API allows creating multiple UART instances when releasing the TX structure as well.
    /// The user must ensure that these instances are not used to create multiple overlapping
    /// UART drivers.
    pub unsafe fn release(self) -> Uart {
        self.0.release()
    }
}
