#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register 0"]
    pub ctrl0: crate::Reg<ctrl0::CTRL0_SPEC>,
    #[doc = "0x04 - Control Register 1"]
    pub ctrl1: crate::Reg<ctrl1::CTRL1_SPEC>,
    #[doc = "0x08 - Data Input/Output"]
    pub data: crate::Reg<data::DATA_SPEC>,
    #[doc = "0x0c - Status Register"]
    pub status: crate::Reg<status::STATUS_SPEC>,
    #[doc = "0x10 - Clock Pre Scale divide value"]
    pub clkprescale: crate::Reg<clkprescale::CLKPRESCALE_SPEC>,
    #[doc = "0x14 - Interrupt Enable Register"]
    pub irq_enb: crate::Reg<irq_enb::IRQ_ENB_SPEC>,
    #[doc = "0x18 - Raw Interrupt Status Register"]
    pub irq_raw: crate::Reg<irq_raw::IRQ_RAW_SPEC>,
    #[doc = "0x1c - Enabled Interrupt Status Register"]
    pub irq_end: crate::Reg<irq_end::IRQ_END_SPEC>,
    #[doc = "0x20 - Clear Interrupt Status Register"]
    pub irq_clr: crate::Reg<irq_clr::IRQ_CLR_SPEC>,
    #[doc = "0x24 - Rx FIFO IRQ Trigger Level"]
    pub rxfifoirqtrg: crate::Reg<rxfifoirqtrg::RXFIFOIRQTRG_SPEC>,
    #[doc = "0x28 - Tx FIFO IRQ Trigger Level"]
    pub txfifoirqtrg: crate::Reg<txfifoirqtrg::TXFIFOIRQTRG_SPEC>,
    #[doc = "0x2c - Clear FIFO Register"]
    pub fifo_clr: crate::Reg<fifo_clr::FIFO_CLR_SPEC>,
    #[doc = "0x30 - Internal STATE of SPI Controller"]
    pub state: crate::Reg<state::STATE_SPEC>,
    _reserved13: [u8; 0x03c8],
    #[doc = "0x3fc - Peripheral ID Register"]
    pub perid: crate::Reg<perid::PERID_SPEC>,
}
#[doc = "CTRL0 register accessor: an alias for `Reg<CTRL0_SPEC>`"]
pub type CTRL0 = crate::Reg<ctrl0::CTRL0_SPEC>;
#[doc = "Control Register 0"]
pub mod ctrl0;
#[doc = "CTRL1 register accessor: an alias for `Reg<CTRL1_SPEC>`"]
pub type CTRL1 = crate::Reg<ctrl1::CTRL1_SPEC>;
#[doc = "Control Register 1"]
pub mod ctrl1;
#[doc = "DATA register accessor: an alias for `Reg<DATA_SPEC>`"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "Data Input/Output"]
pub mod data;
#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status Register"]
pub mod status;
#[doc = "CLKPRESCALE register accessor: an alias for `Reg<CLKPRESCALE_SPEC>`"]
pub type CLKPRESCALE = crate::Reg<clkprescale::CLKPRESCALE_SPEC>;
#[doc = "Clock Pre Scale divide value"]
pub mod clkprescale;
#[doc = "IRQ_ENB register accessor: an alias for `Reg<IRQ_ENB_SPEC>`"]
pub type IRQ_ENB = crate::Reg<irq_enb::IRQ_ENB_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod irq_enb;
#[doc = "IRQ_RAW register accessor: an alias for `Reg<IRQ_RAW_SPEC>`"]
pub type IRQ_RAW = crate::Reg<irq_raw::IRQ_RAW_SPEC>;
#[doc = "Raw Interrupt Status Register"]
pub mod irq_raw;
#[doc = "IRQ_END register accessor: an alias for `Reg<IRQ_END_SPEC>`"]
pub type IRQ_END = crate::Reg<irq_end::IRQ_END_SPEC>;
#[doc = "Enabled Interrupt Status Register"]
pub mod irq_end;
#[doc = "IRQ_CLR register accessor: an alias for `Reg<IRQ_CLR_SPEC>`"]
pub type IRQ_CLR = crate::Reg<irq_clr::IRQ_CLR_SPEC>;
#[doc = "Clear Interrupt Status Register"]
pub mod irq_clr;
#[doc = "RXFIFOIRQTRG register accessor: an alias for `Reg<RXFIFOIRQTRG_SPEC>`"]
pub type RXFIFOIRQTRG = crate::Reg<rxfifoirqtrg::RXFIFOIRQTRG_SPEC>;
#[doc = "Rx FIFO IRQ Trigger Level"]
pub mod rxfifoirqtrg;
#[doc = "TXFIFOIRQTRG register accessor: an alias for `Reg<TXFIFOIRQTRG_SPEC>`"]
pub type TXFIFOIRQTRG = crate::Reg<txfifoirqtrg::TXFIFOIRQTRG_SPEC>;
#[doc = "Tx FIFO IRQ Trigger Level"]
pub mod txfifoirqtrg;
#[doc = "FIFO_CLR register accessor: an alias for `Reg<FIFO_CLR_SPEC>`"]
pub type FIFO_CLR = crate::Reg<fifo_clr::FIFO_CLR_SPEC>;
#[doc = "Clear FIFO Register"]
pub mod fifo_clr;
#[doc = "STATE register accessor: an alias for `Reg<STATE_SPEC>`"]
pub type STATE = crate::Reg<state::STATE_SPEC>;
#[doc = "Internal STATE of SPI Controller"]
pub mod state;
#[doc = "PERID register accessor: an alias for `Reg<PERID_SPEC>`"]
pub type PERID = crate::Reg<perid::PERID_SPEC>;
#[doc = "Peripheral ID Register"]
pub mod perid;
