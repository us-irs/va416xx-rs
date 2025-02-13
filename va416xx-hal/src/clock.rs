//! API for using the [crate::pac::Clkgen] peripheral.
//!
//! It also includes functionality to enable the peripheral clocks.
//! Calling [ClkgenExt::constrain] on the [crate::pac::Clkgen] peripheral generates the
//! [ClkgenCfgr] structure which can be used to configure and set up the clock.
//!
//! Calling [ClkgenCfgr::freeze] returns the frozen clock configuration inside the [Clocks]
//! structure. This structure can also be used to configure other structures provided by this HAL.
//!
//! # Examples
//!
//! - [UART example on the PEB1 board](https://egit.irs.uni-stuttgart.de/rust/va416xx-rs/src/branch/main/examples/simple/examples/uart.rs)
#[cfg(not(feature = "va41628"))]
use crate::adc::ADC_MAX_CLK;
use crate::pac;

use crate::time::Hertz;

pub const HBO_FREQ: Hertz = Hertz::from_raw(20_000_000);
pub const XTAL_OSC_TSTART_MS: u32 = 15;

#[derive(Copy, Clone, PartialEq)]
pub enum PeripheralSelect {
    Spi0 = 0,
    Spi1 = 1,
    Spi2 = 2,
    Spi3 = 3,
    Uart0 = 4,
    Uart1 = 5,
    Uart2 = 6,
    I2c0 = 7,
    I2c1 = 8,
    I2c2 = 9,
    Can0 = 10,
    Can1 = 11,
    Rng = 12,
    Adc = 13,
    Dac = 14,
    Dma = 15,
    Ebi = 16,
    Eth = 17,
    Spw = 18,
    Clkgen = 19,
    IrqRouter = 20,
    IoConfig = 21,
    Utility = 22,
    Watchdog = 23,
    PortA = 24,
    PortB = 25,
    PortC = 26,
    PortD = 27,
    PortE = 28,
    PortF = 29,
    PortG = 30,
}

pub type PeripheralClock = PeripheralSelect;

#[derive(Debug, PartialEq, Eq)]
pub enum FilterClkSel {
    SysClk = 0,
    Clk1 = 1,
    Clk2 = 2,
    Clk3 = 3,
    Clk4 = 4,
    Clk5 = 5,
    Clk6 = 6,
    Clk7 = 7,
}

#[inline(always)]
pub fn enable_peripheral_clock(syscfg: &mut pac::Sysconfig, clock: PeripheralSelect) {
    syscfg
        .peripheral_clk_enable()
        .modify(|r, w| unsafe { w.bits(r.bits() | (1 << clock as u8)) });
}

#[inline(always)]
pub fn disable_peripheral_clock(syscfg: &mut pac::Sysconfig, clock: PeripheralSelect) {
    syscfg
        .peripheral_clk_enable()
        .modify(|r, w| unsafe { w.bits(r.bits() & !(1 << clock as u8)) });
}

#[inline(always)]
pub fn assert_periph_reset(syscfg: &mut pac::Sysconfig, periph: PeripheralSelect) {
    syscfg
        .peripheral_reset()
        .modify(|r, w| unsafe { w.bits(r.bits() & !(1 << periph as u8)) });
}

#[inline(always)]
pub fn deassert_periph_reset(syscfg: &mut pac::Sysconfig, periph: PeripheralSelect) {
    syscfg
        .peripheral_reset()
        .modify(|r, w| unsafe { w.bits(r.bits() | (1 << periph as u8)) });
}

#[inline(always)]
fn assert_periph_reset_for_two_cycles(syscfg: &mut pac::Sysconfig, periph: PeripheralSelect) {
    assert_periph_reset(syscfg, periph);
    cortex_m::asm::nop();
    cortex_m::asm::nop();
    deassert_periph_reset(syscfg, periph);
}

pub trait SyscfgExt {
    fn enable_peripheral_clock(&mut self, clock: PeripheralClock);

    fn disable_peripheral_clock(&mut self, clock: PeripheralClock);

    fn assert_periph_reset(&mut self, periph: PeripheralSelect);

    fn deassert_periph_reset(&mut self, periph: PeripheralSelect);

    fn assert_periph_reset_for_two_cycles(&mut self, periph: PeripheralSelect);
}

impl SyscfgExt for pac::Sysconfig {
    #[inline(always)]
    fn enable_peripheral_clock(&mut self, clock: PeripheralClock) {
        enable_peripheral_clock(self, clock)
    }

    #[inline(always)]
    fn disable_peripheral_clock(&mut self, clock: PeripheralClock) {
        disable_peripheral_clock(self, clock)
    }

    #[inline(always)]
    fn assert_periph_reset(&mut self, clock: PeripheralSelect) {
        assert_periph_reset(self, clock)
    }

    #[inline(always)]
    fn deassert_periph_reset(&mut self, clock: PeripheralSelect) {
        deassert_periph_reset(self, clock)
    }

    #[inline(always)]
    fn assert_periph_reset_for_two_cycles(&mut self, periph: PeripheralSelect) {
        assert_periph_reset_for_two_cycles(self, periph)
    }
}

/// Refer to chapter 8 (p.57) of the programmers guide for detailed information.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClkselSys {
    // Internal Heart-Beat Osciallator. Not tightly controlled (+/-20 %). Not recommended as the regular clock!
    Hbo = 0b00,
    // External clock signal on XTAL_N line, 1-100 MHz
    XtalN = 0b01,
    // Internal Phase-Locked Loop.
    Pll = 0b10,
    // Crystal oscillator amplified, 4-10 MHz.
    XtalOsc = 0b11,
}

/// This selects the input clock to the the CLKGEN peripheral in addition to the HBO clock.
///
/// This can either be a clock connected directly on the XTAL_N line or a chrystal on the XTAL_P
/// line which goes through an oscillator amplifier.
///
/// Refer to chapter 8 (p.57) of the programmers guide for detailed information.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RefClkSel {
    #[default]
    None = 0b00,
    XtalOsc = 0b01,
    XtalN = 0b10,
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClkDivSel {
    #[default]
    Div1 = 0b00,
    Div2 = 0b01,
    Div4 = 0b10,
    Div8 = 0b11,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AdcClkDivSel {
    Div8 = 0b00,
    Div4 = 0b01,
    Div2 = 0b10,
    Div1 = 0b11,
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct PllCfg {
    /// Reference clock divider.
    pub clkr: u8,
    /// Clock divider on feedback path
    pub clkf: u8,
    // Output clock divider.
    pub clkod: u8,
    /// Bandwidth adjustment
    pub bwadj: u8,
}

pub fn clk_after_div(clk: Hertz, div_sel: ClkDivSel) -> Hertz {
    match div_sel {
        ClkDivSel::Div1 => clk,
        ClkDivSel::Div2 => clk / 2,
        ClkDivSel::Div4 => clk / 4,
        ClkDivSel::Div8 => clk / 8,
    }
}

/// Wait for 500 reference clock cycles like specified in the datasheet.
pub fn pll_setup_delay() {
    for _ in 0..500 {
        cortex_m::asm::nop()
    }
}

pub trait ClkgenExt {
    fn constrain(self) -> ClkgenCfgr;
}

impl ClkgenExt for pac::Clkgen {
    fn constrain(self) -> ClkgenCfgr {
        ClkgenCfgr {
            source_clk: None,
            ref_clk_sel: RefClkSel::None,
            clksel_sys: ClkselSys::Hbo,
            clk_div_sel: ClkDivSel::Div1,
            clk_lost_detection: false,
            pll_lock_lost_detection: false,
            pll_cfg: None,
            clkgen: self,
        }
    }
}

pub struct ClkgenCfgr {
    ref_clk_sel: RefClkSel,
    clksel_sys: ClkselSys,
    clk_div_sel: ClkDivSel,
    /// The source clock frequency which is either an external clock connected to XTAL_N, or a
    /// crystal connected to the XTAL_OSC input.
    source_clk: Option<Hertz>,
    pll_cfg: Option<PllCfg>,
    clk_lost_detection: bool,
    /// Feature only works on revision B of the board.
    #[cfg(feature = "revb")]
    pll_lock_lost_detection: bool,
    clkgen: pac::Clkgen,
}

#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ClkSourceFreqNotSet;

#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClkCfgError {
    ClkSourceFreqNotSet,
    PllConfigNotSet,
    PllInitError,
    InconsistentCfg,
}

/// Delays a given amount of milliseconds.
///
/// Taken from the HAL implementation. This implementation is probably not precise and it
/// also blocks!
pub fn hbo_clock_delay_ms(ms: u32) {
    let wdt = unsafe { pac::WatchDog::steal() };
    for _ in 0..ms {
        for _ in 0..10_000 {
            cortex_m::asm::nop();
        }
        wdt.wdogintclr().write(|w| unsafe { w.bits(1) });
    }
}

impl ClkgenCfgr {
    #[inline]
    pub fn source_clk(mut self, src_clk: Hertz) -> Self {
        self.source_clk = Some(src_clk);
        self
    }

    /// This function can be used to utilize the XTAL_N clock input directly without the
    /// oscillator.
    ///
    /// It sets the internal configuration to [ClkselSys::XtalN] and [RefClkSel::XtalN].
    #[inline]
    pub fn xtal_n_clk(mut self) -> Self {
        self.clksel_sys = ClkselSys::XtalN;
        self.ref_clk_sel = RefClkSel::XtalN;
        self
    }

    #[inline]
    pub fn xtal_n_clk_with_src_freq(mut self, src_clk: Hertz) -> Self {
        self = self.xtal_n_clk();
        self.source_clk(src_clk)
    }

    #[inline]
    pub fn clksel_sys(mut self, clksel_sys: ClkselSys) -> Self {
        self.clksel_sys = clksel_sys;
        self
    }

    #[inline]
    pub fn pll_cfg(mut self, pll_cfg: PllCfg) -> Self {
        self.pll_cfg = Some(pll_cfg);
        self
    }

    #[inline]
    pub fn ref_clk_sel(mut self, ref_clk_sel: RefClkSel) -> Self {
        self.ref_clk_sel = ref_clk_sel;
        self
    }

    /// Configures all clocks and return a clock configuration structure containing the final
    /// frozen clocks.
    ///
    /// Internal implementation details: This implementation is based on the HAL implementation
    /// which performs a lot of delays. I do not know if all of those are necessary, but
    /// I am going to be conservative here and assume that the vendor has tested though and
    /// might have had a reason for those, so I am going to keep them. Chances are, this
    /// process only has to be performed once, and it does not matter if it takes a few
    /// microseconds or milliseconds longer.
    pub fn freeze(self, syscfg: &mut pac::Sysconfig) -> Result<Clocks, ClkCfgError> {
        // Sanitize configuration.
        if self.source_clk.is_none() {
            return Err(ClkCfgError::ClkSourceFreqNotSet);
        }
        if self.clksel_sys == ClkselSys::XtalOsc && self.ref_clk_sel != RefClkSel::XtalOsc {
            return Err(ClkCfgError::InconsistentCfg);
        }
        if self.clksel_sys == ClkselSys::XtalN && self.ref_clk_sel != RefClkSel::XtalN {
            return Err(ClkCfgError::InconsistentCfg);
        }
        if self.clksel_sys == ClkselSys::Pll && self.pll_cfg.is_none() {
            return Err(ClkCfgError::PllConfigNotSet);
        }

        syscfg.enable_peripheral_clock(PeripheralSelect::Clkgen);
        let mut final_sysclk = self.source_clk.unwrap();
        // The HAL forces back the HBO clock here with a delay.. Even though this is
        // not stricly necessary when coming from a fresh start, it could be still become relevant
        // later if the clock lost detection mechanism require a re-configuration of the clocks.
        // Therefore, we do it here as well.
        self.clkgen
            .ctrl0()
            .modify(|_, w| unsafe { w.clksel_sys().bits(ClkselSys::Hbo as u8) });
        pll_setup_delay();
        self.clkgen
            .ctrl0()
            .modify(|_, w| unsafe { w.clk_div_sel().bits(ClkDivSel::Div1 as u8) });

        // Set up oscillator and PLL input clock.
        self.clkgen
            .ctrl0()
            .modify(|_, w| unsafe { w.ref_clk_sel().bits(self.ref_clk_sel as u8) });
        self.clkgen.ctrl1().modify(|_, w| {
            w.xtal_en().clear_bit();
            w.xtal_n_en().clear_bit();
            w
        });
        match self.ref_clk_sel {
            RefClkSel::None => pll_setup_delay(),
            RefClkSel::XtalOsc => {
                self.clkgen.ctrl1().modify(|_, w| w.xtal_en().set_bit());
                hbo_clock_delay_ms(XTAL_OSC_TSTART_MS);
            }
            RefClkSel::XtalN => {
                self.clkgen.ctrl1().modify(|_, w| w.xtal_n_en().set_bit());
                pll_setup_delay()
            }
        }

        // Set up PLL configuration.
        match self.pll_cfg {
            Some(cfg) => {
                self.clkgen.ctrl0().modify(|_, w| w.pll_pwdn().clear_bit());
                // Done in C HAL. I guess this gives the PLL some time to power down properly.
                cortex_m::asm::nop();
                cortex_m::asm::nop();
                self.clkgen.ctrl0().modify(|_, w| {
                    unsafe {
                        w.pll_clkf().bits(cfg.clkf);
                    }
                    unsafe {
                        w.pll_clkr().bits(cfg.clkr);
                    }
                    unsafe {
                        w.pll_clkod().bits(cfg.clkod);
                    }
                    unsafe {
                        w.pll_bwadj().bits(cfg.bwadj);
                    }
                    w.pll_test().clear_bit();
                    w.pll_bypass().clear_bit();
                    w.pll_intfb().set_bit()
                });
                // Taken from SystemCoreClockUpdate implementation from Vorago.
                final_sysclk /= cfg.clkr as u32 + 1;
                final_sysclk *= cfg.clkf as u32 + 1;
                final_sysclk /= cfg.clkod as u32 + 1;

                // Reset PLL.
                self.clkgen.ctrl0().modify(|_, w| w.pll_reset().set_bit());
                // The HAL does this, the datasheet specifies a delay of 5 us. I guess it does not
                // really matter because the PLL lock detect is used later..
                pll_setup_delay();
                self.clkgen.ctrl0().modify(|_, w| w.pll_reset().clear_bit());
                pll_setup_delay();

                // check for lock
                let stat = self.clkgen.stat().read();
                if stat.fbslip().bit() || stat.rfslip().bit() {
                    pll_setup_delay();
                    if stat.fbslip().bit() || stat.rfslip().bit() {
                        // This is what the HAL does. We could continue, but then we would at least
                        // have to somehow report a partial error.. Chances are, the user does not
                        // want to continue with a broken PLL clock.
                        return Err(ClkCfgError::PllInitError);
                    }
                }
            }
            None => {
                self.clkgen.ctrl0().modify(|_, w| w.pll_pwdn().set_bit());
            }
        }

        if self.clk_lost_detection {
            rearm_sysclk_lost_with_periph(&self.clkgen)
        }
        #[cfg(feature = "revb")]
        if self.pll_lock_lost_detection {
            rearm_pll_lock_lost_with_periph(&self.clkgen)
        }

        self.clkgen
            .ctrl0()
            .modify(|_, w| unsafe { w.clk_div_sel().bits(self.clk_div_sel as u8) });
        final_sysclk = clk_after_div(final_sysclk, self.clk_div_sel);

        // The HAL does this. I don't know why..
        pll_setup_delay();

        self.clkgen
            .ctrl0()
            .modify(|_, w| unsafe { w.clksel_sys().bits(self.clksel_sys as u8) });

        Ok(Clocks {
            sysclk: final_sysclk,
            apb1: final_sysclk / 2,
            apb2: final_sysclk / 4,
            #[cfg(not(feature = "va41628"))]
            adc_clk: self.cfg_adc_clk_div(final_sysclk),
        })
    }

    #[cfg(not(feature = "va41628"))]
    fn cfg_adc_clk_div(&self, final_sysclk: Hertz) -> Hertz {
        // I will just do the ADC stuff like Vorago does it.
        // ADC clock (must be 2-12.5 MHz)
        // NOTE: Not using divide by 1 or /2 ratio in REVA silicon because of triggering issue
        // For this reason, keep SYSCLK above 8MHz to have the ADC /4 ratio in range)
        if final_sysclk.raw() <= ADC_MAX_CLK.raw() * 4 {
            self.clkgen
                .ctrl1()
                .modify(|_, w| unsafe { w.adc_clk_div_sel().bits(AdcClkDivSel::Div4 as u8) });
            final_sysclk / 4
        } else {
            self.clkgen
                .ctrl1()
                .modify(|_, w| unsafe { w.adc_clk_div_sel().bits(AdcClkDivSel::Div8 as u8) });
            final_sysclk / 8
        }
    }
}

/// Frozen clock frequencies
///
/// The existence of this value indicates that the clock configuration can no longer be changed.
/// The [self] module documentation gives some more information on how to retrieve an instance
/// of this structure.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Clocks {
    sysclk: Hertz,
    apb1: Hertz,
    apb2: Hertz,
    #[cfg(not(feature = "va41628"))]
    adc_clk: Hertz,
}

impl Clocks {
    /// Returns the frequency of the HBO clock
    pub const fn hbo(&self) -> Hertz {
        HBO_FREQ
    }

    /// Returns the frequency of the APB0 which is equal to the system clock.
    pub const fn apb0(&self) -> Hertz {
        self.sysclk()
    }

    /// Returns system clock divied by 2.
    pub const fn apb1(&self) -> Hertz {
        self.apb1
    }

    /// Returns system clock divied by 4.
    pub const fn apb2(&self) -> Hertz {
        self.apb2
    }

    /// Returns the system (core) frequency
    pub const fn sysclk(&self) -> Hertz {
        self.sysclk
    }

    /// Returns the ADC clock frequency which has a separate divider.
    #[cfg(not(feature = "va41628"))]
    pub const fn adc_clk(&self) -> Hertz {
        self.adc_clk
    }
}

pub fn rearm_sysclk_lost() {
    rearm_sysclk_lost_with_periph(&unsafe { pac::Clkgen::steal() })
}

fn rearm_sysclk_lost_with_periph(clkgen: &pac::Clkgen) {
    clkgen
        .ctrl0()
        .modify(|_, w| w.sys_clk_lost_det_en().set_bit());
    clkgen
        .ctrl1()
        .write(|w| w.sys_clk_lost_det_rearm().set_bit());
    clkgen
        .ctrl1()
        .write(|w| w.sys_clk_lost_det_rearm().clear_bit());
}

#[cfg(feature = "revb")]
pub fn rearm_pll_lock_lost() {
    rearm_pll_lock_lost_with_periph(&unsafe { pac::Clkgen::steal() })
}

fn rearm_pll_lock_lost_with_periph(clkgen: &pac::Clkgen) {
    clkgen
        .ctrl1()
        .modify(|_, w| w.pll_lost_lock_det_en().set_bit());
    clkgen.ctrl1().write(|w| w.pll_lck_det_rearm().set_bit());
    clkgen.ctrl1().write(|w| w.pll_lck_det_rearm().clear_bit());
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_basic_div() {
        assert_eq!(
            clk_after_div(Hertz::from_raw(10_000_000), super::ClkDivSel::Div2),
            Hertz::from_raw(5_000_000)
        );
    }
}
