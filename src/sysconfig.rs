#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - System Reset Status"]
    pub rst_stat: crate::Reg<rst_stat::RST_STAT_SPEC>,
    #[doc = "0x04 - ROM Reset Control"]
    pub rst_cntl_rom: crate::Reg<rst_cntl_rom::RST_CNTL_ROM_SPEC>,
    #[doc = "0x08 - RAM Reset Control"]
    pub rst_cntl_ram0: crate::Reg<rst_cntl_ram0::RST_CNTL_RAM0_SPEC>,
    #[doc = "0x0c - RAM Reset Control"]
    pub rst_cntl_ram1: crate::Reg<rst_cntl_ram1::RST_CNTL_RAM1_SPEC>,
    #[doc = "0x10 - ROM Protection Configuration"]
    pub rom_prot: crate::Reg<rom_prot::ROM_PROT_SPEC>,
    #[doc = "0x14 - ROM Scrub Period Configuration"]
    pub rom_scrub: crate::Reg<rom_scrub::ROM_SCRUB_SPEC>,
    #[doc = "0x18 - RAM0 Scrub Period Configuration"]
    pub ram0_scrub: crate::Reg<ram0_scrub::RAM0_SCRUB_SPEC>,
    #[doc = "0x1c - RAM1 Scrub Period Configuration"]
    pub ram1_scrub: crate::Reg<ram1_scrub::RAM1_SCRUB_SPEC>,
    #[doc = "0x20 - Enable EDAC Error Interrupt Register"]
    pub irq_enb: crate::Reg<irq_enb::IRQ_ENB_SPEC>,
    #[doc = "0x24 - Raw EDAC Error Interrupt Status"]
    pub irq_raw: crate::Reg<irq_raw::IRQ_RAW_SPEC>,
    #[doc = "0x28 - Enabled EDAC Error Interrupt Status"]
    pub irq_end: crate::Reg<irq_end::IRQ_END_SPEC>,
    #[doc = "0x2c - Clear EDAC Error Interrupt Status"]
    pub irq_clr: crate::Reg<irq_clr::IRQ_CLR_SPEC>,
    #[doc = "0x30 - Count of RAM0 EDAC Single Bit Errors"]
    pub ram0_sbe: crate::Reg<ram0_sbe::RAM0_SBE_SPEC>,
    #[doc = "0x34 - Count of RAM1 EDAC Single Bit Errors"]
    pub ram1_sbe: crate::Reg<ram1_sbe::RAM1_SBE_SPEC>,
    #[doc = "0x38 - Count of RAM0 EDAC Multi Bit Errors"]
    pub ram0_mbe: crate::Reg<ram0_mbe::RAM0_MBE_SPEC>,
    #[doc = "0x3c - Count of RAM1 EDAC Multi Bit Errors"]
    pub ram1_mbe: crate::Reg<ram1_mbe::RAM1_MBE_SPEC>,
    #[doc = "0x40 - Count of ROM EDAC Single Bit Errors"]
    pub rom_sbe: crate::Reg<rom_sbe::ROM_SBE_SPEC>,
    #[doc = "0x44 - Count of ROM EDAC Multi Bit Errors"]
    pub rom_mbe: crate::Reg<rom_mbe::ROM_MBE_SPEC>,
    #[doc = "0x48 - ROM BOOT Retry count"]
    pub rom_retries: crate::Reg<rom_retries::ROM_RETRIES_SPEC>,
    #[doc = "0x4c - Register Refresh Rate for TMR registers"]
    pub refresh_config_h: crate::Reg<refresh_config_h::REFRESH_CONFIG_H_SPEC>,
    #[doc = "0x50 - TIM Reset Control"]
    pub tim_reset: crate::Reg<tim_reset::TIM_RESET_SPEC>,
    #[doc = "0x54 - TIM Enable Control"]
    pub tim_clk_enable: crate::Reg<tim_clk_enable::TIM_CLK_ENABLE_SPEC>,
    #[doc = "0x58 - Peripheral Reset Control"]
    pub peripheral_reset: crate::Reg<peripheral_reset::PERIPHERAL_RESET_SPEC>,
    #[doc = "0x5c - Peripheral Enable Control"]
    pub peripheral_clk_enable: crate::Reg<peripheral_clk_enable::PERIPHERAL_CLK_ENABLE_SPEC>,
    #[doc = "0x60 - SPW M4 control register"]
    pub spw_m4_ctrl: crate::Reg<spw_m4_ctrl::SPW_M4_CTRL_SPEC>,
    #[doc = "0x64 - PMU Control Register"]
    pub pmu_ctrl: crate::Reg<pmu_ctrl::PMU_CTRL_SPEC>,
    #[doc = "0x68 - Wakeup Control"]
    pub wakeup_cnt: crate::Reg<wakeup_cnt::WAKEUP_CNT_SPEC>,
    #[doc = "0x6c - EBI Config Register 0"]
    pub ebi_cfg0: crate::Reg<ebi_cfg0::EBI_CFG0_SPEC>,
    #[doc = "0x70 - EBI Config Register 1"]
    pub ebi_cfg1: crate::Reg<ebi_cfg1::EBI_CFG1_SPEC>,
    #[doc = "0x74 - EBI Config Register 2"]
    pub ebi_cfg2: crate::Reg<ebi_cfg2::EBI_CFG2_SPEC>,
    #[doc = "0x78 - EBI Config Register 3"]
    pub ebi_cfg3: crate::Reg<ebi_cfg3::EBI_CFG3_SPEC>,
    #[doc = "0x7c - Analog Control Register"]
    pub analog_cntl: crate::Reg<analog_cntl::ANALOG_CNTL_SPEC>,
    #[doc = "0x80 - Initial SpW Clock Divider Value"]
    pub sw_clkdiv10: crate::Reg<sw_clkdiv10::SW_CLKDIV10_SPEC>,
    #[doc = "0x84 - Register Refresh Rate for TMR registers"]
    pub refresh_config_l: crate::Reg<refresh_config_l::REFRESH_CONFIG_L_SPEC>,
    _reserved34: [u8; 0x0f48],
    #[doc = "0xfd0 - DAC0 Calibration Register"]
    pub dac0_cal: crate::Reg<dac0_cal::DAC0_CAL_SPEC>,
    #[doc = "0xfd4 - DAC1 Calibration Register"]
    pub dac1_cal: crate::Reg<dac1_cal::DAC1_CAL_SPEC>,
    #[doc = "0xfd8 - ADC Calibration Register"]
    pub adc_cal: crate::Reg<adc_cal::ADC_CAL_SPEC>,
    #[doc = "0xfdc - Bandgap Calibration Register"]
    pub bg_cal: crate::Reg<bg_cal::BG_CAL_SPEC>,
    #[doc = "0xfe0 - Digital LDO Regulator Calibration Register"]
    pub dreg_cal: crate::Reg<dreg_cal::DREG_CAL_SPEC>,
    #[doc = "0xfe4 - Analog LDO Regulator Calibration Register"]
    pub areg_cal: crate::Reg<areg_cal::AREG_CAL_SPEC>,
    #[doc = "0xfe8 - Heart Beat OSC Calibration Register"]
    pub hbo_cal: crate::Reg<hbo_cal::HBO_CAL_SPEC>,
    #[doc = "0xfec - EFuse Config Register"]
    pub ef_config: crate::Reg<ef_config::EF_CONFIG_SPEC>,
    #[doc = "0xff0 - EFuse ID0 Register"]
    pub ef_id0: crate::Reg<ef_id0::EF_ID0_SPEC>,
    #[doc = "0xff4 - EFuse ID1 Register"]
    pub ef_id1: crate::Reg<ef_id1::EF_ID1_SPEC>,
    #[doc = "0xff8 - Processor ID Register"]
    pub procid: crate::Reg<procid::PROCID_SPEC>,
    #[doc = "0xffc - Peripheral ID Register"]
    pub perid: crate::Reg<perid::PERID_SPEC>,
}
#[doc = "RST_STAT register accessor: an alias for `Reg<RST_STAT_SPEC>`"]
pub type RST_STAT = crate::Reg<rst_stat::RST_STAT_SPEC>;
#[doc = "System Reset Status"]
pub mod rst_stat;
#[doc = "RST_CNTL_ROM register accessor: an alias for `Reg<RST_CNTL_ROM_SPEC>`"]
pub type RST_CNTL_ROM = crate::Reg<rst_cntl_rom::RST_CNTL_ROM_SPEC>;
#[doc = "ROM Reset Control"]
pub mod rst_cntl_rom;
#[doc = "RST_CNTL_RAM0 register accessor: an alias for `Reg<RST_CNTL_RAM0_SPEC>`"]
pub type RST_CNTL_RAM0 = crate::Reg<rst_cntl_ram0::RST_CNTL_RAM0_SPEC>;
#[doc = "RAM Reset Control"]
pub mod rst_cntl_ram0;
#[doc = "RST_CNTL_RAM1 register accessor: an alias for `Reg<RST_CNTL_RAM1_SPEC>`"]
pub type RST_CNTL_RAM1 = crate::Reg<rst_cntl_ram1::RST_CNTL_RAM1_SPEC>;
#[doc = "RAM Reset Control"]
pub mod rst_cntl_ram1;
#[doc = "ROM_PROT register accessor: an alias for `Reg<ROM_PROT_SPEC>`"]
pub type ROM_PROT = crate::Reg<rom_prot::ROM_PROT_SPEC>;
#[doc = "ROM Protection Configuration"]
pub mod rom_prot;
#[doc = "ROM_SCRUB register accessor: an alias for `Reg<ROM_SCRUB_SPEC>`"]
pub type ROM_SCRUB = crate::Reg<rom_scrub::ROM_SCRUB_SPEC>;
#[doc = "ROM Scrub Period Configuration"]
pub mod rom_scrub;
#[doc = "RAM0_SCRUB register accessor: an alias for `Reg<RAM0_SCRUB_SPEC>`"]
pub type RAM0_SCRUB = crate::Reg<ram0_scrub::RAM0_SCRUB_SPEC>;
#[doc = "RAM0 Scrub Period Configuration"]
pub mod ram0_scrub;
#[doc = "RAM1_SCRUB register accessor: an alias for `Reg<RAM1_SCRUB_SPEC>`"]
pub type RAM1_SCRUB = crate::Reg<ram1_scrub::RAM1_SCRUB_SPEC>;
#[doc = "RAM1 Scrub Period Configuration"]
pub mod ram1_scrub;
#[doc = "IRQ_ENB register accessor: an alias for `Reg<IRQ_ENB_SPEC>`"]
pub type IRQ_ENB = crate::Reg<irq_enb::IRQ_ENB_SPEC>;
#[doc = "Enable EDAC Error Interrupt Register"]
pub mod irq_enb;
#[doc = "IRQ_RAW register accessor: an alias for `Reg<IRQ_RAW_SPEC>`"]
pub type IRQ_RAW = crate::Reg<irq_raw::IRQ_RAW_SPEC>;
#[doc = "Raw EDAC Error Interrupt Status"]
pub mod irq_raw;
#[doc = "IRQ_END register accessor: an alias for `Reg<IRQ_END_SPEC>`"]
pub type IRQ_END = crate::Reg<irq_end::IRQ_END_SPEC>;
#[doc = "Enabled EDAC Error Interrupt Status"]
pub mod irq_end;
#[doc = "IRQ_CLR register accessor: an alias for `Reg<IRQ_CLR_SPEC>`"]
pub type IRQ_CLR = crate::Reg<irq_clr::IRQ_CLR_SPEC>;
#[doc = "Clear EDAC Error Interrupt Status"]
pub mod irq_clr;
#[doc = "RAM0_SBE register accessor: an alias for `Reg<RAM0_SBE_SPEC>`"]
pub type RAM0_SBE = crate::Reg<ram0_sbe::RAM0_SBE_SPEC>;
#[doc = "Count of RAM0 EDAC Single Bit Errors"]
pub mod ram0_sbe;
#[doc = "RAM1_SBE register accessor: an alias for `Reg<RAM1_SBE_SPEC>`"]
pub type RAM1_SBE = crate::Reg<ram1_sbe::RAM1_SBE_SPEC>;
#[doc = "Count of RAM1 EDAC Single Bit Errors"]
pub mod ram1_sbe;
#[doc = "RAM0_MBE register accessor: an alias for `Reg<RAM0_MBE_SPEC>`"]
pub type RAM0_MBE = crate::Reg<ram0_mbe::RAM0_MBE_SPEC>;
#[doc = "Count of RAM0 EDAC Multi Bit Errors"]
pub mod ram0_mbe;
#[doc = "RAM1_MBE register accessor: an alias for `Reg<RAM1_MBE_SPEC>`"]
pub type RAM1_MBE = crate::Reg<ram1_mbe::RAM1_MBE_SPEC>;
#[doc = "Count of RAM1 EDAC Multi Bit Errors"]
pub mod ram1_mbe;
#[doc = "ROM_SBE register accessor: an alias for `Reg<ROM_SBE_SPEC>`"]
pub type ROM_SBE = crate::Reg<rom_sbe::ROM_SBE_SPEC>;
#[doc = "Count of ROM EDAC Single Bit Errors"]
pub mod rom_sbe;
#[doc = "ROM_MBE register accessor: an alias for `Reg<ROM_MBE_SPEC>`"]
pub type ROM_MBE = crate::Reg<rom_mbe::ROM_MBE_SPEC>;
#[doc = "Count of ROM EDAC Multi Bit Errors"]
pub mod rom_mbe;
#[doc = "ROM_RETRIES register accessor: an alias for `Reg<ROM_RETRIES_SPEC>`"]
pub type ROM_RETRIES = crate::Reg<rom_retries::ROM_RETRIES_SPEC>;
#[doc = "ROM BOOT Retry count"]
pub mod rom_retries;
#[doc = "REFRESH_CONFIG_H register accessor: an alias for `Reg<REFRESH_CONFIG_H_SPEC>`"]
pub type REFRESH_CONFIG_H = crate::Reg<refresh_config_h::REFRESH_CONFIG_H_SPEC>;
#[doc = "Register Refresh Rate for TMR registers"]
pub mod refresh_config_h;
#[doc = "TIM_RESET register accessor: an alias for `Reg<TIM_RESET_SPEC>`"]
pub type TIM_RESET = crate::Reg<tim_reset::TIM_RESET_SPEC>;
#[doc = "TIM Reset Control"]
pub mod tim_reset;
#[doc = "TIM_CLK_ENABLE register accessor: an alias for `Reg<TIM_CLK_ENABLE_SPEC>`"]
pub type TIM_CLK_ENABLE = crate::Reg<tim_clk_enable::TIM_CLK_ENABLE_SPEC>;
#[doc = "TIM Enable Control"]
pub mod tim_clk_enable;
#[doc = "PERIPHERAL_RESET register accessor: an alias for `Reg<PERIPHERAL_RESET_SPEC>`"]
pub type PERIPHERAL_RESET = crate::Reg<peripheral_reset::PERIPHERAL_RESET_SPEC>;
#[doc = "Peripheral Reset Control"]
pub mod peripheral_reset;
#[doc = "PERIPHERAL_CLK_ENABLE register accessor: an alias for `Reg<PERIPHERAL_CLK_ENABLE_SPEC>`"]
pub type PERIPHERAL_CLK_ENABLE = crate::Reg<peripheral_clk_enable::PERIPHERAL_CLK_ENABLE_SPEC>;
#[doc = "Peripheral Enable Control"]
pub mod peripheral_clk_enable;
#[doc = "SPW_M4_CTRL register accessor: an alias for `Reg<SPW_M4_CTRL_SPEC>`"]
pub type SPW_M4_CTRL = crate::Reg<spw_m4_ctrl::SPW_M4_CTRL_SPEC>;
#[doc = "SPW M4 control register"]
pub mod spw_m4_ctrl;
#[doc = "PMU_CTRL register accessor: an alias for `Reg<PMU_CTRL_SPEC>`"]
pub type PMU_CTRL = crate::Reg<pmu_ctrl::PMU_CTRL_SPEC>;
#[doc = "PMU Control Register"]
pub mod pmu_ctrl;
#[doc = "WAKEUP_CNT register accessor: an alias for `Reg<WAKEUP_CNT_SPEC>`"]
pub type WAKEUP_CNT = crate::Reg<wakeup_cnt::WAKEUP_CNT_SPEC>;
#[doc = "Wakeup Control"]
pub mod wakeup_cnt;
#[doc = "EBI_CFG0 register accessor: an alias for `Reg<EBI_CFG0_SPEC>`"]
pub type EBI_CFG0 = crate::Reg<ebi_cfg0::EBI_CFG0_SPEC>;
#[doc = "EBI Config Register 0"]
pub mod ebi_cfg0;
#[doc = "EBI_CFG1 register accessor: an alias for `Reg<EBI_CFG1_SPEC>`"]
pub type EBI_CFG1 = crate::Reg<ebi_cfg1::EBI_CFG1_SPEC>;
#[doc = "EBI Config Register 1"]
pub mod ebi_cfg1;
#[doc = "EBI_CFG2 register accessor: an alias for `Reg<EBI_CFG2_SPEC>`"]
pub type EBI_CFG2 = crate::Reg<ebi_cfg2::EBI_CFG2_SPEC>;
#[doc = "EBI Config Register 2"]
pub mod ebi_cfg2;
#[doc = "EBI_CFG3 register accessor: an alias for `Reg<EBI_CFG3_SPEC>`"]
pub type EBI_CFG3 = crate::Reg<ebi_cfg3::EBI_CFG3_SPEC>;
#[doc = "EBI Config Register 3"]
pub mod ebi_cfg3;
#[doc = "ANALOG_CNTL register accessor: an alias for `Reg<ANALOG_CNTL_SPEC>`"]
pub type ANALOG_CNTL = crate::Reg<analog_cntl::ANALOG_CNTL_SPEC>;
#[doc = "Analog Control Register"]
pub mod analog_cntl;
#[doc = "SW_CLKDIV10 register accessor: an alias for `Reg<SW_CLKDIV10_SPEC>`"]
pub type SW_CLKDIV10 = crate::Reg<sw_clkdiv10::SW_CLKDIV10_SPEC>;
#[doc = "Initial SpW Clock Divider Value"]
pub mod sw_clkdiv10;
#[doc = "REFRESH_CONFIG_L register accessor: an alias for `Reg<REFRESH_CONFIG_L_SPEC>`"]
pub type REFRESH_CONFIG_L = crate::Reg<refresh_config_l::REFRESH_CONFIG_L_SPEC>;
#[doc = "Register Refresh Rate for TMR registers"]
pub mod refresh_config_l;
#[doc = "DAC0_CAL register accessor: an alias for `Reg<DAC0_CAL_SPEC>`"]
pub type DAC0_CAL = crate::Reg<dac0_cal::DAC0_CAL_SPEC>;
#[doc = "DAC0 Calibration Register"]
pub mod dac0_cal;
#[doc = "DAC1_CAL register accessor: an alias for `Reg<DAC1_CAL_SPEC>`"]
pub type DAC1_CAL = crate::Reg<dac1_cal::DAC1_CAL_SPEC>;
#[doc = "DAC1 Calibration Register"]
pub mod dac1_cal;
#[doc = "ADC_CAL register accessor: an alias for `Reg<ADC_CAL_SPEC>`"]
pub type ADC_CAL = crate::Reg<adc_cal::ADC_CAL_SPEC>;
#[doc = "ADC Calibration Register"]
pub mod adc_cal;
#[doc = "BG_CAL register accessor: an alias for `Reg<BG_CAL_SPEC>`"]
pub type BG_CAL = crate::Reg<bg_cal::BG_CAL_SPEC>;
#[doc = "Bandgap Calibration Register"]
pub mod bg_cal;
#[doc = "DREG_CAL register accessor: an alias for `Reg<DREG_CAL_SPEC>`"]
pub type DREG_CAL = crate::Reg<dreg_cal::DREG_CAL_SPEC>;
#[doc = "Digital LDO Regulator Calibration Register"]
pub mod dreg_cal;
#[doc = "AREG_CAL register accessor: an alias for `Reg<AREG_CAL_SPEC>`"]
pub type AREG_CAL = crate::Reg<areg_cal::AREG_CAL_SPEC>;
#[doc = "Analog LDO Regulator Calibration Register"]
pub mod areg_cal;
#[doc = "HBO_CAL register accessor: an alias for `Reg<HBO_CAL_SPEC>`"]
pub type HBO_CAL = crate::Reg<hbo_cal::HBO_CAL_SPEC>;
#[doc = "Heart Beat OSC Calibration Register"]
pub mod hbo_cal;
#[doc = "EF_CONFIG register accessor: an alias for `Reg<EF_CONFIG_SPEC>`"]
pub type EF_CONFIG = crate::Reg<ef_config::EF_CONFIG_SPEC>;
#[doc = "EFuse Config Register"]
pub mod ef_config;
#[doc = "EF_ID0 register accessor: an alias for `Reg<EF_ID0_SPEC>`"]
pub type EF_ID0 = crate::Reg<ef_id0::EF_ID0_SPEC>;
#[doc = "EFuse ID0 Register"]
pub mod ef_id0;
#[doc = "EF_ID1 register accessor: an alias for `Reg<EF_ID1_SPEC>`"]
pub type EF_ID1 = crate::Reg<ef_id1::EF_ID1_SPEC>;
#[doc = "EFuse ID1 Register"]
pub mod ef_id1;
#[doc = "PROCID register accessor: an alias for `Reg<PROCID_SPEC>`"]
pub type PROCID = crate::Reg<procid::PROCID_SPEC>;
#[doc = "Processor ID Register"]
pub mod procid;
#[doc = "PERID register accessor: an alias for `Reg<PERID_SPEC>`"]
pub type PERID = crate::Reg<perid::PERID_SPEC>;
#[doc = "Peripheral ID Register"]
pub mod perid;
