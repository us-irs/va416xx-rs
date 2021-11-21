#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    #[doc = "0x04 - Clock Scale divide value"]
    pub clkscale: crate::Reg<clkscale::CLKSCALE_SPEC>,
    #[doc = "0x08 - Word Count value"]
    pub words: crate::Reg<words::WORDS_SPEC>,
    #[doc = "0x0c - I2C Address value"]
    pub address: crate::Reg<address::ADDRESS_SPEC>,
    #[doc = "0x10 - Data Input/Output"]
    pub data: crate::Reg<data::DATA_SPEC>,
    #[doc = "0x14 - Command Register"]
    pub cmd: crate::Reg<cmd::CMD_SPEC>,
    #[doc = "0x18 - I2C Controller Status Register"]
    pub status: crate::Reg<status::STATUS_SPEC>,
    #[doc = "0x1c - Internal STATE of I2C Master Controller"]
    pub state: crate::Reg<state::STATE_SPEC>,
    #[doc = "0x20 - TX Count Register"]
    pub txcount: crate::Reg<txcount::TXCOUNT_SPEC>,
    #[doc = "0x24 - RX Count Register"]
    pub rxcount: crate::Reg<rxcount::RXCOUNT_SPEC>,
    #[doc = "0x28 - Interrupt Enable Register"]
    pub irq_enb: crate::Reg<irq_enb::IRQ_ENB_SPEC>,
    #[doc = "0x2c - Raw Interrupt Status Register"]
    pub irq_raw: crate::Reg<irq_raw::IRQ_RAW_SPEC>,
    #[doc = "0x30 - Enabled Interrupt Status Register"]
    pub irq_end: crate::Reg<irq_end::IRQ_END_SPEC>,
    #[doc = "0x34 - Clear Interrupt Status Register"]
    pub irq_clr: crate::Reg<irq_clr::IRQ_CLR_SPEC>,
    #[doc = "0x38 - Rx FIFO IRQ Trigger Level"]
    pub rxfifoirqtrg: crate::Reg<rxfifoirqtrg::RXFIFOIRQTRG_SPEC>,
    #[doc = "0x3c - Tx FIFO IRQ Trigger Level"]
    pub txfifoirqtrg: crate::Reg<txfifoirqtrg::TXFIFOIRQTRG_SPEC>,
    #[doc = "0x40 - Clear FIFO Register"]
    pub fifo_clr: crate::Reg<fifo_clr::FIFO_CLR_SPEC>,
    #[doc = "0x44 - Timing Config Register"]
    pub tmconfig: crate::Reg<tmconfig::TMCONFIG_SPEC>,
    #[doc = "0x48 - Clock Low Timeout Limit Register"]
    pub clktolimit: crate::Reg<clktolimit::CLKTOLIMIT_SPEC>,
    _reserved19: [u8; 0xb4],
    #[doc = "0x100 - Slave Control Register"]
    pub s0_ctrl: crate::Reg<s0_ctrl::S0_CTRL_SPEC>,
    #[doc = "0x104 - Slave MaxWords Register"]
    pub s0_maxwords: crate::Reg<s0_maxwords::S0_MAXWORDS_SPEC>,
    #[doc = "0x108 - Slave I2C Address Value"]
    pub s0_address: crate::Reg<s0_address::S0_ADDRESS_SPEC>,
    #[doc = "0x10c - Slave I2C Address Mask value"]
    pub s0_addressmask: crate::Reg<s0_addressmask::S0_ADDRESSMASK_SPEC>,
    #[doc = "0x110 - Slave Data Input/Output"]
    pub s0_data: crate::Reg<s0_data::S0_DATA_SPEC>,
    #[doc = "0x114 - Slave I2C Last Address value"]
    pub s0_lastaddress: crate::Reg<s0_lastaddress::S0_LASTADDRESS_SPEC>,
    #[doc = "0x118 - Slave I2C Controller Status Register"]
    pub s0_status: crate::Reg<s0_status::S0_STATUS_SPEC>,
    #[doc = "0x11c - Internal STATE of I2C Slave Controller"]
    pub s0_state: crate::Reg<s0_state::S0_STATE_SPEC>,
    #[doc = "0x120 - Slave TX Count Register"]
    pub s0_txcount: crate::Reg<s0_txcount::S0_TXCOUNT_SPEC>,
    #[doc = "0x124 - Slave RX Count Register"]
    pub s0_rxcount: crate::Reg<s0_rxcount::S0_RXCOUNT_SPEC>,
    #[doc = "0x128 - Slave Interrupt Enable Register"]
    pub s0_irq_enb: crate::Reg<s0_irq_enb::S0_IRQ_ENB_SPEC>,
    #[doc = "0x12c - Slave Raw Interrupt Status Register"]
    pub s0_irq_raw: crate::Reg<s0_irq_raw::S0_IRQ_RAW_SPEC>,
    #[doc = "0x130 - Slave Enabled Interrupt Status Register"]
    pub s0_irq_end: crate::Reg<s0_irq_end::S0_IRQ_END_SPEC>,
    #[doc = "0x134 - Slave Clear Interrupt Status Register"]
    pub s0_irq_clr: crate::Reg<s0_irq_clr::S0_IRQ_CLR_SPEC>,
    #[doc = "0x138 - Slave Rx FIFO IRQ Trigger Level"]
    pub s0_rxfifoirqtrg: crate::Reg<s0_rxfifoirqtrg::S0_RXFIFOIRQTRG_SPEC>,
    #[doc = "0x13c - Slave Tx FIFO IRQ Trigger Level"]
    pub s0_txfifoirqtrg: crate::Reg<s0_txfifoirqtrg::S0_TXFIFOIRQTRG_SPEC>,
    #[doc = "0x140 - Slave Clear FIFO Register"]
    pub s0_fifo_clr: crate::Reg<s0_fifo_clr::S0_FIFO_CLR_SPEC>,
    #[doc = "0x144 - Slave I2C Address B Value"]
    pub s0_addressb: crate::Reg<s0_addressb::S0_ADDRESSB_SPEC>,
    #[doc = "0x148 - Slave I2C Address B Mask value"]
    pub s0_addressmaskb: crate::Reg<s0_addressmaskb::S0_ADDRESSMASKB_SPEC>,
    _reserved38: [u8; 0x02b0],
    #[doc = "0x3fc - Peripheral ID Register"]
    pub perid: crate::Reg<perid::PERID_SPEC>,
}
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "CLKSCALE register accessor: an alias for `Reg<CLKSCALE_SPEC>`"]
pub type CLKSCALE = crate::Reg<clkscale::CLKSCALE_SPEC>;
#[doc = "Clock Scale divide value"]
pub mod clkscale;
#[doc = "WORDS register accessor: an alias for `Reg<WORDS_SPEC>`"]
pub type WORDS = crate::Reg<words::WORDS_SPEC>;
#[doc = "Word Count value"]
pub mod words;
#[doc = "ADDRESS register accessor: an alias for `Reg<ADDRESS_SPEC>`"]
pub type ADDRESS = crate::Reg<address::ADDRESS_SPEC>;
#[doc = "I2C Address value"]
pub mod address;
#[doc = "DATA register accessor: an alias for `Reg<DATA_SPEC>`"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "Data Input/Output"]
pub mod data;
#[doc = "CMD register accessor: an alias for `Reg<CMD_SPEC>`"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "Command Register"]
pub mod cmd;
#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "I2C Controller Status Register"]
pub mod status;
#[doc = "STATE register accessor: an alias for `Reg<STATE_SPEC>`"]
pub type STATE = crate::Reg<state::STATE_SPEC>;
#[doc = "Internal STATE of I2C Master Controller"]
pub mod state;
#[doc = "TXCOUNT register accessor: an alias for `Reg<TXCOUNT_SPEC>`"]
pub type TXCOUNT = crate::Reg<txcount::TXCOUNT_SPEC>;
#[doc = "TX Count Register"]
pub mod txcount;
#[doc = "RXCOUNT register accessor: an alias for `Reg<RXCOUNT_SPEC>`"]
pub type RXCOUNT = crate::Reg<rxcount::RXCOUNT_SPEC>;
#[doc = "RX Count Register"]
pub mod rxcount;
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
#[doc = "TMCONFIG register accessor: an alias for `Reg<TMCONFIG_SPEC>`"]
pub type TMCONFIG = crate::Reg<tmconfig::TMCONFIG_SPEC>;
#[doc = "Timing Config Register"]
pub mod tmconfig;
#[doc = "CLKTOLIMIT register accessor: an alias for `Reg<CLKTOLIMIT_SPEC>`"]
pub type CLKTOLIMIT = crate::Reg<clktolimit::CLKTOLIMIT_SPEC>;
#[doc = "Clock Low Timeout Limit Register"]
pub mod clktolimit;
#[doc = "S0_CTRL register accessor: an alias for `Reg<S0_CTRL_SPEC>`"]
pub type S0_CTRL = crate::Reg<s0_ctrl::S0_CTRL_SPEC>;
#[doc = "Slave Control Register"]
pub mod s0_ctrl;
#[doc = "S0_MAXWORDS register accessor: an alias for `Reg<S0_MAXWORDS_SPEC>`"]
pub type S0_MAXWORDS = crate::Reg<s0_maxwords::S0_MAXWORDS_SPEC>;
#[doc = "Slave MaxWords Register"]
pub mod s0_maxwords;
#[doc = "S0_ADDRESS register accessor: an alias for `Reg<S0_ADDRESS_SPEC>`"]
pub type S0_ADDRESS = crate::Reg<s0_address::S0_ADDRESS_SPEC>;
#[doc = "Slave I2C Address Value"]
pub mod s0_address;
#[doc = "S0_ADDRESSMASK register accessor: an alias for `Reg<S0_ADDRESSMASK_SPEC>`"]
pub type S0_ADDRESSMASK = crate::Reg<s0_addressmask::S0_ADDRESSMASK_SPEC>;
#[doc = "Slave I2C Address Mask value"]
pub mod s0_addressmask;
#[doc = "S0_DATA register accessor: an alias for `Reg<S0_DATA_SPEC>`"]
pub type S0_DATA = crate::Reg<s0_data::S0_DATA_SPEC>;
#[doc = "Slave Data Input/Output"]
pub mod s0_data;
#[doc = "S0_LASTADDRESS register accessor: an alias for `Reg<S0_LASTADDRESS_SPEC>`"]
pub type S0_LASTADDRESS = crate::Reg<s0_lastaddress::S0_LASTADDRESS_SPEC>;
#[doc = "Slave I2C Last Address value"]
pub mod s0_lastaddress;
#[doc = "S0_STATUS register accessor: an alias for `Reg<S0_STATUS_SPEC>`"]
pub type S0_STATUS = crate::Reg<s0_status::S0_STATUS_SPEC>;
#[doc = "Slave I2C Controller Status Register"]
pub mod s0_status;
#[doc = "S0_STATE register accessor: an alias for `Reg<S0_STATE_SPEC>`"]
pub type S0_STATE = crate::Reg<s0_state::S0_STATE_SPEC>;
#[doc = "Internal STATE of I2C Slave Controller"]
pub mod s0_state;
#[doc = "S0_TXCOUNT register accessor: an alias for `Reg<S0_TXCOUNT_SPEC>`"]
pub type S0_TXCOUNT = crate::Reg<s0_txcount::S0_TXCOUNT_SPEC>;
#[doc = "Slave TX Count Register"]
pub mod s0_txcount;
#[doc = "S0_RXCOUNT register accessor: an alias for `Reg<S0_RXCOUNT_SPEC>`"]
pub type S0_RXCOUNT = crate::Reg<s0_rxcount::S0_RXCOUNT_SPEC>;
#[doc = "Slave RX Count Register"]
pub mod s0_rxcount;
#[doc = "S0_IRQ_ENB register accessor: an alias for `Reg<S0_IRQ_ENB_SPEC>`"]
pub type S0_IRQ_ENB = crate::Reg<s0_irq_enb::S0_IRQ_ENB_SPEC>;
#[doc = "Slave Interrupt Enable Register"]
pub mod s0_irq_enb;
#[doc = "S0_IRQ_RAW register accessor: an alias for `Reg<S0_IRQ_RAW_SPEC>`"]
pub type S0_IRQ_RAW = crate::Reg<s0_irq_raw::S0_IRQ_RAW_SPEC>;
#[doc = "Slave Raw Interrupt Status Register"]
pub mod s0_irq_raw;
#[doc = "S0_IRQ_END register accessor: an alias for `Reg<S0_IRQ_END_SPEC>`"]
pub type S0_IRQ_END = crate::Reg<s0_irq_end::S0_IRQ_END_SPEC>;
#[doc = "Slave Enabled Interrupt Status Register"]
pub mod s0_irq_end;
#[doc = "S0_IRQ_CLR register accessor: an alias for `Reg<S0_IRQ_CLR_SPEC>`"]
pub type S0_IRQ_CLR = crate::Reg<s0_irq_clr::S0_IRQ_CLR_SPEC>;
#[doc = "Slave Clear Interrupt Status Register"]
pub mod s0_irq_clr;
#[doc = "S0_RXFIFOIRQTRG register accessor: an alias for `Reg<S0_RXFIFOIRQTRG_SPEC>`"]
pub type S0_RXFIFOIRQTRG = crate::Reg<s0_rxfifoirqtrg::S0_RXFIFOIRQTRG_SPEC>;
#[doc = "Slave Rx FIFO IRQ Trigger Level"]
pub mod s0_rxfifoirqtrg;
#[doc = "S0_TXFIFOIRQTRG register accessor: an alias for `Reg<S0_TXFIFOIRQTRG_SPEC>`"]
pub type S0_TXFIFOIRQTRG = crate::Reg<s0_txfifoirqtrg::S0_TXFIFOIRQTRG_SPEC>;
#[doc = "Slave Tx FIFO IRQ Trigger Level"]
pub mod s0_txfifoirqtrg;
#[doc = "S0_FIFO_CLR register accessor: an alias for `Reg<S0_FIFO_CLR_SPEC>`"]
pub type S0_FIFO_CLR = crate::Reg<s0_fifo_clr::S0_FIFO_CLR_SPEC>;
#[doc = "Slave Clear FIFO Register"]
pub mod s0_fifo_clr;
#[doc = "S0_ADDRESSB register accessor: an alias for `Reg<S0_ADDRESSB_SPEC>`"]
pub type S0_ADDRESSB = crate::Reg<s0_addressb::S0_ADDRESSB_SPEC>;
#[doc = "Slave I2C Address B Value"]
pub mod s0_addressb;
#[doc = "S0_ADDRESSMASKB register accessor: an alias for `Reg<S0_ADDRESSMASKB_SPEC>`"]
pub type S0_ADDRESSMASKB = crate::Reg<s0_addressmaskb::S0_ADDRESSMASKB_SPEC>;
#[doc = "Slave I2C Address B Mask value"]
pub mod s0_addressmaskb;
#[doc = "PERID register accessor: an alias for `Reg<PERID_SPEC>`"]
pub type PERID = crate::Reg<perid::PERID_SPEC>;
#[doc = "Peripheral ID Register"]
pub mod perid;
