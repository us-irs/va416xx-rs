use arbitrary_int::{u2, u3, u4, u7, Number};

use crate::{clock::Clocks, enable_peripheral_clock, time::Hertz, PeripheralSelect};

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

#[derive(Debug, thiserror::Error)]
#[error("sjw must be less than or equal to the smaller tseg value")]
pub struct InvalidSjwError(u2);

/// Sample point between 0 and 1.0 for the given time segments.
pub const fn sample_point(tseg1: u4, tseg2: u3) -> f32 {
    let tseg1_val = tseg1.value() as f32;
    (tseg1_val + 1.0) / (1.0 + tseg1_val + tseg2.value() as f32)
}

#[derive(Debug, thiserror::Error)]
#[error("invalid sample point {sample_point} for tseg1 {tseg1} and tseg2 {tseg2}.")]
pub struct InvalidSamplePointError {
    tseg1: u4,
    tseg2: u3,
    /// Sample point, should be larger than 0.5 (50 %) but was not.
    sample_point: f32,
}

#[derive(Debug, thiserror::Error)]
pub enum ClockConfigError {
    #[error("invalid sjw: {0}")]
    InvalidSjw(#[from] InvalidSjwError),
    #[error("invalid sample point: {0}")]
    InvalidSamplePoint(#[from] InvalidSamplePointError),
}

impl ClockConfig {
    /// New clock configuration from the raw configuration values.
    ///
    /// The synchronization jump width MUST be smaller than the smaller of the time segment
    /// configuration values. The sample point must also be larger than 50 %.
    pub fn new(prescaler: u7, tseg1: u4, tseg2: u3, sjw: u2) -> Result<Self, ClockConfigError> {
        let smaller_tseg = core::cmp::min(tseg1.value(), tseg2.value());
        if sjw.value() > smaller_tseg {
            return Err(InvalidSjwError(sjw).into());
        }
        let sample_point = sample_point(tseg1, tseg2);
        if sample_point < 0.5 {
            return Err(InvalidSamplePointError {
                tseg1,
                tseg2,
                sample_point,
            }.into());
        }
        Ok(Self {
            prescaler,
            tseg1,
            tseg2,
            sjw,
        })
    }

    pub fn from_bitrate(clocks: &Clocks, bitrate: Hertz, prescaler: u7, tseg1: u4, tseg2: u3, sjw: u2) {}

    pub fn from_sample_point(clocks: &Clocks, bitrate: Hertz, sample_point: f32) {}
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
