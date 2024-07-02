use crate::{
    clock::{PeripheralClock, PeripheralSelect},
    pac,
    prelude::*,
};

/*
/** DMA channel config struct */
typedef struct stc_dma_chnl_cfg
{
  uint32_t cycle_ctrl:    3;
  uint32_t next_useburst: 1;
  uint32_t n_minus_1:    10;
  uint32_t r_power:       4;
  uint32_t src_prot_ctrl: 3;
  uint32_t dst_prot_ctrl: 3;
  uint32_t src_size:      2;
  uint32_t src_inc:       2;
  uint32_t dst_size:      2;
  uint32_t dst_inc:       2;
} stc_dma_chnl_cfg_t;

/** DMA channel struct */
typedef struct stc_dma_chnl
{
  /* Note: This DMA engine expects source, dest pointers need to point to
       the LAST element, not the first */
  uint32_t src;    // source *END* pointer - addr of last elememt in source
  uint32_t dst;    // destination *END* pointer - addr of last element in dest
  union{
    uint32_t           ctrl_raw; // Control register raw val
    stc_dma_chnl_cfg_t ctrl;     // Control register bitfield
  };
  uint32_t padding; // unused
} stc_dma_chnl_t;

/** DMA control block data structure */
typedef struct stc_dma_control_blk
{
  stc_dma_chnl_t pri[DMA_NUM_CHNLS]; // primary ch   0x00 to 0x3f
  stc_dma_chnl_t alt[DMA_NUM_CHNLS]; // alternate ch 0x40 to 0x7f
} stc_dma_control_blk_t;
*/

bitfield::bitfield! {
    pub struct ChannelConfig(u32);
    impl Debug;
    dst_inc, set_dst_inc: 31, 30;
    dst_size, set_dst_size: 29, 28;
    src_inc, set_src_inc: 27, 26;
    src_size, set_src_size: 25, 24;
    dest_prot_ctrl, set_dest_prot_ctrl: 23, 21;
    src_prot_ctrl, set_src_prot_ctrl: 20, 18;
    r_power, set_r_power: 17, 14;
    n_minus_1, set_n_minus_1: 13, 4;
    next_useburst, set_next_useburst: 3;
}

#[repr(C)]
pub struct DmaChannelControl {
    pub src_end_ptr: u32,
    pub dest_end_ptr: u32,
    pub cfg: ChannelConfig,
    padding: u32,
}

#[repr(C)]
pub struct DmaCtrlBlock {
    pub pri: [DmaChannelControl; 4],
    pub alt: [DmaChannelControl; 4],
}

pub struct Dma {
    dma: pac::Dma,
    ctrl_block: &'static DmaCtrlBlock,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct DmaCfg {
    pub bufferable: bool,
    pub cacheable: bool,
    pub privileged: bool,
}

pub struct DmaChannel {
    idx: u8,
    dma: pac::Dma,
    ch_ctrl_pri: &'static DmaChannelControl,
    ch_ctrl_alt: &'static DmaChannelControl,
}

impl DmaChannel {}

#[derive(Debug, PartialEq, Eq)]
pub struct InvalidCtrlBlockAddr;

impl Dma {
    /// Create a new DMA instance.
    ///
    /// The user must ensure that the DMA control block is placed statically in some memory
    /// which can be accessed by the system as well, for example the SRAM1 block.
    pub fn new(
        syscfg: &mut pac::Sysconfig,
        dma: pac::Dma,
        cfg: DmaCfg,
        ctrl_block: &'static DmaCtrlBlock,
    ) -> Result<Self, InvalidCtrlBlockAddr> {
        syscfg.enable_peripheral_clock(PeripheralClock::Dma);
        syscfg.assert_periph_reset_for_two_cycles(PeripheralSelect::Dma);
        syscfg.enable_peripheral_clock(PeripheralClock::IrqRouter);
        syscfg.assert_periph_reset_for_two_cycles(PeripheralSelect::IrqRouter);
        let dma = Dma { dma, ctrl_block };
        dma.set_protection_bits(&cfg);
        dma.enable();
        let raw_addr = core::ptr::addr_of!(ctrl_block) as *const _ as u32;
        if raw_addr % 128 != 0 {
            return Err(InvalidCtrlBlockAddr);
        }
        // The conversion to u32 is safe here because we are on a 32-bit system.
        dma.dma
            .ctrl_base_ptr()
            .write(|w| unsafe { w.ctrl_base_ptr().bits(raw_addr) });
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

    /// Split the DMA instance into four DMA channels which can be used individually.
    pub fn split(self) -> (DmaChannel, DmaChannel, DmaChannel, DmaChannel) {
        (
            DmaChannel {
                idx: 0,
                dma: unsafe { pac::Dma::steal() },
                ch_ctrl_pri: &self.ctrl_block.pri[0],
                ch_ctrl_alt: &self.ctrl_block.alt[0],
            },
            DmaChannel {
                idx: 1,
                dma: unsafe { pac::Dma::steal() },
                ch_ctrl_pri: &self.ctrl_block.pri[1],
                ch_ctrl_alt: &self.ctrl_block.alt[1],
            },
            DmaChannel {
                idx: 2,
                dma: unsafe { pac::Dma::steal() },
                ch_ctrl_pri: &self.ctrl_block.pri[2],
                ch_ctrl_alt: &self.ctrl_block.alt[2],
            },
            DmaChannel {
                idx: 3,
                dma: unsafe { pac::Dma::steal() },
                ch_ctrl_pri: &self.ctrl_block.pri[3],
                ch_ctrl_alt: &self.ctrl_block.alt[3],
            },
        )
    }
}
