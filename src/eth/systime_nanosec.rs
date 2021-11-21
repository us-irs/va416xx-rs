#[doc = "Register `SYSTIME_NANOSEC` reader"]
pub struct R(crate::R<SYSTIME_NANOSEC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSTIME_NANOSEC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSTIME_NANOSEC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSTIME_NANOSEC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TSSS` reader - Timestamp Sub Seconds"]
pub struct TSSS_R(crate::FieldReader<u32, u32>);
impl TSSS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        TSSS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSSS_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:30 - Timestamp Sub Seconds"]
    #[inline(always)]
    pub fn tsss(&self) -> TSSS_R {
        TSSS_R::new((self.bits & 0x7fff_ffff) as u32)
    }
}
#[doc = "Holds 32 bits of the nano-second field of the system time\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [systime_nanosec](index.html) module"]
pub struct SYSTIME_NANOSEC_SPEC;
impl crate::RegisterSpec for SYSTIME_NANOSEC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [systime_nanosec::R](R) reader structure"]
impl crate::Readable for SYSTIME_NANOSEC_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SYSTIME_NANOSEC to value 0"]
impl crate::Resettable for SYSTIME_NANOSEC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
