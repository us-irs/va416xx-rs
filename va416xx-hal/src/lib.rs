//! This is the **H**ardware **A**bstraction **L**ayer (HAL) for the VA416xx MCU family.
//!
//! It is an additional hardware abstraction on top of the [peripheral access API](https://egit.irs.uni-stuttgart.de/rust/va416xx-rs/src/branch/main/va416xx).
//!
//! It is the result of reading the datasheet for the device and encoding a type-safe layer over the
//! raw PAC. This crate also implements traits specified by the
//! [embedded-hal](https://github.com/rust-embedded/embedded-hal) project, making it compatible with
//! various drivers in the embedded rust ecosystem.
//!
//! It is generally advised to enable ONE of the following device features to use this crate
//! depending on which chip you are using:
//!
//! - `va41630`
//! - `va41629`
//! - `va41628`
//! - `va41620`
//!
//! If no option is specified, only access to APIs which are common for all families or
//! which are not disabled for specific families is granted.
//!
//! When using this HAL and writing applications for the VA416xx family in general, it is strongly
//! recommended that you set up the clock properly, because the default internal HBO clock
//! is not very accurate. You can use the [crate::clock] module for this. If you are working
//! with interrupts, it is strongly recommended to set up the IRQ router with the
//! [crate::irq_router] module at the very least because that peripheral has confusing and/or
//! faulty register reset values which might lead to weird bugs and glitches.
#![no_std]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(test)]
extern crate std;

use gpio::Port;
pub use va416xx as device;
pub use va416xx as pac;

pub mod prelude;

pub mod can;
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

#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
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

#[inline(always)]
pub fn enable_peripheral_clock(clock: PeripheralSelect) {
    // Safety: Only bit of peripheral is modified.
    unsafe { pac::Sysconfig::steal() }
        .peripheral_clk_enable()
        .modify(|r, w| unsafe { w.bits(r.bits() | (1 << clock as u8)) });
}

#[inline(always)]
pub fn disable_peripheral_clock(clock: PeripheralSelect) {
    // Safety: Only bit of peripheral is modified.
    unsafe { pac::Sysconfig::steal() }
        .peripheral_clk_enable()
        .modify(|r, w| unsafe { w.bits(r.bits() & !(1 << clock as u8)) });
}

#[inline(always)]
pub fn assert_periph_reset(periph: PeripheralSelect) {
    // Safety: Only reset bit of peripheral is modified.
    unsafe { pac::Sysconfig::steal() }
        .peripheral_reset()
        .modify(|r, w| unsafe { w.bits(r.bits() & !(1 << periph as u8)) });
}

#[inline(always)]
pub fn deassert_periph_reset(periph: PeripheralSelect) {
    // Safety: Only rest bit of peripheral is modified.
    unsafe { pac::Sysconfig::steal() }
        .peripheral_reset()
        .modify(|r, w| unsafe { w.bits(r.bits() | (1 << periph as u8)) });
}

#[inline(always)]
fn assert_periph_reset_for_two_cycles(periph: PeripheralSelect) {
    assert_periph_reset(periph);
    cortex_m::asm::nop();
    cortex_m::asm::nop();
    deassert_periph_reset(periph);
}

#[derive(Debug, Eq, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FunSel {
    Sel0 = 0b00,
    Sel1 = 0b01,
    Sel2 = 0b10,
    Sel3 = 0b11,
}

#[derive(Debug, PartialEq, Eq, thiserror::Error)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[error("invalid pin with number {0}")]
pub struct InvalidPinError(u8);

/// Can be used to manually manipulate the function select of port pins.
///
/// The function selection table can be found on p.286 of the programmers guide. Please note
/// that most of the structures and APIs in this library will automatically correctly configure
/// the pin or statically expect the correct pin type.
#[inline]
pub fn port_function_select(
    ioconfig: &mut pac::Ioconfig,
    port: Port,
    pin: u8,
    funsel: FunSel,
) -> Result<(), InvalidPinError> {
    if (port == Port::G && pin >= 8) || pin >= 16 {
        return Err(InvalidPinError(pin));
    }
    let reg_block = match port {
        Port::A => ioconfig.porta(pin as usize),
        Port::B => ioconfig.portb0(pin as usize),
        Port::C => ioconfig.portc0(pin as usize),
        Port::D => ioconfig.portd0(pin as usize),
        Port::E => ioconfig.porte0(pin as usize),
        Port::F => ioconfig.portf0(pin as usize),
        Port::G => ioconfig.portg0(pin as usize),
    };

    reg_block.modify(|_, w| unsafe { w.funsel().bits(funsel as u8) });
    Ok(())
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
        enable_peripheral_clock(clock)
    }

    #[inline(always)]
    fn disable_peripheral_clock(&mut self, clock: PeripheralClock) {
        disable_peripheral_clock(clock)
    }

    #[inline(always)]
    fn assert_periph_reset(&mut self, clock: PeripheralSelect) {
        assert_periph_reset(clock)
    }

    #[inline(always)]
    fn deassert_periph_reset(&mut self, clock: PeripheralSelect) {
        deassert_periph_reset(clock)
    }

    #[inline(always)]
    fn assert_periph_reset_for_two_cycles(&mut self, periph: PeripheralSelect) {
        assert_periph_reset_for_two_cycles(periph)
    }
}

/// Enable a specific interrupt using the NVIC peripheral.
///
/// # Safety
///
/// This function is `unsafe` because it can break mask-based critical sections.
#[inline]
pub unsafe fn enable_nvic_interrupt(irq: pac::Interrupt) {
    cortex_m::peripheral::NVIC::unmask(irq);
}

/// Disable a specific interrupt using the NVIC peripheral.
#[inline]
pub fn disable_nvic_interrupt(irq: pac::Interrupt) {
    cortex_m::peripheral::NVIC::mask(irq);
}
