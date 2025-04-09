//! Custom register definitions for the CAN register block to circumvent PAC API / SVD
//! shortcomings.

use arbitrary_int::{u11, u15, u2, u3, u4, u7};

pub const CAN_0_BASE: usize = 0x4001_4000;
pub const CAN_1_BASE: usize = 0x4001_4400;

#[derive(Debug)]
#[bitbybit::bitenum(u4)]
pub enum BufferState {
    /// Passive channel.
    RxNotActive = 0b0000,
    /// This condition indicated that SW wrote RxNotActive to a buffer when a data copy
    /// process is still active.
    RxBusy = 0b0001,
    RxReady = 0b0010,
    /// Indicated that data is being copied for the first time (RxRead -> RxBusy0).
    RxBusy0 = 0b0011,
    RxFull = 0b0100,
    /// Indicated that data is being copied for the second time (RxFull -> RxBusy2).
    RxBusy1 = 0b0101,
    RxOverrun = 0b0110,
    RxBusy2 = 0b0111,
    TxNotActive = 0b1000,
    /// Automatical response to a remote frame.
    TxRtr = 0b1010,
    /// Transmit one frame.
    TxOnce = 0b1100,
    TxBusy0 = 0b1101,
    /// Transmit one frame, and changes to TxRtr after that. This can either be written by
    /// software, or it will be written by the hardware after an auto response of the
    /// [BufferState::TxRtr] state.
    TxOnceRtr = 0b1110,
    TxBusy2 = 0b1111,
}

/// Status control register for individual message buffers.
#[bitbybit::bitfield(u32, default = 0x0)]
#[derive(Debug)]
pub struct BufStatusAndControl {
    /// Data length code.
    #[bits(12..=15, rw)]
    dlc: u4,
    #[bits(4..=7, rw)]
    priority: u4,
    #[bits(0..=3, rw)]
    status: Option<BufferState>,
}

#[derive(Debug)]
pub struct Timestamp(arbitrary_int::UInt<u32, 16>);

impl Timestamp {
    pub fn new(value: u16) -> Self {
        Self(value.into())
    }
    pub fn value(&self) -> u16 {
        self.0.value() as u16
    }

    pub fn write(&mut self, value: u16) {
        self.0 = value.into();
    }
}

#[bitbybit::bitfield(u32, default = 0x0)]
#[derive(Debug)]
pub struct TwoBytesData {
    #[bits(8..=15, rw)]
    data_upper_byte: u8,
    #[bits(8..=15, rw)]
    data_lower_byte: u8,
}

#[derive(derive_mmio::Mmio)]
#[repr(C)]
pub struct CanMsgBuf {
    stat_ctrl: BufStatusAndControl,
    timestamp: Timestamp,
    data3: TwoBytesData,
    data2: TwoBytesData,
    data1: TwoBytesData,
    data0: TwoBytesData,
    id0: ExtendedId,
    id1: BaseId,
}

impl MmioCanMsgBuf<'_> {
    pub fn reset(&mut self) {
        self.write_stat_ctrl(BufStatusAndControl::new_with_raw_value(0));
        self.write_timestamp(Timestamp::new(0));
        self.write_data3(TwoBytesData::new_with_raw_value(0));
        self.write_data2(TwoBytesData::new_with_raw_value(0));
        self.write_data1(TwoBytesData::new_with_raw_value(0));
        self.write_data0(TwoBytesData::new_with_raw_value(0));
        self.write_id1(BaseId::new_with_raw_value(0));
        self.write_id0(ExtendedId::new_with_raw_value(0));
    }
}

#[bitbybit::bitenum(u1, exhaustive = true)]
#[derive(Debug)]
pub enum PinLogicLevel {
    DominantIsZero = 0b0,
    DominantIsOne = 0b1,
}

#[bitbybit::bitenum(u1, exhaustive = true)]
#[derive(Debug)]
pub enum ErrorInterruptType {
    /// EIPND bit is set on every error.
    EveryError = 0b0,
    /// EIPND bit is only set if error state changes as a result of a receive or transmit
    /// error counter increment.
    ErrorOnRxTxCounterChange = 0b1,
}

#[bitbybit::bitenum(u1, exhaustive = true)]
#[derive(Debug)]
pub enum DataDirection {
    FirstByteAtHighestAddr = 0b0,
    LastByteAtHighestAddr = 0b1,
}

#[bitbybit::bitfield(u32)]
pub struct Control {
    #[bit(11, rw)]
    error_interrupt_type: ErrorInterruptType,
    /// Enables special diagnostics features of the CAN like LO, IGNACK, LOOPBACK, INTERNAL.
    #[bit(10, rw)]
    diag_enable: bool,
    /// CANTX and CANRX pins are internally connected to each other.
    #[bit(9, rw)]
    internal: bool,
    /// All messages sent by the CAN controller can also be received by a CAN buffer with a
    /// matching buffer ID.
    #[bit(8, rw)]
    loopback: bool,
    /// IGNACK feature. The CAN does not expect to receive an ACK bit.
    #[bit(7, rw)]
    ignore_ack: bool,
    /// LO feature. The CAN is only configured as a receiver.
    #[bit(6, rw)]
    listen_only: bool,
    #[bit(5, rw)]
    data_dir: DataDirection,
    #[bit(4, rw)]
    timestamp_enable: bool,
    #[bit(3, rw)]
    bufflock: bool,
    #[bit(2, rw)]
    tx_logic_level: PinLogicLevel,
    #[bit(1, rw)]
    rx_logic_level: PinLogicLevel,
    #[bit(0, rw)]
    enable: bool,
}

#[bitbybit::bitfield(u32, default = 0x0)]
#[derive(Debug)]
pub struct TimingConfig {
    #[bits(0..=2, rw)]
    tseg2: u3,
    #[bits(3..=6, rw)]
    tseg1: u4,
    #[bits(7..=8, rw)]
    sync_jump_width: u2,
    #[bits(9..=15, rw)]
    prescaler: u7,
}

#[bitbybit::bitfield(u32)]
#[derive(Debug)]
pub struct InterruptEnable {
    #[bit(15, rw)]
    error: bool,
    #[bit(0, rw)]
    buffer: [bool; 15],
}

#[bitbybit::bitfield(u32)]
#[derive(Debug)]
pub struct InterruptClear {
    #[bit(15, w)]
    error: bool,
    #[bit(0, w)]
    buffer: [bool; 15],
}

#[bitbybit::bitfield(u32)]
#[derive(Debug)]
pub struct InterruptPending {
    #[bit(15, r)]
    error: bool,
    #[bit(0, r)]
    buffer: [bool; 15],
}

#[bitbybit::bitfield(u32)]
#[derive(Debug)]
pub struct ErrorCounter {
    #[bits(0..=7, r)]
    transmit: u8,
    #[bits(8..=15, r)]
    receive: u8,
}

/// This register is unused for standard frames.
#[bitbybit::bitfield(u32, default = 0x0)]
#[derive(Debug)]
pub struct ExtendedId {
    /// Mask for ID bits \[14:0\] of extended frames.
    #[bits(1..=15, rw)]
    mask_14_0: u15,
    /// CAN XRTR bit.
    #[bit(0, rw)]
    xrtr: bool,
}

#[bitbybit::bitfield(u32, default = 0x0)]
#[derive(Debug)]
pub struct BaseId {
    /// This will contain ID\[10:0\] for standard frames and bits [28:18] for extended frames.
    #[bits(5..=15, rw)]
    mask_28_18: u11,
    /// This is the RTR bit for standard frames, and the SRR bit for extended frames.
    #[bit(4, rw)]
    rtr_or_srr: bool,
    /// Identifier extension bit.
    #[bit(3, rw)]
    ide: bool,
    /// Mask for ID bits \[17:15\] of extended frames.
    #[bits(0..=2, rw)]
    mask_17_15: u3,
}

#[derive(derive_mmio::Mmio)]
#[repr(C)]
pub struct Can {
    #[mmio(inner)]
    cmb0: CanMsgBuf,
    #[mmio(inner)]
    cmb1: CanMsgBuf,
    #[mmio(inner)]
    cmb2: CanMsgBuf,
    #[mmio(inner)]
    cmb3: CanMsgBuf,
    #[mmio(inner)]
    cmb4: CanMsgBuf,
    #[mmio(inner)]
    cmb5: CanMsgBuf,
    #[mmio(inner)]
    cmb6: CanMsgBuf,
    #[mmio(inner)]
    cmb7: CanMsgBuf,
    #[mmio(inner)]
    cmb8: CanMsgBuf,
    #[mmio(inner)]
    cmb9: CanMsgBuf,
    #[mmio(inner)]
    cmb10: CanMsgBuf,
    #[mmio(inner)]
    cmb11: CanMsgBuf,
    #[mmio(inner)]
    cmb12: CanMsgBuf,
    #[mmio(inner)]
    cmb13: CanMsgBuf,
    // This CAN message buffer has different mask registers.
    #[mmio(inner)]
    cmb14: CanMsgBuf,
    /// Hidden CAN message buffer. Only allowed to be used internally by the peripheral.
    #[mmio(inner)]
    _hcmb: CanMsgBuf,
    control: Control,
    timing: TimingConfig,
    /// Global mask extension used for buffers 0 to 13.
    gmskx: ExtendedId,
    /// Global mask base used for buffers 0 to 13.
    gmskb: BaseId,
    /// Basic mask extension used for buffer 14.
    bmskx: ExtendedId,
    /// Basic mask base used for buffer 14.
    bmskb: BaseId,
    ien: InterruptEnable,
    #[mmio(PureRead)]
    ipnd: InterruptPending,
    #[mmio(Write)]
    iclr: InterruptClear,
    /// Interrupt Code Enable Register.
    icen: InterruptEnable,
    #[mmio(PureRead)]
    status_pending: u32,
    #[mmio(PureRead)]
    error_counter: ErrorCounter,
    #[mmio(PureRead)]
    diag: u32,
    #[mmio(PureRead)]
    timer: u32,
}

impl Can {
    /// Create a new CAN MMIO instance for peripheral 0.
    ///
    /// # Safety
    ///
    /// This API can be used to potentially create a driver to the same peripheral structure
    /// from multiple threads. The user must ensure that concurrent accesses are safe and do not
    /// interfere with each other.
    pub const unsafe fn new_mmio_fixed_0() -> MmioCan<'static> {
        Self::new_mmio_at(CAN_0_BASE)
    }

    /// Create a new CAN MMIO instance for peripheral 1.
    ///
    /// # Safety
    ///
    /// This API can be used to potentially create a driver to the same peripheral structure
    /// from multiple threads. The user must ensure that concurrent accesses are safe and do not
    /// interfere with each other.
    pub const unsafe fn new_mmio_fixed_1() -> MmioCan<'static> {
        Self::new_mmio_at(CAN_1_BASE)
    }
}

impl MmioCan<'_> {
    // TODO: It would be nice if derive-mmio could generate this for us..
    pub fn msg_buf_block_mut(&mut self, idx: usize) -> MmioCanMsgBuf<'static> {
        assert!(idx < 15, "invalid index for CAN message buffer");
        match idx {
            0 => unsafe { self.steal_cmb0() },
            1 => unsafe { self.steal_cmb1() },
            2 => unsafe { self.steal_cmb2() },
            3 => unsafe { self.steal_cmb3() },
            4 => unsafe { self.steal_cmb4() },
            5 => unsafe { self.steal_cmb5() },
            6 => unsafe { self.steal_cmb6() },
            7 => unsafe { self.steal_cmb7() },
            8 => unsafe { self.steal_cmb8() },
            9 => unsafe { self.steal_cmb9() },
            10 => unsafe { self.steal_cmb10() },
            11 => unsafe { self.steal_cmb11() },
            12 => unsafe { self.steal_cmb12() },
            13 => unsafe { self.steal_cmb13() },
            14 => unsafe { self.steal_cmb14() },
            _ => unreachable!(),
        }
    }
}
