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

pub struct PwmBase {
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
// Common
//==================================================================================================

macro_rules! pwm_common_func {
    () => {
        #[inline]
        fn enable_pwm_a(&mut self) {
            self.reg
                .reg_block()
                .ctrl()
                .modify(|_, w| unsafe { w.status_sel().bits(StatusSelPwm::PwmA as u8) });
        }

        #[inline]
        fn enable_pwm_b(&mut self) {
            self.reg
                .reg_block()
                .ctrl()
                .modify(|_, w| unsafe { w.status_sel().bits(StatusSelPwm::PwmB as u8) });
        }

        #[inline]
        pub fn get_period(&self) -> Hertz {
            self.pwm_base.current_period
        }

        #[inline]
        pub fn set_period(&mut self, period: impl Into<Hertz>) {
            self.pwm_base.current_period = period.into();
            // Avoid division by 0
            if self.pwm_base.current_period.raw() == 0 {
                return;
            }
            self.pwm_base.current_rst_val =
                self.pwm_base.clock.raw() / self.pwm_base.current_period.raw();
            self.reg
                .reg_block()
                .rst_value()
                .write(|w| unsafe { w.bits(self.pwm_base.current_rst_val) });
        }

        #[inline]
        pub fn disable(&mut self) {
            self.reg
                .reg_block()
                .ctrl()
                .modify(|_, w| w.enable().clear_bit());
        }

        #[inline]
        pub fn enable(&mut self) {
            self.reg
                .reg_block()
                .ctrl()
                .modify(|_, w| w.enable().set_bit());
        }

        #[inline]
        pub fn period(&self) -> Hertz {
            self.pwm_base.current_period
        }

        #[inline(always)]
        pub fn duty(&self) -> u16 {
            self.pwm_base.current_duty
        }
    };
}

macro_rules! pwmb_func {
    () => {
        pub fn pwmb_lower_limit(&self) -> u16 {
            self.pwm_base.current_lower_limit
        }

        pub fn pwmb_upper_limit(&self) -> u16 {
            self.pwm_base.current_duty
        }

        /// Set the lower limit for PWMB
        ///
        /// The PWM signal will be 1 as long as the current RST counter is larger than
        /// the lower limit. For example, with a lower limit of 0.5 and and an upper limit
        /// of 0.7, Only a fixed period between 0.5 * period and 0.7 * period will be in a high
        /// state
        pub fn set_pwmb_lower_limit(&mut self, duty: u16) {
            self.pwm_base.current_lower_limit = duty;
            let pwmb_val: u64 = (self.pwm_base.current_rst_val as u64
                * self.pwm_base.current_lower_limit as u64)
                / DUTY_MAX as u64;
            self.reg
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
            self.pwm_base.current_duty = duty;
            let pwma_val: u64 = (self.pwm_base.current_rst_val as u64
                * self.pwm_base.current_duty as u64)
                / DUTY_MAX as u64;
            self.reg
                .reg_block()
                .pwma_value()
                .write(|w| unsafe { w.bits(pwma_val as u32) });
        }
    };
}

//==================================================================================================
// Strongly typed PWM pin
//==================================================================================================

pub struct PwmPin<Pin: TimPin, Tim: ValidTim, Mode = PwmA> {
    reg: TimAndPinRegister<Pin, Tim>,
    pwm_base: PwmBase,
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
            pwm_base: PwmBase {
                current_duty: 0,
                current_lower_limit: 0,
                current_period: initial_period.into(),
                current_rst_val: 0,
                clock: Tim::clock(clocks),
            },
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

    pub fn release(self) -> (Pin, Tim) {
        self.reg.release()
    }

    pwm_common_func!();
}

impl<Pin: TimPin, Tim: ValidTim> From<PwmPin<Pin, Tim, PwmA>> for PwmPin<Pin, Tim, PwmB>
where
    (Pin, Tim): ValidTimAndPin<Pin, Tim>,
{
    fn from(other: PwmPin<Pin, Tim, PwmA>) -> Self {
        let mut pwmb = Self {
            reg: other.reg,
            pwm_base: other.pwm_base,
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
            pwm_base: other.pwm_base,
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
    reg: TimDynRegister,
    pwm_base: PwmBase,
    pin_id: DynPinId,
    mode: PhantomData<Mode>,
}

impl<PIN: TimPin, TIM: ValidTim> From<PwmPin<PIN, TIM>> for ReducedPwmPin<PwmA> {
    fn from(pwm_pin: PwmPin<PIN, TIM>) -> Self {
        ReducedPwmPin {
            reg: TimDynRegister::from(pwm_pin.reg),
            pwm_base: pwm_pin.pwm_base,
            pin_id: PIN::DYN,
            mode: PhantomData,
        }
    }
}

impl<MODE> ReducedPwmPin<MODE> {
    pwm_common_func!();
}

impl From<ReducedPwmPin<PwmA>> for ReducedPwmPin<PwmB> {
    fn from(other: ReducedPwmPin<PwmA>) -> Self {
        let mut pwmb = Self {
            reg: other.reg,
            pwm_base: other.pwm_base,
            pin_id: other.pin_id,
            mode: PhantomData,
        };
        pwmb.enable_pwm_b();
        pwmb
    }
}

impl From<ReducedPwmPin<PwmB>> for ReducedPwmPin<PwmA> {
    fn from(other: ReducedPwmPin<PwmB>) -> Self {
        let mut pwmb = Self {
            reg: other.reg,
            pwm_base: other.pwm_base,
            pin_id: other.pin_id,
            mode: PhantomData,
        };
        pwmb.enable_pwm_a();
        pwmb
    }
}

//==================================================================================================
// PWMB implementations
//==================================================================================================

impl<PIN: TimPin, TIM: ValidTim> PwmPin<PIN, TIM, PwmB>
where
    (PIN, TIM): ValidTimAndPin<PIN, TIM>,
{
    pwmb_func!();
}

impl ReducedPwmPin<PwmB> {
    pwmb_func!();
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
        self.pwm_base.current_duty = duty;
        let pwma_val: u64 = (self.pwm_base.current_rst_val as u64
            * (DUTY_MAX as u64 - self.pwm_base.current_duty as u64))
            / DUTY_MAX as u64;
        self.reg
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
        self.pwm_base.current_duty = duty;
        let pwma_val: u64 = (self.pwm_base.current_rst_val as u64
            * (DUTY_MAX as u64 - self.pwm_base.current_duty as u64))
            / DUTY_MAX as u64;
        self.reg
            .reg_block()
            .pwma_value()
            .write(|w| unsafe { w.bits(pwma_val as u32) });
        Ok(())
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
