//! # Async GPIO functionality for the VA416xx family.
//!
//! This module provides the [InputPinAsync] and [InputDynPinAsync] which both implement
//! the [embedded_hal_async::digital::Wait] trait. These types allow for asynchronous waiting
//! on GPIO pins. Please note that this module does not specify/declare the interrupt handlers
//! which must be provided for async support to work. However, it provides the
//! [on_interrupt_for_async_gpio_for_port] generic interrupt handler. This should be called in all
//! IRQ functions which handle any GPIO interrupts with the corresponding [Port] argument.
//!
//! # Example
//!
//! - [Async GPIO example](https://egit.irs.uni-stuttgart.de/rust/va108xx-rs/src/branch/main/examples/embassy/src/bin/async-gpio.rs)
use core::future::Future;

use embassy_sync::waitqueue::AtomicWaker;
use embedded_hal_async::digital::Wait;
use portable_atomic::AtomicBool;
use va416xx::{self as pac};

use crate::enable_nvic_interrupt;

use super::{
    pin, DynPin, DynPinId, InputConfig, InterruptEdge, InvalidPinTypeError, Pin, PinId, Port,
    NUM_PINS_PORT_A_TO_F,
};

static WAKERS_FOR_PORT_A: [AtomicWaker; NUM_PINS_PORT_A_TO_F] =
    [const { AtomicWaker::new() }; NUM_PINS_PORT_A_TO_F];
static WAKERS_FOR_PORT_B: [AtomicWaker; NUM_PINS_PORT_A_TO_F] =
    [const { AtomicWaker::new() }; NUM_PINS_PORT_A_TO_F];
static WAKERS_FOR_PORT_C: [AtomicWaker; NUM_PINS_PORT_A_TO_F] =
    [const { AtomicWaker::new() }; NUM_PINS_PORT_A_TO_F];
static WAKERS_FOR_PORT_D: [AtomicWaker; NUM_PINS_PORT_A_TO_F] =
    [const { AtomicWaker::new() }; NUM_PINS_PORT_A_TO_F];
static WAKERS_FOR_PORT_E: [AtomicWaker; NUM_PINS_PORT_A_TO_F] =
    [const { AtomicWaker::new() }; NUM_PINS_PORT_A_TO_F];
static WAKERS_FOR_PORT_F: [AtomicWaker; NUM_PINS_PORT_A_TO_F] =
    [const { AtomicWaker::new() }; NUM_PINS_PORT_A_TO_F];

static EDGE_DETECTION_PORT_A: [AtomicBool; NUM_PINS_PORT_A_TO_F] =
    [const { AtomicBool::new(false) }; NUM_PINS_PORT_A_TO_F];
static EDGE_DETECTION_PORT_B: [AtomicBool; NUM_PINS_PORT_A_TO_F] =
    [const { AtomicBool::new(false) }; NUM_PINS_PORT_A_TO_F];
static EDGE_DETECTION_PORT_C: [AtomicBool; NUM_PINS_PORT_A_TO_F] =
    [const { AtomicBool::new(false) }; NUM_PINS_PORT_A_TO_F];
static EDGE_DETECTION_PORT_D: [AtomicBool; NUM_PINS_PORT_A_TO_F] =
    [const { AtomicBool::new(false) }; NUM_PINS_PORT_A_TO_F];
static EDGE_DETECTION_PORT_E: [AtomicBool; NUM_PINS_PORT_A_TO_F] =
    [const { AtomicBool::new(false) }; NUM_PINS_PORT_A_TO_F];
static EDGE_DETECTION_PORT_F: [AtomicBool; NUM_PINS_PORT_A_TO_F] =
    [const { AtomicBool::new(false) }; NUM_PINS_PORT_A_TO_F];

#[derive(Debug, thiserror::Error)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[error("port G does not support async functionality")]
pub struct PortGDoesNotSupportAsyncError;

#[derive(Debug, thiserror::Error)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AsyncDynPinError {
    #[error("invalid pin type: {0}")]
    InvalidPinType(#[from] InvalidPinTypeError),
    #[error("port g does not support async functionality: {0}")]
    PortGDoesNotSupportAsync(#[from] PortGDoesNotSupportAsyncError),
}

/// Generic interrupt handler for GPIO interrupts on a specific port to support async functionalities
///
/// This function should be called in all interrupt handlers which handle any GPIO interrupts
/// matching the [Port] argument.
/// The handler will wake the corresponding wakers for the pins that triggered an interrupts
/// as well as update the static edge detection structures. This allows the pin future tocomplete
/// complete async operations.
pub fn on_interrupt_for_async_gpio_for_port(
    port: Port,
) -> Result<(), PortGDoesNotSupportAsyncError> {
    let periphs = unsafe { pac::Peripherals::steal() };

    let (irq_enb, edge_status, wakers, edge_detection) = match port {
        Port::A => (
            periphs.porta.irq_enb().read().bits(),
            periphs.porta.edge_status().read().bits(),
            &WAKERS_FOR_PORT_A,
            &EDGE_DETECTION_PORT_A,
        ),
        Port::B => (
            periphs.portb.irq_enb().read().bits(),
            periphs.portb.edge_status().read().bits(),
            &WAKERS_FOR_PORT_B,
            &EDGE_DETECTION_PORT_B,
        ),
        Port::C => (
            periphs.portc.irq_enb().read().bits(),
            periphs.portc.edge_status().read().bits(),
            &WAKERS_FOR_PORT_C,
            &EDGE_DETECTION_PORT_C,
        ),
        Port::D => (
            periphs.portd.irq_enb().read().bits(),
            periphs.portd.edge_status().read().bits(),
            &WAKERS_FOR_PORT_D,
            &EDGE_DETECTION_PORT_D,
        ),
        Port::E => (
            periphs.porte.irq_enb().read().bits(),
            periphs.porte.edge_status().read().bits(),
            &WAKERS_FOR_PORT_E,
            &EDGE_DETECTION_PORT_E,
        ),
        Port::F => (
            periphs.portf.irq_enb().read().bits(),
            periphs.portf.edge_status().read().bits(),
            &WAKERS_FOR_PORT_F,
            &EDGE_DETECTION_PORT_F,
        ),
        Port::G => return Err(PortGDoesNotSupportAsyncError),
    };

    on_interrupt_for_port(irq_enb, edge_status, wakers, edge_detection);
    Ok(())
}

#[inline]
fn on_interrupt_for_port(
    mut irq_enb: u32,
    edge_status: u32,
    wakers: &'static [AtomicWaker],
    edge_detection: &'static [AtomicBool],
) {
    while irq_enb != 0 {
        let bit_pos = irq_enb.trailing_zeros() as usize;
        let bit_mask = 1 << bit_pos;

        wakers[bit_pos].wake();

        if edge_status & bit_mask != 0 {
            edge_detection[bit_pos].store(true, core::sync::atomic::Ordering::Relaxed);

            // Clear the processed bit
            irq_enb &= !bit_mask;
        }
    }
}

/// Input pin future which implements the [Future] trait.
///
/// Generally, you want to use the [InputPinAsync] or [InputDynPinAsync] types instead of this
/// which also implements the [embedded_hal_async::digital::Wait] trait. However, access to this
/// struture is granted  to allow writing custom async structures.
pub struct InputPinFuture {
    pin_id: DynPinId,
    waker_group: &'static [AtomicWaker],
    edge_detection_group: &'static [AtomicBool],
}

impl InputPinFuture {
    pub fn new_with_dyn_pin(
        pin: &mut DynPin,
        edge: InterruptEdge,
    ) -> Result<Self, AsyncDynPinError> {
        if !pin.is_input_pin() {
            return Err(InvalidPinTypeError(pin.mode()).into());
        }
        if pin.id().port() == Port::G {
            return Err(PortGDoesNotSupportAsyncError.into());
        }

        let (waker_group, edge_detection_group) =
            Self::pin_group_to_waker_and_edge_detection_group(pin.id().port());
        edge_detection_group[pin.id().num() as usize]
            .store(false, core::sync::atomic::Ordering::Relaxed);
        // Unwraps okay, checked for PORT G previously
        pin.configure_edge_interrupt(edge).unwrap();
        unsafe { enable_nvic_interrupt(pin.irq_id().unwrap()) };
        pin.enable_interrupt();
        Ok(Self {
            pin_id: pin.id(),
            waker_group,
            edge_detection_group,
        })
    }

    pub fn new_with_pin<I: PinId, C: InputConfig>(
        pin: &mut Pin<I, pin::Input<C>>,
        edge: InterruptEdge,
    ) -> Result<Self, PortGDoesNotSupportAsyncError> {
        if pin.id().port() == Port::G {
            return Err(PortGDoesNotSupportAsyncError);
        }
        let (waker_group, edge_detection_group) =
            Self::pin_group_to_waker_and_edge_detection_group(pin.id().port());
        edge_detection_group[pin.id().num() as usize]
            .store(false, core::sync::atomic::Ordering::Relaxed);
        // Unwraps okay, checked for PORT G previously
        pin.configure_edge_interrupt(edge);
        unsafe { enable_nvic_interrupt(I::IRQ.unwrap()) };
        pin.enable_interrupt();
        Ok(Self {
            pin_id: pin.id(),
            waker_group,
            edge_detection_group,
        })
    }

    #[inline]
    pub fn pin_group_to_waker_and_edge_detection_group(
        group: Port,
    ) -> (&'static [AtomicWaker], &'static [AtomicBool]) {
        match group {
            Port::A => (WAKERS_FOR_PORT_A.as_ref(), EDGE_DETECTION_PORT_A.as_ref()),
            Port::B => (WAKERS_FOR_PORT_B.as_ref(), EDGE_DETECTION_PORT_B.as_ref()),
            Port::C => (WAKERS_FOR_PORT_C.as_ref(), EDGE_DETECTION_PORT_C.as_ref()),
            Port::D => (WAKERS_FOR_PORT_D.as_ref(), EDGE_DETECTION_PORT_D.as_ref()),
            Port::E => (WAKERS_FOR_PORT_E.as_ref(), EDGE_DETECTION_PORT_E.as_ref()),
            Port::F => (WAKERS_FOR_PORT_F.as_ref(), EDGE_DETECTION_PORT_F.as_ref()),
            _ => panic!("unexpected pin group G"),
        }
    }
}

impl Drop for InputPinFuture {
    fn drop(&mut self) {
        // The API ensures that we actually own the pin, so stealing it here is okay.
        unsafe { DynPin::steal(self.pin_id) }.disable_interrupt();
    }
}

impl Future for InputPinFuture {
    type Output = ();
    fn poll(
        self: core::pin::Pin<&mut Self>,
        cx: &mut core::task::Context<'_>,
    ) -> core::task::Poll<Self::Output> {
        let idx = self.pin_id.num() as usize;
        self.waker_group[idx].register(cx.waker());
        if self.edge_detection_group[idx].swap(false, core::sync::atomic::Ordering::Relaxed) {
            return core::task::Poll::Ready(());
        }
        core::task::Poll::Pending
    }
}

pub struct InputDynPinAsync {
    pin: DynPin,
}

impl InputDynPinAsync {
    /// Create a new asynchronous input pin from a [DynPin]. The interrupt ID to be used must be
    /// passed as well and is used to route and enable the interrupt.
    ///
    /// Please note that the interrupt handler itself must be provided by the user and the
    /// generic [on_interrupt_for_async_gpio_for_port] function must be called inside that function
    /// for the asynchronous functionality to work.
    pub fn new(pin: DynPin) -> Result<Self, AsyncDynPinError> {
        if !pin.is_input_pin() {
            return Err(InvalidPinTypeError(pin.mode()).into());
        }
        if pin.id().port() == Port::G {
            return Err(PortGDoesNotSupportAsyncError.into());
        }
        Ok(Self { pin })
    }

    /// Asynchronously wait until the pin is high.
    ///
    /// This returns immediately if the pin is already high.
    pub async fn wait_for_high(&mut self) {
        // Unwrap okay, checked pin in constructor.
        let fut =
            InputPinFuture::new_with_dyn_pin(&mut self.pin, InterruptEdge::LowToHigh).unwrap();
        if self.pin.is_high().unwrap() {
            return;
        }
        fut.await;
    }

    /// Asynchronously wait until the pin is low.
    ///
    /// This returns immediately if the pin is already high.
    pub async fn wait_for_low(&mut self) {
        // Unwrap okay, checked pin in constructor.
        let fut =
            InputPinFuture::new_with_dyn_pin(&mut self.pin, InterruptEdge::HighToLow).unwrap();
        if self.pin.is_low().unwrap() {
            return;
        }
        fut.await;
    }

    /// Asynchronously wait until the pin sees a falling edge.
    pub async fn wait_for_falling_edge(&mut self) {
        // Unwrap okay, checked pin in constructor.
        InputPinFuture::new_with_dyn_pin(&mut self.pin, InterruptEdge::HighToLow)
            .unwrap()
            .await;
    }

    /// Asynchronously wait until the pin sees a rising edge.
    pub async fn wait_for_rising_edge(&mut self) {
        // Unwrap okay, checked pin in constructor.
        InputPinFuture::new_with_dyn_pin(&mut self.pin, InterruptEdge::LowToHigh)
            .unwrap()
            .await;
    }

    /// Asynchronously wait until the pin sees any edge (either rising or falling).
    pub async fn wait_for_any_edge(&mut self) {
        // Unwrap okay, checked pin in constructor.
        InputPinFuture::new_with_dyn_pin(&mut self.pin, InterruptEdge::BothEdges)
            .unwrap()
            .await;
    }

    pub fn release(self) -> DynPin {
        self.pin
    }
}

impl embedded_hal::digital::ErrorType for InputDynPinAsync {
    type Error = core::convert::Infallible;
}

impl Wait for InputDynPinAsync {
    async fn wait_for_high(&mut self) -> Result<(), Self::Error> {
        self.wait_for_high().await;
        Ok(())
    }

    async fn wait_for_low(&mut self) -> Result<(), Self::Error> {
        self.wait_for_low().await;
        Ok(())
    }

    async fn wait_for_rising_edge(&mut self) -> Result<(), Self::Error> {
        self.wait_for_rising_edge().await;
        Ok(())
    }

    async fn wait_for_falling_edge(&mut self) -> Result<(), Self::Error> {
        self.wait_for_falling_edge().await;
        Ok(())
    }

    async fn wait_for_any_edge(&mut self) -> Result<(), Self::Error> {
        self.wait_for_any_edge().await;
        Ok(())
    }
}

pub struct InputPinAsync<I: PinId, C: InputConfig> {
    pin: Pin<I, pin::Input<C>>,
}

impl<I: PinId, C: InputConfig> InputPinAsync<I, C> {
    /// Create a new asynchronous input pin from a typed [Pin]. The interrupt ID to be used must be
    /// passed as well and is used to route and enable the interrupt.
    ///
    /// Please note that the interrupt handler itself must be provided by the user and the
    /// generic [on_interrupt_for_async_gpio_for_port] function must be called inside that function
    /// for the asynchronous functionality to work.
    pub fn new(pin: Pin<I, pin::Input<C>>) -> Result<Self, PortGDoesNotSupportAsyncError> {
        if pin.id().port() == Port::G {
            return Err(PortGDoesNotSupportAsyncError);
        }
        Ok(Self { pin })
    }

    /// Asynchronously wait until the pin is high.
    ///
    /// This returns immediately if the pin is already high.
    pub async fn wait_for_high(&mut self) {
        // Unwrap okay, checked pin in constructor.
        let fut = InputPinFuture::new_with_pin(&mut self.pin, InterruptEdge::LowToHigh).unwrap();
        if self.pin.is_high() {
            return;
        }
        fut.await;
    }

    /// Asynchronously wait until the pin is low.
    ///
    /// This returns immediately if the pin is already high.
    pub async fn wait_for_low(&mut self) {
        let fut = InputPinFuture::new_with_pin(&mut self.pin, InterruptEdge::HighToLow).unwrap();
        if self.pin.is_low() {
            return;
        }
        fut.await;
    }

    /// Asynchronously wait until the pin sees falling edge.
    pub async fn wait_for_falling_edge(&mut self) {
        // Unwrap okay, checked pin in constructor.
        InputPinFuture::new_with_pin(&mut self.pin, InterruptEdge::HighToLow)
            .unwrap()
            .await;
    }

    /// Asynchronously wait until the pin sees rising edge.
    pub async fn wait_for_rising_edge(&mut self) {
        // Unwrap okay, checked pin in constructor.
        InputPinFuture::new_with_pin(&mut self.pin, InterruptEdge::LowToHigh)
            .unwrap()
            .await;
    }

    /// Asynchronously wait until the pin sees any edge (either rising or falling).
    pub async fn wait_for_any_edge(&mut self) {
        // Unwrap okay, checked pin in constructor.
        InputPinFuture::new_with_pin(&mut self.pin, InterruptEdge::BothEdges)
            .unwrap()
            .await;
    }

    pub fn release(self) -> Pin<I, pin::Input<C>> {
        self.pin
    }
}
impl<I: PinId, C: InputConfig> embedded_hal::digital::ErrorType for InputPinAsync<I, C> {
    type Error = core::convert::Infallible;
}

impl<I: PinId, C: InputConfig> Wait for InputPinAsync<I, C> {
    async fn wait_for_high(&mut self) -> Result<(), Self::Error> {
        self.wait_for_high().await;
        Ok(())
    }

    async fn wait_for_low(&mut self) -> Result<(), Self::Error> {
        self.wait_for_low().await;
        Ok(())
    }

    async fn wait_for_rising_edge(&mut self) -> Result<(), Self::Error> {
        self.wait_for_rising_edge().await;
        Ok(())
    }

    async fn wait_for_falling_edge(&mut self) -> Result<(), Self::Error> {
        self.wait_for_falling_edge().await;
        Ok(())
    }

    async fn wait_for_any_edge(&mut self) -> Result<(), Self::Error> {
        self.wait_for_any_edge().await;
        Ok(())
    }
}
