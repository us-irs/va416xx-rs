//! # Type-level module for GPIO pins
//!
//! This documentation is strongly based on the
//! [atsamd documentation](https://docs.rs/atsamd-hal/latest/atsamd_hal/gpio/pin/index.html).
//!
//! This module provides a type-level API for GPIO pins. It uses the type system
//! to track the state of pins at compile-time. Representing GPIO pins in this
//! manner incurs no run-time overhead. Each [`Pin`] struct is zero-sized, so
//! there is no data to copy around. Instead, real code is generated as a side
//! effect of type transformations, and the resulting assembly is nearly
//! identical to the equivalent, hand-written C.
//!
//! To track the state of pins at compile-time, this module uses traits to
//! represent [type classes] and types as instances of those type classes. For
//! example, the trait [`InputConfig`] acts as a [type-level enum] of the
//! available input configurations, and the types [`Floating`], [`PullDown`] and
//! [`PullUp`] are its type-level variants.
//!
//! Type-level [`Pin`]s are parameterized by two type-level enums, [`PinId`] and
//! [`PinMode`].
//!
//! ```
//! pub struct Pin<I, M>
//! where
//!     I: PinId,
//!     M: PinMode,
//! {
//!     // ...
//! }
//! ```
//!
//! A `PinId` identifies a pin by it's group (A to G) and pin number. Each
//! `PinId` instance is named according to its datasheet identifier, e.g.
//! [PA2].
//!
//! A `PinMode` represents the various pin modes. The available `PinMode`
//! variants are [`Input`], [`Output`] and [`Alternate`], each with its own
//! corresponding configurations.
//!
//! It is not possible for users to create new instances of a [`Pin`]. Singleton
//! instances of each pin are made available to users through the PinsX
//! struct.
//!
//! Example for the pins of PORT A:
//!
//! To create the [PinsA] struct, users must supply the PAC
//! [Port](crate::pac::Porta) peripheral. The [PinsA] struct takes
//! ownership of the [Porta] and provides the corresponding pins. Each [`Pin`]
//! within the [PinsA] struct can be moved out and used individually.
//!
//!
//! ```no_run
//! let mut peripherals = Peripherals::take().unwrap();
//! let pinsa = PinsA::new(peripherals.porta);
//! ```
//!
//! Pins can be converted between modes using several different methods.
//!
//! ```no_run
//! // Use one of the literal function names
//! let pa0 = pinsa.pa0.into_floating_input();
//! // Use a generic method and one of the `PinMode` variant types
//! let pa0 = pinsa.pa0.into_mode::<FloatingInput>();
//! // Specify the target type and use `From`/`Into`
//! let pa0: Pin<PA0, FloatingInput> = pinsa.pa27.into();
//! ```
//!
//! # Embedded HAL traits
//!
//! This module implements all of the embedded HAL GPIO traits for each [`Pin`]
//! in the corresponding [`PinMode`]s, namely: [`InputPin`], [`OutputPin`],
//! and [`StatefulOutputPin`].
use core::{convert::Infallible, marker::PhantomData, mem::transmute};

pub use crate::clock::FilterClkSel;
use crate::typelevel::Sealed;
use embedded_hal::digital::{ErrorType, InputPin, OutputPin, StatefulOutputPin};
use va416xx::{Porta, Portb, Portc, Portd, Porte, Portf, Portg};

use super::{
    reg::RegisterInterface, DynAlternate, DynGroup, DynInput, DynOutput, DynPinId, DynPinMode,
};

//==================================================================================================
//  Errors and Definitions
//==================================================================================================

#[derive(Debug, PartialEq, Eq)]
pub enum InterruptEdge {
    HighToLow,
    LowToHigh,
    BothEdges,
}

#[derive(Debug, PartialEq, Eq)]
pub enum InterruptLevel {
    Low = 0,
    High = 1,
}

#[derive(Debug, PartialEq, Eq)]
pub enum PinState {
    Low = 0,
    High = 1,
}

//==================================================================================================
// Input configuration
//==================================================================================================

/// Type-level enum for input configurations
///
/// The valid options are [Floating], [PullDown] and [PullUp].
pub trait InputConfig: Sealed {
    /// Corresponding [DynInput]
    const DYN: DynInput;
}

pub enum Floating {}
pub enum PullDown {}
pub enum PullUp {}

impl InputConfig for Floating {
    const DYN: DynInput = DynInput::Floating;
}
impl InputConfig for PullDown {
    const DYN: DynInput = DynInput::PullDown;
}
impl InputConfig for PullUp {
    const DYN: DynInput = DynInput::PullUp;
}

impl Sealed for Floating {}
impl Sealed for PullDown {}
impl Sealed for PullUp {}

/// Type-level variant of [PinMode] for floating input mode
pub type InputFloating = Input<Floating>;
/// Type-level variant of [PinMode] for pull-down input mode
pub type InputPullDown = Input<PullDown>;
/// Type-level variant of [PinMode] for pull-up input mode
pub type InputPullUp = Input<PullUp>;

/// Type-level variant of [PinMode] for input modes
///
/// Type `C` is one of three input configurations: [Floating], [PullDown] or
/// [PullUp]
pub struct Input<C: InputConfig> {
    cfg: PhantomData<C>,
}

impl<C: InputConfig> Sealed for Input<C> {}

#[derive(Debug, PartialEq, Eq)]
pub enum FilterType {
    SystemClock = 0,
    DirectInputWithSynchronization = 1,
    FilterOneClockCycle = 2,
    FilterTwoClockCycles = 3,
    FilterThreeClockCycles = 4,
    FilterFourClockCycles = 5,
}

//==================================================================================================
// Output configuration
//==================================================================================================

pub trait OutputConfig: Sealed {
    const DYN: DynOutput;
}

pub trait ReadableOutput: Sealed {}

/// Type-level variant of [`OutputConfig`] for a push-pull configuration
pub enum PushPull {}
/// Type-level variant of [`OutputConfig`] for an open drain configuration
pub enum OpenDrain {}

/// Type-level variant of [`OutputConfig`] for a readable push-pull configuration
pub enum ReadablePushPull {}
/// Type-level variant of [`OutputConfig`] for a readable open-drain configuration
pub enum ReadableOpenDrain {}

impl Sealed for PushPull {}
impl Sealed for OpenDrain {}
impl Sealed for ReadableOpenDrain {}
impl Sealed for ReadablePushPull {}
impl ReadableOutput for ReadableOpenDrain {}
impl ReadableOutput for ReadablePushPull {}

impl OutputConfig for PushPull {
    const DYN: DynOutput = DynOutput::PushPull;
}
impl OutputConfig for OpenDrain {
    const DYN: DynOutput = DynOutput::OpenDrain;
}
impl OutputConfig for ReadablePushPull {
    const DYN: DynOutput = DynOutput::ReadablePushPull;
}
impl OutputConfig for ReadableOpenDrain {
    const DYN: DynOutput = DynOutput::ReadableOpenDrain;
}

/// Type-level variant of [`PinMode`] for output modes
///
/// Type `C` is one of four output configurations: [`PushPull`], [`OpenDrain`] or
/// their respective readable versions
pub struct Output<C: OutputConfig> {
    cfg: PhantomData<C>,
}

impl<C: OutputConfig> Sealed for Output<C> {}

/// Type-level variant of [`PinMode`] for push-pull output mode
pub type PushPullOutput = Output<PushPull>;
/// Type-level variant of [`PinMode`] for open drain output mode
pub type OutputOpenDrain = Output<OpenDrain>;

pub type OutputReadablePushPull = Output<ReadablePushPull>;
pub type OutputReadableOpenDrain = Output<ReadableOpenDrain>;

//==================================================================================================
//  Alternate configurations
//==================================================================================================

/// Type-level enum for alternate peripheral function configurations
pub trait AlternateConfig: Sealed {
    const DYN: DynAlternate;
}

pub enum Funsel1 {}
pub enum Funsel2 {}
pub enum Funsel3 {}

impl AlternateConfig for Funsel1 {
    const DYN: DynAlternate = DynAlternate::Sel1;
}
impl AlternateConfig for Funsel2 {
    const DYN: DynAlternate = DynAlternate::Sel2;
}
impl AlternateConfig for Funsel3 {
    const DYN: DynAlternate = DynAlternate::Sel3;
}

impl Sealed for Funsel1 {}
impl Sealed for Funsel2 {}
impl Sealed for Funsel3 {}

/// Type-level variant of [`PinMode`] for alternate peripheral functions
///
/// Type `C` is an [`AlternateConfig`]
pub struct Alternate<C: AlternateConfig> {
    cfg: PhantomData<C>,
}

impl<C: AlternateConfig> Sealed for Alternate<C> {}

pub type AltFunc1 = Alternate<Funsel1>;
pub type AltFunc2 = Alternate<Funsel2>;
pub type AltFunc3 = Alternate<Funsel3>;

/// Type alias for the [`PinMode`] at reset
pub type Reset = InputFloating;

//==================================================================================================
//  Pin modes
//==================================================================================================

/// Type-level enum representing pin modes
///
/// The valid options are [Input], [Output] and [Alternate].
pub trait PinMode: Sealed {
    /// Corresponding [DynPinMode]
    const DYN: DynPinMode;
}

impl<C: InputConfig> PinMode for Input<C> {
    const DYN: DynPinMode = DynPinMode::Input(C::DYN);
}
impl<C: OutputConfig> PinMode for Output<C> {
    const DYN: DynPinMode = DynPinMode::Output(C::DYN);
}
impl<C: AlternateConfig> PinMode for Alternate<C> {
    const DYN: DynPinMode = DynPinMode::Alternate(C::DYN);
}

//==================================================================================================
//  Pin IDs
//==================================================================================================

/// Type-level enum for pin IDs
pub trait PinId: Sealed {
    /// Corresponding [DynPinId]
    const DYN: DynPinId;
}

macro_rules! pin_id {
    ($Group:ident, $Id:ident, $NUM:literal $(, $meta: meta)?) => {
        // Need paste macro to use ident in doc attribute
        paste::paste! {
            $(#[$meta])?
            #[doc = "Pin ID representing pin " $Id]
            pub enum $Id {}

            $(#[$meta])?
            impl Sealed for $Id {}

            $(#[$meta])?
            impl PinId for $Id {
                const DYN: DynPinId = DynPinId {
                    group: DynGroup::$Group,
                    num: $NUM,
                };
            }
        }
    };
}

//==================================================================================================
//  Pin
//==================================================================================================

/// A type-level GPIO pin, parameterized by [`PinId`] and [`PinMode`] types
pub struct Pin<I: PinId, M: PinMode> {
    pub(in crate::gpio) regs: Registers<I>,
    mode: PhantomData<M>,
}

impl<I: PinId, M: PinMode> Pin<I, M> {
    /// Create a new [`Pin`]
    ///
    /// # Safety
    ///
    /// Each [`Pin`] must be a singleton. For a given [`PinId`], there must be
    /// at most one corresponding [`Pin`] in existence at any given time.
    /// Violating this requirement is `unsafe`.
    #[inline]
    pub(crate) unsafe fn new() -> Pin<I, M> {
        Pin {
            regs: Registers::new(),
            mode: PhantomData,
        }
    }

    /// Convert the pin to the requested [`PinMode`]
    #[inline]
    pub fn into_mode<N: PinMode>(mut self) -> Pin<I, N> {
        // Only modify registers if we are actually changing pin mode
        // This check should compile away
        if N::DYN != M::DYN {
            self.regs.change_mode::<N>();
        }
        // Safe because we drop the existing Pin
        unsafe { Pin::new() }
    }

    /// Configure the pin for function select 1. See Programmer Guide p.40 for the function table
    #[inline]
    pub fn into_funsel_1(self) -> Pin<I, AltFunc1> {
        self.into_mode()
    }

    /// Configure the pin for function select 2. See Programmer Guide p.40 for the function table
    #[inline]
    pub fn into_funsel_2(self) -> Pin<I, AltFunc2> {
        self.into_mode()
    }

    /// Configure the pin for function select 3. See Programmer Guide p.40 for the function table
    #[inline]
    pub fn into_funsel_3(self) -> Pin<I, AltFunc3> {
        self.into_mode()
    }

    /// Configure the pin to operate as a floating input
    #[inline]
    pub fn into_floating_input(self) -> Pin<I, InputFloating> {
        self.into_mode()
    }

    /// Configure the pin to operate as a pulled down input
    #[inline]
    pub fn into_pull_down_input(self) -> Pin<I, InputPullDown> {
        self.into_mode()
    }

    /// Configure the pin to operate as a pulled up input
    #[inline]
    pub fn into_pull_up_input(self) -> Pin<I, InputPullUp> {
        self.into_mode()
    }

    /// Configure the pin to operate as a push-pull output
    #[inline]
    pub fn into_push_pull_output(self) -> Pin<I, PushPullOutput> {
        self.into_mode()
    }

    /// Configure the pin to operate as a readable push-pull output
    #[inline]
    pub fn into_readable_push_pull_output(self) -> Pin<I, OutputReadablePushPull> {
        self.into_mode()
    }

    /// Configure the pin to operate as a readable open-drain output
    #[inline]
    pub fn into_readable_open_drain_output(self) -> Pin<I, OutputReadableOpenDrain> {
        self.into_mode()
    }

    common_reg_if_functions!();

    #[inline]
    pub(crate) fn _set_high(&mut self) {
        self.regs.write_pin(true)
    }

    #[inline]
    pub(crate) fn _set_low(&mut self) {
        self.regs.write_pin(false)
    }

    #[inline]
    pub(crate) fn _is_low(&self) -> bool {
        !self.regs.read_pin()
    }

    #[inline]
    pub(crate) fn _is_high(&self) -> bool {
        self.regs.read_pin()
    }
}

//==============================================================================
//  AnyPin
//==============================================================================

/// Type class for [`Pin`] types
///
/// This trait uses the [`AnyKind`] trait pattern to create a [type class] for
/// [`Pin`] types. See the `AnyKind` documentation for more details on the
/// pattern.
///
/// ## `v1` Compatibility
///
/// Normally, this trait would use `Is<Type = SpecificPin<Self>>` as a super
/// trait. But doing so would restrict implementations to only the `v2` `Pin`
/// type in this module. To aid in backwards compatibility, we want to implement
/// `AnyPin` for the `v1` `Pin` type as well. This is possible for a few
/// reasons. First, both structs are zero-sized, so there is no meaningful
/// memory layout to begin with. And even if there were, the `v1` `Pin` type is
/// a newtype wrapper around a `v2` `Pin`, and single-field structs are
/// guaranteed to have the same layout as the field, even for `repr(Rust)`.
///
/// [`AnyKind`]: crate::typelevel#anykind-trait-pattern
/// [type class]: crate::typelevel#type-classes
pub trait AnyPin
where
    Self: Sealed,
    Self: From<SpecificPin<Self>>,
    Self: Into<SpecificPin<Self>>,
    Self: AsRef<SpecificPin<Self>>,
    Self: AsMut<SpecificPin<Self>>,
{
    /// [`PinId`] of the corresponding [`Pin`]
    type Id: PinId;
    /// [`PinMode`] of the corresponding [`Pin`]
    type Mode: PinMode;
}

impl<I, M> Sealed for Pin<I, M>
where
    I: PinId,
    M: PinMode,
{
}

impl<I, M> AnyPin for Pin<I, M>
where
    I: PinId,
    M: PinMode,
{
    type Id = I;
    type Mode = M;
}

/// Type alias to recover the specific [`Pin`] type from an implementation of
/// [`AnyPin`]
///
/// See the [`AnyKind`] documentation for more details on the pattern.
///
/// [`AnyKind`]: crate::typelevel#anykind-trait-pattern
pub type SpecificPin<P> = Pin<<P as AnyPin>::Id, <P as AnyPin>::Mode>;

impl<P: AnyPin> AsRef<P> for SpecificPin<P> {
    #[inline]
    fn as_ref(&self) -> &P {
        // SAFETY: This is guaranteed to be safe, because P == SpecificPin<P>
        // Transmuting between `v1` and `v2` `Pin` types is also safe, because
        // both are zero-sized, and single-field, newtype structs are guaranteed
        // to have the same layout as the field anyway, even for repr(Rust).
        unsafe { transmute(self) }
    }
}

impl<P: AnyPin> AsMut<P> for SpecificPin<P> {
    #[inline]
    fn as_mut(&mut self) -> &mut P {
        // SAFETY: This is guaranteed to be safe, because P == SpecificPin<P>
        // Transmuting between `v1` and `v2` `Pin` types is also safe, because
        // both are zero-sized, and single-field, newtype structs are guaranteed
        // to have the same layout as the field anyway, even for repr(Rust).
        unsafe { transmute(self) }
    }
}

//==================================================================================================
//  Additional functionality
//==================================================================================================

impl<I: PinId, C: InputConfig> Pin<I, Input<C>> {
    pub fn interrupt_edge(mut self, edge_type: InterruptEdge) -> Self {
        self.regs.interrupt_edge(edge_type);
        self.irq_enb();
        self
    }

    pub fn interrupt_level(mut self, level_type: InterruptLevel) -> Self {
        self.regs.interrupt_level(level_type);
        self.irq_enb();
        self
    }
}

impl<I: PinId, C: OutputConfig> Pin<I, Output<C>> {
    /// See p.53 of the programmers guide for more information.
    /// Possible delays in clock cycles:
    ///  - Delay 1: 1
    ///  - Delay 2: 2
    ///  - Delay 1 + Delay 2: 3
    #[inline]
    pub fn delay(self, delay_1: bool, delay_2: bool) -> Self {
        self.regs.delay(delay_1, delay_2);
        self
    }

    /// See p.52 of the programmers guide for more information.
    /// When configured for pulse mode, a given pin will set the non-default state for exactly
    /// one clock cycle before returning to the configured default state
    pub fn pulse_mode(self, enable: bool, default_state: PinState) -> Self {
        self.regs.pulse_mode(enable, default_state);
        self
    }

    pub fn interrupt_edge(mut self, edge_type: InterruptEdge) -> Self {
        self.regs.interrupt_edge(edge_type);
        self.irq_enb();
        self
    }

    pub fn interrupt_level(mut self, level_type: InterruptLevel) -> Self {
        self.regs.interrupt_level(level_type);
        self.irq_enb();
        self
    }
}

impl<I: PinId, C: InputConfig> Pin<I, Input<C>> {
    /// See p.37 and p.38 of the programmers guide for more information.
    #[inline]
    pub fn filter_type(self, filter: FilterType, clksel: FilterClkSel) -> Self {
        self.regs.filter_type(filter, clksel);
        self
    }
}

//==================================================================================================
//  Embedded HAL traits
//==================================================================================================

impl<I, M> ErrorType for Pin<I, M>
where
    I: PinId,
    M: PinMode,
{
    type Error = Infallible;
}

impl<I: PinId, C: OutputConfig> OutputPin for Pin<I, Output<C>> {
    #[inline]
    fn set_high(&mut self) -> Result<(), Self::Error> {
        self._set_high();
        Ok(())
    }

    #[inline]
    fn set_low(&mut self) -> Result<(), Self::Error> {
        self._set_low();
        Ok(())
    }
}

impl<I, C> InputPin for Pin<I, Input<C>>
where
    I: PinId,
    C: InputConfig,
{
    #[inline]
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        Ok(self._is_high())
    }
    #[inline]
    fn is_low(&mut self) -> Result<bool, Self::Error> {
        Ok(self._is_low())
    }
}

impl<I, C> StatefulOutputPin for Pin<I, Output<C>>
where
    I: PinId,
    C: OutputConfig + ReadableOutput,
{
    #[inline]
    fn is_set_high(&mut self) -> Result<bool, Self::Error> {
        Ok(self._is_high())
    }
    #[inline]
    fn is_set_low(&mut self) -> Result<bool, Self::Error> {
        Ok(self._is_low())
    }
}

impl<I, C> InputPin for Pin<I, Output<C>>
where
    I: PinId,
    C: OutputConfig + ReadableOutput,
{
    #[inline]
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        Ok(self._is_high())
    }

    #[inline]
    fn is_low(&mut self) -> Result<bool, Self::Error> {
        Ok(self._is_low())
    }
}

//==================================================================================================
//  Registers
//==================================================================================================

/// Provide a safe register interface for [`Pin`]s
///
/// This `struct` takes ownership of a [`PinId`] and provides an API to
/// access the corresponding registers.
pub(in crate::gpio) struct Registers<I: PinId> {
    id: PhantomData<I>,
}

// [`Registers`] takes ownership of the [`PinId`], and [`Pin`] guarantees that
// each pin is a singleton, so this implementation is safe.
unsafe impl<I: PinId> RegisterInterface for Registers<I> {
    #[inline]
    fn id(&self) -> DynPinId {
        I::DYN
    }
}

impl<I: PinId> Registers<I> {
    /// Create a new instance of [`Registers`]
    ///
    /// # Safety
    ///
    /// Users must never create two simultaneous instances of this `struct` with
    /// the same [`PinId`]
    #[inline]
    unsafe fn new() -> Self {
        Registers { id: PhantomData }
    }

    /// Provide a type-level equivalent for the
    /// [`RegisterInterface::change_mode`] method.
    #[inline]
    pub(in crate::gpio) fn change_mode<M: PinMode>(&mut self) {
        RegisterInterface::change_mode(self, M::DYN);
    }
}

//==================================================================================================
//  Pin definitions
//==================================================================================================

macro_rules! pins {
    (
        $Port:ident, $PinsName:ident, $($Id:ident $(, $meta:meta)?)+,
    ) => {
        paste::paste!(
            /// Collection of all the individual [`Pin`]s for a given port (PORTA or PORTB)
            pub struct $PinsName {
                port: $Port,
                $(
                    $(#[$meta])?
                    #[doc = "Pin " $Id]
                    pub [<$Id:lower>]: Pin<$Id, Reset>,
                )+
            }

            impl $PinsName {
                /// Create a new struct containing all the Pins. Passing the IOCONFIG peripheral
                /// is optional because it might be required to create pin definitions for both
                /// ports.
                #[inline]
                pub fn new(
                    syscfg: &mut va416xx::Sysconfig,
                    port: $Port
                ) -> $PinsName {
                    syscfg.peripheral_clk_enable().modify(|_, w| {
                        w.[<$Port:lower>]().set_bit();
                        w.ioconfig().set_bit()
                    });
                    $PinsName {
                        port,
                        // Safe because we only create one `Pin` per `PinId`
                        $(
                            $(#[$meta])?
                            [<$Id:lower>]: unsafe { Pin::new() },
                        )+
                    }
                }

                /// Get the peripheral ID
                /// Safety: Read-only register
                pub fn get_perid() -> u32 {
                    let port = unsafe { &(*$Port::ptr()) };
                    port.perid().read().bits()
                }

                /// Consumes the Pins struct and returns the port definitions
                pub fn release(self) -> $Port {
                    self.port
                }
            }
        );
    }
}

//$Group:ident, $PinsName:ident, $Port:ident, [$(($Id:ident, $NUM:literal $(, $meta:meta)?)),+]
//$Group:ident, $PinsName:ident, $Port:ident, [$(($Id:ident, $NUM:literal, $meta: meta),)+]
macro_rules! declare_pins {
    (
        $Group:ident, $PinsName:ident, $Port:ident, [$(($Id:ident, $NUM:literal $(, $meta:meta)?)),+]
    ) => {
        pins!($Port, $PinsName, $($Id $(, $meta)?)+,);
        $(
            pin_id!($Group, $Id, $NUM $(, $meta)?);
        )+
    }
}

declare_pins!(
    A,
    PinsA,
    Porta,
    [
        (PA0, 0),
        (PA1, 1),
        (PA2, 2),
        (PA3, 3),
        (PA4, 4),
        (PA5, 5),
        (PA6, 6),
        (PA7, 7),
        (PA8, 8),
        (PA9, 9),
        (PA10, 10),
        (PA11, 11),
        (PA12, 12),
        (PA13, 13),
        (PA14, 14),
        (PA15, 15)
    ]
);

declare_pins!(
    B,
    PinsB,
    Portb,
    [
        (PB0, 0),
        (PB1, 1),
        (PB2, 2),
        (PB3, 3),
        (PB4, 4),
        (PB5, 5, cfg(not(feature = "va41628"))),
        (PB6, 6, cfg(not(feature = "va41628"))),
        (PB7, 7, cfg(not(feature = "va41628"))),
        (PB8, 8, cfg(not(feature = "va41628"))),
        (PB9, 9, cfg(not(feature = "va41628"))),
        (PB10, 10, cfg(not(feature = "va41628"))),
        (PB11, 11, cfg(not(feature = "va41628"))),
        (PB12, 12),
        (PB13, 13),
        (PB14, 14),
        (PB15, 15)
    ]
);

declare_pins!(
    C,
    PinsC,
    Portc,
    [
        (PC0, 0),
        (PC1, 1),
        (PC2, 2),
        (PC3, 3),
        (PC4, 4),
        (PC5, 5),
        (PC6, 6),
        (PC7, 7),
        (PC8, 8),
        (PC9, 9),
        (PC10, 10),
        (PC11, 11),
        (PC12, 12),
        (PC13, 13, cfg(not(feature = "va41628"))),
        (PC14, 14),
        (PC15, 15, cfg(not(feature = "va41628")))
    ]
);

declare_pins!(
    D,
    PinsD,
    Portd,
    [
        (PD0, 0, cfg(not(feature = "va41628"))),
        (PD1, 1, cfg(not(feature = "va41628"))),
        (PD2, 2, cfg(not(feature = "va41628"))),
        (PD3, 3, cfg(not(feature = "va41628"))),
        (PD4, 4, cfg(not(feature = "va41628"))),
        (PD5, 5, cfg(not(feature = "va41628"))),
        (PD6, 6, cfg(not(feature = "va41628"))),
        (PD7, 7, cfg(not(feature = "va41628"))),
        (PD8, 8, cfg(not(feature = "va41628"))),
        (PD9, 9, cfg(not(feature = "va41628"))),
        (PD10, 10),
        (PD11, 11),
        (PD12, 12),
        (PD13, 13),
        (PD14, 14),
        (PD15, 15)
    ]
);

declare_pins!(
    E,
    PinsE,
    Porte,
    [
        (PE0, 0),
        (PE1, 1),
        (PE2, 2),
        (PE3, 3),
        (PE4, 4),
        (PE5, 5),
        (PE6, 6),
        (PE7, 7),
        (PE8, 8),
        (PE9, 9),
        (PE10, 10, cfg(not(feature = "va41628"))),
        (PE11, 11, cfg(not(feature = "va41628"))),
        (PE12, 12),
        (PE13, 13),
        (PE14, 14),
        (PE15, 15)
    ]
);

declare_pins!(
    F,
    PinsF,
    Portf,
    [
        (PF0, 0),
        (PF1, 1),
        (PF2, 2, cfg(not(feature = "va41628"))),
        (PF3, 3, cfg(not(feature = "va41628"))),
        (PF4, 4, cfg(not(feature = "va41628"))),
        (PF5, 5, cfg(not(feature = "va41628"))),
        (PF6, 6, cfg(not(feature = "va41628"))),
        (PF7, 7, cfg(not(feature = "va41628"))),
        (PF8, 8, cfg(not(feature = "va41628"))),
        (PF9, 9),
        (PF10, 10, cfg(not(feature = "va41628"))),
        (PF11, 11),
        (PF12, 12),
        (PF13, 13),
        (PF14, 14),
        (PF15, 15)
    ]
);

declare_pins!(
    G,
    PinsG,
    Portg,
    [
        (PG0, 0),
        (PG1, 1),
        (PG2, 2),
        (PG3, 3),
        (PG4, 4),
        (PG5, 5),
        (PG6, 6),
        (PG7, 7)
    ]
);
