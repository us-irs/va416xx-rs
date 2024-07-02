#![no_std]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#[cfg(test)]
extern crate std;

pub use va416xx as device;
pub use va416xx as pac;

pub mod prelude;

pub mod adc;
pub mod clock;
pub mod dac;
pub mod dma;
pub mod gpio;
pub mod i2c;
pub mod pwm;
pub mod spi;
pub mod time;
pub mod timer;
pub mod typelevel;
pub mod uart;
pub mod wdt;

#[derive(Debug, Eq, Copy, Clone, PartialEq)]
pub enum FunSel {
    Sel0 = 0b00,
    Sel1 = 0b01,
    Sel2 = 0b10,
    Sel3 = 0b11,
}

/// Enable a specific interrupt using the NVIC peripheral.
///
/// # Safety
///
/// This function is `unsafe` because it can break mask-based critical sections.
#[inline]
pub unsafe fn enable_interrupt(irq: pac::Interrupt) {
    unsafe {
        cortex_m::peripheral::NVIC::unmask(irq);
    }
}

/// Disable a specific interrupt using the NVIC peripheral.
#[inline]
pub fn disable_interrupt(irq: pac::Interrupt) {
    cortex_m::peripheral::NVIC::mask(irq);
}