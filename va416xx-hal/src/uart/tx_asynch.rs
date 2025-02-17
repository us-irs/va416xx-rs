//! # Async UART transmission functionality for the VA416xx family.
//!
//! This module provides the [TxAsync] struct which implements the [embedded_io_async::Write] trait.
//! This trait allows for asynchronous sending of data streams. Please note that this module does
//! not specify/declare the interrupt handlers which must be provided for async support to work.
//! However, it the [on_interrupt_tx] interrupt handler.
//!
//! This handler should be called in ALL user interrupt handlers which handle UART TX interrupts
//! for a given UART bank.
//!
//! # Example
//!
//! - [Async UART TX example](https://egit.irs.uni-stuttgart.de/rust/va416xx-rs/src/branch/main/examples/embassy/src/bin/async-uart-tx.rs)
use core::{cell::RefCell, future::Future};

use critical_section::Mutex;
use embassy_sync::waitqueue::AtomicWaker;
use embedded_io_async::Write;
use portable_atomic::AtomicBool;

use super::*;

static UART_TX_WAKERS: [AtomicWaker; 3] = [const { AtomicWaker::new() }; 3];
static TX_CONTEXTS: [Mutex<RefCell<TxContext>>; 3] =
    [const { Mutex::new(RefCell::new(TxContext::new())) }; 3];
// Completion flag. Kept outside of the context structure as an atomic to avoid
// critical section.
static TX_DONE: [AtomicBool; 3] = [const { AtomicBool::new(false) }; 3];

/// This is a generic interrupt handler to handle asynchronous UART TX operations for a given
/// UART bank.
///
/// The user has to call this once in the interrupt handler responsible for the TX interrupts on
/// the given UART bank.
pub fn on_interrupt_tx(bank: Bank) {
    let uart = unsafe { bank.reg_block() };
    let idx = bank as usize;
    let irq_enb = uart.irq_enb().read();
    // IRQ is not related to TX.
    if irq_enb.irq_tx().bit_is_clear() || irq_enb.irq_tx_empty().bit_is_clear() {
        return;
    }

    let tx_status = uart.txstatus().read();
    let unexpected_overrun = tx_status.wrlost().bit_is_set();
    let mut context = critical_section::with(|cs| {
        let context_ref = TX_CONTEXTS[idx].borrow(cs);
        *context_ref.borrow()
    });
    context.tx_overrun = unexpected_overrun;
    if context.progress >= context.slice.len && !tx_status.wrbusy().bit_is_set() {
        uart.irq_enb().modify(|_, w| {
            w.irq_tx().clear_bit();
            w.irq_tx_empty().clear_bit();
            w.irq_tx_status().clear_bit()
        });
        uart.enable().modify(|_, w| w.txenable().clear_bit());
        // Write back updated context structure.
        critical_section::with(|cs| {
            let context_ref = TX_CONTEXTS[idx].borrow(cs);
            *context_ref.borrow_mut() = context;
        });
        // Transfer is done.
        TX_DONE[idx].store(true, core::sync::atomic::Ordering::Relaxed);
        UART_TX_WAKERS[idx].wake();
        return;
    }
    // Safety: We documented that the user provided slice must outlive the future, so we convert
    // the raw pointer back to the slice here.
    let slice = unsafe { core::slice::from_raw_parts(context.slice.data, context.slice.len) };
    while context.progress < context.slice.len {
        let wrrdy = uart.txstatus().read().wrrdy().bit_is_set();
        if !wrrdy {
            break;
        }
        // Safety: TX structure is owned by the future which does not write into the the data
        // register, so we can assume we are the only one writing to the data register.
        uart.data()
            .write(|w| unsafe { w.bits(slice[context.progress] as u32) });
        context.progress += 1;
    }

    // Write back updated context structure.
    critical_section::with(|cs| {
        let context_ref = TX_CONTEXTS[idx].borrow(cs);
        *context_ref.borrow_mut() = context;
    });
}

#[derive(Debug, Copy, Clone)]
pub struct TxContext {
    progress: usize,
    tx_overrun: bool,
    slice: RawBufSlice,
}

#[allow(clippy::new_without_default)]
impl TxContext {
    pub const fn new() -> Self {
        Self {
            progress: 0,
            tx_overrun: false,
            slice: RawBufSlice::new_empty(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct RawBufSlice {
    data: *const u8,
    len: usize,
}

/// Safety: This type MUST be used with mutex to ensure concurrent access is valid.
unsafe impl Send for RawBufSlice {}

impl RawBufSlice {
    /// # Safety
    ///
    /// This function stores the raw pointer of the passed data slice. The user MUST ensure
    /// that the slice outlives the data structure.
    #[allow(dead_code)]
    const unsafe fn new(data: &[u8]) -> Self {
        Self {
            data: data.as_ptr(),
            len: data.len(),
        }
    }

    const fn new_empty() -> Self {
        Self {
            data: core::ptr::null(),
            len: 0,
        }
    }

    /// # Safety
    ///
    /// This function stores the raw pointer of the passed data slice. The user MUST ensure
    /// that the slice outlives the data structure.
    pub unsafe fn set(&mut self, data: &[u8]) {
        self.data = data.as_ptr();
        self.len = data.len();
    }
}

pub struct TxFuture {
    uart_idx: usize,
}

impl TxFuture {
    /// # Safety
    ///
    /// This function stores the raw pointer of the passed data slice. The user MUST ensure
    /// that the slice outlives the data structure.
    pub unsafe fn new<Uart: Instance>(tx: &mut Tx<Uart>, data: &[u8]) -> Self {
        TX_DONE[Uart::IDX as usize].store(false, core::sync::atomic::Ordering::Relaxed);
        tx.disable_interrupts();
        tx.disable();
        tx.clear_fifo();

        let uart_tx = unsafe { tx.uart() };
        let init_fill_count = core::cmp::min(data.len(), 16);
        // We fill the FIFO.
        for data in data.iter().take(init_fill_count) {
            uart_tx.data().write(|w| unsafe { w.bits(*data as u32) });
        }
        critical_section::with(|cs| {
            let context_ref = TX_CONTEXTS[Uart::IDX as usize].borrow(cs);
            let mut context = context_ref.borrow_mut();
            context.slice.set(data);
            context.progress = init_fill_count;

            // Ensure those are enabled inside a critical section at the same time. Can lead to
            // weird glitches otherwise.
            tx.enable_interrupts();
            tx.enable();
        });
        Self {
            uart_idx: Uart::IDX as usize,
        }
    }
}

impl Future for TxFuture {
    type Output = Result<usize, TxOverrunError>;

    fn poll(
        self: core::pin::Pin<&mut Self>,
        cx: &mut core::task::Context<'_>,
    ) -> core::task::Poll<Self::Output> {
        UART_TX_WAKERS[self.uart_idx].register(cx.waker());
        if TX_DONE[self.uart_idx].swap(false, core::sync::atomic::Ordering::Relaxed) {
            let progress = critical_section::with(|cs| {
                TX_CONTEXTS[self.uart_idx].borrow(cs).borrow().progress
            });
            return core::task::Poll::Ready(Ok(progress));
        }
        core::task::Poll::Pending
    }
}

impl Drop for TxFuture {
    fn drop(&mut self) {
        let reg_block = match self.uart_idx {
            0 => unsafe { pac::Uart0::reg_block() },
            1 => unsafe { pac::Uart1::reg_block() },
            2 => unsafe { pac::Uart2::reg_block() },
            _ => unreachable!(),
        };

        disable_tx_interrupts(reg_block);
        disable_tx(reg_block);
    }
}

pub struct TxAsync<Uart: Instance> {
    tx: Tx<Uart>,
}

impl<Uart: Instance> TxAsync<Uart> {
    /// Create a new asynchronous TX object.
    ///
    /// This function also enable the NVIC interrupt, but does not enable the peripheral specific
    /// interrupts.
    pub fn new(tx: Tx<Uart>) -> Self {
        // Safety: We own TX now.
        unsafe { enable_nvic_interrupt(Uart::IRQ_TX) };
        Self { tx }
    }

    /// This function also disables the NVIC interrupt.
    pub fn release(self) -> Tx<Uart> {
        disable_nvic_interrupt(Uart::IRQ_TX);
        self.tx
    }
}

#[derive(Debug, thiserror::Error)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[error("TX overrun error")]
pub struct TxOverrunError;

impl embedded_io_async::Error for TxOverrunError {
    fn kind(&self) -> embedded_io_async::ErrorKind {
        embedded_io_async::ErrorKind::Other
    }
}

impl<Uart: Instance> embedded_io::ErrorType for TxAsync<Uart> {
    type Error = TxOverrunError;
}

impl<Uart: Instance> Write for TxAsync<Uart> {
    /// Write a buffer asynchronously.
    ///
    /// This implementation is not side effect free, and a started future might have already
    /// written part of the passed buffer.
    async fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        let fut = unsafe { TxFuture::new(&mut self.tx, buf) };
        fut.await
    }
}
