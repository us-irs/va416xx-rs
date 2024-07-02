//! API for the DMA peripheral
//!
//! ## Examples
//!
//! - [Simple DMA example](https://egit.irs.uni-stuttgart.de/rust/va416xx-rs/src/branch/main/examples/simple/examples/dma.rs)
use crate::{
    clock::{PeripheralClock, PeripheralSelect},
    enable_interrupt, pac,
    prelude::*,
};

const MAX_DMA_TRANSFERS_PER_CYCLE: usize = 1024;
const BASE_PTR_ADDR_MASK: u32 = 0b1111111;

/// DMA cycle control values.
///
/// Refer to chapter 6.3.1 and 6.6.3 of the datasheet for more details.
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CycleControl {
    /// Indicates that the data structure is invalid.
    Stop = 0b000,
    /// The controller must receive a new request prior to entering the arbitration
    /// process, to enable the DMA cycle to complete. This means that the DMA will only
    /// continue to do transfers as long as a trigger signal is still active. Therefore,
    /// this should not be used for momentary triggers like a timer.
    Basic = 0b001,
    /// The controller automatically inserts a request for the appropriate channel during the
    /// arbitration process. This means that the initial request is sufficient to enable the
    /// DMA cycle to complete.
    Auto = 0b010,
    /// This is used to support continuous data flow. Both primary and alternate data structure
    /// are used. The primary data structure is used first. When the first transfer is complete, an
    /// interrupt can be generated, and the DMA switches to the alternate data structure. When the
    /// second transfer is complete, the primary data structure is used. This pattern continues
    /// until software disables the channel.
    PingPong = 0b011,
    MemScatterGatherPrimary = 0b100,
    MemScatterGatherAlternate = 0b101,
    PeriphScatterGatherPrimary = 0b110,
    PeriphScatterGatherAlternate = 0b111,
}

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AddrIncrement {
    Byte = 0b00,
    Halfword = 0b01,
    Word = 0b10,
    None = 0b11,
}

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DataSize {
    Byte = 0b00,
    Halfword = 0b01,
    Word = 0b10,
}

/// This configuration controls how many DMA transfers can occur before the controller arbitrates.
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RPower {
    EachTransfer = 0b0000,
    Every2 = 0b0001,
    Every4 = 0b0010,
    Every8 = 0b0011,
    Every16 = 0b0100,
    Every32 = 0b0101,
    Every64 = 0b0110,
    Every128 = 0b0111,
    Every256 = 0b1000,
    Every512 = 0b1001,
    Every1024Min = 0b1010,
    Every1024 = 0b1111,
}

#[derive(Debug, PartialEq, Eq)]
pub struct InvalidCtrlBlockAddr;

bitfield::bitfield! {
    #[repr(transparent)]
    #[derive(Clone, Copy)]
    pub struct ChannelConfig(u32);
    impl Debug;
    u32;
    pub raw, set_raw: 31,0;
    u8;
    dst_inc, set_dst_inc: 31, 30;
    u8;
    dst_size, set_dst_size: 29, 28;
    u8;
    src_inc, set_src_inc: 27, 26;
    u8;
    src_size, set_src_size: 25, 24;
    u8;
    dest_prot_ctrl, set_dest_prot_ctrl: 23, 21;
    u8;
    src_prot_ctrl, set_src_prot_ctrl: 20, 18;
    u8;
    r_power, set_r_power: 17, 14;
    u16;
    n_minus_1, set_n_minus_1: 13, 4;
    bool;
    next_useburst, set_next_useburst: 3;
    u8;
    cycle_ctrl, set_cycle_ctr: 2, 0;
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DmaChannelControl {
    pub src_end_ptr: u32,
    pub dest_end_ptr: u32,
    pub cfg: ChannelConfig,
    padding: u32,
}

impl Default for DmaChannelControl {
    fn default() -> Self {
        Self {
            src_end_ptr: Default::default(),
            dest_end_ptr: Default::default(),
            cfg: ChannelConfig(0),
            padding: Default::default(),
        }
    }
}
#[repr(C)]
pub struct DmaCtrlBlock {
    pub pri: [DmaChannelControl; 4],
    pub alt: [DmaChannelControl; 4],
}

impl DmaCtrlBlock {
    /// This function creates a DMA control block at the specified memory address.
    ///
    /// The passed address must be 128-byte aligned. The user must also take care of specifying
    /// a valid memory address for the DMA control block which is accessible by the system as well.
    /// For example, the control block can be placed in the SRAM1.
    pub fn new_at_addr(addr: u32) -> Result<*mut DmaCtrlBlock, InvalidCtrlBlockAddr> {
        if addr & BASE_PTR_ADDR_MASK > 0 {
            return Err(InvalidCtrlBlockAddr);
        }
        let ctrl_block_ptr = addr as *mut DmaCtrlBlock;
        unsafe {
            core::ptr::write(
                ctrl_block_ptr,
                DmaCtrlBlock {
                    pri: [DmaChannelControl::default(); 4],
                    alt: [DmaChannelControl::default(); 4],
                },
            )
        }
        Ok(ctrl_block_ptr)
    }
}

pub struct Dma {
    dma: pac::Dma,
    ctrl_block: *mut DmaCtrlBlock,
}

#[derive(Debug, Clone, Copy)]
pub enum DmaTransferInitError {
    SourceDestLenMissmatch {
        src_len: usize,
        dest_len: usize,
    },
    /// Overflow when calculating the source or destination end address.
    AddrOverflow,
    /// Transfer size larger than 1024 units.
    TransferSizeTooLarge(usize),
}

#[derive(Debug, Clone, Copy, Default)]
pub struct DmaCfg {
    pub bufferable: bool,
    pub cacheable: bool,
    pub privileged: bool,
}

pub struct DmaChannel {
    idx: u8,
    done_interrupt: pac::Interrupt,
    active_interrupt: pac::Interrupt,
    pub dma: pac::Dma,
    pub ch_ctrl_pri: &'static mut DmaChannelControl,
    pub ch_ctrl_alt: &'static mut DmaChannelControl,
}

impl DmaChannel {
    #[inline(always)]
    pub fn enable(&mut self) {
        self.dma
            .chnl_enable_set()
            .write(|w| unsafe { w.bits(1 << self.idx) });
    }

    #[inline(always)]
    pub fn is_enabled(&mut self) -> bool {
        ((self.dma.chnl_enable_set().read().bits() >> self.idx) & 0b1) != 0
    }

    #[inline(always)]
    pub fn disable(&mut self) {
        self.dma
            .chnl_enable_clr()
            .write(|w| unsafe { w.bits(1 << self.idx) });
    }

    #[inline(always)]
    pub fn trigger_with_sw_request(&mut self) {
        self.dma
            .chnl_sw_request()
            .write(|w| unsafe { w.bits(1 << self.idx) });
    }

    #[inline(always)]
    pub fn state_raw(&self) -> u8 {
        self.dma.status().read().state().bits()
    }

    #[inline(always)]
    pub fn select_primary_structure(&self) {
        self.dma
            .chnl_pri_alt_clr()
            .write(|w| unsafe { w.bits(1 << self.idx) });
    }

    #[inline(always)]
    pub fn select_alternate_structure(&self) {
        self.dma
            .chnl_pri_alt_set()
            .write(|w| unsafe { w.bits(1 << self.idx) });
    }

    /// Enables the DMA_DONE interrupt for the DMA channel.
    ///
    /// # Safety
    ///
    /// This function is `unsafe` because it can break mask-based critical sections.
    pub unsafe fn enable_done_interrupt(&mut self) {
        enable_interrupt(self.done_interrupt);
    }

    /// Enables the DMA_ACTIVE interrupt for the DMA channel.
    ///
    /// # Safety
    ///
    /// This function is `unsafe` because it can break mask-based critical sections.
    pub unsafe fn enable_active_interrupt(&mut self) {
        enable_interrupt(self.active_interrupt);
    }

    /// Prepares a 8-bit DMA transfer from memory to memory.
    ///
    /// This function does not enable the DMA channel and interrupts and only prepares
    /// the DMA control block parameters for the transfer.
    ///
    /// You can use [Self::enable], [Self::enable_done_interrupt], [Self::enable_active_interrupt]
    /// to finish the transfer preparation and then use [Self::trigger_with_sw_request] to
    /// start the DMA transfer.
    pub fn prepare_mem_to_mem_transfer_8_bit(
        &mut self,
        source: &[u8],
        dest: &mut [u8],
    ) -> Result<(), DmaTransferInitError> {
        let len = Self::common_mem_transfer_checks(source.len(), dest.len())?;
        self.generic_mem_to_mem_transfer_init(
            len,
            (source.as_ptr() as u32)
                .checked_add(len as u32)
                .ok_or(DmaTransferInitError::AddrOverflow)?,
            (dest.as_ptr() as u32)
                .checked_add(len as u32)
                .ok_or(DmaTransferInitError::AddrOverflow)?,
            DataSize::Byte,
            AddrIncrement::Byte,
        );
        Ok(())
    }

    /// Prepares a 16-bit DMA transfer from memory to memory.
    ///
    /// This function does not enable the DMA channel and interrupts and only prepares
    /// the DMA control block parameters for the transfer.
    ///
    /// You can use [Self::enable], [Self::enable_done_interrupt], [Self::enable_active_interrupt]
    /// to finish the transfer preparation and then use [Self::trigger_with_sw_request] to
    /// start the DMA transfer.
    pub fn prepare_mem_to_mem_transfer_16_bit(
        &mut self,
        source: &[u16],
        dest: &mut [u16],
    ) -> Result<(), DmaTransferInitError> {
        let len = Self::common_mem_transfer_checks(source.len(), dest.len())?;
        self.generic_mem_to_mem_transfer_init(
            len,
            (source.as_ptr() as u32)
                .checked_add(len as u32 * core::mem::size_of::<u16>() as u32)
                .ok_or(DmaTransferInitError::AddrOverflow)?,
            (dest.as_ptr() as u32)
                .checked_add(len as u32 * core::mem::size_of::<u16>() as u32)
                .ok_or(DmaTransferInitError::AddrOverflow)?,
            DataSize::Halfword,
            AddrIncrement::Halfword,
        );
        Ok(())
    }

    /// Prepares a 32-bit DMA transfer from memory to memory.
    ///
    /// This function does not enable the DMA channel and interrupts and only prepares
    /// the DMA control block parameters for the transfer.
    ///
    /// You can use [Self::enable], [Self::enable_done_interrupt], [Self::enable_active_interrupt]
    /// to finish the transfer preparation and then use [Self::trigger_with_sw_request] to
    /// start the DMA transfer.
    pub fn prepare_mem_to_mem_transfer_32_bit(
        &mut self,
        source: &[u32],
        dest: &mut [u32],
    ) -> Result<(), DmaTransferInitError> {
        let len = Self::common_mem_transfer_checks(source.len(), dest.len())?;
        self.generic_mem_to_mem_transfer_init(
            len,
            (source.as_ptr() as u32)
                .checked_add(len as u32 * core::mem::size_of::<u32>() as u32)
                .ok_or(DmaTransferInitError::AddrOverflow)?,
            (dest.as_ptr() as u32)
                .checked_add(len as u32 * core::mem::size_of::<u32>() as u32)
                .ok_or(DmaTransferInitError::AddrOverflow)?,
            DataSize::Word,
            AddrIncrement::Word,
        );
        Ok(())
    }

    // This function performs common checks and returns the source length minus one which is
    // relevant for further configuration of the DMA. This is because the DMA API expects N minus
    // 1 and the source and end pointer need to point to the last transfer address.
    fn common_mem_transfer_checks(
        src_len: usize,
        dest_len: usize,
    ) -> Result<usize, DmaTransferInitError> {
        if src_len != dest_len {
            return Err(DmaTransferInitError::SourceDestLenMissmatch { src_len, dest_len });
        }
        if src_len > MAX_DMA_TRANSFERS_PER_CYCLE {
            return Err(DmaTransferInitError::TransferSizeTooLarge(src_len));
        }
        Ok(src_len - 1)
    }

    fn generic_mem_to_mem_transfer_init(
        &mut self,
        n_minus_one: usize,
        src_end_ptr: u32,
        dest_end_ptr: u32,
        data_size: DataSize,
        addr_incr: AddrIncrement,
    ) {
        self.ch_ctrl_pri.cfg.set_raw(0);
        self.ch_ctrl_pri.src_end_ptr = src_end_ptr;
        self.ch_ctrl_pri.dest_end_ptr = dest_end_ptr;
        self.ch_ctrl_pri.cfg.set_cycle_ctr(CycleControl::Auto as u8);
        self.ch_ctrl_pri.cfg.set_src_size(data_size as u8);
        self.ch_ctrl_pri.cfg.set_src_inc(addr_incr as u8);
        self.ch_ctrl_pri.cfg.set_dst_size(data_size as u8);
        self.ch_ctrl_pri.cfg.set_dst_inc(addr_incr as u8);
        self.ch_ctrl_pri.cfg.set_n_minus_1(n_minus_one as u16);
        self.ch_ctrl_pri.cfg.set_r_power(RPower::Every4 as u8);
    }
}

impl Dma {
    /// Create a new DMA instance.
    ///
    /// You can use [DmaCtrlBlock::new_at_addr] to create the DMA control block at a specific address.
    pub fn new(
        syscfg: &mut pac::Sysconfig,
        dma: pac::Dma,
        cfg: DmaCfg,
        ctrl_block: *mut DmaCtrlBlock,
    ) -> Result<Self, InvalidCtrlBlockAddr> {
        // The conversion to u32 is safe here because we are on a 32-bit system.
        let raw_addr = ctrl_block as u32;
        if raw_addr & BASE_PTR_ADDR_MASK > 0 {
            return Err(InvalidCtrlBlockAddr);
        }
        syscfg.enable_peripheral_clock(PeripheralClock::Dma);
        syscfg.assert_periph_reset_for_two_cycles(PeripheralSelect::Dma);
        let dma = Dma { dma, ctrl_block };
        dma.dma
            .ctrl_base_ptr()
            .write(|w| unsafe { w.bits(raw_addr) });
        dma.set_protection_bits(&cfg);
        dma.enable();
        Ok(dma)
    }

    #[inline(always)]
    pub fn enable(&self) {
        self.dma.cfg().write(|w| w.master_enable().set_bit());
    }

    #[inline(always)]
    pub fn disable(&self) {
        self.dma.cfg().write(|w| w.master_enable().clear_bit());
    }

    #[inline(always)]
    pub fn set_protection_bits(&self, cfg: &DmaCfg) {
        self.dma.cfg().write(|w| unsafe {
            w.chnl_prot_ctrl().bits(
                cfg.privileged as u8 | ((cfg.bufferable as u8) << 1) | ((cfg.cacheable as u8) << 2),
            )
        });
    }

    /// Split the DMA instance into four DMA channels which can be used individually. This allows
    /// using the inidividual DMA channels in separate tasks.
    pub fn split(self) -> (DmaChannel, DmaChannel, DmaChannel, DmaChannel) {
        // Safety: The DMA channel API only operates on its respective channels.
        (
            DmaChannel {
                idx: 0,
                done_interrupt: pac::Interrupt::DMA_DONE0,
                active_interrupt: pac::Interrupt::DMA_ACTIVE0,
                dma: unsafe { pac::Dma::steal() },
                ch_ctrl_pri: unsafe { &mut (*self.ctrl_block).pri[0] },
                ch_ctrl_alt: unsafe { &mut (*self.ctrl_block).alt[0] },
            },
            DmaChannel {
                idx: 1,
                done_interrupt: pac::Interrupt::DMA_DONE1,
                active_interrupt: pac::Interrupt::DMA_ACTIVE1,
                dma: unsafe { pac::Dma::steal() },
                ch_ctrl_pri: unsafe { &mut (*self.ctrl_block).pri[1] },
                ch_ctrl_alt: unsafe { &mut (*self.ctrl_block).alt[1] },
            },
            DmaChannel {
                idx: 2,
                done_interrupt: pac::Interrupt::DMA_DONE2,
                active_interrupt: pac::Interrupt::DMA_ACTIVE2,
                dma: unsafe { pac::Dma::steal() },
                ch_ctrl_pri: unsafe { &mut (*self.ctrl_block).pri[2] },
                ch_ctrl_alt: unsafe { &mut (*self.ctrl_block).alt[2] },
            },
            DmaChannel {
                idx: 3,
                done_interrupt: pac::Interrupt::DMA_DONE3,
                active_interrupt: pac::Interrupt::DMA_ACTIVE3,
                dma: unsafe { pac::Dma::steal() },
                ch_ctrl_pri: unsafe { &mut (*self.ctrl_block).pri[3] },
                ch_ctrl_alt: unsafe { &mut (*self.ctrl_block).alt[3] },
            },
        )
    }
}
