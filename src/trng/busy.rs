#[doc = "Register `BUSY` reader"]
pub struct R(crate::R<BUSY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BUSY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BUSY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BUSY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BUSY` reader - Reflects the status of the rng_busy signal"]
pub struct BUSY_R(crate::FieldReader<bool, bool>);
impl BUSY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BUSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Reflects the status of the rng_busy signal"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "Busy Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [busy](index.html) module"]
pub struct BUSY_SPEC;
impl crate::RegisterSpec for BUSY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [busy::R](R) reader structure"]
impl crate::Readable for BUSY_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BUSY to value 0"]
impl crate::Resettable for BUSY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
