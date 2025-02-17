//! # Async UART reception functionality for the VA416xx family.
//!
//! This module provides the [RxAsync] and [RxAsyncOverwriting] struct which both implement the
//! [embedded_io_async::Read] trait.
//! This trait allows for asynchronous reception of data streams. Please note that this module does
//! not specify/declare the interrupt handlers which must be provided for async support to work.
//! However, it provides two interrupt handlers:
//!
//! - [on_interrupt_rx]
//! - [on_interrupt_rx_overwriting]
//!
//! The first two are used for the [RxAsync] struct, while the latter two are used with the
//! [RxAsyncOverwriting] struct. The later two will overwrite old values in the used ring buffer.
//!
//! Error handling is performed in the user interrupt handler by checking the [AsyncUartErrors]
//! structure returned by the interrupt handlers.
//!
//! # Example
//!
//! - [Async UART RX example](https://egit.irs.uni-stuttgart.de/rust/va108xx-rs/src/branch/main/examples/embassy/src/bin/async-uart-rx.rs)
use core::{cell::RefCell, convert::Infallible, future::Future, sync::atomic::Ordering};

use critical_section::Mutex;
use embassy_sync::waitqueue::AtomicWaker;
use embedded_io::ErrorType;
use portable_atomic::AtomicBool;
use va416xx::uart0 as uart_base;

use crate::enable_nvic_interrupt;

use super::{Bank, Instance, Rx, RxError, UartErrors};

static UART_RX_WAKERS: [AtomicWaker; 3] = [const { AtomicWaker::new() }; 3];
static RX_READ_ACTIVE: [AtomicBool; 3] = [const { AtomicBool::new(false) }; 3];
static RX_HAS_DATA: [AtomicBool; 3] = [const { AtomicBool::new(false) }; 3];

struct RxFuture {
    uart_idx: usize,
}

impl RxFuture {
    pub fn new<Uart: Instance>(_rx: &mut Rx<Uart>) -> Self {
        RX_READ_ACTIVE[Uart::IDX as usize].store(true, Ordering::Relaxed);
        Self {
            uart_idx: Uart::IDX as usize,
        }
    }
}

impl Future for RxFuture {
    type Output = Result<(), RxError>;

    fn poll(
        self: core::pin::Pin<&mut Self>,
        cx: &mut core::task::Context<'_>,
    ) -> core::task::Poll<Self::Output> {
        UART_RX_WAKERS[self.uart_idx].register(cx.waker());
        if RX_HAS_DATA[self.uart_idx].load(Ordering::Relaxed) {
            return core::task::Poll::Ready(Ok(()));
        }
        core::task::Poll::Pending
    }
}

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct AsyncUartErrors {
    /// Queue has overflowed, data might have been lost.
    pub queue_overflow: bool,
    /// UART errors.
    pub uart_errors: UartErrors,
}

fn on_interrupt_handle_rx_errors(uart: &'static uart_base::RegisterBlock) -> Option<UartErrors> {
    let rx_status = uart.rxstatus().read();
    if rx_status.rxovr().bit_is_set()
        || rx_status.rxfrm().bit_is_set()
        || rx_status.rxpar().bit_is_set()
    {
        let mut errors_val = UartErrors::default();

        if rx_status.rxovr().bit_is_set() {
            errors_val.overflow = true;
        }
        if rx_status.rxfrm().bit_is_set() {
            errors_val.framing = true;
        }
        if rx_status.rxpar().bit_is_set() {
            errors_val.parity = true;
        }
        return Some(errors_val);
    }
    None
}

fn on_interrupt_rx_common_post_processing(
    bank: Bank,
    rx_enabled: bool,
    read_some_data: bool,
    irq_end: u32,
) -> Option<UartErrors> {
    let idx = bank as usize;
    if read_some_data {
        RX_HAS_DATA[idx].store(true, Ordering::Relaxed);
        if RX_READ_ACTIVE[idx].load(Ordering::Relaxed) {
            UART_RX_WAKERS[idx].wake();
        }
    }

    let mut errors = None;
    let uart_regs = unsafe { bank.reg_block() };
    // Check for RX errors
    if rx_enabled {
        errors = on_interrupt_handle_rx_errors(uart_regs);
    }

    // Clear the interrupt status bits
    uart_regs.irq_clr().write(|w| unsafe { w.bits(irq_end) });
    errors
}

/// Interrupt handler with overwriting behaviour when the ring buffer is full.
///
/// Should be called in the user interrupt handler to enable
/// asynchronous reception. This variant will overwrite old data in the ring buffer in case
/// the ring buffer is full.
pub fn on_interrupt_rx_overwriting<const N: usize>(
    bank: Bank,
    prod: &mut heapless::spsc::Producer<u8, N>,
    shared_consumer: &Mutex<RefCell<Option<heapless::spsc::Consumer<'static, u8, N>>>>,
) -> Result<(), AsyncUartErrors> {
    on_interrupt_rx_async_heapless_queue_overwriting(bank, prod, shared_consumer)
}

pub fn on_interrupt_rx_async_heapless_queue_overwriting<const N: usize>(
    bank: Bank,
    prod: &mut heapless::spsc::Producer<u8, N>,
    shared_consumer: &Mutex<RefCell<Option<heapless::spsc::Consumer<'static, u8, N>>>>,
) -> Result<(), AsyncUartErrors> {
    let uart_regs = unsafe { bank.reg_block() };
    let irq_end = uart_regs.irq_end().read();
    let enb_status = uart_regs.enable().read();
    let rx_enabled = enb_status.rxenable().bit_is_set();
    let mut read_some_data = false;
    let mut queue_overflow = false;

    // Half-Full interrupt. We have a guaranteed amount of data we can read.
    if irq_end.irq_rx().bit_is_set() {
        let available_bytes = uart_regs.rxfifoirqtrg().read().bits() as usize;

        // If this interrupt bit is set, the trigger level is available at the very least.
        // Read everything as fast as possible
        for _ in 0..available_bytes {
            let byte = uart_regs.data().read().bits();
            if !prod.ready() {
                queue_overflow = true;
                critical_section::with(|cs| {
                    let mut cons_ref = shared_consumer.borrow(cs).borrow_mut();
                    cons_ref.as_mut().unwrap().dequeue();
                });
            }
            prod.enqueue(byte as u8).ok();
        }
        read_some_data = true;
    }

    // Timeout, empty the FIFO completely.
    if irq_end.irq_rx_to().bit_is_set() {
        while uart_regs.rxstatus().read().rdavl().bit_is_set() {
            // While there is data in the FIFO, write it into the reception buffer
            let byte = uart_regs.data().read().bits();
            if !prod.ready() {
                queue_overflow = true;
                critical_section::with(|cs| {
                    let mut cons_ref = shared_consumer.borrow(cs).borrow_mut();
                    cons_ref.as_mut().unwrap().dequeue();
                });
            }
            prod.enqueue(byte as u8).ok();
        }
        read_some_data = true;
    }

    let uart_errors =
        on_interrupt_rx_common_post_processing(bank, rx_enabled, read_some_data, irq_end.bits());
    if uart_errors.is_some() || queue_overflow {
        return Err(AsyncUartErrors {
            queue_overflow,
            uart_errors: uart_errors.unwrap_or_default(),
        });
    }
    Ok(())
}

/// Interrupt handler for asynchronous RX operations.
///
/// Should be called in the user interrupt handler to enable asynchronous reception.
pub fn on_interrupt_rx<const N: usize>(
    bank: Bank,
    prod: &mut heapless::spsc::Producer<'_, u8, N>,
) -> Result<(), AsyncUartErrors> {
    on_interrupt_rx_async_heapless_queue(bank, prod)
}

pub fn on_interrupt_rx_async_heapless_queue<const N: usize>(
    bank: Bank,
    prod: &mut heapless::spsc::Producer<'_, u8, N>,
) -> Result<(), AsyncUartErrors> {
    let uart = unsafe { bank.reg_block() };
    let irq_end = uart.irq_end().read();
    let enb_status = uart.enable().read();
    let rx_enabled = enb_status.rxenable().bit_is_set();
    let mut read_some_data = false;
    let mut queue_overflow = false;

    // Half-Full interrupt. We have a guaranteed amount of data we can read.
    if irq_end.irq_rx().bit_is_set() {
        let available_bytes = uart.rxfifoirqtrg().read().bits() as usize;

        // If this interrupt bit is set, the trigger level is available at the very least.
        // Read everything as fast as possible
        for _ in 0..available_bytes {
            let byte = uart.data().read().bits();
            if !prod.ready() {
                queue_overflow = true;
            }
            prod.enqueue(byte as u8).ok();
        }
        read_some_data = true;
    }

    // Timeout, empty the FIFO completely.
    if irq_end.irq_rx_to().bit_is_set() {
        while uart.rxstatus().read().rdavl().bit_is_set() {
            // While there is data in the FIFO, write it into the reception buffer
            let byte = uart.data().read().bits();
            if !prod.ready() {
                queue_overflow = true;
            }
            prod.enqueue(byte as u8).ok();
        }
        read_some_data = true;
    }

    let uart_errors =
        on_interrupt_rx_common_post_processing(bank, rx_enabled, read_some_data, irq_end.bits());
    if uart_errors.is_some() || queue_overflow {
        return Err(AsyncUartErrors {
            queue_overflow,
            uart_errors: uart_errors.unwrap_or_default(),
        });
    }
    Ok(())
}

struct ActiveReadGuard(usize);

impl Drop for ActiveReadGuard {
    fn drop(&mut self) {
        RX_READ_ACTIVE[self.0].store(false, Ordering::Relaxed);
    }
}

struct RxAsyncInner<Uart: Instance, const N: usize> {
    rx: Rx<Uart>,
    pub queue: heapless::spsc::Consumer<'static, u8, N>,
}

/// Core data structure to allow asynchronous UART reception.
///
/// If the ring buffer becomes full, data will be lost.
pub struct RxAsync<Uart: Instance, const N: usize>(Option<RxAsyncInner<Uart, N>>);

impl<Uart: Instance, const N: usize> ErrorType for RxAsync<Uart, N> {
    /// Error reporting is done using the result of the interrupt functions.
    type Error = Infallible;
}

fn stop_async_rx<Uart: Instance>(rx: &mut Rx<Uart>) {
    rx.disable_interrupts();
    rx.disable();
    unsafe {
        enable_nvic_interrupt(Uart::IRQ_RX);
    }
    rx.clear_fifo();
}

impl<Uart: Instance, const N: usize> RxAsync<Uart, N> {
    /// Create a new asynchronous receiver.
    ///
    /// The passed [heapless::spsc::Consumer] will be used to asynchronously receive data which
    /// is filled by the interrupt handler [on_interrupt_rx].
    pub fn new(mut rx: Rx<Uart>, queue: heapless::spsc::Consumer<'static, u8, N>) -> Self {
        rx.disable_interrupts();
        rx.disable();
        rx.clear_fifo();
        // Enable those together.
        critical_section::with(|_| {
            unsafe {
                enable_nvic_interrupt(Uart::IRQ_RX);
            }
            rx.enable_interrupts();
            rx.enable();
        });
        Self(Some(RxAsyncInner { rx, queue }))
    }

    pub fn stop(&mut self) {
        stop_async_rx(&mut self.0.as_mut().unwrap().rx);
    }

    pub fn release(mut self) -> (Rx<Uart>, heapless::spsc::Consumer<'static, u8, N>) {
        self.stop();
        let inner = self.0.take().unwrap();
        (inner.rx, inner.queue)
    }
}

impl<Uart: Instance, const N: usize> Drop for RxAsync<Uart, N> {
    fn drop(&mut self) {
        self.stop();
    }
}

impl<Uart: Instance, const N: usize> embedded_io_async::Read for RxAsync<Uart, N> {
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        // Need to wait for the IRQ to read data and set this flag. If the queue is not
        // empty, we can read data immediately.
        if self.0.as_ref().unwrap().queue.len() == 0 {
            RX_HAS_DATA[Uart::IDX as usize].store(false, Ordering::Relaxed);
        }
        let _guard = ActiveReadGuard(Uart::IDX as usize);
        let mut handle_data_in_queue = |consumer: &mut heapless::spsc::Consumer<'static, u8, N>| {
            let data_to_read = consumer.len().min(buf.len());
            for byte in buf.iter_mut().take(data_to_read) {
                // We own the consumer and we checked that the amount of data is guaranteed to be available.
                *byte = unsafe { consumer.dequeue_unchecked() };
            }
            data_to_read
        };
        let mut_ref = self.0.as_mut().unwrap();
        let fut = RxFuture::new(&mut mut_ref.rx);
        // Data is available, so read that data immediately.
        let read_data = handle_data_in_queue(&mut mut_ref.queue);
        if read_data > 0 {
            return Ok(read_data);
        }
        // Await data.
        let _ = fut.await;
        Ok(handle_data_in_queue(&mut mut_ref.queue))
    }
}

struct RxAsyncOverwritingInner<Uart: Instance, const N: usize> {
    rx: Rx<Uart>,
    pub shared_consumer: &'static Mutex<RefCell<Option<heapless::spsc::Consumer<'static, u8, N>>>>,
}

/// Core data structure to allow asynchronous UART reception.
///
/// If the ring buffer becomes full, the oldest data will be overwritten when using the
/// [on_interrupt_rx_overwriting] interrupt handlers.
pub struct RxAsyncOverwriting<Uart: Instance, const N: usize>(
    Option<RxAsyncOverwritingInner<Uart, N>>,
);

impl<Uart: Instance, const N: usize> ErrorType for RxAsyncOverwriting<Uart, N> {
    /// Error reporting is done using the result of the interrupt functions.
    type Error = Infallible;
}

impl<Uart: Instance, const N: usize> RxAsyncOverwriting<Uart, N> {
    /// Create a new asynchronous receiver.
    ///
    /// The passed shared [heapless::spsc::Consumer] will be used to asynchronously receive data
    /// which is filled by the interrupt handler. The shared property allows using it in the
    /// interrupt handler to overwrite old data.
    pub fn new(
        mut rx: Rx<Uart>,
        shared_consumer: &'static Mutex<RefCell<Option<heapless::spsc::Consumer<'static, u8, N>>>>,
    ) -> Self {
        rx.disable_interrupts();
        rx.disable();
        rx.clear_fifo();
        // Enable those together.
        critical_section::with(|_| {
            rx.enable_interrupts();
            rx.enable();
        });
        Self(Some(RxAsyncOverwritingInner {
            rx,
            shared_consumer,
        }))
    }

    pub fn stop(&mut self) {
        stop_async_rx(&mut self.0.as_mut().unwrap().rx);
    }

    pub fn release(mut self) -> Rx<Uart> {
        self.stop();
        let inner = self.0.take().unwrap();
        inner.rx
    }
}

impl<Uart: Instance, const N: usize> Drop for RxAsyncOverwriting<Uart, N> {
    fn drop(&mut self) {
        self.stop();
    }
}

impl<Uart: Instance, const N: usize> embedded_io_async::Read for RxAsyncOverwriting<Uart, N> {
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        // Need to wait for the IRQ to read data and set this flag. If the queue is not
        // empty, we can read data immediately.

        critical_section::with(|cs| {
            let queue = self.0.as_ref().unwrap().shared_consumer.borrow(cs);
            if queue.borrow().as_ref().unwrap().len() == 0 {
                RX_HAS_DATA[Uart::IDX as usize].store(false, Ordering::Relaxed);
            }
        });
        let _guard = ActiveReadGuard(Uart::IDX as usize);
        let mut handle_data_in_queue = |inner: &mut RxAsyncOverwritingInner<Uart, N>| {
            critical_section::with(|cs| {
                let mut consumer_ref = inner.shared_consumer.borrow(cs).borrow_mut();
                let consumer = consumer_ref.as_mut().unwrap();
                let data_to_read = consumer.len().min(buf.len());
                for byte in buf.iter_mut().take(data_to_read) {
                    // We own the consumer and we checked that the amount of data is guaranteed to be available.
                    *byte = unsafe { consumer.dequeue_unchecked() };
                }
                data_to_read
            })
        };
        let fut = RxFuture::new(&mut self.0.as_mut().unwrap().rx);
        // Data is available, so read that data immediately.
        let read_data = handle_data_in_queue(self.0.as_mut().unwrap());
        if read_data > 0 {
            return Ok(read_data);
        }
        // Await data.
        let _ = fut.await;
        let read_data = handle_data_in_queue(self.0.as_mut().unwrap());
        Ok(read_data)
    }
}
