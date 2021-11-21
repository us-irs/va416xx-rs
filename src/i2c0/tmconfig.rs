#[doc = "Register `TMCONFIG` reader"]
pub struct R(crate::R<TMCONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TMCONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TMCONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TMCONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TMCONFIG` writer"]
pub struct W(crate::W<TMCONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TMCONFIG_SPEC>;
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
impl From<crate::W<TMCONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TMCONFIG_SPEC>) -> Self {
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
#[doc = "Timing Config Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmconfig](index.html) module"]
pub struct TMCONFIG_SPEC;
impl crate::RegisterSpec for TMCONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tmconfig::R](R) reader structure"]
impl crate::Readable for TMCONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tmconfig::W](W) writer structure"]
impl crate::Writable for TMCONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TMCONFIG to value 0"]
impl crate::Resettable for TMCONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
