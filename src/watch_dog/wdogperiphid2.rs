#[doc = "Register `WDOGPERIPHID2` reader"]
pub struct R(crate::R<WDOGPERIPHID2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDOGPERIPHID2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WDOGPERIPHID2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WDOGPERIPHID2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PERIPHID` reader - Peripheral ID"]
pub struct PERIPHID_R(crate::FieldReader<u8, u8>);
impl PERIPHID_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PERIPHID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PERIPHID_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Peripheral ID"]
    #[inline(always)]
    pub fn periphid(&self) -> PERIPHID_R {
        PERIPHID_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Peripheral ID\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdogperiphid2](index.html) module"]
pub struct WDOGPERIPHID2_SPEC;
impl crate::RegisterSpec for WDOGPERIPHID2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wdogperiphid2::R](R) reader structure"]
impl crate::Readable for WDOGPERIPHID2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets WDOGPERIPHID2 to value 0x1b"]
impl crate::Resettable for WDOGPERIPHID2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1b
    }
}
