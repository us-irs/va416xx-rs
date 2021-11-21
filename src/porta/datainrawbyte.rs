#[doc = "Register `DATAINRAWBYTE[%s]` reader"]
pub struct R(crate::R<DATAINRAWBYTE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATAINRAWBYTE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATAINRAWBYTE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATAINRAWBYTE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Data In Raw Register by Byte\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [datainrawbyte](index.html) module"]
pub struct DATAINRAWBYTE_SPEC;
impl crate::RegisterSpec for DATAINRAWBYTE_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [datainrawbyte::R](R) reader structure"]
impl crate::Readable for DATAINRAWBYTE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DATAINRAWBYTE[%s]
to value 0"]
impl crate::Resettable for DATAINRAWBYTE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
