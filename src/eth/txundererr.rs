#[doc = "Register `TXUNDERERR` reader"]
pub struct R(crate::R<TXUNDERERR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXUNDERERR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXUNDERERR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXUNDERERR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `COUNT` reader - Number of frames"]
pub struct COUNT_R(crate::FieldReader<u32, u32>);
impl COUNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        COUNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COUNT_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Number of frames"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new(self.bits as u32)
    }
}
#[doc = "MMC number of frames aborted because of frame underflow error\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txundererr](index.html) module"]
pub struct TXUNDERERR_SPEC;
impl crate::RegisterSpec for TXUNDERERR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txundererr::R](R) reader structure"]
impl crate::Readable for TXUNDERERR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TXUNDERERR to value 0"]
impl crate::Resettable for TXUNDERERR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
