use crate::{
    clock::{PeripheralSelect, SyscfgExt},
    pac,
};

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
