#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x40 - PORTA Pin Configuration Register"]
    pub porta: [crate::Reg<porta::PORTA_SPEC>; 16],
    #[doc = "0x40..0x80 - PORTB Pin Configuration Register"]
    pub portb: [crate::Reg<portb::PORTB_SPEC>; 16],
    #[doc = "0x80..0xc0 - PORTC Pin Configuration Register"]
    pub portc: [crate::Reg<portc::PORTC_SPEC>; 16],
    #[doc = "0xc0..0x100 - PORTD Pin Configuration Register"]
    pub portd: [crate::Reg<portd::PORTD_SPEC>; 16],
    #[doc = "0x100..0x140 - PORTE Pin Configuration Register"]
    pub porte: [crate::Reg<porte::PORTE_SPEC>; 16],
    #[doc = "0x140..0x180 - PORTF Pin Configuration Register"]
    pub portf: [crate::Reg<portf::PORTF_SPEC>; 16],
    #[doc = "0x180..0x1a0 - PORTG Pin Configuration Register"]
    pub portg: [crate::Reg<portg::PORTG_SPEC>; 8],
    _reserved7: [u8; 0x20],
    #[doc = "0x1c0 - Clock divide value. 0 will disable the clock"]
    pub clkdiv0: crate::Reg<clkdiv0::CLKDIV0_SPEC>,
    #[doc = "0x1c4 - Clock divide value. 0 will disable the clock"]
    pub clkdiv1: crate::Reg<clkdiv1::CLKDIV1_SPEC>,
    #[doc = "0x1c8 - Clock divide value. 0 will disable the clock"]
    pub clkdiv2: crate::Reg<clkdiv2::CLKDIV2_SPEC>,
    #[doc = "0x1cc - Clock divide value. 0 will disable the clock"]
    pub clkdiv3: crate::Reg<clkdiv3::CLKDIV3_SPEC>,
    #[doc = "0x1d0 - Clock divide value. 0 will disable the clock"]
    pub clkdiv4: crate::Reg<clkdiv4::CLKDIV4_SPEC>,
    #[doc = "0x1d4 - Clock divide value. 0 will disable the clock"]
    pub clkdiv5: crate::Reg<clkdiv5::CLKDIV5_SPEC>,
    #[doc = "0x1d8 - Clock divide value. 0 will disable the clock"]
    pub clkdiv6: crate::Reg<clkdiv6::CLKDIV6_SPEC>,
    #[doc = "0x1dc - Clock divide value. 0 will disable the clock"]
    pub clkdiv7: crate::Reg<clkdiv7::CLKDIV7_SPEC>,
    _reserved15: [u8; 0x0e1c],
    #[doc = "0xffc - Peripheral ID Register"]
    pub perid: crate::Reg<perid::PERID_SPEC>,
}
#[doc = "PORTA register accessor: an alias for `Reg<PORTA_SPEC>`"]
pub type PORTA = crate::Reg<porta::PORTA_SPEC>;
#[doc = "PORTA Pin Configuration Register"]
pub mod porta;
#[doc = "PORTB register accessor: an alias for `Reg<PORTB_SPEC>`"]
pub type PORTB = crate::Reg<portb::PORTB_SPEC>;
#[doc = "PORTB Pin Configuration Register"]
pub mod portb;
#[doc = "PORTC register accessor: an alias for `Reg<PORTC_SPEC>`"]
pub type PORTC = crate::Reg<portc::PORTC_SPEC>;
#[doc = "PORTC Pin Configuration Register"]
pub mod portc;
#[doc = "PORTD register accessor: an alias for `Reg<PORTD_SPEC>`"]
pub type PORTD = crate::Reg<portd::PORTD_SPEC>;
#[doc = "PORTD Pin Configuration Register"]
pub mod portd;
#[doc = "PORTE register accessor: an alias for `Reg<PORTE_SPEC>`"]
pub type PORTE = crate::Reg<porte::PORTE_SPEC>;
#[doc = "PORTE Pin Configuration Register"]
pub mod porte;
#[doc = "PORTF register accessor: an alias for `Reg<PORTF_SPEC>`"]
pub type PORTF = crate::Reg<portf::PORTF_SPEC>;
#[doc = "PORTF Pin Configuration Register"]
pub mod portf;
#[doc = "PORTG register accessor: an alias for `Reg<PORTG_SPEC>`"]
pub type PORTG = crate::Reg<portg::PORTG_SPEC>;
#[doc = "PORTG Pin Configuration Register"]
pub mod portg;
#[doc = "CLKDIV0 register accessor: an alias for `Reg<CLKDIV0_SPEC>`"]
pub type CLKDIV0 = crate::Reg<clkdiv0::CLKDIV0_SPEC>;
#[doc = "Clock divide value. 0 will disable the clock"]
pub mod clkdiv0;
#[doc = "CLKDIV1 register accessor: an alias for `Reg<CLKDIV1_SPEC>`"]
pub type CLKDIV1 = crate::Reg<clkdiv1::CLKDIV1_SPEC>;
#[doc = "Clock divide value. 0 will disable the clock"]
pub mod clkdiv1;
#[doc = "CLKDIV2 register accessor: an alias for `Reg<CLKDIV2_SPEC>`"]
pub type CLKDIV2 = crate::Reg<clkdiv2::CLKDIV2_SPEC>;
#[doc = "Clock divide value. 0 will disable the clock"]
pub mod clkdiv2;
#[doc = "CLKDIV3 register accessor: an alias for `Reg<CLKDIV3_SPEC>`"]
pub type CLKDIV3 = crate::Reg<clkdiv3::CLKDIV3_SPEC>;
#[doc = "Clock divide value. 0 will disable the clock"]
pub mod clkdiv3;
#[doc = "CLKDIV4 register accessor: an alias for `Reg<CLKDIV4_SPEC>`"]
pub type CLKDIV4 = crate::Reg<clkdiv4::CLKDIV4_SPEC>;
#[doc = "Clock divide value. 0 will disable the clock"]
pub mod clkdiv4;
#[doc = "CLKDIV5 register accessor: an alias for `Reg<CLKDIV5_SPEC>`"]
pub type CLKDIV5 = crate::Reg<clkdiv5::CLKDIV5_SPEC>;
#[doc = "Clock divide value. 0 will disable the clock"]
pub mod clkdiv5;
#[doc = "CLKDIV6 register accessor: an alias for `Reg<CLKDIV6_SPEC>`"]
pub type CLKDIV6 = crate::Reg<clkdiv6::CLKDIV6_SPEC>;
#[doc = "Clock divide value. 0 will disable the clock"]
pub mod clkdiv6;
#[doc = "CLKDIV7 register accessor: an alias for `Reg<CLKDIV7_SPEC>`"]
pub type CLKDIV7 = crate::Reg<clkdiv7::CLKDIV7_SPEC>;
#[doc = "Clock divide value. 0 will disable the clock"]
pub mod clkdiv7;
#[doc = "PERID register accessor: an alias for `Reg<PERID_SPEC>`"]
pub type PERID = crate::Reg<perid::PERID_SPEC>;
#[doc = "Peripheral ID Register"]
pub mod perid;
