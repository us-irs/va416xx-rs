#[doc = "Register `WDOGVALUE` reader"]
pub struct R(crate::R<WDOGVALUE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDOGVALUE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WDOGVALUE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WDOGVALUE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CNT` reader - Actual Count"]
pub struct CNT_R(crate::FieldReader<u32, u32>);
impl CNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNT_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Actual Count"]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "Down Counter Value\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdogvalue](index.html) module"]
pub struct WDOGVALUE_SPEC;
impl crate::RegisterSpec for WDOGVALUE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wdogvalue::R](R) reader structure"]
impl crate::Readable for WDOGVALUE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets WDOGVALUE to value 0xffff_ffff"]
impl crate::Resettable for WDOGVALUE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
