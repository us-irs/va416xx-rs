#[doc = "Register `SYSTIME_SECONDS` reader"]
pub struct R(crate::R<SYSTIME_SECONDS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSTIME_SECONDS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSTIME_SECONDS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSTIME_SECONDS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TSS` reader - Timestamp Second"]
pub struct TSS_R(crate::FieldReader<u32, u32>);
impl TSS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        TSS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSS_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Timestamp Second"]
    #[inline(always)]
    pub fn tss(&self) -> TSS_R {
        TSS_R::new(self.bits as u32)
    }
}
#[doc = "Holds the lower 32 bits of the second field of the system time\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [systime_seconds](index.html) module"]
pub struct SYSTIME_SECONDS_SPEC;
impl crate::RegisterSpec for SYSTIME_SECONDS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [systime_seconds::R](R) reader structure"]
impl crate::Readable for SYSTIME_SECONDS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SYSTIME_SECONDS to value 0"]
impl crate::Resettable for SYSTIME_SECONDS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
