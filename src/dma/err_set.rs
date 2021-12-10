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
#[doc = "Register `ERR_SET` writer"]
pub struct W(crate::W<ERR_SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ERR_SET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<ERR_SET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ERR_SET_SPEC>) -> Self {
        W(writer)
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
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA bus error set\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [err_set](index.html) module"]
pub struct ERR_SET_SPEC;
impl crate::RegisterSpec for ERR_SET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [err_set::R](R) reader structure"]
impl crate::Readable for ERR_SET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [err_set::W](W) writer structure"]
impl crate::Writable for ERR_SET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ERR_SET to value 0"]
impl crate::Resettable for ERR_SET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
