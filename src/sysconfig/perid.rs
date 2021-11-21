#[doc = "Register `PERID` reader"]
pub struct R(crate::R<PERID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PERID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PERID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PERID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MANUFACTURER_ID` reader - MANUFACTURER_ID"]
pub struct MANUFACTURER_ID_R(crate::FieldReader<u16, u16>);
impl MANUFACTURER_ID_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        MANUFACTURER_ID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MANUFACTURER_ID_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PERIPHERAL_ID` reader - PERIPHERAL_ID"]
pub struct PERIPHERAL_ID_R(crate::FieldReader<u8, u8>);
impl PERIPHERAL_ID_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PERIPHERAL_ID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PERIPHERAL_ID_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PERIPHERAL_VER` reader - PERIPHERAL_VER"]
pub struct PERIPHERAL_VER_R(crate::FieldReader<u8, u8>);
impl PERIPHERAL_VER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PERIPHERAL_VER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PERIPHERAL_VER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:11 - MANUFACTURER_ID"]
    #[inline(always)]
    pub fn manufacturer_id(&self) -> MANUFACTURER_ID_R {
        MANUFACTURER_ID_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:23 - PERIPHERAL_ID"]
    #[inline(always)]
    pub fn peripheral_id(&self) -> PERIPHERAL_ID_R {
        PERIPHERAL_ID_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - PERIPHERAL_VER"]
    #[inline(always)]
    pub fn peripheral_ver(&self) -> PERIPHERAL_VER_R {
        PERIPHERAL_VER_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Peripheral ID Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [perid](index.html) module"]
pub struct PERID_SPEC;
impl crate::RegisterSpec for PERID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [perid::R](R) reader structure"]
impl crate::Readable for PERID_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PERID to value 0x0280_07e9"]
impl crate::Resettable for PERID_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0280_07e9
    }
}
