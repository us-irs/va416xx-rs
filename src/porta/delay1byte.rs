#[doc = "Register `DELAY1BYTE[%s]` reader"]
pub struct R(crate::R<DELAY1BYTE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DELAY1BYTE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DELAY1BYTE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DELAY1BYTE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DELAY1BYTE[%s]` writer"]
pub struct W(crate::W<DELAY1BYTE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DELAY1BYTE_SPEC>;
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
impl From<crate::W<DELAY1BYTE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DELAY1BYTE_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Delay1 Register by Byte\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [delay1byte](index.html) module"]
pub struct DELAY1BYTE_SPEC;
impl crate::RegisterSpec for DELAY1BYTE_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [delay1byte::R](R) reader structure"]
impl crate::Readable for DELAY1BYTE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [delay1byte::W](W) writer structure"]
impl crate::Writable for DELAY1BYTE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DELAY1BYTE[%s]
to value 0"]
impl crate::Resettable for DELAY1BYTE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
