#[doc = "Register `BIST_CNTR0` reader"]
pub struct R(crate::R<BIST_CNTR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BIST_CNTR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BIST_CNTR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BIST_CNTR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ROSC_CNTR_VAL` reader - Returns the results of the TRNG BIST counter"]
pub struct ROSC_CNTR_VAL_R(crate::FieldReader<u32, u32>);
impl ROSC_CNTR_VAL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        ROSC_CNTR_VAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ROSC_CNTR_VAL_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:21 - Returns the results of the TRNG BIST counter"]
    #[inline(always)]
    pub fn rosc_cntr_val(&self) -> ROSC_CNTR_VAL_R {
        ROSC_CNTR_VAL_R::new((self.bits & 0x003f_ffff) as u32)
    }
}
#[doc = "BIST Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bist_cntr0](index.html) module"]
pub struct BIST_CNTR0_SPEC;
impl crate::RegisterSpec for BIST_CNTR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bist_cntr0::R](R) reader structure"]
impl crate::Readable for BIST_CNTR0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BIST_CNTR0 to value 0"]
impl crate::Resettable for BIST_CNTR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
