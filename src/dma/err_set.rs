#[doc = "Register `ERR_SET` reader"]
pub struct R(crate::R<ERR_SET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ERR_SET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ERR_SET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ERR_SET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ERR_SET` reader - Set Error"]
pub struct ERR_SET_R(crate::FieldReader<bool, bool>);
impl ERR_SET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ERR_SET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERR_SET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Set Error"]
    #[inline(always)]
    pub fn err_set(&self) -> ERR_SET_R {
        ERR_SET_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "DMA bus error set\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [err_set](index.html) module"]
pub struct ERR_SET_SPEC;
impl crate::RegisterSpec for ERR_SET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [err_set::R](R) reader structure"]
impl crate::Readable for ERR_SET_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ERR_SET to value 0"]
impl crate::Resettable for ERR_SET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
