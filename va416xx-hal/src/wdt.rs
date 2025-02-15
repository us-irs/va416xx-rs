//! # API for the Watchdog peripheral
//!
//! ## Examples
//!
//! - [Watchdog simple example](https://egit.irs.uni-stuttgart.de/rust/va416xx-rs/src/branch/main/examples/simple/examples/wdt.rs)
use crate::time::Hertz;
use crate::{
    clock::{Clocks, PeripheralSelect},
    pac,
    prelude::SyscfgExt,
};
use crate::{disable_nvic_interrupt, enable_nvic_interrupt};

pub const WDT_UNLOCK_VALUE: u32 = 0x1ACC_E551;

/// Watchdog peripheral driver.
pub struct Wdt {
    clock_freq: Hertz,
    wdt: pac::WatchDog,
}

/// Type alias for backwards compatibility
#[deprecated(since = "0.2.0", note = "Please use `Wdt` instead")]
pub type WdtController = Wdt;

/// Enable the watchdog interrupt
///
/// # Safety
///
/// This function is `unsafe` because it can break mask-based critical sections.
#[inline]
pub unsafe fn enable_wdt_interrupts() {
    enable_nvic_interrupt(pac::Interrupt::WATCHDOG)
}

#[inline]
pub fn disable_wdt_interrupts() {
    disable_nvic_interrupt(pac::Interrupt::WATCHDOG)
}

impl Wdt {
    pub fn new(
        syscfg: &mut pac::Sysconfig,
        wdt: pac::WatchDog,
        clocks: &Clocks,
        wdt_freq_ms: u32,
    ) -> Self {
        Self::start(syscfg, wdt, clocks, wdt_freq_ms)
    }

    pub fn start(
        syscfg: &mut pac::Sysconfig,
        wdt: pac::WatchDog,
        clocks: &Clocks,
        wdt_freq_ms: u32,
    ) -> Self {
        syscfg.enable_peripheral_clock(PeripheralSelect::Watchdog);
        syscfg.assert_periph_reset_for_two_cycles(PeripheralSelect::Watchdog);

        let wdt_clock = clocks.apb2();
        let mut wdt_ctrl = Self {
            clock_freq: wdt_clock,
            wdt,
        };
        wdt_ctrl.set_freq(wdt_freq_ms);
        wdt_ctrl.wdt.wdogcontrol().write(|w| w.inten().set_bit());
        wdt_ctrl.feed();
        // Unmask the watchdog interrupt
        unsafe {
            enable_wdt_interrupts();
        }
        wdt_ctrl
    }

    #[inline]
    pub fn set_freq(&mut self, freq_ms: u32) {
        let counter = (self.clock_freq.raw() / 1000) * freq_ms;
        self.wdt.wdogload().write(|w| unsafe { w.bits(counter) });
    }

    #[inline]
    pub fn disable_reset(&mut self) {
        self.wdt.wdogcontrol().modify(|_, w| w.resen().clear_bit());
    }

    #[inline]
    pub fn enable_reset(&mut self) {
        self.wdt.wdogcontrol().modify(|_, w| w.resen().set_bit());
    }

    #[inline]
    pub fn counter(&self) -> u32 {
        self.wdt.wdogvalue().read().bits()
    }

    #[inline]
    pub fn feed(&self) {
        self.wdt.wdogintclr().write(|w| unsafe { w.bits(1) });
    }

    #[inline]
    pub fn lock(&self) {
        self.wdt.wdoglock().write(|w| unsafe { w.bits(0) });
    }

    #[inline]
    pub fn unlock(&self) {
        self.wdt
            .wdoglock()
            .write(|w| unsafe { w.bits(WDT_UNLOCK_VALUE) });
    }

    #[inline]
    pub fn is_locked(&self) -> bool {
        self.wdt.wdogload().read().bits() == 1
    }
}
