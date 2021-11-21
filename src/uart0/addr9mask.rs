#[doc = "Register `ADDR9MASK` reader"]
pub struct R(crate::R<ADDR9MASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADDR9MASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADDR9MASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADDR9MASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADDR9MASK` writer"]
pub struct W(crate::W<ADDR9MASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADDR9MASK_SPEC>;
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
impl From<crate::W<ADDR9MASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADDR9MASK_SPEC>) -> Self {
        W(writer)
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
#[doc = "Address9 Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addr9mask](index.html) module"]
pub struct ADDR9MASK_SPEC;
impl crate::RegisterSpec for ADDR9MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [addr9mask::R](R) reader structure"]
impl crate::Readable for ADDR9MASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [addr9mask::W](W) writer structure"]
impl crate::Writable for ADDR9MASK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADDR9MASK to value 0"]
impl crate::Resettable for ADDR9MASK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
