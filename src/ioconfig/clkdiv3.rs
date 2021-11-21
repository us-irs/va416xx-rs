#[doc = "Register `CLKDIV3` reader"]
pub struct R(crate::R<CLKDIV3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLKDIV3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLKDIV3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLKDIV3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLKDIV3` writer"]
pub struct W(crate::W<CLKDIV3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLKDIV3_SPEC>;
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
impl From<crate::W<CLKDIV3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLKDIV3_SPEC>) -> Self {
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
#[doc = "Clock divide value. 0 will disable the clock\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkdiv3](index.html) module"]
pub struct CLKDIV3_SPEC;
impl crate::RegisterSpec for CLKDIV3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clkdiv3::R](R) reader structure"]
impl crate::Readable for CLKDIV3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clkdiv3::W](W) writer structure"]
impl crate::Writable for CLKDIV3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLKDIV3 to value 0"]
impl crate::Resettable for CLKDIV3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
