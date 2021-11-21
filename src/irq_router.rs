#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Interrupt select for DMA channel 0"]
    pub dmasel0: crate::Reg<dmasel0::DMASEL0_SPEC>,
    #[doc = "0x04 - Interrupt select for DMA channel 1"]
    pub dmasel1: crate::Reg<dmasel1::DMASEL1_SPEC>,
    #[doc = "0x08 - Interrupt select for DMA channel 2"]
    pub dmasel2: crate::Reg<dmasel2::DMASEL2_SPEC>,
    #[doc = "0x0c - Interrupt select for DMA channel 3"]
    pub dmasel3: crate::Reg<dmasel3::DMASEL3_SPEC>,
    #[doc = "0x10 - Trigger select for the DMA channels"]
    pub dmattsel: crate::Reg<dmattsel::DMATTSEL_SPEC>,
    #[doc = "0x14 - Interrupt select for ADC"]
    pub adcsel: crate::Reg<adcsel::ADCSEL_SPEC>,
    #[doc = "0x18 - Interrupt select for DAC0"]
    pub dacsel0: crate::Reg<dacsel0::DACSEL0_SPEC>,
    #[doc = "0x1c - Interrupt select for DAC1"]
    pub dacsel1: crate::Reg<dacsel1::DACSEL1_SPEC>,
    #[doc = "0x20 - DEBUG IRQ_OUT\\[31:0\\]"]
    pub irq_out0: crate::Reg<irq_out0::IRQ_OUT0_SPEC>,
    #[doc = "0x24 - DEBUG IRQ_OUT\\[63:32\\]"]
    pub irq_out1: crate::Reg<irq_out1::IRQ_OUT1_SPEC>,
    #[doc = "0x28 - DEBUG IRQ_OUT\\[95:64\\]"]
    pub irq_out2: crate::Reg<irq_out2::IRQ_OUT2_SPEC>,
    #[doc = "0x2c - DEBUG IRQ_OUT\\[127:96\\]"]
    pub irq_out3: crate::Reg<irq_out3::IRQ_OUT3_SPEC>,
    #[doc = "0x30 - DEBUG IRQ_OUT\\[159:128\\]"]
    pub irq_out4: crate::Reg<irq_out4::IRQ_OUT4_SPEC>,
    #[doc = "0x34 - DEBUG IRQ_OUT\\[179:160\\]"]
    pub irq_out5: crate::Reg<irq_out5::IRQ_OUT5_SPEC>,
    _reserved14: [u8; 0x0fc4],
    #[doc = "0xffc - Peripheral ID Register"]
    pub perid: crate::Reg<perid::PERID_SPEC>,
}
#[doc = "DMASEL0 register accessor: an alias for `Reg<DMASEL0_SPEC>`"]
pub type DMASEL0 = crate::Reg<dmasel0::DMASEL0_SPEC>;
#[doc = "Interrupt select for DMA channel 0"]
pub mod dmasel0;
#[doc = "DMASEL1 register accessor: an alias for `Reg<DMASEL1_SPEC>`"]
pub type DMASEL1 = crate::Reg<dmasel1::DMASEL1_SPEC>;
#[doc = "Interrupt select for DMA channel 1"]
pub mod dmasel1;
#[doc = "DMASEL2 register accessor: an alias for `Reg<DMASEL2_SPEC>`"]
pub type DMASEL2 = crate::Reg<dmasel2::DMASEL2_SPEC>;
#[doc = "Interrupt select for DMA channel 2"]
pub mod dmasel2;
#[doc = "DMASEL3 register accessor: an alias for `Reg<DMASEL3_SPEC>`"]
pub type DMASEL3 = crate::Reg<dmasel3::DMASEL3_SPEC>;
#[doc = "Interrupt select for DMA channel 3"]
pub mod dmasel3;
#[doc = "DMATTSEL register accessor: an alias for `Reg<DMATTSEL_SPEC>`"]
pub type DMATTSEL = crate::Reg<dmattsel::DMATTSEL_SPEC>;
#[doc = "Trigger select for the DMA channels"]
pub mod dmattsel;
#[doc = "ADCSEL register accessor: an alias for `Reg<ADCSEL_SPEC>`"]
pub type ADCSEL = crate::Reg<adcsel::ADCSEL_SPEC>;
#[doc = "Interrupt select for ADC"]
pub mod adcsel;
#[doc = "DACSEL0 register accessor: an alias for `Reg<DACSEL0_SPEC>`"]
pub type DACSEL0 = crate::Reg<dacsel0::DACSEL0_SPEC>;
#[doc = "Interrupt select for DAC0"]
pub mod dacsel0;
#[doc = "DACSEL1 register accessor: an alias for `Reg<DACSEL1_SPEC>`"]
pub type DACSEL1 = crate::Reg<dacsel1::DACSEL1_SPEC>;
#[doc = "Interrupt select for DAC1"]
pub mod dacsel1;
#[doc = "IRQ_OUT0 register accessor: an alias for `Reg<IRQ_OUT0_SPEC>`"]
pub type IRQ_OUT0 = crate::Reg<irq_out0::IRQ_OUT0_SPEC>;
#[doc = "DEBUG IRQ_OUT\\[31:0\\]"]
pub mod irq_out0;
#[doc = "IRQ_OUT1 register accessor: an alias for `Reg<IRQ_OUT1_SPEC>`"]
pub type IRQ_OUT1 = crate::Reg<irq_out1::IRQ_OUT1_SPEC>;
#[doc = "DEBUG IRQ_OUT\\[63:32\\]"]
pub mod irq_out1;
#[doc = "IRQ_OUT2 register accessor: an alias for `Reg<IRQ_OUT2_SPEC>`"]
pub type IRQ_OUT2 = crate::Reg<irq_out2::IRQ_OUT2_SPEC>;
#[doc = "DEBUG IRQ_OUT\\[95:64\\]"]
pub mod irq_out2;
#[doc = "IRQ_OUT3 register accessor: an alias for `Reg<IRQ_OUT3_SPEC>`"]
pub type IRQ_OUT3 = crate::Reg<irq_out3::IRQ_OUT3_SPEC>;
#[doc = "DEBUG IRQ_OUT\\[127:96\\]"]
pub mod irq_out3;
#[doc = "IRQ_OUT4 register accessor: an alias for `Reg<IRQ_OUT4_SPEC>`"]
pub type IRQ_OUT4 = crate::Reg<irq_out4::IRQ_OUT4_SPEC>;
#[doc = "DEBUG IRQ_OUT\\[159:128\\]"]
pub mod irq_out4;
#[doc = "IRQ_OUT5 register accessor: an alias for `Reg<IRQ_OUT5_SPEC>`"]
pub type IRQ_OUT5 = crate::Reg<irq_out5::IRQ_OUT5_SPEC>;
#[doc = "DEBUG IRQ_OUT\\[179:160\\]"]
pub mod irq_out5;
#[doc = "PERID register accessor: an alias for `Reg<PERID_SPEC>`"]
pub type PERID = crate::Reg<perid::PERID_SPEC>;
#[doc = "Peripheral ID Register"]
pub mod perid;
