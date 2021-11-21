#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0100],
    #[doc = "0x100 - Interrupt Mask Register"]
    pub imr: crate::Reg<imr::IMR_SPEC>,
    #[doc = "0x104 - Interrupt Status Register"]
    pub isr: crate::Reg<isr::ISR_SPEC>,
    #[doc = "0x108 - Interrupt Clear Register"]
    pub icr: crate::Reg<icr::ICR_SPEC>,
    #[doc = "0x10c - Configuration Register"]
    pub config: crate::Reg<config::CONFIG_SPEC>,
    #[doc = "0x110 - Valid Register"]
    pub valid: crate::Reg<valid::VALID_SPEC>,
    #[doc = "0x114 - Entropy Holding Register Data Register"]
    pub ehr_data0: crate::Reg<ehr_data0::EHR_DATA0_SPEC>,
    #[doc = "0x118 - Entropy Holding Register Data Register"]
    pub ehr_data1: crate::Reg<ehr_data1::EHR_DATA1_SPEC>,
    #[doc = "0x11c - Entropy Holding Register Data Register"]
    pub ehr_data2: crate::Reg<ehr_data2::EHR_DATA2_SPEC>,
    #[doc = "0x120 - Entropy Holding Register Data Register"]
    pub ehr_data3: crate::Reg<ehr_data3::EHR_DATA3_SPEC>,
    #[doc = "0x124 - Entropy Holding Register Data Register"]
    pub ehr_data4: crate::Reg<ehr_data4::EHR_DATA4_SPEC>,
    #[doc = "0x128 - Entropy Holding Register Data Register"]
    pub ehr_data5: crate::Reg<ehr_data5::EHR_DATA5_SPEC>,
    #[doc = "0x12c - Random Source Enable Register"]
    pub rnd_source_enable: crate::Reg<rnd_source_enable::RND_SOURCE_ENABLE_SPEC>,
    #[doc = "0x130 - Section TBD"]
    pub sample_cnt1: crate::Reg<sample_cnt1::SAMPLE_CNT1_SPEC>,
    #[doc = "0x134 - Auto-correlator Statistic Register"]
    pub autocorr_statistic: crate::Reg<autocorr_statistic::AUTOCORR_STATISTIC_SPEC>,
    #[doc = "0x138 - Section TBD"]
    pub debug_control: crate::Reg<debug_control::DEBUG_CONTROL_SPEC>,
    _reserved15: [u8; 0x04],
    #[doc = "0x140 - Reset Register"]
    pub sw_reset: crate::Reg<sw_reset::SW_RESET_SPEC>,
    _reserved16: [u8; 0x74],
    #[doc = "0x1b8 - Busy Register"]
    pub busy: crate::Reg<busy::BUSY_SPEC>,
    #[doc = "0x1bc - Reset Bits Counter Register"]
    pub rst_bits_counter: crate::Reg<rst_bits_counter::RST_BITS_COUNTER_SPEC>,
    _reserved18: [u8; 0x20],
    #[doc = "0x1e0 - BIST Counter Register"]
    pub bist_cntr0: crate::Reg<bist_cntr0::BIST_CNTR0_SPEC>,
    #[doc = "0x1e4 - BIST Counter Register"]
    pub bist_cntr1: crate::Reg<bist_cntr1::BIST_CNTR1_SPEC>,
    #[doc = "0x1e8 - BIST Counter Register"]
    pub bist_cntr2: crate::Reg<bist_cntr2::BIST_CNTR2_SPEC>,
}
#[doc = "IMR register accessor: an alias for `Reg<IMR_SPEC>`"]
pub type IMR = crate::Reg<imr::IMR_SPEC>;
#[doc = "Interrupt Mask Register"]
pub mod imr;
#[doc = "ISR register accessor: an alias for `Reg<ISR_SPEC>`"]
pub type ISR = crate::Reg<isr::ISR_SPEC>;
#[doc = "Interrupt Status Register"]
pub mod isr;
#[doc = "ICR register accessor: an alias for `Reg<ICR_SPEC>`"]
pub type ICR = crate::Reg<icr::ICR_SPEC>;
#[doc = "Interrupt Clear Register"]
pub mod icr;
#[doc = "CONFIG register accessor: an alias for `Reg<CONFIG_SPEC>`"]
pub type CONFIG = crate::Reg<config::CONFIG_SPEC>;
#[doc = "Configuration Register"]
pub mod config;
#[doc = "VALID register accessor: an alias for `Reg<VALID_SPEC>`"]
pub type VALID = crate::Reg<valid::VALID_SPEC>;
#[doc = "Valid Register"]
pub mod valid;
#[doc = "EHR_DATA0 register accessor: an alias for `Reg<EHR_DATA0_SPEC>`"]
pub type EHR_DATA0 = crate::Reg<ehr_data0::EHR_DATA0_SPEC>;
#[doc = "Entropy Holding Register Data Register"]
pub mod ehr_data0;
#[doc = "EHR_DATA1 register accessor: an alias for `Reg<EHR_DATA1_SPEC>`"]
pub type EHR_DATA1 = crate::Reg<ehr_data1::EHR_DATA1_SPEC>;
#[doc = "Entropy Holding Register Data Register"]
pub mod ehr_data1;
#[doc = "EHR_DATA2 register accessor: an alias for `Reg<EHR_DATA2_SPEC>`"]
pub type EHR_DATA2 = crate::Reg<ehr_data2::EHR_DATA2_SPEC>;
#[doc = "Entropy Holding Register Data Register"]
pub mod ehr_data2;
#[doc = "EHR_DATA3 register accessor: an alias for `Reg<EHR_DATA3_SPEC>`"]
pub type EHR_DATA3 = crate::Reg<ehr_data3::EHR_DATA3_SPEC>;
#[doc = "Entropy Holding Register Data Register"]
pub mod ehr_data3;
#[doc = "EHR_DATA4 register accessor: an alias for `Reg<EHR_DATA4_SPEC>`"]
pub type EHR_DATA4 = crate::Reg<ehr_data4::EHR_DATA4_SPEC>;
#[doc = "Entropy Holding Register Data Register"]
pub mod ehr_data4;
#[doc = "EHR_DATA5 register accessor: an alias for `Reg<EHR_DATA5_SPEC>`"]
pub type EHR_DATA5 = crate::Reg<ehr_data5::EHR_DATA5_SPEC>;
#[doc = "Entropy Holding Register Data Register"]
pub mod ehr_data5;
#[doc = "RND_SOURCE_ENABLE register accessor: an alias for `Reg<RND_SOURCE_ENABLE_SPEC>`"]
pub type RND_SOURCE_ENABLE = crate::Reg<rnd_source_enable::RND_SOURCE_ENABLE_SPEC>;
#[doc = "Random Source Enable Register"]
pub mod rnd_source_enable;
#[doc = "SAMPLE_CNT1 register accessor: an alias for `Reg<SAMPLE_CNT1_SPEC>`"]
pub type SAMPLE_CNT1 = crate::Reg<sample_cnt1::SAMPLE_CNT1_SPEC>;
#[doc = "Section TBD"]
pub mod sample_cnt1;
#[doc = "AUTOCORR_STATISTIC register accessor: an alias for `Reg<AUTOCORR_STATISTIC_SPEC>`"]
pub type AUTOCORR_STATISTIC = crate::Reg<autocorr_statistic::AUTOCORR_STATISTIC_SPEC>;
#[doc = "Auto-correlator Statistic Register"]
pub mod autocorr_statistic;
#[doc = "DEBUG_CONTROL register accessor: an alias for `Reg<DEBUG_CONTROL_SPEC>`"]
pub type DEBUG_CONTROL = crate::Reg<debug_control::DEBUG_CONTROL_SPEC>;
#[doc = "Section TBD"]
pub mod debug_control;
#[doc = "SW_RESET register accessor: an alias for `Reg<SW_RESET_SPEC>`"]
pub type SW_RESET = crate::Reg<sw_reset::SW_RESET_SPEC>;
#[doc = "Reset Register"]
pub mod sw_reset;
#[doc = "BUSY register accessor: an alias for `Reg<BUSY_SPEC>`"]
pub type BUSY = crate::Reg<busy::BUSY_SPEC>;
#[doc = "Busy Register"]
pub mod busy;
#[doc = "RST_BITS_COUNTER register accessor: an alias for `Reg<RST_BITS_COUNTER_SPEC>`"]
pub type RST_BITS_COUNTER = crate::Reg<rst_bits_counter::RST_BITS_COUNTER_SPEC>;
#[doc = "Reset Bits Counter Register"]
pub mod rst_bits_counter;
#[doc = "BIST_CNTR0 register accessor: an alias for `Reg<BIST_CNTR0_SPEC>`"]
pub type BIST_CNTR0 = crate::Reg<bist_cntr0::BIST_CNTR0_SPEC>;
#[doc = "BIST Counter Register"]
pub mod bist_cntr0;
#[doc = "BIST_CNTR1 register accessor: an alias for `Reg<BIST_CNTR1_SPEC>`"]
pub type BIST_CNTR1 = crate::Reg<bist_cntr1::BIST_CNTR1_SPEC>;
#[doc = "BIST Counter Register"]
pub mod bist_cntr1;
#[doc = "BIST_CNTR2 register accessor: an alias for `Reg<BIST_CNTR2_SPEC>`"]
pub type BIST_CNTR2 = crate::Reg<bist_cntr2::BIST_CNTR2_SPEC>;
#[doc = "BIST Counter Register"]
pub mod bist_cntr2;
