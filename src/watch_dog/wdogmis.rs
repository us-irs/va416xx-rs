#[doc = "Register `WDOGMIS` reader"]
pub struct R(crate::R<WDOGMIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDOGMIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WDOGMIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WDOGMIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `INTERRUPT` reader - Masked Interrupt Status"]
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
    #[doc = "Bit 0 - Masked Interrupt Status"]
    #[inline(always)]
    pub fn interrupt(&self) -> INTERRUPT_R {
        INTERRUPT_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "Interrupt status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdogmis](index.html) module"]
pub struct WDOGMIS_SPEC;
impl crate::RegisterSpec for WDOGMIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wdogmis::R](R) reader structure"]
impl crate::Readable for WDOGMIS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets WDOGMIS to value 0"]
impl crate::Resettable for WDOGMIS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
