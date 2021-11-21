#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Data In/Out Register"]
    pub data: crate::Reg<data::DATA_SPEC>,
    #[doc = "0x04 - Enable Register"]
    pub enable: crate::Reg<enable::ENABLE_SPEC>,
    #[doc = "0x08 - Control Register"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    #[doc = "0x0c - Clock Scale Register"]
    pub clkscale: crate::Reg<clkscale::CLKSCALE_SPEC>,
    #[doc = "0x10 - Status Register"]
    pub rxstatus: crate::Reg<rxstatus::RXSTATUS_SPEC>,
    #[doc = "0x14 - Status Register"]
    pub txstatus: crate::Reg<txstatus::TXSTATUS_SPEC>,
    #[doc = "0x18 - Clear FIFO Register"]
    pub fifo_clr: crate::Reg<fifo_clr::FIFO_CLR_SPEC>,
    #[doc = "0x1c - Break Transmit Register"]
    pub txbreak: crate::Reg<txbreak::TXBREAK_SPEC>,
    #[doc = "0x20 - Address9 Register"]
    pub addr9: crate::Reg<addr9::ADDR9_SPEC>,
    #[doc = "0x24 - Address9 Mask Register"]
    pub addr9mask: crate::Reg<addr9mask::ADDR9MASK_SPEC>,
    #[doc = "0x28 - IRQ Enable Register"]
    pub irq_enb: crate::Reg<irq_enb::IRQ_ENB_SPEC>,
    #[doc = "0x2c - IRQ Raw Status Register"]
    pub irq_raw: crate::Reg<irq_raw::IRQ_RAW_SPEC>,
    #[doc = "0x30 - IRQ Enabled Status Register"]
    pub irq_end: crate::Reg<irq_end::IRQ_END_SPEC>,
    #[doc = "0x34 - IRQ Clear Status Register"]
    pub irq_clr: crate::Reg<irq_clr::IRQ_CLR_SPEC>,
    #[doc = "0x38 - Rx FIFO IRQ Trigger Level"]
    pub rxfifoirqtrg: crate::Reg<rxfifoirqtrg::RXFIFOIRQTRG_SPEC>,
    #[doc = "0x3c - Tx FIFO IRQ Trigger Level"]
    pub txfifoirqtrg: crate::Reg<txfifoirqtrg::TXFIFOIRQTRG_SPEC>,
    #[doc = "0x40 - Rx FIFO RTS Trigger Level"]
    pub rxfifortstrg: crate::Reg<rxfifortstrg::RXFIFORTSTRG_SPEC>,
    #[doc = "0x44 - Internal STATE of UART Controller"]
    pub state: crate::Reg<state::STATE_SPEC>,
    _reserved18: [u8; 0x0fb4],
    #[doc = "0xffc - Peripheral ID Register"]
    pub perid: crate::Reg<perid::PERID_SPEC>,
}
#[doc = "DATA register accessor: an alias for `Reg<DATA_SPEC>`"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "Data In/Out Register"]
pub mod data;
#[doc = "ENABLE register accessor: an alias for `Reg<ENABLE_SPEC>`"]
pub type ENABLE = crate::Reg<enable::ENABLE_SPEC>;
#[doc = "Enable Register"]
pub mod enable;
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "CLKSCALE register accessor: an alias for `Reg<CLKSCALE_SPEC>`"]
pub type CLKSCALE = crate::Reg<clkscale::CLKSCALE_SPEC>;
#[doc = "Clock Scale Register"]
pub mod clkscale;
#[doc = "RXSTATUS register accessor: an alias for `Reg<RXSTATUS_SPEC>`"]
pub type RXSTATUS = crate::Reg<rxstatus::RXSTATUS_SPEC>;
#[doc = "Status Register"]
pub mod rxstatus;
#[doc = "TXSTATUS register accessor: an alias for `Reg<TXSTATUS_SPEC>`"]
pub type TXSTATUS = crate::Reg<txstatus::TXSTATUS_SPEC>;
#[doc = "Status Register"]
pub mod txstatus;
#[doc = "FIFO_CLR register accessor: an alias for `Reg<FIFO_CLR_SPEC>`"]
pub type FIFO_CLR = crate::Reg<fifo_clr::FIFO_CLR_SPEC>;
#[doc = "Clear FIFO Register"]
pub mod fifo_clr;
#[doc = "TXBREAK register accessor: an alias for `Reg<TXBREAK_SPEC>`"]
pub type TXBREAK = crate::Reg<txbreak::TXBREAK_SPEC>;
#[doc = "Break Transmit Register"]
pub mod txbreak;
#[doc = "ADDR9 register accessor: an alias for `Reg<ADDR9_SPEC>`"]
pub type ADDR9 = crate::Reg<addr9::ADDR9_SPEC>;
#[doc = "Address9 Register"]
pub mod addr9;
#[doc = "ADDR9MASK register accessor: an alias for `Reg<ADDR9MASK_SPEC>`"]
pub type ADDR9MASK = crate::Reg<addr9mask::ADDR9MASK_SPEC>;
#[doc = "Address9 Mask Register"]
pub mod addr9mask;
#[doc = "IRQ_ENB register accessor: an alias for `Reg<IRQ_ENB_SPEC>`"]
pub type IRQ_ENB = crate::Reg<irq_enb::IRQ_ENB_SPEC>;
#[doc = "IRQ Enable Register"]
pub mod irq_enb;
#[doc = "IRQ_RAW register accessor: an alias for `Reg<IRQ_RAW_SPEC>`"]
pub type IRQ_RAW = crate::Reg<irq_raw::IRQ_RAW_SPEC>;
#[doc = "IRQ Raw Status Register"]
pub mod irq_raw;
#[doc = "IRQ_END register accessor: an alias for `Reg<IRQ_END_SPEC>`"]
pub type IRQ_END = crate::Reg<irq_end::IRQ_END_SPEC>;
#[doc = "IRQ Enabled Status Register"]
pub mod irq_end;
#[doc = "IRQ_CLR register accessor: an alias for `Reg<IRQ_CLR_SPEC>`"]
pub type IRQ_CLR = crate::Reg<irq_clr::IRQ_CLR_SPEC>;
#[doc = "IRQ Clear Status Register"]
pub mod irq_clr;
#[doc = "RXFIFOIRQTRG register accessor: an alias for `Reg<RXFIFOIRQTRG_SPEC>`"]
pub type RXFIFOIRQTRG = crate::Reg<rxfifoirqtrg::RXFIFOIRQTRG_SPEC>;
#[doc = "Rx FIFO IRQ Trigger Level"]
pub mod rxfifoirqtrg;
#[doc = "TXFIFOIRQTRG register accessor: an alias for `Reg<TXFIFOIRQTRG_SPEC>`"]
pub type TXFIFOIRQTRG = crate::Reg<txfifoirqtrg::TXFIFOIRQTRG_SPEC>;
#[doc = "Tx FIFO IRQ Trigger Level"]
pub mod txfifoirqtrg;
#[doc = "RXFIFORTSTRG register accessor: an alias for `Reg<RXFIFORTSTRG_SPEC>`"]
pub type RXFIFORTSTRG = crate::Reg<rxfifortstrg::RXFIFORTSTRG_SPEC>;
#[doc = "Rx FIFO RTS Trigger Level"]
pub mod rxfifortstrg;
#[doc = "STATE register accessor: an alias for `Reg<STATE_SPEC>`"]
pub type STATE = crate::Reg<state::STATE_SPEC>;
#[doc = "Internal STATE of UART Controller"]
pub mod state;
#[doc = "PERID register accessor: an alias for `Reg<PERID_SPEC>`"]
pub type PERID = crate::Reg<perid::PERID_SPEC>;
#[doc = "Peripheral ID Register"]
pub mod perid;
