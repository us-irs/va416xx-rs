#[doc = "Register `CLKPRESCALE` reader"]
pub struct R(crate::R<CLKPRESCALE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLKPRESCALE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLKPRESCALE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLKPRESCALE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLKPRESCALE` writer"]
pub struct W(crate::W<CLKPRESCALE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLKPRESCALE_SPEC>;
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
impl From<crate::W<CLKPRESCALE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLKPRESCALE_SPEC>) -> Self {
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
#[doc = "Clock Pre Scale divide value\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkprescale](index.html) module"]
pub struct CLKPRESCALE_SPEC;
impl crate::RegisterSpec for CLKPRESCALE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clkprescale::R](R) reader structure"]
impl crate::Readable for CLKPRESCALE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clkprescale::W](W) writer structure"]
impl crate::Writable for CLKPRESCALE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLKPRESCALE to value 0"]
impl crate::Resettable for CLKPRESCALE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
