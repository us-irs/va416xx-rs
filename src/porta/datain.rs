#[doc = "Register `DATAIN` reader"]
pub struct R(crate::R<DATAIN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATAIN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATAIN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATAIN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Data In Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [datain](index.html) module"]
pub struct DATAIN_SPEC;
impl crate::RegisterSpec for DATAIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [datain::R](R) reader structure"]
impl crate::Readable for DATAIN_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DATAIN to value 0"]
impl crate::Resettable for DATAIN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
