//! API for Pulse-Width Modulation (PWM)
//!
//! The Vorago VA416xx devices use the TIM peripherals to perform PWM related tasks.
//!
//! ## Examples
//!
//! - [PWM example](https://egit.irs.uni-stuttgart.de/rust/va416xx-rs/src/branch/main/examples/simple/examples/pwm.rs)
use core::convert::Infallible;
use core::marker::PhantomData;

use crate::pac;
use crate::time::Hertz;
pub use crate::timer::ValidTim;
use crate::timer::{TimAndPinRegister, TimDynRegister, TimPin, TimRegInterface, ValidTimAndPin};
use crate::{clock::Clocks, gpio::DynPinId};

const DUTY_MAX: u16 = u16::MAX;

#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub(crate) struct PwmCommon {
    clock: Hertz,
    /// For PWMB, this is the upper limit
    current_duty: u16,
    /// For PWMA, this value will not be used
    current_lower_limit: u16,
    current_period: Hertz,
    current_rst_val: u32,
}

enum StatusSelPwm {
    PwmA = 3,
    PwmB = 4,
}

pub struct PwmA {}
pub struct PwmB {}

//==================================================================================================
// Strongly typed PWM pin
//==================================================================================================

pub struct PwmPin<Pin: TimPin, Tim: ValidTim, Mode = PwmA> {
    reg: TimAndPinRegister<Pin, Tim>,
    inner: ReducedPwmPin<Mode>,
    mode: PhantomData<Mode>,
}

impl<Pin: TimPin, Tim: ValidTim, Mode> PwmPin<Pin, Tim, Mode>
where
    (Pin, Tim): ValidTimAndPin<Pin, Tim>,
{
    /// Create a new stronlgy typed PWM pin
    pub fn new(
        pin_and_tim: (Pin, Tim),
        sys_cfg: &mut pac::Sysconfig,
        clocks: &Clocks,
        initial_period: impl Into<Hertz> + Copy,
    ) -> Self {
        let mut pin = PwmPin {
            inner: ReducedPwmPin::<Mode>::new(
                Tim::ID,
                Pin::DYN,
                PwmCommon {
                    clock: Tim::clock(clocks),
                    current_duty: 0,
                    current_lower_limit: 0,
                    current_period: initial_period.into(),
                    current_rst_val: 0,
                },
            ),
            reg: unsafe { TimAndPinRegister::new(pin_and_tim.0, pin_and_tim.1) },
            mode: PhantomData,
        };
        sys_cfg
            .tim_clk_enable()
            .modify(|r, w| unsafe { w.bits(r.bits() | pin.reg.mask_32()) });
        pin.enable_pwm_a();
        pin.set_period(initial_period);
        pin
    }

    pub fn downgrade(self) -> ReducedPwmPin<Mode> {
        self.inner
    }

    pub fn release(self) -> (Pin, Tim) {
        self.reg.release()
    }

    #[inline]
    fn enable_pwm_a(&mut self) {
        self.inner.enable_pwm_a();
    }

    #[inline]
    fn enable_pwm_b(&mut self) {
        self.inner.enable_pwm_b();
    }

    #[inline]
    pub fn get_period(&self) -> Hertz {
        self.inner.get_period()
    }

    #[inline]
    pub fn set_period(&mut self, period: impl Into<Hertz>) {
        self.inner.set_period(period);
    }

    #[inline]
    pub fn disable(&mut self) {
        self.inner.disable();
    }

    #[inline]
    pub fn enable(&mut self) {
        self.inner.enable();
    }

    #[inline]
    pub fn period(&self) -> Hertz {
        self.inner.period()
    }

    #[inline(always)]
    pub fn duty(&self) -> u16 {
        self.inner.duty()
    }
}

impl<Pin: TimPin, Tim: ValidTim> From<PwmPin<Pin, Tim, PwmA>> for PwmPin<Pin, Tim, PwmB>
where
    (Pin, Tim): ValidTimAndPin<Pin, Tim>,
{
    fn from(other: PwmPin<Pin, Tim, PwmA>) -> Self {
        let mut pwmb = Self {
            reg: other.reg,
            inner: other.inner.into(),
            mode: PhantomData,
        };
        pwmb.enable_pwm_b();
        pwmb
    }
}

impl<PIN: TimPin, TIM: ValidTim> From<PwmPin<PIN, TIM, PwmB>> for PwmPin<PIN, TIM, PwmA>
where
    (PIN, TIM): ValidTimAndPin<PIN, TIM>,
{
    fn from(other: PwmPin<PIN, TIM, PwmB>) -> Self {
        let mut pwmb = Self {
            reg: other.reg,
            inner: other.inner.into(),
            mode: PhantomData,
        };
        pwmb.enable_pwm_a();
        pwmb
    }
}

impl<Pin: TimPin, Tim: ValidTim> PwmPin<Pin, Tim, PwmA>
where
    (Pin, Tim): ValidTimAndPin<Pin, Tim>,
{
    pub fn pwma(
        tim_and_pin: (Pin, Tim),
        sys_cfg: &mut pac::Sysconfig,
        clocks: &Clocks,
        initial_period: impl Into<Hertz> + Copy,
    ) -> Self {
        let mut pin: PwmPin<Pin, Tim, PwmA> =
            Self::new(tim_and_pin, sys_cfg, clocks, initial_period);
        pin.enable_pwm_a();
        pin
    }
}

impl<Pin: TimPin, Tim: ValidTim> PwmPin<Pin, Tim, PwmB>
where
    (Pin, Tim): ValidTimAndPin<Pin, Tim>,
{
    pub fn pwmb(
        tim_and_pin: (Pin, Tim),
        sys_cfg: &mut pac::Sysconfig,
        clocks: &Clocks,
        initial_period: impl Into<Hertz> + Copy,
    ) -> Self {
        let mut pin: PwmPin<Pin, Tim, PwmB> =
            Self::new(tim_and_pin, sys_cfg, clocks, initial_period);
        pin.enable_pwm_b();
        pin
    }
}

//==================================================================================================
// Reduced PWM pin
//==================================================================================================

/// Reduced version where type information is deleted
pub struct ReducedPwmPin<Mode = PwmA> {
    dyn_reg: TimDynRegister,
    common: PwmCommon,
    mode: PhantomData<Mode>,
}

impl<Mode> ReducedPwmPin<Mode> {
    pub(crate) fn new(tim_id: u8, pin_id: DynPinId, common: PwmCommon) -> Self {
        Self {
            dyn_reg: TimDynRegister { tim_id, pin_id },
            common,
            mode: PhantomData,
        }
    }

    #[inline]
    fn enable_pwm_a(&mut self) {
        self.dyn_reg
            .reg_block()
            .ctrl()
            .modify(|_, w| unsafe { w.status_sel().bits(StatusSelPwm::PwmA as u8) });
    }

    #[inline]
    fn enable_pwm_b(&mut self) {
        self.dyn_reg
            .reg_block()
            .ctrl()
            .modify(|_, w| unsafe { w.status_sel().bits(StatusSelPwm::PwmB as u8) });
    }

    #[inline]
    pub fn get_period(&self) -> Hertz {
        self.common.current_period
    }

    #[inline]
    pub fn set_period(&mut self, period: impl Into<Hertz>) {
        self.common.current_period = period.into();
        // Avoid division by 0
        if self.common.current_period.raw() == 0 {
            return;
        }
        self.common.current_rst_val = self.common.clock.raw() / self.common.current_period.raw();
        self.dyn_reg
            .reg_block()
            .rst_value()
            .write(|w| unsafe { w.bits(self.common.current_rst_val) });
    }

    #[inline]
    pub fn disable(&mut self) {
        self.dyn_reg
            .reg_block()
            .ctrl()
            .modify(|_, w| w.enable().clear_bit());
    }

    #[inline]
    pub fn enable(&mut self) {
        self.dyn_reg
            .reg_block()
            .ctrl()
            .modify(|_, w| w.enable().set_bit());
    }

    #[inline]
    pub fn period(&self) -> Hertz {
        self.common.current_period
    }

    #[inline(always)]
    pub fn duty(&self) -> u16 {
        self.common.current_duty
    }
}

impl<Pin: TimPin, Tim: ValidTim> From<PwmPin<Pin, Tim, PwmA>> for ReducedPwmPin<PwmA>
where
    (Pin, Tim): ValidTimAndPin<Pin, Tim>,
{
    fn from(value: PwmPin<Pin, Tim, PwmA>) -> Self {
        value.downgrade()
    }
}

impl<Pin: TimPin, Tim: ValidTim> From<PwmPin<Pin, Tim, PwmB>> for ReducedPwmPin<PwmB>
where
    (Pin, Tim): ValidTimAndPin<Pin, Tim>,
{
    fn from(value: PwmPin<Pin, Tim, PwmB>) -> Self {
        value.downgrade()
    }
}

impl From<ReducedPwmPin<PwmA>> for ReducedPwmPin<PwmB> {
    fn from(other: ReducedPwmPin<PwmA>) -> Self {
        let mut pwmb = Self {
            dyn_reg: other.dyn_reg,
            common: other.common,
            mode: PhantomData,
        };
        pwmb.enable_pwm_b();
        pwmb
    }
}

impl From<ReducedPwmPin<PwmB>> for ReducedPwmPin<PwmA> {
    fn from(other: ReducedPwmPin<PwmB>) -> Self {
        let mut pwmb = Self {
            dyn_reg: other.dyn_reg,
            common: other.common,
            mode: PhantomData,
        };
        pwmb.enable_pwm_a();
        pwmb
    }
}

//==================================================================================================
// PWMB implementations
//==================================================================================================

impl<Pin: TimPin, Tim: ValidTim> PwmPin<Pin, Tim, PwmB>
where
    (Pin, Tim): ValidTimAndPin<Pin, Tim>,
{
    pub fn pwmb_lower_limit(&self) -> u16 {
        self.inner.pwmb_lower_limit()
    }

    pub fn pwmb_upper_limit(&self) -> u16 {
        self.inner.pwmb_upper_limit()
    }

    /// Set the lower limit for PWMB
    ///
    /// The PWM signal will be 1 as long as the current RST counter is larger than
    /// the lower limit. For example, with a lower limit of 0.5 and and an upper limit
    /// of 0.7, Only a fixed period between 0.5 * period and 0.7 * period will be in a high
    /// state
    pub fn set_pwmb_lower_limit(&mut self, duty: u16) {
        self.inner.set_pwmb_lower_limit(duty);
    }

    /// Set the higher limit for PWMB
    ///
    /// The PWM signal will be 1 as long as the current RST counter is smaller than
    /// the higher limit. For example, with a lower limit of 0.5 and and an upper limit
    /// of 0.7, Only a fixed period between 0.5 * period and 0.7 * period will be in a high
    /// state
    pub fn set_pwmb_upper_limit(&mut self, duty: u16) {
        self.inner.set_pwmb_upper_limit(duty);
    }
}

impl ReducedPwmPin<PwmB> {
    #[inline(always)]
    pub fn pwmb_lower_limit(&self) -> u16 {
        self.common.current_lower_limit
    }

    #[inline(always)]
    pub fn pwmb_upper_limit(&self) -> u16 {
        self.common.current_duty
    }

    /// Set the lower limit for PWMB
    ///
    /// The PWM signal will be 1 as long as the current RST counter is larger than
    /// the lower limit. For example, with a lower limit of 0.5 and and an upper limit
    /// of 0.7, Only a fixed period between 0.5 * period and 0.7 * period will be in a high
    /// state
    #[inline(always)]
    pub fn set_pwmb_lower_limit(&mut self, duty: u16) {
        self.common.current_lower_limit = duty;
        let pwmb_val: u64 = (self.common.current_rst_val as u64
            * self.common.current_lower_limit as u64)
            / DUTY_MAX as u64;
        self.dyn_reg
            .reg_block()
            .pwmb_value()
            .write(|w| unsafe { w.bits(pwmb_val as u32) });
    }

    /// Set the higher limit for PWMB
    ///
    /// The PWM signal will be 1 as long as the current RST counter is smaller than
    /// the higher limit. For example, with a lower limit of 0.5 and and an upper limit
    /// of 0.7, Only a fixed period between 0.5 * period and 0.7 * period will be in a high
    /// state
    pub fn set_pwmb_upper_limit(&mut self, duty: u16) {
        self.common.current_duty = duty;
        let pwma_val: u64 = (self.common.current_rst_val as u64 * self.common.current_duty as u64)
            / DUTY_MAX as u64;
        self.dyn_reg
            .reg_block()
            .pwma_value()
            .write(|w| unsafe { w.bits(pwma_val as u32) });
    }
}

//==================================================================================================
// Embedded HAL implementation: PWMA only
//==================================================================================================

impl<Pin: TimPin, Tim: ValidTim> embedded_hal::pwm::ErrorType for PwmPin<Pin, Tim> {
    type Error = Infallible;
}

impl embedded_hal::pwm::ErrorType for ReducedPwmPin {
    type Error = Infallible;
}

impl embedded_hal::pwm::SetDutyCycle for ReducedPwmPin {
    #[inline]
    fn max_duty_cycle(&self) -> u16 {
        DUTY_MAX
    }

    #[inline]
    fn set_duty_cycle(&mut self, duty: u16) -> Result<(), Self::Error> {
        self.common.current_duty = duty;
        let pwma_val: u64 = (self.common.current_rst_val as u64
            * (DUTY_MAX as u64 - self.common.current_duty as u64))
            / DUTY_MAX as u64;
        self.dyn_reg
            .reg_block()
            .pwma_value()
            .write(|w| unsafe { w.bits(pwma_val as u32) });
        Ok(())
    }
}

impl<Pin: TimPin, Tim: ValidTim> embedded_hal::pwm::SetDutyCycle for PwmPin<Pin, Tim> {
    #[inline]
    fn max_duty_cycle(&self) -> u16 {
        DUTY_MAX
    }

    #[inline]
    fn set_duty_cycle(&mut self, duty: u16) -> Result<(), Self::Error> {
        self.inner.set_duty_cycle(duty)
    }
}

/// Get the corresponding u16 duty cycle from a percent value ranging between 0.0 and 1.0.
///
/// Please note that this might load a lot of floating point code because this processor does not
/// have a FPU
pub fn get_duty_from_percent(percent: f32) -> u16 {
    if percent > 1.0 {
        DUTY_MAX
    } else if percent <= 0.0 {
        0
    } else {
        (percent * DUTY_MAX as f32) as u16
    }
}
