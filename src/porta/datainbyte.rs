#[doc = "Register `DATAINBYTE[%s]` reader"]
pub struct R(crate::R<DATAINBYTE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATAINBYTE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATAINBYTE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATAINBYTE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Data In Register by Byte\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [datainbyte](index.html) module"]
pub struct DATAINBYTE_SPEC;
impl crate::RegisterSpec for DATAINBYTE_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [datainbyte::R](R) reader structure"]
impl crate::Readable for DATAINBYTE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DATAINBYTE[%s]
to value 0"]
impl crate::Resettable for DATAINBYTE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
