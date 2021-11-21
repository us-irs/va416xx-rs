#[doc = "Register `CTMR` reader"]
pub struct R(crate::R<CTMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CTMR` reader - Time Stamp Counter"]
pub struct CTMR_R(crate::FieldReader<u16, u16>);
impl CTMR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        CTMR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTMR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - Time Stamp Counter"]
    #[inline(always)]
    pub fn ctmr(&self) -> CTMR_R {
        CTMR_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "CAN Timer Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctmr](index.html) module"]
pub struct CTMR_SPEC;
impl crate::RegisterSpec for CTMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctmr::R](R) reader structure"]
impl crate::Readable for CTMR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CTMR to value 0"]
impl crate::Resettable for CTMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
