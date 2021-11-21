#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Operation mode register for the MAC"]
    pub mac_config: crate::Reg<mac_config::MAC_CONFIG_SPEC>,
    #[doc = "0x04 - Contains the frame filtering controls"]
    pub mac_frame_fltr: crate::Reg<mac_frame_fltr::MAC_FRAME_FLTR_SPEC>,
    _reserved2: [u8; 0x08],
    #[doc = "0x10 - Controls the management cycles to an external PHY"]
    pub mac_gmii_addr: crate::Reg<mac_gmii_addr::MAC_GMII_ADDR_SPEC>,
    #[doc = "0x14 - Contains the data to be written to or read from the PHY register"]
    pub mac_gmii_data: crate::Reg<mac_gmii_data::MAC_GMII_DATA_SPEC>,
    #[doc = "0x18 - Controls the generation of control frames"]
    pub mac_flow_ctrl: crate::Reg<mac_flow_ctrl::MAC_FLOW_CTRL_SPEC>,
    #[doc = "0x1c - Identifies IEEE 802.1Q VLAN type frames"]
    pub mac_vlan_tag: crate::Reg<mac_vlan_tag::MAC_VLAN_TAG_SPEC>,
    _reserved6: [u8; 0x04],
    #[doc = "0x24 - Gives the status of the various internal blocks for debugging"]
    pub mac_debug: crate::Reg<mac_debug::MAC_DEBUG_SPEC>,
    _reserved7: [u8; 0x10],
    #[doc = "0x38 - Contains the interrupt status"]
    pub mac_intr_stat: crate::Reg<mac_intr_stat::MAC_INTR_STAT_SPEC>,
    #[doc = "0x3c - Contains the masks for generating interrupt"]
    pub mac_intr_mask: crate::Reg<mac_intr_mask::MAC_INTR_MASK_SPEC>,
    #[doc = "0x40 - Contains the high 16-bits of the first MAC Address"]
    pub mac_addr_h: crate::Reg<mac_addr_h::MAC_ADDR_H_SPEC>,
    #[doc = "0x44 - Contains the Low 32-bits of the first MAC Address"]
    pub mac_addr_l: crate::Reg<mac_addr_l::MAC_ADDR_L_SPEC>,
    _reserved11: [u8; 0x94],
    #[doc = "0xdc - Controls the watchdog time-out for received frames"]
    pub mac_wdog_to: crate::Reg<mac_wdog_to::MAC_WDOG_TO_SPEC>,
    _reserved12: [u8; 0x20],
    #[doc = "0x100 - MMC Control Register"]
    pub mmc_cntrl: crate::Reg<mmc_cntrl::MMC_CNTRL_SPEC>,
    #[doc = "0x104 - MMC Receive Interrupt Register"]
    pub mmc_intr_rx: crate::Reg<mmc_intr_rx::MMC_INTR_RX_SPEC>,
    #[doc = "0x108 - MMC Transmit Interrupt Register"]
    pub mmc_intr_tx: crate::Reg<mmc_intr_tx::MMC_INTR_TX_SPEC>,
    #[doc = "0x10c - MMC Receive Interrupt Mask Register"]
    pub mmc_intr_mask_rx: crate::Reg<mmc_intr_mask_rx::MMC_INTR_MASK_RX_SPEC>,
    #[doc = "0x110 - MMC Transmit Interrupt Mask Register"]
    pub mmc_intr_mask_tx: crate::Reg<mmc_intr_mask_tx::MMC_INTR_MASK_TX_SPEC>,
    #[doc = "0x114 - MMC Transmit Count"]
    pub txoctetcount_gb: crate::Reg<txoctetcount_gb::TXOCTETCOUNT_GB_SPEC>,
    #[doc = "0x118 - MMC Frame Count Register"]
    pub txframecount_gb: crate::Reg<txframecount_gb::TXFRAMECOUNT_GB_SPEC>,
    #[doc = "0x11c - MMC Good Broadcast Frames Register"]
    pub txbcastframes_g: crate::Reg<txbcastframes_g::TXBCASTFRAMES_G_SPEC>,
    #[doc = "0x120 - MMC Good Multicast Frames Register"]
    pub txmcastframes_g: crate::Reg<txmcastframes_g::TXMCASTFRAMES_G_SPEC>,
    #[doc = "0x124 - MMC Good and bad Frames transmitted with length 64"]
    pub tx64oct_gb: crate::Reg<tx64oct_gb::TX64OCT_GB_SPEC>,
    #[doc = "0x128 - MMC Good and bad Frames transmitted with length 65 to 127"]
    pub tx65to127oct_gb: crate::Reg<tx65to127oct_gb::TX65TO127OCT_GB_SPEC>,
    #[doc = "0x12c - MMC Good and bad Frames transmitted with length 128 to 255"]
    pub tx128to255oct_gb: crate::Reg<tx128to255oct_gb::TX128TO255OCT_GB_SPEC>,
    #[doc = "0x130 - MMC Good and bad Frames transmitted with length 256 to 511"]
    pub tx256to511oct_gb: crate::Reg<tx256to511oct_gb::TX256TO511OCT_GB_SPEC>,
    #[doc = "0x134 - MMC Good and bad Frames transmitted with length 512 to 1023"]
    pub tx512to1023oct_gb: crate::Reg<tx512to1023oct_gb::TX512TO1023OCT_GB_SPEC>,
    #[doc = "0x138 - MMC Good and bad Frames transmitted with length 1024 to max bytes"]
    pub tx1024maxoct_gb: crate::Reg<tx1024maxoct_gb::TX1024MAXOCT_GB_SPEC>,
    #[doc = "0x13c - MMC number of good and bad unicast frames transmitted"]
    pub txucastframe_gb: crate::Reg<txucastframe_gb::TXUCASTFRAME_GB_SPEC>,
    #[doc = "0x140 - MMC number of good and bad MULTIcast frames transmitted"]
    pub txmcastframe_gb: crate::Reg<txmcastframe_gb::TXMCASTFRAME_GB_SPEC>,
    #[doc = "0x144 - MMC number of good and bad broadcast frames transmitted"]
    pub txbcastframe_gb: crate::Reg<txbcastframe_gb::TXBCASTFRAME_GB_SPEC>,
    #[doc = "0x148 - MMC number of frames aborted because of frame underflow error"]
    pub txundererr: crate::Reg<txundererr::TXUNDERERR_SPEC>,
    #[doc = "0x14c - MMC Number of successfully transmitted frames after a single collision"]
    pub txsinglecol_g: crate::Reg<txsinglecol_g::TXSINGLECOL_G_SPEC>,
    #[doc = "0x150 - MMC Number of successfully transmitted frames after multiple collisions"]
    pub txmulticol_g: crate::Reg<txmulticol_g::TXMULTICOL_G_SPEC>,
    #[doc = "0x154 - MMC Number of successfully transmitted frames after a deferral"]
    pub txdeferred: crate::Reg<txdeferred::TXDEFERRED_SPEC>,
    #[doc = "0x158 - MMC Number of aborted frames because of late collision error"]
    pub txlatecol: crate::Reg<txlatecol::TXLATECOL_SPEC>,
    #[doc = "0x15c - MMC Number of aborted frames because of excessive collision errors"]
    pub txexesscol: crate::Reg<txexesscol::TXEXESSCOL_SPEC>,
    #[doc = "0x160 - MMC Number of aborted frames because of carrier sense error"]
    pub txcarriererror: crate::Reg<txcarriererror::TXCARRIERERROR_SPEC>,
    #[doc = "0x164 - MMC Number of bytes transmitted frames only in good frames"]
    pub txoctetcount_g: crate::Reg<txoctetcount_g::TXOCTETCOUNT_G_SPEC>,
    #[doc = "0x168 - MMC Number of good frames transmitted"]
    pub txframecount_g: crate::Reg<txframecount_g::TXFRAMECOUNT_G_SPEC>,
    #[doc = "0x16c - MMC Number of frames aborted because of excessive deferral error"]
    pub txexcessdef: crate::Reg<txexcessdef::TXEXCESSDEF_SPEC>,
    #[doc = "0x170 - MMC Number of good pause frames transmitted"]
    pub txpauseframes: crate::Reg<txpauseframes::TXPAUSEFRAMES_SPEC>,
    #[doc = "0x174 - MMC Number of good VLAN frames transmitted"]
    pub txlanframes_g: crate::Reg<txlanframes_g::TXLANFRAMES_G_SPEC>,
    #[doc = "0x178 - MMC Number of frames transmitted without errors"]
    pub txoversize_g: crate::Reg<txoversize_g::TXOVERSIZE_G_SPEC>,
    _reserved43: [u8; 0x04],
    #[doc = "0x180 - MMC Number of good and bad frames received"]
    pub rxframecount_gb: crate::Reg<rxframecount_gb::RXFRAMECOUNT_GB_SPEC>,
    #[doc = "0x184 - MMC Number of bytes received in good and bad frames"]
    pub rxoctetcount_gb: crate::Reg<rxoctetcount_gb::RXOCTETCOUNT_GB_SPEC>,
    #[doc = "0x188 - MMC Number of bytes received in good frames only"]
    pub rxoctetcount_g: crate::Reg<rxoctetcount_g::RXOCTETCOUNT_G_SPEC>,
    #[doc = "0x18c - MMC Number of good broadcast frames received"]
    pub rxbcastframes_g: crate::Reg<rxbcastframes_g::RXBCASTFRAMES_G_SPEC>,
    #[doc = "0x190 - MMC Number of good multicast frames received"]
    pub rxmcastframes_g: crate::Reg<rxmcastframes_g::RXMCASTFRAMES_G_SPEC>,
    #[doc = "0x194 - MMC Number of frames received with CRC error"]
    pub rxcrcerror: crate::Reg<rxcrcerror::RXCRCERROR_SPEC>,
    #[doc = "0x198 - MMC Number of frames received with alignment error"]
    pub rxalignerror: crate::Reg<rxalignerror::RXALIGNERROR_SPEC>,
    #[doc = "0x19c - MMC Number of frames received with runt error"]
    pub rxrunterror: crate::Reg<rxrunterror::RXRUNTERROR_SPEC>,
    #[doc = "0x1a0 - MMC Number of giant frames received with length greater than 1518 bytes and with CRC error"]
    pub rxjabbererror: crate::Reg<rxjabbererror::RXJABBERERROR_SPEC>,
    #[doc = "0x1a4 - MMC Number of frames received with length less than 64 bytes"]
    pub rxundersize_g: crate::Reg<rxundersize_g::RXUNDERSIZE_G_SPEC>,
    #[doc = "0x1a8 - MMC Number of frames received without errors with length greater than the max size"]
    pub rxoversize_g: crate::Reg<rxoversize_g::RXOVERSIZE_G_SPEC>,
    #[doc = "0x1ac - MMC Number of good and bad frames received with length 64 bytes"]
    pub rx64octets_gb: crate::Reg<rx64octets_gb::RX64OCTETS_GB_SPEC>,
    #[doc = "0x1b0 - MMC Number of good and bad frames received with length between 65 and 127 bytes"]
    pub rx65to127oct_gb: crate::Reg<rx65to127oct_gb::RX65TO127OCT_GB_SPEC>,
    #[doc = "0x1b4 - MMC Number of good and bad frames received with length between 128 and 255 bytes"]
    pub rx128to255oct_gb: crate::Reg<rx128to255oct_gb::RX128TO255OCT_GB_SPEC>,
    #[doc = "0x1b8 - MMC Number of good and bad frames received with length between 256 and 511 bytes"]
    pub rx256to511oct_gb: crate::Reg<rx256to511oct_gb::RX256TO511OCT_GB_SPEC>,
    #[doc = "0x1bc - MMC Number of good and bad frames received with length between 512 and 1023 bytes"]
    pub rx512to1023oct_gb: crate::Reg<rx512to1023oct_gb::RX512TO1023OCT_GB_SPEC>,
    #[doc = "0x1c0 - MMC Number of good and bad frames received with length between 1024 and max size bytes"]
    pub rx1024maxoct_gb: crate::Reg<rx1024maxoct_gb::RX1024MAXOCT_GB_SPEC>,
    #[doc = "0x1c4 - MMC Number of received good unicast frames"]
    pub rxucastframes_g: crate::Reg<rxucastframes_g::RXUCASTFRAMES_G_SPEC>,
    #[doc = "0x1c8 - MMC Number of frames received with length error"]
    pub rxlengtherror: crate::Reg<rxlengtherror::RXLENGTHERROR_SPEC>,
    #[doc = "0x1cc - MMC Number of frames received with length field not equal to the valid frame size"]
    pub rxoutrangetype: crate::Reg<rxoutrangetype::RXOUTRANGETYPE_SPEC>,
    #[doc = "0x1d0 - MMC Number of good and valid Pause frames received"]
    pub rxpauseframes: crate::Reg<rxpauseframes::RXPAUSEFRAMES_SPEC>,
    #[doc = "0x1d4 - MMC Number of missed received frames because of FIFO overflow"]
    pub rxfifooverflow: crate::Reg<rxfifooverflow::RXFIFOOVERFLOW_SPEC>,
    #[doc = "0x1d8 - MMC Number of good and bad VLAN frames received"]
    pub rxvlanframes_gb: crate::Reg<rxvlanframes_gb::RXVLANFRAMES_GB_SPEC>,
    #[doc = "0x1dc - MMC Number of frames received with error because of watchdog timeout error"]
    pub rxwdogerror: crate::Reg<rxwdogerror::RXWDOGERROR_SPEC>,
    #[doc = "0x1e0 - MMC Number of frames received with Receive error or Frame Extension error"]
    pub rxrcverror: crate::Reg<rxrcverror::RXRCVERROR_SPEC>,
    #[doc = "0x1e4 - MMC Number of received good control frames"]
    pub rxctrlframes_g: crate::Reg<rxctrlframes_g::RXCTRLFRAMES_G_SPEC>,
    _reserved69: [u8; 0x039c],
    #[doc = "0x584 - Holds the VLAN Tag for insertion into or replacement in the transmit frames"]
    pub vlan_increplace: crate::Reg<vlan_increplace::VLAN_INCREPLACE_SPEC>,
    #[doc = "0x588 - Holds the VLAN Hash Table"]
    pub vlan_hashtable: crate::Reg<vlan_hashtable::VLAN_HASHTABLE_SPEC>,
    _reserved71: [u8; 0x0174],
    #[doc = "0x700 - Controls the IEEE 1588 timestamp generation and update logic"]
    pub timestamp_ctrl: crate::Reg<timestamp_ctrl::TIMESTAMP_CTRL_SPEC>,
    #[doc = "0x704 - Holds the 8-bit value by which the Sub-Second register is incremented"]
    pub subsec_inc: crate::Reg<subsec_inc::SUBSEC_INC_SPEC>,
    #[doc = "0x708 - Holds the lower 32 bits of the second field of the system time"]
    pub systime_seconds: crate::Reg<systime_seconds::SYSTIME_SECONDS_SPEC>,
    #[doc = "0x70c - Holds 32 bits of the nano-second field of the system time"]
    pub systime_nanosec: crate::Reg<systime_nanosec::SYSTIME_NANOSEC_SPEC>,
    #[doc = "0x710 - Holds the lower 32 bits of the second field to be written to, added to, or subtracted from the system time value"]
    pub systime_secsupdat: crate::Reg<systime_secsupdat::SYSTIME_SECSUPDAT_SPEC>,
    #[doc = "0x714 - Holds 32 bits of the nano-second field to be written to, added to, or subtracted from the system time value"]
    pub systime_nsecup: crate::Reg<systime_nsecup::SYSTIME_NSECUP_SPEC>,
    #[doc = "0x718 - This register is used by software to re-adjust the clock frequency linearly to match the Master clock frequency"]
    pub timestampaddend: crate::Reg<timestampaddend::TIMESTAMPADDEND_SPEC>,
    #[doc = "0x71c - Holds the high 32-bits of time to be compared with the system time"]
    pub target_time_secs: crate::Reg<target_time_secs::TARGET_TIME_SECS_SPEC>,
    #[doc = "0x720 - Holds the lower 32-bits of time to be compared with the system time"]
    pub target_time_nsec: crate::Reg<target_time_nsec::TARGET_TIME_NSEC_SPEC>,
    _reserved80: [u8; 0x08dc],
    #[doc = "0x1000 - Controls the DMA Host Interface Mode"]
    pub dma_bus_mode: crate::Reg<dma_bus_mode::DMA_BUS_MODE_SPEC>,
    #[doc = "0x1004 - Used by the host to instruct the DMA to poll the transmit Descriptor list"]
    pub dma_tx_poll_demand: crate::Reg<dma_tx_poll_demand::DMA_TX_POLL_DEMAND_SPEC>,
    #[doc = "0x1008 - Used by the host to instruct the DMA to poll the Receive Descriptor list"]
    pub dma_rx_poll_demand: crate::Reg<dma_rx_poll_demand::DMA_RX_POLL_DEMAND_SPEC>,
    #[doc = "0x100c - Points the DMA to the start of the Receive Descriptor list"]
    pub dma_rx_desc_list_addr: crate::Reg<dma_rx_desc_list_addr::DMA_RX_DESC_LIST_ADDR_SPEC>,
    #[doc = "0x1010 - Points the DMA to the start of the Transmit Descriptor list"]
    pub dma_tx_desc_list_addr: crate::Reg<dma_tx_desc_list_addr::DMA_TX_DESC_LIST_ADDR_SPEC>,
    #[doc = "0x1014 - Used to determine the status of the DMA"]
    pub dma_status: crate::Reg<dma_status::DMA_STATUS_SPEC>,
    #[doc = "0x1018 - Sets the Receive and Transmit operation mode and command"]
    pub dma_oper_mode: crate::Reg<dma_oper_mode::DMA_OPER_MODE_SPEC>,
    #[doc = "0x101c - Enables the interrupts reported in the status register"]
    pub dma_intr_en: crate::Reg<dma_intr_en::DMA_INTR_EN_SPEC>,
    #[doc = "0x1020 - Contains the counters for discarded frames because no Receive Descriptor is available"]
    pub dma_miss_over_counter: crate::Reg<dma_miss_over_counter::DMA_MISS_OVER_COUNTER_SPEC>,
    #[doc = "0x1024 - Watchdog timeout for Receive Interrupt from DMA"]
    pub dma_rx_intr_wdog_timer: crate::Reg<dma_rx_intr_wdog_timer::DMA_RX_INTR_WDOG_TIMER_SPEC>,
    _reserved90: [u8; 0x04],
    #[doc = "0x102c - Provides the active status of the read and write channels of the AHB master interface"]
    pub dma_ahb_status: crate::Reg<dma_ahb_status::DMA_AHB_STATUS_SPEC>,
    _reserved91: [u8; 0x18],
    #[doc = "0x1048 - Contains the start address of the current Transmit Descriptor read by the DMA"]
    pub dma_curr_tx_desc: crate::Reg<dma_curr_tx_desc::DMA_CURR_TX_DESC_SPEC>,
    #[doc = "0x104c - Contains the start address of the current Receive Descriptor read by the DMA"]
    pub dma_curr_rx_desc: crate::Reg<dma_curr_rx_desc::DMA_CURR_RX_DESC_SPEC>,
    #[doc = "0x1050 - Contains the start address of the current Receive Descriptor read by the DMA"]
    pub dma_curr_tx_bufr_addr: crate::Reg<dma_curr_tx_bufr_addr::DMA_CURR_TX_BUFR_ADDR_SPEC>,
    #[doc = "0x1054 - Contains the current Receive Buffer address read by the DMA"]
    pub dma_curr_rx_bufr_addr: crate::Reg<dma_curr_rx_bufr_addr::DMA_CURR_RX_BUFR_ADDR_SPEC>,
}
#[doc = "MAC_CONFIG register accessor: an alias for `Reg<MAC_CONFIG_SPEC>`"]
pub type MAC_CONFIG = crate::Reg<mac_config::MAC_CONFIG_SPEC>;
#[doc = "Operation mode register for the MAC"]
pub mod mac_config;
#[doc = "MAC_FRAME_FLTR register accessor: an alias for `Reg<MAC_FRAME_FLTR_SPEC>`"]
pub type MAC_FRAME_FLTR = crate::Reg<mac_frame_fltr::MAC_FRAME_FLTR_SPEC>;
#[doc = "Contains the frame filtering controls"]
pub mod mac_frame_fltr;
#[doc = "MAC_GMII_ADDR register accessor: an alias for `Reg<MAC_GMII_ADDR_SPEC>`"]
pub type MAC_GMII_ADDR = crate::Reg<mac_gmii_addr::MAC_GMII_ADDR_SPEC>;
#[doc = "Controls the management cycles to an external PHY"]
pub mod mac_gmii_addr;
#[doc = "MAC_GMII_DATA register accessor: an alias for `Reg<MAC_GMII_DATA_SPEC>`"]
pub type MAC_GMII_DATA = crate::Reg<mac_gmii_data::MAC_GMII_DATA_SPEC>;
#[doc = "Contains the data to be written to or read from the PHY register"]
pub mod mac_gmii_data;
#[doc = "MAC_FLOW_CTRL register accessor: an alias for `Reg<MAC_FLOW_CTRL_SPEC>`"]
pub type MAC_FLOW_CTRL = crate::Reg<mac_flow_ctrl::MAC_FLOW_CTRL_SPEC>;
#[doc = "Controls the generation of control frames"]
pub mod mac_flow_ctrl;
#[doc = "MAC_VLAN_TAG register accessor: an alias for `Reg<MAC_VLAN_TAG_SPEC>`"]
pub type MAC_VLAN_TAG = crate::Reg<mac_vlan_tag::MAC_VLAN_TAG_SPEC>;
#[doc = "Identifies IEEE 802.1Q VLAN type frames"]
pub mod mac_vlan_tag;
#[doc = "MAC_DEBUG register accessor: an alias for `Reg<MAC_DEBUG_SPEC>`"]
pub type MAC_DEBUG = crate::Reg<mac_debug::MAC_DEBUG_SPEC>;
#[doc = "Gives the status of the various internal blocks for debugging"]
pub mod mac_debug;
#[doc = "MAC_INTR_STAT register accessor: an alias for `Reg<MAC_INTR_STAT_SPEC>`"]
pub type MAC_INTR_STAT = crate::Reg<mac_intr_stat::MAC_INTR_STAT_SPEC>;
#[doc = "Contains the interrupt status"]
pub mod mac_intr_stat;
#[doc = "MAC_INTR_MASK register accessor: an alias for `Reg<MAC_INTR_MASK_SPEC>`"]
pub type MAC_INTR_MASK = crate::Reg<mac_intr_mask::MAC_INTR_MASK_SPEC>;
#[doc = "Contains the masks for generating interrupt"]
pub mod mac_intr_mask;
#[doc = "MAC_ADDR_H register accessor: an alias for `Reg<MAC_ADDR_H_SPEC>`"]
pub type MAC_ADDR_H = crate::Reg<mac_addr_h::MAC_ADDR_H_SPEC>;
#[doc = "Contains the high 16-bits of the first MAC Address"]
pub mod mac_addr_h;
#[doc = "MAC_ADDR_L register accessor: an alias for `Reg<MAC_ADDR_L_SPEC>`"]
pub type MAC_ADDR_L = crate::Reg<mac_addr_l::MAC_ADDR_L_SPEC>;
#[doc = "Contains the Low 32-bits of the first MAC Address"]
pub mod mac_addr_l;
#[doc = "MAC_WDOG_TO register accessor: an alias for `Reg<MAC_WDOG_TO_SPEC>`"]
pub type MAC_WDOG_TO = crate::Reg<mac_wdog_to::MAC_WDOG_TO_SPEC>;
#[doc = "Controls the watchdog time-out for received frames"]
pub mod mac_wdog_to;
#[doc = "MMC_CNTRL register accessor: an alias for `Reg<MMC_CNTRL_SPEC>`"]
pub type MMC_CNTRL = crate::Reg<mmc_cntrl::MMC_CNTRL_SPEC>;
#[doc = "MMC Control Register"]
pub mod mmc_cntrl;
#[doc = "MMC_INTR_RX register accessor: an alias for `Reg<MMC_INTR_RX_SPEC>`"]
pub type MMC_INTR_RX = crate::Reg<mmc_intr_rx::MMC_INTR_RX_SPEC>;
#[doc = "MMC Receive Interrupt Register"]
pub mod mmc_intr_rx;
#[doc = "MMC_INTR_TX register accessor: an alias for `Reg<MMC_INTR_TX_SPEC>`"]
pub type MMC_INTR_TX = crate::Reg<mmc_intr_tx::MMC_INTR_TX_SPEC>;
#[doc = "MMC Transmit Interrupt Register"]
pub mod mmc_intr_tx;
#[doc = "MMC_INTR_MASK_RX register accessor: an alias for `Reg<MMC_INTR_MASK_RX_SPEC>`"]
pub type MMC_INTR_MASK_RX = crate::Reg<mmc_intr_mask_rx::MMC_INTR_MASK_RX_SPEC>;
#[doc = "MMC Receive Interrupt Mask Register"]
pub mod mmc_intr_mask_rx;
#[doc = "MMC_INTR_MASK_TX register accessor: an alias for `Reg<MMC_INTR_MASK_TX_SPEC>`"]
pub type MMC_INTR_MASK_TX = crate::Reg<mmc_intr_mask_tx::MMC_INTR_MASK_TX_SPEC>;
#[doc = "MMC Transmit Interrupt Mask Register"]
pub mod mmc_intr_mask_tx;
#[doc = "TXOCTETCOUNT_GB register accessor: an alias for `Reg<TXOCTETCOUNT_GB_SPEC>`"]
pub type TXOCTETCOUNT_GB = crate::Reg<txoctetcount_gb::TXOCTETCOUNT_GB_SPEC>;
#[doc = "MMC Transmit Count"]
pub mod txoctetcount_gb;
#[doc = "TXFRAMECOUNT_GB register accessor: an alias for `Reg<TXFRAMECOUNT_GB_SPEC>`"]
pub type TXFRAMECOUNT_GB = crate::Reg<txframecount_gb::TXFRAMECOUNT_GB_SPEC>;
#[doc = "MMC Frame Count Register"]
pub mod txframecount_gb;
#[doc = "TXBCASTFRAMES_G register accessor: an alias for `Reg<TXBCASTFRAMES_G_SPEC>`"]
pub type TXBCASTFRAMES_G = crate::Reg<txbcastframes_g::TXBCASTFRAMES_G_SPEC>;
#[doc = "MMC Good Broadcast Frames Register"]
pub mod txbcastframes_g;
#[doc = "TXMCASTFRAMES_G register accessor: an alias for `Reg<TXMCASTFRAMES_G_SPEC>`"]
pub type TXMCASTFRAMES_G = crate::Reg<txmcastframes_g::TXMCASTFRAMES_G_SPEC>;
#[doc = "MMC Good Multicast Frames Register"]
pub mod txmcastframes_g;
#[doc = "TX64OCT_GB register accessor: an alias for `Reg<TX64OCT_GB_SPEC>`"]
pub type TX64OCT_GB = crate::Reg<tx64oct_gb::TX64OCT_GB_SPEC>;
#[doc = "MMC Good and bad Frames transmitted with length 64"]
pub mod tx64oct_gb;
#[doc = "TX65TO127OCT_GB register accessor: an alias for `Reg<TX65TO127OCT_GB_SPEC>`"]
pub type TX65TO127OCT_GB = crate::Reg<tx65to127oct_gb::TX65TO127OCT_GB_SPEC>;
#[doc = "MMC Good and bad Frames transmitted with length 65 to 127"]
pub mod tx65to127oct_gb;
#[doc = "TX128TO255OCT_GB register accessor: an alias for `Reg<TX128TO255OCT_GB_SPEC>`"]
pub type TX128TO255OCT_GB = crate::Reg<tx128to255oct_gb::TX128TO255OCT_GB_SPEC>;
#[doc = "MMC Good and bad Frames transmitted with length 128 to 255"]
pub mod tx128to255oct_gb;
#[doc = "TX256TO511OCT_GB register accessor: an alias for `Reg<TX256TO511OCT_GB_SPEC>`"]
pub type TX256TO511OCT_GB = crate::Reg<tx256to511oct_gb::TX256TO511OCT_GB_SPEC>;
#[doc = "MMC Good and bad Frames transmitted with length 256 to 511"]
pub mod tx256to511oct_gb;
#[doc = "TX512TO1023OCT_GB register accessor: an alias for `Reg<TX512TO1023OCT_GB_SPEC>`"]
pub type TX512TO1023OCT_GB = crate::Reg<tx512to1023oct_gb::TX512TO1023OCT_GB_SPEC>;
#[doc = "MMC Good and bad Frames transmitted with length 512 to 1023"]
pub mod tx512to1023oct_gb;
#[doc = "TX1024MAXOCT_GB register accessor: an alias for `Reg<TX1024MAXOCT_GB_SPEC>`"]
pub type TX1024MAXOCT_GB = crate::Reg<tx1024maxoct_gb::TX1024MAXOCT_GB_SPEC>;
#[doc = "MMC Good and bad Frames transmitted with length 1024 to max bytes"]
pub mod tx1024maxoct_gb;
#[doc = "TXUCASTFRAME_GB register accessor: an alias for `Reg<TXUCASTFRAME_GB_SPEC>`"]
pub type TXUCASTFRAME_GB = crate::Reg<txucastframe_gb::TXUCASTFRAME_GB_SPEC>;
#[doc = "MMC number of good and bad unicast frames transmitted"]
pub mod txucastframe_gb;
#[doc = "TXMCASTFRAME_GB register accessor: an alias for `Reg<TXMCASTFRAME_GB_SPEC>`"]
pub type TXMCASTFRAME_GB = crate::Reg<txmcastframe_gb::TXMCASTFRAME_GB_SPEC>;
#[doc = "MMC number of good and bad MULTIcast frames transmitted"]
pub mod txmcastframe_gb;
#[doc = "TXBCASTFRAME_GB register accessor: an alias for `Reg<TXBCASTFRAME_GB_SPEC>`"]
pub type TXBCASTFRAME_GB = crate::Reg<txbcastframe_gb::TXBCASTFRAME_GB_SPEC>;
#[doc = "MMC number of good and bad broadcast frames transmitted"]
pub mod txbcastframe_gb;
#[doc = "TXUNDERERR register accessor: an alias for `Reg<TXUNDERERR_SPEC>`"]
pub type TXUNDERERR = crate::Reg<txundererr::TXUNDERERR_SPEC>;
#[doc = "MMC number of frames aborted because of frame underflow error"]
pub mod txundererr;
#[doc = "TXSINGLECOL_G register accessor: an alias for `Reg<TXSINGLECOL_G_SPEC>`"]
pub type TXSINGLECOL_G = crate::Reg<txsinglecol_g::TXSINGLECOL_G_SPEC>;
#[doc = "MMC Number of successfully transmitted frames after a single collision"]
pub mod txsinglecol_g;
#[doc = "TXMULTICOL_G register accessor: an alias for `Reg<TXMULTICOL_G_SPEC>`"]
pub type TXMULTICOL_G = crate::Reg<txmulticol_g::TXMULTICOL_G_SPEC>;
#[doc = "MMC Number of successfully transmitted frames after multiple collisions"]
pub mod txmulticol_g;
#[doc = "TXDEFERRED register accessor: an alias for `Reg<TXDEFERRED_SPEC>`"]
pub type TXDEFERRED = crate::Reg<txdeferred::TXDEFERRED_SPEC>;
#[doc = "MMC Number of successfully transmitted frames after a deferral"]
pub mod txdeferred;
#[doc = "TXLATECOL register accessor: an alias for `Reg<TXLATECOL_SPEC>`"]
pub type TXLATECOL = crate::Reg<txlatecol::TXLATECOL_SPEC>;
#[doc = "MMC Number of aborted frames because of late collision error"]
pub mod txlatecol;
#[doc = "TXEXESSCOL register accessor: an alias for `Reg<TXEXESSCOL_SPEC>`"]
pub type TXEXESSCOL = crate::Reg<txexesscol::TXEXESSCOL_SPEC>;
#[doc = "MMC Number of aborted frames because of excessive collision errors"]
pub mod txexesscol;
#[doc = "TXCARRIERERROR register accessor: an alias for `Reg<TXCARRIERERROR_SPEC>`"]
pub type TXCARRIERERROR = crate::Reg<txcarriererror::TXCARRIERERROR_SPEC>;
#[doc = "MMC Number of aborted frames because of carrier sense error"]
pub mod txcarriererror;
#[doc = "TXOCTETCOUNT_G register accessor: an alias for `Reg<TXOCTETCOUNT_G_SPEC>`"]
pub type TXOCTETCOUNT_G = crate::Reg<txoctetcount_g::TXOCTETCOUNT_G_SPEC>;
#[doc = "MMC Number of bytes transmitted frames only in good frames"]
pub mod txoctetcount_g;
#[doc = "TXFRAMECOUNT_G register accessor: an alias for `Reg<TXFRAMECOUNT_G_SPEC>`"]
pub type TXFRAMECOUNT_G = crate::Reg<txframecount_g::TXFRAMECOUNT_G_SPEC>;
#[doc = "MMC Number of good frames transmitted"]
pub mod txframecount_g;
#[doc = "TXEXCESSDEF register accessor: an alias for `Reg<TXEXCESSDEF_SPEC>`"]
pub type TXEXCESSDEF = crate::Reg<txexcessdef::TXEXCESSDEF_SPEC>;
#[doc = "MMC Number of frames aborted because of excessive deferral error"]
pub mod txexcessdef;
#[doc = "TXPAUSEFRAMES register accessor: an alias for `Reg<TXPAUSEFRAMES_SPEC>`"]
pub type TXPAUSEFRAMES = crate::Reg<txpauseframes::TXPAUSEFRAMES_SPEC>;
#[doc = "MMC Number of good pause frames transmitted"]
pub mod txpauseframes;
#[doc = "TXLANFRAMES_G register accessor: an alias for `Reg<TXLANFRAMES_G_SPEC>`"]
pub type TXLANFRAMES_G = crate::Reg<txlanframes_g::TXLANFRAMES_G_SPEC>;
#[doc = "MMC Number of good VLAN frames transmitted"]
pub mod txlanframes_g;
#[doc = "TXOVERSIZE_G register accessor: an alias for `Reg<TXOVERSIZE_G_SPEC>`"]
pub type TXOVERSIZE_G = crate::Reg<txoversize_g::TXOVERSIZE_G_SPEC>;
#[doc = "MMC Number of frames transmitted without errors"]
pub mod txoversize_g;
#[doc = "RXFRAMECOUNT_GB register accessor: an alias for `Reg<RXFRAMECOUNT_GB_SPEC>`"]
pub type RXFRAMECOUNT_GB = crate::Reg<rxframecount_gb::RXFRAMECOUNT_GB_SPEC>;
#[doc = "MMC Number of good and bad frames received"]
pub mod rxframecount_gb;
#[doc = "RXOCTETCOUNT_GB register accessor: an alias for `Reg<RXOCTETCOUNT_GB_SPEC>`"]
pub type RXOCTETCOUNT_GB = crate::Reg<rxoctetcount_gb::RXOCTETCOUNT_GB_SPEC>;
#[doc = "MMC Number of bytes received in good and bad frames"]
pub mod rxoctetcount_gb;
#[doc = "RXOCTETCOUNT_G register accessor: an alias for `Reg<RXOCTETCOUNT_G_SPEC>`"]
pub type RXOCTETCOUNT_G = crate::Reg<rxoctetcount_g::RXOCTETCOUNT_G_SPEC>;
#[doc = "MMC Number of bytes received in good frames only"]
pub mod rxoctetcount_g;
#[doc = "RXBCASTFRAMES_G register accessor: an alias for `Reg<RXBCASTFRAMES_G_SPEC>`"]
pub type RXBCASTFRAMES_G = crate::Reg<rxbcastframes_g::RXBCASTFRAMES_G_SPEC>;
#[doc = "MMC Number of good broadcast frames received"]
pub mod rxbcastframes_g;
#[doc = "RXMCASTFRAMES_G register accessor: an alias for `Reg<RXMCASTFRAMES_G_SPEC>`"]
pub type RXMCASTFRAMES_G = crate::Reg<rxmcastframes_g::RXMCASTFRAMES_G_SPEC>;
#[doc = "MMC Number of good multicast frames received"]
pub mod rxmcastframes_g;
#[doc = "RXCRCERROR register accessor: an alias for `Reg<RXCRCERROR_SPEC>`"]
pub type RXCRCERROR = crate::Reg<rxcrcerror::RXCRCERROR_SPEC>;
#[doc = "MMC Number of frames received with CRC error"]
pub mod rxcrcerror;
#[doc = "RXALIGNERROR register accessor: an alias for `Reg<RXALIGNERROR_SPEC>`"]
pub type RXALIGNERROR = crate::Reg<rxalignerror::RXALIGNERROR_SPEC>;
#[doc = "MMC Number of frames received with alignment error"]
pub mod rxalignerror;
#[doc = "RXRUNTERROR register accessor: an alias for `Reg<RXRUNTERROR_SPEC>`"]
pub type RXRUNTERROR = crate::Reg<rxrunterror::RXRUNTERROR_SPEC>;
#[doc = "MMC Number of frames received with runt error"]
pub mod rxrunterror;
#[doc = "RXJABBERERROR register accessor: an alias for `Reg<RXJABBERERROR_SPEC>`"]
pub type RXJABBERERROR = crate::Reg<rxjabbererror::RXJABBERERROR_SPEC>;
#[doc = "MMC Number of giant frames received with length greater than 1518 bytes and with CRC error"]
pub mod rxjabbererror;
#[doc = "RXUNDERSIZE_G register accessor: an alias for `Reg<RXUNDERSIZE_G_SPEC>`"]
pub type RXUNDERSIZE_G = crate::Reg<rxundersize_g::RXUNDERSIZE_G_SPEC>;
#[doc = "MMC Number of frames received with length less than 64 bytes"]
pub mod rxundersize_g;
#[doc = "RXOVERSIZE_G register accessor: an alias for `Reg<RXOVERSIZE_G_SPEC>`"]
pub type RXOVERSIZE_G = crate::Reg<rxoversize_g::RXOVERSIZE_G_SPEC>;
#[doc = "MMC Number of frames received without errors with length greater than the max size"]
pub mod rxoversize_g;
#[doc = "RX64OCTETS_GB register accessor: an alias for `Reg<RX64OCTETS_GB_SPEC>`"]
pub type RX64OCTETS_GB = crate::Reg<rx64octets_gb::RX64OCTETS_GB_SPEC>;
#[doc = "MMC Number of good and bad frames received with length 64 bytes"]
pub mod rx64octets_gb;
#[doc = "RX65TO127OCT_GB register accessor: an alias for `Reg<RX65TO127OCT_GB_SPEC>`"]
pub type RX65TO127OCT_GB = crate::Reg<rx65to127oct_gb::RX65TO127OCT_GB_SPEC>;
#[doc = "MMC Number of good and bad frames received with length between 65 and 127 bytes"]
pub mod rx65to127oct_gb;
#[doc = "RX128TO255OCT_GB register accessor: an alias for `Reg<RX128TO255OCT_GB_SPEC>`"]
pub type RX128TO255OCT_GB = crate::Reg<rx128to255oct_gb::RX128TO255OCT_GB_SPEC>;
#[doc = "MMC Number of good and bad frames received with length between 128 and 255 bytes"]
pub mod rx128to255oct_gb;
#[doc = "RX256TO511OCT_GB register accessor: an alias for `Reg<RX256TO511OCT_GB_SPEC>`"]
pub type RX256TO511OCT_GB = crate::Reg<rx256to511oct_gb::RX256TO511OCT_GB_SPEC>;
#[doc = "MMC Number of good and bad frames received with length between 256 and 511 bytes"]
pub mod rx256to511oct_gb;
#[doc = "RX512TO1023OCT_GB register accessor: an alias for `Reg<RX512TO1023OCT_GB_SPEC>`"]
pub type RX512TO1023OCT_GB = crate::Reg<rx512to1023oct_gb::RX512TO1023OCT_GB_SPEC>;
#[doc = "MMC Number of good and bad frames received with length between 512 and 1023 bytes"]
pub mod rx512to1023oct_gb;
#[doc = "RX1024MAXOCT_GB register accessor: an alias for `Reg<RX1024MAXOCT_GB_SPEC>`"]
pub type RX1024MAXOCT_GB = crate::Reg<rx1024maxoct_gb::RX1024MAXOCT_GB_SPEC>;
#[doc = "MMC Number of good and bad frames received with length between 1024 and max size bytes"]
pub mod rx1024maxoct_gb;
#[doc = "RXUCASTFRAMES_G register accessor: an alias for `Reg<RXUCASTFRAMES_G_SPEC>`"]
pub type RXUCASTFRAMES_G = crate::Reg<rxucastframes_g::RXUCASTFRAMES_G_SPEC>;
#[doc = "MMC Number of received good unicast frames"]
pub mod rxucastframes_g;
#[doc = "RXLENGTHERROR register accessor: an alias for `Reg<RXLENGTHERROR_SPEC>`"]
pub type RXLENGTHERROR = crate::Reg<rxlengtherror::RXLENGTHERROR_SPEC>;
#[doc = "MMC Number of frames received with length error"]
pub mod rxlengtherror;
#[doc = "RXOUTRANGETYPE register accessor: an alias for `Reg<RXOUTRANGETYPE_SPEC>`"]
pub type RXOUTRANGETYPE = crate::Reg<rxoutrangetype::RXOUTRANGETYPE_SPEC>;
#[doc = "MMC Number of frames received with length field not equal to the valid frame size"]
pub mod rxoutrangetype;
#[doc = "RXPAUSEFRAMES register accessor: an alias for `Reg<RXPAUSEFRAMES_SPEC>`"]
pub type RXPAUSEFRAMES = crate::Reg<rxpauseframes::RXPAUSEFRAMES_SPEC>;
#[doc = "MMC Number of good and valid Pause frames received"]
pub mod rxpauseframes;
#[doc = "RXFIFOOVERFLOW register accessor: an alias for `Reg<RXFIFOOVERFLOW_SPEC>`"]
pub type RXFIFOOVERFLOW = crate::Reg<rxfifooverflow::RXFIFOOVERFLOW_SPEC>;
#[doc = "MMC Number of missed received frames because of FIFO overflow"]
pub mod rxfifooverflow;
#[doc = "RXVLANFRAMES_GB register accessor: an alias for `Reg<RXVLANFRAMES_GB_SPEC>`"]
pub type RXVLANFRAMES_GB = crate::Reg<rxvlanframes_gb::RXVLANFRAMES_GB_SPEC>;
#[doc = "MMC Number of good and bad VLAN frames received"]
pub mod rxvlanframes_gb;
#[doc = "RXWDOGERROR register accessor: an alias for `Reg<RXWDOGERROR_SPEC>`"]
pub type RXWDOGERROR = crate::Reg<rxwdogerror::RXWDOGERROR_SPEC>;
#[doc = "MMC Number of frames received with error because of watchdog timeout error"]
pub mod rxwdogerror;
#[doc = "RXRCVERROR register accessor: an alias for `Reg<RXRCVERROR_SPEC>`"]
pub type RXRCVERROR = crate::Reg<rxrcverror::RXRCVERROR_SPEC>;
#[doc = "MMC Number of frames received with Receive error or Frame Extension error"]
pub mod rxrcverror;
#[doc = "RXCTRLFRAMES_G register accessor: an alias for `Reg<RXCTRLFRAMES_G_SPEC>`"]
pub type RXCTRLFRAMES_G = crate::Reg<rxctrlframes_g::RXCTRLFRAMES_G_SPEC>;
#[doc = "MMC Number of received good control frames"]
pub mod rxctrlframes_g;
#[doc = "VLAN_INCREPLACE register accessor: an alias for `Reg<VLAN_INCREPLACE_SPEC>`"]
pub type VLAN_INCREPLACE = crate::Reg<vlan_increplace::VLAN_INCREPLACE_SPEC>;
#[doc = "Holds the VLAN Tag for insertion into or replacement in the transmit frames"]
pub mod vlan_increplace;
#[doc = "VLAN_HASHTABLE register accessor: an alias for `Reg<VLAN_HASHTABLE_SPEC>`"]
pub type VLAN_HASHTABLE = crate::Reg<vlan_hashtable::VLAN_HASHTABLE_SPEC>;
#[doc = "Holds the VLAN Hash Table"]
pub mod vlan_hashtable;
#[doc = "TIMESTAMP_CTRL register accessor: an alias for `Reg<TIMESTAMP_CTRL_SPEC>`"]
pub type TIMESTAMP_CTRL = crate::Reg<timestamp_ctrl::TIMESTAMP_CTRL_SPEC>;
#[doc = "Controls the IEEE 1588 timestamp generation and update logic"]
pub mod timestamp_ctrl;
#[doc = "SUBSEC_INC register accessor: an alias for `Reg<SUBSEC_INC_SPEC>`"]
pub type SUBSEC_INC = crate::Reg<subsec_inc::SUBSEC_INC_SPEC>;
#[doc = "Holds the 8-bit value by which the Sub-Second register is incremented"]
pub mod subsec_inc;
#[doc = "SYSTIME_SECONDS register accessor: an alias for `Reg<SYSTIME_SECONDS_SPEC>`"]
pub type SYSTIME_SECONDS = crate::Reg<systime_seconds::SYSTIME_SECONDS_SPEC>;
#[doc = "Holds the lower 32 bits of the second field of the system time"]
pub mod systime_seconds;
#[doc = "SYSTIME_NANOSEC register accessor: an alias for `Reg<SYSTIME_NANOSEC_SPEC>`"]
pub type SYSTIME_NANOSEC = crate::Reg<systime_nanosec::SYSTIME_NANOSEC_SPEC>;
#[doc = "Holds 32 bits of the nano-second field of the system time"]
pub mod systime_nanosec;
#[doc = "SYSTIME_SECSUPDAT register accessor: an alias for `Reg<SYSTIME_SECSUPDAT_SPEC>`"]
pub type SYSTIME_SECSUPDAT = crate::Reg<systime_secsupdat::SYSTIME_SECSUPDAT_SPEC>;
#[doc = "Holds the lower 32 bits of the second field to be written to, added to, or subtracted from the system time value"]
pub mod systime_secsupdat;
#[doc = "SYSTIME_NSECUP register accessor: an alias for `Reg<SYSTIME_NSECUP_SPEC>`"]
pub type SYSTIME_NSECUP = crate::Reg<systime_nsecup::SYSTIME_NSECUP_SPEC>;
#[doc = "Holds 32 bits of the nano-second field to be written to, added to, or subtracted from the system time value"]
pub mod systime_nsecup;
#[doc = "TIMESTAMPADDEND register accessor: an alias for `Reg<TIMESTAMPADDEND_SPEC>`"]
pub type TIMESTAMPADDEND = crate::Reg<timestampaddend::TIMESTAMPADDEND_SPEC>;
#[doc = "This register is used by software to re-adjust the clock frequency linearly to match the Master clock frequency"]
pub mod timestampaddend;
#[doc = "TARGET_TIME_SECS register accessor: an alias for `Reg<TARGET_TIME_SECS_SPEC>`"]
pub type TARGET_TIME_SECS = crate::Reg<target_time_secs::TARGET_TIME_SECS_SPEC>;
#[doc = "Holds the high 32-bits of time to be compared with the system time"]
pub mod target_time_secs;
#[doc = "TARGET_TIME_NSEC register accessor: an alias for `Reg<TARGET_TIME_NSEC_SPEC>`"]
pub type TARGET_TIME_NSEC = crate::Reg<target_time_nsec::TARGET_TIME_NSEC_SPEC>;
#[doc = "Holds the lower 32-bits of time to be compared with the system time"]
pub mod target_time_nsec;
#[doc = "DMA_BUS_MODE register accessor: an alias for `Reg<DMA_BUS_MODE_SPEC>`"]
pub type DMA_BUS_MODE = crate::Reg<dma_bus_mode::DMA_BUS_MODE_SPEC>;
#[doc = "Controls the DMA Host Interface Mode"]
pub mod dma_bus_mode;
#[doc = "DMA_TX_POLL_DEMAND register accessor: an alias for `Reg<DMA_TX_POLL_DEMAND_SPEC>`"]
pub type DMA_TX_POLL_DEMAND = crate::Reg<dma_tx_poll_demand::DMA_TX_POLL_DEMAND_SPEC>;
#[doc = "Used by the host to instruct the DMA to poll the transmit Descriptor list"]
pub mod dma_tx_poll_demand;
#[doc = "DMA_RX_POLL_DEMAND register accessor: an alias for `Reg<DMA_RX_POLL_DEMAND_SPEC>`"]
pub type DMA_RX_POLL_DEMAND = crate::Reg<dma_rx_poll_demand::DMA_RX_POLL_DEMAND_SPEC>;
#[doc = "Used by the host to instruct the DMA to poll the Receive Descriptor list"]
pub mod dma_rx_poll_demand;
#[doc = "DMA_RX_DESC_LIST_ADDR register accessor: an alias for `Reg<DMA_RX_DESC_LIST_ADDR_SPEC>`"]
pub type DMA_RX_DESC_LIST_ADDR = crate::Reg<dma_rx_desc_list_addr::DMA_RX_DESC_LIST_ADDR_SPEC>;
#[doc = "Points the DMA to the start of the Receive Descriptor list"]
pub mod dma_rx_desc_list_addr;
#[doc = "DMA_TX_DESC_LIST_ADDR register accessor: an alias for `Reg<DMA_TX_DESC_LIST_ADDR_SPEC>`"]
pub type DMA_TX_DESC_LIST_ADDR = crate::Reg<dma_tx_desc_list_addr::DMA_TX_DESC_LIST_ADDR_SPEC>;
#[doc = "Points the DMA to the start of the Transmit Descriptor list"]
pub mod dma_tx_desc_list_addr;
#[doc = "DMA_STATUS register accessor: an alias for `Reg<DMA_STATUS_SPEC>`"]
pub type DMA_STATUS = crate::Reg<dma_status::DMA_STATUS_SPEC>;
#[doc = "Used to determine the status of the DMA"]
pub mod dma_status;
#[doc = "DMA_OPER_MODE register accessor: an alias for `Reg<DMA_OPER_MODE_SPEC>`"]
pub type DMA_OPER_MODE = crate::Reg<dma_oper_mode::DMA_OPER_MODE_SPEC>;
#[doc = "Sets the Receive and Transmit operation mode and command"]
pub mod dma_oper_mode;
#[doc = "DMA_INTR_EN register accessor: an alias for `Reg<DMA_INTR_EN_SPEC>`"]
pub type DMA_INTR_EN = crate::Reg<dma_intr_en::DMA_INTR_EN_SPEC>;
#[doc = "Enables the interrupts reported in the status register"]
pub mod dma_intr_en;
#[doc = "DMA_MISS_OVER_COUNTER register accessor: an alias for `Reg<DMA_MISS_OVER_COUNTER_SPEC>`"]
pub type DMA_MISS_OVER_COUNTER = crate::Reg<dma_miss_over_counter::DMA_MISS_OVER_COUNTER_SPEC>;
#[doc = "Contains the counters for discarded frames because no Receive Descriptor is available"]
pub mod dma_miss_over_counter;
#[doc = "DMA_RX_INTR_WDOG_TIMER register accessor: an alias for `Reg<DMA_RX_INTR_WDOG_TIMER_SPEC>`"]
pub type DMA_RX_INTR_WDOG_TIMER = crate::Reg<dma_rx_intr_wdog_timer::DMA_RX_INTR_WDOG_TIMER_SPEC>;
#[doc = "Watchdog timeout for Receive Interrupt from DMA"]
pub mod dma_rx_intr_wdog_timer;
#[doc = "DMA_AHB_STATUS register accessor: an alias for `Reg<DMA_AHB_STATUS_SPEC>`"]
pub type DMA_AHB_STATUS = crate::Reg<dma_ahb_status::DMA_AHB_STATUS_SPEC>;
#[doc = "Provides the active status of the read and write channels of the AHB master interface"]
pub mod dma_ahb_status;
#[doc = "DMA_CURR_TX_DESC register accessor: an alias for `Reg<DMA_CURR_TX_DESC_SPEC>`"]
pub type DMA_CURR_TX_DESC = crate::Reg<dma_curr_tx_desc::DMA_CURR_TX_DESC_SPEC>;
#[doc = "Contains the start address of the current Transmit Descriptor read by the DMA"]
pub mod dma_curr_tx_desc;
#[doc = "DMA_CURR_RX_DESC register accessor: an alias for `Reg<DMA_CURR_RX_DESC_SPEC>`"]
pub type DMA_CURR_RX_DESC = crate::Reg<dma_curr_rx_desc::DMA_CURR_RX_DESC_SPEC>;
#[doc = "Contains the start address of the current Receive Descriptor read by the DMA"]
pub mod dma_curr_rx_desc;
#[doc = "DMA_CURR_TX_BUFR_ADDR register accessor: an alias for `Reg<DMA_CURR_TX_BUFR_ADDR_SPEC>`"]
pub type DMA_CURR_TX_BUFR_ADDR = crate::Reg<dma_curr_tx_bufr_addr::DMA_CURR_TX_BUFR_ADDR_SPEC>;
#[doc = "Contains the start address of the current Receive Descriptor read by the DMA"]
pub mod dma_curr_tx_bufr_addr;
#[doc = "DMA_CURR_RX_BUFR_ADDR register accessor: an alias for `Reg<DMA_CURR_RX_BUFR_ADDR_SPEC>`"]
pub type DMA_CURR_RX_BUFR_ADDR = crate::Reg<dma_curr_rx_bufr_addr::DMA_CURR_RX_BUFR_ADDR_SPEC>;
#[doc = "Contains the current Receive Buffer address read by the DMA"]
pub mod dma_curr_rx_bufr_addr;
