#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Buffer Status / Control Register"]
    pub cnstat_cmb0: crate::Reg<cnstat_cmb0::CNSTAT_CMB0_SPEC>,
    #[doc = "0x04 - CAN Frame Timestamp"]
    pub tstp_cmb0: crate::Reg<tstp_cmb0::TSTP_CMB0_SPEC>,
    #[doc = "0x08 - CAN Frame Data Word 3"]
    pub data3_cmb0: crate::Reg<data3_cmb0::DATA3_CMB0_SPEC>,
    #[doc = "0x0c - CAN Frame Data Word 2"]
    pub data2_cmb0: crate::Reg<data2_cmb0::DATA2_CMB0_SPEC>,
    #[doc = "0x10 - CAN Frame Data Word 1"]
    pub data1_cmb0: crate::Reg<data1_cmb0::DATA1_CMB0_SPEC>,
    #[doc = "0x14 - CAN Frame Data Word 0"]
    pub data0_cmb0: crate::Reg<data0_cmb0::DATA0_CMB0_SPEC>,
    #[doc = "0x18 - CAN Frame Identifier Word 0"]
    pub id0_cmb0: crate::Reg<id0_cmb0::ID0_CMB0_SPEC>,
    #[doc = "0x1c - CAN Frame Identifier Word 1"]
    pub id1_cmb0: crate::Reg<id1_cmb0::ID1_CMB0_SPEC>,
    #[doc = "0x20 - Buffer Status / Control Register"]
    pub cnstat_cmb1: crate::Reg<cnstat_cmb1::CNSTAT_CMB1_SPEC>,
    #[doc = "0x24 - CAN Frame Timestamp"]
    pub tstp_cmb1: crate::Reg<tstp_cmb1::TSTP_CMB1_SPEC>,
    #[doc = "0x28 - CAN Frame Data Word 3"]
    pub data3_cmb1: crate::Reg<data3_cmb1::DATA3_CMB1_SPEC>,
    #[doc = "0x2c - CAN Frame Data Word 2"]
    pub data2_cmb1: crate::Reg<data2_cmb1::DATA2_CMB1_SPEC>,
    #[doc = "0x30 - CAN Frame Data Word 2"]
    pub data1_cmb1: crate::Reg<data1_cmb1::DATA1_CMB1_SPEC>,
    #[doc = "0x34 - CAN Frame Data Word 0"]
    pub data0_cmb1: crate::Reg<data0_cmb1::DATA0_CMB1_SPEC>,
    #[doc = "0x38 - CAN Frame Identifier Word 0"]
    pub id0_cmb1: crate::Reg<id0_cmb1::ID0_CMB1_SPEC>,
    #[doc = "0x3c - CAN Frame Identifier Word 1"]
    pub id1_cmb1: crate::Reg<id1_cmb1::ID1_CMB1_SPEC>,
    #[doc = "0x40 - Buffer Status / Control Register"]
    pub cnstat_cmb2: crate::Reg<cnstat_cmb2::CNSTAT_CMB2_SPEC>,
    #[doc = "0x44 - CAN Frame Timestamp"]
    pub tstp_cmb2: crate::Reg<tstp_cmb2::TSTP_CMB2_SPEC>,
    #[doc = "0x48 - CAN Frame Data Word 3"]
    pub data3_cmb2: crate::Reg<data3_cmb2::DATA3_CMB2_SPEC>,
    #[doc = "0x4c - CAN Frame Data Word 2"]
    pub data2_cmb2: crate::Reg<data2_cmb2::DATA2_CMB2_SPEC>,
    #[doc = "0x50 - CAN Frame Data Word 2"]
    pub data1_cmb2: crate::Reg<data1_cmb2::DATA1_CMB2_SPEC>,
    #[doc = "0x54 - CAN Frame Data Word 0"]
    pub data0_cmb2: crate::Reg<data0_cmb2::DATA0_CMB2_SPEC>,
    #[doc = "0x58 - CAN Frame Identifier Word 0"]
    pub id0_cmb2: crate::Reg<id0_cmb2::ID0_CMB2_SPEC>,
    #[doc = "0x5c - CAN Frame Identifier Word 1"]
    pub id1_cmb2: crate::Reg<id1_cmb2::ID1_CMB2_SPEC>,
    #[doc = "0x60 - Buffer Status / Control Register"]
    pub cnstat_cmb3: crate::Reg<cnstat_cmb3::CNSTAT_CMB3_SPEC>,
    #[doc = "0x64 - CAN Frame Timestamp"]
    pub tstp_cmb3: crate::Reg<tstp_cmb3::TSTP_CMB3_SPEC>,
    #[doc = "0x68 - CAN Frame Data Word 3"]
    pub data3_cmb3: crate::Reg<data3_cmb3::DATA3_CMB3_SPEC>,
    #[doc = "0x6c - CAN Frame Data Word 2"]
    pub data2_cmb3: crate::Reg<data2_cmb3::DATA2_CMB3_SPEC>,
    #[doc = "0x70 - CAN Frame Data Word 2"]
    pub data1_cmb3: crate::Reg<data1_cmb3::DATA1_CMB3_SPEC>,
    #[doc = "0x74 - CAN Frame Data Word 0"]
    pub data0_cmb3: crate::Reg<data0_cmb3::DATA0_CMB3_SPEC>,
    #[doc = "0x78 - CAN Frame Identifier Word 0"]
    pub id0_cmb3: crate::Reg<id0_cmb3::ID0_CMB3_SPEC>,
    #[doc = "0x7c - CAN Frame Identifier Word 1"]
    pub id1_cmb3: crate::Reg<id1_cmb3::ID1_CMB3_SPEC>,
    #[doc = "0x80 - Buffer Status / Control Register"]
    pub cnstat_cmb4: crate::Reg<cnstat_cmb4::CNSTAT_CMB4_SPEC>,
    #[doc = "0x84 - CAN Frame Timestamp"]
    pub tstp_cmb4: crate::Reg<tstp_cmb4::TSTP_CMB4_SPEC>,
    #[doc = "0x88 - CAN Frame Data Word 3"]
    pub data3_cmb4: crate::Reg<data3_cmb4::DATA3_CMB4_SPEC>,
    #[doc = "0x8c - CAN Frame Data Word 2"]
    pub data2_cmb4: crate::Reg<data2_cmb4::DATA2_CMB4_SPEC>,
    #[doc = "0x90 - CAN Frame Data Word 2"]
    pub data1_cmb4: crate::Reg<data1_cmb4::DATA1_CMB4_SPEC>,
    #[doc = "0x94 - CAN Frame Data Word 0"]
    pub data0_cmb4: crate::Reg<data0_cmb4::DATA0_CMB4_SPEC>,
    #[doc = "0x98 - CAN Frame Identifier Word 0"]
    pub id0_cmb4: crate::Reg<id0_cmb4::ID0_CMB4_SPEC>,
    #[doc = "0x9c - CAN Frame Identifier Word 1"]
    pub id1_cmb4: crate::Reg<id1_cmb4::ID1_CMB4_SPEC>,
    #[doc = "0xa0 - Buffer Status / Control Register"]
    pub cnstat_cmb5: crate::Reg<cnstat_cmb5::CNSTAT_CMB5_SPEC>,
    #[doc = "0xa4 - CAN Frame Timestamp"]
    pub tstp_cmb5: crate::Reg<tstp_cmb5::TSTP_CMB5_SPEC>,
    #[doc = "0xa8 - CAN Frame Data Word 3"]
    pub data3_cmb5: crate::Reg<data3_cmb5::DATA3_CMB5_SPEC>,
    #[doc = "0xac - CAN Frame Data Word 2"]
    pub data2_cmb5: crate::Reg<data2_cmb5::DATA2_CMB5_SPEC>,
    #[doc = "0xb0 - CAN Frame Data Word 2"]
    pub data1_cmb5: crate::Reg<data1_cmb5::DATA1_CMB5_SPEC>,
    #[doc = "0xb4 - CAN Frame Data Word 0"]
    pub data0_cmb5: crate::Reg<data0_cmb5::DATA0_CMB5_SPEC>,
    #[doc = "0xb8 - CAN Frame Identifier Word 0"]
    pub id0_cmb5: crate::Reg<id0_cmb5::ID0_CMB5_SPEC>,
    #[doc = "0xbc - CAN Frame Identifier Word 1"]
    pub id1_cmb5: crate::Reg<id1_cmb5::ID1_CMB5_SPEC>,
    #[doc = "0xc0 - Buffer Status / Control Register"]
    pub cnstat_cmb6: crate::Reg<cnstat_cmb6::CNSTAT_CMB6_SPEC>,
    #[doc = "0xc4 - CAN Frame Timestamp"]
    pub tstp_cmb6: crate::Reg<tstp_cmb6::TSTP_CMB6_SPEC>,
    #[doc = "0xc8 - CAN Frame Data Word 3"]
    pub data3_cmb6: crate::Reg<data3_cmb6::DATA3_CMB6_SPEC>,
    #[doc = "0xcc - CAN Frame Data Word 2"]
    pub data2_cmb6: crate::Reg<data2_cmb6::DATA2_CMB6_SPEC>,
    #[doc = "0xd0 - CAN Frame Data Word 2"]
    pub data1_cmb6: crate::Reg<data1_cmb6::DATA1_CMB6_SPEC>,
    #[doc = "0xd4 - CAN Frame Data Word 0"]
    pub data0_cmb6: crate::Reg<data0_cmb6::DATA0_CMB6_SPEC>,
    #[doc = "0xd8 - CAN Frame Identifier Word 0"]
    pub id0_cmb6: crate::Reg<id0_cmb6::ID0_CMB6_SPEC>,
    #[doc = "0xdc - CAN Frame Identifier Word 1"]
    pub id1_cmb6: crate::Reg<id1_cmb6::ID1_CMB6_SPEC>,
    #[doc = "0xe0 - Buffer Status / Control Register"]
    pub cnstat_cmb7: crate::Reg<cnstat_cmb7::CNSTAT_CMB7_SPEC>,
    #[doc = "0xe4 - CAN Frame Timestamp"]
    pub tstp_cmb7: crate::Reg<tstp_cmb7::TSTP_CMB7_SPEC>,
    #[doc = "0xe8 - CAN Frame Data Word 3"]
    pub data3_cmb7: crate::Reg<data3_cmb7::DATA3_CMB7_SPEC>,
    #[doc = "0xec - CAN Frame Data Word 2"]
    pub data2_cmb7: crate::Reg<data2_cmb7::DATA2_CMB7_SPEC>,
    #[doc = "0xf0 - CAN Frame Data Word 2"]
    pub data1_cmb7: crate::Reg<data1_cmb7::DATA1_CMB7_SPEC>,
    #[doc = "0xf4 - CAN Frame Data Word 0"]
    pub data0_cmb7: crate::Reg<data0_cmb7::DATA0_CMB7_SPEC>,
    #[doc = "0xf8 - CAN Frame Identifier Word 0"]
    pub id0_cmb7: crate::Reg<id0_cmb7::ID0_CMB7_SPEC>,
    #[doc = "0xfc - CAN Frame Identifier Word 1"]
    pub id1_cmb7: crate::Reg<id1_cmb7::ID1_CMB7_SPEC>,
    #[doc = "0x100 - Buffer Status / Control Register"]
    pub cnstat_cmb8: crate::Reg<cnstat_cmb8::CNSTAT_CMB8_SPEC>,
    #[doc = "0x104 - CAN Frame Timestamp"]
    pub tstp_cmb8: crate::Reg<tstp_cmb8::TSTP_CMB8_SPEC>,
    #[doc = "0x108 - CAN Frame Data Word 3"]
    pub data3_cmb8: crate::Reg<data3_cmb8::DATA3_CMB8_SPEC>,
    #[doc = "0x10c - CAN Frame Data Word 2"]
    pub data2_cmb8: crate::Reg<data2_cmb8::DATA2_CMB8_SPEC>,
    #[doc = "0x110 - CAN Frame Data Word 2"]
    pub data1_cmb8: crate::Reg<data1_cmb8::DATA1_CMB8_SPEC>,
    #[doc = "0x114 - CAN Frame Data Word 0"]
    pub data0_cmb8: crate::Reg<data0_cmb8::DATA0_CMB8_SPEC>,
    #[doc = "0x118 - CAN Frame Identifier Word 0"]
    pub id0_cmb8: crate::Reg<id0_cmb8::ID0_CMB8_SPEC>,
    #[doc = "0x11c - CAN Frame Identifier Word 1"]
    pub id1_cmb8: crate::Reg<id1_cmb8::ID1_CMB8_SPEC>,
    #[doc = "0x120 - Buffer Status / Control Register"]
    pub cnstat_cmb9: crate::Reg<cnstat_cmb9::CNSTAT_CMB9_SPEC>,
    #[doc = "0x124 - CAN Frame Timestamp"]
    pub tstp_cmb9: crate::Reg<tstp_cmb9::TSTP_CMB9_SPEC>,
    #[doc = "0x128 - CAN Frame Data Word 3"]
    pub data3_cmb9: crate::Reg<data3_cmb9::DATA3_CMB9_SPEC>,
    #[doc = "0x12c - CAN Frame Data Word 2"]
    pub data2_cmb9: crate::Reg<data2_cmb9::DATA2_CMB9_SPEC>,
    #[doc = "0x130 - CAN Frame Data Word 2"]
    pub data1_cmb9: crate::Reg<data1_cmb9::DATA1_CMB9_SPEC>,
    #[doc = "0x134 - CAN Frame Data Word 0"]
    pub data0_cmb9: crate::Reg<data0_cmb9::DATA0_CMB9_SPEC>,
    #[doc = "0x138 - CAN Frame Identifier Word 0"]
    pub id0_cmb9: crate::Reg<id0_cmb9::ID0_CMB9_SPEC>,
    #[doc = "0x13c - CAN Frame Identifier Word 1"]
    pub id1_cmb9: crate::Reg<id1_cmb9::ID1_CMB9_SPEC>,
    #[doc = "0x140 - Buffer Status / Control Register"]
    pub cnstat_cmb10: crate::Reg<cnstat_cmb10::CNSTAT_CMB10_SPEC>,
    #[doc = "0x144 - CAN Frame Timestamp"]
    pub tstp_cmb10: crate::Reg<tstp_cmb10::TSTP_CMB10_SPEC>,
    #[doc = "0x148 - CAN Frame Data Word 3"]
    pub data3_cmb10: crate::Reg<data3_cmb10::DATA3_CMB10_SPEC>,
    #[doc = "0x14c - CAN Frame Data Word 2"]
    pub data2_cmb10: crate::Reg<data2_cmb10::DATA2_CMB10_SPEC>,
    #[doc = "0x150 - CAN Frame Data Word 2"]
    pub data1_cmb10: crate::Reg<data1_cmb10::DATA1_CMB10_SPEC>,
    #[doc = "0x154 - CAN Frame Data Word 0"]
    pub data0_cmb10: crate::Reg<data0_cmb10::DATA0_CMB10_SPEC>,
    #[doc = "0x158 - CAN Frame Identifier Word 0"]
    pub id0_cmb10: crate::Reg<id0_cmb10::ID0_CMB10_SPEC>,
    #[doc = "0x15c - CAN Frame Identifier Word 1"]
    pub id1_cmb10: crate::Reg<id1_cmb10::ID1_CMB10_SPEC>,
    #[doc = "0x160 - Buffer Status / Control Register"]
    pub cnstat_cmb11: crate::Reg<cnstat_cmb11::CNSTAT_CMB11_SPEC>,
    #[doc = "0x164 - CAN Frame Timestamp"]
    pub tstp_cmb11: crate::Reg<tstp_cmb11::TSTP_CMB11_SPEC>,
    #[doc = "0x168 - CAN Frame Data Word 3"]
    pub data3_cmb11: crate::Reg<data3_cmb11::DATA3_CMB11_SPEC>,
    #[doc = "0x16c - CAN Frame Data Word 2"]
    pub data2_cmb11: crate::Reg<data2_cmb11::DATA2_CMB11_SPEC>,
    #[doc = "0x170 - CAN Frame Data Word 2"]
    pub data1_cmb11: crate::Reg<data1_cmb11::DATA1_CMB11_SPEC>,
    #[doc = "0x174 - CAN Frame Data Word 0"]
    pub data0_cmb11: crate::Reg<data0_cmb11::DATA0_CMB11_SPEC>,
    #[doc = "0x178 - CAN Frame Identifier Word 0"]
    pub id0_cmb11: crate::Reg<id0_cmb11::ID0_CMB11_SPEC>,
    #[doc = "0x17c - CAN Frame Identifier Word 1"]
    pub id1_cmb11: crate::Reg<id1_cmb11::ID1_CMB11_SPEC>,
    #[doc = "0x180 - Buffer Status / Control Register"]
    pub cnstat_cmb12: crate::Reg<cnstat_cmb12::CNSTAT_CMB12_SPEC>,
    #[doc = "0x184 - CAN Frame Timestamp"]
    pub tstp_cmb12: crate::Reg<tstp_cmb12::TSTP_CMB12_SPEC>,
    #[doc = "0x188 - CAN Frame Data Word 3"]
    pub data3_cmb12: crate::Reg<data3_cmb12::DATA3_CMB12_SPEC>,
    #[doc = "0x18c - CAN Frame Data Word 2"]
    pub data2_cmb12: crate::Reg<data2_cmb12::DATA2_CMB12_SPEC>,
    #[doc = "0x190 - CAN Frame Data Word 2"]
    pub data1_cmb12: crate::Reg<data1_cmb12::DATA1_CMB12_SPEC>,
    #[doc = "0x194 - CAN Frame Data Word 0"]
    pub data0_cmb12: crate::Reg<data0_cmb12::DATA0_CMB12_SPEC>,
    #[doc = "0x198 - CAN Frame Identifier Word 0"]
    pub id0_cmb12: crate::Reg<id0_cmb12::ID0_CMB12_SPEC>,
    #[doc = "0x19c - CAN Frame Identifier Word 1"]
    pub id1_cmb12: crate::Reg<id1_cmb12::ID1_CMB12_SPEC>,
    #[doc = "0x1a0 - Buffer Status / Control Register"]
    pub cnstat_cmb13: crate::Reg<cnstat_cmb13::CNSTAT_CMB13_SPEC>,
    #[doc = "0x1a4 - CAN Frame Timestamp"]
    pub tstp_cmb13: crate::Reg<tstp_cmb13::TSTP_CMB13_SPEC>,
    #[doc = "0x1a8 - CAN Frame Data Word 3"]
    pub data3_cmb13: crate::Reg<data3_cmb13::DATA3_CMB13_SPEC>,
    #[doc = "0x1ac - CAN Frame Data Word 2"]
    pub data2_cmb13: crate::Reg<data2_cmb13::DATA2_CMB13_SPEC>,
    #[doc = "0x1b0 - CAN Frame Data Word 2"]
    pub data1_cmb13: crate::Reg<data1_cmb13::DATA1_CMB13_SPEC>,
    #[doc = "0x1b4 - CAN Frame Data Word 0"]
    pub data0_cmb13: crate::Reg<data0_cmb13::DATA0_CMB13_SPEC>,
    #[doc = "0x1b8 - CAN Frame Identifier Word 0"]
    pub id0_cmb13: crate::Reg<id0_cmb13::ID0_CMB13_SPEC>,
    #[doc = "0x1bc - CAN Frame Identifier Word 1"]
    pub id1_cmb13: crate::Reg<id1_cmb13::ID1_CMB13_SPEC>,
    #[doc = "0x1c0 - Buffer Status / Control Register"]
    pub cnstat_cmb14: crate::Reg<cnstat_cmb14::CNSTAT_CMB14_SPEC>,
    #[doc = "0x1c4 - CAN Frame Timestamp"]
    pub tstp_cmb14: crate::Reg<tstp_cmb14::TSTP_CMB14_SPEC>,
    #[doc = "0x1c8 - CAN Frame Data Word 3"]
    pub data3_cmb14: crate::Reg<data3_cmb14::DATA3_CMB14_SPEC>,
    #[doc = "0x1cc - CAN Frame Data Word 2"]
    pub data2_cmb14: crate::Reg<data2_cmb14::DATA2_CMB14_SPEC>,
    #[doc = "0x1d0 - CAN Frame Data Word 2"]
    pub data1_cmb14: crate::Reg<data1_cmb14::DATA1_CMB14_SPEC>,
    #[doc = "0x1d4 - CAN Frame Data Word 0"]
    pub data0_cmb14: crate::Reg<data0_cmb14::DATA0_CMB14_SPEC>,
    #[doc = "0x1d8 - CAN Frame Identifier Word 0"]
    pub id0_cmb14: crate::Reg<id0_cmb14::ID0_CMB14_SPEC>,
    #[doc = "0x1dc - CAN Frame Identifier Word 1"]
    pub id1_cmb14: crate::Reg<id1_cmb14::ID1_CMB14_SPEC>,
    #[doc = "0x1e0 - Buffer Status / Control Register"]
    pub cnstat_hcmb: crate::Reg<cnstat_hcmb::CNSTAT_HCMB_SPEC>,
    #[doc = "0x1e4 - CAN Frame Timestamp"]
    pub tstp_hcmb: crate::Reg<tstp_hcmb::TSTP_HCMB_SPEC>,
    #[doc = "0x1e8 - CAN Frame Data Word 3"]
    pub data3_hcmb: crate::Reg<data3_hcmb::DATA3_HCMB_SPEC>,
    #[doc = "0x1ec - CAN Frame Data Word 2"]
    pub data2_hcmb: crate::Reg<data2_hcmb::DATA2_HCMB_SPEC>,
    #[doc = "0x1f0 - CAN Frame Data Word 2"]
    pub data1_hcmb: crate::Reg<data1_hcmb::DATA1_HCMB_SPEC>,
    #[doc = "0x1f4 - CAN Frame Data Word 0"]
    pub data0_hcmb: crate::Reg<data0_hcmb::DATA0_HCMB_SPEC>,
    #[doc = "0x1f8 - CAN Frame Identifier Word 0"]
    pub id0_hcmb: crate::Reg<id0_hcmb::ID0_HCMB_SPEC>,
    #[doc = "0x1fc - CAN Frame Identifier Word 1"]
    pub id1_hcmb: crate::Reg<id1_hcmb::ID1_HCMB_SPEC>,
    #[doc = "0x200 - CAN Global Configuration Register"]
    pub cgcr: crate::Reg<cgcr::CGCR_SPEC>,
    #[doc = "0x204 - CAN Timing Register"]
    pub ctim: crate::Reg<ctim::CTIM_SPEC>,
    #[doc = "0x208 - CAN Global Mask Extension"]
    pub gmskx: crate::Reg<gmskx::GMSKX_SPEC>,
    #[doc = "0x20c - CAN Global Mask Base"]
    pub gmskb: crate::Reg<gmskb::GMSKB_SPEC>,
    #[doc = "0x210 - CAN Basic Mask Extension"]
    pub bmskx: crate::Reg<bmskx::BMSKX_SPEC>,
    #[doc = "0x214 - CAN Basic Mask Base"]
    pub bmskb: crate::Reg<bmskb::BMSKB_SPEC>,
    #[doc = "0x218 - CAN Interrupt Enable Register"]
    pub cien: crate::Reg<cien::CIEN_SPEC>,
    #[doc = "0x21c - CAN Interrupt Pending Register"]
    pub cipnd: crate::Reg<cipnd::CIPND_SPEC>,
    #[doc = "0x220 - CAN Interrupt Clear Register"]
    pub ciclr: crate::Reg<ciclr::CICLR_SPEC>,
    #[doc = "0x224 - CAN Interrupt Code Enable Register"]
    pub cicen: crate::Reg<cicen::CICEN_SPEC>,
    #[doc = "0x228 - CAN Status Pending Register"]
    pub cstpnd: crate::Reg<cstpnd::CSTPND_SPEC>,
    #[doc = "0x22c - CAN Error Counter Register"]
    pub canec: crate::Reg<canec::CANEC_SPEC>,
    #[doc = "0x230 - CAN Error Diagnostic Register"]
    pub cediag: crate::Reg<cediag::CEDIAG_SPEC>,
    #[doc = "0x234 - CAN Timer Register"]
    pub ctmr: crate::Reg<ctmr::CTMR_SPEC>,
}
#[doc = "CNSTAT_CMB0 register accessor: an alias for `Reg<CNSTAT_CMB0_SPEC>`"]
pub type CNSTAT_CMB0 = crate::Reg<cnstat_cmb0::CNSTAT_CMB0_SPEC>;
#[doc = "Buffer Status / Control Register"]
pub mod cnstat_cmb0;
#[doc = "TSTP_CMB0 register accessor: an alias for `Reg<TSTP_CMB0_SPEC>`"]
pub type TSTP_CMB0 = crate::Reg<tstp_cmb0::TSTP_CMB0_SPEC>;
#[doc = "CAN Frame Timestamp"]
pub mod tstp_cmb0;
#[doc = "DATA3_CMB0 register accessor: an alias for `Reg<DATA3_CMB0_SPEC>`"]
pub type DATA3_CMB0 = crate::Reg<data3_cmb0::DATA3_CMB0_SPEC>;
#[doc = "CAN Frame Data Word 3"]
pub mod data3_cmb0;
#[doc = "DATA2_CMB0 register accessor: an alias for `Reg<DATA2_CMB0_SPEC>`"]
pub type DATA2_CMB0 = crate::Reg<data2_cmb0::DATA2_CMB0_SPEC>;
#[doc = "CAN Frame Data Word 2"]
pub mod data2_cmb0;
#[doc = "DATA1_CMB0 register accessor: an alias for `Reg<DATA1_CMB0_SPEC>`"]
pub type DATA1_CMB0 = crate::Reg<data1_cmb0::DATA1_CMB0_SPEC>;
#[doc = "CAN Frame Data Word 1"]
pub mod data1_cmb0;
#[doc = "DATA0_CMB0 register accessor: an alias for `Reg<DATA0_CMB0_SPEC>`"]
pub type DATA0_CMB0 = crate::Reg<data0_cmb0::DATA0_CMB0_SPEC>;
#[doc = "CAN Frame Data Word 0"]
pub mod data0_cmb0;
#[doc = "ID0_CMB0 register accessor: an alias for `Reg<ID0_CMB0_SPEC>`"]
pub type ID0_CMB0 = crate::Reg<id0_cmb0::ID0_CMB0_SPEC>;
#[doc = "CAN Frame Identifier Word 0"]
pub mod id0_cmb0;
#[doc = "ID1_CMB0 register accessor: an alias for `Reg<ID1_CMB0_SPEC>`"]
pub type ID1_CMB0 = crate::Reg<id1_cmb0::ID1_CMB0_SPEC>;
#[doc = "CAN Frame Identifier Word 1"]
pub mod id1_cmb0;
#[doc = "CNSTAT_CMB1 register accessor: an alias for `Reg<CNSTAT_CMB1_SPEC>`"]
pub type CNSTAT_CMB1 = crate::Reg<cnstat_cmb1::CNSTAT_CMB1_SPEC>;
#[doc = "Buffer Status / Control Register"]
pub mod cnstat_cmb1;
#[doc = "TSTP_CMB1 register accessor: an alias for `Reg<TSTP_CMB1_SPEC>`"]
pub type TSTP_CMB1 = crate::Reg<tstp_cmb1::TSTP_CMB1_SPEC>;
#[doc = "CAN Frame Timestamp"]
pub mod tstp_cmb1;
#[doc = "DATA3_CMB1 register accessor: an alias for `Reg<DATA3_CMB1_SPEC>`"]
pub type DATA3_CMB1 = crate::Reg<data3_cmb1::DATA3_CMB1_SPEC>;
#[doc = "CAN Frame Data Word 3"]
pub mod data3_cmb1;
#[doc = "DATA2_CMB1 register accessor: an alias for `Reg<DATA2_CMB1_SPEC>`"]
pub type DATA2_CMB1 = crate::Reg<data2_cmb1::DATA2_CMB1_SPEC>;
#[doc = "CAN Frame Data Word 2"]
pub mod data2_cmb1;
#[doc = "DATA1_CMB1 register accessor: an alias for `Reg<DATA1_CMB1_SPEC>`"]
pub type DATA1_CMB1 = crate::Reg<data1_cmb1::DATA1_CMB1_SPEC>;
#[doc = "CAN Frame Data Word 2"]
pub mod data1_cmb1;
#[doc = "DATA0_CMB1 register accessor: an alias for `Reg<DATA0_CMB1_SPEC>`"]
pub type DATA0_CMB1 = crate::Reg<data0_cmb1::DATA0_CMB1_SPEC>;
#[doc = "CAN Frame Data Word 0"]
pub mod data0_cmb1;
#[doc = "ID0_CMB1 register accessor: an alias for `Reg<ID0_CMB1_SPEC>`"]
pub type ID0_CMB1 = crate::Reg<id0_cmb1::ID0_CMB1_SPEC>;
#[doc = "CAN Frame Identifier Word 0"]
pub mod id0_cmb1;
#[doc = "ID1_CMB1 register accessor: an alias for `Reg<ID1_CMB1_SPEC>`"]
pub type ID1_CMB1 = crate::Reg<id1_cmb1::ID1_CMB1_SPEC>;
#[doc = "CAN Frame Identifier Word 1"]
pub mod id1_cmb1;
#[doc = "CNSTAT_CMB2 register accessor: an alias for `Reg<CNSTAT_CMB2_SPEC>`"]
pub type CNSTAT_CMB2 = crate::Reg<cnstat_cmb2::CNSTAT_CMB2_SPEC>;
#[doc = "Buffer Status / Control Register"]
pub mod cnstat_cmb2;
#[doc = "TSTP_CMB2 register accessor: an alias for `Reg<TSTP_CMB2_SPEC>`"]
pub type TSTP_CMB2 = crate::Reg<tstp_cmb2::TSTP_CMB2_SPEC>;
#[doc = "CAN Frame Timestamp"]
pub mod tstp_cmb2;
#[doc = "DATA3_CMB2 register accessor: an alias for `Reg<DATA3_CMB2_SPEC>`"]
pub type DATA3_CMB2 = crate::Reg<data3_cmb2::DATA3_CMB2_SPEC>;
#[doc = "CAN Frame Data Word 3"]
pub mod data3_cmb2;
#[doc = "DATA2_CMB2 register accessor: an alias for `Reg<DATA2_CMB2_SPEC>`"]
pub type DATA2_CMB2 = crate::Reg<data2_cmb2::DATA2_CMB2_SPEC>;
#[doc = "CAN Frame Data Word 2"]
pub mod data2_cmb2;
#[doc = "DATA1_CMB2 register accessor: an alias for `Reg<DATA1_CMB2_SPEC>`"]
pub type DATA1_CMB2 = crate::Reg<data1_cmb2::DATA1_CMB2_SPEC>;
#[doc = "CAN Frame Data Word 2"]
pub mod data1_cmb2;
#[doc = "DATA0_CMB2 register accessor: an alias for `Reg<DATA0_CMB2_SPEC>`"]
pub type DATA0_CMB2 = crate::Reg<data0_cmb2::DATA0_CMB2_SPEC>;
#[doc = "CAN Frame Data Word 0"]
pub mod data0_cmb2;
#[doc = "ID0_CMB2 register accessor: an alias for `Reg<ID0_CMB2_SPEC>`"]
pub type ID0_CMB2 = crate::Reg<id0_cmb2::ID0_CMB2_SPEC>;
#[doc = "CAN Frame Identifier Word 0"]
pub mod id0_cmb2;
#[doc = "ID1_CMB2 register accessor: an alias for `Reg<ID1_CMB2_SPEC>`"]
pub type ID1_CMB2 = crate::Reg<id1_cmb2::ID1_CMB2_SPEC>;
#[doc = "CAN Frame Identifier Word 1"]
pub mod id1_cmb2;
#[doc = "CNSTAT_CMB3 register accessor: an alias for `Reg<CNSTAT_CMB3_SPEC>`"]
pub type CNSTAT_CMB3 = crate::Reg<cnstat_cmb3::CNSTAT_CMB3_SPEC>;
#[doc = "Buffer Status / Control Register"]
pub mod cnstat_cmb3;
#[doc = "TSTP_CMB3 register accessor: an alias for `Reg<TSTP_CMB3_SPEC>`"]
pub type TSTP_CMB3 = crate::Reg<tstp_cmb3::TSTP_CMB3_SPEC>;
#[doc = "CAN Frame Timestamp"]
pub mod tstp_cmb3;
#[doc = "DATA3_CMB3 register accessor: an alias for `Reg<DATA3_CMB3_SPEC>`"]
pub type DATA3_CMB3 = crate::Reg<data3_cmb3::DATA3_CMB3_SPEC>;
#[doc = "CAN Frame Data Word 3"]
pub mod data3_cmb3;
#[doc = "DATA2_CMB3 register accessor: an alias for `Reg<DATA2_CMB3_SPEC>`"]
pub type DATA2_CMB3 = crate::Reg<data2_cmb3::DATA2_CMB3_SPEC>;
#[doc = "CAN Frame Data Word 2"]
pub mod data2_cmb3;
#[doc = "DATA1_CMB3 register accessor: an alias for `Reg<DATA1_CMB3_SPEC>`"]
pub type DATA1_CMB3 = crate::Reg<data1_cmb3::DATA1_CMB3_SPEC>;
#[doc = "CAN Frame Data Word 2"]
pub mod data1_cmb3;
#[doc = "DATA0_CMB3 register accessor: an alias for `Reg<DATA0_CMB3_SPEC>`"]
pub type DATA0_CMB3 = crate::Reg<data0_cmb3::DATA0_CMB3_SPEC>;
#[doc = "CAN Frame Data Word 0"]
pub mod data0_cmb3;
#[doc = "ID0_CMB3 register accessor: an alias for `Reg<ID0_CMB3_SPEC>`"]
pub type ID0_CMB3 = crate::Reg<id0_cmb3::ID0_CMB3_SPEC>;
#[doc = "CAN Frame Identifier Word 0"]
pub mod id0_cmb3;
#[doc = "ID1_CMB3 register accessor: an alias for `Reg<ID1_CMB3_SPEC>`"]
pub type ID1_CMB3 = crate::Reg<id1_cmb3::ID1_CMB3_SPEC>;
#[doc = "CAN Frame Identifier Word 1"]
pub mod id1_cmb3;
#[doc = "CNSTAT_CMB4 register accessor: an alias for `Reg<CNSTAT_CMB4_SPEC>`"]
pub type CNSTAT_CMB4 = crate::Reg<cnstat_cmb4::CNSTAT_CMB4_SPEC>;
#[doc = "Buffer Status / Control Register"]
pub mod cnstat_cmb4;
#[doc = "TSTP_CMB4 register accessor: an alias for `Reg<TSTP_CMB4_SPEC>`"]
pub type TSTP_CMB4 = crate::Reg<tstp_cmb4::TSTP_CMB4_SPEC>;
#[doc = "CAN Frame Timestamp"]
pub mod tstp_cmb4;
#[doc = "DATA3_CMB4 register accessor: an alias for `Reg<DATA3_CMB4_SPEC>`"]
pub type DATA3_CMB4 = crate::Reg<data3_cmb4::DATA3_CMB4_SPEC>;
#[doc = "CAN Frame Data Word 3"]
pub mod data3_cmb4;
#[doc = "DATA2_CMB4 register accessor: an alias for `Reg<DATA2_CMB4_SPEC>`"]
pub type DATA2_CMB4 = crate::Reg<data2_cmb4::DATA2_CMB4_SPEC>;
#[doc = "CAN Frame Data Word 2"]
pub mod data2_cmb4;
#[doc = "DATA1_CMB4 register accessor: an alias for `Reg<DATA1_CMB4_SPEC>`"]
pub type DATA1_CMB4 = crate::Reg<data1_cmb4::DATA1_CMB4_SPEC>;
#[doc = "CAN Frame Data Word 2"]
pub mod data1_cmb4;
#[doc = "DATA0_CMB4 register accessor: an alias for `Reg<DATA0_CMB4_SPEC>`"]
pub type DATA0_CMB4 = crate::Reg<data0_cmb4::DATA0_CMB4_SPEC>;
#[doc = "CAN Frame Data Word 0"]
pub mod data0_cmb4;
#[doc = "ID0_CMB4 register accessor: an alias for `Reg<ID0_CMB4_SPEC>`"]
pub type ID0_CMB4 = crate::Reg<id0_cmb4::ID0_CMB4_SPEC>;
#[doc = "CAN Frame Identifier Word 0"]
pub mod id0_cmb4;
#[doc = "ID1_CMB4 register accessor: an alias for `Reg<ID1_CMB4_SPEC>`"]
pub type ID1_CMB4 = crate::Reg<id1_cmb4::ID1_CMB4_SPEC>;
#[doc = "CAN Frame Identifier Word 1"]
pub mod id1_cmb4;
#[doc = "CNSTAT_CMB5 register accessor: an alias for `Reg<CNSTAT_CMB5_SPEC>`"]
pub type CNSTAT_CMB5 = crate::Reg<cnstat_cmb5::CNSTAT_CMB5_SPEC>;
#[doc = "Buffer Status / Control Register"]
pub mod cnstat_cmb5;
#[doc = "TSTP_CMB5 register accessor: an alias for `Reg<TSTP_CMB5_SPEC>`"]
pub type TSTP_CMB5 = crate::Reg<tstp_cmb5::TSTP_CMB5_SPEC>;
#[doc = "CAN Frame Timestamp"]
pub mod tstp_cmb5;
#[doc = "DATA3_CMB5 register accessor: an alias for `Reg<DATA3_CMB5_SPEC>`"]
pub type DATA3_CMB5 = crate::Reg<data3_cmb5::DATA3_CMB5_SPEC>;
#[doc = "CAN Frame Data Word 3"]
pub mod data3_cmb5;
#[doc = "DATA2_CMB5 register accessor: an alias for `Reg<DATA2_CMB5_SPEC>`"]
pub type DATA2_CMB5 = crate::Reg<data2_cmb5::DATA2_CMB5_SPEC>;
#[doc = "CAN Frame Data Word 2"]
pub mod data2_cmb5;
#[doc = "DATA1_CMB5 register accessor: an alias for `Reg<DATA1_CMB5_SPEC>`"]
pub type DATA1_CMB5 = crate::Reg<data1_cmb5::DATA1_CMB5_SPEC>;
#[doc = "CAN Frame Data Word 2"]
pub mod data1_cmb5;
#[doc = "DATA0_CMB5 register accessor: an alias for `Reg<DATA0_CMB5_SPEC>`"]
pub type DATA0_CMB5 = crate::Reg<data0_cmb5::DATA0_CMB5_SPEC>;
#[doc = "CAN Frame Data Word 0"]
pub mod data0_cmb5;
#[doc = "ID0_CMB5 register accessor: an alias for `Reg<ID0_CMB5_SPEC>`"]
pub type ID0_CMB5 = crate::Reg<id0_cmb5::ID0_CMB5_SPEC>;
#[doc = "CAN Frame Identifier Word 0"]
pub mod id0_cmb5;
#[doc = "ID1_CMB5 register accessor: an alias for `Reg<ID1_CMB5_SPEC>`"]
pub type ID1_CMB5 = crate::Reg<id1_cmb5::ID1_CMB5_SPEC>;
#[doc = "CAN Frame Identifier Word 1"]
pub mod id1_cmb5;
#[doc = "CNSTAT_CMB6 register accessor: an alias for `Reg<CNSTAT_CMB6_SPEC>`"]
pub type CNSTAT_CMB6 = crate::Reg<cnstat_cmb6::CNSTAT_CMB6_SPEC>;
#[doc = "Buffer Status / Control Register"]
pub mod cnstat_cmb6;
#[doc = "TSTP_CMB6 register accessor: an alias for `Reg<TSTP_CMB6_SPEC>`"]
pub type TSTP_CMB6 = crate::Reg<tstp_cmb6::TSTP_CMB6_SPEC>;
#[doc = "CAN Frame Timestamp"]
pub mod tstp_cmb6;
#[doc = "DATA3_CMB6 register accessor: an alias for `Reg<DATA3_CMB6_SPEC>`"]
pub type DATA3_CMB6 = crate::Reg<data3_cmb6::DATA3_CMB6_SPEC>;
#[doc = "CAN Frame Data Word 3"]
pub mod data3_cmb6;
#[doc = "DATA2_CMB6 register accessor: an alias for `Reg<DATA2_CMB6_SPEC>`"]
pub type DATA2_CMB6 = crate::Reg<data2_cmb6::DATA2_CMB6_SPEC>;
#[doc = "CAN Frame Data Word 2"]
pub mod data2_cmb6;
#[doc = "DATA1_CMB6 register accessor: an alias for `Reg<DATA1_CMB6_SPEC>`"]
pub type DATA1_CMB6 = crate::Reg<data1_cmb6::DATA1_CMB6_SPEC>;
#[doc = "CAN Frame Data Word 2"]
pub mod data1_cmb6;
#[doc = "DATA0_CMB6 register accessor: an alias for `Reg<DATA0_CMB6_SPEC>`"]
pub type DATA0_CMB6 = crate::Reg<data0_cmb6::DATA0_CMB6_SPEC>;
#[doc = "CAN Frame Data Word 0"]
pub mod data0_cmb6;
#[doc = "ID0_CMB6 register accessor: an alias for `Reg<ID0_CMB6_SPEC>`"]
pub type ID0_CMB6 = crate::Reg<id0_cmb6::ID0_CMB6_SPEC>;
#[doc = "CAN Frame Identifier Word 0"]
pub mod id0_cmb6;
#[doc = "ID1_CMB6 register accessor: an alias for `Reg<ID1_CMB6_SPEC>`"]
pub type ID1_CMB6 = crate::Reg<id1_cmb6::ID1_CMB6_SPEC>;
#[doc = "CAN Frame Identifier Word 1"]
pub mod id1_cmb6;
#[doc = "CNSTAT_CMB7 register accessor: an alias for `Reg<CNSTAT_CMB7_SPEC>`"]
pub type CNSTAT_CMB7 = crate::Reg<cnstat_cmb7::CNSTAT_CMB7_SPEC>;
#[doc = "Buffer Status / Control Register"]
pub mod cnstat_cmb7;
#[doc = "TSTP_CMB7 register accessor: an alias for `Reg<TSTP_CMB7_SPEC>`"]
pub type TSTP_CMB7 = crate::Reg<tstp_cmb7::TSTP_CMB7_SPEC>;
#[doc = "CAN Frame Timestamp"]
pub mod tstp_cmb7;
#[doc = "DATA3_CMB7 register accessor: an alias for `Reg<DATA3_CMB7_SPEC>`"]
pub type DATA3_CMB7 = crate::Reg<data3_cmb7::DATA3_CMB7_SPEC>;
#[doc = "CAN Frame Data Word 3"]
pub mod data3_cmb7;
#[doc = "DATA2_CMB7 register accessor: an alias for `Reg<DATA2_CMB7_SPEC>`"]
pub type DATA2_CMB7 = crate::Reg<data2_cmb7::DATA2_CMB7_SPEC>;
#[doc = "CAN Frame Data Word 2"]
pub mod data2_cmb7;
#[doc = "DATA1_CMB7 register accessor: an alias for `Reg<DATA1_CMB7_SPEC>`"]
pub type DATA1_CMB7 = crate::Reg<data1_cmb7::DATA1_CMB7_SPEC>;
#[doc = "CAN Frame Data Word 2"]
pub mod data1_cmb7;
#[doc = "DATA0_CMB7 register accessor: an alias for `Reg<DATA0_CMB7_SPEC>`"]
pub type DATA0_CMB7 = crate::Reg<data0_cmb7::DATA0_CMB7_SPEC>;
#[doc = "CAN Frame Data Word 0"]
pub mod data0_cmb7;
#[doc = "ID0_CMB7 register accessor: an alias for `Reg<ID0_CMB7_SPEC>`"]
pub type ID0_CMB7 = crate::Reg<id0_cmb7::ID0_CMB7_SPEC>;
#[doc = "CAN Frame Identifier Word 0"]
pub mod id0_cmb7;
#[doc = "ID1_CMB7 register accessor: an alias for `Reg<ID1_CMB7_SPEC>`"]
pub type ID1_CMB7 = crate::Reg<id1_cmb7::ID1_CMB7_SPEC>;
#[doc = "CAN Frame Identifier Word 1"]
pub mod id1_cmb7;
#[doc = "CNSTAT_CMB8 register accessor: an alias for `Reg<CNSTAT_CMB8_SPEC>`"]
pub type CNSTAT_CMB8 = crate::Reg<cnstat_cmb8::CNSTAT_CMB8_SPEC>;
#[doc = "Buffer Status / Control Register"]
pub mod cnstat_cmb8;
#[doc = "TSTP_CMB8 register accessor: an alias for `Reg<TSTP_CMB8_SPEC>`"]
pub type TSTP_CMB8 = crate::Reg<tstp_cmb8::TSTP_CMB8_SPEC>;
#[doc = "CAN Frame Timestamp"]
pub mod tstp_cmb8;
#[doc = "DATA3_CMB8 register accessor: an alias for `Reg<DATA3_CMB8_SPEC>`"]
pub type DATA3_CMB8 = crate::Reg<data3_cmb8::DATA3_CMB8_SPEC>;
#[doc = "CAN Frame Data Word 3"]
pub mod data3_cmb8;
#[doc = "DATA2_CMB8 register accessor: an alias for `Reg<DATA2_CMB8_SPEC>`"]
pub type DATA2_CMB8 = crate::Reg<data2_cmb8::DATA2_CMB8_SPEC>;
#[doc = "CAN Frame Data Word 2"]
pub mod data2_cmb8;
#[doc = "DATA1_CMB8 register accessor: an alias for `Reg<DATA1_CMB8_SPEC>`"]
pub type DATA1_CMB8 = crate::Reg<data1_cmb8::DATA1_CMB8_SPEC>;
#[doc = "CAN Frame Data Word 2"]
pub mod data1_cmb8;
#[doc = "DATA0_CMB8 register accessor: an alias for `Reg<DATA0_CMB8_SPEC>`"]
pub type DATA0_CMB8 = crate::Reg<data0_cmb8::DATA0_CMB8_SPEC>;
#[doc = "CAN Frame Data Word 0"]
pub mod data0_cmb8;
#[doc = "ID0_CMB8 register accessor: an alias for `Reg<ID0_CMB8_SPEC>`"]
pub type ID0_CMB8 = crate::Reg<id0_cmb8::ID0_CMB8_SPEC>;
#[doc = "CAN Frame Identifier Word 0"]
pub mod id0_cmb8;
#[doc = "ID1_CMB8 register accessor: an alias for `Reg<ID1_CMB8_SPEC>`"]
pub type ID1_CMB8 = crate::Reg<id1_cmb8::ID1_CMB8_SPEC>;
#[doc = "CAN Frame Identifier Word 1"]
pub mod id1_cmb8;
#[doc = "CNSTAT_CMB9 register accessor: an alias for `Reg<CNSTAT_CMB9_SPEC>`"]
pub type CNSTAT_CMB9 = crate::Reg<cnstat_cmb9::CNSTAT_CMB9_SPEC>;
#[doc = "Buffer Status / Control Register"]
pub mod cnstat_cmb9;
#[doc = "TSTP_CMB9 register accessor: an alias for `Reg<TSTP_CMB9_SPEC>`"]
pub type TSTP_CMB9 = crate::Reg<tstp_cmb9::TSTP_CMB9_SPEC>;
#[doc = "CAN Frame Timestamp"]
pub mod tstp_cmb9;
#[doc = "DATA3_CMB9 register accessor: an alias for `Reg<DATA3_CMB9_SPEC>`"]
pub type DATA3_CMB9 = crate::Reg<data3_cmb9::DATA3_CMB9_SPEC>;
#[doc = "CAN Frame Data Word 3"]
pub mod data3_cmb9;
#[doc = "DATA2_CMB9 register accessor: an alias for `Reg<DATA2_CMB9_SPEC>`"]
pub type DATA2_CMB9 = crate::Reg<data2_cmb9::DATA2_CMB9_SPEC>;
#[doc = "CAN Frame Data Word 2"]
pub mod data2_cmb9;
#[doc = "DATA1_CMB9 register accessor: an alias for `Reg<DATA1_CMB9_SPEC>`"]
pub type DATA1_CMB9 = crate::Reg<data1_cmb9::DATA1_CMB9_SPEC>;
#[doc = "CAN Frame Data Word 2"]
pub mod data1_cmb9;
#[doc = "DATA0_CMB9 register accessor: an alias for `Reg<DATA0_CMB9_SPEC>`"]
pub type DATA0_CMB9 = crate::Reg<data0_cmb9::DATA0_CMB9_SPEC>;
#[doc = "CAN Frame Data Word 0"]
pub mod data0_cmb9;
#[doc = "ID0_CMB9 register accessor: an alias for `Reg<ID0_CMB9_SPEC>`"]
pub type ID0_CMB9 = crate::Reg<id0_cmb9::ID0_CMB9_SPEC>;
#[doc = "CAN Frame Identifier Word 0"]
pub mod id0_cmb9;
#[doc = "ID1_CMB9 register accessor: an alias for `Reg<ID1_CMB9_SPEC>`"]
pub type ID1_CMB9 = crate::Reg<id1_cmb9::ID1_CMB9_SPEC>;
#[doc = "CAN Frame Identifier Word 1"]
pub mod id1_cmb9;
#[doc = "CNSTAT_CMB10 register accessor: an alias for `Reg<CNSTAT_CMB10_SPEC>`"]
pub type CNSTAT_CMB10 = crate::Reg<cnstat_cmb10::CNSTAT_CMB10_SPEC>;
#[doc = "Buffer Status / Control Register"]
pub mod cnstat_cmb10;
#[doc = "TSTP_CMB10 register accessor: an alias for `Reg<TSTP_CMB10_SPEC>`"]
pub type TSTP_CMB10 = crate::Reg<tstp_cmb10::TSTP_CMB10_SPEC>;
#[doc = "CAN Frame Timestamp"]
pub mod tstp_cmb10;
#[doc = "DATA3_CMB10 register accessor: an alias for `Reg<DATA3_CMB10_SPEC>`"]
pub type DATA3_CMB10 = crate::Reg<data3_cmb10::DATA3_CMB10_SPEC>;
#[doc = "CAN Frame Data Word 3"]
pub mod data3_cmb10;
#[doc = "DATA2_CMB10 register accessor: an alias for `Reg<DATA2_CMB10_SPEC>`"]
pub type DATA2_CMB10 = crate::Reg<data2_cmb10::DATA2_CMB10_SPEC>;
#[doc = "CAN Frame Data Word 2"]
pub mod data2_cmb10;
#[doc = "DATA1_CMB10 register accessor: an alias for `Reg<DATA1_CMB10_SPEC>`"]
pub type DATA1_CMB10 = crate::Reg<data1_cmb10::DATA1_CMB10_SPEC>;
#[doc = "CAN Frame Data Word 2"]
pub mod data1_cmb10;
#[doc = "DATA0_CMB10 register accessor: an alias for `Reg<DATA0_CMB10_SPEC>`"]
pub type DATA0_CMB10 = crate::Reg<data0_cmb10::DATA0_CMB10_SPEC>;
#[doc = "CAN Frame Data Word 0"]
pub mod data0_cmb10;
#[doc = "ID0_CMB10 register accessor: an alias for `Reg<ID0_CMB10_SPEC>`"]
pub type ID0_CMB10 = crate::Reg<id0_cmb10::ID0_CMB10_SPEC>;
#[doc = "CAN Frame Identifier Word 0"]
pub mod id0_cmb10;
#[doc = "ID1_CMB10 register accessor: an alias for `Reg<ID1_CMB10_SPEC>`"]
pub type ID1_CMB10 = crate::Reg<id1_cmb10::ID1_CMB10_SPEC>;
#[doc = "CAN Frame Identifier Word 1"]
pub mod id1_cmb10;
#[doc = "CNSTAT_CMB11 register accessor: an alias for `Reg<CNSTAT_CMB11_SPEC>`"]
pub type CNSTAT_CMB11 = crate::Reg<cnstat_cmb11::CNSTAT_CMB11_SPEC>;
#[doc = "Buffer Status / Control Register"]
pub mod cnstat_cmb11;
#[doc = "TSTP_CMB11 register accessor: an alias for `Reg<TSTP_CMB11_SPEC>`"]
pub type TSTP_CMB11 = crate::Reg<tstp_cmb11::TSTP_CMB11_SPEC>;
#[doc = "CAN Frame Timestamp"]
pub mod tstp_cmb11;
#[doc = "DATA3_CMB11 register accessor: an alias for `Reg<DATA3_CMB11_SPEC>`"]
pub type DATA3_CMB11 = crate::Reg<data3_cmb11::DATA3_CMB11_SPEC>;
#[doc = "CAN Frame Data Word 3"]
pub mod data3_cmb11;
#[doc = "DATA2_CMB11 register accessor: an alias for `Reg<DATA2_CMB11_SPEC>`"]
pub type DATA2_CMB11 = crate::Reg<data2_cmb11::DATA2_CMB11_SPEC>;
#[doc = "CAN Frame Data Word 2"]
pub mod data2_cmb11;
#[doc = "DATA1_CMB11 register accessor: an alias for `Reg<DATA1_CMB11_SPEC>`"]
pub type DATA1_CMB11 = crate::Reg<data1_cmb11::DATA1_CMB11_SPEC>;
#[doc = "CAN Frame Data Word 2"]
pub mod data1_cmb11;
#[doc = "DATA0_CMB11 register accessor: an alias for `Reg<DATA0_CMB11_SPEC>`"]
pub type DATA0_CMB11 = crate::Reg<data0_cmb11::DATA0_CMB11_SPEC>;
#[doc = "CAN Frame Data Word 0"]
pub mod data0_cmb11;
#[doc = "ID0_CMB11 register accessor: an alias for `Reg<ID0_CMB11_SPEC>`"]
pub type ID0_CMB11 = crate::Reg<id0_cmb11::ID0_CMB11_SPEC>;
#[doc = "CAN Frame Identifier Word 0"]
pub mod id0_cmb11;
#[doc = "ID1_CMB11 register accessor: an alias for `Reg<ID1_CMB11_SPEC>`"]
pub type ID1_CMB11 = crate::Reg<id1_cmb11::ID1_CMB11_SPEC>;
#[doc = "CAN Frame Identifier Word 1"]
pub mod id1_cmb11;
#[doc = "CNSTAT_CMB12 register accessor: an alias for `Reg<CNSTAT_CMB12_SPEC>`"]
pub type CNSTAT_CMB12 = crate::Reg<cnstat_cmb12::CNSTAT_CMB12_SPEC>;
#[doc = "Buffer Status / Control Register"]
pub mod cnstat_cmb12;
#[doc = "TSTP_CMB12 register accessor: an alias for `Reg<TSTP_CMB12_SPEC>`"]
pub type TSTP_CMB12 = crate::Reg<tstp_cmb12::TSTP_CMB12_SPEC>;
#[doc = "CAN Frame Timestamp"]
pub mod tstp_cmb12;
#[doc = "DATA3_CMB12 register accessor: an alias for `Reg<DATA3_CMB12_SPEC>`"]
pub type DATA3_CMB12 = crate::Reg<data3_cmb12::DATA3_CMB12_SPEC>;
#[doc = "CAN Frame Data Word 3"]
pub mod data3_cmb12;
#[doc = "DATA2_CMB12 register accessor: an alias for `Reg<DATA2_CMB12_SPEC>`"]
pub type DATA2_CMB12 = crate::Reg<data2_cmb12::DATA2_CMB12_SPEC>;
#[doc = "CAN Frame Data Word 2"]
pub mod data2_cmb12;
#[doc = "DATA1_CMB12 register accessor: an alias for `Reg<DATA1_CMB12_SPEC>`"]
pub type DATA1_CMB12 = crate::Reg<data1_cmb12::DATA1_CMB12_SPEC>;
#[doc = "CAN Frame Data Word 2"]
pub mod data1_cmb12;
#[doc = "DATA0_CMB12 register accessor: an alias for `Reg<DATA0_CMB12_SPEC>`"]
pub type DATA0_CMB12 = crate::Reg<data0_cmb12::DATA0_CMB12_SPEC>;
#[doc = "CAN Frame Data Word 0"]
pub mod data0_cmb12;
#[doc = "ID0_CMB12 register accessor: an alias for `Reg<ID0_CMB12_SPEC>`"]
pub type ID0_CMB12 = crate::Reg<id0_cmb12::ID0_CMB12_SPEC>;
#[doc = "CAN Frame Identifier Word 0"]
pub mod id0_cmb12;
#[doc = "ID1_CMB12 register accessor: an alias for `Reg<ID1_CMB12_SPEC>`"]
pub type ID1_CMB12 = crate::Reg<id1_cmb12::ID1_CMB12_SPEC>;
#[doc = "CAN Frame Identifier Word 1"]
pub mod id1_cmb12;
#[doc = "CNSTAT_CMB13 register accessor: an alias for `Reg<CNSTAT_CMB13_SPEC>`"]
pub type CNSTAT_CMB13 = crate::Reg<cnstat_cmb13::CNSTAT_CMB13_SPEC>;
#[doc = "Buffer Status / Control Register"]
pub mod cnstat_cmb13;
#[doc = "TSTP_CMB13 register accessor: an alias for `Reg<TSTP_CMB13_SPEC>`"]
pub type TSTP_CMB13 = crate::Reg<tstp_cmb13::TSTP_CMB13_SPEC>;
#[doc = "CAN Frame Timestamp"]
pub mod tstp_cmb13;
#[doc = "DATA3_CMB13 register accessor: an alias for `Reg<DATA3_CMB13_SPEC>`"]
pub type DATA3_CMB13 = crate::Reg<data3_cmb13::DATA3_CMB13_SPEC>;
#[doc = "CAN Frame Data Word 3"]
pub mod data3_cmb13;
#[doc = "DATA2_CMB13 register accessor: an alias for `Reg<DATA2_CMB13_SPEC>`"]
pub type DATA2_CMB13 = crate::Reg<data2_cmb13::DATA2_CMB13_SPEC>;
#[doc = "CAN Frame Data Word 2"]
pub mod data2_cmb13;
#[doc = "DATA1_CMB13 register accessor: an alias for `Reg<DATA1_CMB13_SPEC>`"]
pub type DATA1_CMB13 = crate::Reg<data1_cmb13::DATA1_CMB13_SPEC>;
#[doc = "CAN Frame Data Word 2"]
pub mod data1_cmb13;
#[doc = "DATA0_CMB13 register accessor: an alias for `Reg<DATA0_CMB13_SPEC>`"]
pub type DATA0_CMB13 = crate::Reg<data0_cmb13::DATA0_CMB13_SPEC>;
#[doc = "CAN Frame Data Word 0"]
pub mod data0_cmb13;
#[doc = "ID0_CMB13 register accessor: an alias for `Reg<ID0_CMB13_SPEC>`"]
pub type ID0_CMB13 = crate::Reg<id0_cmb13::ID0_CMB13_SPEC>;
#[doc = "CAN Frame Identifier Word 0"]
pub mod id0_cmb13;
#[doc = "ID1_CMB13 register accessor: an alias for `Reg<ID1_CMB13_SPEC>`"]
pub type ID1_CMB13 = crate::Reg<id1_cmb13::ID1_CMB13_SPEC>;
#[doc = "CAN Frame Identifier Word 1"]
pub mod id1_cmb13;
#[doc = "CNSTAT_CMB14 register accessor: an alias for `Reg<CNSTAT_CMB14_SPEC>`"]
pub type CNSTAT_CMB14 = crate::Reg<cnstat_cmb14::CNSTAT_CMB14_SPEC>;
#[doc = "Buffer Status / Control Register"]
pub mod cnstat_cmb14;
#[doc = "TSTP_CMB14 register accessor: an alias for `Reg<TSTP_CMB14_SPEC>`"]
pub type TSTP_CMB14 = crate::Reg<tstp_cmb14::TSTP_CMB14_SPEC>;
#[doc = "CAN Frame Timestamp"]
pub mod tstp_cmb14;
#[doc = "DATA3_CMB14 register accessor: an alias for `Reg<DATA3_CMB14_SPEC>`"]
pub type DATA3_CMB14 = crate::Reg<data3_cmb14::DATA3_CMB14_SPEC>;
#[doc = "CAN Frame Data Word 3"]
pub mod data3_cmb14;
#[doc = "DATA2_CMB14 register accessor: an alias for `Reg<DATA2_CMB14_SPEC>`"]
pub type DATA2_CMB14 = crate::Reg<data2_cmb14::DATA2_CMB14_SPEC>;
#[doc = "CAN Frame Data Word 2"]
pub mod data2_cmb14;
#[doc = "DATA1_CMB14 register accessor: an alias for `Reg<DATA1_CMB14_SPEC>`"]
pub type DATA1_CMB14 = crate::Reg<data1_cmb14::DATA1_CMB14_SPEC>;
#[doc = "CAN Frame Data Word 2"]
pub mod data1_cmb14;
#[doc = "DATA0_CMB14 register accessor: an alias for `Reg<DATA0_CMB14_SPEC>`"]
pub type DATA0_CMB14 = crate::Reg<data0_cmb14::DATA0_CMB14_SPEC>;
#[doc = "CAN Frame Data Word 0"]
pub mod data0_cmb14;
#[doc = "ID0_CMB14 register accessor: an alias for `Reg<ID0_CMB14_SPEC>`"]
pub type ID0_CMB14 = crate::Reg<id0_cmb14::ID0_CMB14_SPEC>;
#[doc = "CAN Frame Identifier Word 0"]
pub mod id0_cmb14;
#[doc = "ID1_CMB14 register accessor: an alias for `Reg<ID1_CMB14_SPEC>`"]
pub type ID1_CMB14 = crate::Reg<id1_cmb14::ID1_CMB14_SPEC>;
#[doc = "CAN Frame Identifier Word 1"]
pub mod id1_cmb14;
#[doc = "CNSTAT_HCMB register accessor: an alias for `Reg<CNSTAT_HCMB_SPEC>`"]
pub type CNSTAT_HCMB = crate::Reg<cnstat_hcmb::CNSTAT_HCMB_SPEC>;
#[doc = "Buffer Status / Control Register"]
pub mod cnstat_hcmb;
#[doc = "TSTP_HCMB register accessor: an alias for `Reg<TSTP_HCMB_SPEC>`"]
pub type TSTP_HCMB = crate::Reg<tstp_hcmb::TSTP_HCMB_SPEC>;
#[doc = "CAN Frame Timestamp"]
pub mod tstp_hcmb;
#[doc = "DATA3_HCMB register accessor: an alias for `Reg<DATA3_HCMB_SPEC>`"]
pub type DATA3_HCMB = crate::Reg<data3_hcmb::DATA3_HCMB_SPEC>;
#[doc = "CAN Frame Data Word 3"]
pub mod data3_hcmb;
#[doc = "DATA2_HCMB register accessor: an alias for `Reg<DATA2_HCMB_SPEC>`"]
pub type DATA2_HCMB = crate::Reg<data2_hcmb::DATA2_HCMB_SPEC>;
#[doc = "CAN Frame Data Word 2"]
pub mod data2_hcmb;
#[doc = "DATA1_HCMB register accessor: an alias for `Reg<DATA1_HCMB_SPEC>`"]
pub type DATA1_HCMB = crate::Reg<data1_hcmb::DATA1_HCMB_SPEC>;
#[doc = "CAN Frame Data Word 2"]
pub mod data1_hcmb;
#[doc = "DATA0_HCMB register accessor: an alias for `Reg<DATA0_HCMB_SPEC>`"]
pub type DATA0_HCMB = crate::Reg<data0_hcmb::DATA0_HCMB_SPEC>;
#[doc = "CAN Frame Data Word 0"]
pub mod data0_hcmb;
#[doc = "ID0_HCMB register accessor: an alias for `Reg<ID0_HCMB_SPEC>`"]
pub type ID0_HCMB = crate::Reg<id0_hcmb::ID0_HCMB_SPEC>;
#[doc = "CAN Frame Identifier Word 0"]
pub mod id0_hcmb;
#[doc = "ID1_HCMB register accessor: an alias for `Reg<ID1_HCMB_SPEC>`"]
pub type ID1_HCMB = crate::Reg<id1_hcmb::ID1_HCMB_SPEC>;
#[doc = "CAN Frame Identifier Word 1"]
pub mod id1_hcmb;
#[doc = "CGCR register accessor: an alias for `Reg<CGCR_SPEC>`"]
pub type CGCR = crate::Reg<cgcr::CGCR_SPEC>;
#[doc = "CAN Global Configuration Register"]
pub mod cgcr;
#[doc = "CTIM register accessor: an alias for `Reg<CTIM_SPEC>`"]
pub type CTIM = crate::Reg<ctim::CTIM_SPEC>;
#[doc = "CAN Timing Register"]
pub mod ctim;
#[doc = "GMSKX register accessor: an alias for `Reg<GMSKX_SPEC>`"]
pub type GMSKX = crate::Reg<gmskx::GMSKX_SPEC>;
#[doc = "CAN Global Mask Extension"]
pub mod gmskx;
#[doc = "GMSKB register accessor: an alias for `Reg<GMSKB_SPEC>`"]
pub type GMSKB = crate::Reg<gmskb::GMSKB_SPEC>;
#[doc = "CAN Global Mask Base"]
pub mod gmskb;
#[doc = "BMSKX register accessor: an alias for `Reg<BMSKX_SPEC>`"]
pub type BMSKX = crate::Reg<bmskx::BMSKX_SPEC>;
#[doc = "CAN Basic Mask Extension"]
pub mod bmskx;
#[doc = "BMSKB register accessor: an alias for `Reg<BMSKB_SPEC>`"]
pub type BMSKB = crate::Reg<bmskb::BMSKB_SPEC>;
#[doc = "CAN Basic Mask Base"]
pub mod bmskb;
#[doc = "CIEN register accessor: an alias for `Reg<CIEN_SPEC>`"]
pub type CIEN = crate::Reg<cien::CIEN_SPEC>;
#[doc = "CAN Interrupt Enable Register"]
pub mod cien;
#[doc = "CIPND register accessor: an alias for `Reg<CIPND_SPEC>`"]
pub type CIPND = crate::Reg<cipnd::CIPND_SPEC>;
#[doc = "CAN Interrupt Pending Register"]
pub mod cipnd;
#[doc = "CICLR register accessor: an alias for `Reg<CICLR_SPEC>`"]
pub type CICLR = crate::Reg<ciclr::CICLR_SPEC>;
#[doc = "CAN Interrupt Clear Register"]
pub mod ciclr;
#[doc = "CICEN register accessor: an alias for `Reg<CICEN_SPEC>`"]
pub type CICEN = crate::Reg<cicen::CICEN_SPEC>;
#[doc = "CAN Interrupt Code Enable Register"]
pub mod cicen;
#[doc = "CSTPND register accessor: an alias for `Reg<CSTPND_SPEC>`"]
pub type CSTPND = crate::Reg<cstpnd::CSTPND_SPEC>;
#[doc = "CAN Status Pending Register"]
pub mod cstpnd;
#[doc = "CANEC register accessor: an alias for `Reg<CANEC_SPEC>`"]
pub type CANEC = crate::Reg<canec::CANEC_SPEC>;
#[doc = "CAN Error Counter Register"]
pub mod canec;
#[doc = "CEDIAG register accessor: an alias for `Reg<CEDIAG_SPEC>`"]
pub type CEDIAG = crate::Reg<cediag::CEDIAG_SPEC>;
#[doc = "CAN Error Diagnostic Register"]
pub mod cediag;
#[doc = "CTMR register accessor: an alias for `Reg<CTMR_SPEC>`"]
pub type CTMR = crate::Reg<ctmr::CTMR_SPEC>;
#[doc = "CAN Timer Register"]
pub mod ctmr;
