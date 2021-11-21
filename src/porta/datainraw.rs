#[doc = "Register `DATAINRAW` reader"]
pub struct R(crate::R<DATAINRAW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATAINRAW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATAINRAW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATAINRAW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Data In Raw Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [datainraw](index.html) module"]
pub struct DATAINRAW_SPEC;
impl crate::RegisterSpec for DATAINRAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [datainraw::R](R) reader structure"]
impl crate::Readable for DATAINRAW_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DATAINRAW to value 0"]
impl crate::Resettable for DATAINRAW_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
