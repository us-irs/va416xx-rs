#[doc = "Register `EF_CONFIG` reader"]
pub struct R(crate::R<EF_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EF_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EF_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EF_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ROM_SPEED` reader - Specifies the speed of ROM_SCK"]
pub struct ROM_SPEED_R(crate::FieldReader<u8, u8>);
impl ROM_SPEED_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ROM_SPEED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ROM_SPEED_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ROM_SIZE` reader - Specifies how much of the full 128K byte address space is loaded from the external SPI memory after a reset"]
pub struct ROM_SIZE_R(crate::FieldReader<u8, u8>);
impl ROM_SIZE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ROM_SIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ROM_SIZE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ROM_NOCHECK` reader - When set to 1, the ROM check is skipped"]
pub struct ROM_NOCHECK_R(crate::FieldReader<bool, bool>);
impl ROM_NOCHECK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ROM_NOCHECK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ROM_NOCHECK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BOOT_DELAY` reader - Specifies the boot delay from the end of the Power-On-Sequence to the release of Reset"]
pub struct BOOT_DELAY_R(crate::FieldReader<u8, u8>);
impl BOOT_DELAY_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BOOT_DELAY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BOOT_DELAY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ROM_READ` reader - SPI ROM read instruction code"]
pub struct ROM_READ_R(crate::FieldReader<u8, u8>);
impl ROM_READ_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ROM_READ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ROM_READ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ROM_LATENCY` reader - Number of bits of latency from Address until data from the SPI ROM"]
pub struct ROM_LATENCY_R(crate::FieldReader<u8, u8>);
impl ROM_LATENCY_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ROM_LATENCY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ROM_LATENCY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ROM_ADDRESS` reader - ROM Address Mode"]
pub struct ROM_ADDRESS_R(crate::FieldReader<u8, u8>);
impl ROM_ADDRESS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ROM_ADDRESS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ROM_ADDRESS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ROM_DLYCAP` reader - ROM SPI Delayed capture"]
pub struct ROM_DLYCAP_R(crate::FieldReader<bool, bool>);
impl ROM_DLYCAP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ROM_DLYCAP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ROM_DLYCAP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ROM_STATUS` reader - The first data byte from the SPI ROM following an address is taken as a status byte"]
pub struct ROM_STATUS_R(crate::FieldReader<bool, bool>);
impl ROM_STATUS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ROM_STATUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ROM_STATUS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RM` reader - This bit controls the internal RAM read timing and must be maintained at this value"]
pub struct RM_R(crate::FieldReader<bool, bool>);
impl RM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WM` reader - This bit controls the internal RAM write timing and must be maintained at this value"]
pub struct WM_R(crate::FieldReader<bool, bool>);
impl WM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:1 - Specifies the speed of ROM_SCK"]
    #[inline(always)]
    pub fn rom_speed(&self) -> ROM_SPEED_R {
        ROM_SPEED_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:5 - Specifies how much of the full 128K byte address space is loaded from the external SPI memory after a reset"]
    #[inline(always)]
    pub fn rom_size(&self) -> ROM_SIZE_R {
        ROM_SIZE_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bit 6 - When set to 1, the ROM check is skipped"]
    #[inline(always)]
    pub fn rom_nocheck(&self) -> ROM_NOCHECK_R {
        ROM_NOCHECK_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 7:9 - Specifies the boot delay from the end of the Power-On-Sequence to the release of Reset"]
    #[inline(always)]
    pub fn boot_delay(&self) -> BOOT_DELAY_R {
        BOOT_DELAY_R::new(((self.bits >> 7) & 0x07) as u8)
    }
    #[doc = "Bits 10:17 - SPI ROM read instruction code"]
    #[inline(always)]
    pub fn rom_read(&self) -> ROM_READ_R {
        ROM_READ_R::new(((self.bits >> 10) & 0xff) as u8)
    }
    #[doc = "Bits 18:22 - Number of bits of latency from Address until data from the SPI ROM"]
    #[inline(always)]
    pub fn rom_latency(&self) -> ROM_LATENCY_R {
        ROM_LATENCY_R::new(((self.bits >> 18) & 0x1f) as u8)
    }
    #[doc = "Bits 23:24 - ROM Address Mode"]
    #[inline(always)]
    pub fn rom_address(&self) -> ROM_ADDRESS_R {
        ROM_ADDRESS_R::new(((self.bits >> 23) & 0x03) as u8)
    }
    #[doc = "Bit 25 - ROM SPI Delayed capture"]
    #[inline(always)]
    pub fn rom_dlycap(&self) -> ROM_DLYCAP_R {
        ROM_DLYCAP_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - The first data byte from the SPI ROM following an address is taken as a status byte"]
    #[inline(always)]
    pub fn rom_status(&self) -> ROM_STATUS_R {
        ROM_STATUS_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - This bit controls the internal RAM read timing and must be maintained at this value"]
    #[inline(always)]
    pub fn rm(&self) -> RM_R {
        RM_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - This bit controls the internal RAM write timing and must be maintained at this value"]
    #[inline(always)]
    pub fn wm(&self) -> WM_R {
        WM_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
#[doc = "EFuse Config Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ef_config](index.html) module"]
pub struct EF_CONFIG_SPEC;
impl crate::RegisterSpec for EF_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ef_config::R](R) reader structure"]
impl crate::Readable for EF_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets EF_CONFIG to value 0x0a80_0c40"]
impl crate::Resettable for EF_CONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0a80_0c40
    }
}
