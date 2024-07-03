//! Digital to Analog Converter (DAC) driver.
//!
//! ## Examples
//!
//! - [ADC and DAC example](https://github.com/us-irs/va416xx-rs/blob/main/examples/simple/examples/dac-adc.rs)
use core::ops::Deref;

use crate::{
    clock::{Clocks, PeripheralSelect, SyscfgExt},
    pac,
};

pub type DacRegisterBlock = pac::dac0::RegisterBlock;

/// Common trait implemented by all PAC peripheral access structures. The register block
/// format is the same for all DAC blocks.
pub trait Instance: Deref<Target = DacRegisterBlock> {
    const IDX: u8;

    fn ptr() -> *const DacRegisterBlock;
}

impl Instance for pac::Dac0 {
    const IDX: u8 = 0;

    #[inline(always)]
    fn ptr() -> *const DacRegisterBlock {
        Self::ptr()
    }
}

impl Instance for pac::Dac1 {
    const IDX: u8 = 1;

    #[inline(always)]
    fn ptr() -> *const DacRegisterBlock {
        Self::ptr()
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DacSettling {
    NoSettling = 0,
    Apb2Times25 = 1,
    Apb2Times50 = 2,
    Apb2Times75 = 3,
    Apb2Times100 = 4,
    Apb2Times125 = 5,
    Apb2Times150 = 6,
}

pub struct Dac<DacInstance> {
    dac: DacInstance,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ValueTooLarge;

impl<DacInstance: Instance> Dac<DacInstance> {
    /// Create a new [Dac] driver instance.
    ///
    /// The [Clocks] structure is expected here as well to ensure the clock was set up properly.
    pub fn new(
        syscfg: &mut pac::Sysconfig,
        dac: DacInstance,
        dac_settling: DacSettling,
        _clocks: &Clocks,
    ) -> Self {
        syscfg.enable_peripheral_clock(PeripheralSelect::Dac);

        dac.ctrl1().write(|w| {
            w.dac_en().set_bit();
            // SAFETY: Enum values are valid values only.
            unsafe { w.dac_settling().bits(dac_settling as u8) }
        });
        let dac = Self { dac };
        dac.clear_fifo();
        dac.clear_irqs();
        dac
    }

    #[inline(always)]
    pub fn clear_irqs(&self) {
        self.dac.irq_clr().write(|w| {
            w.fifo_oflow().set_bit();
            w.fifo_uflow().set_bit();
            w.dac_done().set_bit();
            w.trig_error().set_bit()
        });
    }

    #[inline(always)]
    pub fn clear_fifo(&self) {
        self.dac.fifo_clr().write(|w| unsafe { w.bits(1) });
    }

    /// Load next value into the FIFO.
    ///
    /// Uses the [nb] API to allow blocking and non-blocking usage.
    #[inline(always)]
    pub fn load_value(&self, val: u16) -> nb::Result<(), ValueTooLarge> {
        if val > 2_u16.pow(12) - 1 {
            return Err(nb::Error::Other(ValueTooLarge));
        }
        if self.dac.status().read().fifo_entry_cnt().bits() >= 32_u8 {
            return Err(nb::Error::WouldBlock);
        }
        self.dac
            .fifo_data()
            .write(|w| unsafe { w.bits(val.into()) });
        Ok(())
    }

    /// This loads and triggers the next value immediately. It also clears the FIFO before
    /// loading the passed value.
    #[inline(always)]
    pub fn load_and_trigger_manually(&self, val: u16) -> Result<(), ValueTooLarge> {
        if val > 2_u16.pow(12) - 1 {
            return Err(ValueTooLarge);
        }
        self.clear_fifo();
        // This should never block, the FIFO was cleared. We checked the value as well, so unwrap
        // is okay here.
        nb::block!(self.load_value(val)).unwrap();
        self.trigger_manually();
        Ok(())
    }

    /// Manually trigger the DAC. This will de-queue the next value inside the FIFO
    /// to be processed by the DAC.
    #[inline(always)]
    pub fn trigger_manually(&self) {
        self.dac.ctrl0().write(|w| w.man_trig_en().set_bit());
    }

    #[inline(always)]
    pub fn enable_external_trigger(&self) {
        self.dac.ctrl0().write(|w| w.ext_trig_en().set_bit());
    }

    pub fn is_settled(&self) -> nb::Result<(), ()> {
        if self.dac.status().read().dac_busy().bit_is_set() {
            return Err(nb::Error::WouldBlock);
        }
        Ok(())
    }

    #[inline(always)]
    pub fn reset(&mut self, syscfg: &mut pac::Sysconfig) {
        syscfg.enable_peripheral_clock(PeripheralSelect::Dac);
        syscfg.assert_periph_reset_for_two_cycles(PeripheralSelect::Dac);
    }

    /// Relases the DAC, which also disables its peripheral clock.
    #[inline(always)]
    pub fn release(self, syscfg: &mut pac::Sysconfig) -> DacInstance {
        syscfg.disable_peripheral_clock(PeripheralSelect::Dac);
        self.dac
    }
}
