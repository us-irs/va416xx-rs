#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    #[doc = "0x04 - Status/Interrupt Source Register"]
    pub sts: crate::Reg<sts::STS_SPEC>,
    #[doc = "0x08 - Node Address Register"]
    pub defaddr: crate::Reg<defaddr::DEFADDR_SPEC>,
    #[doc = "0x0c - Clock Divisor Register"]
    pub clkdiv: crate::Reg<clkdiv::CLKDIV_SPEC>,
    #[doc = "0x10 - Destination Key"]
    pub dkey: crate::Reg<dkey::DKEY_SPEC>,
    #[doc = "0x14 - Time Code Register"]
    pub tc: crate::Reg<tc::TC_SPEC>,
    #[doc = "0x18 - Timer and Disconnect Register"]
    pub tdr: crate::Reg<tdr::TDR_SPEC>,
    _reserved7: [u8; 0x04],
    #[doc = "0x20 - DMA Control Register"]
    pub dmactrl0: crate::Reg<dmactrl0::DMACTRL0_SPEC>,
    #[doc = "0x24 - DMA RX Maximum Length Register"]
    pub dmamaxlen0: crate::Reg<dmamaxlen0::DMAMAXLEN0_SPEC>,
    #[doc = "0x28 - DMA Transmitter Descriptor Table Address Register"]
    pub dmatxdesc0: crate::Reg<dmatxdesc0::DMATXDESC0_SPEC>,
    #[doc = "0x2c - DMA Receiver Table Destination Register"]
    pub dmarxdesc0: crate::Reg<dmarxdesc0::DMARXDESC0_SPEC>,
    #[doc = "0x30 - DMA Receiver Table Address Register"]
    pub dmaaddr0: crate::Reg<dmaaddr0::DMAADDR0_SPEC>,
}
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "STS register accessor: an alias for `Reg<STS_SPEC>`"]
pub type STS = crate::Reg<sts::STS_SPEC>;
#[doc = "Status/Interrupt Source Register"]
pub mod sts;
#[doc = "DEFADDR register accessor: an alias for `Reg<DEFADDR_SPEC>`"]
pub type DEFADDR = crate::Reg<defaddr::DEFADDR_SPEC>;
#[doc = "Node Address Register"]
pub mod defaddr;
#[doc = "CLKDIV register accessor: an alias for `Reg<CLKDIV_SPEC>`"]
pub type CLKDIV = crate::Reg<clkdiv::CLKDIV_SPEC>;
#[doc = "Clock Divisor Register"]
pub mod clkdiv;
#[doc = "DKEY register accessor: an alias for `Reg<DKEY_SPEC>`"]
pub type DKEY = crate::Reg<dkey::DKEY_SPEC>;
#[doc = "Destination Key"]
pub mod dkey;
#[doc = "TC register accessor: an alias for `Reg<TC_SPEC>`"]
pub type TC = crate::Reg<tc::TC_SPEC>;
#[doc = "Time Code Register"]
pub mod tc;
#[doc = "TDR register accessor: an alias for `Reg<TDR_SPEC>`"]
pub type TDR = crate::Reg<tdr::TDR_SPEC>;
#[doc = "Timer and Disconnect Register"]
pub mod tdr;
#[doc = "DMACTRL0 register accessor: an alias for `Reg<DMACTRL0_SPEC>`"]
pub type DMACTRL0 = crate::Reg<dmactrl0::DMACTRL0_SPEC>;
#[doc = "DMA Control Register"]
pub mod dmactrl0;
#[doc = "DMAMAXLEN0 register accessor: an alias for `Reg<DMAMAXLEN0_SPEC>`"]
pub type DMAMAXLEN0 = crate::Reg<dmamaxlen0::DMAMAXLEN0_SPEC>;
#[doc = "DMA RX Maximum Length Register"]
pub mod dmamaxlen0;
#[doc = "DMATXDESC0 register accessor: an alias for `Reg<DMATXDESC0_SPEC>`"]
pub type DMATXDESC0 = crate::Reg<dmatxdesc0::DMATXDESC0_SPEC>;
#[doc = "DMA Transmitter Descriptor Table Address Register"]
pub mod dmatxdesc0;
#[doc = "DMARXDESC0 register accessor: an alias for `Reg<DMARXDESC0_SPEC>`"]
pub type DMARXDESC0 = crate::Reg<dmarxdesc0::DMARXDESC0_SPEC>;
#[doc = "DMA Receiver Table Destination Register"]
pub mod dmarxdesc0;
#[doc = "DMAADDR0 register accessor: an alias for `Reg<DMAADDR0_SPEC>`"]
pub type DMAADDR0 = crate::Reg<dmaaddr0::DMAADDR0_SPEC>;
#[doc = "DMA Receiver Table Address Register"]
pub mod dmaaddr0;
