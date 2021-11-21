#[doc = "Register `SYND_CHECK_32_52_DATA` reader"]
pub struct R(crate::R<SYND_CHECK_32_52_DATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYND_CHECK_32_52_DATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYND_CHECK_32_52_DATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYND_CHECK_32_52_DATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "EDAC Decode Data\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [synd_check_32_52_data](index.html) module"]
pub struct SYND_CHECK_32_52_DATA_SPEC;
impl crate::RegisterSpec for SYND_CHECK_32_52_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [synd_check_32_52_data::R](R) reader structure"]
impl crate::Readable for SYND_CHECK_32_52_DATA_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SYND_CHECK_32_52_DATA to value 0"]
impl crate::Resettable for SYND_CHECK_32_52_DATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
