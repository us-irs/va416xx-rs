#[doc = "Register `VALID` reader"]
pub struct R(crate::R<VALID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VALID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VALID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VALID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EHR_VALID` reader - Indicates that the collection of bits in the TRNG is complete"]
pub struct EHR_VALID_R(crate::FieldReader<bool, bool>);
impl EHR_VALID_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EHR_VALID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EHR_VALID_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Indicates that the collection of bits in the TRNG is complete"]
    #[inline(always)]
    pub fn ehr_valid(&self) -> EHR_VALID_R {
        EHR_VALID_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "Valid Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [valid](index.html) module"]
pub struct VALID_SPEC;
impl crate::RegisterSpec for VALID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [valid::R](R) reader structure"]
impl crate::Readable for VALID_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets VALID to value 0"]
impl crate::Resettable for VALID_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
