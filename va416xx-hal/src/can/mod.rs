use arbitrary_int::{u2, u3, u4, u7, Number};

use crate::{clock::Clocks, enable_peripheral_clock, time::Hertz, PeripheralSelect};
use libm::roundf;

pub mod regs;

pub const PRESCALER_MIN: u8 = 2;
pub const PRESCALER_MAX: u8 = 128;
/// 1 is the minimum value, but not recommended by Vorago.
pub const TSEG1_MIN: u8 = 1;
pub const TSEG1_MAX: u8 = 16;
pub const TSEG2_MAX: u8 = 8;
/// In addition, SJW may not be larger than TSEG2.
pub const SJW_MAX: u8 = 4;

pub const MIN_SAMPLE_POINT: f32 = 0.5;
pub const MAX_BITRATE_DEVIATION: f32 = 0.005;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CanId {
    Can0 = 0,
    Can1 = 1,
}

impl CanId {}

pub struct Can {
    id: CanId,
}

/// Sample point between 0 and 1.0 for the given time segments.
pub const fn calculate_sample_point(tseg1: u8, tseg2: u8) -> f32 {
    let tseg1_val = tseg1 as f32;
    (tseg1_val + 1.0) / (1.0 + tseg1_val + tseg2 as f32)
}

/// Calculate all viable clock configurations for the given input clock, the target bitrate and
/// for a sample point between 0.5 and 1.0.
///
/// There are various recommendations for the sample point when using the CAN bus. The value
/// depends on different parameters like the bus length and propagation time, as well as
/// the information processing time of the nodes. It should always be at least 50 %.
/// In doubt, select a value like 0.75.
///
///  - The [Python CAN library](https://python-can.readthedocs.io/en/stable/bit_timing.html)
///    assumes a default value of 69 % as the sample point if none is specified.
///  - CiA-301 recommends 87.5 %
///  - For simpler setups like laboratory setups, smaller values should work as well.
///
/// A clock configuration is consideres viable when
///
///  - The sample point deviation is less than 5 %.
///  - The bitrate error is less than +-0.5 %.
///
///  SJW will be set to either TSEG2 or 4, whichever is smaller.
#[cfg(feature = "alloc")]
pub fn calculate_all_viable_clock_configs(
    apb1_clock: Hertz,
    bitrate: Hertz,
    sample_point: f32,
) -> Result<alloc::vec::Vec<ClockConfig>, InvalidSamplePointError> {
    if sample_point < 0.5 || sample_point > 1.0 {
        return Err(InvalidSamplePointError { sample_point });
    }
    let mut configs = alloc::vec::Vec::new();
    for prescaler in PRESCALER_MIN..PRESCALER_MAX {
        let nom_bit_time = apb1_clock / (bitrate * prescaler as u32);
        // This is taken from the Python CAN library. NBT should not be too small.
        if nom_bit_time < 8 {
            break;
        }
        let actual_bitrate = apb1_clock / (prescaler as u32 * nom_bit_time);
        let bitrate_deviation = ((actual_bitrate.raw() as i32 - bitrate.raw() as i32).abs() as f32)
            / bitrate.raw() as f32;
        if bitrate_deviation > 0.05 {
            continue;
        }
        let tseg1 = roundf(sample_point * nom_bit_time as f32) as u32 - 1;
        if tseg1 > TSEG1_MAX as u32 || tseg1 < TSEG1_MIN as u32 {
            continue;
        }
        // limit tseg1, so tseg2 is at least 1 TQ
        let tseg1 = core::cmp::min(tseg1, nom_bit_time - 2) as u8;
        let tseg2 = nom_bit_time - tseg1 as u32 - 1;
        if tseg2 > TSEG2_MAX as u32 {
            continue;
        }
        let tseg2 = tseg2 as u8;
        let sjw = core::cmp::min(tseg2, 4) as u8;
        // Use percent to have a higher resolution for the sample point deviation.
        let sample_point_actual = roundf(calculate_sample_point(tseg1, tseg2) * 100.0) as u32;
        let sample_point = roundf(sample_point * 100.0) as u32;
        let deviation = (sample_point_actual as i32 - sample_point as i32).abs();
        if deviation > 5 {
            continue;
        }
        configs.push(ClockConfig {
            prescaler,
            tseg1,
            tseg2,
            sjw,
        });
    }
    Ok(configs)
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

#[derive(Debug, Clone, Copy)]
pub struct ClockConfig {
    prescaler: u8,
    tseg1: u8,
    tseg2: u8,
    sjw: u8,
}

#[derive(Debug, thiserror::Error)]
#[error("sjw must be less than or equal to the smaller tseg value")]
pub struct InvalidSjwError(u8);

#[derive(Debug, thiserror::Error)]
#[error("invalid sample point {sample_point}")]
pub struct InvalidSamplePointError {
    /// Sample point, should be larger than 0.5 (50 %) but was not.
    sample_point: f32,
}

#[derive(Debug, thiserror::Error)]
pub enum ClockConfigError {
    #[error("invalid sjw: {0}")]
    InvalidSjw(#[from] InvalidSjwError),
    #[error("TSEG is zero which is not allowed")]
    TsegIsZero,
    #[error("TSEG1 is larger than 16")]
    InvalidTseg1,
    #[error("TSEG1 is larger than 8")]
    InvalidTseg2,
    #[error("invalid sample point: {0}")]
    InvalidSamplePoint(#[from] InvalidSamplePointError),
    #[error("bitrate is zero")]
    BitrateIsZero,
    #[error("bitrate error larger than +-0.5 %")]
    BitrateErrorTooLarge,
    #[error("maximum or minimum allowed prescaler is not sufficient for target bitrate clock")]
    CanNotFindPrescaler,
}

impl ClockConfig {
    /// New clock configuration from the raw configuration values.
    ///
    /// The values specified here are not the register values, but the actual numerical values
    /// relevant for calculations.
    ///
    /// The values have the following requirements:
    ///
    /// - Prescaler must be between 2 and 128.
    /// - TSEG1 must be smaller than 16 and should be larger than 1.
    /// - TSEG2 must be smaller than 8 and small enough so that the calculated sample point
    ///   is larger than 0.5 (50 %).
    /// - SJW (Synchronization Jump Width) must be smaller than the smaller of the time segment
    ///   configuration values and smaller than 4.
    pub fn new(prescaler: u8, tseg1: u8, tseg2: u8, sjw: u8) -> Result<Self, ClockConfigError> {
        if !(PRESCALER_MIN..=PRESCALER_MAX).contains(&prescaler.value()) {
            return Err(ClockConfigError::CanNotFindPrescaler);
        }
        if tseg1 == 0 || tseg2 == 0 {
            return Err(ClockConfigError::TsegIsZero);
        }
        if tseg1 > TSEG1_MAX {
            return Err(ClockConfigError::InvalidTseg1);
        }
        if tseg2 > TSEG2_MAX {
            return Err(ClockConfigError::InvalidTseg2);
        }
        let smaller_tseg = core::cmp::min(tseg1.value(), tseg2.value());
        if sjw.value() > smaller_tseg || sjw > SJW_MAX {
            return Err(InvalidSjwError(sjw).into());
        }
        let sample_point = calculate_sample_point(tseg1, tseg2);
        if sample_point < MIN_SAMPLE_POINT {
            return Err(InvalidSamplePointError { sample_point }.into());
        }
        Ok(Self {
            prescaler,
            tseg1,
            tseg2,
            sjw,
        })
    }

    /// Calculate the clock configuration for the given input clock, the target bitrate and for a
    /// set of timing parameters.
    ///
    /// This function basically calculates the necessary prescaler to achieve the given timing
    /// parameters. It also performs sanity and validity checks for the calculated prescaler:
    /// The bitrate error for the given prescaler needs to be smaller than 0.5 %.
    pub fn from_bitrate_and_segments(
        clocks: &Clocks,
        bitrate: Hertz,
        tseg1: u8,
        tseg2: u8,
        sjw: u8,
    ) -> Result<ClockConfig, ClockConfigError> {
        if bitrate.raw() == 0 {
            return Err(ClockConfigError::BitrateIsZero);
        }
        let prescaler = roundf(
            clocks.apb1().raw() as f32
                / (bitrate.raw() as f32 * (1.0 + tseg1.as_u32() as f32 + tseg2.as_u32() as f32)),
        ) as u32;
        if !(PRESCALER_MIN as u32..=PRESCALER_MAX as u32).contains(&prescaler) {
            return Err(ClockConfigError::CanNotFindPrescaler);
        }

        let actual_bitrate = clocks.apb1() / (prescaler * (1 + tseg1.as_u32() + tseg2.as_u32()));
        let bitrate_deviation = ((actual_bitrate.raw() as i32 - bitrate.raw() as i32).abs() as f32)
            / bitrate.raw() as f32;
        if bitrate_deviation > MAX_BITRATE_DEVIATION {
            return Err(ClockConfigError::BitrateErrorTooLarge);
        }
        // The subtractions are fine because we made checks to avoid underflows.
        Self::new(prescaler as u8, tseg1, tseg2, sjw)
    }

    pub fn sjw_reg_value(&self) -> u2 {
        u2::new(self.sjw.value() - 1)
    }

    pub fn tseg1_reg_value(&self) -> u4 {
        u4::new(self.tseg1.value() - 1)
    }

    pub fn tseg2_reg_value(&self) -> u3 {
        u3::new(self.tseg2.value() - 1)
    }

    pub fn prescaler_reg_value(&self) -> u7 {
        u7::new(self.prescaler.value() - 2)
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

    pub fn id(&self) -> CanId {
        self.id
    }
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "alloc")]
    use std::println;

    #[cfg(feature = "alloc")]
    #[test]
    pub fn test_clock_calculator_example_1() {
        let configs = super::calculate_all_viable_clock_configs(
            crate::time::Hertz::from_raw(50_000_000),
            crate::time::Hertz::from_raw(25_000),
            0.75,
        )
        .expect("clock calculation failed");
        // Bitrate: 25278.05 Hz. Sample point: 0.7391
        assert_eq!(configs[0].prescaler, 84);
        assert_eq!(configs[0].tseg1, 16);
        assert_eq!(configs[0].tseg2, 6);
        assert_eq!(configs[0].sjw, 4);
        // Vorago sample value.
        let sample_cfg = configs
            .iter()
            .find(|c| c.prescaler == 100)
            .expect("clock config not found");
        // Slightly different distribution because we use a different sample point, but
        // the sum of TSEG1 and TSEG2 is the same as the Vorago example 1.
        assert_eq!(sample_cfg.tseg1, 14);
        assert_eq!(sample_cfg.tseg2, 5);
    }
}
