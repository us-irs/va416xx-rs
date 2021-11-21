#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Clock Generation Module Control Register 0"]
    pub ctrl0: crate::Reg<ctrl0::CTRL0_SPEC>,
    #[doc = "0x04 - Clock Generation Module Status Register"]
    pub stat: crate::Reg<stat::STAT_SPEC>,
    #[doc = "0x08 - Clock Generation Module Control Register 1"]
    pub ctrl1: crate::Reg<ctrl1::CTRL1_SPEC>,
}
#[doc = "CTRL0 register accessor: an alias for `Reg<CTRL0_SPEC>`"]
pub type CTRL0 = crate::Reg<ctrl0::CTRL0_SPEC>;
#[doc = "Clock Generation Module Control Register 0"]
pub mod ctrl0;
#[doc = "STAT register accessor: an alias for `Reg<STAT_SPEC>`"]
pub type STAT = crate::Reg<stat::STAT_SPEC>;
#[doc = "Clock Generation Module Status Register"]
pub mod stat;
#[doc = "CTRL1 register accessor: an alias for `Reg<CTRL1_SPEC>`"]
pub type CTRL1 = crate::Reg<ctrl1::CTRL1_SPEC>;
#[doc = "Clock Generation Module Control Register 1"]
pub mod ctrl1;
