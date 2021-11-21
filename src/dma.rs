#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMA Status"]
    pub status: crate::Reg<status::STATUS_SPEC>,
    #[doc = "0x04 - DMA Configuration"]
    pub cfg: crate::Reg<cfg::CFG_SPEC>,
    #[doc = "0x08 - Base Pointer for DMA Control Registers"]
    pub ctrl_base_ptr: crate::Reg<ctrl_base_ptr::CTRL_BASE_PTR_SPEC>,
    #[doc = "0x0c - DMA Channel alternate control data base pointer"]
    pub alt_ctrl_base_ptr: crate::Reg<alt_ctrl_base_ptr::ALT_CTRL_BASE_PTR_SPEC>,
    #[doc = "0x10 - DMA channel wait on request status"]
    pub waitonreq_status: crate::Reg<waitonreq_status::WAITONREQ_STATUS_SPEC>,
    #[doc = "0x14 - DMA channel software request"]
    pub chnl_sw_request: crate::Reg<chnl_sw_request::CHNL_SW_REQUEST_SPEC>,
    #[doc = "0x18 - DMA channel useburst set"]
    pub chnl_useburst_set: crate::Reg<chnl_useburst_set::CHNL_USEBURST_SET_SPEC>,
    #[doc = "0x1c - DMA channel useburst clear"]
    pub chnl_useburst_clr: crate::Reg<chnl_useburst_clr::CHNL_USEBURST_CLR_SPEC>,
    #[doc = "0x20 - DMA channel request mask set"]
    pub chnl_req_mask_set: crate::Reg<chnl_req_mask_set::CHNL_REQ_MASK_SET_SPEC>,
    #[doc = "0x24 - DMA channel request mask clear"]
    pub chnl_req_mask_clr: crate::Reg<chnl_req_mask_clr::CHNL_REQ_MASK_CLR_SPEC>,
    #[doc = "0x28 - DMA channel enable set"]
    pub chnl_enable_set: crate::Reg<chnl_enable_set::CHNL_ENABLE_SET_SPEC>,
    #[doc = "0x2c - DMA channel enable clear"]
    pub chnl_enable_clr: crate::Reg<chnl_enable_clr::CHNL_ENABLE_CLR_SPEC>,
    #[doc = "0x30 - DMA channel primary alternate set"]
    pub chnl_pri_alt_set: crate::Reg<chnl_pri_alt_set::CHNL_PRI_ALT_SET_SPEC>,
    #[doc = "0x34 - DMA channel primary alternate clear"]
    pub chnl_pri_alt_clr: crate::Reg<chnl_pri_alt_clr::CHNL_PRI_ALT_CLR_SPEC>,
    #[doc = "0x38 - DMA channel priority set"]
    pub chnl_priority_set: crate::Reg<chnl_priority_set::CHNL_PRIORITY_SET_SPEC>,
    #[doc = "0x3c - DMA channel priority clear"]
    pub chnl_priority_clr: crate::Reg<chnl_priority_clr::CHNL_PRIORITY_CLR_SPEC>,
    _reserved16: [u8; 0x0c],
    #[doc = "0x4c - DMA bus error clear"]
    pub err_clr: crate::Reg<err_clr::ERR_CLR_SPEC>,
    _reserved17: [u8; 0x0db0],
    #[doc = "0xe00 - DMA integration configuration"]
    pub integration_cfg: crate::Reg<integration_cfg::INTEGRATION_CFG_SPEC>,
    _reserved18: [u8; 0x04],
    #[doc = "0xe08 - DMA stall status"]
    pub stall_status: crate::Reg<stall_status::STALL_STATUS_SPEC>,
    _reserved19: [u8; 0x04],
    #[doc = "0xe10 - DMA Configuration"]
    pub dma_req_status: crate::Reg<dma_req_status::DMA_REQ_STATUS_SPEC>,
    _reserved20: [u8; 0x04],
    #[doc = "0xe18 - DMA single request status"]
    pub dma_sreq_status: crate::Reg<dma_sreq_status::DMA_SREQ_STATUS_SPEC>,
    _reserved21: [u8; 0x04],
    #[doc = "0xe20 - DMA done set"]
    pub dma_done_set: crate::Reg<dma_done_set::DMA_DONE_SET_SPEC>,
    #[doc = "0xe24 - DMA done clear"]
    pub dma_done_clr: crate::Reg<dma_done_clr::DMA_DONE_CLR_SPEC>,
    #[doc = "0xe28 - DMA active set"]
    pub dma_active_set: crate::Reg<dma_active_set::DMA_ACTIVE_SET_SPEC>,
    #[doc = "0xe2c - DMA active clear"]
    pub dma_active_clr: crate::Reg<dma_active_clr::DMA_ACTIVE_CLR_SPEC>,
    _reserved25: [u8; 0x18],
    #[doc = "0xe48 - DMA bus error set"]
    pub err_set: crate::Reg<err_set::ERR_SET_SPEC>,
    _reserved26: [u8; 0x0184],
    #[doc = "0xfd0 - DMA Peripheral ID 4"]
    pub periph_id_4: crate::Reg<periph_id_4::PERIPH_ID_4_SPEC>,
    _reserved27: [u8; 0x0c],
    #[doc = "0xfe0 - DMA Peripheral ID 0"]
    pub periph_id_0: crate::Reg<periph_id_0::PERIPH_ID_0_SPEC>,
    #[doc = "0xfe4 - DMA Peripheral ID 1"]
    pub periph_id_1: crate::Reg<periph_id_1::PERIPH_ID_1_SPEC>,
    #[doc = "0xfe8 - DMA Peripheral ID 2"]
    pub periph_id_2: crate::Reg<periph_id_2::PERIPH_ID_2_SPEC>,
    #[doc = "0xfec - DMA Peripheral ID 3"]
    pub periph_id_3: crate::Reg<periph_id_3::PERIPH_ID_3_SPEC>,
    #[doc = "0xff0 - DMA PrimeCell ID 0"]
    pub primecell_id_0: crate::Reg<primecell_id_0::PRIMECELL_ID_0_SPEC>,
    #[doc = "0xff4 - DMA PrimeCell ID 1"]
    pub primecell_id_1: crate::Reg<primecell_id_1::PRIMECELL_ID_1_SPEC>,
    #[doc = "0xff8 - DMA PrimeCell ID 2"]
    pub primecell_id_2: crate::Reg<primecell_id_2::PRIMECELL_ID_2_SPEC>,
    #[doc = "0xffc - DMA PrimeCell ID 3"]
    pub primecell_id_3: crate::Reg<primecell_id_3::PRIMECELL_ID_3_SPEC>,
}
#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "DMA Status"]
pub mod status;
#[doc = "CFG register accessor: an alias for `Reg<CFG_SPEC>`"]
pub type CFG = crate::Reg<cfg::CFG_SPEC>;
#[doc = "DMA Configuration"]
pub mod cfg;
#[doc = "CTRL_BASE_PTR register accessor: an alias for `Reg<CTRL_BASE_PTR_SPEC>`"]
pub type CTRL_BASE_PTR = crate::Reg<ctrl_base_ptr::CTRL_BASE_PTR_SPEC>;
#[doc = "Base Pointer for DMA Control Registers"]
pub mod ctrl_base_ptr;
#[doc = "ALT_CTRL_BASE_PTR register accessor: an alias for `Reg<ALT_CTRL_BASE_PTR_SPEC>`"]
pub type ALT_CTRL_BASE_PTR = crate::Reg<alt_ctrl_base_ptr::ALT_CTRL_BASE_PTR_SPEC>;
#[doc = "DMA Channel alternate control data base pointer"]
pub mod alt_ctrl_base_ptr;
#[doc = "WAITONREQ_STATUS register accessor: an alias for `Reg<WAITONREQ_STATUS_SPEC>`"]
pub type WAITONREQ_STATUS = crate::Reg<waitonreq_status::WAITONREQ_STATUS_SPEC>;
#[doc = "DMA channel wait on request status"]
pub mod waitonreq_status;
#[doc = "CHNL_SW_REQUEST register accessor: an alias for `Reg<CHNL_SW_REQUEST_SPEC>`"]
pub type CHNL_SW_REQUEST = crate::Reg<chnl_sw_request::CHNL_SW_REQUEST_SPEC>;
#[doc = "DMA channel software request"]
pub mod chnl_sw_request;
#[doc = "CHNL_USEBURST_SET register accessor: an alias for `Reg<CHNL_USEBURST_SET_SPEC>`"]
pub type CHNL_USEBURST_SET = crate::Reg<chnl_useburst_set::CHNL_USEBURST_SET_SPEC>;
#[doc = "DMA channel useburst set"]
pub mod chnl_useburst_set;
#[doc = "CHNL_USEBURST_CLR register accessor: an alias for `Reg<CHNL_USEBURST_CLR_SPEC>`"]
pub type CHNL_USEBURST_CLR = crate::Reg<chnl_useburst_clr::CHNL_USEBURST_CLR_SPEC>;
#[doc = "DMA channel useburst clear"]
pub mod chnl_useburst_clr;
#[doc = "CHNL_REQ_MASK_SET register accessor: an alias for `Reg<CHNL_REQ_MASK_SET_SPEC>`"]
pub type CHNL_REQ_MASK_SET = crate::Reg<chnl_req_mask_set::CHNL_REQ_MASK_SET_SPEC>;
#[doc = "DMA channel request mask set"]
pub mod chnl_req_mask_set;
#[doc = "CHNL_REQ_MASK_CLR register accessor: an alias for `Reg<CHNL_REQ_MASK_CLR_SPEC>`"]
pub type CHNL_REQ_MASK_CLR = crate::Reg<chnl_req_mask_clr::CHNL_REQ_MASK_CLR_SPEC>;
#[doc = "DMA channel request mask clear"]
pub mod chnl_req_mask_clr;
#[doc = "CHNL_ENABLE_SET register accessor: an alias for `Reg<CHNL_ENABLE_SET_SPEC>`"]
pub type CHNL_ENABLE_SET = crate::Reg<chnl_enable_set::CHNL_ENABLE_SET_SPEC>;
#[doc = "DMA channel enable set"]
pub mod chnl_enable_set;
#[doc = "CHNL_ENABLE_CLR register accessor: an alias for `Reg<CHNL_ENABLE_CLR_SPEC>`"]
pub type CHNL_ENABLE_CLR = crate::Reg<chnl_enable_clr::CHNL_ENABLE_CLR_SPEC>;
#[doc = "DMA channel enable clear"]
pub mod chnl_enable_clr;
#[doc = "CHNL_PRI_ALT_SET register accessor: an alias for `Reg<CHNL_PRI_ALT_SET_SPEC>`"]
pub type CHNL_PRI_ALT_SET = crate::Reg<chnl_pri_alt_set::CHNL_PRI_ALT_SET_SPEC>;
#[doc = "DMA channel primary alternate set"]
pub mod chnl_pri_alt_set;
#[doc = "CHNL_PRI_ALT_CLR register accessor: an alias for `Reg<CHNL_PRI_ALT_CLR_SPEC>`"]
pub type CHNL_PRI_ALT_CLR = crate::Reg<chnl_pri_alt_clr::CHNL_PRI_ALT_CLR_SPEC>;
#[doc = "DMA channel primary alternate clear"]
pub mod chnl_pri_alt_clr;
#[doc = "CHNL_PRIORITY_SET register accessor: an alias for `Reg<CHNL_PRIORITY_SET_SPEC>`"]
pub type CHNL_PRIORITY_SET = crate::Reg<chnl_priority_set::CHNL_PRIORITY_SET_SPEC>;
#[doc = "DMA channel priority set"]
pub mod chnl_priority_set;
#[doc = "CHNL_PRIORITY_CLR register accessor: an alias for `Reg<CHNL_PRIORITY_CLR_SPEC>`"]
pub type CHNL_PRIORITY_CLR = crate::Reg<chnl_priority_clr::CHNL_PRIORITY_CLR_SPEC>;
#[doc = "DMA channel priority clear"]
pub mod chnl_priority_clr;
#[doc = "ERR_CLR register accessor: an alias for `Reg<ERR_CLR_SPEC>`"]
pub type ERR_CLR = crate::Reg<err_clr::ERR_CLR_SPEC>;
#[doc = "DMA bus error clear"]
pub mod err_clr;
#[doc = "INTEGRATION_CFG register accessor: an alias for `Reg<INTEGRATION_CFG_SPEC>`"]
pub type INTEGRATION_CFG = crate::Reg<integration_cfg::INTEGRATION_CFG_SPEC>;
#[doc = "DMA integration configuration"]
pub mod integration_cfg;
#[doc = "STALL_STATUS register accessor: an alias for `Reg<STALL_STATUS_SPEC>`"]
pub type STALL_STATUS = crate::Reg<stall_status::STALL_STATUS_SPEC>;
#[doc = "DMA stall status"]
pub mod stall_status;
#[doc = "DMA_REQ_STATUS register accessor: an alias for `Reg<DMA_REQ_STATUS_SPEC>`"]
pub type DMA_REQ_STATUS = crate::Reg<dma_req_status::DMA_REQ_STATUS_SPEC>;
#[doc = "DMA Configuration"]
pub mod dma_req_status;
#[doc = "DMA_SREQ_STATUS register accessor: an alias for `Reg<DMA_SREQ_STATUS_SPEC>`"]
pub type DMA_SREQ_STATUS = crate::Reg<dma_sreq_status::DMA_SREQ_STATUS_SPEC>;
#[doc = "DMA single request status"]
pub mod dma_sreq_status;
#[doc = "DMA_DONE_SET register accessor: an alias for `Reg<DMA_DONE_SET_SPEC>`"]
pub type DMA_DONE_SET = crate::Reg<dma_done_set::DMA_DONE_SET_SPEC>;
#[doc = "DMA done set"]
pub mod dma_done_set;
#[doc = "DMA_DONE_CLR register accessor: an alias for `Reg<DMA_DONE_CLR_SPEC>`"]
pub type DMA_DONE_CLR = crate::Reg<dma_done_clr::DMA_DONE_CLR_SPEC>;
#[doc = "DMA done clear"]
pub mod dma_done_clr;
#[doc = "DMA_ACTIVE_SET register accessor: an alias for `Reg<DMA_ACTIVE_SET_SPEC>`"]
pub type DMA_ACTIVE_SET = crate::Reg<dma_active_set::DMA_ACTIVE_SET_SPEC>;
#[doc = "DMA active set"]
pub mod dma_active_set;
#[doc = "DMA_ACTIVE_CLR register accessor: an alias for `Reg<DMA_ACTIVE_CLR_SPEC>`"]
pub type DMA_ACTIVE_CLR = crate::Reg<dma_active_clr::DMA_ACTIVE_CLR_SPEC>;
#[doc = "DMA active clear"]
pub mod dma_active_clr;
#[doc = "ERR_SET register accessor: an alias for `Reg<ERR_SET_SPEC>`"]
pub type ERR_SET = crate::Reg<err_set::ERR_SET_SPEC>;
#[doc = "DMA bus error set"]
pub mod err_set;
#[doc = "PERIPH_ID_4 register accessor: an alias for `Reg<PERIPH_ID_4_SPEC>`"]
pub type PERIPH_ID_4 = crate::Reg<periph_id_4::PERIPH_ID_4_SPEC>;
#[doc = "DMA Peripheral ID 4"]
pub mod periph_id_4;
#[doc = "PERIPH_ID_0 register accessor: an alias for `Reg<PERIPH_ID_0_SPEC>`"]
pub type PERIPH_ID_0 = crate::Reg<periph_id_0::PERIPH_ID_0_SPEC>;
#[doc = "DMA Peripheral ID 0"]
pub mod periph_id_0;
#[doc = "PERIPH_ID_1 register accessor: an alias for `Reg<PERIPH_ID_1_SPEC>`"]
pub type PERIPH_ID_1 = crate::Reg<periph_id_1::PERIPH_ID_1_SPEC>;
#[doc = "DMA Peripheral ID 1"]
pub mod periph_id_1;
#[doc = "PERIPH_ID_2 register accessor: an alias for `Reg<PERIPH_ID_2_SPEC>`"]
pub type PERIPH_ID_2 = crate::Reg<periph_id_2::PERIPH_ID_2_SPEC>;
#[doc = "DMA Peripheral ID 2"]
pub mod periph_id_2;
#[doc = "PERIPH_ID_3 register accessor: an alias for `Reg<PERIPH_ID_3_SPEC>`"]
pub type PERIPH_ID_3 = crate::Reg<periph_id_3::PERIPH_ID_3_SPEC>;
#[doc = "DMA Peripheral ID 3"]
pub mod periph_id_3;
#[doc = "PRIMECELL_ID_0 register accessor: an alias for `Reg<PRIMECELL_ID_0_SPEC>`"]
pub type PRIMECELL_ID_0 = crate::Reg<primecell_id_0::PRIMECELL_ID_0_SPEC>;
#[doc = "DMA PrimeCell ID 0"]
pub mod primecell_id_0;
#[doc = "PRIMECELL_ID_1 register accessor: an alias for `Reg<PRIMECELL_ID_1_SPEC>`"]
pub type PRIMECELL_ID_1 = crate::Reg<primecell_id_1::PRIMECELL_ID_1_SPEC>;
#[doc = "DMA PrimeCell ID 1"]
pub mod primecell_id_1;
#[doc = "PRIMECELL_ID_2 register accessor: an alias for `Reg<PRIMECELL_ID_2_SPEC>`"]
pub type PRIMECELL_ID_2 = crate::Reg<primecell_id_2::PRIMECELL_ID_2_SPEC>;
#[doc = "DMA PrimeCell ID 2"]
pub mod primecell_id_2;
#[doc = "PRIMECELL_ID_3 register accessor: an alias for `Reg<PRIMECELL_ID_3_SPEC>`"]
pub type PRIMECELL_ID_3 = crate::Reg<primecell_id_3::PRIMECELL_ID_3_SPEC>;
#[doc = "DMA PrimeCell ID 3"]
pub mod primecell_id_3;
