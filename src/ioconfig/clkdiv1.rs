#[doc = "Register `CLKDIV1` reader"]
pub struct R(crate::R<CLKDIV1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLKDIV1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLKDIV1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLKDIV1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLKDIV1` writer"]
pub struct W(crate::W<CLKDIV1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLKDIV1_SPEC>;
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
impl From<crate::W<CLKDIV1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLKDIV1_SPEC>) -> Self {
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
#[doc = "Clock divide value. 0 will disable the clock\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkdiv1](index.html) module"]
pub struct CLKDIV1_SPEC;
impl crate::RegisterSpec for CLKDIV1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clkdiv1::R](R) reader structure"]
impl crate::Readable for CLKDIV1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clkdiv1::W](W) writer structure"]
impl crate::Writable for CLKDIV1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLKDIV1 to value 0"]
impl crate::Resettable for CLKDIV1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
