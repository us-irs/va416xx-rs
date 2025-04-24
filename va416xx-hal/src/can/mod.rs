//! CAN driver.
//!
//! The VA416xx CAN module is based on the CP3UB26 module.
use arbitrary_int::{u11, u15, u2, u3, u4, u7, Number};
use embedded_can::Frame;
use regs::{
    BaseId, BufStatusAndControl, Control, DataDirection, ExtendedId, MmioCan, TimingConfig,
};

use crate::{clock::Clocks, enable_peripheral_clock, time::Hertz, PeripheralSelect};
use libm::roundf;

pub mod frame;
pub mod regs;
pub use frame::*;

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

impl CanId {
    /// Steal the register block for the CAN ID.
    ///
    /// # Safety
    ///
    /// See safety of the [regs::Can::new_mmio_fixed_0].
    pub unsafe fn steal_regs(&self) -> regs::MmioCan<'static> {
        match self {
            CanId::Can0 => unsafe { regs::Can::new_mmio_fixed_0() },
            CanId::Can1 => unsafe { regs::Can::new_mmio_fixed_1() },
        }
    }
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
#[error("invalid buffer index {0}")]
pub struct InvalidBufferIndexError(usize);

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

pub struct Can {
    regs: regs::MmioCan<'static>,
    id: CanId,
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
        // Disable the CAN bus before configuring it.
        regs.write_control(Control::new_with_raw_value(0));
        for i in 0..15 {
            regs.msg_buf_block_mut(i).reset();
        }
        regs.write_timing(
            TimingConfig::builder()
                .with_tseg2(clk_config.tseg2_reg_value())
                .with_tseg1(clk_config.tseg1_reg_value())
                .with_sync_jump_width(clk_config.sjw_reg_value())
                .with_prescaler(clk_config.prescaler_reg_value())
                .build(),
        );
        Self { regs, id }
    }

    /// This configures the global mask so that acceptance is only determined by an exact match
    /// with the ID in the receive message buffers. This is the default reset configuration for
    /// the global mask as well.
    pub fn set_global_mask_for_exact_id_match(&mut self) {
        self.regs.write_gmskx(ExtendedId::new_with_raw_value(0));
        self.regs.write_gmskb(BaseId::new_with_raw_value(0));
    }

    /// Similar to [Self::set_global_mask_for_exact_id_match] but masks the XRTR and RTR/SRR bits.
    ///
    /// This is useful for when transmitting remote frames with the RTR bit set. The hardware
    /// will automatically go into the [regs::BufferState::RxReady] state after the transmission,
    /// but the XRTR and RTR/SRR bits need to be masked for the response frame to be accepted
    /// on that buffer.
    pub fn set_global_mask_for_exact_id_match_with_rtr_masked(&mut self) {
        self.regs.write_gmskx(
            ExtendedId::builder()
                .with_mask_14_0(u15::new(0))
                .with_xrtr(true)
                .build(),
        );
        self.regs.write_gmskb(
            BaseId::builder()
                .with_mask_28_18(u11::new(0))
                .with_rtr_or_srr(true)
                .with_ide(false)
                .with_mask_17_15(u3::new(0))
                .build(),
        );
    }

    /// This configures the base mask for buffer 14 so that acceptance is only determined by an
    /// exact match with the ID in the receive message buffers. This is the default reset
    /// configuration for the global mask as well.
    pub fn set_base_mask_for_exact_id_match(&mut self) {
        self.regs.write_bmskx(ExtendedId::new_with_raw_value(0));
        self.regs.write_bmskb(BaseId::new_with_raw_value(0));
    }

    /// This configures the base mask so that all CAN frames which are not handled by any other
    /// buffers are accepted by the base buffer 14.
    pub fn set_base_mask_for_all_match(&mut self) {
        self.regs
            .write_bmskx(ExtendedId::new_with_raw_value(0xffff));
        self.regs.write_bmskb(BaseId::new_with_raw_value(0xffff));
    }

    #[inline]
    pub fn regs(&mut self) -> &mut MmioCan<'static> {
        &mut self.regs
    }

    #[inline]
    pub fn id(&self) -> CanId {
        self.id
    }

    #[inline]
    pub fn write_ctrl_reg(&mut self, ctrl: Control) {
        self.regs.write_control(ctrl);
    }

    #[inline]
    pub fn enable_bufflock(&mut self) {
        self.regs.modify_control(|mut ctrl| {
            ctrl.set_bufflock(true);
            ctrl
        });
    }

    #[inline]
    pub fn enable(&mut self) {
        self.regs.modify_control(|mut ctrl| {
            ctrl.set_enable(true);
            ctrl
        });
    }
}

#[derive(Debug)]
pub enum ChannelState {
    Idle,
    Receiving,
    Transmitting,
    AwaitingRtrReply,
}

pub struct CanChannel {
    can_id: CanId,
    idx: usize,
    regs: regs::MmioCanMsgBuf<'static>,
    mode: ChannelState,
}

impl core::fmt::Debug for CanChannel {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("CanChannel")
            .field("can_id", &self.can_id)
            .field("idx", &self.idx)
            .field("mode", &self.mode)
            .finish()
    }
}

impl CanChannel {
    pub fn configure_for_reception_with_standard_id(
        &mut self,
        standard_id: embedded_can::StandardId,
        set_rtr: bool,
    ) -> Result<(), InvalidBufferIndexError> {
        let mut id1_reg = standard_id.as_raw() << 5;
        if set_rtr {
            id1_reg |= 1 << 4;
        }
        self.regs
            .write_id1(BaseId::new_with_raw_value(id1_reg as u32));

        self.regs.write_stat_ctrl(
            BufStatusAndControl::builder()
                .with_dlc(u4::new(0))
                .with_priority(u4::new(0))
                .with_status(regs::BufferState::RxReady)
                .build(),
        );
        Ok(())
    }

    pub fn configure_for_reception_with_extended_id(
        &mut self,
        extended_id: embedded_can::ExtendedId,
        set_rtr: bool,
    ) -> Result<(), InvalidBufferIndexError> {
        let mut regs = unsafe { self.can_id.steal_regs() };
        let mut cmb_block = regs.msg_buf_block_mut(self.idx);
        let id_raw = extended_id.as_raw();
        let id1_reg = (((id_raw >> 18) & 0x7FF) << 4) as u16 | ((id_raw >> 15) & 0b111) as u16;
        cmb_block.write_id1(BaseId::new_with_raw_value(id1_reg as u32));
        let id0_reg = ((id_raw & 0x7FFF) << 1) as u16 | set_rtr as u16;
        cmb_block.write_id0(ExtendedId::new_with_raw_value(id0_reg as u32));
        cmb_block.write_stat_ctrl(
            BufStatusAndControl::builder()
                .with_dlc(u4::new(0))
                .with_priority(u4::new(0))
                .with_status(regs::BufferState::RxReady)
                .build(),
        );
        self.mode = ChannelState::Receiving;
        Ok(())
    }

    pub fn configure_for_transmission(
        &mut self,
        tx_priority: u4,
    ) -> Result<(), InvalidBufferIndexError> {
        let mut regs = unsafe { self.can_id.steal_regs() };
        let mut cmb_block = regs.msg_buf_block_mut(self.idx);
        cmb_block.write_stat_ctrl(
            BufStatusAndControl::builder()
                .with_dlc(u4::new(0))
                .with_priority(tx_priority)
                .with_status(regs::BufferState::TxNotActive)
                .build(),
        );
        self.mode = ChannelState::Receiving;
        Ok(())
    }

    /// Reads a received CAN frame from the message buffer.
    ///
    /// This function does not check whether the pre-requisites for reading a CAN frame were
    /// met and assumes this was already checked by the user.
    pub fn read_frame_unchecked(&self) -> CanFrame {
        let id0 = self.regs.read_id0();
        let id1 = self.regs.read_id1();
        let data0 = self.regs.read_data0();
        let data1 = self.regs.read_data1();
        let data2 = self.regs.read_data2();
        let data3 = self.regs.read_data3();
        let mut data: [u8; 8] = [0; 8];
        let mut read_data = |dlc: u4| {
            (0..dlc.as_usize()).for_each(|i| match i {
                0 => data[i] = data3.data_upper_byte().as_u8(),
                1 => data[i] = data3.data_lower_byte().as_u8(),
                2 => data[i] = data2.data_upper_byte().as_u8(),
                3 => data[i] = data2.data_lower_byte().as_u8(),
                4 => data[i] = data1.data_upper_byte().as_u8(),
                5 => data[i] = data1.data_lower_byte().as_u8(),
                6 => data[i] = data0.data_upper_byte().as_u8(),
                7 => data[i] = data0.data_lower_byte().as_u8(),
                _ => unreachable!(),
            });
        };
        let (id, rtr) = if !id1.ide() {
            let id = embedded_can::Id::Standard(
                embedded_can::StandardId::new(id1.mask_28_18().as_u16()).unwrap(),
            );
            if id1.rtr_or_srr() {
                (id, true)
            } else {
                (id, false)
            }
        } else {
            let id_raw = (id1.mask_28_18().as_u32() << 18)
                | (id1.mask_17_15().as_u32() << 15)
                | id0.mask_14_0().as_u32();
            let id = embedded_can::Id::Extended(embedded_can::ExtendedId::new(id_raw).unwrap());
            if id0.xrtr() {
                (id, true)
            } else {
                (id, false)
            }
        };
        if rtr {
            CanFrameRtr::new(id, self.regs.read_stat_ctrl().dlc().as_usize()).into()
        } else {
            let dlc = self.regs.read_stat_ctrl().dlc();
            read_data(dlc);
            CanFrameNormal::new(id, &data[0..dlc.as_usize()]).into()
        }
    }

    pub fn transmit_frame_unchecked(&mut self, frame: CanFrame) {
        let is_remote = frame.is_remote_frame();
        self.write_id(frame.id(), is_remote);
        let dlc = frame.dlc();
        self.regs.modify_stat_ctrl(|mut ctrl| {
            ctrl.set_status(regs::BufferState::TxOnce);
            ctrl
        });
    }

    fn write_id(&mut self, id: embedded_can::Id, is_remote: bool) {
        match id {
            embedded_can::Id::Standard(standard_id) => {
                self.regs.write_id1(
                    BaseId::builder()
                        .with_mask_28_18(u11::new(standard_id.as_raw()))
                        .with_rtr_or_srr(is_remote)
                        .with_ide(false)
                        .with_mask_17_15(u3::new(0))
                        .build(),
                );
                self.regs.write_id0(ExtendedId::new_with_raw_value(0));
            }
            embedded_can::Id::Extended(extended_id) => {
                let id_raw = extended_id.as_raw();
                self.regs.write_id1(
                    BaseId::builder()
                        .with_mask_28_18(u11::new(((id_raw >> 18) & 0x7FF) as u16))
                        .with_rtr_or_srr(true)
                        .with_ide(true)
                        .with_mask_17_15(u3::new(((id_raw >> 15) & 0b111) as u8))
                        .build(),
                );
                self.regs.write_id0(
                    ExtendedId::builder()
                        .with_mask_14_0(u15::new((id_raw & 0x7FFF) as u16))
                        .with_xrtr(is_remote)
                        .build(),
                );
            }
        }
    }
}

pub struct CanWorker {
    can: Can,
    channels: [CanChannel; 15],
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
