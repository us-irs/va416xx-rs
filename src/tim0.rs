#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    #[doc = "0x04 - The value that counter start from after reaching 0."]
    pub rst_value: crate::Reg<rst_value::RST_VALUE_SPEC>,
    #[doc = "0x08 - The current value of the counter"]
    pub cnt_value: crate::Reg<cnt_value::CNT_VALUE_SPEC>,
    #[doc = "0x0c - Alternate access to the Counter ENABLE bit in the CTRL Register"]
    pub enable: crate::Reg<enable::ENABLE_SPEC>,
    #[doc = "0x10 - The Cascade Control Register. Controls the counter external enable signals"]
    pub csd_ctrl: crate::Reg<csd_ctrl::CSD_CTRL_SPEC>,
    #[doc = "0x14 - Cascade Enable Selection"]
    pub cascade0: crate::Reg<cascade0::CASCADE0_SPEC>,
    #[doc = "0x18 - Cascade Enable Selection"]
    pub cascade1: crate::Reg<cascade1::CASCADE1_SPEC>,
    #[doc = "0x1c - Cascade Enable Selection"]
    pub cascade2: crate::Reg<cascade2::CASCADE2_SPEC>,
    _reserved_8_pwm_value: [u8; 0x04],
    #[doc = "0x24 - The Pulse Width Modulation ValueB"]
    pub pwmb_value: crate::Reg<pwmb_value::PWMB_VALUE_SPEC>,
    _reserved10: [u8; 0x03d4],
    #[doc = "0x3fc - Peripheral ID Register"]
    pub perid: crate::Reg<perid::PERID_SPEC>,
}
impl RegisterBlock {
    #[doc = "0x20 - The Pulse Width Modulation ValueA"]
    #[inline(always)]
    pub fn pwma_value(&self) -> &crate::Reg<pwma_value::PWMA_VALUE_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(32usize)
                as *const crate::Reg<pwma_value::PWMA_VALUE_SPEC>)
        }
    }
    #[doc = "0x20 - The Pulse Width Modulation Value"]
    #[inline(always)]
    pub fn pwm_value(&self) -> &crate::Reg<pwm_value::PWM_VALUE_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(32usize)
                as *const crate::Reg<pwm_value::PWM_VALUE_SPEC>)
        }
    }
}
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "RST_VALUE register accessor: an alias for `Reg<RST_VALUE_SPEC>`"]
pub type RST_VALUE = crate::Reg<rst_value::RST_VALUE_SPEC>;
#[doc = "The value that counter start from after reaching 0."]
pub mod rst_value;
#[doc = "CNT_VALUE register accessor: an alias for `Reg<CNT_VALUE_SPEC>`"]
pub type CNT_VALUE = crate::Reg<cnt_value::CNT_VALUE_SPEC>;
#[doc = "The current value of the counter"]
pub mod cnt_value;
#[doc = "ENABLE register accessor: an alias for `Reg<ENABLE_SPEC>`"]
pub type ENABLE = crate::Reg<enable::ENABLE_SPEC>;
#[doc = "Alternate access to the Counter ENABLE bit in the CTRL Register"]
pub mod enable;
#[doc = "CSD_CTRL register accessor: an alias for `Reg<CSD_CTRL_SPEC>`"]
pub type CSD_CTRL = crate::Reg<csd_ctrl::CSD_CTRL_SPEC>;
#[doc = "The Cascade Control Register. Controls the counter external enable signals"]
pub mod csd_ctrl;
#[doc = "CASCADE0 register accessor: an alias for `Reg<CASCADE0_SPEC>`"]
pub type CASCADE0 = crate::Reg<cascade0::CASCADE0_SPEC>;
#[doc = "Cascade Enable Selection"]
pub mod cascade0;
#[doc = "CASCADE1 register accessor: an alias for `Reg<CASCADE1_SPEC>`"]
pub type CASCADE1 = crate::Reg<cascade1::CASCADE1_SPEC>;
#[doc = "Cascade Enable Selection"]
pub mod cascade1;
#[doc = "CASCADE2 register accessor: an alias for `Reg<CASCADE2_SPEC>`"]
pub type CASCADE2 = crate::Reg<cascade2::CASCADE2_SPEC>;
#[doc = "Cascade Enable Selection"]
pub mod cascade2;
#[doc = "PWM_VALUE register accessor: an alias for `Reg<PWM_VALUE_SPEC>`"]
pub type PWM_VALUE = crate::Reg<pwm_value::PWM_VALUE_SPEC>;
#[doc = "The Pulse Width Modulation Value"]
pub mod pwm_value;
#[doc = "PWMA_VALUE register accessor: an alias for `Reg<PWMA_VALUE_SPEC>`"]
pub type PWMA_VALUE = crate::Reg<pwma_value::PWMA_VALUE_SPEC>;
#[doc = "The Pulse Width Modulation ValueA"]
pub mod pwma_value;
#[doc = "PWMB_VALUE register accessor: an alias for `Reg<PWMB_VALUE_SPEC>`"]
pub type PWMB_VALUE = crate::Reg<pwmb_value::PWMB_VALUE_SPEC>;
#[doc = "The Pulse Width Modulation ValueB"]
pub mod pwmb_value;
#[doc = "PERID register accessor: an alias for `Reg<PERID_SPEC>`"]
pub type PERID = crate::Reg<perid::PERID_SPEC>;
#[doc = "Peripheral ID Register"]
pub mod perid;
