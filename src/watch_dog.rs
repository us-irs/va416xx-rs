#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Counter Start Value"]
    pub wdogload: crate::Reg<wdogload::WDOGLOAD_SPEC>,
    #[doc = "0x04 - Down Counter Value"]
    pub wdogvalue: crate::Reg<wdogvalue::WDOGVALUE_SPEC>,
    #[doc = "0x08 - Enable for block reset and interrupt"]
    pub wdogcontrol: crate::Reg<wdogcontrol::WDOGCONTROL_SPEC>,
    #[doc = "0x0c - A write of any value clears the WDT module interrupt, and reloads the counter from the value in the WDOGLOAD Register"]
    pub wdogintclr: crate::Reg<wdogintclr::WDOGINTCLR_SPEC>,
    #[doc = "0x10 - Raw interrupt status"]
    pub wdogris: crate::Reg<wdogris::WDOGRIS_SPEC>,
    #[doc = "0x14 - Interrupt status"]
    pub wdogmis: crate::Reg<wdogmis::WDOGMIS_SPEC>,
    _reserved6: [u8; 0xa8],
    #[doc = "0xc0 - Lock"]
    pub wdoglock: crate::Reg<wdoglock::WDOGLOCK_SPEC>,
    _reserved7: [u8; 0x0e3c],
    #[doc = "0xf00 - Integration test control"]
    pub wdogitcr: crate::Reg<wdogitcr::WDOGITCR_SPEC>,
    #[doc = "0xf04 - Integration test output set"]
    pub wdogitop: crate::Reg<wdogitop::WDOGITOP_SPEC>,
    _reserved9: [u8; 0xd8],
    #[doc = "0xfe0 - Peripheral ID"]
    pub wdogperiphid0: crate::Reg<wdogperiphid0::WDOGPERIPHID0_SPEC>,
    #[doc = "0xfe4 - Peripheral ID"]
    pub wdogperiphid1: crate::Reg<wdogperiphid1::WDOGPERIPHID1_SPEC>,
    #[doc = "0xfe8 - Peripheral ID"]
    pub wdogperiphid2: crate::Reg<wdogperiphid2::WDOGPERIPHID2_SPEC>,
    #[doc = "0xfec - Peripheral ID"]
    pub wdogperiphid3: crate::Reg<wdogperiphid3::WDOGPERIPHID3_SPEC>,
    #[doc = "0xff0 - PrimeCell ID"]
    pub wdogpcellid0: crate::Reg<wdogpcellid0::WDOGPCELLID0_SPEC>,
    #[doc = "0xff4 - PrimeCell ID"]
    pub wdogpcellid1: crate::Reg<wdogpcellid1::WDOGPCELLID1_SPEC>,
    #[doc = "0xff8 - PrimeCell ID"]
    pub wdogpcellid2: crate::Reg<wdogpcellid2::WDOGPCELLID2_SPEC>,
    #[doc = "0xffc - PrimeCell ID"]
    pub wdogpcellid3: crate::Reg<wdogpcellid3::WDOGPCELLID3_SPEC>,
}
#[doc = "WDOGLOAD register accessor: an alias for `Reg<WDOGLOAD_SPEC>`"]
pub type WDOGLOAD = crate::Reg<wdogload::WDOGLOAD_SPEC>;
#[doc = "Counter Start Value"]
pub mod wdogload;
#[doc = "WDOGVALUE register accessor: an alias for `Reg<WDOGVALUE_SPEC>`"]
pub type WDOGVALUE = crate::Reg<wdogvalue::WDOGVALUE_SPEC>;
#[doc = "Down Counter Value"]
pub mod wdogvalue;
#[doc = "WDOGCONTROL register accessor: an alias for `Reg<WDOGCONTROL_SPEC>`"]
pub type WDOGCONTROL = crate::Reg<wdogcontrol::WDOGCONTROL_SPEC>;
#[doc = "Enable for block reset and interrupt"]
pub mod wdogcontrol;
#[doc = "WDOGINTCLR register accessor: an alias for `Reg<WDOGINTCLR_SPEC>`"]
pub type WDOGINTCLR = crate::Reg<wdogintclr::WDOGINTCLR_SPEC>;
#[doc = "A write of any value clears the WDT module interrupt, and reloads the counter from the value in the WDOGLOAD Register"]
pub mod wdogintclr;
#[doc = "WDOGRIS register accessor: an alias for `Reg<WDOGRIS_SPEC>`"]
pub type WDOGRIS = crate::Reg<wdogris::WDOGRIS_SPEC>;
#[doc = "Raw interrupt status"]
pub mod wdogris;
#[doc = "WDOGMIS register accessor: an alias for `Reg<WDOGMIS_SPEC>`"]
pub type WDOGMIS = crate::Reg<wdogmis::WDOGMIS_SPEC>;
#[doc = "Interrupt status"]
pub mod wdogmis;
#[doc = "WDOGLOCK register accessor: an alias for `Reg<WDOGLOCK_SPEC>`"]
pub type WDOGLOCK = crate::Reg<wdoglock::WDOGLOCK_SPEC>;
#[doc = "Lock"]
pub mod wdoglock;
#[doc = "WDOGITCR register accessor: an alias for `Reg<WDOGITCR_SPEC>`"]
pub type WDOGITCR = crate::Reg<wdogitcr::WDOGITCR_SPEC>;
#[doc = "Integration test control"]
pub mod wdogitcr;
#[doc = "WDOGITOP register accessor: an alias for `Reg<WDOGITOP_SPEC>`"]
pub type WDOGITOP = crate::Reg<wdogitop::WDOGITOP_SPEC>;
#[doc = "Integration test output set"]
pub mod wdogitop;
#[doc = "WDOGPERIPHID0 register accessor: an alias for `Reg<WDOGPERIPHID0_SPEC>`"]
pub type WDOGPERIPHID0 = crate::Reg<wdogperiphid0::WDOGPERIPHID0_SPEC>;
#[doc = "Peripheral ID"]
pub mod wdogperiphid0;
#[doc = "WDOGPERIPHID1 register accessor: an alias for `Reg<WDOGPERIPHID1_SPEC>`"]
pub type WDOGPERIPHID1 = crate::Reg<wdogperiphid1::WDOGPERIPHID1_SPEC>;
#[doc = "Peripheral ID"]
pub mod wdogperiphid1;
#[doc = "WDOGPERIPHID2 register accessor: an alias for `Reg<WDOGPERIPHID2_SPEC>`"]
pub type WDOGPERIPHID2 = crate::Reg<wdogperiphid2::WDOGPERIPHID2_SPEC>;
#[doc = "Peripheral ID"]
pub mod wdogperiphid2;
#[doc = "WDOGPERIPHID3 register accessor: an alias for `Reg<WDOGPERIPHID3_SPEC>`"]
pub type WDOGPERIPHID3 = crate::Reg<wdogperiphid3::WDOGPERIPHID3_SPEC>;
#[doc = "Peripheral ID"]
pub mod wdogperiphid3;
#[doc = "WDOGPCELLID0 register accessor: an alias for `Reg<WDOGPCELLID0_SPEC>`"]
pub type WDOGPCELLID0 = crate::Reg<wdogpcellid0::WDOGPCELLID0_SPEC>;
#[doc = "PrimeCell ID"]
pub mod wdogpcellid0;
#[doc = "WDOGPCELLID1 register accessor: an alias for `Reg<WDOGPCELLID1_SPEC>`"]
pub type WDOGPCELLID1 = crate::Reg<wdogpcellid1::WDOGPCELLID1_SPEC>;
#[doc = "PrimeCell ID"]
pub mod wdogpcellid1;
#[doc = "WDOGPCELLID2 register accessor: an alias for `Reg<WDOGPCELLID2_SPEC>`"]
pub type WDOGPCELLID2 = crate::Reg<wdogpcellid2::WDOGPCELLID2_SPEC>;
#[doc = "PrimeCell ID"]
pub mod wdogpcellid2;
#[doc = "WDOGPCELLID3 register accessor: an alias for `Reg<WDOGPCELLID3_SPEC>`"]
pub type WDOGPCELLID3 = crate::Reg<wdogpcellid3::WDOGPCELLID3_SPEC>;
#[doc = "PrimeCell ID"]
pub mod wdogpcellid3;
