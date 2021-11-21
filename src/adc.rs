#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    #[doc = "0x04 - FIFO data"]
    pub fifo_data: crate::Reg<fifo_data::FIFO_DATA_SPEC>,
    #[doc = "0x08 - Status"]
    pub status: crate::Reg<status::STATUS_SPEC>,
    #[doc = "0x0c - Interrupt Enable"]
    pub irq_enb: crate::Reg<irq_enb::IRQ_ENB_SPEC>,
    #[doc = "0x10 - Raw Interrupt Status"]
    pub irq_raw: crate::Reg<irq_raw::IRQ_RAW_SPEC>,
    #[doc = "0x14 - Enabled Interrupt Status"]
    pub irq_end: crate::Reg<irq_end::IRQ_END_SPEC>,
    #[doc = "0x18 - Clear Interrupt"]
    pub irq_clr: crate::Reg<irq_clr::IRQ_CLR_SPEC>,
    #[doc = "0x1c - Receive FIFO Interrupt Trigger Value"]
    pub rxfifoirqtrg: crate::Reg<rxfifoirqtrg::RXFIFOIRQTRG_SPEC>,
    #[doc = "0x20 - FIFO Clear"]
    pub fifo_clr: crate::Reg<fifo_clr::FIFO_CLR_SPEC>,
    _reserved9: [u8; 0x0fd8],
    #[doc = "0xffc - Peripheral ID Register"]
    pub perid: crate::Reg<perid::PERID_SPEC>,
}
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "FIFO_DATA register accessor: an alias for `Reg<FIFO_DATA_SPEC>`"]
pub type FIFO_DATA = crate::Reg<fifo_data::FIFO_DATA_SPEC>;
#[doc = "FIFO data"]
pub mod fifo_data;
#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status"]
pub mod status;
#[doc = "IRQ_ENB register accessor: an alias for `Reg<IRQ_ENB_SPEC>`"]
pub type IRQ_ENB = crate::Reg<irq_enb::IRQ_ENB_SPEC>;
#[doc = "Interrupt Enable"]
pub mod irq_enb;
#[doc = "IRQ_RAW register accessor: an alias for `Reg<IRQ_RAW_SPEC>`"]
pub type IRQ_RAW = crate::Reg<irq_raw::IRQ_RAW_SPEC>;
#[doc = "Raw Interrupt Status"]
pub mod irq_raw;
#[doc = "IRQ_END register accessor: an alias for `Reg<IRQ_END_SPEC>`"]
pub type IRQ_END = crate::Reg<irq_end::IRQ_END_SPEC>;
#[doc = "Enabled Interrupt Status"]
pub mod irq_end;
#[doc = "IRQ_CLR register accessor: an alias for `Reg<IRQ_CLR_SPEC>`"]
pub type IRQ_CLR = crate::Reg<irq_clr::IRQ_CLR_SPEC>;
#[doc = "Clear Interrupt"]
pub mod irq_clr;
#[doc = "RXFIFOIRQTRG register accessor: an alias for `Reg<RXFIFOIRQTRG_SPEC>`"]
pub type RXFIFOIRQTRG = crate::Reg<rxfifoirqtrg::RXFIFOIRQTRG_SPEC>;
#[doc = "Receive FIFO Interrupt Trigger Value"]
pub mod rxfifoirqtrg;
#[doc = "FIFO_CLR register accessor: an alias for `Reg<FIFO_CLR_SPEC>`"]
pub type FIFO_CLR = crate::Reg<fifo_clr::FIFO_CLR_SPEC>;
#[doc = "FIFO Clear"]
pub mod fifo_clr;
#[doc = "PERID register accessor: an alias for `Reg<PERID_SPEC>`"]
pub type PERID = crate::Reg<perid::PERID_SPEC>;
#[doc = "Peripheral ID Register"]
pub mod perid;
