#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Data Register"]
    pub synd_data: crate::Reg<synd_data::SYND_DATA_SPEC>,
    #[doc = "0x04 - Syndrome Data Register"]
    pub synd_synd: crate::Reg<synd_synd::SYND_SYND_SPEC>,
    #[doc = "0x08 - EDAC Encode"]
    pub synd_enc_32_44: crate::Reg<synd_enc_32_44::SYND_ENC_32_44_SPEC>,
    #[doc = "0x0c - EDAC Decode Data"]
    pub synd_check_32_44_data: crate::Reg<synd_check_32_44_data::SYND_CHECK_32_44_DATA_SPEC>,
    #[doc = "0x10 - EDAC Decode Syndrome"]
    pub synd_check_32_44_synd: crate::Reg<synd_check_32_44_synd::SYND_CHECK_32_44_SYND_SPEC>,
    #[doc = "0x14 - ROM EDAC Trap Address"]
    pub rom_trap_address: crate::Reg<rom_trap_address::ROM_TRAP_ADDRESS_SPEC>,
    #[doc = "0x18 - ROM EDAC Trap Syndrome"]
    pub rom_trap_synd: crate::Reg<rom_trap_synd::ROM_TRAP_SYND_SPEC>,
    #[doc = "0x1c - RAM0 EDAC Trap Address"]
    pub ram_trap_addr0: crate::Reg<ram_trap_addr0::RAM_TRAP_ADDR0_SPEC>,
    #[doc = "0x20 - RAM0 EDAC Trap Syndrome"]
    pub ram_trap_synd0: crate::Reg<ram_trap_synd0::RAM_TRAP_SYND0_SPEC>,
    #[doc = "0x24 - RAM1 EDAC Trap Address"]
    pub ram_trap_addr1: crate::Reg<ram_trap_addr1::RAM_TRAP_ADDR1_SPEC>,
    #[doc = "0x28 - RAM1 EDAC Trap Syndrome"]
    pub ram_trap_synd1: crate::Reg<ram_trap_synd1::RAM_TRAP_SYND1_SPEC>,
    _reserved11: [u8; 0xf4],
    #[doc = "0x120 - EDAC Encode"]
    pub synd_enc_32_52: crate::Reg<synd_enc_32_52::SYND_ENC_32_52_SPEC>,
    #[doc = "0x124 - EDAC Decode Data"]
    pub synd_check_32_52_data: crate::Reg<synd_check_32_52_data::SYND_CHECK_32_52_DATA_SPEC>,
    #[doc = "0x128 - EDAC Decode Syndrome"]
    pub synd_check_32_52_synd: crate::Reg<synd_check_32_52_synd::SYND_CHECK_32_52_SYND_SPEC>,
    _reserved14: [u8; 0x0ed0],
    #[doc = "0xffc - Peripheral ID Register"]
    pub perid: crate::Reg<perid::PERID_SPEC>,
}
#[doc = "SYND_DATA register accessor: an alias for `Reg<SYND_DATA_SPEC>`"]
pub type SYND_DATA = crate::Reg<synd_data::SYND_DATA_SPEC>;
#[doc = "Data Register"]
pub mod synd_data;
#[doc = "SYND_SYND register accessor: an alias for `Reg<SYND_SYND_SPEC>`"]
pub type SYND_SYND = crate::Reg<synd_synd::SYND_SYND_SPEC>;
#[doc = "Syndrome Data Register"]
pub mod synd_synd;
#[doc = "SYND_ENC_32_44 register accessor: an alias for `Reg<SYND_ENC_32_44_SPEC>`"]
pub type SYND_ENC_32_44 = crate::Reg<synd_enc_32_44::SYND_ENC_32_44_SPEC>;
#[doc = "EDAC Encode"]
pub mod synd_enc_32_44;
#[doc = "SYND_CHECK_32_44_DATA register accessor: an alias for `Reg<SYND_CHECK_32_44_DATA_SPEC>`"]
pub type SYND_CHECK_32_44_DATA = crate::Reg<synd_check_32_44_data::SYND_CHECK_32_44_DATA_SPEC>;
#[doc = "EDAC Decode Data"]
pub mod synd_check_32_44_data;
#[doc = "SYND_CHECK_32_44_SYND register accessor: an alias for `Reg<SYND_CHECK_32_44_SYND_SPEC>`"]
pub type SYND_CHECK_32_44_SYND = crate::Reg<synd_check_32_44_synd::SYND_CHECK_32_44_SYND_SPEC>;
#[doc = "EDAC Decode Syndrome"]
pub mod synd_check_32_44_synd;
#[doc = "ROM_TRAP_ADDRESS register accessor: an alias for `Reg<ROM_TRAP_ADDRESS_SPEC>`"]
pub type ROM_TRAP_ADDRESS = crate::Reg<rom_trap_address::ROM_TRAP_ADDRESS_SPEC>;
#[doc = "ROM EDAC Trap Address"]
pub mod rom_trap_address;
#[doc = "ROM_TRAP_SYND register accessor: an alias for `Reg<ROM_TRAP_SYND_SPEC>`"]
pub type ROM_TRAP_SYND = crate::Reg<rom_trap_synd::ROM_TRAP_SYND_SPEC>;
#[doc = "ROM EDAC Trap Syndrome"]
pub mod rom_trap_synd;
#[doc = "RAM_TRAP_ADDR0 register accessor: an alias for `Reg<RAM_TRAP_ADDR0_SPEC>`"]
pub type RAM_TRAP_ADDR0 = crate::Reg<ram_trap_addr0::RAM_TRAP_ADDR0_SPEC>;
#[doc = "RAM0 EDAC Trap Address"]
pub mod ram_trap_addr0;
#[doc = "RAM_TRAP_SYND0 register accessor: an alias for `Reg<RAM_TRAP_SYND0_SPEC>`"]
pub type RAM_TRAP_SYND0 = crate::Reg<ram_trap_synd0::RAM_TRAP_SYND0_SPEC>;
#[doc = "RAM0 EDAC Trap Syndrome"]
pub mod ram_trap_synd0;
#[doc = "RAM_TRAP_ADDR1 register accessor: an alias for `Reg<RAM_TRAP_ADDR1_SPEC>`"]
pub type RAM_TRAP_ADDR1 = crate::Reg<ram_trap_addr1::RAM_TRAP_ADDR1_SPEC>;
#[doc = "RAM1 EDAC Trap Address"]
pub mod ram_trap_addr1;
#[doc = "RAM_TRAP_SYND1 register accessor: an alias for `Reg<RAM_TRAP_SYND1_SPEC>`"]
pub type RAM_TRAP_SYND1 = crate::Reg<ram_trap_synd1::RAM_TRAP_SYND1_SPEC>;
#[doc = "RAM1 EDAC Trap Syndrome"]
pub mod ram_trap_synd1;
#[doc = "SYND_ENC_32_52 register accessor: an alias for `Reg<SYND_ENC_32_52_SPEC>`"]
pub type SYND_ENC_32_52 = crate::Reg<synd_enc_32_52::SYND_ENC_32_52_SPEC>;
#[doc = "EDAC Encode"]
pub mod synd_enc_32_52;
#[doc = "SYND_CHECK_32_52_DATA register accessor: an alias for `Reg<SYND_CHECK_32_52_DATA_SPEC>`"]
pub type SYND_CHECK_32_52_DATA = crate::Reg<synd_check_32_52_data::SYND_CHECK_32_52_DATA_SPEC>;
#[doc = "EDAC Decode Data"]
pub mod synd_check_32_52_data;
#[doc = "SYND_CHECK_32_52_SYND register accessor: an alias for `Reg<SYND_CHECK_32_52_SYND_SPEC>`"]
pub type SYND_CHECK_32_52_SYND = crate::Reg<synd_check_32_52_synd::SYND_CHECK_32_52_SYND_SPEC>;
#[doc = "EDAC Decode Syndrome"]
pub mod synd_check_32_52_synd;
#[doc = "PERID register accessor: an alias for `Reg<PERID_SPEC>`"]
pub type PERID = crate::Reg<perid::PERID_SPEC>;
#[doc = "Peripheral ID Register"]
pub mod perid;
