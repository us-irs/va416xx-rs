//! This is the **H**ardware **A**bstraction **L**ayer (HAL) for the VA416xx MCU family.
//!
//! It is an additional hardware abstraction on top of the [peripheral access API](https://egit.irs.uni-stuttgart.de/rust/va416xx-rs/src/branch/main/va416xx).

//! It is the result of reading the datasheet for the device and encoding a type-safe layer over the
//! raw PAC. This crate also implements traits specified by the
//! [embedded-hal](https://github.com/rust-embedded/embedded-hal) project, making it compatible with
//! various drivers in the embedded rust ecosystem.

//! You have to enable one of the following device features to use this crate depending on
//! which chip you are using:

//! - `va41630`
//! - `va41629`
//! - `va41628`
//! - `va41620`
//!
//! When using this HAL and writing applications for the VA416xx family in general, it is strongly
//! recommended that you set up the clock properly, because the default internal HBO clock
//! is not very accurate. You can use the [crate::clock] module for this. If you are working
//! with interrupts, it is strongly recommended to set up the IRQ router with the
//! [crate::irq_router] module at the very least because that peripheral has confusing and/or
//! faulty register reset values which might lead to weird bugs and glitches.
#![no_std]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#[cfg(test)]
extern crate std;

#[cfg(not(feature = "device-selected"))]
compile_error!(
    "This crate requires one of the following device features enabled:
        va41630
        va41629
        va41628
"
);

pub use va416xx as device;
pub use va416xx as pac;

pub mod prelude;

pub mod clock;
pub mod dma;
pub mod edac;
pub mod gpio;
pub mod i2c;
pub mod irq_router;
pub mod pwm;
pub mod spi;
pub mod time;
pub mod timer;
pub mod typelevel;
pub mod uart;
pub mod wdt;

#[cfg(feature = "va41630")]
pub mod nvm;

#[cfg(not(feature = "va41628"))]
pub mod adc;
#[cfg(not(feature = "va41628"))]
pub mod dac;

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
