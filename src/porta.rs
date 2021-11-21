#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_datain: [u8; 0x04],
    _reserved_1_datainraw: [u8; 0x04],
    _reserved_2_dataout: [u8; 0x04],
    _reserved_3_dataoutraw: [u8; 0x04],
    _reserved_4_setout: [u8; 0x04],
    _reserved_5_clrout: [u8; 0x04],
    _reserved_6_togout: [u8; 0x04],
    _reserved_7_datamask: [u8; 0x04],
    _reserved_8_dir: [u8; 0x04],
    _reserved_9_pulse: [u8; 0x04],
    _reserved_10_pulsebase: [u8; 0x04],
    _reserved_11_delay: [u8; 0x04],
    _reserved_12_delay: [u8; 0x04],
    #[doc = "0x34 - Interrupt Sense Register (1:Level Sensitive, 0:Edge Sensitive)"]
    pub irq_sen: crate::Reg<irq_sen::IRQ_SEN_SPEC>,
    #[doc = "0x38 - Interrupt Both Edge Register (1:Both Edges, 0:Single Edge)"]
    pub irq_edge: crate::Reg<irq_edge::IRQ_EDGE_SPEC>,
    #[doc = "0x3c - Interrupt Event Register (1:HighLevel/L->H Edge, 0:LowLevel/H->L Edge)"]
    pub irq_evt: crate::Reg<irq_evt::IRQ_EVT_SPEC>,
    #[doc = "0x40 - Interrupt Enable Register"]
    pub irq_enb: crate::Reg<irq_enb::IRQ_ENB_SPEC>,
    #[doc = "0x44 - Raw Interrupt Status"]
    pub irq_raw: crate::Reg<irq_raw::IRQ_RAW_SPEC>,
    #[doc = "0x48 - Masked Interrupt Status"]
    pub irq_end: crate::Reg<irq_end::IRQ_END_SPEC>,
    #[doc = "0x4c - Edge Status Register"]
    pub edge_status: crate::Reg<edge_status::EDGE_STATUS_SPEC>,
    _reserved20: [u8; 0x03ac],
    #[doc = "0x3fc - Peripheral ID Register"]
    pub perid: crate::Reg<perid::PERID_SPEC>,
}
impl RegisterBlock {
    #[doc = "0x00 - Data In Register by Byte"]
    #[inline(always)]
    pub fn datainbyte(&self) -> &[crate::Reg<datainbyte::DATAINBYTE_SPEC>; 4] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(0usize)
                as *const [crate::Reg<datainbyte::DATAINBYTE_SPEC>; 4])
        }
    }
    #[doc = "0x00 - Data In Register"]
    #[inline(always)]
    pub fn datain(&self) -> &crate::Reg<datain::DATAIN_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(0usize)
                as *const crate::Reg<datain::DATAIN_SPEC>)
        }
    }
    #[doc = "0x04 - Data In Raw Register by Byte"]
    #[inline(always)]
    pub fn datainrawbyte(&self) -> &[crate::Reg<datainrawbyte::DATAINRAWBYTE_SPEC>; 4] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4usize)
                as *const [crate::Reg<datainrawbyte::DATAINRAWBYTE_SPEC>; 4])
        }
    }
    #[doc = "0x04 - Data In Raw Register"]
    #[inline(always)]
    pub fn datainraw(&self) -> &crate::Reg<datainraw::DATAINRAW_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4usize)
                as *const crate::Reg<datainraw::DATAINRAW_SPEC>)
        }
    }
    #[doc = "0x08 - Data Out Register by Byte"]
    #[inline(always)]
    pub fn dataoutbyte(&self) -> &[crate::Reg<dataoutbyte::DATAOUTBYTE_SPEC>; 4] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(8usize)
                as *const [crate::Reg<dataoutbyte::DATAOUTBYTE_SPEC>; 4])
        }
    }
    #[doc = "0x08 - Data Out Register"]
    #[inline(always)]
    pub fn dataout(&self) -> &crate::Reg<dataout::DATAOUT_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(8usize)
                as *const crate::Reg<dataout::DATAOUT_SPEC>)
        }
    }
    #[doc = "0x0c - Data Out Register by Byte"]
    #[inline(always)]
    pub fn dataoutrawbyte(&self) -> &[crate::Reg<dataoutrawbyte::DATAOUTRAWBYTE_SPEC>; 4] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(12usize)
                as *const [crate::Reg<dataoutrawbyte::DATAOUTRAWBYTE_SPEC>; 4])
        }
    }
    #[doc = "0x0c - Data Out Register"]
    #[inline(always)]
    pub fn dataoutraw(&self) -> &crate::Reg<dataoutraw::DATAOUTRAW_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(12usize)
                as *const crate::Reg<dataoutraw::DATAOUTRAW_SPEC>)
        }
    }
    #[doc = "0x10 - Set Out Register by Byte"]
    #[inline(always)]
    pub fn setoutbyte(&self) -> &[crate::Reg<setoutbyte::SETOUTBYTE_SPEC>; 4] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(16usize)
                as *const [crate::Reg<setoutbyte::SETOUTBYTE_SPEC>; 4])
        }
    }
    #[doc = "0x10 - Set Out Register"]
    #[inline(always)]
    pub fn setout(&self) -> &crate::Reg<setout::SETOUT_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(16usize)
                as *const crate::Reg<setout::SETOUT_SPEC>)
        }
    }
    #[doc = "0x14 - Clear Out Register by Byte"]
    #[inline(always)]
    pub fn clroutbyte(&self) -> &[crate::Reg<clroutbyte::CLROUTBYTE_SPEC>; 4] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(20usize)
                as *const [crate::Reg<clroutbyte::CLROUTBYTE_SPEC>; 4])
        }
    }
    #[doc = "0x14 - Clear Out Register"]
    #[inline(always)]
    pub fn clrout(&self) -> &crate::Reg<clrout::CLROUT_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(20usize)
                as *const crate::Reg<clrout::CLROUT_SPEC>)
        }
    }
    #[doc = "0x18 - Toggle Out Register by Byte"]
    #[inline(always)]
    pub fn togoutbyte(&self) -> &[crate::Reg<togoutbyte::TOGOUTBYTE_SPEC>; 4] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(24usize)
                as *const [crate::Reg<togoutbyte::TOGOUTBYTE_SPEC>; 4])
        }
    }
    #[doc = "0x18 - Toggle Out Register"]
    #[inline(always)]
    pub fn togout(&self) -> &crate::Reg<togout::TOGOUT_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(24usize)
                as *const crate::Reg<togout::TOGOUT_SPEC>)
        }
    }
    #[doc = "0x1c - Data Out Register by Byte"]
    #[inline(always)]
    pub fn datamaskbyte(&self) -> &[crate::Reg<datamaskbyte::DATAMASKBYTE_SPEC>; 4] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(28usize)
                as *const [crate::Reg<datamaskbyte::DATAMASKBYTE_SPEC>; 4])
        }
    }
    #[doc = "0x1c - Data mask Register"]
    #[inline(always)]
    pub fn datamask(&self) -> &crate::Reg<datamask::DATAMASK_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(28usize)
                as *const crate::Reg<datamask::DATAMASK_SPEC>)
        }
    }
    #[doc = "0x20 - Direction Register by Byte"]
    #[inline(always)]
    pub fn dirbyte(&self) -> &[crate::Reg<dirbyte::DIRBYTE_SPEC>; 4] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(32usize)
                as *const [crate::Reg<dirbyte::DIRBYTE_SPEC>; 4])
        }
    }
    #[doc = "0x20 - Direction Register (1:Output, 0:Input)"]
    #[inline(always)]
    pub fn dir(&self) -> &crate::Reg<dir::DIR_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(32usize)
                as *const crate::Reg<dir::DIR_SPEC>)
        }
    }
    #[doc = "0x24 - Pulse Mode Register by Byte"]
    #[inline(always)]
    pub fn pulsebyte(&self) -> &[crate::Reg<pulsebyte::PULSEBYTE_SPEC>; 4] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(36usize)
                as *const [crate::Reg<pulsebyte::PULSEBYTE_SPEC>; 4])
        }
    }
    #[doc = "0x24 - Pulse Mode Register"]
    #[inline(always)]
    pub fn pulse(&self) -> &crate::Reg<pulse::PULSE_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(36usize)
                as *const crate::Reg<pulse::PULSE_SPEC>)
        }
    }
    #[doc = "0x28 - Pulse Base Mode Register by Byte"]
    #[inline(always)]
    pub fn pulsebasebyte(&self) -> &[crate::Reg<pulsebasebyte::PULSEBASEBYTE_SPEC>; 4] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(40usize)
                as *const [crate::Reg<pulsebasebyte::PULSEBASEBYTE_SPEC>; 4])
        }
    }
    #[doc = "0x28 - Pulse Base Value Register"]
    #[inline(always)]
    pub fn pulsebase(&self) -> &crate::Reg<pulsebase::PULSEBASE_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(40usize)
                as *const crate::Reg<pulsebase::PULSEBASE_SPEC>)
        }
    }
    #[doc = "0x2c - Delay1 Register by Byte"]
    #[inline(always)]
    pub fn delay1byte(&self) -> &[crate::Reg<delay1byte::DELAY1BYTE_SPEC>; 4] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(44usize)
                as *const [crate::Reg<delay1byte::DELAY1BYTE_SPEC>; 4])
        }
    }
    #[doc = "0x2c - Delay1 Register"]
    #[inline(always)]
    pub fn delay1(&self) -> &crate::Reg<delay1::DELAY1_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(44usize)
                as *const crate::Reg<delay1::DELAY1_SPEC>)
        }
    }
    #[doc = "0x30 - Delay2 Register by Byte"]
    #[inline(always)]
    pub fn delay2byte(&self) -> &[crate::Reg<delay2byte::DELAY2BYTE_SPEC>; 4] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(48usize)
                as *const [crate::Reg<delay2byte::DELAY2BYTE_SPEC>; 4])
        }
    }
    #[doc = "0x30 - Delay2 Register"]
    #[inline(always)]
    pub fn delay2(&self) -> &crate::Reg<delay2::DELAY2_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(48usize)
                as *const crate::Reg<delay2::DELAY2_SPEC>)
        }
    }
}
#[doc = "DATAIN register accessor: an alias for `Reg<DATAIN_SPEC>`"]
pub type DATAIN = crate::Reg<datain::DATAIN_SPEC>;
#[doc = "Data In Register"]
pub mod datain;
#[doc = "DATAINBYTE register accessor: an alias for `Reg<DATAINBYTE_SPEC>`"]
pub type DATAINBYTE = crate::Reg<datainbyte::DATAINBYTE_SPEC>;
#[doc = "Data In Register by Byte"]
pub mod datainbyte;
#[doc = "DATAINRAW register accessor: an alias for `Reg<DATAINRAW_SPEC>`"]
pub type DATAINRAW = crate::Reg<datainraw::DATAINRAW_SPEC>;
#[doc = "Data In Raw Register"]
pub mod datainraw;
#[doc = "DATAINRAWBYTE register accessor: an alias for `Reg<DATAINRAWBYTE_SPEC>`"]
pub type DATAINRAWBYTE = crate::Reg<datainrawbyte::DATAINRAWBYTE_SPEC>;
#[doc = "Data In Raw Register by Byte"]
pub mod datainrawbyte;
#[doc = "DATAOUT register accessor: an alias for `Reg<DATAOUT_SPEC>`"]
pub type DATAOUT = crate::Reg<dataout::DATAOUT_SPEC>;
#[doc = "Data Out Register"]
pub mod dataout;
#[doc = "DATAOUTBYTE register accessor: an alias for `Reg<DATAOUTBYTE_SPEC>`"]
pub type DATAOUTBYTE = crate::Reg<dataoutbyte::DATAOUTBYTE_SPEC>;
#[doc = "Data Out Register by Byte"]
pub mod dataoutbyte;
#[doc = "DATAOUTRAW register accessor: an alias for `Reg<DATAOUTRAW_SPEC>`"]
pub type DATAOUTRAW = crate::Reg<dataoutraw::DATAOUTRAW_SPEC>;
#[doc = "Data Out Register"]
pub mod dataoutraw;
#[doc = "DATAOUTRAWBYTE register accessor: an alias for `Reg<DATAOUTRAWBYTE_SPEC>`"]
pub type DATAOUTRAWBYTE = crate::Reg<dataoutrawbyte::DATAOUTRAWBYTE_SPEC>;
#[doc = "Data Out Register by Byte"]
pub mod dataoutrawbyte;
#[doc = "SETOUT register accessor: an alias for `Reg<SETOUT_SPEC>`"]
pub type SETOUT = crate::Reg<setout::SETOUT_SPEC>;
#[doc = "Set Out Register"]
pub mod setout;
#[doc = "SETOUTBYTE register accessor: an alias for `Reg<SETOUTBYTE_SPEC>`"]
pub type SETOUTBYTE = crate::Reg<setoutbyte::SETOUTBYTE_SPEC>;
#[doc = "Set Out Register by Byte"]
pub mod setoutbyte;
#[doc = "CLROUT register accessor: an alias for `Reg<CLROUT_SPEC>`"]
pub type CLROUT = crate::Reg<clrout::CLROUT_SPEC>;
#[doc = "Clear Out Register"]
pub mod clrout;
#[doc = "CLROUTBYTE register accessor: an alias for `Reg<CLROUTBYTE_SPEC>`"]
pub type CLROUTBYTE = crate::Reg<clroutbyte::CLROUTBYTE_SPEC>;
#[doc = "Clear Out Register by Byte"]
pub mod clroutbyte;
#[doc = "TOGOUT register accessor: an alias for `Reg<TOGOUT_SPEC>`"]
pub type TOGOUT = crate::Reg<togout::TOGOUT_SPEC>;
#[doc = "Toggle Out Register"]
pub mod togout;
#[doc = "TOGOUTBYTE register accessor: an alias for `Reg<TOGOUTBYTE_SPEC>`"]
pub type TOGOUTBYTE = crate::Reg<togoutbyte::TOGOUTBYTE_SPEC>;
#[doc = "Toggle Out Register by Byte"]
pub mod togoutbyte;
#[doc = "DATAMASK register accessor: an alias for `Reg<DATAMASK_SPEC>`"]
pub type DATAMASK = crate::Reg<datamask::DATAMASK_SPEC>;
#[doc = "Data mask Register"]
pub mod datamask;
#[doc = "DATAMASKBYTE register accessor: an alias for `Reg<DATAMASKBYTE_SPEC>`"]
pub type DATAMASKBYTE = crate::Reg<datamaskbyte::DATAMASKBYTE_SPEC>;
#[doc = "Data Out Register by Byte"]
pub mod datamaskbyte;
#[doc = "DIR register accessor: an alias for `Reg<DIR_SPEC>`"]
pub type DIR = crate::Reg<dir::DIR_SPEC>;
#[doc = "Direction Register (1:Output, 0:Input)"]
pub mod dir;
#[doc = "DIRBYTE register accessor: an alias for `Reg<DIRBYTE_SPEC>`"]
pub type DIRBYTE = crate::Reg<dirbyte::DIRBYTE_SPEC>;
#[doc = "Direction Register by Byte"]
pub mod dirbyte;
#[doc = "PULSE register accessor: an alias for `Reg<PULSE_SPEC>`"]
pub type PULSE = crate::Reg<pulse::PULSE_SPEC>;
#[doc = "Pulse Mode Register"]
pub mod pulse;
#[doc = "PULSEBYTE register accessor: an alias for `Reg<PULSEBYTE_SPEC>`"]
pub type PULSEBYTE = crate::Reg<pulsebyte::PULSEBYTE_SPEC>;
#[doc = "Pulse Mode Register by Byte"]
pub mod pulsebyte;
#[doc = "PULSEBASE register accessor: an alias for `Reg<PULSEBASE_SPEC>`"]
pub type PULSEBASE = crate::Reg<pulsebase::PULSEBASE_SPEC>;
#[doc = "Pulse Base Value Register"]
pub mod pulsebase;
#[doc = "PULSEBASEBYTE register accessor: an alias for `Reg<PULSEBASEBYTE_SPEC>`"]
pub type PULSEBASEBYTE = crate::Reg<pulsebasebyte::PULSEBASEBYTE_SPEC>;
#[doc = "Pulse Base Mode Register by Byte"]
pub mod pulsebasebyte;
#[doc = "DELAY1 register accessor: an alias for `Reg<DELAY1_SPEC>`"]
pub type DELAY1 = crate::Reg<delay1::DELAY1_SPEC>;
#[doc = "Delay1 Register"]
pub mod delay1;
#[doc = "DELAY1BYTE register accessor: an alias for `Reg<DELAY1BYTE_SPEC>`"]
pub type DELAY1BYTE = crate::Reg<delay1byte::DELAY1BYTE_SPEC>;
#[doc = "Delay1 Register by Byte"]
pub mod delay1byte;
#[doc = "DELAY2 register accessor: an alias for `Reg<DELAY2_SPEC>`"]
pub type DELAY2 = crate::Reg<delay2::DELAY2_SPEC>;
#[doc = "Delay2 Register"]
pub mod delay2;
#[doc = "DELAY2BYTE register accessor: an alias for `Reg<DELAY2BYTE_SPEC>`"]
pub type DELAY2BYTE = crate::Reg<delay2byte::DELAY2BYTE_SPEC>;
#[doc = "Delay2 Register by Byte"]
pub mod delay2byte;
#[doc = "IRQ_SEN register accessor: an alias for `Reg<IRQ_SEN_SPEC>`"]
pub type IRQ_SEN = crate::Reg<irq_sen::IRQ_SEN_SPEC>;
#[doc = "Interrupt Sense Register (1:Level Sensitive, 0:Edge Sensitive)"]
pub mod irq_sen;
#[doc = "IRQ_EDGE register accessor: an alias for `Reg<IRQ_EDGE_SPEC>`"]
pub type IRQ_EDGE = crate::Reg<irq_edge::IRQ_EDGE_SPEC>;
#[doc = "Interrupt Both Edge Register (1:Both Edges, 0:Single Edge)"]
pub mod irq_edge;
#[doc = "IRQ_EVT register accessor: an alias for `Reg<IRQ_EVT_SPEC>`"]
pub type IRQ_EVT = crate::Reg<irq_evt::IRQ_EVT_SPEC>;
#[doc = "Interrupt Event Register (1:HighLevel/L->H Edge, 0:LowLevel/H->L Edge)"]
pub mod irq_evt;
#[doc = "IRQ_ENB register accessor: an alias for `Reg<IRQ_ENB_SPEC>`"]
pub type IRQ_ENB = crate::Reg<irq_enb::IRQ_ENB_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod irq_enb;
#[doc = "IRQ_RAW register accessor: an alias for `Reg<IRQ_RAW_SPEC>`"]
pub type IRQ_RAW = crate::Reg<irq_raw::IRQ_RAW_SPEC>;
#[doc = "Raw Interrupt Status"]
pub mod irq_raw;
#[doc = "IRQ_END register accessor: an alias for `Reg<IRQ_END_SPEC>`"]
pub type IRQ_END = crate::Reg<irq_end::IRQ_END_SPEC>;
#[doc = "Masked Interrupt Status"]
pub mod irq_end;
#[doc = "EDGE_STATUS register accessor: an alias for `Reg<EDGE_STATUS_SPEC>`"]
pub type EDGE_STATUS = crate::Reg<edge_status::EDGE_STATUS_SPEC>;
#[doc = "Edge Status Register"]
pub mod edge_status;
#[doc = "PERID register accessor: an alias for `Reg<PERID_SPEC>`"]
pub type PERID = crate::Reg<perid::PERID_SPEC>;
#[doc = "Peripheral ID Register"]
pub mod perid;
