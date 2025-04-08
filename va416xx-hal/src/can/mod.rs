use arbitrary_int::{u2, u3, u4, u7};

use crate::{clock::Clocks, enable_peripheral_clock, PeripheralSelect};

pub mod regs;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CanId {
    Can0 = 0,
    Can1 = 1,
}

impl CanId {}

pub struct Can {
    id: CanId,
}

pub trait Instance {
    const ID: CanId;
    const PERIPH_SEL: PeripheralSelect;
}

impl Instance for va416xx::Can0 {
    const ID: CanId = CanId::Can0;
    const PERIPH_SEL: PeripheralSelect = PeripheralSelect::Can0;
}

impl Instance for va416xx::Can1 {
    const ID: CanId = CanId::Can1;
    const PERIPH_SEL: PeripheralSelect = PeripheralSelect::Can1;
}

pub struct ClockConfig {
    prescaler: u7,
    tseg1: u4,
    tseg2: u3,
    sjw: u2,
}

impl ClockConfig {
    /// New clock configuration from the raw configuration values.
    pub fn new(prescaler: u7, tseg1: u4, tseg2: u3, sjw: u2) -> Self {
        Self {
            prescaler,
            tseg1,
            tseg2,
            sjw,
        }
    }
}

impl Can {
    pub fn new<CanI: Instance>(_can: CanI, clk_config: ClockConfig) -> Self {
        enable_peripheral_clock(CanI::PERIPH_SEL);
        let id = CanI::ID;
        let mut regs = if id == CanId::Can0 {
            unsafe { regs::Can::new_mmio_fixed_0() }
        } else {
            unsafe { regs::Can::new_mmio_fixed_1() }
        };
        for i in 0..15 {
            regs.msg_buf_block_mut(i).reset();
        }
        Self { id }
    }
}
