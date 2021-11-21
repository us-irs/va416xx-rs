#[doc = "Register `WDOGRIS` reader"]
pub struct R(crate::R<WDOGRIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDOGRIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WDOGRIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WDOGRIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `INTERRUPT` reader - Interrupt Status"]
pub struct INTERRUPT_R(crate::FieldReader<bool, bool>);
impl INTERRUPT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INTERRUPT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTERRUPT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Interrupt Status"]
    #[inline(always)]
    pub fn interrupt(&self) -> INTERRUPT_R {
        INTERRUPT_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "Raw interrupt status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdogris](index.html) module"]
pub struct WDOGRIS_SPEC;
impl crate::RegisterSpec for WDOGRIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wdogris::R](R) reader structure"]
impl crate::Readable for WDOGRIS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets WDOGRIS to value 0"]
impl crate::Resettable for WDOGRIS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
