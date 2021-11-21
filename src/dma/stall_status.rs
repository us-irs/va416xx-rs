#[doc = "Register `STALL_STATUS` reader"]
pub struct R(crate::R<STALL_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STALL_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STALL_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STALL_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `STALL_STATUS` reader - DMA is stalled"]
pub struct STALL_STATUS_R(crate::FieldReader<bool, bool>);
impl STALL_STATUS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        STALL_STATUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STALL_STATUS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - DMA is stalled"]
    #[inline(always)]
    pub fn stall_status(&self) -> STALL_STATUS_R {
        STALL_STATUS_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "DMA stall status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stall_status](index.html) module"]
pub struct STALL_STATUS_SPEC;
impl crate::RegisterSpec for STALL_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stall_status::R](R) reader structure"]
impl crate::Readable for STALL_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STALL_STATUS to value 0"]
impl crate::Resettable for STALL_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
