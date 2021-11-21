#[doc = "Register `EHR_DATA2` reader"]
pub struct R(crate::R<EHR_DATA2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EHR_DATA2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EHR_DATA2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EHR_DATA2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EHR_DATA` reader - 32 Bits of Entropy Holding Register"]
pub struct EHR_DATA_R(crate::FieldReader<u32, u32>);
impl EHR_DATA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        EHR_DATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EHR_DATA_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - 32 Bits of Entropy Holding Register"]
    #[inline(always)]
    pub fn ehr_data(&self) -> EHR_DATA_R {
        EHR_DATA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "Entropy Holding Register Data Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ehr_data2](index.html) module"]
pub struct EHR_DATA2_SPEC;
impl crate::RegisterSpec for EHR_DATA2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ehr_data2::R](R) reader structure"]
impl crate::Readable for EHR_DATA2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets EHR_DATA2 to value 0"]
impl crate::Resettable for EHR_DATA2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
