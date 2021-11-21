#[doc = "Register `TOGOUT` writer"]
pub struct W(crate::W<TOGOUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TOGOUT_SPEC>;
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
impl From<crate::W<TOGOUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TOGOUT_SPEC>) -> Self {
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
#[doc = "Toggle Out Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [togout](index.html) module"]
pub struct TOGOUT_SPEC;
impl crate::RegisterSpec for TOGOUT_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [togout::W](W) writer structure"]
impl crate::Writable for TOGOUT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TOGOUT to value 0"]
impl crate::Resettable for TOGOUT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
