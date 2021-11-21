#[doc = "Register `PULSEBASE` reader"]
pub struct R(crate::R<PULSEBASE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PULSEBASE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PULSEBASE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PULSEBASE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PULSEBASE` writer"]
pub struct W(crate::W<PULSEBASE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PULSEBASE_SPEC>;
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
impl From<crate::W<PULSEBASE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PULSEBASE_SPEC>) -> Self {
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
#[doc = "Pulse Base Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pulsebase](index.html) module"]
pub struct PULSEBASE_SPEC;
impl crate::RegisterSpec for PULSEBASE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pulsebase::R](R) reader structure"]
impl crate::Readable for PULSEBASE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pulsebase::W](W) writer structure"]
impl crate::Writable for PULSEBASE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PULSEBASE to value 0"]
impl crate::Resettable for PULSEBASE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
