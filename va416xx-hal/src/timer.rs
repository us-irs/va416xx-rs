//! API for the TIM peripherals
//!
//! ## Examples
//!
//! - [Timer MS and Second Tick Example](https://github.com/us-irs/va416xx-rs/blob/main/examples/simple/examples/timer-ticks.rs)
use core::cell::Cell;

use cortex_m::interrupt::Mutex;

use crate::clock::Clocks;
use crate::gpio::{
    AltFunc1, AltFunc2, AltFunc3, DynPinId, Pin, PinId, PA0, PA1, PA10, PA11, PA12, PA13, PA14,
    PA15, PA2, PA3, PA4, PA5, PA6, PA7, PB0, PB1, PB10, PB11, PB12, PB13, PB14, PB15, PB2, PB3,
    PB4, PB5, PB6, PB7, PB8, PB9, PC0, PC1, PD0, PD1, PD10, PD11, PD12, PD13, PD14, PD15, PD2, PD3,
    PD4, PD5, PD6, PD7, PD8, PD9, PE0, PE1, PE10, PE11, PE12, PE13, PE14, PE15, PE2, PE3, PE4, PE5,
    PE6, PE7, PE8, PE9, PF0, PF1, PF10, PF11, PF12, PF13, PF14, PF15, PF2, PF3, PF4, PF5, PF6, PF7,
    PF8, PF9, PG0, PG1, PG2, PG3, PG6,
};
use crate::time::Hertz;
use crate::typelevel::Sealed;
use crate::{disable_interrupt, prelude::*};
use crate::{enable_interrupt, pac};

pub static MS_COUNTER: Mutex<Cell<u32>> = Mutex::new(Cell::new(0));

//==================================================================================================
// Defintions
//==================================================================================================

/// Interrupt events
//pub enum Event {
/// Timer timed out / count down ended
//TimeOut,
//}

#[derive(Default, Debug, PartialEq, Eq, Copy, Clone)]
pub struct CascadeCtrl {
    /// Enable Cascade 0 signal active as a requirement for counting
    pub enb_start_src_csd0: bool,
    /// Invert Cascade 0, making it active low
    pub inv_csd0: bool,
    /// Enable Cascade 1 signal active as a requirement for counting
    pub enb_start_src_csd1: bool,
    /// Invert Cascade 1, making it active low
    pub inv_csd1: bool,
    /// Specify required operation if both Cascade 0 and Cascade 1 are active.
    /// 0 is a logical AND of both cascade signals, 1 is a logical OR
    pub dual_csd_op: bool,
    /// Enable trigger mode for Cascade 0. In trigger mode, couting will start with the selected
    /// cascade signal active, but once the counter is active, cascade control will be ignored
    pub trg_csd0: bool,
    /// Trigger mode, identical to [`trg_csd0`](CascadeCtrl) but for Cascade 1
    pub trg_csd1: bool,
    /// Enable Cascade 2 signal active as a requirement to stop counting. This mode is similar
    /// to the REQ_STOP control bit, but signalled by a Cascade source
    pub enb_stop_src_csd2: bool,
    /// Invert Cascade 2, making it active low
    pub inv_csd2: bool,
    /// The counter is automatically disabled if the corresponding Cascade 2 level-sensitive input
    /// souce is active when the count reaches 0. If the counter is not 0, the cascade control is
    /// ignored
    pub trg_csd2: bool,
}

#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CascadeSel {
    Sel0 = 0,
    Sel1 = 1,
    Sel2 = 2,
}

#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct InvalidCascadeSourceId;

#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CascadeSource {
    PortA(u8),
    PortB(u8),
    PortC(u8),
    PortD(u8),
    PortE(u8),
    Tim(u8),
    TxEv,
    AdcIrq,
    RomSbe,
    RomMbe,
    Ram0Sbe,
    Ram0Mbe,
    Ram1Sbe,
    Ram2Mbe,
    WdogIrq,
}

impl CascadeSource {
    fn id(&self) -> Result<u8, InvalidCascadeSourceId> {
        let port_check = |base: u8, id: u8| {
            if id > 15 {
                return Err(InvalidCascadeSourceId);
            }
            Ok(base + id)
        };
        match self {
            CascadeSource::PortA(id) => port_check(0, *id),
            CascadeSource::PortB(id) => port_check(16, *id),
            CascadeSource::PortC(id) => port_check(32, *id),
            CascadeSource::PortD(id) => port_check(48, *id),
            CascadeSource::PortE(id) => port_check(65, *id),
            CascadeSource::Tim(id) => {
                if *id > 23 {
                    return Err(InvalidCascadeSourceId);
                }
                Ok(80 + id)
            }
            CascadeSource::TxEv => Ok(104),
            CascadeSource::AdcIrq => Ok(105),
            CascadeSource::RomSbe => Ok(106),
            CascadeSource::RomMbe => Ok(106),
            CascadeSource::Ram0Sbe => Ok(108),
            CascadeSource::Ram0Mbe => Ok(109),
            CascadeSource::Ram1Sbe => Ok(110),
            CascadeSource::Ram2Mbe => Ok(111),
            CascadeSource::WdogIrq => Ok(112),
        }
    }
}

//==================================================================================================
// Valid TIM and PIN combinations
//==================================================================================================

pub trait TimPin {
    const DYN: DynPinId;
}

pub trait ValidTim {
    // TIM ID ranging from 0 to 23 for 24 TIM peripherals
    const TIM_ID: u8;
    const IRQ: pac::Interrupt;

    fn clock(clocks: &Clocks) -> Hertz {
        if Self::TIM_ID <= 15 {
            clocks.apb1()
        } else {
            clocks.apb2()
        }
    }
}

macro_rules! tim_markers {
    (
        $(
            ($TimX:path, $id:expr, $Irq:path),
        )+
    ) => {
        $(
            impl ValidTim for $TimX {
                const TIM_ID: u8 = $id;
                const IRQ: pac::Interrupt = $Irq;
            }
        )+
    };
}

tim_markers!(
    (pac::Tim0, 0, pac::Interrupt::TIM0),
    (pac::Tim1, 1, pac::Interrupt::TIM1),
    (pac::Tim2, 2, pac::Interrupt::TIM2),
    (pac::Tim3, 3, pac::Interrupt::TIM3),
    (pac::Tim4, 4, pac::Interrupt::TIM4),
    (pac::Tim5, 5, pac::Interrupt::TIM5),
    (pac::Tim6, 6, pac::Interrupt::TIM6),
    (pac::Tim7, 7, pac::Interrupt::TIM7),
    (pac::Tim8, 8, pac::Interrupt::TIM8),
    (pac::Tim9, 9, pac::Interrupt::TIM9),
    (pac::Tim10, 10, pac::Interrupt::TIM10),
    (pac::Tim11, 11, pac::Interrupt::TIM11),
    (pac::Tim12, 12, pac::Interrupt::TIM12),
    (pac::Tim13, 13, pac::Interrupt::TIM13),
    (pac::Tim14, 14, pac::Interrupt::TIM14),
    (pac::Tim15, 15, pac::Interrupt::TIM15),
    (pac::Tim16, 16, pac::Interrupt::TIM16),
    (pac::Tim17, 17, pac::Interrupt::TIM17),
    (pac::Tim18, 18, pac::Interrupt::TIM18),
    (pac::Tim19, 19, pac::Interrupt::TIM19),
    (pac::Tim20, 20, pac::Interrupt::TIM20),
    (pac::Tim21, 21, pac::Interrupt::TIM21),
    (pac::Tim22, 22, pac::Interrupt::TIM22),
    (pac::Tim23, 23, pac::Interrupt::TIM23),
);

pub trait ValidTimAndPin<Pin: TimPin, Tim: ValidTim>: Sealed {}

macro_rules! valid_pin_and_tims {
    (
        $(
            ($PinX:ident, $AltFunc:ident, $TimX:path),
        )+
    ) => {
        $(
            impl TimPin for Pin<$PinX, $AltFunc>
            where
                $PinX: PinId,
            {
                const DYN: DynPinId = $PinX::DYN;
            }

            impl<
                PinInstance: TimPin,
                Tim: ValidTim
            > ValidTimAndPin<PinInstance, Tim> for (Pin<$PinX, $AltFunc>, $TimX)
            where
                Pin<$PinX, $AltFunc>: TimPin,
                $PinX: PinId,
            {
            }

            impl Sealed for (Pin<$PinX, $AltFunc>, $TimX) {}
        )+
    };
}

valid_pin_and_tims!(
    (PA0, AltFunc1, pac::Tim0),
    (PA1, AltFunc1, pac::Tim1),
    (PA2, AltFunc1, pac::Tim2),
    (PA3, AltFunc1, pac::Tim3),
    (PA4, AltFunc1, pac::Tim4),
    (PA5, AltFunc1, pac::Tim5),
    (PA6, AltFunc1, pac::Tim6),
    (PA7, AltFunc1, pac::Tim7),
    (PA10, AltFunc2, pac::Tim23),
    (PA11, AltFunc2, pac::Tim22),
    (PA12, AltFunc2, pac::Tim21),
    (PA13, AltFunc2, pac::Tim20),
    (PA14, AltFunc2, pac::Tim19),
    (PA15, AltFunc2, pac::Tim18),
    (PB0, AltFunc2, pac::Tim17),
    (PB1, AltFunc2, pac::Tim16),
    (PB2, AltFunc2, pac::Tim15),
    (PB3, AltFunc2, pac::Tim14),
    (PB4, AltFunc2, pac::Tim13),
    (PB5, AltFunc2, pac::Tim12),
    (PB6, AltFunc2, pac::Tim11),
    (PB7, AltFunc2, pac::Tim10),
    (PB8, AltFunc2, pac::Tim9),
    (PB9, AltFunc2, pac::Tim8),
    (PB10, AltFunc2, pac::Tim7),
    (PB11, AltFunc2, pac::Tim6),
    (PB12, AltFunc2, pac::Tim5),
    (PB13, AltFunc2, pac::Tim4),
    (PB14, AltFunc2, pac::Tim3),
    (PB15, AltFunc2, pac::Tim2),
    (PC0, AltFunc2, pac::Tim1),
    (PC1, AltFunc2, pac::Tim0),
    (PD0, AltFunc2, pac::Tim0),
    (PD1, AltFunc2, pac::Tim1),
    (PD2, AltFunc2, pac::Tim2),
    (PD3, AltFunc2, pac::Tim3),
    (PD4, AltFunc2, pac::Tim4),
    (PD5, AltFunc2, pac::Tim5),
    (PD6, AltFunc2, pac::Tim6),
    (PD7, AltFunc2, pac::Tim7),
    (PD8, AltFunc2, pac::Tim8),
    (PD9, AltFunc2, pac::Tim9),
    (PD10, AltFunc2, pac::Tim10),
    (PD11, AltFunc2, pac::Tim11),
    (PD12, AltFunc2, pac::Tim12),
    (PD13, AltFunc2, pac::Tim13),
    (PD14, AltFunc2, pac::Tim14),
    (PD15, AltFunc2, pac::Tim15),
    (PE0, AltFunc2, pac::Tim16),
    (PE1, AltFunc2, pac::Tim17),
    (PE2, AltFunc2, pac::Tim18),
    (PE3, AltFunc2, pac::Tim19),
    (PE4, AltFunc2, pac::Tim20),
    (PE5, AltFunc2, pac::Tim21),
    (PE6, AltFunc2, pac::Tim22),
    (PE7, AltFunc2, pac::Tim23),
    (PE8, AltFunc3, pac::Tim16),
    (PE9, AltFunc3, pac::Tim17),
    (PE10, AltFunc3, pac::Tim18),
    (PE11, AltFunc3, pac::Tim19),
    (PE12, AltFunc3, pac::Tim20),
    (PE13, AltFunc3, pac::Tim21),
    (PE14, AltFunc3, pac::Tim22),
    (PE15, AltFunc3, pac::Tim23),
    (PF0, AltFunc3, pac::Tim0),
    (PF1, AltFunc3, pac::Tim1),
    (PF2, AltFunc3, pac::Tim2),
    (PF3, AltFunc3, pac::Tim3),
    (PF4, AltFunc3, pac::Tim4),
    (PF5, AltFunc3, pac::Tim5),
    (PF6, AltFunc3, pac::Tim6),
    (PF7, AltFunc3, pac::Tim7),
    (PF8, AltFunc3, pac::Tim8),
    (PF9, AltFunc3, pac::Tim9),
    (PF10, AltFunc3, pac::Tim10),
    (PF11, AltFunc3, pac::Tim11),
    (PF12, AltFunc3, pac::Tim12),
    (PF13, AltFunc2, pac::Tim19),
    (PF14, AltFunc2, pac::Tim20),
    (PF15, AltFunc2, pac::Tim21),
    (PG0, AltFunc2, pac::Tim22),
    (PG1, AltFunc2, pac::Tim23),
    (PG2, AltFunc1, pac::Tim9),
    (PG3, AltFunc1, pac::Tim10),
    (PG6, AltFunc1, pac::Tim12),
);

//==================================================================================================
// Register Interface for TIM registers and TIM pins
//==================================================================================================

/// Clear the reset bit of the TIM, holding it in reset
///
/// # Safety
///
/// Only the bit related to the corresponding TIM peripheral is modified
#[inline]
fn assert_tim_reset(syscfg: &mut pac::Sysconfig, tim_id: u8) {
    syscfg
        .tim_reset()
        .modify(|r, w| unsafe { w.bits(r.bits() & !(1 << tim_id as u32)) })
}

#[inline]
fn deassert_tim_reset(syscfg: &mut pac::Sysconfig, tim_id: u8) {
    syscfg
        .tim_reset()
        .modify(|r, w| unsafe { w.bits(r.bits() | (1 << tim_id as u32)) })
}

pub type TimRegBlock = pac::tim0::RegisterBlock;

/// Register interface.
///
/// This interface provides valid TIM pins a way to access their corresponding TIM
/// registers
///
/// # Safety
///
/// Users should only implement the [`tim_id`] function. No default function
/// implementations should be overridden. The implementing type must also have
/// "control" over the corresponding pin ID, i.e. it must guarantee that a each
/// pin ID is a singleton.
pub(super) unsafe trait TimRegInterface {
    fn tim_id(&self) -> u8;

    const PORT_BASE: *const pac::tim0::RegisterBlock = pac::Tim0::ptr() as *const _;

    /// All 24 TIM blocks are identical. This helper functions returns the correct
    /// memory mapped peripheral depending on the TIM ID.
    #[inline(always)]
    fn reg(&self) -> &TimRegBlock {
        unsafe { &*Self::PORT_BASE.offset(self.tim_id() as isize) }
    }

    #[inline(always)]
    fn mask_32(&self) -> u32 {
        1 << self.tim_id()
    }

    /// Clear the reset bit of the TIM, holding it in reset
    ///
    /// # Safety
    ///
    /// Only the bit related to the corresponding TIM peripheral is modified
    #[inline]
    #[allow(dead_code)]
    fn assert_tim_reset(&self, syscfg: &mut pac::Sysconfig) {
        assert_tim_reset(syscfg, self.tim_id());
    }

    #[inline]
    #[allow(dead_code)]
    fn deassert_time_reset(&self, syscfg: &mut pac::Sysconfig) {
        deassert_tim_reset(syscfg, self.tim_id());
    }
}

/// Provide a safe register interface for [`ValidTimAndPin`]s
///
/// This `struct` takes ownership of a [`ValidTimAndPin`] and provides an API to
/// access the corresponding registers.
pub(super) struct TimAndPinRegister<Pin: TimPin, Tim: ValidTim> {
    pin: Pin,
    tim: Tim,
}

pub(super) struct TimRegister<TIM: ValidTim> {
    tim: TIM,
}

impl<TIM: ValidTim> TimRegister<TIM> {
    #[inline]
    pub(super) unsafe fn new(tim: TIM) -> Self {
        TimRegister { tim }
    }

    pub(super) fn release(self) -> TIM {
        self.tim
    }
}

unsafe impl<Tim: ValidTim> TimRegInterface for TimRegister<Tim> {
    #[inline(always)]
    fn tim_id(&self) -> u8 {
        Tim::TIM_ID
    }
}

impl<Pin: TimPin, Tim: ValidTim> TimAndPinRegister<Pin, Tim>
where
    (Pin, Tim): ValidTimAndPin<Pin, Tim>,
{
    #[inline]
    pub(super) unsafe fn new(pin: Pin, tim: Tim) -> Self {
        TimAndPinRegister { pin, tim }
    }

    pub(super) fn release(self) -> (Pin, Tim) {
        (self.pin, self.tim)
    }
}

unsafe impl<Pin: TimPin, Tim: ValidTim> TimRegInterface for TimAndPinRegister<Pin, Tim> {
    #[inline(always)]
    fn tim_id(&self) -> u8 {
        Tim::TIM_ID
    }
}

pub(super) struct TimDynRegister {
    tim_id: u8,
    #[allow(dead_code)]
    pin_id: DynPinId,
}

impl<Pin: TimPin, Tim: ValidTim> From<TimAndPinRegister<Pin, Tim>> for TimDynRegister {
    fn from(_reg: TimAndPinRegister<Pin, Tim>) -> Self {
        Self {
            tim_id: Tim::TIM_ID,
            pin_id: Pin::DYN,
        }
    }
}

unsafe impl TimRegInterface for TimDynRegister {
    #[inline(always)]
    fn tim_id(&self) -> u8 {
        self.tim_id
    }
}

//==================================================================================================
// Timers
//==================================================================================================

/// Hardware timers.
///
/// These timers also implement the [embedded_hal::delay::DelayNs] trait and can be used to delay
/// with a higher resolution compared to the Cortex-M systick delays.
pub struct CountdownTimer<TIM: ValidTim> {
    tim: TimRegister<TIM>,
    curr_freq: Hertz,
    clock: Hertz,
    rst_val: u32,
    last_cnt: u32,
    listening: bool,
}

#[inline]
fn enable_tim_clk(syscfg: &mut pac::Sysconfig, idx: u8) {
    syscfg
        .tim_clk_enable()
        .modify(|r, w| unsafe { w.bits(r.bits() | (1 << idx)) });
}

unsafe impl<TIM: ValidTim> TimRegInterface for CountdownTimer<TIM> {
    #[inline]
    fn tim_id(&self) -> u8 {
        TIM::TIM_ID
    }
}

impl<Tim: ValidTim> CountdownTimer<Tim> {
    /// Create a new countdown timer, but does not start it.
    ///
    /// You can use [Self::start] to start the countdown timer, and you may optionally call
    /// [Self::listen] to enable interrupts for the TIM peripheral as well.
    pub fn new(syscfg: &mut pac::Sysconfig, tim: Tim, clocks: &Clocks) -> Self {
        enable_tim_clk(syscfg, Tim::TIM_ID);
        assert_tim_reset(syscfg, Tim::TIM_ID);
        cortex_m::asm::nop();
        cortex_m::asm::nop();
        deassert_tim_reset(syscfg, Tim::TIM_ID);

        CountdownTimer {
            tim: unsafe { TimRegister::new(tim) },
            clock: Tim::clock(clocks),
            rst_val: 0,
            curr_freq: 0_u32.Hz(),
            listening: false,
            last_cnt: 0,
        }
    }

    #[inline]
    pub fn start(&mut self, timeout: impl Into<Hertz>) {
        self.load(timeout);
        self.enable();
    }

    /// Listen for events. Depending on the IRQ configuration, this also activates the IRQ in the
    /// IRQSEL peripheral for the provided interrupt and unmasks the interrupt
    #[inline]
    pub fn listen(&mut self) {
        self.listening = true;
        self.enable_interrupt();
        unsafe { enable_interrupt(Tim::IRQ) }
    }

    /// Return `Ok` if the timer has wrapped. Peripheral will automatically clear the
    /// flag and restart the time if configured correctly
    pub fn wait(&mut self) -> nb::Result<(), void::Void> {
        let cnt = self.tim.reg().cnt_value().read().bits();
        if (cnt > self.last_cnt) || cnt == 0 {
            self.last_cnt = self.rst_val;
            Ok(())
        } else {
            self.last_cnt = cnt;
            Err(nb::Error::WouldBlock)
        }
    }

    #[inline]
    pub fn stop(&mut self) {
        self.tim.reg().ctrl().write(|w| w.enable().clear_bit());
    }

    #[inline]
    pub fn unlisten(&mut self) {
        self.listening = true;
        self.disable_interrupt();
        disable_interrupt(Tim::IRQ);
    }

    #[inline(always)]
    pub fn enable_interrupt(&mut self) {
        self.tim.reg().ctrl().modify(|_, w| w.irq_enb().set_bit());
    }

    #[inline(always)]
    pub fn disable_interrupt(&mut self) {
        self.tim.reg().ctrl().modify(|_, w| w.irq_enb().clear_bit());
    }

    #[inline]
    pub fn release(self, syscfg: &mut pac::Sysconfig) -> Tim {
        self.tim.reg().ctrl().write(|w| w.enable().clear_bit());
        syscfg
            .tim_clk_enable()
            .modify(|r, w| unsafe { w.bits(r.bits() & !(1 << Tim::TIM_ID)) });
        self.tim.release()
    }

    /// Load the count down timer with a timeout but do not start it.
    pub fn load(&mut self, timeout: impl Into<Hertz>) {
        self.tim.reg().ctrl().modify(|_, w| w.enable().clear_bit());
        self.curr_freq = timeout.into();
        self.rst_val = self.clock.raw() / self.curr_freq.raw();
        self.set_reload(self.rst_val);
        self.set_count(0);
    }

    #[inline(always)]
    pub fn set_reload(&mut self, val: u32) {
        self.tim.reg().rst_value().write(|w| unsafe { w.bits(val) });
    }

    #[inline(always)]
    pub fn set_count(&mut self, val: u32) {
        self.tim.reg().cnt_value().write(|w| unsafe { w.bits(val) });
    }

    #[inline(always)]
    pub fn count(&self) -> u32 {
        self.tim.reg().cnt_value().read().bits()
    }

    #[inline(always)]
    pub fn enable(&mut self) {
        self.tim.reg().ctrl().modify(|_, w| w.enable().set_bit());
    }

    #[inline(always)]
    pub fn disable(&mut self) {
        self.tim.reg().ctrl().modify(|_, w| w.enable().clear_bit());
    }

    /// Disable the counter, setting both enable and active bit to 0
    #[inline]
    pub fn auto_disable(self, enable: bool) -> Self {
        if enable {
            self.tim
                .reg()
                .ctrl()
                .modify(|_, w| w.auto_disable().set_bit());
        } else {
            self.tim
                .reg()
                .ctrl()
                .modify(|_, w| w.auto_disable().clear_bit());
        }
        self
    }

    /// This option only applies when the Auto-Disable functionality is 0.
    ///
    /// The active bit is changed to 0 when count reaches 0, but the counter stays
    /// enabled. When Auto-Disable is 1, Auto-Deactivate is implied
    #[inline]
    pub fn auto_deactivate(self, enable: bool) -> Self {
        if enable {
            self.tim
                .reg()
                .ctrl()
                .modify(|_, w| w.auto_deactivate().set_bit());
        } else {
            self.tim
                .reg()
                .ctrl()
                .modify(|_, w| w.auto_deactivate().clear_bit());
        }
        self
    }

    /// Configure the cascade parameters
    #[inline]
    pub fn cascade_control(&mut self, ctrl: CascadeCtrl) {
        self.tim.reg().csd_ctrl().write(|w| {
            w.csden0().bit(ctrl.enb_start_src_csd0);
            w.csdinv0().bit(ctrl.inv_csd0);
            w.csden1().bit(ctrl.enb_start_src_csd1);
            w.csdinv1().bit(ctrl.inv_csd1);
            w.dcasop().bit(ctrl.dual_csd_op);
            w.csdtrg0().bit(ctrl.trg_csd0);
            w.csdtrg1().bit(ctrl.trg_csd1);
            w.csden2().bit(ctrl.enb_stop_src_csd2);
            w.csdinv2().bit(ctrl.inv_csd2);
            w.csdtrg2().bit(ctrl.trg_csd2)
        });
    }

    #[inline]
    pub fn cascade_0_source(&mut self, src: CascadeSource) -> Result<(), InvalidCascadeSourceId> {
        let id = src.id()?;
        self.tim
            .reg()
            .cascade0()
            .write(|w| unsafe { w.cassel().bits(id) });
        Ok(())
    }

    #[inline]
    pub fn cascade_1_source(&mut self, src: CascadeSource) -> Result<(), InvalidCascadeSourceId> {
        let id = src.id()?;
        self.tim
            .reg()
            .cascade1()
            .write(|w| unsafe { w.cassel().bits(id) });
        Ok(())
    }

    #[inline]
    pub fn cascade_2_source(&mut self, src: CascadeSource) -> Result<(), InvalidCascadeSourceId> {
        let id = src.id()?;
        self.tim
            .reg()
            .cascade2()
            .write(|w| unsafe { w.cassel().bits(id) });
        Ok(())
    }

    #[inline]
    pub fn curr_freq(&self) -> Hertz {
        self.curr_freq
    }

    #[inline]
    pub fn listening(&self) -> bool {
        self.listening
    }
}

impl<Tim: ValidTim> embedded_hal::delay::DelayNs for CountdownTimer<Tim> {
    fn delay_ns(&mut self, ns: u32) {
        let ticks = (u64::from(ns)) * (u64::from(self.clock.raw())) / 1_000_000_000;

        let full_cycles = ticks >> 32;
        let mut last_count;
        let mut new_count;
        if full_cycles > 0 {
            self.set_reload(u32::MAX);
            self.set_count(u32::MAX);
            self.enable();

            for _ in 0..full_cycles {
                // Always ensure that both values are the same at the start.
                new_count = self.count();
                last_count = new_count;
                loop {
                    new_count = self.count();
                    if new_count == 0 {
                        // Wait till timer has wrapped.
                        while self.count() == 0 {
                            cortex_m::asm::nop()
                        }
                        break;
                    }
                    // Timer has definitely wrapped.
                    if new_count > last_count {
                        break;
                    }
                    last_count = new_count;
                }
            }
        }
        let ticks = (ticks & u32::MAX as u64) as u32;
        self.disable();
        if ticks > 1 {
            self.set_reload(ticks);
            self.set_count(ticks);
            self.enable();
            last_count = ticks;

            loop {
                new_count = self.count();
                if new_count == 0 || (new_count > last_count) {
                    break;
                }
                last_count = new_count;
            }
        }

        self.disable();
    }
}

//==================================================================================================
// MS tick implementations
//==================================================================================================

// Set up a millisecond timer on TIM0. Please note that the user still has to provide an IRQ handler
// which should call [default_ms_irq_handler].
pub fn set_up_ms_tick<Tim: ValidTim>(
    sys_cfg: &mut pac::Sysconfig,
    tim: Tim,
    clocks: &Clocks,
) -> CountdownTimer<Tim> {
    let mut ms_timer = CountdownTimer::new(sys_cfg, tim, clocks);
    ms_timer.listen();
    ms_timer.start(1000.Hz());
    ms_timer
}

/// This function can be called in a specified interrupt handler to increment
/// the MS counter
pub fn default_ms_irq_handler() {
    cortex_m::interrupt::free(|cs| {
        let mut ms = MS_COUNTER.borrow(cs).get();
        ms += 1;
        MS_COUNTER.borrow(cs).set(ms);
    });
}

/// Get the current MS tick count
pub fn get_ms_ticks() -> u32 {
    cortex_m::interrupt::free(|cs| MS_COUNTER.borrow(cs).get())
}

pub struct DelayMs<Tim: ValidTim = pac::Tim0>(CountdownTimer<Tim>);

impl<Tim: ValidTim> DelayMs<Tim> {
    pub fn new(timer: CountdownTimer<Tim>) -> Option<Self> {
        if timer.curr_freq() != Hertz::from_raw(1000) || !timer.listening() {
            return None;
        }
        Some(Self(timer))
    }
}

/// This assumes that the user has already set up a MS tick timer with [set_up_ms_tick]
impl embedded_hal::delay::DelayNs for DelayMs {
    fn delay_ns(&mut self, ns: u32) {
        let ns_as_ms = ns / 1_000_000;
        if self.0.curr_freq() != Hertz::from_raw(1000) || !self.0.listening() {
            return;
        }
        let start_time = get_ms_ticks();
        while get_ms_ticks() - start_time < ns_as_ms {
            cortex_m::asm::nop();
        }
    }
}
