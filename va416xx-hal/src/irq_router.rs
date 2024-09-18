//! IRQ Router peripheral support.
use crate::{
    clock::{PeripheralSelect, SyscfgExt},
    pac,
};

/// This enables and initiates the peripheral.
///
/// Please note that this method also writes 0 to the registers which do not have 0 as the default
/// reset value. The programmers guide v1.2 and the actual values inspected using a SVD viewer
/// are inconsistent here, and the registers being non-zero can actually lead to weird bugs
/// when working with interrupts. Registers DMASELx and ADCSEL/DMASELx will reset to 0x7f and 0x1f
/// respectively instead of 0x00.
pub fn enable_and_init_irq_router(sysconfig: &mut pac::Sysconfig, irq_router: &pac::IrqRouter) {
    sysconfig.enable_peripheral_clock(PeripheralSelect::IrqRouter);
    sysconfig.assert_periph_reset_for_two_cycles(PeripheralSelect::IrqRouter);
    unsafe {
        irq_router.dmasel0().write_with_zero(|w| w);
        irq_router.dmasel1().write_with_zero(|w| w);
        irq_router.dmasel2().write_with_zero(|w| w);
        irq_router.dmasel3().write_with_zero(|w| w);
        irq_router.adcsel().write_with_zero(|w| w);
        irq_router.dacsel0().write_with_zero(|w| w);
        irq_router.dacsel1().write_with_zero(|w| w);
    }
}
