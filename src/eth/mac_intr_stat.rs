#[doc = "Register `MAC_INTR_STAT` reader"]
pub struct R(crate::R<MAC_INTR_STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAC_INTR_STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAC_INTR_STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAC_INTR_STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TSIS` reader - Timestamp Interrupt Status"]
pub struct TSIS_R(crate::FieldReader<bool, bool>);
impl TSIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TSIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MMCRXIPIS` reader - MMC Receive Checksum Offload Interrupt Status"]
pub struct MMCRXIPIS_R(crate::FieldReader<bool, bool>);
impl MMCRXIPIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MMCRXIPIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MMCRXIPIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MMCTXIS` reader - MMC Transmit Interrupt Status"]
pub struct MMCTXIS_R(crate::FieldReader<bool, bool>);
impl MMCTXIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MMCTXIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MMCTXIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MMCRXIS` reader - MMC Receive Interrupt Status"]
pub struct MMCRXIS_R(crate::FieldReader<bool, bool>);
impl MMCRXIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MMCRXIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MMCRXIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MMCIS` reader - MMC Interrupt Status"]
pub struct MMCIS_R(crate::FieldReader<bool, bool>);
impl MMCIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MMCIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MMCIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 9 - Timestamp Interrupt Status"]
    #[inline(always)]
    pub fn tsis(&self) -> TSIS_R {
        TSIS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 7 - MMC Receive Checksum Offload Interrupt Status"]
    #[inline(always)]
    pub fn mmcrxipis(&self) -> MMCRXIPIS_R {
        MMCRXIPIS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - MMC Transmit Interrupt Status"]
    #[inline(always)]
    pub fn mmctxis(&self) -> MMCTXIS_R {
        MMCTXIS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - MMC Receive Interrupt Status"]
    #[inline(always)]
    pub fn mmcrxis(&self) -> MMCRXIS_R {
        MMCRXIS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - MMC Interrupt Status"]
    #[inline(always)]
    pub fn mmcis(&self) -> MMCIS_R {
        MMCIS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
#[doc = "Contains the interrupt status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_intr_stat](index.html) module"]
pub struct MAC_INTR_STAT_SPEC;
impl crate::RegisterSpec for MAC_INTR_STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mac_intr_stat::R](R) reader structure"]
impl crate::Readable for MAC_INTR_STAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MAC_INTR_STAT to value 0"]
impl crate::Resettable for MAC_INTR_STAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
