use embedded_hal::digital::{ErrorKind, ErrorType, InputPin, OutputPin, StatefulOutputPin};

use super::{
    reg::RegisterInterface, FilterClkSel, FilterType, InterruptEdge, InterruptLevel, Pin, PinId,
    PinMode, PinState,
};

//==================================================================================================
//  DynPinMode configurations
//==================================================================================================

/// Value-level `enum` for disabled configurations
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum DynDisabled {
    Floating,
    PullDown,
    PullUp,
}

/// Value-level `enum` for input configurations
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum DynInput {
    Floating,
    PullDown,
    PullUp,
}

/// Value-level `enum` for output configurations
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum DynOutput {
    PushPull,
    OpenDrain,
    ReadablePushPull,
    ReadableOpenDrain,
}

pub type DynAlternate = crate::FunSel;

//==================================================================================================
//  DynPinMode
//==================================================================================================

/// Value-level `enum` representing pin modes
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum DynPinMode {
    Input(DynInput),
    Output(DynOutput),
    Alternate(DynAlternate),
}

/// Value-level variant of [`DynPinMode`] for floating input mode
pub const DYN_FLOATING_INPUT: DynPinMode = DynPinMode::Input(DynInput::Floating);
/// Value-level variant of [`DynPinMode`] for pull-down input mode
pub const DYN_PULL_DOWN_INPUT: DynPinMode = DynPinMode::Input(DynInput::PullDown);
/// Value-level variant of [`DynPinMode`] for pull-up input mode
pub const DYN_PULL_UP_INPUT: DynPinMode = DynPinMode::Input(DynInput::PullUp);

/// Value-level variant of [`DynPinMode`] for push-pull output mode
pub const DYN_PUSH_PULL_OUTPUT: DynPinMode = DynPinMode::Output(DynOutput::PushPull);
/// Value-level variant of [`DynPinMode`] for open-drain output mode
pub const DYN_OPEN_DRAIN_OUTPUT: DynPinMode = DynPinMode::Output(DynOutput::OpenDrain);
/// Value-level variant of [`DynPinMode`] for readable push-pull output mode
pub const DYN_RD_PUSH_PULL_OUTPUT: DynPinMode = DynPinMode::Output(DynOutput::ReadablePushPull);
/// Value-level variant of [`DynPinMode`] for readable opendrain output mode
pub const DYN_RD_OPEN_DRAIN_OUTPUT: DynPinMode = DynPinMode::Output(DynOutput::ReadableOpenDrain);

/// Value-level variant of [`DynPinMode`] for function select 1
pub const DYN_ALT_FUNC_1: DynPinMode = DynPinMode::Alternate(DynAlternate::Sel1);
/// Value-level variant of [`DynPinMode`] for function select 2
pub const DYN_ALT_FUNC_2: DynPinMode = DynPinMode::Alternate(DynAlternate::Sel2);
/// Value-level variant of [`DynPinMode`] for function select 3
pub const DYN_ALT_FUNC_3: DynPinMode = DynPinMode::Alternate(DynAlternate::Sel3);

//==================================================================================================
//  DynGroup & DynPinId
//==================================================================================================

/// Value-level `enum` for pin groups
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum DynGroup {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

/// Value-level `struct` representing pin IDs
#[derive(PartialEq, Eq, Clone, Copy)]
pub struct DynPinId {
    pub group: DynGroup,
    pub num: u8,
}

//==============================================================================
//  DynRegisters
//==============================================================================

/// Provide a safe register interface for [`DynPin`]s
///
/// This `struct` takes ownership of a [`DynPinId`] and provides an API to
/// access the corresponding regsiters.
struct DynRegisters {
    id: DynPinId,
}

// [`DynRegisters`] takes ownership of the [`DynPinId`], and [`DynPin`]
// guarantees that each pin is a singleton, so this implementation is safe.
unsafe impl RegisterInterface for DynRegisters {
    #[inline]
    fn id(&self) -> DynPinId {
        self.id
    }
}

impl DynRegisters {
    /// Create a new instance of [`DynRegisters`]
    ///
    /// # Safety
    ///
    /// Users must never create two simultaneous instances of this `struct` with
    /// the same [`DynPinId`]
    #[inline]
    unsafe fn new(id: DynPinId) -> Self {
        DynRegisters { id }
    }
}

//==============================================================================
//  Error
//==============================================================================

/// GPIO error type
///
/// [`DynPin`]s are not tracked and verified at compile-time, so run-time
/// operations are fallible. This `enum` represents the corresponding errors.
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct InvalidPinTypeError(pub(crate) ());

impl embedded_hal::digital::Error for InvalidPinTypeError {
    fn kind(&self) -> embedded_hal::digital::ErrorKind {
        ErrorKind::Other
    }
}

//==================================================================================================
//  DynPin
//==================================================================================================

/// A value-level pin, parameterized by [`DynPinId`] and [`DynPinMode`]
///
/// This type acts as a type-erased version of [`Pin`]. Every pin is represented
/// by the same type, and pins are tracked and distinguished at run-time.
pub struct DynPin {
    regs: DynRegisters,
    mode: DynPinMode,
}

impl DynPin {
    /// Create a new [`DynPin`]
    ///
    /// # Safety
    ///
    /// Each [`DynPin`] must be a singleton. For a given [`DynPinId`], there
    /// must be at most one corresponding [`DynPin`] in existence at any given
    /// time.  Violating this requirement is `unsafe`.
    #[inline]
    unsafe fn new(id: DynPinId, mode: DynPinMode) -> Self {
        DynPin {
            regs: DynRegisters::new(id),
            mode,
        }
    }

    /// Return a copy of the pin ID
    #[inline]
    pub fn id(&self) -> DynPinId {
        self.regs.id
    }

    /// Return a copy of the pin mode
    #[inline]
    pub fn mode(&self) -> DynPinMode {
        self.mode
    }

    /// Convert the pin to the requested [`DynPinMode`]
    #[inline]
    pub fn into_mode(&mut self, mode: DynPinMode) {
        // Only modify registers if we are actually changing pin mode
        if mode != self.mode {
            self.regs.change_mode(mode);
            self.mode = mode;
        }
    }

    #[inline]
    pub fn into_funsel_1(&mut self) {
        self.into_mode(DYN_ALT_FUNC_1);
    }

    #[inline]
    pub fn into_funsel_2(&mut self) {
        self.into_mode(DYN_ALT_FUNC_2);
    }

    #[inline]
    pub fn into_funsel_3(&mut self) {
        self.into_mode(DYN_ALT_FUNC_3);
    }

    /// Configure the pin to operate as a floating input
    #[inline]
    pub fn into_floating_input(&mut self) {
        self.into_mode(DYN_FLOATING_INPUT);
    }

    /// Configure the pin to operate as a pulled down input
    #[inline]
    pub fn into_pull_down_input(&mut self) {
        self.into_mode(DYN_PULL_DOWN_INPUT);
    }

    /// Configure the pin to operate as a pulled up input
    #[inline]
    pub fn into_pull_up_input(&mut self) {
        self.into_mode(DYN_PULL_UP_INPUT);
    }

    /// Configure the pin to operate as a push-pull output
    #[inline]
    pub fn into_push_pull_output(&mut self) {
        self.into_mode(DYN_PUSH_PULL_OUTPUT);
    }

    /// Configure the pin to operate as a push-pull output
    #[inline]
    pub fn into_open_drain_output(&mut self) {
        self.into_mode(DYN_OPEN_DRAIN_OUTPUT);
    }

    /// Configure the pin to operate as a push-pull output
    #[inline]
    pub fn into_readable_push_pull_output(&mut self) {
        self.into_mode(DYN_RD_PUSH_PULL_OUTPUT);
    }

    /// Configure the pin to operate as a push-pull output
    #[inline]
    pub fn into_readable_open_drain_output(&mut self) {
        self.into_mode(DYN_RD_OPEN_DRAIN_OUTPUT);
    }

    common_reg_if_functions!();

    /// See p.53 of the programmers guide for more information.
    /// Possible delays in clock cycles:
    ///  - Delay 1: 1
    ///  - Delay 2: 2
    ///  - Delay 1 + Delay 2: 3
    #[inline]
    pub fn delay(self, delay_1: bool, delay_2: bool) -> Result<Self, InvalidPinTypeError> {
        match self.mode {
            DynPinMode::Output(_) => {
                self.regs.delay(delay_1, delay_2);
                Ok(self)
            }
            _ => Err(InvalidPinTypeError(())),
        }
    }

    /// See p.52 of the programmers guide for more information.
    /// When configured for pulse mode, a given pin will set the non-default state for exactly
    /// one clock cycle before returning to the configured default state
    pub fn pulse_mode(
        self,
        enable: bool,
        default_state: PinState,
    ) -> Result<Self, InvalidPinTypeError> {
        match self.mode {
            DynPinMode::Output(_) => {
                self.regs.pulse_mode(enable, default_state);
                Ok(self)
            }
            _ => Err(InvalidPinTypeError(())),
        }
    }

    /// See p.37 and p.38 of the programmers guide for more information.
    #[inline]
    pub fn filter_type(
        self,
        filter: FilterType,
        clksel: FilterClkSel,
    ) -> Result<Self, InvalidPinTypeError> {
        match self.mode {
            DynPinMode::Input(_) => {
                self.regs.filter_type(filter, clksel);
                Ok(self)
            }
            _ => Err(InvalidPinTypeError(())),
        }
    }

    pub fn interrupt_edge(mut self, edge_type: InterruptEdge) -> Result<Self, InvalidPinTypeError> {
        match self.mode {
            DynPinMode::Input(_) | DynPinMode::Output(_) => {
                self.regs.interrupt_edge(edge_type);
                self.irq_enb();
                Ok(self)
            }
            _ => Err(InvalidPinTypeError(())),
        }
    }

    pub fn interrupt_level(
        mut self,
        level_type: InterruptLevel,
    ) -> Result<Self, InvalidPinTypeError> {
        match self.mode {
            DynPinMode::Input(_) | DynPinMode::Output(_) => {
                self.regs.interrupt_level(level_type);
                self.irq_enb();
                Ok(self)
            }
            _ => Err(InvalidPinTypeError(())),
        }
    }

    #[inline]
    fn _read(&self) -> Result<bool, InvalidPinTypeError> {
        match self.mode {
            DynPinMode::Input(_) | DYN_RD_OPEN_DRAIN_OUTPUT | DYN_RD_PUSH_PULL_OUTPUT => {
                Ok(self.regs.read_pin())
            }
            _ => Err(InvalidPinTypeError(())),
        }
    }
    #[inline]
    fn _write(&mut self, bit: bool) -> Result<(), InvalidPinTypeError> {
        match self.mode {
            DynPinMode::Output(_) => {
                self.regs.write_pin(bit);
                Ok(())
            }
            _ => Err(InvalidPinTypeError(())),
        }
    }

    #[inline]
    fn _is_low(&self) -> Result<bool, InvalidPinTypeError> {
        self._read().map(|v| !v)
    }
    #[inline]
    fn _is_high(&self) -> Result<bool, InvalidPinTypeError> {
        self._read()
    }
    #[inline]
    fn _set_low(&mut self) -> Result<(), InvalidPinTypeError> {
        self._write(false)
    }
    #[inline]
    fn _set_high(&mut self) -> Result<(), InvalidPinTypeError> {
        self._write(true)
    }
}

//==============================================================================
//  Convert between Pin and DynPin
//==============================================================================

impl<I, M> From<Pin<I, M>> for DynPin
where
    I: PinId,
    M: PinMode,
{
    /// Erase the type-level information in a [`Pin`] and return a value-level
    /// [`DynPin`]
    #[inline]
    fn from(_pin: Pin<I, M>) -> Self {
        // The `Pin` is consumed, so it is safe to replace it with the
        // corresponding `DynPin`
        unsafe { DynPin::new(I::DYN, M::DYN) }
    }
}

impl<I, M> TryFrom<DynPin> for Pin<I, M>
where
    I: PinId,
    M: PinMode,
{
    type Error = InvalidPinTypeError;

    /// Try to recreate a type-level [`Pin`] from a value-level [`DynPin`]
    ///
    /// There is no way for the compiler to know if the conversion will be
    /// successful at compile-time. We must verify the conversion at run-time
    /// or refuse to perform it.
    #[inline]
    fn try_from(pin: DynPin) -> Result<Self, Self::Error> {
        if pin.regs.id == I::DYN && pin.mode == M::DYN {
            // The `DynPin` is consumed, so it is safe to replace it with the
            // corresponding `Pin`
            Ok(unsafe { Self::new() })
        } else {
            Err(InvalidPinTypeError(()))
        }
    }
}

//==============================================================================
// Embedded HAL v1 traits
//==============================================================================

impl ErrorType for DynPin {
    type Error = InvalidPinTypeError;
}

impl OutputPin for DynPin {
    #[inline]
    fn set_high(&mut self) -> Result<(), Self::Error> {
        self._set_high()
    }
    #[inline]
    fn set_low(&mut self) -> Result<(), Self::Error> {
        self._set_low()
    }
}

impl InputPin for DynPin {
    #[inline]
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        self._is_high()
    }
    #[inline]
    fn is_low(&mut self) -> Result<bool, Self::Error> {
        self._is_low()
    }
}

impl StatefulOutputPin for DynPin {
    #[inline]
    fn is_set_high(&mut self) -> Result<bool, Self::Error> {
        self._is_high()
    }
    #[inline]
    fn is_set_low(&mut self) -> Result<bool, Self::Error> {
        self._is_low()
    }
}
